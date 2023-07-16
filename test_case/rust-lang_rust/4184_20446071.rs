
trait AdditiveGroup<RHS, Result>: ops::Add<RHS, Result> { }

impl uint: AdditiveGroup<uint, uint> { }

struct Mat<T> {
    data: ~[T],
}

impl<T: AdditiveGroup<T, T>> Mat<T> {
    fn f(data: ~[T]) { }

    fn g() {
        self.f(~[])
    }
}

fn main() {}
