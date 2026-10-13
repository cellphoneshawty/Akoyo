use std::net::TcpListener;

use axum::Router;
use sqlx::SqlitePool;
use tokio::sync::oneshot;

use crate::{api, auth, config::Paths, db, web};

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
}

impl AppState {
    pub async fn init(paths: &Paths) -> anyhow::Result<Self> {
        let db = db::connect(&paths.database_file).await?;
        tokio::spawn(auth::cleanup_expired_sessions(db.clone()));
        Ok(Self { db })
    }
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .nest("/api", api::router())
        .fallback(web::static_handler)
        .with_state(state)
}

pub async fn serve(
    listener: TcpListener,
    state: AppState,
    shutdown: oneshot::Receiver<()>,
) -> anyhow::Result<()> {
    let listener = tokio::net::TcpListener::from_std(listener)?;
    axum::serve(listener, router(state))
        .with_graceful_shutdown(async {
            let _ = shutdown.await;
        })
        .await?;
    Ok(())
}
