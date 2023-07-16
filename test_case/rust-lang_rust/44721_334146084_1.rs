rust
fn foo(impl Iterator) { .. }

// becomes:

fn foo<T: Iterator>(t: T) { // "universally quantified"
}
