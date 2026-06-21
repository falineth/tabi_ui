use std::fmt::Display;

use dioxus::prelude::*;

use crate::hooks::use_controlled;

pub trait ValidatableValue: Display + Default + Clone + PartialEq {
    fn validation_error(&self) -> Option<String>;
    fn finalize_value(self) -> Self;
    fn parse_text(value: String) -> Self;
}

#[component]
pub fn ValidatedTextInput<T: ValidatableValue + 'static>(
    #[props(default)] class: String,

    #[props(default)] default_value: T,

    #[props(default)] value: ReadSignal<Option<T>>,

    #[props(default)] on_value_change: Callback<T>,

    #[props(default)] on_blur: Callback<Event<FocusData>>,

    #[props(default)] on_mounted: Callback<Event<MountedData>>,

    #[props(default)] on_accept: Callback<T>,

    #[props(extends = GlobalAttributes, extends = input)] rest: Vec<Attribute>,
    children: Element,
) -> Element {
    let (value, set_value_internal) = use_controlled(value, default_value, on_value_change);

    let handle_blur = use_callback(move |e: Event<FocusData>| {
        set_value_internal(value.read().cloned().finalize_value());
        on_blur.call(e);
    });

    rsx! {
        div { class: "flex flex-col", class: "{class}",
            input {
                "data-slot": "input",
                class: "border-view-foregroundnormal/20 hover:border-complementary-decorationhover focus-visible:border-complementary-decorationfocus placeholder:text-view-foregroundinactive",
                class: "aria-invalid:ring-1 aria-invalid:ring-view-foregroundnegative/10 aria-invalid:border-view-foregroundnegative",
                class: "file:h-6 file:text-xs/relaxed file:font-medium file:text-foreground file:inline-flex file:border-0 file:bg-transparent",
                class: "disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50",
                class: "h-7 rounded-sm border px-2 py-0.5 text-sm transition-colors md:text-xs/relaxed w-full min-w-0 outline-none",

                value: "{value}",
                aria_invalid: if value.read().validation_error().is_some() { "true" },
                oninput: move |e| set_value_internal(T::parse_text(e.data.value())),
                onblur: handle_blur,
                onmounted: move |e| on_mounted.call(e),
                onkeydown: move |e| {
                    if e.key() == Key::Enter {
                        on_accept.call(value.cloned());
                    }
                },
                ..rest,
                {children}
            }

            if let Some(err) = value.read().validation_error() {
                span { class: "text-view-foregroundnegative", "{err}" }
            }
        }
    }
}
