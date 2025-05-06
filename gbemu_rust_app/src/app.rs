use gbemu_rust_lib::prelude::Emulator;

pub struct GbemuApp {
    label: String,
    counter: u32,
    emulator: Emulator,
}

impl GbemuApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            label: "Hello World!".to_owned(),
            counter: 0,
            emulator: Emulator::new().unwrap(),
        }
    }
}

impl eframe::App for GbemuApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let _ = self.emulator.step();

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // the top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // no quit on web pages
                if !cfg!(target_arch = "wasm32") {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
