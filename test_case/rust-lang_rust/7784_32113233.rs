 rust
fn main() {
    let x = [1, 2, 3];

    match x {
        [_a, _b, _c] => {}
    }
    match x {
        [_a, .. _b] => {}
    }
    match x {
        [_a, .. _b, _c] => {}
    }

    /*
    let [_a, _b, _c] = x;
    let [_a, .. _b] = x;
    let [_a, .. _b, _c] = x;
    */
}
