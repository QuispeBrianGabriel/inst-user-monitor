use leptos::prelude::*;

/// Campo de formulario accesible con etiqueta, mensaje de error y, para
/// contraseñas, botón para mostrar u ocultar el texto.
#[component]
pub fn TextField(
    /// Identificador único; también se usa como `name`.
    #[prop(into)]
    id: String,
    #[prop(into)] label: String,
    value: RwSignal<String>,
    #[prop(into)] error: Signal<Option<&'static str>>,
    #[prop(default = "text")] input_type: &'static str,
    #[prop(optional)] autocomplete: Option<&'static str>,
    #[prop(optional, into)] placeholder: Option<String>,
    /// Contenido extra bajo el campo (p. ej. un medidor).
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    let is_password = input_type == "password";
    let revealed = RwSignal::new(false);
    let current_type = move || {
        if is_password && revealed.get() {
            "text"
        } else {
            input_type
        }
    };

    let error_id = format!("{id}-error");
    let described_by = {
        let error_id = error_id.clone();
        move || error.get().map(|_| error_id.clone())
    };

    view! {
        <div class="field" class:has-error=move || error.get().is_some()>
            <label class="field__label" for=id.clone()>
                {label}
            </label>
            <div class="field__control">
                <input
                    class="field__input"
                    id=id.clone()
                    name=id
                    type=current_type
                    autocomplete=autocomplete
                    placeholder=placeholder
                    spellcheck="false"
                    aria-invalid=move || error.get().is_some().to_string()
                    aria-describedby=described_by
                    bind:value=value
                />
                {is_password
                    .then(|| {
                        view! {
                            <button
                                type="button"
                                class="field__toggle"
                                aria-pressed=move || revealed.get().to_string()
                                aria-label=move || {
                                    if revealed.get() {
                                        "Ocultar contraseña"
                                    } else {
                                        "Mostrar contraseña"
                                    }
                                }
                                on:click=move |_| revealed.update(|r| *r = !*r)
                            >
                                {move || if revealed.get() { "Ocultar" } else { "Ver" }}
                            </button>
                        }
                    })}
            </div>
            {children.map(|children| children())}
            <p class="field__error" id=error_id>
                {move || error.get().unwrap_or_default()}
            </p>
        </div>
    }
}
