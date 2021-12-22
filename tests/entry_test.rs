mod apps;

use dotenv;

#[cfg(test)]
#[ctor::ctor]
fn init() {
    dotenv::from_filename(".env.test").ok();
    env_logger::init();
}
