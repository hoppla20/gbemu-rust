#![allow(unused_variables)]

mod action;
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
    Paused,
}

impl Display for AppState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Idle => "Idle",
            Self::FileDialog(_) => "File Dialog",
            Self::Running => "Running",
            Self::Paused => "Paused",
        })
    }
}

pub struct GbemuApp {
    scale: f32,

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
            scale: 1.0,
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
                            repeat: false,
                            modifiers,
                            ..
                        } => {
                            if let Some(action) = self.input_handler.handle(key, pressed, modifiers)
                            {
                                match action {
                                    action::Action::TogglePause => {
                                        self.state = AppState::Paused;
                                    },
                                    action::Action::KeyEvent { key, pressed } => {
                                        self.emulator
                                            .as_mut()
                                            .unwrap()
                                            .system
                                            .io
                                            .joypad
                                            .key_event(key, pressed);
                                    },
                                }
                            }
                        },
                        _ => {},
                    }
                }

                for _ in 0..cycles {
                    let _ = self.emulator.as_mut().unwrap().step();
                }

                ctx.request_repaint();
            },
            AppState::Paused => {
                let events = ctx.input(|i| i.events.clone());
                for event in &events {
                    #[allow(clippy::single_match)]
                    match event {
                        egui::Event::Key {
                            key,
                            pressed,
                            repeat: false,
                            modifiers,
                            ..
                        } => match self.input_handler.handle(key, pressed, modifiers) {
                            Some(action::Action::TogglePause) => {
                                self.state = AppState::Running;
                            },
                            _ => {},
                        },
                        _ => {},
                    }
                }
            },
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // the top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                // no quit on web pages
                ui.menu_button("File", |ui| {
                    if ui
                        .add_enabled(
                            matches!(self.state, AppState::Idle),
                            egui::Button::new("Open"),
                        )
                        .clicked()
                    {
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

                    let toggle_pause_label = if matches!(self.state, AppState::Paused) {
                        "Continue"
                    } else {
                        "Pause"
                    };
                    if ui
                        .add_enabled(
                            matches!(self.state, AppState::Running)
                                || matches!(self.state, AppState::Paused),
                            egui::Button::new(toggle_pause_label),
                        )
                        .clicked()
                    {
                        self.state = match self.state {
                            AppState::Running => AppState::Paused,
                            AppState::Paused => AppState::Running,
                            _ => unreachable!(),
                        }
                    }

                    if ui
                        .add_enabled(
                            matches!(self.state, AppState::Running)
                                || matches!(self.state, AppState::Paused),
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
                    if ui
                        .add_enabled(self.scale < 1.0, egui::Button::new("Zoom in"))
                        .clicked()
                    {
                        self.scale += 0.25;
                    }

                    if ui.button("Zoom out").clicked() {
                        self.scale -= 0.25;
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
                    let frame_buffer = match self.state {
                        AppState::Running | AppState::Paused => self
                            .emulator
                            .as_ref()
                            .unwrap()
                            .system
                            .graphics
                            .renderer
                            .get_framebuffer(),
                        _ => [[Pixel::Color0; LCD_WIDTH]; LCD_HEIGHT],
                    };
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

                    // Use all of the available width or height. The available width needs
                    // to be subtracted with the height of the top and bottom bars. Their
                    // default height is `ui.style().spacing.interact_size.y`
                    let available_size = ui
                        .available_height()
                        .min(ui.available_width() - (2.0 * ui.style().spacing.interact_size.y));

                    ui.add(
                        egui::Image::new(&self.texture).fit_to_exact_size(
                            self.scale * Vec2::new(available_size, available_size),
                        ),
                    );
                },
            );
        });
    }
}
