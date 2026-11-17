use leptos::prelude::*;
use leptos_router::{
    components::{ParentRoute, Route, Router, Routes},
    path,
};

use crate::components::protected_layout::ProtectedLayout;
use crate::pages::{auth::AuthPage, dashboard::Dashboard, not_found::NotFound};
use crate::state::auth::provide_auth;

/// Componente raíz: estado global y rutas.
///
/// Rutas (ver también `config::routes`):
/// - `/`            → login y registro (`?mode=register`)
/// - `/dashboard`   → privada, dentro de `ProtectedLayout`
#[component]
pub fn App() -> impl IntoView {
    provide_auth();

    view! {
        <Router>
            <Routes fallback=|| view! { <NotFound /> }>
                <Route path=path!("/") view=AuthPage />
                <ParentRoute path=path!("/dashboard") view=ProtectedLayout>
                    <Route path=path!("") view=Dashboard />
                </ParentRoute>
            </Routes>
        </Router>
    }
}
