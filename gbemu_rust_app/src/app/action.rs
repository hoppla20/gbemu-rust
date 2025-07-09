use gbemu_rust_lib::prelude::Key;

pub enum Action {
    TogglePause,
    KeyEvent { key: Key, pressed: bool },
}
