
fn main() -> i32 {
    let mut res = 0;

    enum E {
        X(u8),
    }
    let v = E::X(4);
    if let E::X(n) = v {
        res = n;
    }

    0
}
