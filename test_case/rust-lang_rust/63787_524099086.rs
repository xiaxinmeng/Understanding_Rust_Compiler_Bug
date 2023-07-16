rust
struct DetachedRc<T> {
    refcount: *mut i32,
    ptr: *const T,
}
