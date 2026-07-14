use dioxus::prelude::*;

use crate::hooks::use_controlled;
use crate::variant_classes;

variant_classes!(
    ToggleVariant,
    Default,
    "",
    Outline,
    "border-input hover:bg-muted border bg-transparent"
);

variant_classes!(
    ToggleSize,
    Default,
    "h-7 min-w-7 px-2",
    SM,
    "h-6 min-w-6 rounded-[min(var(--radius-md),8px)] px-1.5 text-[0.625rem] [&_svg:not([class*='size-'])]:size-3",
    LG,
    "h-8 min-w-8 px-2"
);

#[component]
pub fn Toggle(
    #[props(default)] variant: ToggleVariant,
    #[props(default)] size: ToggleSize,

    #[props(default)] class: String,

    #[props(default)] default_value: bool,
    #[props(default)] value: ReadSignal<Option<bool>>,
    #[props(default)] on_value_change: Callback<bool>,

    #[props(extends = GlobalAttributes, extends = button)] rest: Vec<Attribute>,

    children: Element,
) -> Element {
    let (value, set_value_internal) = use_controlled(value, default_value, on_value_change);

    let handle_toggle = move |_| {
        set_value_internal(!*value.read());
    };

    rsx! {
        button {
            "data-toggle": "toggle",
            aria_pressed: if *value.read() { "true" } else { "false" },
            class: "gap-1 rounded-sm text-xs font-medium transition-all [&_svg:not([class*='size-'])]:size-3.5 group/toggle inline-flex items-center justify-center whitespace-nowrap outline-none [&_svg]:shrink-0 [&_svg]:pointer-events-none select-none",
            class: "disabled:pointer-events-none disabled:opacity-50",
            class: "hover:border-view-decorationhover focus-visible:border-view-decorationfocus",
            class: "focus-visible:border-ring focus-visible:ring-1 focus-visible:ring-window-decorationfocus/5",
            class: "aria-pressed:bg-view-decorationfocus/20 aria-pressed:border-view-decorationhover/50",
            class: "aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive",
            class: "bg-window-backgroundnormal border border-window-foregroundnormal/20",
            class: "{variant.class()}",
            class: "{size.class()}",
            class: "{class}",
            onclick: handle_toggle,
            ..rest,
            {children}
        }
    }
}
