rust
macro_rules! make_ref {
    ($ptr:expr) => {{
        let ptr = $ptr;
        if false && size_from_ptr(ptr) == 0 {
            // Use a non-null pointer value
            &*(1 as *mut _)
        } else {
            &*ptr
        }
    }};
}
