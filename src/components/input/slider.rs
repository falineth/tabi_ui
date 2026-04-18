use std::rc::Rc;

use dioxus::logger::tracing;
use dioxus::prelude::*;

use crate::hooks::use_controlled;
use crate::utils::nanoid;

#[component]
pub fn Slider(
    #[props(default)] class: String,

    #[props(default)] default_value: f32,
    #[props(default)] value: ReadSignal<Option<f32>>,
    #[props(default)] on_value_change: Callback<f32>,

    #[props(default)] min: f32,
    #[props(default = 100.0)] max: f32,

    #[props(default = 1.0)] small_step: f32,
    #[props(default = 10.0)] large_step: f32,
) -> Element {
    /*

       State

    */

    let (value, set_value_internal) = use_controlled(value, default_value, on_value_change);

    let thumb_id = use_signal(nanoid);

    let mut slider_width: Signal<Option<f64>> = use_signal(Option::default);

    let mut drag_start_position: Signal<Option<f64>> = use_signal(Option::default);

    let mut dragging_value: Signal<Option<f32>> = use_signal(Option::default);

    let mut thumb_ref: Signal<Option<Rc<MountedData>>> = use_signal(Option::default);

    /*

       Memos

    */

    let display_position = use_memo(move || {
        let display_value = dragging_value.read().unwrap_or(*value.read());

        let value_range = max - min;

        return 100.0 * (display_value - min) / value_range;
    });

    /*

       Callbacks

    */

    let handle_keydown = use_callback(move |event: Event<KeyboardData>| {
        if drag_start_position.read().is_some() {
            if event.key() == Key::Escape {
                dragging_value.set(None);
                drag_start_position.set(None);

                event.prevent_default();
            }

            return;
        };

        match event.key() {
            Key::Home => {
                set_value_internal.call(min);
                event.prevent_default();
            }
            Key::End => {
                set_value_internal.call(max);
                event.prevent_default();
            }
            Key::PageUp => {
                set_value_internal.call((*value.read() + large_step).clamp(min, max));
                event.prevent_default();
            }
            Key::PageDown => {
                set_value_internal.call((*value.read() - large_step).clamp(min, max));
                event.prevent_default();
            }
            Key::ArrowRight | Key::ArrowUp => {
                set_value_internal.call((*value.read() + small_step).clamp(min, max));
                event.prevent_default();
            }
            Key::ArrowLeft | Key::ArrowDown => {
                set_value_internal.call((*value.read() - small_step).clamp(min, max));
                event.prevent_default();
            }
            _ => {}
        }
    });

    let handle_pointerdown = use_callback(move |event: Event<PointerData>| {
        drag_start_position.set(Some(event.client_coordinates().x));

        let pointer_id = event.data().pointer_id();

        let thumb_id = thumb_id.cloned();

        event.prevent_default();

        spawn(async move {
            if let Some(thumb_ref) = &*thumb_ref.read() {
                _ = thumb_ref.set_focus(true).await;
            }

            _ = document::eval(&format!(
                "document.getElementById('{}').setPointerCapture({})",
                thumb_id.as_str(),
                pointer_id
            ))
            .await;
        });
    });

    let handle_pointermove = use_callback(move |event: Event<PointerData>| {
        let Some(slider_width) = *slider_width.read() else {
            return;
        };

        let Some(start) = *drag_start_position.read() else {
            return;
        };

        let slide_amount = event.client_coordinates().x - start;

        let value_range: f64 = (max - min).into();

        let value_change = value_range * slide_amount / slider_width;

        #[allow(clippy::cast_possible_truncation)]
        let new_value = (*value.read() + (value_change as f32)).clamp(min, max);

        dragging_value.set(Some(new_value));
    });

    let handle_pointerup = use_callback(move |event: Event<PointerData>| {
        let pointer_id = event.data().pointer_id();

        let thumb_id = thumb_id.cloned();

        if let Some(new_value) = *dragging_value.read() {
            set_value_internal.call(new_value);
        }

        dragging_value.set(None);
        drag_start_position.set(None);

        spawn(async move {
            _ = document::eval(&format!(
                r#"
const thumb = document.getElementById('{}');
const pointerId = {};
if (thumb && thumb.hasPointerCapture(pointerId)) {{
    thumb.releasePointerCapture(pointerId);
}}
                "#,
                thumb_id.as_str(),
                pointer_id
            ))
            .await;
        });
    });

    let handle_resize = use_callback(move |event: Event<ResizeData>| {
        let Ok(size) = event.get_content_box_size() else {
            return;
        };

        slider_width.set(Some(size.width));
    });

    let handle_thumb_mounted = use_callback(move |event: Event<MountedData>| {
        thumb_ref.set(Some(event.data()));
    });

    /*

       Elements

    */

    rsx! {
        span {
            dir: "ltr",
            "data-orientation": "horizontal",
            aria_disabled: false,
            "data-slot": "slider",
            class: "relative flex w-full touch-none items-center select-none data-disabled:opacity-50 data-vertical:h-full data-vertical:min-h-40 data-vertical:w-auto data-vertical:flex-col",
            class: "{class}",
            onresize: handle_resize,
            span {
                "data-orientation": "horizontal",
                "data-slot": "slider-track",
                class: "relative grow overflow-hidden rounded-md bg-muted data-horizontal:h-1 data-horizontal:w-full data-vertical:h-full data-vertical:w-1",
                span {
                    "data-orientation": "horizontal",
                    "data-slot": "slider-range",
                    class: "absolute bg-primary select-none data-horizontal:h-full data-vertical:w-full",
                    style: "left: 0%; right: {100.0 - *display_position.read()}%",
                }
            }
            span {
                "data-slot": "slider-thumb",
                style: "transform: translateX(-50%); position: absolute; left: calc({*display_position.read()}% + 0px);",
                span {
                    id: thumb_id,
                    role: "slider",
                    tabindex: "0",
                    aria_valuemin: min,
                    aria_valuemax: max,
                    aria_orientation: "horizontal",
                    aria_valuenow: *value.read(),
                    "data-orientation": "horizontal",
                    "data-slot": "slider-thumb",
                    "data-radix-collection-item": "",
                    class: "relative block size-3 shrink-0 rounded-md border border-ring bg-white ring-ring/30 transition-[color,box-shadow] select-none after:absolute after:-inset-2 hover:ring-2 focus-visible:ring-2 focus-visible:outline-hidden active:ring-2 disabled:pointer-events-none disabled:opacity-50",
                    onmounted: handle_thumb_mounted,
                    onkeydown: handle_keydown,
                    onpointerdown: handle_pointerdown,
                    onpointermove: handle_pointermove,
                    onpointerup: handle_pointerup,
                }
            }
        }
    }
}
