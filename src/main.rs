#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate dotenv;
#[macro_use]
extern crate log;
extern crate num_cpus;

mod apps;
mod db;
mod schema;
mod server;

use dotenv::dotenv;

fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    db::init();
    server::run()
}
