use songbird::SerenityInit;
use serenity::{
    async_trait,
    client::{Client, Context, EventHandler},
    model::{gateway::Ready},
    http::{client::Http},
};

pub mod tracks;
pub mod guilds;
pub mod playback;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
pub async fn start(token: String) {
    tracing_subscriber::fmt::init();

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .register_songbird()
        .await
        .expect("Failed to create discord client");

    let http = Http::new_with_token(&token);
    let _ = client.start().await.map_err(|why| println!("Client ended: {:?}", why));
}
