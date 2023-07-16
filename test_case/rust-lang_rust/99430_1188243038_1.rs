rust
fn f(p: &mut &mut i32) {

    let mut number = 111;
    *p = &mut number;

    **p = 2;
    println!("{}", **p);
}
