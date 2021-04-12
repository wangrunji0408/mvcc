use crate::tx::{TxId, TxStatus};
use std::collections::{HashMap, HashSet};
use std::sync::{atomic::*, Mutex};

#[derive(Default)]
pub struct TxManager {
    txid: AtomicU64,
    inner: Mutex<Inner>,
}

#[derive(Default)]
struct Inner {
    status: HashMap<TxId, TxStatus>,
    active_set: HashSet<TxId>,
}

impl TxManager {
    /// Allocate a new transaction ID.
    pub fn alloc_txid(&self) -> TxId {
        self.txid.fetch_add(1, Ordering::SeqCst)
    }

    /// Get current snapshot of a transaction.
    pub fn get_snapshot(&self, txid: TxId) -> Snapshot {
        let inner = self.inner.lock().unwrap();
        Snapshot {
            txid,
            active_set: inner.active_set.clone(),
        }
    }

    /// Set status of a transaction.
    pub fn set_status(&self, txid: TxId, status: TxStatus) {
        let mut inner = self.inner.lock().unwrap();
        inner.status.insert(txid, status);
        if let TxStatus::Active = status {
            inner.active_set.insert(txid);
        } else {
            inner.active_set.remove(&txid);
        }
    }

    /// Get status of a transaction.
    pub fn get_status(&self, txid: TxId) -> TxStatus {
        let inner = self.inner.lock().unwrap();
        inner.status.get(&txid).cloned().unwrap_or_default()
    }
}

#[derive(Debug)]
pub struct Snapshot {
    txid: TxId,
    active_set: HashSet<TxId>,
}

impl Snapshot {
    pub fn can_see(&self, txid: TxId) -> bool {
        debug!("{:?} can see {:?}?", self, txid);
        txid == self.txid || txid < self.txid && !self.active_set.contains(&txid)
    }
}
