rs
struct T {
    v: Vec<Box<usize>>,
}
fn main() {
    let mut t = T { v: vec![] };
    t.v.push(Box::new(1usize));
}
