
#[test] #[should_fail]
fn test() {
  spawn(||fail!());
}
