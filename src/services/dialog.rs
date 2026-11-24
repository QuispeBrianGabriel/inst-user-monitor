//! Diálogos nativos del navegador.

/// Pide confirmación con `window.confirm`. Si no se puede mostrar, `false`:
/// una acción destructiva nunca se confirma sola.
pub fn confirm(message: &str) -> bool {
    web_sys::window()
        .and_then(|window| window.confirm_with_message(message).ok())
        .unwrap_or(false)
}
