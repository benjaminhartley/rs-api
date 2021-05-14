#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate lazy_static;

extern crate bcrypt;
extern crate dotenv;
extern crate jsonwebtoken;

#[macro_use]
extern crate log;
extern crate log4rs;

extern crate r2d2;
extern crate r2d2_postgres;

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;

pub mod controllers;
pub mod db;
pub mod routes;
pub mod utils;

fn main() {
    log4rs::init_file("log4rs.yml", Default::default()).expect("failed to initialize logger");

    dotenv::dotenv().ok();

    debug!("starting Rocket");
    rocket::ignite()
        .mount(
            "/",
            routes![
                routes::auth::signup,
                routes::auth::login,
                routes::auth::validate,
                routes::user::get_user,
                routes::user::update_password,
            ],
        )
        .launch();
}
