use axum;
use serde_json;
use std::collections::HashMap;
use super::err::Error;

impl axum::response::IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let mut payload = HashMap::<&str, String>::new();
        match self {
            Error::Message(m) => {
                payload.insert("code", m.code.unwrap_or("none".into()));
                payload.insert("message", m.message);
            },
            Error::Error(err) => {
                payload.insert("message", "System Error".into());
                if let Some(code) = err.code {
                    payload.insert("code", format!("error.{}", code));
                }
                if let Some(info) = err.info {
                    payload.insert("info", info);
                }
                if let Some(err) = err.error {
                    payload.insert("error", format!("{:?}", err));
                }
                if let Some(trace) = err.trace {
                    payload.insert("trace", trace);
                }
                if let Some(ext) = err.ext {
                    payload.insert("ext", ext);
                }
            }
        };
        let json = serde_json::to_string(&payload).unwrap();
        let body = axum::body::Body::new(json);
        axum::response::Response::builder()
            .status(axum::http::StatusCode::OK)
            .header("Content-Type", "application/json")
            .body(body)
            .unwrap()
    }
}