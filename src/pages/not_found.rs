use leptos::prelude::*;
use leptos_router::components::A;

use crate::config::routes;

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <section class="page page--centered">
            <h1>"404"</h1>
            <p class="muted">"La página no existe."</p>
            <A href=routes::AUTH attr:class="btn btn--ghost">
                "Volver al inicio"
            </A>
        </section>
    }
}
