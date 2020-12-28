#![feature(proc_macro_hygiene, decl_macro)]
extern crate config;
#[macro_use] extern crate serde;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use std::env;
use std::thread;
use std::process;
use config::{ConfigError};

mod api;
mod bot;

#[derive(Debug, Deserialize)]
pub struct Settings {
    token: String,
    folder: String,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut settings = config::Config::default();
        match settings.merge(config::File::with_name("config")) {
            Err(why) => Err(why),
            Ok(_) => settings.try_into(),
        }
    }
}

fn main() {
    let settings = match Settings::new() {
        Err(why) => {
            println!("Failed to parse configuration file ({})", why);
            process::exit(1);
        },
        Ok(settings) => settings,
    };
    let api_thread = thread::spawn(||api::start());

    bot::start(settings.token);
    api_thread.join().expect("Could not start thread!");
}
