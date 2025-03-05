#[macro_use]
extern crate rocket;

mod application;
mod config;
mod outbound; // Ensure environment loading works

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(application::place::place_category_module::stage())
        .attach(outbound::mongodb::adapter::MongoDB {})
}
