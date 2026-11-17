use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <section class="page">
            <h1>"404"</h1>
            <p>"La página no existe."</p>
            <A href="/">"Volver al inicio"</A>
        </section>
    }
}
