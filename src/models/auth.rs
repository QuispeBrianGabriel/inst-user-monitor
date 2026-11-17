use std::fmt;

use serde::Serialize;

/// Datos enviados para iniciar sesión.
#[derive(Debug, Clone, Serialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

/// Datos enviados para crear una cuenta.
#[derive(Debug, Clone, Serialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
}

/// Errores de autenticación que la UI sabe mostrar.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthError {
    InvalidCredentials,
    EmailTaken,
    TooManyRequests,
    /// Rechazo de la API con su mensaje (p. ej. contraseña débil).
    Rejected(String),
    /// La API no responde o la respuesta no es la esperada.
    Network,
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::InvalidCredentials => "Correo o contraseña incorrectos.",
            Self::EmailTaken => "Ya existe una cuenta con ese correo.",
            Self::TooManyRequests => "Demasiados intentos. Espera un minuto.",
            Self::Rejected(message) => message,
            Self::Network => "No se pudo conectar con el servidor.",
        };
        f.write_str(message)
    }
}

impl std::error::Error for AuthError {}
