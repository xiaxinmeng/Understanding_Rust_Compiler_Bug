 rust
#[returns_internal_raw_pointer]
fn as_ptr(&self) -> *c_char { ... }

let p = x.to_c_str().as_ptr(); // warning: internal raw pointer may be invalidated at the end of this statement
