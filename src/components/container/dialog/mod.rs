// TODO: ensure focus cannot leave dialog

use std::rc::Rc;

use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::bs_icons::BsX;

use crate::components::{
    Button, ButtonSize, ButtonVariant, PortalId, PortalIn, PortalOut, use_portal,
};
use crate::hooks::{ModalOffset, use_controlled, use_modal_offset_context, use_window_size};

#[derive(Clone)]
pub struct DialogContext {
    pub portal: PortalId,
    pub open: Memo<bool>,
    pub set_open: Callback<bool>,
}

pub fn use_dialog_context() -> DialogContext {
    use_context::<DialogContext>()
}

#[derive(Props, Clone, PartialEq)]
pub struct DialogProps {
    #[props(default)]
    open: ReadSignal<Option<bool>>,
    #[props(default)]
    default_open: bool,
    #[props(default)]
    on_open_change: Callback<bool>,
    children: Element,
}

#[component]
pub fn Dialog(props: DialogProps) -> Element {
    let DialogProps {
        open,
        default_open,
        on_open_change,
        children,
    } = props;

    let (open, set_open) = use_controlled::<bool>(open, default_open, on_open_change);

    let portal = use_portal();

    use_context_provider(|| DialogContext {
        portal,
        open,
        set_open,
    });

    use_context_provider(|| Signal::new(ModalOffset::default()));

    rsx! {
        div { {children} }

        PortalOut { portal }
    }
}

#[component]
pub fn DialogOverlay(
    #[props(default)] class: String,
    #[props(extends = GlobalAttributes, extends = div)] rest: Vec<Attribute>,
    #[props(default)] allow_dismiss: bool,
    children: Element,
) -> Element {
    let dialog_context = use_dialog_context();

    let handle_dismiss_dialog = move |_| {
        if allow_dismiss {
            dialog_context.set_open.call(false);
        }
    };

    rsx! {
        div {
            "data-slot": "dialog-overlay",
            "data-state": if dialog_context.open.read().eq(&true) { "open" } else { "closed" },
            class: "hidden data-open:block data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 bg-black/80 duration-100 supports-backdrop-filter:backdrop-blur-xs fixed inset-0 isolate z-50",
            class: "{class}",
            onclick: handle_dismiss_dialog,
            ..rest,
            {children}
        }
    }
}

#[component]
pub fn DialogContent(
    #[props(default)] class: String,
    #[props(default)] allow_dismiss: bool,
    #[props(default = true)] full_width: bool,
    #[props(extends = GlobalAttributes, extends = div)] rest: Vec<Attribute>,
    children: Element,
) -> Element {
    let dialog_context = use_dialog_context();

    let mut modal_offset = use_modal_offset_context();

    let window_size = use_window_size();

    /*

        Refs

    */
    let mut combo_element: Signal<Option<Rc<MountedData>>> = use_signal(|| None);

    use_effect(move || {
        if *dialog_context.open.read()
            && combo_element.read().is_some()
            && window_size.read().height > -1.0
        {
            spawn(async move {
                if let Some(combo_element) = combo_element.read().as_ref()
                    && let Ok(rect) = combo_element.get_client_rect().await
                {
                    let current_offset = modal_offset.read();

                    if current_offset.x != rect.min_x() || current_offset.y != rect.min_y() {
                        drop(current_offset);
                        let mut modal_offset = modal_offset.write();
                        modal_offset.x = rect.min_x();
                        modal_offset.y = rect.min_y();
                    }
                }
            });
        }
    });

    rsx! {
        PortalIn { portal: dialog_context.portal,
            DialogOverlay { allow_dismiss }
            div {
                "data-slot": "dialog-content",
                "data-state": if dialog_context.open.read().eq(&true) { "open" } else { "closed" },
                class: "hidden data-open:grid bg-background data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 ring-foreground/10 max-w-[calc(100%-2rem)] gap-4 rounded-xl p-4 text-xs/relaxed ring-1 duration-100 sm:max-w-sm md:max-w-md lg:max-w-lg xl:max-w-xl 2xl:max-w-2xl fixed top-1/2 left-1/2 z-50 -translate-x-1/2 -translate-y-1/2 outline-none",
                class: if full_width { "w-full" },
                onmounted: move |e| {
                    combo_element.set(Some(e.data()));
                },
                ..rest,
                {children}
                DialogClose {
                    "data-slot": "dialog-close",
                    class: "absolute top-2 right-2",
                    Button {
                        variant: ButtonVariant::Ghost,
                        size: ButtonSize::IconSM,
                        Icon { icon: BsX }
                        span { class: "sr-only", "Close" }
                    }
                }
            }
        }
    }
}

#[component]
pub fn DialogHeader(
    #[props(default)] class: String,
    #[props(extends = GlobalAttributes, extends = div)] rest: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        div {
            "data-slot": "dialog-header",
            class: "gap-1 flex flex-col",
            class: "{class}",
            ..rest,
            {children}
        }
    }
}

#[component]
pub fn DialogTitle(
    #[props(default)] class: String,
    #[props(extends = GlobalAttributes, extends = div)] rest: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        div {
            "data-slot": "dialog-title",
            class: "text-sm font-medium",
            class: "{class}",
            ..rest,
            {children}
        }
    }
}

#[component]
pub fn DialogDescription(
    #[props(default)] class: String,
    #[props(extends = GlobalAttributes, extends = div)] rest: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        div {
            "data-slot": "dialog-description",
            class: "text-muted-foreground *:[a]:hover:text-foreground text-xs/relaxed *:[a]:underline *:[a]:underline-offset-3",
            class: "{class}",
            ..rest,
            {children}
        }
    }
}

#[component]
pub fn DialogFooter(
    #[props(default)] show_close_button: bool,
    #[props(default)] class: String,
    #[props(extends = GlobalAttributes, extends = div)] rest: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        div {
            "data-slot": "dialog-footer",
            class: "flex flex-col-reverse gap-2 sm:flex-row sm:justify-end",
            class: "{class}",
            ..rest,
            {children}
            if show_close_button {
                Button { variant: ButtonVariant::Outline, "Close" }
            }
        }
    }
}

#[component]
pub fn DialogClose(
    #[props(default)] show_close_button: bool,
    #[props(default)] class: String,
    #[props(extends = GlobalAttributes, extends = div)] rest: Vec<Attribute>,
    children: Element,
) -> Element {
    let dialog_context = use_dialog_context();

    let handle_open_dialog = move |_| {
        dialog_context.set_open.call(false);
    };

    rsx! {
        div {
            "data-slot": "dialog-close",
            class: "{class}",
            onclick: handle_open_dialog,
            ..rest,
            {children}
        }
    }
}

#[component]
pub fn DialogTrigger(
    #[props(default)] disabled: ReadSignal<bool>,
    #[props(default)] show_close_button: ReadSignal<bool>,
    #[props(default)] class: ReadSignal<String>,
    #[props(extends = GlobalAttributes, extends = div)] rest: Vec<Attribute>,
    children: Element,
) -> Element {
    let dialog_context = use_dialog_context();

    let handle_open_dialog = move |_| {
        if !*disabled.read() {
            dialog_context.set_open.call(true);
        }
    };

    rsx! {
        div { class: "{class}", onclick: handle_open_dialog, ..rest, {children} }
    }
}
