rust
fn main() {
    let mut xs: Vec<Box<dyn Fn()>> = Vec::new();
    xs.push(Box::new(|| println!("I've been used!")));
    xs.push(Box::new(|| println!("I've been used, two!")));
    xs[0]();
    xs.remove(1);
}
