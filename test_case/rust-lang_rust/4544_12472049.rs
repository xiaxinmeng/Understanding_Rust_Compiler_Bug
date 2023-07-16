
fn double(x: &int) -> int {
    *x * 2
}

fn main() {
    let v = (~[1, 2, 3]).map(double);
}
