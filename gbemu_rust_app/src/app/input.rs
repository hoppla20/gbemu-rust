use gbemu_rust_lib::prelude::Key;

use super::action::Action;

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
        key: &egui::Key,
        pressed: &bool,
        modifiers: &egui::Modifiers,
    ) -> Option<Action> {
        if *modifiers == egui::Modifiers::NONE && *key == egui::Key::Escape && *pressed {
            return Some(Action::TogglePause);
        }

        // emulator inputs are not allowed to have modifiers
        if *modifiers != egui::Modifiers::NONE {
            return None;
        }

        if *key == self.arrow_down_key {
            log::debug!(
                "Arrown down key {}",
                if *pressed { "pressed" } else { "released" }
            );
            return Some(Action::KeyEvent {
                key: Key::Down,
                pressed: *pressed,
            });
        } else if *key == self.arrow_up_key {
            log::debug!(
                "Arrown up key {}",
                if *pressed { "pressed" } else { "released" }
            );
            return Some(Action::KeyEvent {
                key: Key::Up,
                pressed: *pressed,
            });
        } else if *key == self.arrow_left_key {
            log::debug!(
                "Arrown left key {}",
                if *pressed { "pressed" } else { "released" }
            );
            return Some(Action::KeyEvent {
                key: Key::Left,
                pressed: *pressed,
            });
        } else if *key == self.arrow_right_key {
            log::debug!(
                "Arrown right key {}",
                if *pressed { "pressed" } else { "released" }
            );
            return Some(Action::KeyEvent {
                key: Key::Right,
                pressed: *pressed,
            });
        } else if *key == self.b_key {
            log::debug!("B key {}", if *pressed { "pressed" } else { "released" });
            return Some(Action::KeyEvent {
                key: Key::B,
                pressed: *pressed,
            });
        } else if *key == self.a_key {
            log::debug!("A key {}", if *pressed { "pressed" } else { "released" });
            return Some(Action::KeyEvent {
                key: Key::A,
                pressed: *pressed,
            });
        } else if *key == self.select_key {
            log::debug!(
                "SELECT key {}",
                if *pressed { "pressed" } else { "released" }
            );
            return Some(Action::KeyEvent {
                key: Key::Select,
                pressed: *pressed,
            });
        } else if *key == self.start_key {
            log::debug!(
                "START key {}",
                if *pressed { "pressed" } else { "released" }
            );
            return Some(Action::KeyEvent {
                key: Key::Start,
                pressed: *pressed,
            });
        }

        None
    }
}
