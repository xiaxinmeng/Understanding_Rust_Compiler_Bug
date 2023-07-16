rust
fn g(s: &S) {
    println!("{}", s.b);
}

fn f4(s: &mut S) {
    let r = &mut s.a;
    g(s); // ERROR trying to borrow the entire s, while s.a is exclusively borrowed by r
    drop(r);
}
