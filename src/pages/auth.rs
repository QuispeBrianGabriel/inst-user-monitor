use leptos::prelude::*;
use leptos_router::{components::Redirect, hooks::use_query_map};

use crate::components::auth::{login_form::LoginForm, register_form::RegisterForm};
use crate::config::{routes, APP_NAME};
use crate::state::auth::use_auth;

/// Pestaña activa de la página, controlada por `?mode=` para que sea enlazable.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AuthMode {
    Login,
    Register,
}

impl AuthMode {
    fn from_query(mode: Option<&str>) -> Self {
        match mode {
            Some("register") => Self::Register,
            _ => Self::Login,
        }
    }
}

/// Página de inicio: login + registro. Si ya hay sesión, redirige al panel.
#[component]
pub fn AuthPage() -> impl IntoView {
    let auth = use_auth();
    let query = use_query_map();
    let mode = Memo::new(move |_| AuthMode::from_query(query.read().get("mode").as_deref()));
    let is_login = move || mode.get() == AuthMode::Login;

    view! {
        <Show
            when=move || !auth.is_authenticated()
            fallback=|| view! { <Redirect path=routes::DASHBOARD /> }
        >
            <main class="auth">
                <header class="auth__brand">
                    <span class="brand__mark" aria-hidden="true"></span>
                    <h1 class="auth__title">{APP_NAME}</h1>
                    <p class="auth__status">
                        <span class="dot" aria-hidden="true"></span>
                        "Sistema en línea"
                    </p>
                </header>

                <section class="auth-card">
                    <nav class="auth-tabs" aria-label="Acceso">
                        <a
                            href=routes::AUTH
                            class="auth-tabs__tab"
                            class:is-active=is_login
                            aria-current=move || is_login().then_some("page")
                        >
                            "Iniciar sesión"
                        </a>
                        <a
                            href=routes::REGISTER
                            class="auth-tabs__tab"
                            class:is-active=move || !is_login()
                            aria-current=move || (!is_login()).then_some("page")
                        >
                            "Crear cuenta"
                        </a>
                    </nav>

                    {move || {
                        if is_login() {
                            view! { <LoginForm /> }.into_any()
                        } else {
                            view! { <RegisterForm /> }.into_any()
                        }
                    }}
                </section>
            </main>
        </Show>
    }
}

#[cfg(test)]
mod tests {
    use super::AuthMode;

    #[test]
    fn mode_from_query() {
        assert_eq!(AuthMode::from_query(None), AuthMode::Login);
        assert_eq!(AuthMode::from_query(Some("login")), AuthMode::Login);
        assert_eq!(AuthMode::from_query(Some("otro")), AuthMode::Login);
        assert_eq!(AuthMode::from_query(Some("register")), AuthMode::Register);
    }
}
