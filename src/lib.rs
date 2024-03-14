use axum::{
    extract::Request,
    response::{Html, IntoResponse},
};
use query::Response;

pub mod query;

pub async fn index(request: Request) -> Result<axum::response::Response, String> {
    let path = request.uri().path().to_string();
    let query_string = request.uri().query().unwrap_or_default();

    if path.is_empty() || path == "/" && query_string.is_empty() {
        return Ok(Html(include_str!("../templates/index.html")).into_response());
    }
    let mut response: Response = serde_qs::from_str(query_string).map_err(|e| e.to_string())?;
    response.path.replace(path);

    Ok(response.into_response())
}
