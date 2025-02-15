//!
//! # Utils
//!

///
/// # Prelude
///
/// A collection of commonly used items.
///
/// - `CONFIG`: The global configuration.
/// - `Config`: The configuration struct.
/// - `hal`: The hardware abstraction layer.
/// - `riscv`: The RISC-V module.
///
#[allow(unused_imports)]
pub mod prelude {
    pub use super::config::{CONFIG, Config};
    pub use esp_hal::{self as hal, riscv};
}

pub mod init;
pub mod log;

mod config;
mod macros;

///
/// Custom halt function.
///
#[unsafe(no_mangle)]
fn custom_halt() -> ! {
    ::esp_hal::riscv::interrupt::disable();
    log::warn!("Halted! Interrupts disabled.");
    loop {}
}
