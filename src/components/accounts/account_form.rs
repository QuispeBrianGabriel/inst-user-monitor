use leptos::{ev::SubmitEvent, prelude::*, task::spawn_local};

use crate::components::ui::{alert::Alert, button::SubmitButton, text_field::TextField};
use crate::models::account::{Account, AccountRequest};
use crate::services::accounts_service;
use crate::state::auth::use_auth;
use crate::utils::validation::{
    normalize_username, validate_full_name, validate_notes, validate_username,
};

/// Alta o edición de una cuenta a monitorear: el usuario de Instagram (no se
/// comprueba que exista) y datos libres sobre ella.
///
/// Con `account` edita esa cuenta; sin él, crea una nueva.
#[component]
pub fn AccountForm(
    #[prop(optional)] account: Option<Account>,
    #[prop(into)] on_saved: Callback<Account>,
) -> impl IntoView {
    let auth = use_auth();
    let editing = account.as_ref().map(|account| account.id.clone());
    let is_edit = editing.is_some();

    let initial = account.unwrap_or_else(|| Account {
        id: String::new(),
        username: String::new(),
        full_name: None,
        notes: None,
        created_at: String::new(),
        updated_at: String::new(),
    });
    let username = RwSignal::new(initial.username);
    let full_name = RwSignal::new(initial.full_name.unwrap_or_default());
    let notes = RwSignal::new(initial.notes.unwrap_or_default());
    let submitted = RwSignal::new(false);
    let pending = RwSignal::new(false);
    let server_error = RwSignal::new(None::<String>);

    let username_error = Signal::derive(move || {
        submitted
            .get()
            .then(|| username.with(|v| validate_username(v).err()))
            .flatten()
    });
    let full_name_error = Signal::derive(move || full_name.with(|v| validate_full_name(v).err()));
    let notes_error = Signal::derive(move || notes.with(|v| validate_notes(v).err()));

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        if pending.get_untracked() {
            return;
        }
        submitted.set(true);
        if username_error.get_untracked().is_some()
            || full_name_error.get_untracked().is_some()
            || notes_error.get_untracked().is_some()
        {
            return;
        }

        let optional = |text: String| {
            let text = text.trim();
            (!text.is_empty()).then(|| text.to_owned())
        };
        let request = AccountRequest {
            username: normalize_username(&username.get_untracked()),
            full_name: optional(full_name.get_untracked()),
            notes: optional(notes.get_untracked()),
        };
        let token = auth.token();
        let editing = editing.clone();
        pending.set(true);
        server_error.set(None);
        spawn_local(async move {
            let result = match editing {
                Some(id) => accounts_service::update(&token, &id, &request).await,
                None => accounts_service::create(&token, &request).await,
            };
            pending.set(false);
            match result {
                Ok(account) => on_saved.run(account),
                Err(err) => {
                    auth.handle_error(&err);
                    server_error.set(Some(err.to_string()));
                }
            }
        });
    };

    view! {
        <form class="form" on:submit=on_submit novalidate>
            <Alert message=server_error />
            <TextField
                id="account-username"
                label="Usuario de Instagram"
                autocomplete="off"
                placeholder="@usuario"
                value=username
                error=username_error
            />
            <TextField
                id="account-full-name"
                label="Nombre (opcional)"
                autocomplete="off"
                placeholder="Nombre visible de la cuenta"
                value=full_name
                error=full_name_error
            />
            <div class="field" class:has-error=move || notes_error.get().is_some()>
                <label class="field__label" for="account-notes">
                    "Notas (opcional)"
                </label>
                <textarea
                    class="field__input field__textarea"
                    id="account-notes"
                    name="account-notes"
                    rows="3"
                    placeholder="Por qué la monitoreas, de quién es, categoría…"
                    prop:value=move || notes.get()
                    on:input=move |ev| notes.set(event_target_value(&ev))
                ></textarea>
                <p class="field__error">{move || notes_error.get().unwrap_or_default()}</p>
            </div>
            <SubmitButton
                label=if is_edit { "Guardar cambios" } else { "Agregar cuenta" }
                pending_label="Guardando…"
                pending=pending
            />
        </form>
    }
}
