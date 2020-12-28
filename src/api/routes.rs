use crate::bot;
use crate::CONFIG;
use rocket_contrib::json::JsonValue;

#[get("/tracks")]
fn tracks() -> JsonValue {
    let track_list = bot::tracks::get_tracks_in_dir(&CONFIG.folder);

    match track_list {
        Ok(tracks) => success!(tracks),
        Err(why) => failure!(why),
    }
}

pub fn mount() -> rocket::Rocket {
    rocket::ignite()
        .mount("/api", routes![tracks])
}
