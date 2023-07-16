rust
fn cast_to_static(fn_: fn(&'static u8)) -> fn(&'a u8) {
    // This *must* not be allowed to work.
    (&fn_ as &dyn Any).downcast_ref().unwrap()
}

static CELL: SyncOnceCell<&'static u8> = OnceCell::new();
fn store_static(val: &'static u8) {
    CELL.get_or_init(|| val)
}

fn break_it() {
    let stack: u8 = 0u8;
    cast_from_static(store_static)(&stack)
}
