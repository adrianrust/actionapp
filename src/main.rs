#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate mysql;
extern crate dotenv;

mod routes;
use mysql::Pool;
mod db;

fn main() {
    let mut pool: Pool = db::new_connection();
    db::create_table_action(&mut pool);

    rocket::ignite().manage(pool).mount("/", routes![routes::new_action]).launch();
}
