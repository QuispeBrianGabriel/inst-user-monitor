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
    pub name: String,
    pub email: String,
    pub password: String,
}

/// Errores de autenticación que la UI sabe mostrar.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthError {
    InvalidCredentials,
    EmailTaken,
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::InvalidCredentials => "Correo o contraseña incorrectos.",
            Self::EmailTaken => "Ya existe una cuenta con ese correo.",
        };
        f.write_str(message)
    }
}

impl std::error::Error for AuthError {}
