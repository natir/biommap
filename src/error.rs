//! Error struct of project biommap

/* crate use */
use thiserror;

/// Enum to manage error
#[derive(std::fmt::Debug, thiserror::Error)]
pub enum Error {
    }

/// Alias of result
pub type Result<T> = core::result::Result<T, Error>;
