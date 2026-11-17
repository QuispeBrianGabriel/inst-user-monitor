use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="header">
            <span class="brand">"Inst User Monitor"</span>
            <nav class="nav">
                <A href="/">"Inicio"</A>
            </nav>
        </header>
    }
}
