use dioxus::prelude::*;
use dioxus_free_icons::icons::md_action_icons::MdAutorenew;

use crate::components::Icon;
use crate::icons::MdCircle;

#[component]
pub fn Spinner(#[props(default)] class: String) -> Element {
    rsx! {
        div {
            role: "status",
            aria_label: "Loading",
            class: "{class}",
            class: "inline-block animate-spin",
            Icon { icon: MdAutorenew, height: 12, width: 12 }
        }
    }
}
