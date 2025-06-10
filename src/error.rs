//! Общая иерархия ошибок.

use thiserror::Error;

/// Результат, который возвращают все публичные функции библиотеки.
pub type Result<T, E = Error> = core::result::Result<T, E>;

/// Возможные ошибки клиента.
#[derive(Debug, Error)]
pub enum Error {
    /// Ошибка HTTP-клиента.
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),

    /// Ошибка при сериализации/десериализации JSON.
    #[error("serde json error: {0}")]
    Json(#[from] serde_json::Error),

    /// Низкоуровневая ошибка Gate.io (код + сообщение).
    #[error("gate.io error {code}: {msg}")]
    Gate { code: i64, msg: String },

    /// Прочие ошибки.
    #[error("{0}")]
    Other(String),
}