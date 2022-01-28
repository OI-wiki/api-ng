use api::preview;
use config::Config;
use file_cache::FileCache;
use warp::Filter;

mod api;
mod config;
mod file_cache;
#[tokio::main]
async fn main() {
  env_logger::init_from_env(
    env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
  );
  let config: Config = config::Config::load(None).await.unwrap_or_default();
  log::info!("oi_wiki_api now running!");
  log::info!("{:#?}", config);
  let file_cache = FileCache::new(config.lru_cap);
  let file_cache_route = warp::any().map(move || file_cache.clone());
  let config_move = config.clone();
  let config_route = warp::any().map(move || config_move.clone());
  let cors = warp::cors()
    .allow_any_origin()
    .allow_methods(vec!["POST", "GET"]);
  let preview_route = warp::get()
    .and(warp::path("preview"))
    .and(warp::query::<preview::Parameter>())
    .and(config_route)
    .and(file_cache_route)
    .and_then(preview::preview);
  let route = preview_route.with(cors).with(warp::log("wiki-api"));
  warp::serve(route).run((config.ip, config.port)).await;
}
