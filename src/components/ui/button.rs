use leptos::prelude::*;

/// Botón de envío de formulario con estado de carga.
#[component]
pub fn SubmitButton(
    #[prop(into)] label: String,
    #[prop(into)] pending_label: String,
    #[prop(into)] pending: Signal<bool>,
) -> impl IntoView {
    view! {
        <button
            type="submit"
            class="btn btn--primary"
            class:is-pending=pending
            disabled=pending
            aria-busy=move || pending.get().to_string()
        >
            <span class="btn__spinner" aria-hidden="true"></span>
            <span class="btn__label">
                {move || if pending.get() { pending_label.clone() } else { label.clone() }}
            </span>
        </button>
    }
}
