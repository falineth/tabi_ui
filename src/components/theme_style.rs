use dioxus::prelude::*;

use crate::use_theme_colors;

#[component]
pub fn ThemeStyle() -> Element {
    let theme_css = use_theme_colors();

    rsx! {
        style { "{theme_css}" }
    }
}
