use dioxus::prelude::*;

#[derive(Default, Clone, PartialEq)]
pub struct WindowSize {
    pub height: f64,
    pub width: f64,
}

pub fn use_window_size() -> Signal<WindowSize> {
    use_context::<Signal<WindowSize>>()
}
