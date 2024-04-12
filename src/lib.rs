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

const GIST_REGEXES: &[(&str, &str)] = &[
    (
        r"^gist\.(?:github|githubusercontent)\.com/(.+?)/([a-f0-9]+)$",
        "https://gist.githubusercontent.com/$1/$2/raw",
    ),
    (
        r"^gist\.(?:github|githubusercontent)\.com/(.+?)/([a-f0-9]+)/raw/(.+)$",
        "https://gist.githubusercontent.com/$1/$2/raw/$3",
    ),
];

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

    for (regex, replacement) in GIST_REGEXES {
        let re = Regex::new(regex).unwrap();
        if re.captures(url).is_some() {
            let replaced = re.replace(url, *replacement);
            return Ok(replaced.to_string());
        }
    }

    Err("Unrecognized gist URL format".into())
}
