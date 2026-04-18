use dioxus::prelude::*;

use crate::hooks::use_controlled;
use crate::variant_classes;

variant_classes!(SwitchSize, Default, "default", SM, "sm");

#[component]
pub fn Switch(
    #[props(default)] size: SwitchSize,

    #[props(default)] class: String,

    #[props(default)] default_value: bool,
    #[props(default)] value: ReadSignal<Option<bool>>,
    #[props(default)] on_value_change: Callback<bool>,

    #[props(extends = GlobalAttributes, extends = button)] rest: Vec<Attribute>,
) -> Element {
    let (value, set_value_internal) = use_controlled(value, default_value, on_value_change);

    let handle_toggle = move |_| {
        set_value_internal(!*value.read());
    };

    rsx! {
        button {
            r#type: "button",
            role: "switch",
            aria_checked: value,
            "data-state": if value() { "checked" } else { "unchecked" },
            value: "on",
            "data-slot": "switch",
            "data-size": size.class(),
            class: "peer group/switch relative inline-flex shrink-0 items-center rounded-full border border-transparent transition-all outline-none after:absolute after:-inset-x-3 after:-inset-y-2 focus-visible:border-ring focus-visible:ring-2 focus-visible:ring-ring/30 aria-invalid:border-destructive aria-invalid:ring-2 aria-invalid:ring-destructive/20 data-[size=default]:h-[16.6px] data-[size=default]:w-7 data-[size=sm]:h-3.5 data-[size=sm]:w-6 dark:aria-invalid:border-destructive/50 dark:aria-invalid:ring-destructive/40 data-checked:bg-primary data-unchecked:bg-input dark:data-unchecked:bg-input/80 data-disabled:cursor-not-allowed data-disabled:opacity-50",
            class: "{class}",
            onclick: handle_toggle,
            ..rest,
            span {
                "data-state": if value() { "checked" } else { "unchecked" },
                "data-slot": "switch-thumb",
                class: "pointer-events-none block rounded-full bg-background ring-0 transition-transform group-data-[size=default]/switch:size-3.5 group-data-[size=sm]/switch:size-3 group-data-[size=default]/switch:data-checked:translate-x-[calc(100%-2px)] group-data-[size=sm]/switch:data-checked:translate-x-[calc(100%-2px)] dark:data-checked:bg-primary-foreground group-data-[size=default]/switch:data-unchecked:translate-x-0 group-data-[size=sm]/switch:data-unchecked:translate-x-0 dark:data-unchecked:bg-foreground",
            }
        }
    }
}
