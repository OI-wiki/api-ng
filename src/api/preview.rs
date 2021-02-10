use std::convert::TryInto;

use serde::{Deserialize, Serialize};
use warp::{http, Rejection, Reply};

use crate::config::Config;
use crate::markdown;

#[derive(Debug, Serialize, Deserialize)]
pub struct Parameter {
    pub path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub html: String,
    pub title: String,
}

pub async fn preview(param: Parameter, config: Config) -> Result<warp::reply::Response, Rejection> {
    let path = format!(
        "{}/docs{}.md",
        config.repo_path,
        param.path.trim_end_matches('/')
    );
    log::debug!("{}", path);
    let file = tokio::fs::read_to_string(path).await;
    if let Ok(t) = file {
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
            .skip(1)
            .take(config.preview_lines.try_into().unwrap())
            .map(|l| l.to_string())
            .collect::<Vec<String>>()
            .join("\n");
        let html = markdown::render(lines);
        return Ok(warp::reply::json(&Response { html, title }).into_response());
    }
    Ok(
        warp::reply::with_status(String::from("not found"), http::StatusCode::NOT_FOUND)
            .into_response(),
    )
}
