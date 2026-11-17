use leptos::prelude::*;

/// Mensaje de error a nivel de formulario. No ocupa espacio si no hay mensaje.
#[component]
pub fn Alert(#[prop(into)] message: Signal<Option<String>>) -> impl IntoView {
    view! {
        <div class="alert-slot" role="alert" aria-live="assertive">
            {move || {
                message
                    .get()
                    .map(|text| {
                        view! {
                            <p class="alert">
                                <span class="alert__icon" aria-hidden="true">
                                    "!"
                                </span>
                                {text}
                            </p>
                        }
                    })
            }}
        </div>
    }
}
