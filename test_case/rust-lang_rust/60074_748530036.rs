rust
trait Function {
    fn call(&self) -> i32;
}

impl<F: Fn() -> Result<i32, bool>> Function for F {
    fn call(&self) -> i32 {
        self().unwrap()
    }
}

impl<F: Fn() -> i32> Function for F {
    fn call(&self) -> i32 {
        self()
    }
}

// error[E0119]: conflicting implementations of trait `Function`
