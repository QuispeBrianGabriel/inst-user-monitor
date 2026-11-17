use leptos::prelude::*;
use leptos_router::components::A;

use crate::config::{routes, APP_NAME};
use crate::state::auth::use_auth;

#[component]
pub fn Header() -> impl IntoView {
    let auth = use_auth();
    let user_name = move || auth.user().map(|user| user.name).unwrap_or_default();

    view! {
        <header class="header">
            <A href=routes::DASHBOARD attr:class="brand">
                <span class="brand__mark" aria-hidden="true"></span>
                <span>{APP_NAME}</span>
            </A>
            <div class="header__session">
                <span class="header__user">{user_name}</span>
                <button type="button" class="btn btn--ghost" on:click=move |_| auth.sign_out()>
                    "Cerrar sesión"
                </button>
            </div>
        </header>
    }
}
