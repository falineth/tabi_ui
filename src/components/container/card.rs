use dioxus::prelude::*;

use crate::variant_classes;

variant_classes!(CardSize, Default, "default", SM, "sm");

#[component]
pub fn Card(
    #[props(default)] size: CardSize,
    #[props(default)] class: String,
    #[props(default = true)] auto_padding: bool,
    #[props(default)] onclick: EventHandler<Event<MouseData>>,
    #[props(extends = GlobalAttributes, extends = div)] rest: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        div {
            "data-slot": "card",
            "data-size": "{size}",
            class: "ring-foreground/10 bg-card text-card-foreground gap-4 overflow-hidden rounded-lg text-xs/relaxed ring-1 data-[size=sm]:gap-3 *:[img:first-child]:rounded-t-lg *:[img:last-child]:rounded-b-lg group/card flex flex-col",
            class: if auto_padding { "py-4 has-[>img:first-child]:pt-0 data-[size=sm]:py-3" },
            class: "{class}",
            onclick: move |e| onclick.call(e),
            ..rest,
            {children}
        }
    }
}

#[component]
pub fn CardHeader(
    #[props(default)] class: String,
    #[props(extends = GlobalAttributes, extends = div)] rest: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        div {
            "data-slot": "card-header",
            class: "gap-1 rounded-t-lg px-4 group-data-[size=sm]/card:px-3 [.border-b]:pb-4 group-data-[size=sm]/card:[.border-b]:pb-3 group/card-header @container/card-header grid auto-rows-min items-start has-data-[slot=card-action]:grid-cols-[1fr_auto] has-data-[slot=card-description]:grid-rows-[auto_auto]",
            class: "{class}",
            ..rest,
            {children}
        }
    }
}

#[component]
pub fn CardTitle(
    #[props(default)] class: String,
    #[props(extends = GlobalAttributes, extends = div)] rest: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        div {
            "data-slot": "card-title",
            class: "text-sm font-medium",
            class: "{class}",
            ..rest,
            {children}
        }
    }
}

#[component]
pub fn CardDescription(
    #[props(default)] class: String,
    #[props(extends = GlobalAttributes, extends = div)] rest: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        div {
            "data-slot": "card-description",
            class: "text-muted-foreground text-xs/relaxed",
            class: "{class}",
            ..rest,
            {children}
        }
    }
}

#[component]
pub fn CardAction(
    #[props(default)] class: String,
    #[props(extends = GlobalAttributes, extends = div)] rest: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        div {
            "data-slot": "card-action",
            class: "col-start-2 row-span-2 row-start-1 self-start justify-self-end",
            class: "{class}",
            ..rest,
            {children}
        }
    }
}

#[component]
pub fn CardContent(
    #[props(default)] class: String,
    #[props(extends = GlobalAttributes, extends = div)] rest: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        div {
            "data-slot": "card-content",
            class: "px-4 group-data-[size=sm]/card:px-3",
            class: "{class}",
            ..rest,
            {children}
        }
    }
}

#[component]
pub fn CardFooter(
    #[props(default)] class: String,
    #[props(extends = GlobalAttributes, extends = div)] rest: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        div {
            "data-slot": "card-footer",
            class: "rounded-b-lg px-4 group-data-[size=sm]/card:px-3 [.border-t]:pt-4 group-data-[size=sm]/card:[.border-t]:pt-3 flex items-center",
            class: "{class}",
            ..rest,
            {children}
        }
    }
}
