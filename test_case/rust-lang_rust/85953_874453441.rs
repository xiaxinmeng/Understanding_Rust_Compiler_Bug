plain
    Checking object v0.22.0
    Checking miniz_oxide v0.4.0
    Checking hashbrown v0.11.0
    Checking addr2line v0.14.0
error: cannot find macro `weak` in this scope
    |
    |
548 |     weak!(fn __pthread_get_minstack(*const libc::pthread_attr_t) -> libc::size_t);
    |
    = note: consider importing this macro:
            crate::sys::weak::weak


error[E0425]: cannot find value `__pthread_get_minstack` in this scope
    |
    |
550 |     match __pthread_get_minstack.get() {

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0425`.
