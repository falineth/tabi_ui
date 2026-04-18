use std::rc::Rc;

use dioxus::prelude::*;

use crate::components::{
    Button, ButtonVariant, Dialog, DialogContent, DialogFooter, DialogHeader, DialogTitle,
    DialogTrigger, TextInput, use_dialog_context,
};
use crate::hooks::use_controlled;

#[derive(Clone, PartialEq)]
pub struct ButtonDetails {
    pub text: String,
    pub variant: ButtonVariant,
    pub handler: Option<Callback<()>>,
}

#[component]
pub fn MessageDialog(
    #[props(default)] disabled: ReadSignal<bool>,

    #[props(default)] title: ReadSignal<String>,
    #[props(default)] message: ReadSignal<String>,
    #[props(default)] placeholder: ReadSignal<String>,
    #[props(default)] accept_text: ReadSignal<String>,

    #[props(default)] open: ReadSignal<Option<bool>>,
    #[props(default)] default_open: bool,
    #[props(default)] on_open_change: Callback<bool>,

    #[props(default)] buttons: ReadSignal<Vec<ButtonDetails>>,

    children: Element,
) -> Element {
    let (open, set_open) = use_controlled::<bool>(open, default_open, on_open_change);

    rsx! {
        Dialog { open: open(), on_open_change: set_open,
            DialogTrigger { disabled, {children} }
            DialogContent { full_width: false,
                MessageDialogCore {
                    title,
                    message,
                    placeholder,
                    accept_text,
                    buttons,
                }
            }
        }
    }
}

#[component]
pub fn MessageDialogCore(
    #[props(default)] title: ReadSignal<String>,
    #[props(default)] message: ReadSignal<String>,
    #[props(default)] placeholder: ReadSignal<String>,
    #[props(default)] accept_text: ReadSignal<String>,

    #[props(default)] buttons: ReadSignal<Vec<ButtonDetails>>,

    children: Element,
) -> Element {
    rsx! {
        DialogHeader {
            DialogTitle { "{title}" }
        }
        span { "{message}" }
        div { class: "flex justify-end",
            for button in buttons.iter() {
                MessageDialogButton { button: button.cloned() }
            }
        }
    }
}

#[component]
pub fn MessageDialogButton(button: ReadSignal<ButtonDetails>) -> Element {
    let dialog_context = use_dialog_context();

    let handle_click = use_callback(move |_| {
        if let Some(handler) = button.read().handler {
            handler.call(());
        }

        dialog_context.set_open.call(false);
    });

    rsx! {
        Button { onclick: handle_click, variant: button.read().variant, "{button.read().text}" }
    }
}
