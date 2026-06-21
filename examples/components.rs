use std::fmt::Display;

use dioxus::prelude::*;
use tabi_ui::ThemeContext;
use tabi_ui::components::*;
use tabi_ui::icons::MdAdd;

use crate::ValidatedNumber::{EnteredNumber, InvalidValue, Number};

const TAILWIND_CSS: Asset = asset!("../assets/tailwind.css");

#[cfg(feature = "desktop")]
#[tokio::main(flavor = "multi_thread")]
async fn main() {
    use dioxus::desktop::{Config, WindowBuilder};
    use tabi_ui::get_window_background_color;

    let theme_context = ThemeContext::init().await;

    let mut config = Config::default()
        .with_background_color(theme_context.bg_color)
        .with_window(WindowBuilder::new().with_title("Component Example"));

    if cfg!(not(debug_assertions)) {
        config = config.with_menu(None);
    }

    LaunchBuilder::new()
        .with_cfg(config)
        .with_context(theme_context)
        .launch(App);
}

#[cfg(feature = "web")]
fn main() {
    dioxus::launch(App);
}

#[derive(Clone, PartialEq)]
pub enum ValidatedNumber<const MIN: i32 = 0, const MAX: i32 = 1000, const DECIMALS: usize = 2> {
    Number(f32),
    EnteredNumber(String, f32),
    InvalidValue(String, String),
}

impl<const MIN: i32, const MAX: i32, const DECIMALS: usize> Default
    for ValidatedNumber<MIN, MAX, DECIMALS>
{
    fn default() -> Self {
        Number(0.0)
    }
}

impl<const MIN: i32, const MAX: i32, const DECIMALS: usize> Display
    for ValidatedNumber<MIN, MAX, DECIMALS>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Number(value) => write!(f, "{:.DECIMALS$}", value),
            EnteredNumber(text, _value) => write!(f, "{}", text),
            InvalidValue(text, _err) => write!(f, "{}", text),
        }
    }
}

impl<const MIN: i32, const MAX: i32, const DECIMALS: usize> ValidatableValue
    for ValidatedNumber<MIN, MAX, DECIMALS>
{
    fn validation_error(&self) -> Option<String> {
        if let InvalidValue(_text, error) = self {
            return Some(error.clone());
        }

        None
    }

    fn finalize_value(self) -> Self {
        if let EnteredNumber(_text, value) = self {
            let multiplier = 10.0f32.powf(DECIMALS as f32);
            return Number((value * multiplier).round() / multiplier);
        }

        self
    }

    fn parse_text(text: String) -> Self {
        if text.chars().all(|c| c.is_whitespace()) {
            return InvalidValue(text, "Required".to_string());
        }

        let valid_value = match text.parse::<f32>() {
            Ok(value) => value,
            Err(_err) => return InvalidValue(text, "Enter a valid number".to_string()),
        };

        let max = MAX as f32;

        if valid_value > max {
            return InvalidValue(text, format!("Maximum value {max:.DECIMALS$}"));
        }

        let min = MIN as f32;

        if valid_value < min {
            return InvalidValue(text, format!("Minimum value {min:.DECIMALS$}"));
        }

        EnteredNumber(text, valid_value)
    }
}

