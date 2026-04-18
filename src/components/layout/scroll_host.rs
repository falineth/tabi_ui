use dioxus::prelude::*;

#[component]
pub fn ScrollHost(
    #[props(default)] class: String,
    #[props(extends = GlobalAttributes, extends = div)] rest: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        div { class: "h-full overflow-hidden", class: "{class}", ..rest, {children} }
    }
}
