use axum::{Json, extract::State, http::StatusCode};
use axum_extra::extract::cookie::CookieJar;
use serde::Deserialize;

use crate::{
    auth::{self, CurrentUser, LANGUAGES, SESSION_COOKIE, User},
    db::{is_unique_violation, now},
    error::ApiError,
    server::AppState,
};

#[derive(Deserialize)]
pub struct RegisterRequest {
    username: String,
    password: String,
    language: Option<String>,
}

/// Creates an account and signs it in. The very first account becomes the admin.
pub async fn register(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(body): Json<RegisterRequest>,
) -> Result<(CookieJar, Json<User>), ApiError> {
    let username = body.username.trim();
    validate_username(username)?;
    validate_password(&body.password)?;
    let language = body
        .language
        .as_deref()
        .filter(|lang| LANGUAGES.contains(lang))
        .unwrap_or("en");

    let password_hash = auth::hash_password(body.password).await?;

    // Deciding admin inside the INSERT keeps it atomic if two people register at once.
    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (username, password_hash, is_admin, language, created_at)
         VALUES (?1, ?2, (SELECT COUNT(*) = 0 FROM users), ?3, ?4)
         RETURNING id, username, is_admin, language",
    )
    .bind(username)
    .bind(&password_hash)
    .bind(language)
    .bind(now())
    .fetch_one(&state.db)
    .await
    .map_err(|err| {
        if is_unique_violation(&err) {
            ApiError::Conflict("username_taken")
        } else {
            err.into()
        }
    })?;

    let cookie = auth::create_session(&state.db, user.id).await?;
    Ok((jar.add(cookie), Json(user)))
}

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

pub async fn login(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(body): Json<LoginRequest>,
) -> Result<(CookieJar, Json<User>), ApiError> {
    let row = sqlx::query_as::<_, (i64, String, bool, String, String)>(
        "SELECT id, username, is_admin, language, password_hash FROM users WHERE username = ?1",
    )
    .bind(body.username.trim())
    .fetch_optional(&state.db)
    .await?;

    let hash = row.as_ref().map(|row| row.4.clone());
    if !auth::verify_password(body.password, hash).await? {
        return Err(ApiError::Unauthorized("invalid_credentials"));
    }
    let Some((id, username, is_admin, language, _)) = row else {
        return Err(ApiError::Unauthorized("invalid_credentials"));
    };

    let cookie = auth::create_session(&state.db, id).await?;
    let user = User {
        id,
        username,
        is_admin,
        language,
    };
    Ok((jar.add(cookie), Json(user)))
}

pub async fn logout(
    State(state): State<AppState>,
    jar: CookieJar,
) -> Result<(CookieJar, StatusCode), ApiError> {
    if let Some(cookie) = jar.get(SESSION_COOKIE) {
        auth::delete_session(&state.db, cookie.value()).await?;
    }
    Ok((jar.remove(auth::removal_cookie()), StatusCode::NO_CONTENT))
}

pub async fn me(CurrentUser(user): CurrentUser) -> Json<User> {
    Json(user)
}

#[derive(Deserialize)]
pub struct UpdateMeRequest {
    language: Option<String>,
}

pub async fn update_me(
    State(state): State<AppState>,
    CurrentUser(mut user): CurrentUser,
    Json(body): Json<UpdateMeRequest>,
) -> Result<Json<User>, ApiError> {
    if let Some(language) = body.language {
        if !LANGUAGES.contains(&language.as_str()) {
            return Err(ApiError::BadRequest("unsupported_language"));
        }
        sqlx::query("UPDATE users SET language = ?1 WHERE id = ?2")
            .bind(&language)
            .bind(user.id)
            .execute(&state.db)
            .await?;
        user.language = language;
    }
    Ok(Json(user))
}

fn validate_username(username: &str) -> Result<(), ApiError> {
    let length_ok = (3..=32).contains(&username.chars().count());
    let chars_ok = username
        .chars()
        .all(|c| c.is_alphanumeric() || matches!(c, '_' | '-' | '.'));
    if length_ok && chars_ok {
        Ok(())
    } else {
        Err(ApiError::BadRequest("invalid_username"))
    }
}

fn validate_password(password: &str) -> Result<(), ApiError> {
    if (8..=256).contains(&password.chars().count()) {
        Ok(())
    } else {
        Err(ApiError::BadRequest("weak_password"))
    }
}
