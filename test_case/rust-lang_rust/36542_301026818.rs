rust
fn foo2<'a>(mut b: &'a mut u64, c: &'a mut u64) {
    let x = &mut b; // allowed mutable ref to mutable ref, 
                    // because 'mut b' above instead of 'b'.
    *x = c; // allowed to deref the mutable ref.
}

fn foo3<'a>(mut b: &'a mut u64, c: &'a mut u64) {
    b = c; // allowed because of 'mut b' above instead of just 'b'.
}
