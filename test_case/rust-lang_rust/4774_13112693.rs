
impl extern fn(T,T) -> T : Op<T> {
    fn call(&self, a:T, b:T) -> T { (*self)(a,b) }
}
