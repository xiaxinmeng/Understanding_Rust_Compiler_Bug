 Rust
extern {
    #[link_name="__imp__foo"]
    static foo: extern "C" fn() -> u32;
}
