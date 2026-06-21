use dioxus::prelude::*;

const DEFAULT_THEME: &str = include_str!("../../../assets/default_theme.css");
const DEFAULT_WINDOW_BACKGROUND: (u8, u8, u8, u8) = (0xEF, 0xF0, 0xF1, 0xFF);

pub fn use_theme_colors() -> ReadSignal<String> {
    use_signal(get_theme_colors).into()
}

fn get_theme_colors() -> String {
    DEFAULT_THEME.to_string()
}

pub fn get_window_background_color() -> (u8, u8, u8, u8) {
    return DEFAULT_WINDOW_BACKGROUND;
}

#[derive(Clone)]
pub struct ThemeContext {
    pub theme_css: String,
    pub bg_color: (u8, u8, u8, u8),
}

impl ThemeContext {
    pub fn init() -> Self {
        ThemeContext {
            theme_css: DEFAULT_THEME.to_string(),
            bg_color: DEFAULT_WINDOW_BACKGROUND,
        }
    }
}
