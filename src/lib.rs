#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate dotenv;
#[macro_use]
extern crate log;
extern crate num_cpus;

pub mod apps;
pub mod db;
pub mod schema;
pub mod server;
