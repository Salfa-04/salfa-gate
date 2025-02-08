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
