rs
#[derive(Copy, Clone)]
enum T { A, B }
fn rep() -> [T; 64] {
    [T::A; 64]
}
