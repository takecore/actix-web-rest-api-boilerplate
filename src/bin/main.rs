use dotenv::dotenv;

use lib::db::connection;
use lib::server;

fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    connection::init();
    server::run()
}
