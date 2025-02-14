///
/// Re-export the necessary modules.
///
pub use esp_hal::{self as hal, riscv};
#[allow(unused_imports)]
pub use esp_println::{dbg, print, println};
pub use init::{ispa_init, wifi_init};

mod init;
mod macros;

///
/// Custom halt function.
///
#[unsafe(no_mangle)]
fn custom_halt() -> ! {
    riscv::interrupt::disable();
    println!("Halted! Interrupts disabled.");
    loop {}
}
