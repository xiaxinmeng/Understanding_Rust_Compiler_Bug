 rust
fn main() {
    let mut fns: Vec<Box<FnMut()>> = vec![Box::new(|| println!("called"))];

    (fns[0])(); // error: expected function, found `Box<std::ops::FnMut()>`
    (*&mut fns[0])();
}
