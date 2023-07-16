rust
enum E {
    A(i32),
    B(i32),
    C(i32),
    D(i32),
}

#[no_mangle]
fn add1_match(x: &E, y: &E) -> i32 {
    if (mem::discriminant(x) != mem::discriminant(y)) {
        return 99992;
    }

    match (x,y) {
        (&E::A(x), &E::A(y)) => x+y,
        (&E::B(x), &E::B(y)) => x+y,
        (&E::C(x), &E::C(y)) => x+y,
        (&E::D(x), &E::D(y)) => x+y,
        _ => 99992,
    }
}
