//! Constantes de configuración de la aplicación.

/// Nombre visible de la aplicación.
pub const APP_NAME: &str = "User Inspector";

/// Rutas de la aplicación, para enlaces y redirecciones.
///
/// Las definiciones de `<Route>` en `app.rs` usan la macro `path!`, que exige
/// literales; mantén ambos sitios sincronizados.
pub mod routes {
    pub const AUTH: &str = "/";
    pub const REGISTER: &str = "/?mode=register";
    pub const DASHBOARD: &str = "/dashboard";
}

/// Claves de `localStorage`, con prefijo para no colisionar con otras apps.
pub mod storage_keys {
    pub const SESSION: &str = "user-inspector.session";
    pub const MOCK_USERS: &str = "user-inspector.mock-users";
}
