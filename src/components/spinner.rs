use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::bs_icons::BsArrowRepeat;

#[component]
pub fn Spinner(#[props(default)] class: String) -> Element {
    rsx! {
        div {
            role: "status",
            aria_label: "Loading",
            class: "{class}",
            class: "inline-block animate-spin",
            Icon { icon: BsArrowRepeat, height: 12, width: 12 }
        }
    }
}
