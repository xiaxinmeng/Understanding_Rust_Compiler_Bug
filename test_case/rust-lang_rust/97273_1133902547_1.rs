rust
fn f1(s: &mut S) {
    let r = &mut s.a;
    println!("{}", s.b); // OK, because the exclusive borrow was only on s.a
    drop(r);
}
