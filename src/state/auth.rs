use leptos::prelude::*;

use crate::config::storage_keys;
use crate::models::user::{Session, User};
use crate::services::storage;

/// Estado de autenticación compartido por toda la app.
///
/// Es `Copy`, así que se puede mover libremente a closures y tareas async.
#[derive(Clone, Copy)]
pub struct AuthState {
    session: RwSignal<Option<Session>>,
}

impl AuthState {
    /// Restaura la sesión guardada, si la hay.
    fn restore() -> Self {
        Self {
            session: RwSignal::new(storage::load(storage_keys::SESSION)),
        }
    }

    pub fn is_authenticated(&self) -> bool {
        self.session.with(Option::is_some)
    }

    pub fn user(&self) -> Option<User> {
        self.session
            .with(|session| session.as_ref().map(|s| s.user.clone()))
    }

    pub fn sign_in(&self, session: Session) {
        storage::save(storage_keys::SESSION, &session);
        self.session.set(Some(session));
    }

    pub fn sign_out(&self) {
        storage::remove(storage_keys::SESSION);
        self.session.set(None);
    }
}

/// Crea el estado de autenticación y lo registra como contexto. Llamar una vez, en la raíz.
pub fn provide_auth() {
    provide_context(AuthState::restore());
}

/// Obtiene el estado de autenticación del contexto.
pub fn use_auth() -> AuthState {
    expect_context::<AuthState>()
}
