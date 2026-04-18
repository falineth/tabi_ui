use std::rc::Rc;

use dioxus::prelude::*;

use crate::components::SelectContext;
use crate::hooks::use_window_size;

#[derive(Props, Clone, PartialEq)]
pub struct SelectTriggerProps {
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes, extends = button)]
    rest: Vec<Attribute>,
    children: Element,
}

#[component]
pub fn SelectTrigger<T: Clone + PartialEq + 'static>(props: SelectTriggerProps) -> Element {
    /*

        Props

    */
    let SelectTriggerProps {
        class,
        rest,
        children,
    } = props;

    let window_size = use_window_size();

    /*

        Refs

    */
    let mut combo_element: Signal<Option<Rc<MountedData>>> = use_signal(|| None);

    /*

        Context

    */
    let mut ctx = use_context::<SelectContext<T>>();

    use_effect(move || {
        if *ctx.menu_open.read()
            && combo_element.read().is_some()
            && window_size.read().height > -1.0
        {
            spawn(async move {
                if let Some(combo_element) = combo_element.read().as_ref()
                    && let Ok(rect) = combo_element.get_client_rect().await
                {
                    let current_rect = *ctx.combo_rect.read();

                    if current_rect != rect {
                        ctx.combo_rect.set(rect);
                    }
                }
            });
        }
    });

    rsx! {
        button {
            role: "combobox",
            class: "border-input data-placeholder:text-muted-foreground bg-input/20 dark:bg-input/30 dark:hover:bg-input/50 focus-visible:border-ring focus-visible:ring-ring/30 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:aria-invalid:border-destructive/50 gap-1.5 rounded-md border px-2 py-1.5 text-xs/relaxed transition-colors focus-visible:ring-2 aria-invalid:ring-2 data-[size=default]:h-7 data-[size=sm]:h-6 *:data-[slot=select-value]:flex *:data-[slot=select-value]:gap-1.5 [&_svg:not([class*='size-'])]:size-3.5 flex w-fit items-center justify-between whitespace-nowrap outline-none disabled:cursor-not-allowed disabled:opacity-50 *:data-[slot=select-value]:line-clamp-1 *:data-[slot=select-value]:items-center [&_svg]:pointer-events-none [&_svg]:shrink-0",
            class: "{class}",
            aria_controls: ctx.list_id,
            aria_expanded: ctx.menu_open,
            aria_haspopup: "listbox",
            aria_activedescendant: if let Some(active_value) = ctx.active_value.read().as_ref() { active_value.id.clone() },
            "data-placeholder": if ctx.value.read().is_none() { "" },
            "data-size": "default",
            "data-state": if *ctx.menu_open.read() { "open" } else { "closed" },
            onmounted: move |e| {
                combo_element.set(Some(e.data()));
            },
            onclick: move |_| {
                let new_open_state = !*ctx.menu_open.read();

                ctx.set_menu_open.call(new_open_state);
            },
            ..rest,
            {children}
        }
    }
}
