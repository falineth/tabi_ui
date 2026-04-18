use dioxus::prelude::*;

#[derive(Default, Clone, PartialEq, Debug)]
pub struct ModalOffset {
    pub x: f64,
    pub y: f64,
}

pub fn use_modal_offset_context() -> Signal<ModalOffset> {
    use_context::<Signal<ModalOffset>>()
}
