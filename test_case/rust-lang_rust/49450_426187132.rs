rust
union Transmute<T, U> {
    t: T,
    u: U,
}
pub const unsafe fn my_hacky_transmute<T, U>(t: T) -> U {
    Transmute { t }.u
}
