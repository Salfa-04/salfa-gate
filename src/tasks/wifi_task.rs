use crate::init_ticker;
use crate::utils::log::dbg;
use crate::utils::prelude::*;

use embassy_net::Stack;
use hal::rng::Rng;
use net_buf::NetBuffer;
use reqwless::request::Method;

///
/// local modules
///
// mod email;
mod net_buf;

#[super::task]
pub async fn wifi_task(stack: Stack<'static>, rng: Rng) -> ! {
    dbg!(stack.wait_link_up().await);
    dbg!(stack.wait_config_up().await);
    dbg!(stack.config_v4());

    let mut buffer: NetBuffer<8, 1024, 1024> = NetBuffer::new(rng);
    let (mut client, buf) = buffer.configure(stack);

    let mut request = client
        .request(Method::GET, "https://api.cloudflare.com/")
        .await
        .unwrap();

    let handler = request.send(buf).await.unwrap();
    let body = handler.body().read_to_end().await;

    let _ = dbg!(core::str::from_utf8(body.unwrap()));

    let mut t = init_ticker!(600);

    loop {
        t.next().await;
    }
}
