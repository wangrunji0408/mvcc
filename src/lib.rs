#[macro_use]
extern crate log;

mod db;
#[cfg(test)]
mod tests;
mod tx;
mod tx_manager;

pub use crate::db::Database;
pub use crate::tx::{Transaction, TxId, TxStatus};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotFound,
    Abort,
    TxNotActive,
}
