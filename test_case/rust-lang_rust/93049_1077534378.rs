rust
fn and_chain() {
    let z;
    if let true = true && { z = 3; true } && z == 3 {}
}
