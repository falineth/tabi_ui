// This file (or portion of this file) is part of Dioxus Components
// https://github.com/DioxusLabs/components/blob/main/primitives/src/lib.rs
// SPDX-FileCopyrightText: 2025 Jonathan Kelley
// SPDX-FileCopyrightText: 2025 Evan Almloff
// SPDX-FileCopyrightText: 2025 Miles Murgaw
// SPDX-License-Identifier: MIT OR Apache-2.0

use dioxus::prelude::*;

/// Allows some state to be either controlled or uncontrolled.
pub fn use_controlled<T: Clone + PartialEq + 'static>(
    prop: ReadSignal<Option<T>>,
    default: T,
    on_change: Callback<T>,
) -> (Memo<T>, Callback<T>) {
    let mut internal_value = use_signal(|| prop.cloned().unwrap_or(default));
    let value = use_memo(move || prop.cloned().unwrap_or_else(&*internal_value));

    let set_value = use_callback(move |x: T| {
        internal_value.set(x.clone());
        on_change.call(x);
    });

    (value, set_value)
}
