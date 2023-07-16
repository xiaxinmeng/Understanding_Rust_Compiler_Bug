 rust
trait MyFn: Fn(usize) -> usize {}

// Add this blanket implementation
impl<F> MyFn for F where F: Fn(usize) -> usize {}

fn call<F>(f: F) -> usize where F: MyFn {
    f(5)
}

fn main() {
    call(|a| a + 3);
}
