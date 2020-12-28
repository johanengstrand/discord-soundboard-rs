#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate serde;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate lazy_static;

use std::thread;
use std::process;

mod api;
mod bot;
mod settings;

lazy_static! {
    static ref CONFIG: settings::Settings = match settings::Settings::new() {
        Err(why) => {
            println!("Failed to parse configuration file ({})", why);
            process::exit(1);
        },
        Ok(settings) => settings,
    };
}

fn main() {
    let api_thread = thread::spawn(|| api::start());
    bot::start();
    api_thread.join().expect("Could not start thread!");
}