#[component]
fn App() -> Element {
    let role_list = use_store(|| {
        vec![
            "Developer".to_owned(),
            "Designer".to_owned(),
            "Manager".to_owned(),
            "Other".to_owned(),
        ]
    });

    let framework_list = use_store(|| {
        vec![
            "Next.js".to_owned(),
            "SvelteKit".to_owned(),
            "Nuxt.js".to_owned(),
            "Remix".to_owned(),
            "Astro".to_owned(),
        ]
    });

    let mut selected_role = use_signal(|| Some(None));

    let mut selected_framework = use_signal(|| Some(None));

    let mut subscribe = use_signal(|| Some(false));

    let handle_subscribe_changed = use_callback(move |value: bool| {
        subscribe.set(Some(value));
    });

    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        TabiDefaultContext { class: "flex flex-wrap h-lvh",
            div { class: "p-4",
                Card { class: "w-96 relative max-w-sm overflow-hidden pt-0",
                    div { class: "bg-primary absolute inset-0 z-30 aspect-video opacity-50 mix-blend-color" }
                    img {
                        src: asset!("/assets/KanjiReadings.png"),
                        alt: "Kanji reading practice banner",
                        title: "Kanji reading practice banner",
                        class: "relative z-20 aspect-video w-full object-cover brightness-60 grayscale",
                    }
                    CardHeader {
                        CardTitle { "Observability Plus is replacing Monitoring" }
                        CardDescription {
                            "Switch to the improved way to explore your data, with natural"
                            "language. Monitoring will no longer be available on the Pro plan in"
                            "November, 2025"
                        }
                    }
                    CardFooter {
                        Dialog {
                            DialogTrigger {
                                Button {
                                    Icon { icon: MdAdd }
                                    "Show Dialog"
                                }
                            }
                            DialogContent { class: "sm: max-w-sm",
                                DialogHeader {
                                    DialogTitle { "Test Dialog" }
                                    DialogDescription { "Detailed dialog description goes here." }
                                }
                                div {
                                    div { "Line 1" }
                                    div { "Line 2" }
                                    div { "Line 3" }
                                    div { "Line 4" }
                                    div { "Line 5" }
                                    div { "Line 6" }
                                }
                                DialogFooter {
                                    DialogClose {
                                        Button { variant: ButtonVariant::Outline, "Cancel" }
                                    }
                                    Button { "Save changes" }
                                }
                            }
                        }
                        Badge {
                            variant: BadgeVariant::Secondary,
                            class: "ml-auto",
                            "Warning"
                        }
                    }
                }
            }

            div { class: "p-4",
                Card { class: "w-96",
                    CardHeader {
                        CardTitle { "User Information" }
                        CardDescription { "Please fill in your details below" }
                        CardAction {
                            // Drop down menu
                        }
                        CardContent { class: "flex flex-col gap-2",
                            div { class: "grid grid-cols-2 gap-4",
                                div {
                                    span { "Name" }
                                    TextInput {
                                        id: "small-form-name",
                                        placeholder: "Enter your name",
                                        required: true,
                                    }
                                }
                                div {
                                    span { "Role" }
                                    Select::<String> {
                                        class: "w-full",
                                        value: selected_role,
                                        on_value_change: move |value| {
                                            //log_error_message(format!("Selected {value:?}").as_str());
                                            selected_role.set(Some(value));
                                        },
                                        SelectTrigger::<String> { class: "w-full",
                                            SelectValue::<String> {
                                                class: "inline-block w-full text-left",
                                                placeholder: "Select a role",
                                            }
                                        }
                                        SelectList::<String> {
                                            for (index , value) in role_list.iter().enumerate() {
                                                SelectOption::<String> {
                                                    index,
                                                    value,
                                                    text_value: "{value}",
                                                    "{value}"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            span { "Distance" }
                            ValidatedTextInput::<ValidatedNumber<0,255,3>> { default_value: ValidatedNumber::Number(2.55) }
                            span { "Framework" }
                            // Combo box
                            Select::<String> {
                                class: "w-full",
                                value: selected_framework,
                                on_value_change: move |value| {
                                    //log_error_message(format!("Selected {value:?}").as_str());
                                    selected_framework.set(Some(value));
                                },
                                SelectTrigger::<String> { class: "w-full",
                                    SelectValue::<String> {
                                        class: "inline-block w-full text-left",
                                        placeholder: "Select a framework",
                                    }
                                }
                                SelectList::<String> {
                                    for (index , value) in framework_list.iter().enumerate() {
                                        SelectOption::<String> {
                                            index,
                                            value,
                                            text_value: "{value}",
                                            "{value}"
                                        }
                                    }
                                }
                            }
                            span { "Comments" }
                            TextArea { placeholder: "Add any additional comments" }
                            span { "Subscribe" }
                            div { class: "flex gap-1 items-center",
                                Switch {
                                    value: subscribe,
                                    on_value_change: handle_subscribe_changed,
                                }
                                if subscribe.read().unwrap_or_default() {
                                    "on"
                                } else {

                                    "off"
                                }
                            }
                            div { class: "flex gap-2",
                                Button { "Submit" }
                                Button { variant: ButtonVariant::Outline, "Cancel" }
                            }
                        }
                    }
                }
            }
        }
    }
}
