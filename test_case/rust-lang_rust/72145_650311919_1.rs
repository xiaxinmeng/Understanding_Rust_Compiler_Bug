Rust
thread_local! {
    static DUMMY_IMPL: RefCell<Option<TokenStream>> = RefCell::new(None);
}
