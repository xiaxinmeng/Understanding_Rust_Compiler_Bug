 rust
#[thread_local]
static ROOT_STACK: RootStack = RootStack {
    roots: [Cell::new(ptr::null()); 32 * 1024],
    top: Cell::new(0)
};
