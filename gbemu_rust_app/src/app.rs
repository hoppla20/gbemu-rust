#![allow(unused_variables)]

mod gbemu_wgpu;
mod stats;

use stats::Stats;

use gbemu_rust_lib::prelude::Emulator;
use gbemu_wgpu::GbemuCallback;
use gbemu_wgpu::GbemuResources;

use eframe::egui_wgpu;
use eframe::wgpu;
use eframe::wgpu::util::DeviceExt as _;
use egui::Ui;
use poll_promise::Promise;
use rfd::AsyncFileDialog;
use std::fmt::Display;
use std::num::NonZeroU64;

static CYCLES_PER_SECOND: u32 = 4_194_304;

enum AppState {
    Idle,
    FileDialog(Promise<Vec<u8>>),
    Running,
}

impl Display for AppState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Idle => "Idle",
            Self::FileDialog(_) => "File Dialog",
            Self::Running => "Running",
            #[allow(unreachable_patterns)]
            _ => "Unknown",
        })
    }
}

pub struct GbemuApp {
    angle: f32,

    stats: Stats,

    state: AppState,
    emulator: Option<Emulator>,
}

impl GbemuApp {
    pub fn new<'a>(cc: &'a &eframe::CreationContext<'a>) -> Option<Self> {
        let wgpu_render_state = cc.wgpu_render_state.as_ref()?;

        let device = &wgpu_render_state.device;

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("custom3d"),
            source: wgpu::ShaderSource::Wgsl(include_str!("./app/gbemu_wgpu_shader.wgsl").into()),
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("custom3d"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: NonZeroU64::new(16),
                },
                count: None,
            }],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("custom3d"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("custom3d"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: None,
                buffers: &[],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu_render_state.target_format.into())],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("custom3d"),
            contents: bytemuck::cast_slice(&[0.0_f32; 4]), // 16 bytes aligned!
            // Mapping at creation (as done by the create_buffer_init utility) doesn't require us to to add the MAP_WRITE usage
            // (this *happens* to workaround this bug )
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("custom3d"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        // Because the graphics pipeline must have the same lifetime as the egui render pass,
        // instead of storing the pipeline in our `Custom3D` struct, we insert it into the
        // `paint_callback_resources` type map, which is stored alongside the render pass.
        wgpu_render_state
            .renderer
            .write()
            .callback_resources
            .insert(GbemuResources {
                pipeline,
                bind_group,
                uniform_buffer,
            });

        Some(Self {
            angle: 0.0,
            stats: Stats::default(),
            state: AppState::Idle,
            emulator: None,
        })
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn execute<F: std::future::Future<Output = Vec<u8>> + Send + 'static>(f: F) -> Promise<Vec<u8>> {
    Promise::spawn_async(f)
}

#[cfg(target_arch = "wasm32")]
fn execute<F: std::future::Future<Output = Vec<u8>> + 'static>(f: F) -> Promise<Vec<u8>> {
    Promise::spawn_local(f)
}

impl eframe::App for GbemuApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        match &self.state {
            AppState::Idle => {},
            AppState::FileDialog(promise) => {
                if let Some(rom) = promise.ready() {
                    self.emulator =
                        Some(Emulator::new_from_buffer(rom.clone(), true, None, None).unwrap());
                    self.state = AppState::Running;
                }
            },
            AppState::Running => {
                let dt = ctx.input(|i| i.stable_dt);
                let cycles = (((CYCLES_PER_SECOND as f32) * dt).round() as u32)
                    .min(((CYCLES_PER_SECOND as f32) * (1.0 / 10.0)) as u32);

                self.stats
                    .on_frame_update(ctx.input(|i| i.time), dt, cycles);

                log::debug!("Executing {} emulator cycles", cycles);

                for _ in 0..cycles {
                    let _ = self.emulator.as_mut().unwrap().step();
                }

                ctx.request_repaint();
            },
            #[allow(unreachable_patterns)]
            _ => {
                panic!("Unknown App State!");
            },
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // the top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // no quit on web pages
                ui.menu_button("File", |ui| {
                    if ui.button("Open").clicked() {
                        let ctx_clone = ctx.clone();
                        self.state = AppState::FileDialog(execute(async move {
                            let file = AsyncFileDialog::new().pick_file().await.unwrap();
                            let result = file.read().await;
                            ctx_clone.request_repaint();
                            result
                        }));

                        ui.close_menu();
                    }

                    if ui
                        .add_enabled(
                            matches!(self.state, AppState::Running),
                            egui::Button::new("Stop"),
                        )
                        .clicked()
                    {
                        self.state = AppState::Idle;
                        self.emulator = None;
                        self.stats.reset();
                    }

                    if !cfg!(target_arch = "wasm32") && ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.add_space(16.0);

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                self.paint_game_window(ui);
            });
        });

        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui| {
                    ui.label(format!("Status: {}", self.state));
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                    self.stats.status_bar_ui(ui);
                });
            })
        });
    }
}

impl GbemuApp {
    fn paint_game_window(&mut self, ui: &mut Ui) {
        let (rect, response) =
            ui.allocate_exact_size(egui::Vec2::splat(300.0), egui::Sense::drag());

        self.angle += response.drag_motion().x * 0.01;

        ui.painter().add(egui_wgpu::Callback::new_paint_callback(
            rect,
            GbemuCallback { angle: self.angle },
        ));
    }
}
