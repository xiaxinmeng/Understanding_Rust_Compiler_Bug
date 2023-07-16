rust
struct D(usize);

impl Drop for D {
    fn drop(&mut self) {
        dbg!(self.0);
    }
}

fn main() {
    if let Some(d) = Some(D(0)) {
        D(1);
    }
    D(2);

    let mut x = Some(D(3));
    while let Some(d) = x.take() {
        D(4);
    }
    D(5);
}
