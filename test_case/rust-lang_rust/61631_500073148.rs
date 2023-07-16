rust
enum L<P = Self> {
    N,
    C(u8, Box<P>),
}

fn main() {
    L::C(0, Box::new(L::<()>::N));
}
