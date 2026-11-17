use leptos::{ev::SubmitEvent, prelude::*, task::spawn_local};

use crate::components::ui::{
    alert::Alert, button::SubmitButton, password_meter::PasswordMeter, text_field::TextField,
};
use crate::models::auth::RegisterRequest;
use crate::services::auth_service;
use crate::state::auth::use_auth;
use crate::utils::validation::{
    validate_email, validate_new_password, validate_password_confirmation,
};

/// Formulario de creación de cuenta. Mismo patrón de validación que `LoginForm`.
#[component]
pub fn RegisterForm() -> impl IntoView {
    let auth = use_auth();

    let email = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());
    let confirmation = RwSignal::new(String::new());
    let submitted = RwSignal::new(false);
    let pending = RwSignal::new(false);
    let server_error = RwSignal::new(None::<String>);

    let shown =
        move |result: Result<(), &'static str>| submitted.get().then_some(result.err()).flatten();
    let email_error = Signal::derive(move || shown(email.with(|v| validate_email(v))));
    let password_error = Signal::derive(move || shown(password.with(|v| validate_new_password(v))));
    let confirmation_error = Signal::derive(move || {
        shown(password.with(|p| confirmation.with(|c| validate_password_confirmation(p, c))))
    });

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        if pending.get_untracked() {
            return;
        }
        submitted.set(true);
        let has_errors = [email_error, password_error, confirmation_error]
            .iter()
            .any(|error| error.get_untracked().is_some());
        if has_errors {
            return;
        }

        let request = RegisterRequest {
            email: email.get_untracked().trim().to_owned(),
            password: password.get_untracked(),
        };
        pending.set(true);
        server_error.set(None);
        spawn_local(async move {
            match auth_service::register(request).await {
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
                id="register-email"
                label="Correo electrónico"
                input_type="email"
                autocomplete="email"
                placeholder="tu@correo.com"
                value=email
                error=email_error
            />
            <TextField
                id="register-password"
                label="Contraseña"
                input_type="password"
                autocomplete="new-password"
                placeholder="Mínimo 8 caracteres"
                value=password
                error=password_error
            >
                <PasswordMeter password=password />
            </TextField>
            <TextField
                id="register-confirmation"
                label="Confirmar contraseña"
                input_type="password"
                autocomplete="new-password"
                placeholder="Repite la contraseña"
                value=confirmation
                error=confirmation_error
            />
            <SubmitButton label="Crear cuenta" pending_label="Creando cuenta…" pending=pending />
        </form>
    }
}
