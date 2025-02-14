//!
//! # Global Config
//!
//! This module contains the Config for the project.
//!
//! ## Usage
//! In `cfg.toml`:
//! ```toml
//! [salfa-gate] #<-- The name of the project
//! wifi_ssid       = "SSID"
//! wifi_psk        = "PASSWORD"
//! ```
//!
//! ## Example
//!
//! ```rust
//! mod config;
//! use config::CONFIG;
//!
//! let config = CONFIG;
//! ```
//!

#[toml_cfg::toml_config]
struct Config {
    //?
    //? The Config for WiFi.
    //?
    #[default("")]
    pub wifi_ssid: &'static str,
    #[default("")]
    pub wifi_psk: &'static str,
    #[default("SalGate")]
    pub hostname: &'static str,

    //?
    //? The Config for CloudFlare.
    //?
    #[default("")]
    pub cf_apikey: &'static str,
    #[default("")]
    pub cf_zoneid: &'static str,
    #[default("")]
    pub cf_accountid: &'static str,
}
