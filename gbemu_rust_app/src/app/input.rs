use gbemu_rust_lib::prelude::Emulator;

pub struct InputHandler {
    arrow_down_key: egui::Key,
    arrow_up_key: egui::Key,
    arrow_left_key: egui::Key,
    arrow_right_key: egui::Key,

    b_key: egui::Key,
    a_key: egui::Key,
    select_key: egui::Key,
    start_key: egui::Key,
}

impl Default for InputHandler {
    fn default() -> Self {
        Self {
            arrow_down_key: egui::Key::ArrowDown,
            arrow_up_key: egui::Key::ArrowUp,
            arrow_left_key: egui::Key::ArrowLeft,
            arrow_right_key: egui::Key::ArrowRight,

            b_key: egui::Key::Z,
            a_key: egui::Key::X,
            select_key: egui::Key::Comma,
            start_key: egui::Key::Period,
        }
    }
}

impl InputHandler {
    pub fn handle(
        &self,
        emulator: &mut Emulator,
        key: &egui::Key,
        pressed: &bool,
        repeat: &bool,
        modifiers: &egui::Modifiers,
    ) {
        if modifiers != &egui::Modifiers::NONE || *repeat {
            return;
        }

        if *key == self.arrow_down_key {
            log::debug!(
                "Arrown down key {}",
                if *pressed { "pressed" } else { "released" }
            );
            emulator.system.io.joypad.down_pressed(*pressed);
        } else if *key == self.arrow_up_key {
            log::debug!(
                "Arrown up key {}",
                if *pressed { "pressed" } else { "released" }
            );
            emulator.system.io.joypad.up_pressed(*pressed);
        } else if *key == self.arrow_left_key {
            log::debug!(
                "Arrown left key {}",
                if *pressed { "pressed" } else { "released" }
            );
            emulator.system.io.joypad.left_pressed(*pressed);
        } else if *key == self.arrow_right_key {
            log::debug!(
                "Arrown right key {}",
                if *pressed { "pressed" } else { "released" }
            );
            emulator.system.io.joypad.right_pressed(*pressed);
        } else if *key == self.b_key {
            log::debug!("B key {}", if *pressed { "pressed" } else { "released" });
            emulator.system.io.joypad.b_pressed(*pressed);
        } else if *key == self.a_key {
            log::debug!("A key {}", if *pressed { "pressed" } else { "released" });
            emulator.system.io.joypad.a_pressed(*pressed);
        } else if *key == self.select_key {
            log::debug!(
                "SELECT key {}",
                if *pressed { "pressed" } else { "released" }
            );
            emulator.system.io.joypad.select_pressed(*pressed);
        } else if *key == self.start_key {
            log::debug!(
                "START key {}",
                if *pressed { "pressed" } else { "released" }
            );
            emulator.system.io.joypad.start_pressed(*pressed);
        }
    }
}
