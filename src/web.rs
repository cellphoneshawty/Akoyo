use std::borrow::Cow;

use axum::{
    body::Body,
    http::{StatusCode, Uri, header},
    response::{IntoResponse, Response},
};
use rust_embed::RustEmbed;

/// The built web UI (`web/dist`), compiled into the executable.
#[derive(RustEmbed)]
#[folder = "web/dist/"]
struct Assets;

pub async fn static_handler(uri: Uri) -> Response {
    let path = uri.path().trim_start_matches('/');
    if let Some(response) = asset(path) {
        return response;
    }

    // A missing file is a real 404; anything else is a client-side route.
    let is_file = path.rsplit('/').next().is_some_and(|name| name.contains('.'));
    if is_file {
        return StatusCode::NOT_FOUND.into_response();
    }
    asset("index.html").unwrap_or_else(|| {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            "The Akoyo web interface is missing from this build. \
             Run `npm run build` in the `web` folder, then rebuild Akoyo.",
        )
            .into_response()
    })
}

fn asset(path: &str) -> Option<Response> {
    let path = if path.is_empty() { "index.html" } else { path };
    let file = Assets::get(path)?;

    let mime = mime_guess::from_path(path).first_or_octet_stream();
    // Vite puts content-hashed files in `assets/`, so they never change.
    let cache = if path.starts_with("assets/") {
        "public, max-age=31536000, immutable"
    } else {
        "no-cache"
    };
    let body = match file.data {
        Cow::Borrowed(bytes) => Body::from(bytes),
        Cow::Owned(bytes) => Body::from(bytes),
    };

    Some(
        (
            [
                (header::CONTENT_TYPE, mime.as_ref().to_owned()),
                (header::CACHE_CONTROL, cache.to_owned()),
            ],
            body,
        )
            .into_response(),
    )
}
