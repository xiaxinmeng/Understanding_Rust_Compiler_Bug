rust
trait Dual<T, E> : Sized + From<Result<T, E>> + Into<Result<T, E>> {
    fn into_result(self) -> Result<T, E> {
        self.into()
    }
    fn and<U, B: Dual<U, E>>(self, b: B) -> B {
        self.into_result().and(b.into_result()).into()
    }
    fn and_then<U, B: Dual<U, E>, F: FnOnce(T) -> B>(self, f: F) -> B {
        self.into_result().and_then(|r| f(r).into_result()).into()
    }
    fn or<C, B: Dual<T, C>>(self, b: B) -> B {
        self.into_result().or(b.into_result()).into()
    }
    fn or_else<B: Dual<T, E>, F: FnOnce(E) -> B>(self, f: F) -> B {
        self.into_result().or_else(|r| f(r).into_result()).into()
    }
    fn map<U, B: Dual<U, E>, F: FnOnce(T) -> U>(self, f: F) -> B {
        self.into_result().map(|r| f(r)).into()
    }
    fn map_err<C, B: Dual<T, C>, F: FnOnce(E) -> C>(self, f: F) -> B {
        self.into_result().map_err(|e| f(e)).into()
    }
}
