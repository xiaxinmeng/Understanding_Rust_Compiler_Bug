rust
fn foo(x: *mut u8) { ... }
extern "Rust" {
    fn foos_mangled_name(x: &mut u8);
}
