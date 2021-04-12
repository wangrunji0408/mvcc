use super::*;
use crate::tx_manager::Snapshot;

pub struct Transaction<'a> {
    id: TxId,
    status: TxStatus,
    snapshot: Snapshot,
    db: &'a Database,
}

pub type TxId = u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TxStatus {
    Init,
    Active,
    Aborted,
    Committed,
}

impl Default for TxStatus {
    fn default() -> Self {
        TxStatus::Init
    }
}

impl TxStatus {
    fn ensure_is_active(&self) -> Result<()> {
        if let TxStatus::Active = self {
            Ok(())
        } else {
            Err(Error::TxNotActive)
        }
    }
}

impl<'a> Transaction<'a> {
    pub fn begin(db: &'a Database) -> Result<Self> {
        let id = db.tx_manager.alloc_txid();
        info!("tx {}: begin", id);
        db.tx_manager.set_status(id, TxStatus::Active);
        Ok(Transaction {
            id,
            status: TxStatus::Active,
            snapshot: db.tx_manager.get_snapshot(id),
            db,
        })
    }

    pub fn abort(&mut self) -> Result<()> {
        self.status.ensure_is_active()?;
        info!("tx {}: abort", self.id);
        self.status = TxStatus::Aborted;
        self.db.tx_manager.set_status(self.id, self.status);
        Ok(())
        // todo!()
    }

    pub fn commit(&mut self) -> Result<()> {
        self.status.ensure_is_active()?;
        info!("tx {}: committing", self.id);
        self.status = TxStatus::Committed;
        self.db.tx_manager.set_status(self.id, self.status);
        Ok(())
        // todo!()
    }

    pub fn id(&self) -> TxId {
        self.id
    }

    pub fn status(&self) -> TxStatus {
        self.status
    }

    pub(crate) fn snapshot(&self) -> &Snapshot {
        &self.snapshot
    }

    pub fn put(&mut self, key: usize, value: usize) -> Result<()> {
        self.status.ensure_is_active()?;
        let res = self.db.put(&self, key, value);
        if let Err(Error::Abort) = res {
            self.abort().unwrap();
        }
        res
    }

    pub fn get(&mut self, key: usize) -> Result<usize> {
        self.status.ensure_is_active()?;
        let res = self.db.get(&self, key);
        if let Err(Error::Abort) = res {
            self.abort().unwrap();
        }
        res
    }
}

impl Drop for Transaction<'_> {
    fn drop(&mut self) {
        if let TxStatus::Active = self.status {
            warn!("tx {}: drop on running", self.id);
            self.abort().unwrap();
        }
    }
}
