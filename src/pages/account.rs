use leptos::{prelude::*, task::spawn_local};
use leptos_router::{
    components::A,
    hooks::{use_navigate, use_params_map},
};

use crate::components::accounts::{
    account_form::AccountForm, entries::ProfileLink, list_report::ListReportView,
    paste_form::PasteForm,
};
use crate::components::ui::{alert::Alert, modal::Modal};
use crate::config::routes;
use crate::models::{account::AccountDetail, api::ApiError};
use crate::services::{accounts_service, dialog};
use crate::state::auth::use_auth;
use crate::utils::format;

/// Detalle de una cuenta monitoreada: sus datos, el ingreso manual de su
/// lista de seguidores y qué cambió desde la captura anterior.
#[component]
pub fn AccountPage() -> impl IntoView {
    let auth = use_auth();
    let params = use_params_map();
    let id = Memo::new(move |_| params.read().get("id").unwrap_or_default());

    let reload = RwSignal::new(0u32);
    let detail = LocalResource::new(move || {
        reload.track();
        let id = id.get();
        let token = auth.token();
        async move { accounts_service::get(&token, &id).await }
    });
    Effect::new(move || {
        if let Some(Err(err)) = detail.get() {
            auth.handle_error(&err);
        }
    });
    let refresh = Callback::new(move |()| reload.update(|n| *n += 1));

    view! {
        <section class="page">
            <A href=routes::DASHBOARD attr:class="back-link">
                "← Cuentas"
            </A>
            <Suspense fallback=|| {
                view! { <p class="muted">"Cargando cuenta…"</p> }
            }>
                {move || {
                    detail
                        .get()
                        .map(|result| match result {
                            Ok(detail) => {
                                view! { <AccountView detail=detail on_changed=refresh /> }
                                    .into_any()
                            }
                            Err(ApiError::NotFound) => {
                                view! {
                                    <div class="empty card">
                                        <h2>"La cuenta no existe"</h2>
                                        <p class="muted">
                                            "Puede que se haya eliminado. Vuelve al panel para ver tus cuentas."
                                        </p>
                                    </div>
                                }
                                    .into_any()
                            }
                            Err(err) => view! { <p class="alert">{err.to_string()}</p> }.into_any(),
                        })
                }}
            </Suspense>
        </section>
    }
}

#[component]
fn AccountView(detail: AccountDetail, on_changed: Callback<()>) -> impl IntoView {
    let auth = use_auth();
    let navigate = use_navigate();
    let detail = StoredValue::new(detail);
    let account = detail.with_value(|detail| detail.account.clone());
    let account_id = account.id.clone();
    let username = account.username.clone();

    let pasting = RwSignal::new(false);
    let editing = RwSignal::new(false);
    let error = RwSignal::new(None::<String>);

    let on_pasted = move |_| {
        pasting.set(false);
        on_changed.run(());
    };
    let on_edited = move |_| {
        editing.set(false);
        on_changed.run(());
    };

    let remove = {
        let account_id = account_id.clone();
        let username = username.clone();
        move |_| {
            let message = format!(
                "¿Dejar de monitorear @{username}? Se borrarán sus datos y capturas. No se puede deshacer."
            );
            if !dialog::confirm(&message) {
                return;
            }
            let token = auth.token();
            let account_id = account_id.clone();
            let navigate = navigate.clone();
            error.set(None);
            spawn_local(async move {
                match accounts_service::delete(&token, &account_id).await {
                    Ok(()) => navigate(routes::DASHBOARD, Default::default()),
                    Err(err) => {
                        auth.handle_error(&err);
                        error.set(Some(err.to_string()));
                    }
                }
            });
        }
    };

    view! {
        <header class="page__header">
            <div class="account-head">
                <h1>"@" {account.username.clone()}</h1>
                <p class="account-head__name">
                    {account.full_name.clone().unwrap_or_else(|| "Sin nombre".to_owned())} " · "
                    <ProfileLink username=account.username.clone() />
                </p>
                {account
                    .notes
                    .clone()
                    .map(|notes| view! { <p class="account-head__notes">{notes}</p> })}
                <p class="muted account-head__meta">
                    "Monitoreada desde el " {format::datetime(&account.created_at)}
                </p>
            </div>
            <div class="page__actions">
                <button type="button" class="btn btn--accent" on:click=move |_| pasting.set(true)>
                    "Ingresar datos"
                </button>
                <button type="button" class="btn btn--ghost" on:click=move |_| editing.set(true)>
                    "Editar"
                </button>
                <button type="button" class="btn btn--ghost btn--danger" on:click=remove>
                    "Eliminar"
                </button>
            </div>
        </header>
        <Alert message=error />

        {
            let account_id = account_id.clone();
            match detail.with_value(|detail| detail.followers.clone()) {
                Some(report) => {
                    view! {
                        <ListReportView report=report account_id=account_id on_changed=on_changed />
                    }
                        .into_any()
                }
                None => {
                    view! {
                        <div class="empty card">
                            <h2>"Todavía no hay datos de seguidores"</h2>
                            <p class="muted">
                                "Copia la lista de seguidores desde Instagram y pégala con «Ingresar datos»."
                            </p>
                            <button
                                type="button"
                                class="btn btn--accent"
                                on:click=move |_| pasting.set(true)
                            >
                                "Ingresar datos"
                            </button>
                        </div>
                    }
                        .into_any()
                }
            }
        }

        <Modal title="Ingresar datos" open=pasting>
            <PasteForm account_id=account_id.clone() username=username.clone() on_saved=on_pasted />
        </Modal>
        <Modal title="Editar cuenta" open=editing>
            <AccountForm account=account.clone() on_saved=on_edited />
        </Modal>
    }
}
