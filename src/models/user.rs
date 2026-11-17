use serde::{Deserialize, Serialize};

/// Usuario autenticado.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub email: String,
}

/// Sesión activa: el usuario y el token con el que se autentican las peticiones.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Session {
    pub user: User,
    pub token: String,
}
