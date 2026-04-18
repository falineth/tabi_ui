use dioxus::prelude::*;

use crate::components::SelectContext;

#[derive(Props, Clone, PartialEq)]
pub struct SelectValueProps {
    #[props(default)]
    placeholder: ReadSignal<String>,
    #[props(default)]
    class: String,
    #[props(extends = GlobalAttributes, extends = span)]
    rest: Vec<Attribute>,
}

#[component]
pub fn SelectValue<T: Clone + PartialEq + 'static>(props: SelectValueProps) -> Element {
    let SelectValueProps {
        placeholder,
        class,
        rest,
    } = props;

    let ctx = use_context::<SelectContext<T>>();

    let display_value = use_memo(move || {
        let value = ctx.value.read();

        let selected_text_value = value.as_ref().and_then(|value| {
            ctx.options
                .iter()
                .find(|option| option.value.as_ref() == Some(value))
                .map(|option| option.text_value.clone())
        });

        return selected_text_value.unwrap_or_else(|| placeholder.cloned());
    });

    rsx! {
        span { class, ..rest, {display_value} }
    }
}
