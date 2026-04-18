use std::rc::Rc;

use dioxus::html::geometry::Pixels;
use dioxus::html::geometry::euclid::Size2D;
use dioxus::prelude::*;

use crate::components::{PortalIn, SelectContext};
use crate::hooks::{use_id_or, use_modal_offset_context, use_unique_id, use_window_size};
use crate::utils::get_now;

#[derive(Props, Clone, PartialEq)]
pub struct SelectListProps {
    #[props(default)]
    pub id: ReadSignal<Option<String>>,

    #[props(default)]
    pub class: ReadSignal<String>,

    #[props(extends = GlobalAttributes, extends = div)]
    pub rest: Vec<Attribute>,

    pub children: Element,
}

#[allow(unused)]
#[derive(Debug)]
enum SelectAction {
    Close,
    CloseSelect,
    First,
    Last,
    Next,
    Open,
    Previous,
    Select,
    Type(String),
}

fn map_key_to_action(key: Key, modifiers: Modifiers, menu_open: bool) -> Option<SelectAction> {
    let is_space = match &key {
        Key::Character(characters) => matches!(characters.as_str(), " "),
        _ => false,
    };

    if !menu_open && (is_space || matches!(key, Key::ArrowDown | Key::ArrowUp | Key::Enter)) {
        return Some(SelectAction::Open);
    }

    match key {
        Key::Home => return Some(SelectAction::First),
        Key::End => return Some(SelectAction::Last),
        _ => {}
    }

    if modifiers.is_empty()
        && !is_space
        && let Key::Character(text) = key
    {
        return Some(SelectAction::Type(text));
    }

    if menu_open {
        if matches!(key, Key::ArrowUp) && modifiers.alt() {
            return Some(SelectAction::CloseSelect);
        }

        if matches!(key, Key::ArrowDown) && !modifiers.alt() {
            return Some(SelectAction::Next);
        }

        if is_space {
            return Some(SelectAction::CloseSelect);
        }

        return match key {
            Key::ArrowUp => Some(SelectAction::Previous),
            Key::Escape => Some(SelectAction::Close),
            Key::Enter => Some(SelectAction::CloseSelect),
            _ => None,
        };
    }

    return None;
}

