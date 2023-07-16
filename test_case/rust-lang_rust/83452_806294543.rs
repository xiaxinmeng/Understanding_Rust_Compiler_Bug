rust
fn clone_something<T>(x: &T) -> T {
     <T as std::clone::Clone>::clone(x)
}
