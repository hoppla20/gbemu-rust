#![allow(unused_variables)]

mod input;
mod stats;
mod task;

use egui::Vec2;
use gbemu_rust_lib::prelude::LCD_HEIGHT;
use gbemu_rust_lib::prelude::LCD_WIDTH;
use gbemu_rust_lib::prelude::Pixel;
use input::InputHandler;
use stats::Stats;

use gbemu_rust_lib::prelude::Emulator;

use poll_promise::Promise;
use rfd::AsyncFileDialog;
use std::cmp::min;
use std::fmt::Display;

const MIN_FPS: f32 = 10.0;
const TEXTURE_SIZE: [usize; 2] = [LCD_WIDTH, LCD_HEIGHT];
const CYCLES_PER_SECOND: u32 = 4_194_304;
const DEFAULT_PALETTE: [egui::Color32; 4] = [
    egui::Color32::from_rgba_premultiplied(0xe0, 0xf0, 0xe7, 0xff), // White
    egui::Color32::from_rgba_premultiplied(0x8b, 0xa3, 0x94, 0xff), // Light gray
    egui::Color32::from_rgba_premultiplied(0x55, 0x64, 0x5a, 0xff), // Dark gray
    egui::Color32::from_rgba_premultiplied(0x34, 0x3d, 0x37, 0xff), // Black
];

enum AppState {
    Idle,
    FileDialog(Promise<Option<Vec<u8>>>),
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
    scale: usize,

    stats: Stats,
    input_handler: InputHandler,

    state: AppState,
    emulator: Option<Emulator>,

    texture: egui::TextureHandle,
}

impl GbemuApp {
    pub fn new<'a>(
        cc: &'a &eframe::CreationContext<'a>,
        emulator: Option<Emulator>,
    ) -> Option<Self> {
        Some(Self {
            scale: 2,
            stats: Stats::default(),
            input_handler: InputHandler::default(),
            state: if emulator.is_none() {
                AppState::Idle
            } else {
                AppState::Running
            },
            emulator,
            texture: cc.egui_ctx.load_texture(
                "gbemu",
                egui::ColorImage::new(TEXTURE_SIZE, DEFAULT_PALETTE[0]),
                egui::TextureOptions::NEAREST,
            ),
        })
    }
}

impl eframe::App for GbemuApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        match &self.state {
            AppState::Idle => {},
            AppState::FileDialog(promise) => {
                if let Some(rom_file) = promise.ready() {
                    if let Some(rom) = rom_file {
                        self.emulator =
                            Some(Emulator::new_from_buffer(rom.clone(), true, None, None).unwrap());
                        self.state = AppState::Running;
                    } else {
                        self.state = AppState::Idle;
                    }
                }
            },
            AppState::Running => {
                let dt = ctx.input(|i| i.stable_dt);
                let cycles = min(
                    ((CYCLES_PER_SECOND as f32) * dt).round() as u32,
                    ((CYCLES_PER_SECOND as f32) * (1.0 / MIN_FPS)) as u32,
                ) / 4;

                self.stats
                    .on_frame_update(ctx.input(|i| i.time), dt, cycles);

                let events = ctx.input(|i| i.events.clone());
                for event in &events {
                    #[allow(clippy::single_match)]
                    match event {
                        egui::Event::Key {
                            key,
                            pressed,
                            repeat,
                            modifiers,
                            ..
                        } => self.input_handler.handle(
                            self.emulator.as_mut().unwrap(),
                            key,
                            pressed,
                            repeat,
                            modifiers,
                        ),
                        _ => {},
                    }
                }

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
                        self.state = AppState::FileDialog(task::execute(async move {
                            let result;
                            if let Some(file) = AsyncFileDialog::new().pick_file().await {
                                result = Some(file.read().await);
                            } else {
                                result = None;
                            }
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

                ui.menu_button("View", |ui| {
                    if ui.button("Zoom in").clicked() {
                        self.scale += 1;
                    }

                    if ui
                        .add_enabled(self.scale > 1, egui::Button::new("Zoom out"))
                        .clicked()
                    {
                        self.scale -= 1;
                    }
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                    egui::widgets::global_theme_preference_buttons(ui)
                });
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

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(
                egui::Layout::centered_and_justified(egui::Direction::TopDown),
                |ui| {
                    let mut frame_buffer = [[Pixel::Color0; LCD_WIDTH]; LCD_HEIGHT];
                    if let AppState::Running = self.state {
                        frame_buffer = self
                            .emulator
                            .as_ref()
                            .unwrap()
                            .system
                            .graphics
                            .renderer
                            .get_framebuffer();
                    }
                    let mut frame_data: Vec<egui::Color32> = vec![];

                    for y in 0..LCD_HEIGHT {
                        for x in 0..LCD_WIDTH {
                            frame_data.push(
                                DEFAULT_PALETTE
                                    [<Pixel as Into<u8>>::into(frame_buffer[y][x]) as usize],
                            );
                        }
                    }

                    let frame = egui::ColorImage {
                        size: TEXTURE_SIZE,
                        pixels: frame_data,
                    };

                    self.texture.set(frame, egui::TextureOptions::NEAREST);

                    let size = self.texture.size_vec2();
                    let sized_texture = egui::load::SizedTexture::new(self.texture.id(), size);
                    ui.add(
                        egui::Image::new(sized_texture).fit_to_exact_size(
                            size * Vec2::new(self.scale as f32, self.scale as f32),
                        ),
                    );
                },
            );
        });
    }
}
