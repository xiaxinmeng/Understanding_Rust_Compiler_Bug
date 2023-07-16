rust
trait FnPtr {
    // Alternatively `-> usize` idc
    fn as_ptr(self) -> *const ();
    // Other additions, like `const VARIADIC: bool` or whatever can be added if needed
}
