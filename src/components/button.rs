use dioxus::prelude::*;

const BASE_CLASSES: &str = "focus-visible:border-ring focus-visible:ring-ring/30 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 rounded-md border border-transparent bg-clip-padding text-xs/relaxed font-medium focus-visible:ring-[2px] aria-invalid:ring-[2px] [&_svg:not([class*='size-'])]:size-4 inline-flex items-center justify-center whitespace-nowrap transition-all disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none shrink-0 [&_svg]:shrink-0 outline-none group/button select-none";

use crate::variant_classes;

variant_classes!(
    ButtonVariant,
    Default,
    "bg-primary text-primary-foreground hover:bg-primary/80",
    Outline,
    "border-border dark:bg-input/30 hover:bg-input/50 hover:text-foreground aria-expanded:bg-muted aria-expanded:text-foreground",
    Secondary,
    "bg-secondary text-secondary-foreground hover:bg-secondary/80 aria-expanded:bg-secondary aria-expanded:text-secondary-foreground",
    Ghost,
    "hover:bg-muted hover:text-foreground dark:hover:bg-muted/50 aria-expanded:bg-muted aria-expanded:text-foreground",
    GhostPrimary,
    "bg-primary/10 hover:bg-primary/20 focus-visible:ring-primary/20 dark:focus-visible:ring-primary/40 dark:bg-primary/20 text-primary-foreground focus-visible:border-primary/40 dark:hover:bg-primary/30",
    Destructive,
    "bg-destructive/10 hover:bg-destructive/20 focus-visible:ring-destructive/20 dark:focus-visible:ring-destructive/40 dark:bg-destructive/20 text-destructive focus-visible:border-destructive/40 dark:hover:bg-destructive/30",
    Link,
    "text-primary underline-offset-4 hover:underline",
    Custom,
    ""
);

variant_classes!(
    ButtonSize,
    Default,
    "h-7 gap-1 px-2 text-xs/relaxed has-data-[icon=inline-end]:pr-1.5 has-data-[icon=inline-start]:pl-1.5 [&_svg:not([class*='size-'])]:size-3.5",
    XS,
    "h-5 gap-1 rounded-sm px-2 text-[0.625rem] has-data-[icon=inline-end]:pr-1.5 has-data-[icon=inline-start]:pl-1.5 [&_svg:not([class*='size-'])]:size-2.5",
    SM,
    "h-6 gap-1 px-2 text-xs/relaxed has-data-[icon=inline-end]:pr-1.5 has-data-[icon=inline-start]:pl-1.5 [&_svg:not([class*='size-'])]:size-3",
    LG,
    "h-8 gap-1 px-2.5 text-xs/relaxed has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2 [&_svg:not([class*='size-'])]:size-4",
    Icon,
    "size-7 [&_svg:not([class*='size-'])]:size-3.5",
    IconXS,
    "size-5 rounded-sm [&_svg:not([class*='size-'])]:size-2.5",
    IconSM,
    "size-6 [&_svg:not([class*='size-'])]:size-3",
    IconLG,
    "size-8 [&_svg:not([class*='size-'])]:size-4"
);

#[component]
pub fn Button(
    #[props(default)] variant: ButtonVariant,
    #[props(default)] size: ButtonSize,
    #[props(default)] class: String,
    #[props(extends = GlobalAttributes, extends = button)] rest: Vec<Attribute>,
    #[props(default)] onclick: EventHandler<Event<MouseData>>,
    children: Element,
) -> Element {
    rsx! {
        button {
            "data-slot": "button",
            "data-variant": "{variant}",
            "data-size": "{size}",
            class: "{BASE_CLASSES}",
            class: "{variant.class()}",
            class: "{size.class()}",
            class: "{class}",
            onclick: move |e| onclick.call(e),
            ..rest,
            {children}
        }
    }
}
