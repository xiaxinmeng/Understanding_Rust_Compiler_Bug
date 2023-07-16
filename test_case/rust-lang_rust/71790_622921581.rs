rust
struct Struct<T> {
    field: *const T,
}

// Overly strict implicit `Sized` bound, consider making this trait bound explicit:
struct Struct<T: ?Sized> {
    field: *const T,
}
// or
struct Struct<T: Sized> {
    field: *const T,
}
