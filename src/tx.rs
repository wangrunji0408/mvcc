use super::*;

pub struct Transaction<'a> {
    id: u64,
    status: TxStatus,
    db: &'a Database,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TxStatus {
    Running,
    Aborted,
    Committed,
}

impl<'a> Transaction<'a> {
    pub fn begin(db: &'a Database) -> Result<Self> {
        todo!()
    }

    pub fn abort(&mut self) -> Result<()> {
        todo!()
    }

    pub fn commit(&mut self) -> Result<()> {
        todo!()
    }

    pub fn status(&self) -> TxStatus {
        self.status
    }

    pub fn put(&mut self, key: usize, value: usize) -> Result<()> {
        todo!()
    }

    pub fn get(&mut self, key: usize) -> Result<usize> {
        todo!()
    }

    pub fn del(&mut self, key: usize) -> Result<()> {
        todo!()
    }
}
