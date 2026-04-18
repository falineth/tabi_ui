use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::bs_icons::{BsChevronRight, BsThreeDots};

//use crate::Route;

#[component]
pub fn Breadcrumb(
    #[props(default)] class: String,
    #[props(extends = GlobalAttributes, extends = nav)] rest: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        nav {
            aria_label: "breadcrumb",
            "data-slot": "breadcrumb",
            class: "{class}",
            ..rest,
            {children}
        }
    }
}

#[component]
pub fn BreadcrumbList(
    #[props(default)] class: String,
    #[props(extends = GlobalAttributes, extends = ol)] rest: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        ol {
            "data-slot": "breadcrumb-list",
            class: "text-muted-foreground gap-1.5 text-xs/relaxed flex flex-wrap items-center wrap-break-word",
            class: "{class}",
            ..rest,
            {children}
        }
    }
}

#[component]
pub fn BreadcrumbItem(
    #[props(default)] class: String,
    #[props(extends = GlobalAttributes, extends = li)] rest: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        li {
            "data-slot": "breadcrumb-item",
            class: "gap-1 inline-flex items-center",
            class: "{class}",
            ..rest,
            {children}
        }
    }
}

#[component]
pub fn BreadcrumbLink(
    #[props(into)]
    to: NavigationTarget,
    #[props(default)] class: String,
    #[props(extends = GlobalAttributes, extends = span)] rest: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        Link { to,
            span {
                "data-slot": "breadcrumb-item",
                class: "hover:text-foreground transition-colors",
                class: "{class}",
                ..rest,
                {children}
            }
        }
    }
}

#[component]
pub fn BreadcrumbPage(
    #[props(default)] class: String,
    #[props(extends = GlobalAttributes, extends = span)] rest: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        span {
            "data-slot": "breadcrumb-page",
            role: "link",
            aria_disabled: "true",
            aria_current: "page",
            class: "text-foreground font-normal",
            class: "{class}",
            ..rest,
            {children}
        }
    }
}

#[component]
pub fn BreadcrumbSeparator(
    #[props(default)] class: String,
    #[props(extends = GlobalAttributes, extends = li)] rest: Vec<Attribute>,
    children: Option<Element>,
) -> Element {
    rsx! {
        li {
            "data-slot": "breadcrumb-separator",
            role: "presentation",
            aria_hidden: "true",
            class: "[&>svg]:size-3.5",
            class: "{class}",
            ..rest,
            if children.is_none() {
                Icon { icon: BsChevronRight }
            } else {
                {children}
            }
        }
    }
}

#[component]
pub fn BreadcrumbEllipsis(
    #[props(default)] class: String,
    #[props(extends = GlobalAttributes, extends = span)] rest: Vec<Attribute>,
    children: Option<Element>,
) -> Element {
    rsx! {
        span {
            "data-slot": "breadcrumb-ellipsis",
            role: "presentation",
            aria_hidden: "true",
            class: "size-4 [&>svg]:size-3.5 flex items-center justify-center",
            class: "{class}",
            ..rest,
            Icon { icon: BsThreeDots }
            span { class: "sr-only", "More" }
        }
    }
}
