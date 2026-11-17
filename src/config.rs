//! Constantes de configuración de la aplicación.

/// Nombre visible de la aplicación.
pub const APP_NAME: &str = "User Inspector";

/// URL base de la API, sin barra final. Se fija al compilar con la variable
/// de entorno `API_URL`; por defecto, la API local.
pub const API_URL: &str = match option_env!("API_URL") {
    Some(url) => url,
    None => "http://localhost:8080",
};

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
}
