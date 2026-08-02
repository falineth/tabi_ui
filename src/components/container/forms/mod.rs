use dioxus::prelude::*;

#[component]
pub fn FormSection(#[props(default)] label: ReadSignal<String>) -> Element {
    rsx! {
        tr {
            td {
                class: "text-center text-lg font-semibold pt-6 select-none border-b border-view-foregroundnormal/20",
                colspan: "2",
                "{label}"
            }
        }
    }
}

#[component]
pub fn FormEntry(
    #[props(default)] label: ReadSignal<String>,
    #[props(default)] class: ReadSignal<String>,
    children: Element,
) -> Element {
    rsx! {
        tr {
            td { class: "text-end pt-1 pe-2 select-none align-top", "{label}" }
            td { class: "pt-1", class: "{class}", {children} }
        }
    }
}
