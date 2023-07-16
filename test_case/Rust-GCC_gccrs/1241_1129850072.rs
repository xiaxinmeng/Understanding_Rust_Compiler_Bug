
enum E {
    X(u8),
}

fn main() -> i32 {
    let mut res = 0;
    let v = E::X(4);
    if let E::X(n) = v {
        res = n;
    }

    0
}
