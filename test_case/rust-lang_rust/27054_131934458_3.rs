 rust
fn main() {
    let x = Box::new(0);
    let mut y = 0;
    *{ drop(x); &mut y } += *x;
    println!("{}", y);
}
