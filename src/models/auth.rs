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
    /// La API respondió con un error interno (5xx) o un cuerpo inesperado.
    Server(u16),
    /// La API no responde (red, DNS, CORS...).
    Network,
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::InvalidCredentials => "Correo o contraseña incorrectos.",
            Self::EmailTaken => "Ya existe una cuenta con ese correo.",
            Self::TooManyRequests => "Demasiados intentos. Espera un minuto.",
            Self::Rejected(message) => message,
            Self::Server(status) => {
                return write!(f, "Error del servidor ({status}). Inténtalo más tarde.");
            }
            Self::Network => "No se pudo conectar con el servidor.",
        };
        f.write_str(message)
    }
}

impl std::error::Error for AuthError {}
