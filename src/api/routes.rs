use rocket_contrib::json::JsonValue;

#[get("/test")]
fn test() -> JsonValue {
    json!({
        "api": "hello world"
    })
}

pub fn mount() -> rocket::Rocket {
    rocket::ignite()
        .mount("/api", routes![test])
}
