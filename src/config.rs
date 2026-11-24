//! Configuración de la aplicación.

use std::sync::OnceLock;

/// Nombre visible de la aplicación.
pub const APP_NAME: &str = "User Inspector";

/// URL de la API cuando no hay configuración en runtime (p. ej. en tests).
const DEFAULT_API_URL: &str = "http://localhost:8080";

/// URL base de la API, sin barra final.
///
/// Se lee **en runtime** de `window.__APP_CONFIG__.apiUrl`, que define
/// `/config.js` antes de cargar el wasm. En desarrollo ese fichero sale de
/// `public/config.js`; en producción lo genera el contenedor al arrancar a
/// partir de la variable de entorno `API_URL` (factor III de 12-factor).
pub fn api_url() -> &'static str {
    static API_URL: OnceLock<String> = OnceLock::new();
    API_URL.get_or_init(|| {
        runtime_value("apiUrl")
            .unwrap_or_else(|| DEFAULT_API_URL.to_owned())
            .trim_end_matches('/')
            .to_owned()
    })
}

/// Lee `window.__APP_CONFIG__[key]` como texto, si existe y no está vacío.
fn runtime_value(key: &str) -> Option<String> {
    if !cfg!(target_arch = "wasm32") {
        return None;
    }
    let window = web_sys::window()?;
    let config = js_sys::Reflect::get(&window, &"__APP_CONFIG__".into()).ok()?;
    if !config.is_object() {
        return None;
    }
    js_sys::Reflect::get(&config, &key.into())
        .ok()?
        .as_string()
        .filter(|value| !value.is_empty())
}

/// Rutas de la aplicación, para enlaces y redirecciones.
///
/// Las definiciones de `<Route>` en `app.rs` usan la macro `path!`, que exige
/// literales; mantén ambos sitios sincronizados.
pub mod routes {
    pub const AUTH: &str = "/";
    pub const REGISTER: &str = "/?mode=register";
    pub const DASHBOARD: &str = "/dashboard";

    /// Detalle de una cuenta monitoreada (`/dashboard/accounts/:id`).
    pub fn account(id: &str) -> String {
        format!("{DASHBOARD}/accounts/{id}")
    }
}

/// Claves de `localStorage`, con prefijo para no colisionar con otras apps.
pub mod storage_keys {
    pub const SESSION: &str = "user-inspector.session";
}
