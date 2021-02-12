use api::preview;
use config::Config;
use warp::Filter;

extern crate log;
extern crate pretty_env_logger;
mod api;
mod config;
#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let config: Config = config::Config::load(None).await.unwrap_or_default();
    let config_move = config.clone();
    let config_route = warp::any().map(move || config_move.clone());
    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["POST", "GET"]);
    let preview_route = warp::get()
        .and(warp::path("preview"))
        .and(warp::query::<preview::Parameter>())
        .and(config_route)
        .and_then(preview::preview);
    let route = preview_route.with(cors).with(warp::log("wiki-api"));
    warp::serve(route).run((config.ip, config.port)).await;
}
