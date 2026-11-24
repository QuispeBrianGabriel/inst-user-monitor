use leptos::prelude::*;

use crate::components::accounts::entries::EntryList;
use crate::models::account::Relations;
use crate::utils::format;

/// Cruce de las últimas listas de seguidores y seguidos de la cuenta.
#[component]
pub fn RelationsView(relations: Relations) -> impl IntoView {
    view! {
        <div class="report">
            <div class="stats">
                <article class="stat">
                    <span class="stat__label">"Se siguen mutuamente"</span>
                    <span class="stat__value">{format::count(relations.mutual as i64)}</span>
                </article>
                <article class="stat">
                    <span class="stat__label">"No le siguen de vuelta"</span>
                    <span class="stat__value" data-tone="down">
                        {format::count(relations.not_following_back.len() as i64)}
                    </span>
                </article>
                <article class="stat">
                    <span class="stat__label">"No sigue de vuelta"</span>
                    <span class="stat__value">
                        {format::count(relations.not_followed_back.len() as i64)}
                    </span>
                </article>
            </div>
            <p class="muted">
                "Según las últimas capturas de cada lista. Si son de fechas distintas, "
                "actualiza las dos para que el cruce sea exacto."
            </p>
            <div class="changes changes--two">
                <EntryList
                    title="Sigue y no le siguen"
                    entries=relations.not_following_back
                    tone="down"
                />
                <EntryList title="Le siguen y no sigue" entries=relations.not_followed_back />
            </div>
        </div>
    }
}
