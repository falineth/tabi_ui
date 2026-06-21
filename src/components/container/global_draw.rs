use dioxus::prelude::*;
use dioxus_free_icons::icons::md_navigation_icons::MdMenu;

use crate::icons::MdCircle;
use crate::{Button, ButtonVariant, Icon, IconShape};

#[component]
pub fn DrawAction<T: IconShape + Clone + PartialEq + 'static>(
    icon: T,
    #[props(default)] label: String,
    #[props(default)] onclick: EventHandler<Event<MouseData>>,
    #[props(default)] aria_expanded: bool,
) -> Element {
    let ctx = use_context::<GlobalDrawContext>();

    let extra_styles = use_memo(move || if ctx.open.cloned() { "" } else { "w-8" });

    rsx! {
        Button {
            onclick,
            class: "h-8 ps-0 pe-0 relative {extra_styles}",
            title: if !*ctx.open.read() { "{label}" },
            variant: ButtonVariant::GhostView,
            aria_expanded,
            div { class: "w-full flex items-center px-px",
                div { class: "h-7 w-7 flex justify-center items-center shrink-0",
                    Icon { class: "size-5", icon }
                }
                if ctx.open.cloned() {
                    span { class: "truncate", "{label}" }
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct GlobalDrawContext {
    pub open: Signal<bool>,
}

#[component]
pub fn GlobalDraw(
    children: Element,
    actions: Option<Element>,
    content: Option<Element>,
) -> Element {
    let mut open = use_signal(bool::default);

    use_context_provider(|| GlobalDrawContext { open });

    let handle_toggle_open = use_callback(move |_| {
        open.with_mut(|value| *value = !*value);
    });

    rsx! {
        div { class: "flex h-lvh w-lvw overflow-hidden",
            div {
                class: "flex flex-col justify-between h-full p-2 border-r overflow-hidden transition-all shrink-0",
                class: "bg-view-backgroundnormal text-view-foregroundnormal border-window-foregroundnormal/20",
                class: if *open.read() { "w-44" } else { "w-12" },
                div { class: "flex flex-col gap-1", {actions} }
                if let Some(content) = content {
                    div { class: "flex flex-col gap-1",
                        if open.cloned() {
                            {content}
                        }
                        DrawAction {
                            icon: MdMenu,
                            label: if *open.read() { "Close Sidebar" } else { "Open Sidebar" },
                            aria_expanded: *open.read(),
                            onclick: handle_toggle_open,
                        }
                    }
                }
            }
            {children}
        }
    }
}
