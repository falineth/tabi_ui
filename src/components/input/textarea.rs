use dioxus::prelude::*;

use crate::hooks::use_controlled;

#[component]
pub fn TextArea(
    #[props(default)] class: String,

    #[props(default)] default_value: String,
    #[props(default)] value: ReadSignal<Option<String>>,
    #[props(default)] on_value_change: Callback<String>,

    #[props(extends = GlobalAttributes, extends = textarea)] rest: Vec<Attribute>,

    children: Element,
) -> Element {
    let (value, set_value_internal) = use_controlled(value, default_value, on_value_change);

    rsx! {
        textarea {
            "data-slot": "textarea",
            class: "border-input bg-input/20 dark:bg-input/30 focus-visible:border-ring focus-visible:ring-ring/30 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 resize-none rounded-md border px-2 py-2 text-sm transition-colors focus-visible:ring-2 aria-invalid:ring-2 md:text-xs/relaxed placeholder:text-muted-foreground flex field-sizing-content min-h-16 w-full outline-none disabled:cursor-not-allowed disabled:opacity-50",
            class: "{class}",
            value,
            oninput: move |e| set_value_internal(e.data.value()),
            ..rest,
            {children}
        }
    }
}
