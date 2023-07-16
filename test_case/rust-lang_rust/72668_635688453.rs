rust
// run-rustfix
fn apply_to_3<F>(f: F) -> i32 where F: Fn<i32, Output=i32> { f(3) }
//~^ ERROR E0658
