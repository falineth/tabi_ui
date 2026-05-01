#[cfg(not(all(target_os = "linux", feature = "desktop")))]
mod default;
#[cfg(all(target_os = "linux", feature = "desktop"))]
mod linux;

#[cfg(not(all(target_os = "linux", feature = "desktop")))]
pub use default::*;
#[cfg(all(target_os = "linux", feature = "desktop"))]
pub use linux::*;
