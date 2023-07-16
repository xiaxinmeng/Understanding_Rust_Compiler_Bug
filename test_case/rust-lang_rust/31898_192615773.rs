
fn main() {
    let v: Vec<usize> = Vec::new();
    let v = &v;
    let w = Some(1).and_then(|_| Some(1).map(|_| &v[..]));
}
