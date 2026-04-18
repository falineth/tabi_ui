// TODO:
// - Scroll Management
//     - Better indicate drop down is scrollable
// - Focus combobox on completing selection
//     - Receive keyboard events when closed

mod context;
mod list;
mod none_option;
mod option;
mod trigger;
mod value;

use std::rc::Rc;

use dioxus::html::geometry::Pixels;
use dioxus::html::geometry::euclid::Rect;
use dioxus::prelude::*;

pub use crate::components::input::select::context::*;
pub use crate::components::input::select::list::*;
pub use crate::components::input::select::none_option::*;
pub use crate::components::input::select::option::*;
pub use crate::components::input::select::trigger::*;
pub use crate::components::input::select::value::*;
use crate::components::{PortalOut, use_portal};
use crate::hooks::use_controlled;

#[derive(Props, Clone, PartialEq)]
pub struct SelectProps<T: Clone + PartialEq + 'static = String> {
    #[props(default)]
    pub value: ReadSignal<Option<Option<T>>>,

    #[props(default)]
    pub default_value: Option<T>,

    #[props(default)]
    pub on_value_change: Callback<Option<T>>,

    #[props(default)]
    class: String,

    #[props(extends = GlobalAttributes, extends = button)]
    rest: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn Select<T: Clone + PartialEq + 'static>(props: SelectProps<T>) -> Element {
    /*

       Props

    */
    let SelectProps {
        value,
        default_value,
        on_value_change,
        class,
        rest,
        children,
    } = props;

    /*

       Uncontrolled


    */
    let (value, set_value_internal) = use_controlled(value, default_value, on_value_change);

    /*

       State

    */
    let active_value: Signal<Option<OptionState<T>>> = use_signal(|| None);
    let options: Signal<Vec<OptionState<T>>> = use_signal(Vec::default);
    let mut menu_open: Signal<bool> = use_signal(|| false);
    let list_id: Signal<Option<String>> = use_signal(|| None);
    let combo_rect: Signal<Rect<f64, Pixels>> = use_signal(Rect::default);
    let search_string: Signal<SearchText> = use_signal(SearchText::default);

    /*

       Refs

    */
    let combo_element: Signal<Option<Rc<MountedData>>> = use_signal(|| None);

    /*

       Callbacks

    */
    let set_value = use_callback(move |value: Option<T>| {
        set_value_internal.call(value);
    });

    let set_menu_open = use_callback(move |value: bool| {
        spawn(async move {
            if *menu_open.read() == value {
                return;
            }

            menu_open.set(value);

            if let Some(combo_element) = combo_element.read().as_ref() {
                _ = combo_element.scroll_to(ScrollBehavior::Smooth).await;
            }
        });
    });

    /*

       Hooks

    */

    let portal = use_portal();

    /*

       Context

    */
    use_context_provider(|| SelectContext {
        active_value,
        value,
        set_value,
        options,
        menu_open: menu_open.into(),
        set_menu_open,
        list_id,
        combo_rect,
        search_string,
        portal,
    });

    rsx! {
        div {
            "data-slot": "select",
            class: "w-auto",
            class: "{class}",
            tabindex: 0,
            ..rest,
            {children}
        }
        PortalOut { portal }
    }
}
