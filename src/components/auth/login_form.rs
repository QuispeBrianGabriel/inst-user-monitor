use leptos::{ev::SubmitEvent, prelude::*, task::spawn_local};

use crate::components::ui::{alert::Alert, button::SubmitButton, text_field::TextField};
use crate::models::auth::LoginRequest;
use crate::services::auth_service;
use crate::state::auth::use_auth;
use crate::utils::validation::{validate_email, validate_password_present};

/// Formulario de inicio de sesión.
///
/// Los errores de campo aparecen tras el primer intento de envío y a partir de
/// ahí se recalculan en vivo. Al autenticarse, la página redirige sola.
#[component]
pub fn LoginForm() -> impl IntoView {
    let auth = use_auth();

    let email = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());
    let submitted = RwSignal::new(false);
    let pending = RwSignal::new(false);
    let server_error = RwSignal::new(None::<String>);

    let email_error = Signal::derive(move || {
        submitted
            .get()
            .then(|| email.with(|v| validate_email(v).err()))
            .flatten()
    });
    let password_error = Signal::derive(move || {
        submitted
            .get()
            .then(|| password.with(|v| validate_password_present(v).err()))
            .flatten()
    });

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        if pending.get_untracked() {
            return;
        }
        submitted.set(true);
        if email_error.get_untracked().is_some() || password_error.get_untracked().is_some() {
            return;
        }

        let request = LoginRequest {
            email: email.get_untracked().trim().to_owned(),
            password: password.get_untracked(),
        };
        pending.set(true);
        server_error.set(None);
        spawn_local(async move {
            match auth_service::login(request).await {
                Ok(session) => auth.sign_in(session),
                Err(err) => {
                    server_error.set(Some(err.to_string()));
                    pending.set(false);
                }
            }
        });
    };

    view! {
        <form class="auth-form" on:submit=on_submit novalidate>
            <Alert message=server_error />
            <TextField
                id="login-email"
                label="Correo electrónico"
                input_type="email"
                autocomplete="email"
                placeholder="tu@correo.com"
                value=email
                error=email_error
            />
            <TextField
                id="login-password"
                label="Contraseña"
                input_type="password"
                autocomplete="current-password"
                placeholder="••••••••"
                value=password
                error=password_error
            />
            <SubmitButton label="Iniciar sesión" pending_label="Verificando…" pending=pending />
        </form>
    }
}
