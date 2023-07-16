rust
fn foo(function_pointer: fn(&mut u32)) {
    Clone::clone(&function_pointer);
}
