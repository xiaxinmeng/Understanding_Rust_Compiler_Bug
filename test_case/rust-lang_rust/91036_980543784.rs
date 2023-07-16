
pub trait Transaction<'db> {
    type Cursor<'tx, T>: Cursor<'tx, T>
    where
        'db: 'tx,
        Self: 'tx,
        T: Table + 'db + 'tx;
}
