
fn to_any_type<A>() -> A { fail; }
fn shift() {
    let s = to_any_type();
    3 >> s
}
