///
/// Import the necessary modules.
///
use crate::hal;
use core::str::FromStr;
use embassy_executor::SendSpawner;
use embassy_net::{DhcpConfig, Runner, Stack, StackResources};
use esp_hal_embassy::InterruptExecutor;
use esp_wifi::wifi::Configuration::Client;
use esp_wifi::wifi::{ClientConfiguration, WifiStaDevice};
use esp_wifi::wifi::{WifiController, WifiDevice};
use esp_wifi::{EspWifiController, wifi::new_with_mode};
use hal::interrupt::software::SoftwareInterruptControl as SIT;
use hal::peripherals::{RADIO_CLK, RNG, TIMG0, WIFI};
use hal::timer::{systimer::SystemTimer as ST, timg::TimerGroup};
use hal::{interrupt::Priority as P, rng::Rng};
use static_cell::StaticCell;

///
/// # ispa_init
///
/// Initialize the ISPA (Interrupt Service Priority Assignment) system.
///
/// `ipsa_init()` is a helper function that initializes the ISPA system.
/// It takes a SystemTimer and a SoftwareInterruptControl as arguments.
/// It returns a tuple of three SendSpawner objects:
///
/// `[SendSpawner, SendSpawner, SendSpawner]`
///
/// The first SendSpawner object is assigned the `Priority1(min)` priority.
///
/// The second SendSpawner object is assigned the `Priority2` priority.
///
/// The third SendSpawner object is assigned the `Priority3` priority.
///
/// ## Example
/// ```
/// let st = SystemTimer::new(p.SYSTIMER);
/// let sw_it = SoftwareInterruptControl::new(p.SW_INTERRUPT);
///
/// let ispa = ispa_init(st, sw_it);
/// ```
///
pub fn ispa_init(st: ST, sw_it: SIT) -> (SendSpawner, SendSpawner, SendSpawner) {
    esp_alloc::heap_allocator!(1024 * 72);

    static EXECUTOR: StaticCell<(
        InterruptExecutor<0>,
        InterruptExecutor<1>,
        InterruptExecutor<2>,
    )> = StaticCell::new();

    esp_hal_embassy::init([st.alarm0, st.alarm1, st.alarm2]);

    let executor = EXECUTOR.init((
        InterruptExecutor::new(sw_it.software_interrupt0),
        InterruptExecutor::new(sw_it.software_interrupt1),
        InterruptExecutor::new(sw_it.software_interrupt2),
    ));

    (
        executor.0.start(P::Priority1),
        executor.1.start(P::Priority2),
        executor.2.start(P::Priority3),
    )
}

///
/// # wifi_init
///
/// Initialize the WiFi stack.
///
/// `wifi_init()` is a helper function that initializes the WiFi stack.
/// It takes a tuple of four peripherals and two string slices as arguments.
/// It returns a tuple of three objects:
///
/// `[Interface, WifiDevice, WifiController]`
///
/// The first object is an Interface object.
///
/// The second object is a WifiDevice object.
///
/// The third object is a WifiController object.
///
/// ## Example
/// ```
/// let (wifi_controller, stack, runner) = {
///     let p = (p.TIMG0, p.RNG, p.RADIO_CLK, p.WIFI);
///     wifi_init((p.TIMG0, p.RNG, p.RADIO_CLK, p.WIFI));
/// }
/// ```
///
pub fn wifi_init(
    (timg0, rng, r_clk, wifi): (TIMG0, RNG, RADIO_CLK, WIFI),
) -> (
    WifiController<'static>,
    Runner<'static, WifiDevice<'static, WifiStaDevice>>,
    Stack<'static>,
    Rng,
) {
    let config = crate::CONFIG;

    let (wifi_ssid, password, dhcpv4_hostname) = (
        FromStr::from_str(config.wifi_ssid).expect("WIFI SSID name too long: [>32]"),
        FromStr::from_str(config.wifi_psk).expect("WIFI password too long: [>64]"),
        FromStr::from_str(config.hostname).expect("DHCPv4 hostname too long: [>32]"),
    );

    let mut rng = Rng::new(rng);
    let ewc = {
        let t = TimerGroup::new(timg0).timer0;
        static EWC: StaticCell<EspWifiController> = StaticCell::new();
        // safe to unwrap here, as we know it is a hardware failure
        EWC.init(esp_wifi::init(t, rng, r_clk).unwrap())
    };

    let config = Client(ClientConfiguration {
        ssid: wifi_ssid,
        password,
        ..Default::default()
    });

    let (driver, mut controller) = {
        // safe to unwrap here, as we know it is a hardware failure
        new_with_mode(ewc, wifi, WifiStaDevice).unwrap()
    };

    // set the configuration, safe to unwrap here
    controller.set_configuration(&config).unwrap();

    // Create the network stack and runner for the wifi driver
    static STACK: StaticCell<StackResources<3>> = StaticCell::new();
    let mut config = DhcpConfig::default();
    config.hostname = Some(dhcpv4_hostname);
    let config = embassy_net::Config::dhcpv4(config);
    let seed = (rng.random() as u64) << 32 | rng.random() as u64;
    let resources: &mut StackResources<3> = STACK.init(StackResources::new());
    let (stack, runner) = {
        // Create the network stack and runner for the wifi driver
        embassy_net::new(driver, config, resources, seed)
    };

    (controller, runner, stack, rng)
}
