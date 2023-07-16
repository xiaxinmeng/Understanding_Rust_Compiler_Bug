 rust
fn call<F, T(f: F) -> T 
    where F: FnOnce(i32) -> T
{ 
    f(1)
}
