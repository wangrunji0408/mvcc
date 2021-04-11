use super::*;

#[test]
fn no_dirty_read() -> Result<()> {
    let db = Database::new();
    let mut tx1 = Transaction::begin(&db)?;
    let mut tx2 = Transaction::begin(&db)?;

    let v0 = tx1.get(0)?;
    tx2.put(0, 1)?;
    let v1 = tx1.get(0)?;
    assert_eq!(v0, v1);
    Ok(())
}

#[test]
fn repeatable_read() -> Result<()> {
    let db = Database::new();
    let mut tx1 = Transaction::begin(&db)?;
    let mut tx2 = Transaction::begin(&db)?;

    let v0 = tx1.get(0)?;
    tx2.put(0, 1)?;
    tx2.commit()?;
    let v1 = tx1.get(0)?;
    assert_eq!(v0, v1);
    Ok(())
}

#[test]
fn write_skew_is_allowed() -> Result<()> {
    let db = Database::new();

    let mut tx0 = Transaction::begin(&db)?;
    tx0.put(1, 1)?;
    tx0.put(2, 2)?;
    tx0.commit()?;

    let mut tx1 = Transaction::begin(&db)?;
    let mut tx2 = Transaction::begin(&db)?;

    let v1 = tx1.get(1)?;
    tx1.put(2, v1)?;
    let v2 = tx2.get(2)?;
    tx2.put(1, v2)?;
    tx1.commit()?;
    tx2.commit()?;

    let mut tx3 = Transaction::begin(&db)?;
    assert_eq!(tx3.get(1)?, v2);
    assert_eq!(tx3.get(2)?, v1);
    Ok(())
}

#[test]
fn write_conflict_should_abort() -> Result<()> {
    let db = Database::new();
    let mut tx1 = Transaction::begin(&db)?;
    let mut tx2 = Transaction::begin(&db)?;

    tx1.put(0, 1)?;
    assert_eq!(tx2.put(0, 2), Err(Error::Abort));
    tx1.commit()?;
    Ok(())
}
