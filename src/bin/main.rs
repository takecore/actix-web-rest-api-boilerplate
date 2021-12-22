use dotenv;

use lib::db::connection;
use lib::server;

fn main() -> std::io::Result<()> {
    dotenv::from_filename(".env").ok();
    env_logger::init();
    connection::init();
    server::run()
}
