///
/// Exports
///
#[allow(unused_imports)]
pub use config::{CONFIG, Config};
pub use esp_hal::{self as hal};

///
/// Exports
///
pub mod init;
pub mod log;

///
/// Exports
///
mod config;
mod macros;

///
/// Custom halt function.
///
#[unsafe(no_mangle)]
fn custom_halt() -> ! {
    hal::riscv::interrupt::disable();
    log::warn!("Halted! Interrupts disabled.");
    loop {}
}
