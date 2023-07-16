rust
const fn static_assert(condition) {
    match condition {
        true => (),
        false => panic!("Assertion failed"),
     }
}
