//! Envoltorio tipado sobre `localStorage`.
//!
//! Todas las operaciones son tolerantes a fallos: si el almacenamiento no está
//! disponible (modo privado, cuota llena, datos corruptos) se comportan como si
//! no hubiera nada guardado, en lugar de romper la app.

use serde::{de::DeserializeOwned, Serialize};
use web_sys::Storage;

fn local_storage() -> Option<Storage> {
    web_sys::window()?.local_storage().ok().flatten()
}

/// Lee y deserializa el valor de `key`.
pub fn load<T: DeserializeOwned>(key: &str) -> Option<T> {
    let raw = local_storage()?.get_item(key).ok().flatten()?;
    serde_json::from_str(&raw).ok()
}

/// Serializa y guarda `value` en `key`.
pub fn save<T: Serialize>(key: &str, value: &T) {
    let (Some(storage), Ok(raw)) = (local_storage(), serde_json::to_string(value)) else {
        return;
    };
    let _ = storage.set_item(key, &raw);
}

/// Elimina `key`.
pub fn remove(key: &str) {
    if let Some(storage) = local_storage() {
        let _ = storage.remove_item(key);
    }
}
