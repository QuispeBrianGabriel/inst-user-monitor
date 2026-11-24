use leptos::prelude::*;
use leptos_router::components::A;

use crate::components::accounts::entries::ProfileLink;
use crate::components::ui::delta::Delta;
use crate::config::routes;
use crate::models::account::{AccountSummary, ListKind, ListSummary};
use crate::utils::format;

/// Tabla del panel: cada cuenta con el estado actual de sus listas y los
/// cambios desde la captura anterior.
#[component]
pub fn AccountsTable(
    accounts: Vec<AccountSummary>,
    /// Abre el ingreso de datos de la cuenta: `(id, usuario)`.
    #[prop(into)]
    on_paste: Callback<(String, String)>,
) -> impl IntoView {
    view! {
        <div class="table-wrap card">
            <table class="table table--accounts">
                <thead>
                    <tr>
                        <th>"Cuenta"</th>
                        <th>{ListKind::Followers.title()}</th>
                        <th>{ListKind::Following.title()}</th>
                        <th>"Última actualización"</th>
                        <th></th>
                    </tr>
                </thead>
                <tbody>
                    {accounts
                        .into_iter()
                        .map(|summary| {
                            let last_update = summary.last_update().map(str::to_owned);
                            let AccountSummary { account, followers, following } = summary;
                            let detail = routes::account(&account.id);
                            let paste_target = (account.id.clone(), account.username.clone());
                            let username = account.username.clone();
                            let full_name = account.full_name.unwrap_or_default();
                            view! {
                                <tr>
                                    <td>
                                        <A href=detail.clone() attr:class="account-cell">
                                            <strong>{username}</strong>
                                            <span class="muted">{full_name}</span>
                                        </A>
                                        <ProfileLink username=account.username />
                                    </td>
                                    <td>
                                        <ListCell list=followers />
                                    </td>
                                    <td>
                                        <ListCell list=following />
                                    </td>
                                    <td>
                                        {match last_update {
                                            Some(date) => {
                                                view! {
                                                    <span>{format::datetime(&date)}</span>
                                                    <span class="muted cell__sub">{format::since(&date)}</span>
                                                }
                                                    .into_any()
                                            }
                                            None => {
                                                view! { <span class="muted">"Nunca"</span> }.into_any()
                                            }
                                        }}
                                    </td>
                                    <td class="table__actions">
                                        <button
                                            type="button"
                                            class="btn btn--accent btn--sm"
                                            on:click=move |_| on_paste.run(paste_target.clone())
                                        >
                                            "Ingresar datos"
                                        </button>
                                        <A href=detail attr:class="btn btn--ghost btn--sm">
                                            "Ver"
                                        </A>
                                    </td>
                                </tr>
                            }
                        })
                        .collect_view()}
                </tbody>
            </table>
        </div>
    }
}

/// Cantidad de una lista, su variación y las altas y bajas.
#[component]
fn ListCell(list: Option<ListSummary>) -> impl IntoView {
    match list {
        None => view! { <span class="muted">"Sin datos"</span> }.into_any(),
        Some(list) => {
            let movement = list.added.zip(list.removed).map(|(added, removed)| {
                view! {
                    <span class="cell__sub">
                        <span data-tone="up">"+" {format::count(added as i64)}</span>
                        " / "
                        <span data-tone="down">"−" {format::count(removed as i64)}</span>
                    </span>
                }
            });
            view! {
                <span class="cell__value">
                    {format::count(list.latest.total)} <Delta value=list.total_delta />
                </span>
                {movement}
            }
            .into_any()
        }
    }
}
