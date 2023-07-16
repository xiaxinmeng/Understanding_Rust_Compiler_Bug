rust
fn x<F>(_: F)
where
    for<'c> F: Fn(&'c ()) -> &'c (),
{
}

fn main() {
    // This works
    x(|c| c);

    // This fails with "implementation of `FnOnce` is not general enough"
    let z = |c| c;
    x(z);
    
    // This fails with "expected reference &() found reference &'c ()"
    let z = |c: &_| c;
    x(z);
}
