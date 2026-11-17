//! Servicio de autenticación.
//!
//! Por ahora es una **implementación simulada** que no necesita backend: guarda
//! en `localStorage` la lista de usuarios registrados (solo nombre y correo,
//! nunca contraseñas) y emite tokens ficticios. Cuando exista la API, basta con
//! reimplementar `login` y `register` con peticiones HTTP manteniendo las
//! mismas firmas; el resto de la app no cambia.

use gloo_timers::future::TimeoutFuture;

use crate::config::storage_keys;
use crate::models::auth::{AuthError, LoginRequest, RegisterRequest};
use crate::models::user::{Session, User};
use crate::services::storage;

/// Latencia simulada para que la UI muestre sus estados de carga.
const SIMULATED_LATENCY_MS: u32 = 700;

pub async fn login(request: LoginRequest) -> Result<Session, AuthError> {
    TimeoutFuture::new(SIMULATED_LATENCY_MS).await;

    let email = normalize_email(&request.email);
    registered_users()
        .into_iter()
        .find(|user| user.email == email)
        .map(new_session)
        .ok_or(AuthError::InvalidCredentials)
}

pub async fn register(request: RegisterRequest) -> Result<Session, AuthError> {
    TimeoutFuture::new(SIMULATED_LATENCY_MS).await;

    let email = normalize_email(&request.email);
    let mut users = registered_users();
    if users.iter().any(|user| user.email == email) {
        return Err(AuthError::EmailTaken);
    }

    let user = User {
        name: request.name.trim().to_owned(),
        email,
    };
    users.push(user.clone());
    storage::save(storage_keys::MOCK_USERS, &users);
    Ok(new_session(user))
}

fn registered_users() -> Vec<User> {
    storage::load(storage_keys::MOCK_USERS).unwrap_or_default()
}

fn new_session(user: User) -> Session {
    let issued_at = now_ms();
    Session {
        token: format!("mock.{issued_at}"),
        user,
    }
}

fn normalize_email(email: &str) -> String {
    email.trim().to_lowercase()
}

/// Marca de tiempo en milisegundos, solo para que cada token ficticio sea distinto.
fn now_ms() -> u64 {
    web_sys::window()
        .and_then(|window| window.performance())
        .map(|performance| performance.time_origin() + performance.now())
        .unwrap_or_default() as u64
}

#[cfg(test)]
mod tests {
    use super::normalize_email;

    #[test]
    fn normalizes_email_case_and_whitespace() {
        assert_eq!(normalize_email("  Ana@Example.COM "), "ana@example.com");
    }
}
