#[macro_use]
extern crate diesel;
extern crate dotenv;

mod apps;
mod config;
mod db;
mod schema;
mod server;

fn main() -> std::io::Result<()> {
    server::run()
}
