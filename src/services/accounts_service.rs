//! Cuentas monitoreadas y sus capturas (`/accounts` de la API).
//!
//! Todas las peticiones llevan el token de la sesión. Un 401 se traduce en
//! `ApiError::Unauthorized`: la UI cierra entonces la sesión.

use gloo_net::http::{Request, RequestBuilder, Response};
use serde::{de::DeserializeOwned, Deserialize};

use crate::config::api_url;
use crate::models::account::{
    Account, AccountDetail, AccountRequest, AccountSummary, Snapshot, SnapshotRequest,
};
use crate::models::api::ApiError;

/// Cuerpo de error de la API (formato de Loco), como en `auth_service`.
#[derive(Deserialize)]
struct ApiErrorBody {
    error: String,
    #[serde(default)]
    description: Option<String>,
}

pub async fn list(token: &str) -> Result<Vec<AccountSummary>, ApiError> {
    let response = authorized(Request::get(&url("/accounts")), token)
        .send()
        .await?;
    parse(response).await
}

pub async fn get(token: &str, id: &str) -> Result<AccountDetail, ApiError> {
    let response = authorized(Request::get(&url(&format!("/accounts/{id}"))), token)
        .send()
        .await?;
    parse(response).await
}

pub async fn create(token: &str, request: &AccountRequest) -> Result<Account, ApiError> {
    let response = authorized(Request::post(&url("/accounts")), token)
        .json(request)?
        .send()
        .await?;
    parse(response).await
}

pub async fn update(token: &str, id: &str, request: &AccountRequest) -> Result<Account, ApiError> {
    let response = authorized(Request::put(&url(&format!("/accounts/{id}"))), token)
        .json(request)?
        .send()
        .await?;
    parse(response).await
}

pub async fn delete(token: &str, id: &str) -> Result<(), ApiError> {
    let response = authorized(Request::delete(&url(&format!("/accounts/{id}"))), token)
        .send()
        .await?;
    parse_empty(response).await
}

/// Envía el texto pegado. Si no es una lista válida, la API no guarda nada y
/// responde `invalid_format` con la línea que falla.
pub async fn add_snapshot(
    token: &str,
    account_id: &str,
    request: &SnapshotRequest,
) -> Result<Snapshot, ApiError> {
    let path = format!("/accounts/{account_id}/snapshots");
    let response = authorized(Request::post(&url(&path)), token)
        .json(request)?
        .send()
        .await?;
    parse(response).await
}

pub async fn delete_snapshot(
    token: &str,
    account_id: &str,
    snapshot_id: &str,
) -> Result<(), ApiError> {
    let path = format!("/accounts/{account_id}/snapshots/{snapshot_id}");
    let response = authorized(Request::delete(&url(&path)), token)
        .send()
        .await?;
    parse_empty(response).await
}

fn authorized(request: RequestBuilder, token: &str) -> RequestBuilder {
    request.header("Authorization", &format!("Bearer {token}"))
}

/// Devuelve el cuerpo si la respuesta es 2xx; si no, traduce el error.
async fn parse<T: DeserializeOwned>(response: Response) -> Result<T, ApiError> {
    if response.ok() {
        let status = response.status();
        return response.json().await.map_err(|_| ApiError::Server(status));
    }
    Err(error(response).await)
}

/// Para las respuestas sin cuerpo (204).
async fn parse_empty(response: Response) -> Result<(), ApiError> {
    if response.ok() {
        Ok(())
    } else {
        Err(error(response).await)
    }
}

async fn error(response: Response) -> ApiError {
    let status = response.status();
    match status {
        401 => return ApiError::Unauthorized,
        404 => return ApiError::NotFound,
        413 => return ApiError::TooLarge,
        500.. => return ApiError::Server(status),
        _ => {}
    }
    match response.json::<ApiErrorBody>().await {
        Ok(body) => ApiError::Rejected {
            message: body.description.unwrap_or_else(|| body.error.clone()),
            code: body.error,
        },
        Err(_) => ApiError::Server(status),
    }
}

fn url(path: &str) -> String {
    format!("{}{path}", api_url())
}
