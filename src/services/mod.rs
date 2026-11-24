//! Acceso a datos externos: API y almacenamiento del navegador.
//!
//! La UI nunca habla directamente con `fetch` ni con `localStorage`: pasa por
//! aquí, de modo que cambiar de backend no afecta a los componentes.

pub mod accounts_service;
pub mod auth_service;
pub mod dialog;
pub mod storage;
