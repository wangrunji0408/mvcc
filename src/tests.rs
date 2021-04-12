use super::*;
use std::sync::Once;

fn init_logger() {
    static INIT_LOGGER: Once = Once::new();
    INIT_LOGGER.call_once(|| env_logger::init());
}

#[test]
fn no_dirty_read() -> Result<()> {
    init_logger();
    let db = Database::new();

    let mut tx0 = Transaction::begin(&db).unwrap();
    tx0.put(0, 0).unwrap();
    tx0.commit().unwrap();

    let mut tx1 = Transaction::begin(&db).unwrap();
    let mut tx2 = Transaction::begin(&db).unwrap();

    let v0 = tx1.get(0).unwrap();
    tx2.put(0, 1).unwrap();
    let v1 = tx1.get(0).unwrap();
    assert_eq!(v0, v1);

    tx1.commit().unwrap();
    tx2.commit().unwrap();
    Ok(())
}

#[test]
fn repeatable_read() -> Result<()> {
    init_logger();
    let db = Database::new();

    let mut tx0 = Transaction::begin(&db).unwrap();
    tx0.put(0, 0).unwrap();
    tx0.commit().unwrap();

    let mut tx1 = Transaction::begin(&db).unwrap();
    let mut tx2 = Transaction::begin(&db).unwrap();

    let v0 = tx1.get(0).unwrap();
    tx2.put(0, 1).unwrap();
    tx2.commit().unwrap();
    let v1 = tx1.get(0).unwrap();
    assert_eq!(v0, v1);

    tx1.commit().unwrap();
    Ok(())
}

#[test]
fn write_skew_is_allowed() -> Result<()> {
    init_logger();
    let db = Database::new();

    let mut tx0 = Transaction::begin(&db).unwrap();
    tx0.put(1, 1).unwrap();
    tx0.put(2, 2).unwrap();
    tx0.commit().unwrap();

    let mut tx1 = Transaction::begin(&db).unwrap();
    let mut tx2 = Transaction::begin(&db).unwrap();

    let v1 = tx1.get(1).unwrap();
    tx1.put(2, v1).unwrap();
    let v2 = tx2.get(2).unwrap();
    tx2.put(1, v2).unwrap();
    tx1.commit().unwrap();
    tx2.commit().unwrap();

    let mut tx3 = Transaction::begin(&db).unwrap();
    assert_eq!(tx3.get(1).unwrap(), v2);
    assert_eq!(tx3.get(2).unwrap(), v1);
    tx3.commit().unwrap();
    Ok(())
}

#[test]
fn write_conflict_should_abort() -> Result<()> {
    init_logger();
    let db = Database::new();
    let mut tx1 = Transaction::begin(&db).unwrap();
    let mut tx2 = Transaction::begin(&db).unwrap();

    tx2.put(0, 2).unwrap();
    assert_eq!(tx1.put(0, 1), Err(Error::Abort));
    assert_eq!(tx1.status(), TxStatus::Aborted);

    tx2.commit().unwrap();
    Ok(())
}
