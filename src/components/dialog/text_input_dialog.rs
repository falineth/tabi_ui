use std::rc::Rc;

use dioxus::prelude::*;

use crate::components::{
    Button, Dialog, DialogContent, DialogFooter, DialogHeader, DialogTitle, DialogTrigger,
    TextInput, use_dialog_context,
};

#[component]
pub fn TextInputDialog(
    #[props(default)] disabled: ReadSignal<bool>,

    #[props(default)] title: ReadSignal<String>,
    #[props(default)] placeholder: ReadSignal<String>,
    #[props(default)] accept_text: ReadSignal<String>,

    #[props(default)] on_accept_value: Callback<String, Option<String>>,

    children: Element,
) -> Element {
    rsx! {
        Dialog {
            DialogTrigger { disabled, {children} }
            DialogContent {
                TextInputDialogCore {
                    title,
                    placeholder,
                    accept_text,
                    on_accept_value,
                }
            }
        }
    }
}

#[component]
pub fn TextInputDialogCore(
    #[props(default)] title: ReadSignal<String>,
    #[props(default)] placeholder: ReadSignal<String>,
    #[props(default)] accept_text: ReadSignal<String>,

    #[props(default)] on_accept_value: Callback<String, Option<String>>,
) -> Element {
    /*

       State

    */

    let mut entered_text = use_signal(|| Some(String::default()));

    let mut error_message: Signal<Option<String>> = use_signal(Option::default);

    let mut text_input: Signal<Option<Rc<MountedData>>> = use_signal(Option::default);

    let dialog_context = use_dialog_context();

    /*

       Callbacks

    */

    let handle_value_change = use_callback(move |value: String| entered_text.set(Some(value)));

    let handle_accept_value = use_callback(move |value: Option<String>| {
        if let Some(value) = value {
            if let Some(new_error_message) = on_accept_value.call(value.clone()) {
                error_message.set(Some(new_error_message));
            } else {
                dialog_context.set_open.call(false);
            }
        }
    });

    let handle_input_mounted = use_callback(move |e: Event<MountedData>| {
        text_input.set(Some(e.data()));
    });

    /*

       Effects

    */

    use_effect(move || {
        if *dialog_context.open.read() {
            entered_text.set(Some(String::default()));

            spawn(async move {
                if let Some(text_input) = text_input.read().as_ref() {
                    _ = text_input.set_focus(true).await;
                }
            });
        }
    });

    rsx! {

        DialogHeader {
            DialogTitle { "{title}" }
        }
        TextInput {
            value: entered_text,
            placeholder,
            on_value_change: handle_value_change,
            on_mounted: handle_input_mounted,
            on_accept: move |value| handle_accept_value.call(Some(value)),
        }
        if let Some(error_message) = error_message.read().as_ref() {
            span { class: "text-destructive", "* {error_message}" }
        }
        DialogFooter {
            Button {
                disabled: if let Some(entered_text) = entered_text.read().as_ref() && entered_text.is_empty() { "true" },
                onclick: move |_| handle_accept_value.call(entered_text.cloned()),
                "{accept_text}"
            }
        }
    }
}
