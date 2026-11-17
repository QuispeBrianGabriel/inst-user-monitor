use leptos::prelude::*;
use leptos_router::components::{Outlet, Redirect};

use crate::components::header::Header;
use crate::config::routes;
use crate::state::auth::use_auth;

/// Layout de las rutas privadas: redirige al login si no hay sesión.
#[component]
pub fn ProtectedLayout() -> impl IntoView {
    let auth = use_auth();

    view! {
        <Show
            when=move || auth.is_authenticated()
            fallback=|| view! { <Redirect path=routes::AUTH /> }
        >
            <div class="layout">
                <Header />
                <main class="content">
                    <Outlet />
                </main>
            </div>
        </Show>
    }
}
