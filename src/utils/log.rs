//!
//! # Log and print macros.
//!
//! This module re-exports the `log` and `esp_println` macros.
//!
//! ```
//! log::info!("This is a info message!");
//! log::warn!("This is a warning message!");
//! log::error!("This is an error message!");
//! log::debug!("This is a debug message!");
//! log::trace!("This is a trace message!");
//!
//! log::dbg!("This is a dbg message!");
//! log::print!("This is a print message!");
//! log::println!("This is a println message!");
//! ```
//!

#[allow(unused_imports)]
pub use esp_println::{dbg, print, println};
#[allow(unused_imports)]
pub use log::{debug, error, info, trace, warn};
