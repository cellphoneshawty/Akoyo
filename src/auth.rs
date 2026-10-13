use std::{sync::LazyLock, time::Duration};

use anyhow::anyhow;
use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::SaltString,
};
use axum::{extract::FromRequestParts, http::request::Parts};
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use serde::Serialize;
use sha2::{Digest, Sha256};
use sqlx::SqlitePool;
use tracing::warn;

use crate::{db::now, error::ApiError, server::AppState};

pub const SESSION_COOKIE: &str = "akoyo_session";
const SESSION_DAYS: i64 = 30;

pub const LANGUAGES: &[&str] = &["en", "ru"];

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub is_admin: bool,
    pub language: String,
}

/// Extractor for handlers that require a signed-in user.
pub struct CurrentUser(pub User);

impl FromRequestParts<AppState> for CurrentUser {
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, ApiError> {
        let jar = CookieJar::from_headers(&parts.headers);
        let token = jar
            .get(SESSION_COOKIE)
            .ok_or(ApiError::Unauthorized("not_signed_in"))?;

        let user = sqlx::query_as::<_, User>(
            "SELECT u.id, u.username, u.is_admin, u.language
             FROM sessions s JOIN users u ON u.id = s.user_id
             WHERE s.token_hash = ?1 AND s.expires_at > ?2",
        )
        .bind(hash_token(token.value()))
        .bind(now())
        .fetch_optional(&state.db)
        .await?
        .ok_or(ApiError::Unauthorized("not_signed_in"))?;

        Ok(Self(user))
    }
}

pub async fn hash_password(password: String) -> anyhow::Result<String> {
    tokio::task::spawn_blocking(move || {
        let mut salt = [0u8; 16];
        getrandom::fill(&mut salt).map_err(|err| anyhow!("no randomness available: {err}"))?;
        let salt = SaltString::encode_b64(&salt).map_err(|err| anyhow!("{err}"))?;
        let hash = Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map_err(|err| anyhow!("could not hash password: {err}"))?;
        Ok(hash.to_string())
    })
    .await?
}

/// Verifies a password. When the user doesn't exist (`hash` is `None`) a dummy
/// hash is checked anyway, so response time doesn't reveal which usernames exist.
pub async fn verify_password(password: String, hash: Option<String>) -> anyhow::Result<bool> {
    static DUMMY_HASH: LazyLock<String> = LazyLock::new(|| {
        let salt = SaltString::encode_b64(b"akoyo-dummy-salt").expect("valid salt");
        Argon2::default()
            .hash_password(b"akoyo-dummy-password", &salt)
            .expect("dummy hash")
            .to_string()
    });

    tokio::task::spawn_blocking(move || {
        let exists = hash.is_some();
        let hash = hash.unwrap_or_else(|| DUMMY_HASH.clone());
        let parsed = PasswordHash::new(&hash).map_err(|err| anyhow!("stored hash is invalid: {err}"))?;
        let valid = Argon2::default()
            .verify_password(password.as_bytes(), &parsed)
            .is_ok();
        Ok(exists && valid)
    })
    .await?
}

/// Creates a session and returns the cookie to send back to the browser.
pub async fn create_session(db: &SqlitePool, user_id: i64) -> anyhow::Result<Cookie<'static>> {
    let mut bytes = [0u8; 32];
    getrandom::fill(&mut bytes).map_err(|err| anyhow!("no randomness available: {err}"))?;
    let token = hex::encode(bytes);

    let created_at = now();
    sqlx::query(
        "INSERT INTO sessions (token_hash, user_id, created_at, expires_at) VALUES (?1, ?2, ?3, ?4)",
    )
    .bind(hash_token(&token))
    .bind(user_id)
    .bind(created_at)
    .bind(created_at + SESSION_DAYS * 24 * 60 * 60)
    .execute(db)
    .await?;

    Ok(Cookie::build((SESSION_COOKIE, token))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .max_age(time::Duration::days(SESSION_DAYS))
        .build())
}

pub async fn delete_session(db: &SqlitePool, token: &str) -> anyhow::Result<()> {
    sqlx::query("DELETE FROM sessions WHERE token_hash = ?1")
        .bind(hash_token(token))
        .execute(db)
        .await?;
    Ok(())
}

pub fn removal_cookie() -> Cookie<'static> {
    Cookie::build((SESSION_COOKIE, "")).path("/").build()
}

pub async fn cleanup_expired_sessions(db: SqlitePool) {
    let mut interval = tokio::time::interval(Duration::from_secs(60 * 60));
    loop {
        interval.tick().await;
        if let Err(err) = sqlx::query("DELETE FROM sessions WHERE expires_at <= ?1")
            .bind(now())
            .execute(&db)
            .await
        {
            warn!("could not clean up expired sessions: {err}");
        }
    }
}

fn hash_token(token: &str) -> String {
    hex::encode(Sha256::digest(token.as_bytes()))
}
