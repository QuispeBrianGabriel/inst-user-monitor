//! Cuentas de Instagram monitoreadas y las capturas de sus seguidores, tal
//! como las devuelve la API (`/accounts`).

use serde::{Deserialize, Serialize};

/// Datos de la cuenta que indica el usuario.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Account {
    pub id: String,
    pub username: String,
    pub full_name: Option<String>,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// Cuerpo de `POST /accounts` y `PUT /accounts/{id}`.
#[derive(Debug, Clone, Serialize)]
pub struct AccountRequest {
    pub username: String,
    pub full_name: Option<String>,
    pub notes: Option<String>,
}

/// Cuerpo de `POST /accounts/{id}/snapshots`: la lista de seguidores pegada.
#[derive(Debug, Clone, Serialize)]
pub struct SnapshotRequest {
    pub text: String,
}

/// Una captura de seguidores, sin su lista.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Snapshot {
    pub id: String,
    pub total: i64,
    pub captured_at: String,
}

/// Una cuenta de una lista.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Entry {
    pub username: String,
    pub full_name: Option<String>,
}

/// Seguidores en el listado de cuentas.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct ListSummary {
    pub latest: Snapshot,
    /// `None` si es la primera captura.
    pub total_delta: Option<i64>,
    pub added: Option<usize>,
    pub removed: Option<usize>,
}

/// Elemento de `GET /accounts`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct AccountSummary {
    pub account: Account,
    /// `None` si aún no hay capturas.
    pub followers: Option<ListSummary>,
}

impl AccountSummary {
    /// Fecha de la última captura (ISO 8601).
    pub fn last_update(&self) -> Option<&str> {
        self.followers
            .as_ref()
            .map(|followers| followers.latest.captured_at.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Rename {
    pub username: String,
    pub before: Option<String>,
    pub after: Option<String>,
}

/// Cambios de una lista respecto de la captura anterior.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Changes {
    pub total_delta: i64,
    pub added: Vec<Entry>,
    pub removed: Vec<Entry>,
    pub renamed: Vec<Rename>,
}

/// Seguidores en el detalle de la cuenta.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct ListReport {
    pub latest: Snapshot,
    pub previous: Option<Snapshot>,
    pub entries: Vec<Entry>,
    pub changes: Option<Changes>,
    /// De la más reciente a la más antigua.
    pub history: Vec<Snapshot>,
}

/// `GET /accounts/{id}`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct AccountDetail {
    pub account: Account,
    /// `None` si aún no hay capturas.
    pub followers: Option<ListReport>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_api_summary() {
        let json = r#"{
            "account": {
                "id": "0193", "username": "duki", "full_name": null, "notes": "x",
                "created_at": "2026-11-24T10:00:00Z", "updated_at": "2026-11-24T10:00:00Z"
            },
            "followers": {
                "latest": { "id": "1", "total": 222, "captured_at": "2026-11-24T12:00:00Z" },
                "total_delta": -3, "added": 1, "removed": 4
            }
        }"#;
        let summary: AccountSummary = serde_json::from_str(json).unwrap();
        assert_eq!(summary.followers.as_ref().unwrap().total_delta, Some(-3));
        assert_eq!(summary.last_update(), Some("2026-11-24T12:00:00Z"));
    }

    #[test]
    fn summary_without_snapshots_has_no_last_update() {
        let json = r#"{
            "account": {
                "id": "0193", "username": "duki", "full_name": null, "notes": null,
                "created_at": "2026-11-24T10:00:00Z", "updated_at": "2026-11-24T10:00:00Z"
            },
            "followers": null
        }"#;
        let summary: AccountSummary = serde_json::from_str(json).unwrap();
        assert_eq!(summary.last_update(), None);
    }

    #[test]
    fn snapshot_request_is_just_the_text() {
        let request = SnapshotRequest {
            text: "duki\n5202".to_owned(),
        };
        assert_eq!(
            serde_json::to_string(&request).unwrap(),
            r#"{"text":"duki\n5202"}"#
        );
    }
}
