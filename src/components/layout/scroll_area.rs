use dioxus::prelude::*;

#[component]
pub fn ScrollArea(
    #[props(default)] class: String,
    #[props(extends = GlobalAttributes, extends = div)] rest: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        div { class: "overflow-y-auto h-full", class: "{class}", ..rest, {children} }
    }
}
