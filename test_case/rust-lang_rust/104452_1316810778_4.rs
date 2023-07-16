rs
#[derive(Copy, Clone)]
enum T { A, B(usize) }
fn rep() -> [T; 64] {
    [T::A; 64]
}
