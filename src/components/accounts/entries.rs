//! Listas de cuentas de Instagram: compacta (cambios, relaciones) y tabla
//! completa con búsqueda.

use std::collections::HashSet;

use leptos::prelude::*;

use crate::models::account::Entry;
use crate::utils::format;

/// Filas que se pintan como máximo: una lista de miles de cuentas haría la
/// página lenta. Para el resto, la búsqueda.
const MAX_ROWS: usize = 500;

/// Enlace al perfil público en Instagram.
pub fn profile_url(username: &str) -> String {
    format!("https://www.instagram.com/{username}/")
}

/// `@usuario` enlazado al perfil, en una pestaña nueva.
#[component]
pub fn ProfileLink(#[prop(into)] username: String) -> impl IntoView {
    let href = profile_url(&username);
    view! {
        <a class="handle" href=href target="_blank" rel="noopener noreferrer">
            "@"
            {username}
        </a>
    }
}

/// Lista compacta de cuentas con un título y su cantidad.
#[component]
pub fn EntryList(
    #[prop(into)] title: String,
    entries: Vec<Entry>,
    /// `up`, `down` o `flat`: color del contador.
    #[prop(default = "flat")]
    tone: &'static str,
    #[prop(into, default = "Ninguna".to_owned())] empty: String,
) -> impl IntoView {
    let total = entries.len();
    view! {
        <section class="card entry-list">
            <header class="card__header">
                <h3>{title}</h3>
                <span class="pill" data-tone=tone>
                    {format::count(total as i64)}
                </span>
            </header>
            {if entries.is_empty() {
                view! { <p class="muted entry-list__empty">{empty}</p> }.into_any()
            } else {
                view! {
                    <ul class="entry-list__items">
                        {entries
                            .into_iter()
                            .map(|entry| {
                                view! {
                                    <li>
                                        <ProfileLink username=entry.username />
                                        <span class="muted">{entry.full_name}</span>
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

/// Todas las cuentas de una captura, con búsqueda por usuario o nombre. Las
/// de `highlighted` (nuevas desde la captura anterior) se marcan.
#[component]
pub fn EntriesTable(entries: Vec<Entry>, highlighted: HashSet<String>) -> impl IntoView {
    let query = RwSignal::new(String::new());
    let entries = StoredValue::new(entries);
    let highlighted = StoredValue::new(highlighted);

    let filtered = Memo::new(move |_| {
        let query = query.get().trim().to_lowercase();
        entries.with_value(|entries| {
            entries
                .iter()
                .enumerate()
                .filter(|(_, entry)| matches(entry, &query))
                .map(|(index, entry)| (index + 1, entry.clone()))
                .collect::<Vec<_>>()
        })
    });

    view! {
        <section class="card">
            <header class="card__header">
                <h3>"Lista completa"</h3>
                <input
                    class="field__input search"
                    type="search"
                    placeholder="Buscar usuario o nombre"
                    aria-label="Buscar en la lista"
                    prop:value=move || query.get()
                    on:input=move |ev| query.set(event_target_value(&ev))
                />
            </header>
            <div class="table-wrap">
                <table class="table">
                    <thead>
                        <tr>
                            <th class="table__num">"#"</th>
                            <th>"Usuario"</th>
                            <th>"Nombre"</th>
                        </tr>
                    </thead>
                    <tbody>
                        {move || {
                            filtered
                                .get()
                                .into_iter()
                                .take(MAX_ROWS)
                                .map(|(position, entry)| {
                                    let is_new = highlighted
                                        .with_value(|set| set.contains(&entry.username));
                                    view! {
                                        <tr class:is-new=is_new>
                                            <td class="table__num muted">{position}</td>
                                            <td>
                                                <ProfileLink username=entry.username />
                                                {is_new
                                                    .then(|| {
                                                        view! {
                                                            <span class="pill" data-tone="up">
                                                                "Nuevo"
                                                            </span>
                                                        }
                                                    })}
                                            </td>
                                            <td>{entry.full_name.unwrap_or_default()}</td>
                                        </tr>
                                    }
                                })
                                .collect_view()
                        }}
                    </tbody>
                </table>
            </div>
            <p class="muted table__footer">
                {move || {
                    let shown = filtered.with(Vec::len);
                    let total = entries.with_value(Vec::len);
                    if shown == 0 {
                        "Ninguna cuenta coincide con la búsqueda.".to_owned()
                    } else if shown > MAX_ROWS {
                        format!(
                            "Mostrando {} de {}: usa la búsqueda para ver el resto.",
                            format::count(MAX_ROWS as i64),
                            format::count(shown as i64),
                        )
                    } else if shown < total {
                        format!(
                            "{} de {} cuentas",
                            format::count(shown as i64),
                            format::count(total as i64),
                        )
                    } else {
                        format!("{} cuentas", format::count(total as i64))
                    }
                }}
            </p>
        </section>
    }
}

/// `true` si la cuenta coincide con la búsqueda (ya en minúsculas).
fn matches(entry: &Entry, query: &str) -> bool {
    query.is_empty()
        || entry.username.contains(query)
        || entry
            .full_name
            .as_deref()
            .is_some_and(|name| name.to_lowercase().contains(query))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_matches_username_or_name() {
        let entry = Entry {
            username: "rocioteran__".to_owned(),
            full_name: Some("Rocío Terán".to_owned()),
        };
        assert!(matches(&entry, ""));
        assert!(matches(&entry, "teran__"));
        assert!(matches(&entry, "rocío"));
        assert!(!matches(&entry, "duki"));
        let nameless = Entry {
            username: "angelmeeza".to_owned(),
            full_name: None,
        };
        assert!(!matches(&nameless, "ángel"));
    }

    #[test]
    fn profile_links_point_to_instagram() {
        assert_eq!(profile_url("duki"), "https://www.instagram.com/duki/");
    }
}
