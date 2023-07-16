 rust
#[test]
fn test1() {
    fail!()
}
#[test]
fn test2() {
}
#[test]
#[should_fail]
fn test3() {
    fail!()
}
#[test]
#[should_fail]
fn test4() {
}
