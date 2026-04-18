use std::rc::Rc;

use dioxus::prelude::*;

use crate::components::{OptionState, SelectContext};
use crate::hooks::{use_effect_cleanup, use_id_or, use_unique_id};

#[derive(Props, Clone, PartialEq)]
pub struct SelectNoneOptionProps {
    #[props(default)]
    pub id: ReadSignal<Option<String>>,

    pub index: ReadSignal<usize>,

    #[props(default)]
    pub aria_label: Option<String>,

    #[props(default)]
    pub class: String,

    #[props(extends = GlobalAttributes, extends = div)]
    pub rest: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn SelectNoneOption<T: Clone + PartialEq + 'static>(props: SelectNoneOptionProps) -> Element {
    /*

       Props

    */
    #![allow(unused_variables)]
    let SelectNoneOptionProps {
        id,
        index,
        aria_label,
        class,
        rest,
        children,
    } = props;

    /*

       State

    */
    let option_id = use_unique_id();

    /*

       Refs


    */
    let mut option_element: Signal<Option<Rc<MountedData>>> = use_signal(|| None);

    /*

       Hooks

    */
    let id = use_id_or(option_id, id);

    /*

       Context

    */
    let mut ctx = use_context::<SelectContext<T>>();

    /*

       Memos

    */
    use_effect_cleanup(move || {
        ctx.options.write().retain(|opt| opt.id != *id.read());
    });

    let selected = use_memo(move || ctx.value.read().is_none());

    let active = use_memo(move || {
        ctx.active_value.read().as_ref().map(|value| &value.id) == Some(&id.read())
    });

    /*

       Use Effects

    */
    use_effect(move || {
        let option_state = OptionState {
            index: index(),
            value: None,
            text_value: String::default(),
            id: id.cloned(),
        };

        ctx.options.write().push(option_state);
    });

    use_effect(move || {
        spawn(async move {
            if *active.read()
                && let Some(option_element) = option_element.read().as_ref()
            {
                _ = option_element
                    .scroll_to_with_options(ScrollToOptions {
                        behavior: ScrollBehavior::Smooth,
                        vertical: ScrollLogicalPosition::Nearest,
                        horizontal: ScrollLogicalPosition::Nearest,
                    })
                    .await;
            }
        });
    });

    rsx! {
        div {
            role: "option",
            id,
            class: "data-highlighted:bg-accent data-highlighted:text-accent-foreground not-data-[variant=destructive]:data-highlighted:**:text-accent-foreground min-h-7 gap-2 rounded-md px-2 py-1 text-xs/relaxed [&_svg:not([class*='size-'])]:size-3.5 relative flex w-full cursor-default items-center outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0",
            class: if index == 0 { "combo-option option-current" } else { "combo-option" },
            class: "{class}",
            aria_selected: selected(),
            "data-highlighted": if active() { true },
            onmounted: move |e| {
                option_element.set(Some(e.data()));
            },
            onclick: move |e| {
                e.prevent_default();

                ctx.active_value
                    .set(
                        Some(OptionState {
                            index: index(),
                            value: None,
                            text_value: String::default(),
                            id: id.cloned(),
                        }),
                    );
                ctx.set_value.call(None);
                ctx.set_menu_open.call(false);
            },
            {children}
            span { class: "pointer-events-none absolute right-2 flex items-center justify-center" }
        }
        hr { class: "m-1" }
    }
}
