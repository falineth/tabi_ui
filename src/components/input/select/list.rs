use std::rc::Rc;

use dioxus::prelude::*;

use crate::components::input::select::list::PopupDirection::Down;
use crate::components::{PortalIn, SelectContext};
use crate::hooks::{use_id_or, use_unique_id};
use crate::utils::get_now;
use crate::{WindowSize, use_window_size};

const LIST_SPACING: f64 = 8.0;

#[derive(Props, Clone, PartialEq)]
pub struct SelectListProps {
    #[props(default)]
    pub id: ReadSignal<Option<String>>,

    #[props(default)]
    pub class: ReadSignal<String>,

    #[props(default)]
    pub direction: Option<PopupDirection>,

    #[props(extends = GlobalAttributes, extends = div)]
    pub rest: Vec<Attribute>,

    pub children: Element,
}

#[derive(Clone, Copy, PartialEq)]
pub enum PopupDirection {
    Up(f64),
    Down(f64),
}

impl Default for PopupDirection {
    fn default() -> Self {
        Down(50.0)
    }
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
        direction,
    } = props;

    /*

       State

    */

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
    let window_size = try_use_context::<Signal<WindowSize>>();
    let list_id = use_unique_id();
    let id = use_id_or(list_id, id);

    /*

       Memos

    */
    let position_style = use_memo(move || {
        let combo_rect = ctx.combo_rect.read();

        let list_height = ctx.list_height.read().unwrap_or_default();

        let direction: PopupDirection = if let Some(window_size) = window_size {
            let ws = window_size.cloned();
            debug!("window_size: {}h {}w", ws.height, ws.width);

            let space_below = window_size.read().height - combo_rect.max_y();
            let space_above = combo_rect.min_y();

            debug!("space_above: {space_above}");
            debug!("space_below: {space_below}");

            if list_height + LIST_SPACING < space_below {
                PopupDirection::Down(space_below - LIST_SPACING)
            } else if space_above > space_below {
                PopupDirection::Up(space_above - LIST_SPACING)
            } else {
                PopupDirection::Down(space_below - LIST_SPACING)
            }
        } else {
            direction.unwrap_or_default()
        };

        match direction {
            PopupDirection::Down(max_height) => {
                format!(
                    "top: calc(var(--spacing) + {}px); max-height: {}px;",
                    combo_rect.size.height, max_height,
                )
            }
            PopupDirection::Up(max_height) => {
                format!(
                    "bottom: calc(var(--spacing) + {}px); max-height: {}px;",
                    combo_rect.size.height, max_height,
                )
            }
        }
    });

    let popup_width_style = use_memo(move || {
        let combo_rect = ctx.combo_rect.read();

        format!("width: {}px;", combo_rect.size.width)
    });

    /*

       Callbacks

    */

    let handle_resize = use_callback(move |e: Event<ResizeData>| {
        if let Ok(size) = e.get_border_box_size()
            && *ctx.list_height.read() != Some(size.height)
        {
            ctx.list_height.set(Some(size.height));
        }
    });

    rsx! {
        div {
            class: "z-50 absolute left-0",
            style: if *ctx.menu_open.read() { "{position_style}" } else { "top: auto; bottom: auto;" },
            ..rest,
            div {
                class: "bg-window-backgroundnormal text-window-foregroundnormal border-window-foregroundnormal/20 ring-window-foregroundnormal/5 *:data-[slot=input-group]:bg-input/30 *:data-[slot=input-group]:border-input/30",
                class: "transition-none data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 cn-menu-target group/combobox-content relative max-h-72 overflow-hidden rounded-sm shadow-md ring-1 border duration-100 *:data-[slot=input-group]:m-1 *:data-[slot=input-group]:mb-0 *:data-[slot=input-group]:h-8 *:data-[slot=input-group]:shadow-none",
                style: if !*ctx.menu_open.read() { "opacity: 0%; height: 0; {popup_width_style}" } else { "opacity: 100%; height: auto; {popup_width_style}" },
                div {
                    id,
                    role: "listbox",
                    class: "no-scrollbar max-h-72 scroll-py-1 overflow-y-auto overscroll-contain p-1 data-empty:p-0",
                    class: "top-0",
                    "aria-labelledby": "combo1-label",
                    onresize: handle_resize,
                    onmounted: move |e| {
                        list_element.set(Some(e.data()));
                    },
                    div { class: "{class}", {children} }
                }
            }
        }
    }
}
