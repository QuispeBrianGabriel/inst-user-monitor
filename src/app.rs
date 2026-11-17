use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

use crate::components::header::Header;
use crate::pages::{home::Home, not_found::NotFound};

/// Componente raíz: define el layout general y las rutas.
#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <div class="layout">
                <Header />
                <main class="content">
                    <Routes fallback=|| view! { <NotFound /> }>
                        <Route path=path!("/") view=Home />
                    </Routes>
                </main>
            </div>
        </Router>
    }
}
