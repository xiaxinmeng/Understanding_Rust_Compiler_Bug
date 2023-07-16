
struct S;

mod m {
    use super::S;
    pub use super::*;
}

fn main() {
    let s = m::S;
}
