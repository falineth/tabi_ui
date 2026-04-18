#[cfg(feature = "web")]
pub fn get_now() -> f64 {
    web_sys::js_sys::Date::now()
}

#[cfg(feature = "desktop")]
pub fn get_now() -> f64 {
    std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs_f64()
        * 1000f64
}

#[cfg(feature = "web")]
pub fn nanoid() -> String {
    nanoid_wasm::nanoid!(21)
}

#[cfg(feature = "desktop")]
pub fn nanoid() -> String {
    nanoid::nanoid!()
}
