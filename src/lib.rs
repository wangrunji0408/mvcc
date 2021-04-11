use std::{
    collections::VecDeque,
    sync::{atomic::*, Mutex},
};

#[cfg(test)]
mod tests;
mod tx;

use crate::tx::*;

pub struct Database {
    data: Vec<Mutex<VecDeque<Record>>>,
    txid: AtomicU64,
}

struct Record {
    version: u64,
    data: usize,
}

impl Database {
    fn new() -> Self {
        todo!()
    }
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotFound,
    Abort,
}
