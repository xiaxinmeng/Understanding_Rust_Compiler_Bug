rust
fn late<'a>() {}
fn early<'a: 'a>() {}

fn check<'a>() {
    early::<'a>(); // OK
    late::<'a>(); // ERROR: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
}
