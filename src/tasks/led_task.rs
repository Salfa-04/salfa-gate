use crate::{hal, init_ticker};
use hal::gpio::OutputOpenDrain as Output;

#[super::task]
pub async fn led_task(mut bled: Output<'static>) {
    let mut t = init_ticker!(500);

    loop {
        bled.toggle();
        t.next().await;
    }
}
