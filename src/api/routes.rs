use rocket_contrib::json::JsonValue;

#[get("/tracks")]
fn tracks() -> JsonValue {
    success!([ "Track 1", "Track 2", "Track 3", "Track 4" ])
}

pub fn mount() -> rocket::Rocket {
    rocket::ignite()
        .mount("/api", routes![tracks])
}
