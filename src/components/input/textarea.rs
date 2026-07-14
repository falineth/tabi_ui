use dioxus::prelude::*;

use crate::hooks::use_controlled;

#[component]
pub fn TextArea(
    #[props(default)] class: String,

    #[props(default)] default_value: String,
    #[props(default)] value: ReadSignal<Option<String>>,
    #[props(default)] on_value_change: Callback<String>,
    #[props(default)] on_accept: Option<Callback<String>>,

    #[props(extends = GlobalAttributes, extends = textarea)] rest: Vec<Attribute>,

    children: Element,
) -> Element {
    let (value, set_value_internal) = use_controlled(value, default_value, on_value_change);

    rsx! {
        textarea {
            "data-slot": "textarea",
            class: "border-view-foregroundnormal/20 bg-view-backgroundnormal text-view-foregroundnormal hover:border-view-decorationhover focus-visible:border-view-decorationfocus placeholder:text-view-foregroundinactive",
            class: "aria-invalid:ring-view-foregroundnegative/20 aria-invalid:border-view-foregroundnegative",
            class: "resize-none rounded-sm border px-2 py-2 text-sm transition-colors aria-invalid:ring-2 md:text-xs/relaxed flex field-sizing-content min-h-16 w-full outline-none disabled:cursor-not-allowed disabled:opacity-50",
            class: "{class}",
            value,
            oninput: move |e| set_value_internal(e.data.value()),
            onkeydown: move |e| {
                if let Some(on_accept) = on_accept && e.key() == Key::Enter
                    && e.modifiers().is_empty()
                {
                    e.prevent_default();
                    on_accept.call(value.cloned());
                }
            },
            ..rest,
            {children}
        }
    }
}
