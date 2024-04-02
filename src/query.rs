use std::{collections::HashMap, error, str::FromStr};

use axum::{
    http::{HeaderName, HeaderValue, StatusCode},
    response::IntoResponse,
};
use b64::FromBase64;
use lazy_static::lazy_static;
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Response {
    #[serde(alias = "s", alias = "code", default = "default_status")]
    pub status: u16,
    #[serde(alias = "b", alias = "d", alias = "data", default)]
    body: Option<String>,
    #[serde(
        alias = "body.b64",
        alias = "data.b64",
        alias = "b64",
        alias = "base64",
        default
    )]
    body_b64: Option<String>,
    #[serde(alias = "h", default)]
    pub headers: HashMap<String, String>,
    #[serde(default, deserialize_with = "deserialize_bool")]
    pub cors: bool,
    #[serde(skip)]
    pub path: Option<String>,
}

fn default_status() -> u16 {
    StatusCode::OK.as_u16()
}

fn deserialize_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    match String::deserialize(deserializer)?.as_str() {
        "false" | "f" | "no" | "n" | "0" => Ok(false),
        _ => Ok(true),
    }
}

lazy_static! {
    static ref HEADER_ALIAS: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("ct", "content-type");
        map.insert("c", "set-cookie");
        map.insert("sc", "set-cookie");
        map.insert("cookie", "set-cookie");
        map.insert("l", "location");
        map.insert("loc", "location");
        map.insert("csp", "content-security-policy");
        map
    };
}

impl Response {
    pub fn get_body(&self) -> Result<Vec<u8>, b64::FromBase64Error> {
        Ok(match (&self.body, &self.body_b64) {
            (Some(body), _) => body.clone().into(),
            (None, Some(body_b64)) => body_b64.replace(' ', "+").as_str().from_base64()?,
            _ => vec![],
        })
    }

    fn try_into_response(self) -> Result<axum::response::Response, Box<dyn error::Error>> {
        let mut response = axum::response::Response::builder().status(self.status);

        // Add requested headers and resolve aliases
        let headers = response.headers_mut().expect("builder won't have errors");
        for (key, value) in &self.headers {
            let key = HEADER_ALIAS.get(key.as_str()).copied().unwrap_or(key);
            headers.append(HeaderName::from_str(key)?, HeaderValue::from_str(value)?);
        }
        // Set Content-Type from path extension if not set
        headers.entry("content-type").or_insert(
            self.path
                .as_deref()
                .and_then(|path| mime_guess::from_path(path).first_raw())
                .map(|mime| HeaderValue::from_str(mime).expect("mime_guess is valid"))
                .unwrap_or(HeaderValue::from_static("text/plain")),
        );
        // Allow all Cross-Origin Resource Sharing
        if self.cors {
            let any = HeaderValue::from_static("*");
            headers.insert("Access-Control-Allow-Origin", any.clone());
            headers.insert("Access-Control-Allow-Methods", any.clone());
            headers.insert("Access-Control-Allow-Headers", any);
        }

        Ok(response.body(self.get_body()?.into())?)
    }
}

impl IntoResponse for Response {
    fn into_response(self) -> axum::response::Response {
        match self
            .try_into_response()
            .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response())
        {
            Ok(response) | Err(response) => response,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize() {
        let query_string = "status=200&body=Hello%2C%20world%21&headers[content-type]=text/plain";

        let response: Response = serde_qs::from_str(query_string).unwrap();

        assert_eq!(response.body, Some("Hello, world!".to_string()));
        assert_eq!(response.get_body().unwrap(), b"Hello, world!");
        let axum_response = response.into_response();

        assert_eq!(axum_response.status(), StatusCode::OK);
        let headers = axum_response.headers();
        assert_eq!(headers.get("content-type").unwrap(), "text/plain");
    }

    #[test]
    fn deserialize_multiple() {
        let query_string =
            "status=404&body=Hello%2C%20world%21&headers[Content-Type]=text/plain&headers[content-length]=42";

        let response: Response = serde_qs::from_str(query_string).unwrap();

        assert_eq!(response.body, Some("Hello, world!".to_string()));
        assert_eq!(response.get_body().unwrap(), b"Hello, world!");
        let axum_response = response.into_response();

        assert_eq!(axum_response.status(), StatusCode::NOT_FOUND);
        let headers = axum_response.headers();
        assert_eq!(headers.get("content-type").unwrap(), "text/plain");
        assert_eq!(headers.get("content-length").unwrap(), "42");
    }

    #[test]
    fn deserialize_default() {
        let query_string = "";

        let response: Response = serde_qs::from_str(query_string).unwrap();

        assert_eq!(response.body, None);
        assert_eq!(response.get_body().unwrap(), b"");
        let axum_response = response.into_response();

        assert_eq!(axum_response.status(), StatusCode::OK);
        let headers = axum_response.headers();
        assert_eq!(headers.get("content-type").unwrap(), "text/plain");
    }

    #[test]
    fn deserialize_alias() {
        let query_string = "s=500&b=Hello%2C%20world%21&h[content-type]=text/plain";

        let response: Response = serde_qs::from_str(query_string).unwrap();

        assert_eq!(response.body, Some("Hello, world!".to_string()));
        assert_eq!(response.get_body().unwrap(), b"Hello, world!");
        let axum_response = response.into_response();

        assert_eq!(axum_response.status(), StatusCode::INTERNAL_SERVER_ERROR);
        let headers = axum_response.headers();
        assert_eq!(headers.get("content-type").unwrap(), "text/plain");
    }

    #[test]
    fn deserialize_header_alias() {
        let query_string = "h[l]=file:///etc/passwd&h[ct]=text/html&h[c]=cookie%3D42";

        let response: Response = serde_qs::from_str(query_string).unwrap();
        let axum_response = response.into_response();

        let headers = axum_response.headers();
        assert_eq!(headers.get("location").unwrap(), "file:///etc/passwd");
        assert_eq!(headers.get("content-type").unwrap(), "text/html");
        assert_eq!(headers.get("set-cookie").unwrap(), "cookie=42");
    }

    #[test]
    fn deserialize_cors() {
        fn assert_cors(query_string: &str, enabled: bool) {
            let response = serde_qs::from_str::<Response>(query_string)
                .unwrap()
                .into_response();
            let headers = response.headers();
            if enabled {
                assert_eq!(headers.get("access-control-allow-origin").unwrap(), "*");
            } else {
                assert!(headers.get("access-control-allow-origin").is_none());
            }
        }

        assert_cors("b=body", false);
        assert_cors("b=body&cors=true", true);
        assert_cors("b=body&cors=false", false);
        assert_cors("b=body&cors=n", false);
        assert_cors("b=body&cors=", true);
        assert_cors("b=body&cors", true);
    }

    #[test]
    fn deserialize_b64() {
        let query_string = "body.b64=SGVsbG8sIHdvcmxkIQ%3D"; // invalid padding, should be '=='

        let response: Response = serde_qs::from_str(query_string).unwrap();
        dbg!(&response);

        assert_eq!(response.body_b64, Some("SGVsbG8sIHdvcmxkIQ=".to_string()));
        assert_eq!(response.get_body().unwrap(), b"Hello, world!");
    }
}
