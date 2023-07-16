rust
fn main() {
    let a = &1;
    let b = &2;
    let val = if true {
        *a
    } else if true {
        b
    } else {
        &0
    };
}
