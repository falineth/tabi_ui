use std::thread::sleep;
use std::time::Duration;

use dioxus::prelude::*;

use crate::{ThemeContext, ThemeStyle, WindowSize};

/// # TabiDefaultContext
///
/// The [`TabiDefaultContext`] component is a container that simplifies bringing in
/// contexts and utilities used by Tabi components.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use tabi_ui::*;
///
/// #[derive(Debug, Clone, Routable, PartialEq)]
/// enum Route {
///     #[layout(NavbarView)]
///     #[route("/")]
///     HomeView {},
/// }
///
/// #[component]
/// pub fn HomeView() -> Element {
///     rsx! {}
/// }
///
/// #[component]
/// pub fn NavbarView() -> Element {
///     rsx! {
///         TabiDefaultContext { class: "h-lvh w-lvw overflow-hidden", Outlet::<Route> {} }
///     }
/// }
/// ```
///
/// ## Function
///
/// Delay showing the app content to allow styles to load
///
/// Provides the contexts
///  - WindowSize
///  - ModalOffset
///
#[component]
pub fn TabiDefaultContext(#[props(default)] class: String, children: Element) -> Element {
    let mut window_size = use_context_provider(|| Signal::new(WindowSize::default()));

    let mut loaded = use_signal(bool::default);

    let theme_context = try_use_context::<ThemeContext>();

    let bg_color = use_memo(move || {
        if let Some(theme_context) = theme_context.as_ref() {
            let (r, g, b, _a) = theme_context.bg_color;
            return format!("background-color: #{r:02x}{g:02x}{b:02x};");
        }

        return String::default();
    });

    use_future(move || async move {
        #[cfg(not(feature = "web"))]
        {
            _ = tokio::spawn(async move {
                sleep(Duration::from_millis(10));
            })
            .await;
        }

        loaded.set(true);
    });

    rsx! {
        ThemeStyle {}

        if !*loaded.read() {
            div {
                class: "bg-white",
                style: "height: 100lvh; width: 100lvw; {bg_color}",
            }
        }

        div {
            class,
            style: if !*loaded.read() { "display: none;" },
            onresize: move |e| {
                if let Ok(size) = e.get_content_box_size() {
                    window_size
                        .set(WindowSize {
                            width: size.width,
                            height: size.height,
                        });
                }
            },

            {children}
        }
    }
}
