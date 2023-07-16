 rust
fn foo(s: &str) -> *c_char { 
    s.to_c_str().as_ptr() 
}
