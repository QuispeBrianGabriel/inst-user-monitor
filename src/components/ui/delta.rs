use leptos::prelude::*;

use crate::utils::format;

/// Variación de una cantidad respecto de la captura anterior: verde si
/// sube, roja si baja. Sin valor (primera captura) no muestra nada.
#[component]
pub fn Delta(#[prop(into)] value: Option<i64>) -> impl IntoView {
    value.map(|value| {
        let tone = match value {
            0 => "flat",
            1.. => "up",
            _ => "down",
        };
        view! {
            <span class="delta" data-tone=tone title="Respecto de la captura anterior">
                {format::delta(value)}
            </span>
        }
    })
}
