use dioxus::prelude::*;

use crate::hooks::use_controlled;

#[component]
pub fn TextInput(
    #[props(default)] class: String,

    #[props(default)] default_value: String,

    #[props(default)] value: ReadSignal<Option<String>>,

    #[props(default)] on_value_change: Callback<String>,

    #[props(default)] on_blur: Callback<Event<FocusData>>,

    #[props(default)] on_mounted: Callback<Event<MountedData>>,

    #[props(default)] on_accept: Callback<String>,

    #[props(extends = GlobalAttributes, extends = input)] rest: Vec<Attribute>,
    children: Element,
) -> Element {
    let (value, set_value_internal) = use_controlled(value, default_value, on_value_change);

    rsx! {
        input {
            "data-slot": "input",
            class: "bg-input/20 dark:bg-input/30 border-input focus-visible:border-ring focus-visible:ring-ring/30 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 h-7 rounded-md border px-2 py-0.5 text-sm transition-colors file:h-6 file:text-xs/relaxed file:font-medium focus-visible:ring-2 aria-invalid:ring-2 md:text-xs/relaxed file:text-foreground placeholder:text-muted-foreground w-full min-w-0 outline-none file:inline-flex file:border-0 file:bg-transparent disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50",
            class: "{class}",
            value,
            oninput: move |e| set_value_internal(e.data.value()),
            onblur: move |e| on_blur.call(e),
            onmounted: move |e| on_mounted.call(e),
            onkeydown: move |e| {
                if e.key() == Key::Enter {
                    on_accept.call(value.cloned());
                }
            },
            ..rest,
            {children}
        }
    }
}
