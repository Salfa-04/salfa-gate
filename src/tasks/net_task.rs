use crate::{init_ticker, log::dbg};

use embassy_net::Runner;
use esp_wifi::wifi::{WifiController, WifiEvent, WifiState};
use esp_wifi::wifi::{WifiDevice, WifiStaDevice, sta_state};

#[super::task]
pub async fn net_task(mut runner: Runner<'static, WifiDevice<'static, WifiStaDevice>>) {
    runner.run().await
}

#[super::task]
pub async fn conn_task(mut controller: WifiController<'static>) {
    let mut t = init_ticker!(5000); // 5s

    loop {
        if let Ok(false) = controller.is_started() {
            if let Err(x) = controller.start_async().await {
                dbg!("Failed to Start Wifi:", x);
            } else {
                dbg!("Wifi Started!");
            };
        }

        if sta_state() == WifiState::StaConnected {
            // wait until we're no longer connected
            controller.wait_for_event(WifiEvent::StaDisconnected).await;
            dbg!("Wifi Disconnected!");
        }

        if let Ok(false) = controller.is_connected() {
            if let Err(x) = controller.connect_async().await {
                dbg!("Failed to Connect Wifi:", x);
            } else {
                dbg!("Wifi Connected!");
            };
        }

        t.next().await;
    }
}
