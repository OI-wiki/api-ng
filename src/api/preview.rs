use std::convert::TryInto;

use serde::{Deserialize, Serialize};
use warp::{http, Rejection, Reply};

use crate::{config::Config, file_cache::FileCache};

#[derive(Debug, Serialize, Deserialize)]
pub struct Parameter {
  pub path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
  pub text: String,
  pub title: String,
}

pub async fn preview(
  param: Parameter,
  config: Config,
  cache: FileCache,
) -> Result<warp::reply::Response, Rejection> {
  let path = format!(
    "{}/docs{}.md",
    config.repo_path,
    param.path.trim_end_matches('/')
  );
  log::debug!("{}", path);
  let file = cache.cached_get(path).await;
  if let Some(t) = file {
    let mut split_count: u32 = 0;
    let title: String = t
      .lines()
      .take_while(|line| {
        log::debug!("{}", line);
        if *line == "---" {
          split_count += 1;
        }
        split_count < 2
      })
      .filter(|line| line.starts_with("title:"))
      .map(|line| line.split("title: ").last().unwrap_or(""))
      .collect::<Vec<_>>()
      .get(0)
      .map(|l| l.to_string())
      .unwrap_or_else(|| "".to_string());
    split_count = 0;
    let lines: String = t
      .lines()
      .skip_while(|line| {
        if *line == "---" {
          split_count += 1;
        }
        split_count < 2
      })
      .map(|l| l.trim())
      .filter(|l| {
        !l.starts_with("author:")
          && !l.starts_with('#')
          && !l.starts_with("!!!")
          && !l.starts_with("???")
      })
      .skip(1)
      .take(config.preview_lines.try_into().unwrap())
      .map(|l| l.to_string())
      .collect::<Vec<String>>()
      .join("\n")
      .chars()
      .filter(|c| *c != '$')
      .collect();
    let text = strip_markdown::strip_markdown(&lines);
    return Ok(warp::reply::json(&Response { text, title }).into_response());
  }
  Ok(
    warp::reply::with_status(String::from("not found"), http::StatusCode::NOT_FOUND)
      .into_response(),
  )
}
