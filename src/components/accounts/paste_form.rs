use leptos::{ev::SubmitEvent, prelude::*, task::spawn_local};

use crate::components::ui::{alert::Alert, button::SubmitButton};
use crate::models::account::{Snapshot, SnapshotRequest};
use crate::services::accounts_service;
use crate::state::auth::use_auth;
use crate::utils::format;

/// Ingreso manual de datos: el usuario pega la lista de seguidores copiada
/// de Instagram. Si el formato no es válido, la API no guarda nada y aquí se
/// muestra la línea que falla.
#[component]
pub fn PasteForm(
    #[prop(into)] account_id: String,
    #[prop(into)] username: String,
    #[prop(into)] on_saved: Callback<Snapshot>,
) -> impl IntoView {
    let auth = use_auth();
    let text = RwSignal::new(String::new());
    let pending = RwSignal::new(false);
    let error = RwSignal::new(None::<String>);

    let lines = Memo::new(move |_| {
        text.with(|text| text.lines().filter(|line| !line.trim().is_empty()).count())
    });

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        if pending.get_untracked() {
            return;
        }
        if lines.get_untracked() == 0 {
            error.set(Some("Pega la lista copiada de Instagram.".to_owned()));
            return;
        }
        let request = SnapshotRequest {
            text: text.get_untracked(),
        };
        let token = auth.token();
        let account_id = account_id.clone();
        pending.set(true);
        error.set(None);
        spawn_local(async move {
            let result = accounts_service::add_snapshot(&token, &account_id, &request).await;
            pending.set(false);
            match result {
                Ok(snapshot) => {
                    text.set(String::new());
                    on_saved.run(snapshot);
                }
                Err(err) => {
                    auth.handle_error(&err);
                    error.set(Some(err.to_string()));
                }
            }
        });
    };

    view! {
        <form class="form" on:submit=on_submit novalidate>
            <ol class="steps">
                <li>
                    "En Instagram abre la lista de " <strong>"seguidores"</strong> " de @"
                    {username} " y desplázate hasta el final para que cargue entera."
                </li>
                <li>"Selecciona todo (Ctrl+A), cópialo (Ctrl+C) y pégalo aquí (Ctrl+V)."</li>
            </ol>

            <div class="field">
                <label class="field__label" for="paste-text">
                    "Texto copiado"
                </label>
                <textarea
                    class="field__input field__textarea paste__text"
                    id="paste-text"
                    name="paste-text"
                    rows="12"
                    spellcheck="false"
                    placeholder="usuario\nNombre visible\notro.usuario\nOtro nombre\n…"
                    prop:value=move || text.get()
                    on:input=move |ev| text.set(event_target_value(&ev))
                ></textarea>
                <p class="field__hint">
                    {move || match lines.get() {
                        0 => "Formato: una línea con el usuario y, debajo, su nombre.".to_owned(),
                        1 => "1 línea".to_owned(),
                        n => format!("{} líneas", format::count(n as i64)),
                    }}
                </p>
            </div>

            <Alert message=error />
            <SubmitButton label="Cargar datos" pending_label="Procesando…" pending=pending />
        </form>
    }
}
