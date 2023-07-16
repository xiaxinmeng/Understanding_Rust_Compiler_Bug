Rust
#[lang = "must_use_fn"]
pub const fn must_use<T, const msg: &'static str>(value: T) -> T {
    value
}
