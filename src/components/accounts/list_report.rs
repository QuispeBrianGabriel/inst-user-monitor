use std::collections::HashSet;

use leptos::{prelude::*, task::spawn_local};

use crate::components::accounts::entries::{EntriesTable, EntryList, ProfileLink};
use crate::components::ui::{alert::Alert, delta::Delta};
use crate::models::account::{ListKind, ListReport, Rename, Snapshot};
use crate::services::{accounts_service, dialog};
use crate::state::auth::use_auth;
use crate::utils::format;

/// Estado de una lista (seguidores o seguidos) de una cuenta: cifras, qué
/// cambió desde la captura anterior, la lista completa y el historial.
#[component]
pub fn ListReportView(
    kind: ListKind,
    report: ListReport,
    #[prop(into)] account_id: String,
    /// Tras borrar una captura, para recargar.
    #[prop(into)]
    on_changed: Callback<()>,
) -> impl IntoView {
    let ListReport {
        latest,
        previous,
        entries,
        changes,
        history,
    } = report;

    let delta = changes.as_ref().map(|changes| changes.total_delta);
    let count_of = |pick: fn(&crate::models::account::Changes) -> usize| {
        changes.as_ref().map_or_else(
            || "—".to_owned(),
            |changes| format::count(pick(changes) as i64),
        )
    };
    let added_count = count_of(|changes| changes.added.len());
    let removed_count = count_of(|changes| changes.removed.len());
    let renamed_count = count_of(|changes| changes.renamed.len());
    let highlighted: HashSet<String> = changes
        .as_ref()
        .map(|changes| {
            changes
                .added
                .iter()
                .map(|entry| entry.username.clone())
                .collect()
        })
        .unwrap_or_default();

    let comparison = match &previous {
        Some(previous) => format!(
            "Comparado con la captura del {}",
            format::datetime(&previous.captured_at)
        ),
        None => "Primera captura: en la próxima verás qué cambió.".to_owned(),
    };

    view! {
        <div class="report">
            <div class="stats">
                <article class="stat">
                    <span class="stat__label">{kind.title()}</span>
                    <span class="stat__value">
                        {format::count(latest.total)} <Delta value=delta />
                    </span>
                </article>
                <article class="stat">
                    <span class="stat__label">{kind.added_title()}</span>
                    <span class="stat__value" data-tone="up">
                        {added_count}
                    </span>
                </article>
                <article class="stat">
                    <span class="stat__label">{kind.removed_title()}</span>
                    <span class="stat__value" data-tone="down">
                        {removed_count}
                    </span>
                </article>
                <article class="stat">
                    <span class="stat__label">"Cambios de nombre"</span>
                    <span class="stat__value">{renamed_count}</span>
                </article>
                <article class="stat stat--wide">
                    <span class="stat__label">"Última actualización"</span>
                    <span class="stat__value stat__value--small">
                        {format::datetime(&latest.captured_at)}
                    </span>
                    <span class="stat__hint">
                        {format::since(&latest.captured_at)} " · " {comparison}
                    </span>
                </article>
            </div>

            {changes
                .map(|changes| {
                    view! {
                        <div class="changes">
                            <EntryList title=kind.added_title() entries=changes.added tone="up" />
                            <EntryList
                                title=kind.removed_title()
                                entries=changes.removed
                                tone="down"
                            />
                            <RenamedList renamed=changes.renamed />
                        </div>
                    }
                })}

            <EntriesTable entries=entries highlighted=highlighted />
            <History history=history account_id=account_id on_changed=on_changed />
        </div>
    }
}

#[component]
fn RenamedList(renamed: Vec<Rename>) -> impl IntoView {
    let total = renamed.len();
    let name = |name: Option<String>| name.unwrap_or_else(|| "(sin nombre)".to_owned());
    view! {
        <section class="card entry-list">
            <header class="card__header">
                <h3>"Cambiaron de nombre"</h3>
                <span class="pill">{format::count(total as i64)}</span>
            </header>
            {if renamed.is_empty() {
                view! { <p class="muted entry-list__empty">"Ninguna"</p> }.into_any()
            } else {
                view! {
                    <ul class="entry-list__items">
                        {renamed
                            .into_iter()
                            .map(|rename| {
                                view! {
                                    <li>
                                        <ProfileLink username=rename.username />
                                        <span class="muted">
                                            {name(rename.before)} " → " {name(rename.after)}
                                        </span>
                                    </li>
                                }
                            })
                            .collect_view()}
                    </ul>
                }
                    .into_any()
            }}
        </section>
    }
}

/// Capturas de la lista, de la más reciente a la más antigua. Se puede
/// borrar una (p. ej. si se pegó la lista equivocada).
#[component]
fn History(history: Vec<Snapshot>, account_id: String, on_changed: Callback<()>) -> impl IntoView {
    let auth = use_auth();
    let error = RwSignal::new(None::<String>);
    let account_id = StoredValue::new(account_id);

    let remove = move |snapshot: Snapshot| {
        let message = format!(
            "¿Borrar la captura del {}? No se puede deshacer.",
            format::datetime(&snapshot.captured_at)
        );
        if !dialog::confirm(&message) {
            return;
        }
        let token = auth.token();
        let account_id = account_id.get_value();
        error.set(None);
        spawn_local(async move {
            match accounts_service::delete_snapshot(&token, &account_id, &snapshot.id).await {
                Ok(()) => on_changed.run(()),
                Err(err) => {
                    auth.handle_error(&err);
                    error.set(Some(err.to_string()));
                }
            }
        });
    };

    // Variación de cada captura respecto de la anterior (la siguiente en la lista).
    let deltas: Vec<Option<i64>> = (0..history.len())
        .map(|index| {
            history
                .get(index + 1)
                .map(|older| history[index].total - older.total)
        })
        .collect();

    view! {
        <section class="card">
            <header class="card__header">
                <h3>"Historial de capturas"</h3>
                <span class="pill">{history.len()}</span>
            </header>
            <Alert message=error />
            <div class="table-wrap">
                <table class="table">
                    <thead>
                        <tr>
                            <th>"Fecha"</th>
                            <th class="table__num">"Cuentas"</th>
                            <th class="table__num">"Cambio"</th>
                            <th></th>
                        </tr>
                    </thead>
                    <tbody>
                        {history
                            .into_iter()
                            .zip(deltas)
                            .map(|(snapshot, delta)| {
                                let total = snapshot.total;
                                let captured_at = snapshot.captured_at.clone();
                                view! {
                                    <tr>
                                        <td>
                                            {format::datetime(&captured_at)} " "
                                            <span class="muted">{format::since(&captured_at)}</span>
                                        </td>
                                        <td class="table__num">{format::count(total)}</td>
                                        <td class="table__num">
                                            <Delta value=delta />
                                        </td>
                                        <td class="table__actions">
                                            <button
                                                type="button"
                                                class="btn btn--ghost btn--sm btn--danger"
                                                on:click=move |_| remove(snapshot.clone())
                                            >
                                                "Borrar"
                                            </button>
                                        </td>
                                    </tr>
                                }
                            })
                            .collect_view()}
                    </tbody>
                </table>
            </div>
        </section>
    }
}
