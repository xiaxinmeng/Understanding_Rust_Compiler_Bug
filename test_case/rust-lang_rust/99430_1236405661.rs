rust
fn f(mut p @ _: &mut i32) {
    let mut number = 111;
    p = &mut number;
    *p = 2;
    println!("{}", *p);
}
