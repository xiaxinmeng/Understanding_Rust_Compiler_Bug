rust
fn f(p: &mut i32) {
    let mut p = p; // <-- suppresses error!

    let mut number = 111;
    p = &mut number;

    *p = 2;
    println!("{}", *p);
}
