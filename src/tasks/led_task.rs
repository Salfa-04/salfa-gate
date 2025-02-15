use crate::hal::gpio::OutputOpenDrain as Output;
use crate::init_ticker;

#[super::task]
pub async fn led_task(mut bled: Output<'static>) {
    let mut t = init_ticker!(500);

    loop {
        bled.toggle();
        t.next().await;
    }
}
