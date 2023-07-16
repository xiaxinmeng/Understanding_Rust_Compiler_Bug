rust
union Transmuter<T, U> {
    start: T,
    end: U
}

unsafe fn transmute<T, U>(value: T) -> U {
    Transmuter { start: value }.end
}
