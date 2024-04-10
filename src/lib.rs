use axum::{
    body::Bytes,
    extract::Request,
    response::{Html, IntoResponse},
};
use response::Response;

pub mod response;

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

    Ok(response.into_response())
}
