#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate dotenv;
#[macro_use]
extern crate log;

mod apps;
mod db;
mod schema;
mod server;

use dotenv::dotenv;

fn main() -> std::io::Result<()> {
    dotenv().ok();
    server::run()
}
