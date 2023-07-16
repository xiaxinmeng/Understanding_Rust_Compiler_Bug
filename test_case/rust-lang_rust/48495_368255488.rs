rust
fn bar(foo: SimpleFoo, other_num: &mut u8) {
    match {foo} {
        SimpleFoo::Ref(ref mut inner) => {
            *inner = *other_num;
        },
        _ => panic!()
    }
    let x = foo; // ERROR: use after move
}

// Similar example, with an lvalue
fn bar(foo: SimpleFoo, other_num: &mut u8) {
    match foo {
        // note: mut removed to suppress the other error
        SimpleFoo::Ref(ref inner) => {

        },
        _ => panic!()
    }
    let x = foo; // ok; the match did not move `foo`
}
