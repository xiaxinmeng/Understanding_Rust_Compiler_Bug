 rust
#[thread_local]
static PANICKING: Cell<bool> = Cell::new(false);

// and then in on_panic instead of PANICKING.with(||{}) simply use it as regular static which contains a Cell.
