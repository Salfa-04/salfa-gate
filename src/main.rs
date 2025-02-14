#![no_std]
#![no_main]
#![feature(impl_trait_in_assoc_type)]

use crate::utils::*;
use esp_backtrace as _;
extern crate alloc;

mod tasks;
mod utils;

#[esp_hal_embassy::main]
async fn entry(spaw: embassy_executor::Spawner) {
    let p = {
        use hal::{Config, clock::CpuClock, init};
        init(Config::default().with_cpu_clock(CpuClock::max()))
    };

    let _ispa = {
        // Initialize the ISPA& System
        init::ispa_init((p.SYSTIMER, p.SW_INTERRUPT))
    };

    {
        // Spawn the LED task
        use hal::gpio::{Level, OutputOpenDrain as O, Pull::None};
        let l = O::new(p.GPIO8, Level::High, None);
        spaw.must_spawn(tasks::led_task(l));
    }

    {
        // Spawn the WiFi task
        let wifi = {
            // Initialize the WiFi module
            init::wifi_init((p.TIMG0, p.RNG, p.RADIO_CLK, p.WIFI))
        };

        spaw.must_spawn(tasks::conn_task(wifi.0));
        spaw.must_spawn(tasks::wifi_task(wifi.2, wifi.3));
        spaw.must_spawn(tasks::net_task(wifi.1));
    }
}
