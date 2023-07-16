rust
fn matching(x: E2<Infallible>) {
    if let E2::V1 { .. } = x {
        unreachable!()
    }
}

pub fn main() {
    matching(E2::V3);
}
