use std::fs::read_to_string;
use std::sync::Arc;

use dbus::message::MatchRule;
use dbus::nonblock::{MsgMatch, SyncConnection};
use dioxus::prelude::*;

const DEFAULT_THEME: &str = include_str!("../../../assets/default_theme.css");
const DEFAULT_WINDOW_BACKGROUND: (u8, u8, u8, u8) = (0xEF, 0xF0, 0xF1, 0xFF);
const KDE_COLOUR_CONFIG: &str = "kdeglobals";

pub fn use_theme_colors() -> ReadSignal<String, SyncStorage> {
    let theme_colors: Signal<String, SyncStorage> = use_signal_sync(get_theme_colors);

    use_dbus_theme_event_listener(theme_colors);

    return theme_colors.into();
}

fn use_dbus_theme_event_listener(mut theme_colors: Signal<String, SyncStorage>) {
    let mut dbus_conn: Signal<Option<Arc<SyncConnection>>, SyncStorage> =
        use_signal_sync(Option::default);

    let mut dbus_incoming_signal: Signal<Option<MsgMatch>, SyncStorage> =
        use_signal_sync(Option::default);

    spawn(async move {
        let Ok((resource, conn)) = dbus_tokio::connection::new_session_sync() else {
            return;
        };

        tokio::spawn(async {
            let err = resource.await;
            println!("DBus connection lost: {:?}", err);
        });

        let signal_match_rule =
            MatchRule::new_signal("org.freedesktop.portal.Settings", "SettingChanged");

        let Ok(message_match) = conn
            .add_match(signal_match_rule)
            .await
            .inspect_err(|err| println!("Error 1 {err:?}"))
        else {
            return;
        };

        let incoming_signal = message_match.cb(move |_, (source, setting): (String, String)| {
            if source == "org.kde.kdeglobals.General" && setting == "ColorScheme" {
                println!("Theme colors changed");
                let new_theme_colors = get_theme_colors();
                theme_colors.set(new_theme_colors);
            }

            true
        });

        dbus_conn.set(Some(conn));
        dbus_incoming_signal.set(Some(incoming_signal));
    });
}

pub fn get_window_background_color() -> (u8, u8, u8, u8) {
    let Some(config_dir) = dirs::config_dir() else {
        return DEFAULT_WINDOW_BACKGROUND;
    };

    let color_config_path = config_dir.join(KDE_COLOUR_CONFIG);

    let Ok(mut color_data) = read_to_string(&color_config_path) else {
        return DEFAULT_WINDOW_BACKGROUND;
    };

    color_data.make_ascii_lowercase();

    let mut current_color_section: Option<&str> = None;

    for line in color_data.lines() {
        if let Some(section_name) = as_config_section_name(line) {
            current_color_section = as_color_section_name(section_name);
            continue;
        }

        if !matches!(current_color_section, Some("window")) {
            continue;
        }

        if let Some(css_color_value) = as_css_color_tuple_for_key(line, "backgroundnormal") {
            return css_color_value;
        }
    }

    return DEFAULT_WINDOW_BACKGROUND;
}

fn get_theme_colors() -> String {
    let Some(config_dir) = dirs::config_dir() else {
        return DEFAULT_THEME.to_string();
    };

    let color_config_path = config_dir.join(KDE_COLOUR_CONFIG);

    let Ok(mut color_data) = read_to_string(&color_config_path) else {
        return DEFAULT_THEME.to_string();
    };

    color_data.make_ascii_lowercase();

    let mut current_color_section: Option<&str> = None;

    let mut theme_color_list: Vec<String> = vec![":root {".to_string()];

    for line in color_data.lines() {
        if let Some(section_name) = as_config_section_name(line) {
            current_color_section = as_color_section_name(section_name);
            continue;
        }

        if let Some(current_theme_section_name) = current_color_section
            && let Some(css_color_value) = as_css_color_value(line, current_theme_section_name)
        {
            theme_color_list.push(css_color_value);
            continue;
        }
    }

    if theme_color_list.len() == 1 {
        return DEFAULT_THEME.to_string();
    }

    theme_color_list.push("--color-background:var(--color-window-backgroundnormal);--color-foreground:var(--color-window-foregroundnormal);--background:var(--color-window-backgroundnormal);--foreground:var(--color-window-foregroundnormal);body{background-color:var(--color-window-backgroundnormal);}}".to_string());

    return theme_color_list.concat();
}

fn as_config_section_name(line: &str) -> Option<&str> {
    line.strip_prefix("[")
        .and_then(|value| value.strip_suffix("]"))
}

fn as_color_section_name(section_name: &str) -> Option<&str> {
    let section_name = section_name.strip_prefix("colors:")?;

    if !section_name.bytes().all(|c| c.is_ascii_alphabetic()) {
        return None;
    }

    return Some(section_name);
}

fn as_css_color_tuple_for_key(line: &str, wanted_key: &str) -> Option<(u8, u8, u8, u8)> {
    let (key, value) = line.split_once("=")?;

    if key != wanted_key {
        return None;
    }

    let (r, g_b) = value.split_once(",")?;

    let r = r.parse::<u8>().ok()?;

    let (g, b) = g_b.split_once(",")?;

    let g = g.parse::<u8>().ok()?;

    let b = b.parse::<u8>().ok()?;

    return Some((r, g, b, 0xFF));
}

fn as_css_color_value(line: &str, theme_section_name: &str) -> Option<String> {
    let (key, value) = line.split_once("=")?;

    if !key.bytes().all(|c| c.is_ascii_alphabetic()) {
        return None;
    }

    let (r, g_b) = value.split_once(",")?;

    let r = r.parse::<u8>().ok()?;

    let (g, b) = g_b.split_once(",")?;

    let g = g.parse::<u8>().ok()?;

    let b = b.parse::<u8>().ok()?;

    return Some(format!(
        "--color-{theme_section_name}-{key}:#{r:02x}{g:02x}{b:02x};"
    ));
}
