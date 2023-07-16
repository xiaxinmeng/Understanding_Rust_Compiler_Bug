rust
fn foo<'a, 'b>(...) { // note: a and b have no relation 
    callback(|x| {
        // x: &'a &'b i32, so we get an implied relationship that `'a: 'b`.
        //
        // this ought to be impossible, but maybe the closure is never called.
    })
}
