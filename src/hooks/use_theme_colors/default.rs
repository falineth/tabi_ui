use dioxus::prelude::*;

const DEFAULT_THEME: &str = include_str!("../../../assets/default_theme.css");

pub fn use_theme_colors() -> ReadSignal<String> {
    use_signal(get_theme_colors).into()
}

fn get_theme_colors() -> String {
    DEFAULT_THEME.to_string()
}
