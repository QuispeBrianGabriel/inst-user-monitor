use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

use crate::components::accounts::{
    account_form::AccountForm, accounts_table::AccountsTable, paste_form::PasteForm,
};
use crate::components::ui::modal::Modal;
use crate::config::routes;
use crate::models::account::AccountSummary;
use crate::services::accounts_service;
use crate::state::auth::use_auth;
use crate::utils::format;

/// Panel principal: las cuentas monitoreadas, el estado de sus listas y los
/// cambios desde la última captura. Desde aquí se agregan cuentas y se
/// ingresan sus datos.
#[component]
pub fn Dashboard() -> impl IntoView {
    let auth = use_auth();
    let navigate = use_navigate();

    let reload = RwSignal::new(0u32);
    let accounts = LocalResource::new(move || {
        reload.track();
        let token = auth.token();
        async move { accounts_service::list(&token).await }
    });
    Effect::new(move || {
        if let Some(Err(err)) = accounts.get() {
            auth.handle_error(&err);
        }
    });

    let adding = RwSignal::new(false);
    let pasting = RwSignal::new(false);
    let paste_target = RwSignal::new(None::<(String, String)>);

    let on_created = move |account: crate::models::account::Account| {
        adding.set(false);
        navigate(&routes::account(&account.id), Default::default());
    };
    let on_paste = move |target: (String, String)| {
        paste_target.set(Some(target));
        pasting.set(true);
    };
    let on_pasted = move |_| {
        pasting.set(false);
        reload.update(|n| *n += 1);
    };

    view! {
        <section class="page">
            <header class="page__header">
                <div>
                    <h1>"Cuentas monitoreadas"</h1>
                    <p class="muted">
                        "Registra cuentas de Instagram y pega su lista de seguidores para ver qué cambió."
                    </p>
                </div>
                <button type="button" class="btn btn--accent" on:click=move |_| adding.set(true)>
                    "+ Agregar cuenta"
                </button>
            </header>

            <Suspense fallback=|| {
                view! { <p class="muted">"Cargando cuentas…"</p> }
            }>
                {move || {
                    accounts
                        .get()
                        .map(|result| match result {
                            Err(err) => view! { <p class="alert">{err.to_string()}</p> }.into_any(),
                            Ok(list) if list.is_empty() => {
                                view! {
                                    <div class="empty card">
                                        <h2>"Aún no monitoreas ninguna cuenta"</h2>
                                        <p class="muted">
                                            "Agrega la primera con su usuario de Instagram. Después podrás pegar su lista de seguidores."
                                        </p>
                                        <button
                                            type="button"
                                            class="btn btn--accent"
                                            on:click=move |_| adding.set(true)
                                        >
                                            "+ Agregar cuenta"
                                        </button>
                                    </div>
                                }
                                    .into_any()
                            }
                            Ok(list) => {
                                view! {
                                    <Overview accounts=list.clone() />
                                    <AccountsTable accounts=list on_paste=on_paste />
                                }
                                    .into_any()
                            }
                        })
                }}
            </Suspense>

            <Modal title="Agregar cuenta" open=adding>
                <AccountForm on_saved=on_created.clone() />
            </Modal>
            <Modal title="Ingresar datos" open=pasting>
                {move || {
                    paste_target
                        .get()
                        .map(|(id, username)| {
                            view! {
                                <PasteForm account_id=id username=username on_saved=on_pasted />
                            }
                        })
                }}
            </Modal>
        </section>
    }
}

/// Resumen de todas las cuentas: cuántas hay y cuánto se movieron sus
/// seguidores en la última captura.
#[component]
fn Overview(accounts: Vec<AccountSummary>) -> impl IntoView {
    let followers = || {
        accounts
            .iter()
            .filter_map(|summary| summary.followers.as_ref())
    };
    let added: usize = followers().filter_map(|list| list.added).sum();
    let removed: usize = followers().filter_map(|list| list.removed).sum();
    let last_update = accounts
        .iter()
        .filter_map(AccountSummary::last_update)
        .max()
        .map(str::to_owned);
    let pending = accounts
        .iter()
        .filter(|summary| summary.last_update().is_none())
        .count();

    view! {
        <div class="stats">
            <article class="stat">
                <span class="stat__label">"Cuentas"</span>
                <span class="stat__value">{format::count(accounts.len() as i64)}</span>
                {(pending > 0)
                    .then(|| {
                        view! {
                            <span class="stat__hint">
                                {format!("{pending} sin datos todavía")}
                            </span>
                        }
                    })}
            </article>
            <article class="stat">
                <span class="stat__label">"Nuevos seguidores"</span>
                <span class="stat__value" data-tone="up">
                    {format::count(added as i64)}
                </span>
                <span class="stat__hint">"En la última captura de cada cuenta"</span>
            </article>
            <article class="stat">
                <span class="stat__label">"Dejaron de seguir"</span>
                <span class="stat__value" data-tone="down">
                    {format::count(removed as i64)}
                </span>
                <span class="stat__hint">"En la última captura de cada cuenta"</span>
            </article>
            <article class="stat">
                <span class="stat__label">"Última actualización"</span>
                {match last_update {
                    Some(date) => {
                        view! {
                            <span class="stat__value stat__value--small">
                                {format::datetime(&date)}
                            </span>
                            <span class="stat__hint">{format::since(&date)}</span>
                        }
                            .into_any()
                    }
                    None => {
                        view! { <span class="stat__value stat__value--small">"—"</span> }
                            .into_any()
                    }
                }}
            </article>
        </div>
    }
}