#[component]
pub fn SelectList<T: Clone + PartialEq + 'static>(props: SelectListProps) -> Element {
    /*

       Props

    */
    let SelectListProps {
        id,
        class,
        rest,
        children,
    } = props;

    /*

       State

    */
    let mut content_size: Signal<Size2D<f64, Pixels>> = use_signal(Size2D::default);

    /*

        Context

    */
    let mut ctx = use_context::<SelectContext<T>>();

    /*

       Refs

    */
    let mut list_element: Signal<Option<Rc<MountedData>>> = use_signal(|| None);

    /*

       Hooks

    */
    let window_size = use_window_size();
    let modal_offset = use_modal_offset_context();
    let list_id = use_unique_id();
    let id = use_id_or(list_id, id);

    /*

       Memos

    */

    let below = use_memo(move || {
        let combo_rect = ctx.combo_rect.read();
        let list_size = content_size.read();

        window_size.read().height - combo_rect.max_y() > list_size.height
            || window_size.read().height - combo_rect.max_y() > combo_rect.min_y()
    });

    let transform = use_memo(move || {
        let combo_rect = ctx.combo_rect.read();

        if *below.read() {
            let x = combo_rect.min_x() - modal_offset.read().x;
            let y = combo_rect.max_y() - modal_offset.read().y + 6f64;

            format!("translate({x}px, {y}px)")
        } else {
            let x = combo_rect.min_x() - modal_offset.read().x;
            let y = combo_rect.min_y() - modal_offset.read().y - 6f64;

            format!("translate({x}px, calc({y}px - 100%))")
        }
    });

    let available = use_memo(move || {
        let combo_rect = ctx.combo_rect.read();

        if *below.read() {
            format!(
                "--available-width: {}px; --available-height: {}px",
                window_size.read().width - combo_rect.min_x(),
                window_size.read().height - (combo_rect.max_y() + 6f64)
            )
        } else {
            format!(
                "--available-width: {}px; --available-height: {}px",
                window_size.read().width - combo_rect.min_x(),
                combo_rect.min_y() + 6f64
            )
        }
    });

    /*

       Callbacks

    */
    let do_action = use_callback(move |params: (SelectAction, Event<KeyboardData>)| {
        let (action, event) = params;

        let active_value = ctx.active_value.read().cloned();

        if matches!(action, SelectAction::Last | SelectAction::First) {
            ctx.set_menu_open.call(true);
        }

        match action {
            SelectAction::First => {
                event.prevent_default();

                let Some(active_value) = active_value else {
                    return;
                };

                if let Some(first) = ctx.options.read().iter().min_by_key(|option| option.index)
                    && first.index != active_value.index
                {
                    ctx.active_value.set(Some(first.clone()));
                }
            }
            SelectAction::Previous => {
                event.prevent_default();

                let Some(active_value) = active_value else {
                    return;
                };

                if let Some(prev) = ctx
                    .options
                    .read()
                    .iter()
                    .filter(|option| option.index < active_value.index)
                    .max_by_key(|option| option.index)
                {
                    ctx.active_value.set(Some(prev.clone()));
                }
            }
            SelectAction::Next => {
                event.prevent_default();

                let Some(active_value) = active_value else {
                    return;
                };

                if let Some(next) = ctx
                    .options
                    .read()
                    .iter()
                    .filter(|option| option.index > active_value.index)
                    .min_by_key(|option| option.index)
                {
                    ctx.active_value.set(Some(next.clone()));
                }
            }
            SelectAction::Last => {
                event.prevent_default();

                let Some(active_value) = active_value else {
                    return;
                };

                if let Some(last) = ctx.options.read().iter().max_by_key(|option| option.index)
                    && last.index != active_value.index
                {
                    ctx.active_value.set(Some(last.clone()));
                }
            }
            SelectAction::CloseSelect => {
                event.prevent_default();

                if let Some(active_value) = active_value {
                    ctx.active_value.set(Some(active_value.clone()));
                }

                ctx.set_menu_open.call(false);
            }
            SelectAction::Close => {
                event.prevent_default();
                ctx.set_menu_open.call(false);
            }
            SelectAction::Type(text) => {
                ctx.set_menu_open.call(true);

                let mut search_string = ctx.search_string.write();

                if get_now() - search_string.age > 500f64 {
                    search_string.text.clear();
                }

                let lowercase_text = text.to_lowercase();

                search_string.text.push_str(&lowercase_text);
                search_string.age = get_now();

                let matching_option = ctx
                    .options
                    .read()
                    .iter()
                    .filter(|option| {
                        option
                            .text_value
                            .to_lowercase()
                            .starts_with(search_string.text.as_str())
                    })
                    .min_by_key(|option| option.index)
                    .cloned();

                if let Some(matching_option) = matching_option {
                    let matching_option = Some(matching_option.clone());

                    ctx.active_value.set(matching_option);
                } else {
                    search_string.text = lowercase_text;
                }
            }
            SelectAction::Open => {
                event.prevent_default();
                ctx.set_menu_open.call(true);
            }
            _ => {}
        }
    });

    rsx! {
        PortalIn { portal: ctx.portal,
            div {
                class: "absolute left-0 top-0 z-50",
                style: "--anchor-width: {ctx.combo_rect.read().size.width}px; --anchor-height: {ctx.combo_rect.read().size.height}px;",
                style: "{available}",
                transform,
                ..rest,
                div {
                    class: "bg-popover text-popover-foreground data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 *:data-[slot=input-group]:bg-input/30 *:data-[slot=input-group]:border-input/30 data-[side=inline-start]:slide-in-from-right-2 data-[side=inline-end]:slide-in-from-left-2 cn-menu-target group/combobox-content relative max-h-72 w-(--anchor-width) max-w-(--available-width) min-w-[calc(var(--anchor-width)+--spacing(7))] origin-(--transform-origin) overflow-hidden rounded-lg shadow-md ring-1 duration-100 data-[chips=true]:min-w-(--anchor-width) *:data-[slot=input-group]:m-1 *:data-[slot=input-group]:mb-0 *:data-[slot=input-group]:h-8 *:data-[slot=input-group]:shadow-none",
                    class: if !*ctx.menu_open.read() { "hidden" },
                    div {
                        id,
                        role: "listbox",
                        class: "no-scrollbar max-h-[min(calc(--spacing(72)---spacing(9)),calc(var(--available-height)---spacing(9)))] scroll-py-1 overflow-y-auto overscroll-contain p-1 data-empty:p-0",
                        class: "top-0",
                        "aria-labelledby": "combo1-label",
                        tabindex: 0,
                        onmounted: move |e| {
                            list_element.set(Some(e.data()));
                        },
                        onvisible: move |_| {
                            spawn(async move {
                                if let Some(list_element) = list_element() {
                                    _ = list_element.set_focus(true).await;
                                }
                            });

                        },
                        onblur: move |_| {
                            ctx.set_menu_open.call(false);
                        },
                        onkeydown: move |e| {
                            let action = map_key_to_action(e.key(), e.modifiers(), *ctx.menu_open.read());

                            if let Some(action) = action {
                                do_action.call((action, e));
                            }
                        },
                        div {
                            class: "{class}",
                            onresize: move |e| {
                                let mut current_content_size = content_size.write();

                                if let Ok(content_size) = e.get_content_box_size()
                                    && content_size != Size2D::default()
                                    && *current_content_size != content_size
                                {
                                    *current_content_size = content_size;
                                }
                            },
                            {children}
                        }
                    }
                }
            }
        }
    }
}
