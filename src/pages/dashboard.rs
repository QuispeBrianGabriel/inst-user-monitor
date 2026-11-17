use leptos::prelude::*;

use crate::state::auth::use_auth;

/// Panel principal tras iniciar sesión.
#[component]
pub fn Dashboard() -> impl IntoView {
    let auth = use_auth();
    let name = move || auth.user().map(|user| user.name).unwrap_or_default();

    view! {
        <section class="page">
            <h1>"Hola, " {name}</h1>
            <p class="muted">"Sesión iniciada. El panel de inspección se construirá aquí."</p>
        </section>
    }
}
