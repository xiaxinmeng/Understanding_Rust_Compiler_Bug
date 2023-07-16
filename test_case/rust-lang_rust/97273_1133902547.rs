rust
struct S {
    a: i32,
    b: i32,
}

fn f0(s: &mut S) {
    println!("{}", s.a); // OK
    let r = &mut s.a;
    println!("{}", s.a); // ERROR
    drop(r);
}
