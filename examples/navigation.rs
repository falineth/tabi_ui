use dioxus::prelude::*;
use dioxus_free_icons::icons::bs_icons::{BsGear, BsHouse, BsInfoCircle};
use tabi_ui::*;

const TAILWIND_CSS: Asset = asset!("../assets/tailwind.css");

#[component]
pub fn NavbarView() -> Element {
    let nav = navigator();

    let handle_goto_home = use_callback(move |_| {
        nav.replace(Route::HomeView {});
    });

    let handle_goto_settings = use_callback(move |_| {
        nav.replace(Route::SettingsView {});
    });

    let handle_goto_about = use_callback(move |_| {
        nav.replace(Route::AboutView {});
    });

    rsx! {
        GlobalDraw {
            actions: rsx! {
                DrawAction { icon: BsHouse, label: "Home", onclick: handle_goto_home }
                DrawAction { icon: BsGear, label: "Settings", onclick: handle_goto_settings }
                DrawAction { icon: BsInfoCircle, label: "About", onclick: handle_goto_about }
            },
            content: rsx! {
                div { class: "p-2", Slider {} }
            },
            Outlet::<Route> {}
        }
    }
}

#[component]
pub fn HomeView() -> Element {
    rsx! {
        div { class: "w-full",
            BreadcrumbBar {
                Breadcrumb {
                    BreadcrumbList {
                        BreadcrumbItem {
                            BreadcrumbPage { "Home" }
                        }
                    }
                }
            }
            div { class: "p-4", "Home View" }
        }
    }
}

#[component]
pub fn SettingsView() -> Element {
    rsx! {
        div { class: "w-full",
            div {
                class: "flex px-4 items-center h-12 w-full border-b",
                class: "border-window-foregroundnormal/20",
                Breadcrumb {
                    BreadcrumbList {
                        BreadcrumbItem {
                            BreadcrumbLink { to: Route::HomeView {}, "Home" }
                        }
                        BreadcrumbSeparator {}
                        BreadcrumbItem {
                            BreadcrumbPage { "Settings" }
                        }
                    }
                }
            }
            div { class: "p-4", "Settings View" }
        }
    }
}

#[component]
pub fn AboutView() -> Element {
    rsx! {
        div { class: "w-full",
            div {
                class: "flex px-4 items-center h-12 w-full border-b",
                class: "border-window-foregroundnormal/20",
                Breadcrumb {
                    BreadcrumbList {
                        BreadcrumbItem {
                            BreadcrumbLink { to: Route::HomeView {}, "Home" }
                        }
                        BreadcrumbSeparator {}
                        BreadcrumbItem {
                            BreadcrumbPage { "About" }
                        }
                    }
                }
            }
            div { class: "p-4", "About View" }
        }
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Routable, PartialEq)]
enum Route {
    #[layout(NavbarView)]
    #[route("/")]
    HomeView {},

    #[route("/settings")]
    SettingsView {},

    #[route("/about")]
    AboutView {},
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        TabiDefaultContext { Router::<Route> {} }
    }
}

#[cfg(feature = "desktop")]
fn launch_app() {
    use dioxus::desktop::{Config, WindowBuilder};

    let mut config = Config::default()
        .with_background_color(get_window_background_color())
        .with_window(WindowBuilder::new().with_title("Navigation Example"));

    if cfg!(not(debug_assertions)) {
        config = config.with_menu(None);
    }

    LaunchBuilder::new().with_cfg(config).launch(App);
}

#[cfg(feature = "web")]
fn launch_app() {
    dioxus::launch(App);
}

fn main() {
    launch_app();
}
