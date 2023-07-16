 rust
fn main() {
    let x = Box::new(0);
    let mut y = 0;
    *{ drop(x); let _ = Box::new(main); &mut y } = *x;
    println!("{}", y);
}
