rust
fn bar(foo: SimpleFoo, other_num: &mut u8) {
    match (0, foo) {
        (0, SimpleFoo::Ref(ref mut inner)) => {
            *inner = *other_num;
        },
        _ => panic!()
    }
    let _ = foo; // This compiles!!
}
