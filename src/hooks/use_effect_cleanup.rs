// This file (or portion of this file) is part of Dioxus Components
// https://github.com/DioxusLabs/components/blob/main/primitives/src/lib.rs
// SPDX-FileCopyrightText: 2025 Evan Almloff
// SPDX-License-Identifier: MIT OR Apache-2.0

use dioxus::prelude::*;

//use crate::use_effect_cleanup;
/// Run some cleanup code when the component is unmounted if the effect was run.
pub fn use_effect_cleanup<F: FnOnce() + 'static>(#[allow(unused)] cleanup: F) {
    client!(use_drop(cleanup))
}
