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
            class: "border-view-foregroundnormal/20 hover:border-complementary-decorationhover focus-visible:border-complementary-decorationfocus placeholder:text-muted-foreground",
            class: "aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50",
            class: "file:h-6 file:text-xs/relaxed file:font-medium file:text-foreground file:inline-flex file:border-0 file:bg-transparent",
            class: "disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50",
            class: "h-7 rounded-sm border px-2 py-0.5 text-sm transition-colors aria-invalid:ring-2 md:text-xs/relaxed w-full min-w-0 outline-none",
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
