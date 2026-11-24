use leptos::prelude::*;

/// Ventana modal sobre la página. Se cierra con el botón, haciendo clic
/// fuera o con Escape. El contenido solo existe mientras está abierta.
#[component]
pub fn Modal(
    #[prop(into)] title: String,
    open: RwSignal<bool>,
    children: ChildrenFn,
) -> impl IntoView {
    let title = StoredValue::new(title);
    let close = move || open.set(false);

    view! {
        <Show when=move || open.get()>
            <div
                class="modal"
                on:keydown=move |ev| {
                    if ev.key() == "Escape" {
                        close();
                    }
                }
            >
                <div class="modal__backdrop" on:click=move |_| close()></div>
                <section
                    class="modal__panel"
                    role="dialog"
                    aria-modal="true"
                    aria-label=title.get_value()
                >
                    <header class="modal__header">
                        <h2>{title.get_value()}</h2>
                        <button
                            type="button"
                            class="modal__close"
                            aria-label="Cerrar"
                            on:click=move |_| close()
                        >
                            "×"
                        </button>
                    </header>
                    {children()}
                </section>
            </div>
        </Show>
    }
}
