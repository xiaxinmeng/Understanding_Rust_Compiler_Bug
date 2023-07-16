rust
fn f2(mut s: Pin<&mut S>) {
    let r = &mut s.a; // looks like a disjoint borrow, but actually the entire s is borrowed
    println!("{}", s.b); // ERROR, s is not accessible here
    drop(r);
}
