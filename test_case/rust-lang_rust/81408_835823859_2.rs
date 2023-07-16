rust
use std::{cell::Cell, marker::PhantomData, sync::Once};

static X: SyncLazy<()> = SyncLazy::new(|| ());

pub fn run() -> impl FnOnce() {
    || {
        let _ = SyncLazy::force(&X);
    }
}

struct SyncOnceCell<T> {
    once: Once,
    _phantom: PhantomData<T>,
}

impl<T> SyncOnceCell<T> {
    const fn new() -> Self {
        SyncOnceCell {
            once: Once::new(),
            _phantom: PhantomData,
        }
    }
    fn get_or_init<F>(&self, _f: F) -> &T
    where
        F: FnOnce() -> T,
    {
        self.once.call_once(|| unimplemented!());
        unimplemented!()
    }
}

struct SyncLazy<T, F = fn() -> T> {
    cell: SyncOnceCell<T>,
    _init: Cell<Option<F>>,
}

unsafe impl<T, F: Send> Sync for SyncLazy<T, F> where SyncOnceCell<T>: Sync {}

impl<T, F> SyncLazy<T, F> {
    const fn new(f: F) -> Self {
        SyncLazy {
            cell: SyncOnceCell::new(),
            _init: Cell::new(Some(f)),
        }
    }
    fn force(this: &Self) -> &T {
        this.cell.get_or_init(|| unimplemented!())
    }
}
