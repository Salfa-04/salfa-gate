//!
//! # Email Routing
//!
//! Email Routing allows you to route email for a domain to a specific server.
//! This is useful if you have a mail server that is not hosted by Cloudflare,
//! but you want to use Cloudflare to manage your DNS.
//!
//! https://developers.cloudflare.com/api/resources/email_routing/
//!
//! ## API Details for Email Routing
//!
//! | Action                        | Method  | API Endpoint                  |
//! |-------------------------------|---------|-------------------------------|
//! | Enable Email Routing          | POST    | Zone Settings Edit            |
//! | Create A Destination Address  | POST    | Email Routing Addresses Edit  |
//! | List Routing Rules            | GET     | Email Routing Rules Edit      |
//! | Delete Routing Rule           | DELETE  | Email Routing Rules Edit      |
//! | Update Catch All Rule         | PUT     | Email Routing Rules Edit      |
//!
//! ## API Parameters for Email Routing
//!
//! * `zone_id`:      The Zone ID
//! * `account_id`:   The Account ID
//!

use alloc::format;

#[non_exhaustive]
struct EmailRouting<'t> {
    zone_id: &'t str,
    account_id: &'t str,
}

impl<'t> EmailRouting<'t> {
    const API_PREFIX: &'static str = "https://api.cloudflare.com/client/v4";

    pub fn new() -> EmailRouting<'t> {
        let config = crate::CONFIG;

        EmailRouting {
            zone_id: config.cf_zoneid,
            account_id: config.cf_accountid,
        }
    }

    pub async fn enable(&self) {
        let url = format!(
            "{}/zones/{}/email/routing/enable",
            Self::API_PREFIX,
            self.zone_id
        );

        // let client = reqwless::Client::new();
        // let response = client
        //     .request(Method::POST, &url)
        //     .header("X-Auth-Email", crate::CONFIG.cf_email)
        //     .header("X-Auth-Key", crate::CONFIG.cf_apikey)
        //     .send()
        //     .await
        //     .unwrap();

        // let body = response.body().read_to_end().await;
        // let _ = dbg!(core::str::from_utf8(body.unwrap()));
    }
}
