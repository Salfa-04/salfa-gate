//!
//! # Tasks
//!

///
/// Re-export Tasks.
///
pub use led_task::led_task;
pub use net_task::conn_task;
pub use net_task::net_task;
pub use wifi_task::wifi_task;

///
/// local modules
///
use embassy_executor::task;

mod led_task;
mod net_task;
mod wifi_task;
