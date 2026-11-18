//! Servicio de autenticación contra la API (`config::api_url()`).
//!
//! - `login`: `POST /auth/login` y luego `GET /auth/me` para obtener el usuario.
//! - `register`: `POST /auth/register` y, a continuación, `login`, para que
//!   el usuario entre directamente sin volver a escribir sus datos.

use gloo_net::http::{Request, Response};
use serde::{de::DeserializeOwned, Deserialize};

use crate::config::api_url;
use crate::models::auth::{AuthError, LoginRequest, RegisterRequest};
use crate::models::user::{Session, User};

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
}

/// Cuerpo de error de la API: `{"error": "<código>", "message": "<texto>"}`.
#[derive(Deserialize)]
struct ApiError {
    error: String,
    message: String,
}

impl From<gloo_net::Error> for AuthError {
    fn from(_: gloo_net::Error) -> Self {
        Self::Network
    }
}

pub async fn login(request: LoginRequest) -> Result<Session, AuthError> {
    let response = Request::post(&url("/auth/login"))
        .json(&request)?
        .send()
        .await?;
    let token: TokenResponse = parse(response).await?;
    let user = me(&token.access_token).await?;
    Ok(Session {
        user,
        token: token.access_token,
    })
}

pub async fn register(request: RegisterRequest) -> Result<Session, AuthError> {
    let response = Request::post(&url("/auth/register"))
        .json(&request)?
        .send()
        .await?;
    parse::<User>(response).await?;
    login(LoginRequest {
        email: request.email,
        password: request.password,
    })
    .await
}

async fn me(token: &str) -> Result<User, AuthError> {
    let response = Request::get(&url("/auth/me"))
        .header("Authorization", &format!("Bearer {token}"))
        .send()
        .await?;
    parse(response).await
}

/// Devuelve el cuerpo si la respuesta es 2xx; si no, traduce el error de la API.
async fn parse<T: DeserializeOwned>(response: Response) -> Result<T, AuthError> {
    if response.ok() {
        return Ok(response.json().await?);
    }
    if response.status() == 429 {
        return Err(AuthError::TooManyRequests);
    }
    let body: ApiError = response.json().await?;
    Err(match body.error.as_str() {
        "invalid_credentials" => AuthError::InvalidCredentials,
        "email_taken" => AuthError::EmailTaken,
        _ => AuthError::Rejected(body.message),
    })
}

fn url(path: &str) -> String {
    format!("{}{path}", api_url())
}
