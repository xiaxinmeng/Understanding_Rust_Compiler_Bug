 rust
trait T1: T0 {
    fn m0<F>(self, f: F) -> bool where F: Fn(<Self as T0>::O) -> bool;
}

impl<A> T1 for S<A> {
    fn m0<F>(self, f: F) -> bool where F: Fn(A) -> bool { f(self.0) }
}
