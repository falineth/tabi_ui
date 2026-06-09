#[cfg(all(
    not(feature = "web"),
    not(all(target_os = "linux", feature = "desktop"))
))]
mod default;
#[cfg(all(not(feature = "web"), all(target_os = "linux", feature = "desktop")))]
mod linux;
#[cfg(feature = "web")]
mod web;

#[cfg(all(
    not(feature = "web"),
    not(all(target_os = "linux", feature = "desktop"))
))]
pub use default::*;
#[cfg(all(not(feature = "web"), all(target_os = "linux", feature = "desktop")))]
pub use linux::*;
#[cfg(feature = "web")]
pub use web::*;
