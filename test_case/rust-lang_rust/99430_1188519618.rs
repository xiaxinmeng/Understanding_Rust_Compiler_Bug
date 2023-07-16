rust
fn f((_i, mut p): (usize, &mut i32)) {

    let mut number = 111;
    p = &mut number;

    *p = 2;
    println!("{}", *p);
}
