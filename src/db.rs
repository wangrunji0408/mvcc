use crate::{tx::*, tx_manager::*, Error, Result};
use std::{collections::VecDeque, sync::Mutex};

pub struct Database {
    // fixed table for now
    data: Vec<Versions>,
    pub(crate) tx_manager: TxManager,
}

impl Database {
    pub fn new() -> Self {
        let mut data = Vec::new();
        data.resize_with(10, Default::default);
        Database {
            data,
            tx_manager: TxManager::default(),
        }
    }

    pub(crate) fn put(&self, tx: &Transaction, key: usize, value: usize) -> Result<()> {
        let is_committed = |txid| self.tx_manager.get_status(txid) == TxStatus::Committed;
        self.data[key].put(tx.id(), value, is_committed)
    }

    pub(crate) fn get(&self, tx: &Transaction, key: usize) -> Result<usize> {
        self.data[key].get(|txid| tx.snapshot().can_see(txid))
    }
}

#[derive(Default)]
struct Versions {
    /// Old-to-new version list.
    records: Mutex<VecDeque<Record>>,
}

#[derive(Debug)]
struct Record {
    tmin: TxId,
    tmax: TxId,
    data: usize,
}

impl Versions {
    fn put(&self, txid: TxId, data: usize, is_committed: impl Fn(TxId) -> bool) -> Result<()> {
        trace!("put: txid={:?}, data={:?}", txid, data);
        let mut records = self.records.lock().unwrap();
        if let Some(record) = records.back_mut() {
            if record.tmin >= txid {
                return Err(Error::Abort);
            } else if is_committed(record.tmin) {
                record.tmax = txid;
            } else {
                todo!("stall until commit");
            }
        }
        records.push_back(Record {
            tmin: txid,
            tmax: TxId::max_value(),
            data,
        });
        Ok(())
    }

    fn get(&self, can_see: impl Fn(TxId) -> bool) -> Result<usize> {
        trace!("get:");
        let records = self.records.lock().unwrap();
        for record in records.iter().rev() {
            if can_see(record.tmin) {
                return Ok(record.data);
            }
        }
        Err(Error::NotFound)
    }
}
