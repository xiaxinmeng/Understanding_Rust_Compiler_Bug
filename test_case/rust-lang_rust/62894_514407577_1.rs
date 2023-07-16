rust
/// Use a map-style function (`FnOnce(T) -> T`) to overwrite a `&mut T`. Useful
/// when using a `flat_map_*` or `filter_map_*` method within a `visit_`
/// method. Abort the program if the closure panics.
//
// No `noop_` prefix because there isn't a corresponding method in `MutVisitor`.
pub fn visit_clobber<T, F>(t: &mut T, f: F) where F: FnOnce(T) -> T {
    unsafe {
        // Safe because `t` is used in a read-only fashion by `read()` before
        // being overwritten by `write()`.
        let old_t = ptr::read(t);
        let new_t = panic::catch_unwind(panic::AssertUnwindSafe(|| f(old_t)))
            .unwrap_or_else(|_| process::abort());
        ptr::write(t, new_t);
    }
}
