 rust
trait Append<Rhs> {
    type Result;
    fn append(self, rhs: Rhs) -> Self::Result;
}

#[derive(Copy, Debug)]
struct Writer<A, B>(A, B);

impl<From, U, Added, T: Append<U, Result = Added>> Writer<From, T> {
    fn write<To, Func: FnOnce(From) -> Writer<To, U>>(self, func: Func) -> Writer<To, Added> {
        let applied = func(self.0);
        Writer(applied.0, self.1.append(applied.1))
    }
}

impl<T, Func: FnOnce(&mut T)> Append<Box<Func>> for T {
    type Result = T;
    fn append(mut self, func: Box<Func>) -> T {
        (*func)(&mut self);
        self
    }
}

fn main() {
    Writer((),()).write(|_| Writer((), Box::new(|_: &mut ()| ())));
}
