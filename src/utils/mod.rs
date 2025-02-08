///
/// Re-export the necessary modules.
///
pub use esp_hal::{self as hal, riscv};
#[allow(unused_imports)]
pub use esp_println::{dbg, print, println};
pub use init::{ispa_init, wifi_init};

mod init;
mod macros;

#[toml_cfg::toml_config]
struct Config {
    //?
    //? The Config for WiFi.
    //?
    #[default("")]
    pub wifi_ssid: &'static str,
    #[default("")]
    pub wifi_psk: &'static str,
    #[default("Salfa-ESP32")]
    pub hostname: &'static str,

    //?
    //? The Config for CloudFlare.
    //?
    #[default("")]
    pub cf_zoneid: &'static str,
    #[default("")]
    pub cf_accountid: &'static str,
}

///
/// Custom halt function.
///
#[unsafe(no_mangle)]
fn custom_halt() -> ! {
    riscv::interrupt::disable();
    println!("Halted! Interrupts disabled.");
    loop {}
}
