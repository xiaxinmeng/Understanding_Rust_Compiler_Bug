Rust
pub const unsafe fn fake_type<T>() -> T {
    hint_unreachable()
}

pub const unsafe fn hint_unreachable() -> ! {
    fake_type()
}
