use leptos::prelude::*;
use leptos_router::{
    components::{ParentRoute, Route, Router, Routes},
    path,
};

use crate::components::protected_layout::ProtectedLayout;
use crate::pages::{
    account::AccountPage, auth::AuthPage, dashboard::Dashboard, not_found::NotFound,
};
use crate::state::auth::provide_auth;

/// Componente raíz: estado global y rutas.
///
/// Rutas (ver también `config::routes`):
/// - `/`            → login y registro (`?mode=register`)
/// - `/dashboard`   → privada, dentro de `ProtectedLayout`: cuentas monitoreadas
/// - `/dashboard/accounts/:id` → privada: detalle de una cuenta
#[component]
pub fn App() -> impl IntoView {
    provide_auth();

    view! {
        <Router>
            <Routes fallback=|| view! { <NotFound /> }>
                <Route path=path!("/") view=AuthPage />
                <ParentRoute path=path!("/dashboard") view=ProtectedLayout>
                    <Route path=path!("") view=Dashboard />
                    <Route path=path!("accounts/:id") view=AccountPage />
                </ParentRoute>
            </Routes>
        </Router>
    }
}
