rust
fn main() {
    let split = match Some(1) {
        Some(v) => v,
        None => return,
    };

    let _prev = Some(split);
    assert_eq!(split, 1);
    //~^ fails with `left: 0, right: 1`
}
