use dotenv::dotenv;

use lib::db;
use lib::server;

fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    db::init();
    server::run()
}
