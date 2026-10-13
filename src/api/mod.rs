mod account;

use axum::{
    Json, Router,
    extract::State,
    routing::{get, post},
};
use serde::Serialize;

use crate::{error::ApiError, server::AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/status", get(status))
        .route("/auth/register", post(account::register))
        .route("/auth/login", post(account::login))
        .route("/auth/logout", post(account::logout))
        .route("/me", get(account::me).patch(account::update_me))
        .fallback(|| async { ApiError::NotFound })
}

#[derive(Serialize)]
struct Status {
    /// Lets a second Akoyo launch recognise that Akoyo already owns the port.
    app: &'static str,
    version: &'static str,
    needs_setup: bool,
}

async fn status(State(state): State<AppState>) -> Result<Json<Status>, ApiError> {
    let users: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
        .fetch_one(&state.db)
        .await?;
    Ok(Json(Status {
        app: "akoyo",
        version: env!("CARGO_PKG_VERSION"),
        needs_setup: users == 0,
    }))
}
