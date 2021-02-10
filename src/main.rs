use config::Config;

extern crate log;
extern crate pretty_env_logger;
mod config;
#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let config: Config = config::Config::load(None).await.unwrap_or_default();
    println!("Hello, world!");
}
