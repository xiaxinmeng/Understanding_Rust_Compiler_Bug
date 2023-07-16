rs
enum Enum {
    One,
    Two,
    Three,
}
fn main() {
    use Enum::*;
    let a = One;
    let b = Two;
    match (a, b) {
        (One, One) => {}
        (Two, One) => {}
        //(Three, _) => {}
    }
}
