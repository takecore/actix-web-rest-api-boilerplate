mod apps;
mod config;
mod server;

fn main() -> std::io::Result<()> {
    server::run()
}
