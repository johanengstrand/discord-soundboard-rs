use std::sync::{Arc, Mutex};
use serenity::http::Http;

#[macro_use] pub mod response;
pub mod routes;

pub struct Handles {
    pub http: Arc<Http>,
}
