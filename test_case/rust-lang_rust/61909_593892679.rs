rust
fn f<'a>(v: &'a [i32]) -> impl Iterator<Item = i32> + 'a {
    v.iter().map(|e| if v.len() > 10 { e * 2 } else { e * 4 })
}

fn main() {
    let v = vec![1, 2, 3];
    for e in f(&v) {
        dbg!(e);
    }
}
