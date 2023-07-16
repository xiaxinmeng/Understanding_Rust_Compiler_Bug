rust
fn foo<const N: usize>() where { const_fn_bound(N) } {
     // ..
}

fn bar<const N: usize>() where { const_fn_bound(N + 1) } {
// ^ this where clause is required for the call to `foo` to compile
     foo::<N + 1>();
}
