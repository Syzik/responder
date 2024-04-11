use std::error;

use axum::{
    body::Bytes,
    extract::Request,
    response::{Html, IntoResponse},
};
use regex::Regex;
use response::Response;

pub mod response;

type Err = Box<dyn error::Error>;

const GIST_REGEX: &str = r"^(gist\.github\.com|gist\.githubusercontent\.com)/([\w\.]+/[a-f0-9]+)(?:/(blob|raw))?(?:/)?([\w\.]+)?(?:/)?(.+)?";

pub async fn index(request: Request) -> Result<axum::response::Response, String> {
    let path = request.uri().path().to_string();
    let query_string = request.uri().query().unwrap_or_default();

    if query_string.is_empty() {
        match path.as_str() {
            "/favicon.ico" => {
                return Ok(
                    Bytes::from_static(include_bytes!("../frontend/dist/favicon.ico"))
                        .into_response(),
                );
            }
            "" | "/" => {
                return Ok(Html(include_str!("../frontend/dist/index.html")).into_response());
            }
            _ => {}
        }
    }
    let mut response: Response = serde_qs::from_str(query_string).map_err(|e| e.to_string())?;
    response.path.replace(path);

    Ok(response.into_response().await)
}

pub fn parse_gist_url(url: &str) -> Result<String, Err> {
    let url = url
        .trim()
        .trim_start_matches("https://")
        .trim_start_matches("http://");
    if !url.starts_with("gist.github.com/") && !url.starts_with("gist.githubusercontent.com/") {
        return Err(
            "Invalid Gist domain (must be gist.github.com or gist.githubusercontent.com)".into(),
        );
    };

    let re = Regex::new(GIST_REGEX).unwrap();
    if let Some(captures) = re.captures(url) {
        let domain = captures.get(1).unwrap().as_str();
        let user_and_id = captures.get(2).unwrap().as_str();
        let blob_or_raw = captures.get(3).map(|m| m.as_str()).unwrap_or("raw");
        let hash = captures.get(4).map(|m| m.as_str()).unwrap_or_default();
        let file = captures.get(5).map(|m| m.as_str()).unwrap_or_default();
        Ok(format!(
            "https://{domain}/{user_and_id}/{blob_or_raw}/{hash}/{file}"
        ))
    } else {
        Err("Invalid Gist URL".into())
    }
}
