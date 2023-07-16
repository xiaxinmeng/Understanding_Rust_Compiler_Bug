rust
fn f<U>() -> [Vec<U>; 2] {
    const EMPTY: Vec<U> = Vec::new();
    [EMPTY; 2]
}

fn main() {
    let empty = f::<u32>();
    dbg!(empty);
}
