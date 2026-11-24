//! Cuentas de Instagram monitoreadas y sus capturas, tal como las devuelve
//! la API (`/accounts`).

use serde::{Deserialize, Serialize};

/// Lista de una cuenta que se puede capturar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ListKind {
    Followers,
    Following,
}

impl ListKind {
    pub const ALL: [Self; 2] = [Self::Followers, Self::Following];

    pub fn title(self) -> &'static str {
        match self {
            Self::Followers => "Seguidores",
            Self::Following => "Seguidos",
        }
    }

    /// Título de las cuentas que aparecieron desde la captura anterior.
    pub fn added_title(self) -> &'static str {
        match self {
            Self::Followers => "Nuevos seguidores",
            Self::Following => "Empezó a seguir",
        }
    }

    /// Título de las cuentas que ya no están.
    pub fn removed_title(self) -> &'static str {
        match self {
            Self::Followers => "Dejaron de seguirle",
            Self::Following => "Dejó de seguir",
        }
    }
}

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

/// Cuerpo de `POST /accounts/{id}/snapshots`.
#[derive(Debug, Clone, Serialize)]
pub struct SnapshotRequest {
    pub kind: ListKind,
    pub text: String,
}

/// Una captura, sin su lista.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Snapshot {
    pub id: String,
    pub kind: ListKind,
    pub total: i64,
    pub captured_at: String,
}

/// Una cuenta de una lista.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Entry {
    pub username: String,
    pub full_name: Option<String>,
}

/// Estado de una lista en el listado de cuentas.
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
    pub followers: Option<ListSummary>,
    pub following: Option<ListSummary>,
}

impl AccountSummary {
    pub fn list(&self, kind: ListKind) -> Option<&ListSummary> {
        match kind {
            ListKind::Followers => self.followers.as_ref(),
            ListKind::Following => self.following.as_ref(),
        }
    }

    /// Fecha de la captura más reciente de cualquiera de sus listas (ISO 8601).
    pub fn last_update(&self) -> Option<&str> {
        ListKind::ALL
            .into_iter()
            .filter_map(|kind| self.list(kind))
            .map(|list| list.latest.captured_at.as_str())
            // RFC 3339 en UTC: el orden de texto es el de fecha (salvo
            // fracciones dentro del mismo segundo, irrelevante aquí).
            .max()
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

/// Una lista en el detalle de la cuenta.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct ListReport {
    pub latest: Snapshot,
    pub previous: Option<Snapshot>,
    pub entries: Vec<Entry>,
    pub changes: Option<Changes>,
    /// De la más reciente a la más antigua.
    pub history: Vec<Snapshot>,
}

/// Cruce de seguidores y seguidos.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Relations {
    pub mutual: usize,
    /// Sigue a estas cuentas y no le siguen.
    pub not_following_back: Vec<Entry>,
    /// Le siguen y no las sigue.
    pub not_followed_back: Vec<Entry>,
}

/// `GET /accounts/{id}`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct AccountDetail {
    pub account: Account,
    pub followers: Option<ListReport>,
    pub following: Option<ListReport>,
    pub relations: Option<Relations>,
}

impl AccountDetail {
    pub fn list(&self, kind: ListKind) -> Option<&ListReport> {
        match kind {
            ListKind::Followers => self.followers.as_ref(),
            ListKind::Following => self.following.as_ref(),
        }
    }
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
                "latest": { "id": "1", "kind": "followers", "total": 222, "captured_at": "2026-11-24T12:00:00Z" },
                "total_delta": -3, "added": 1, "removed": 4
            },
            "following": {
                "latest": { "id": "2", "kind": "following", "total": 10, "captured_at": "2026-11-25T08:00:00Z" },
                "total_delta": null, "added": null, "removed": null
            }
        }"#;
        let summary: AccountSummary = serde_json::from_str(json).unwrap();
        assert_eq!(summary.followers.as_ref().unwrap().total_delta, Some(-3));
        assert_eq!(summary.list(ListKind::Following).unwrap().added, None);
        assert_eq!(summary.last_update(), Some("2026-11-25T08:00:00Z"));
    }

    #[test]
    fn summary_without_snapshots_has_no_last_update() {
        let json = r#"{
            "account": {
                "id": "0193", "username": "duki", "full_name": null, "notes": null,
                "created_at": "2026-11-24T10:00:00Z", "updated_at": "2026-11-24T10:00:00Z"
            },
            "followers": null, "following": null
        }"#;
        let summary: AccountSummary = serde_json::from_str(json).unwrap();
        assert_eq!(summary.last_update(), None);
    }

    #[test]
    fn kind_serializes_like_the_api() {
        let request = SnapshotRequest {
            kind: ListKind::Following,
            text: String::new(),
        };
        assert_eq!(
            serde_json::to_string(&request).unwrap(),
            r#"{"kind":"following","text":""}"#
        );
    }
}
