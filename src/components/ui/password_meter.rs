use leptos::prelude::*;

use crate::utils::validation::PasswordStrength;

/// Medidor visual de la fortaleza de una contraseña.
#[component]
pub fn PasswordMeter(#[prop(into)] password: Signal<String>) -> impl IntoView {
    let strength = Memo::new(move |_| password.with(|p| PasswordStrength::evaluate(p)));
    let level = move || strength.get().level();

    view! {
        <div class="meter" data-level=level aria-live="polite">
            <div class="meter__bars" aria-hidden="true">
                {(1..=4u8)
                    .map(|segment| {
                        view! {
                            <span
                                class="meter__bar"
                                class:is-on=move || { level() >= segment }
                            ></span>
                        }
                    })
                    .collect_view()}
            </div>
            <span class="meter__label">
                {move || {
                    let label = strength.get().label();
                    if label.is_empty() { String::new() } else { format!("Seguridad: {label}") }
                }}
            </span>
        </div>
    }
}
