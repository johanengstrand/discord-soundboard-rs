use rocket_contrib::serve::StaticFiles;

#[macro_use] pub mod response;
pub mod routes;

pub fn start() {
    routes::mount()
        .mount("/", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/public")))
        .launch();
}
