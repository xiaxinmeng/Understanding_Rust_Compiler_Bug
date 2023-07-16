rust
trait Unwrap<T, E> : Dual<T, E> {
    fn unwrap(self) -> T;
    fn unwrap_err(self) -> E;
}
use std::fmt::Debug;
impl<T: Debug, E: Debug, A: Dual<T, E>> Unwrap<T, E> for A {
    fn unwrap(self) -> T {
        self.into_result().unwrap()
    }
    fn unwrap_err(self) -> E {
        self.into_result().unwrap_err()
    }
}
// [E] conflicting implementations of trait `Unwrap<_, _>`
impl<T, E, A: Dual<T, E>> Unwrap<T, E> for A {
    fn unwrap(self) -> T {
        match self.into_result() {
            Ok(t) => t,
            Err(e) => panic!("unwrap found Err"),
        }
    }
    fn unwrap_err(self) -> E {
        match self.into_result() {
            Err(e) => e,
            Ok(_) => panic!("unwrap_err found Ok"),
        }
    }
}
// [E] conflicting implementations of trait `Unwrap<_, _>`
impl<T: Debug, E, A: Dual<T, E>> Unwrap<T, E> for A {
    fn unwrap(self) -> T {
        match self.into_result() {
            Ok(t) => t,
            Err(e) => panic!("unwrap found Err"),
        }
    }
    fn unwrap_err(self) -> E {
        self.into_result().unwrap_err()
    }
}
// [E] conflicting implementations of trait `Unwrap<_, _>`
impl<T, E: Debug, A: Dual<T, E>> Unwrap<T, E> for A {
    fn unwrap(self) -> T {
        self.into_result().unwrap()
    }
    fn unwrap_err(self) -> E {
        match self.into_result() {
            Err(e) => e,
            Ok(_) => panic!("unwrap_err found Ok"),
        }
    }
}
