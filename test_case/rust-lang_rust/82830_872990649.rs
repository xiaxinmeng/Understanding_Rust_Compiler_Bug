rust
trait A<Y> { type B; }
type MaybeBox<T> = <T as A<T>>::B;

struct P { _t: MaybeBox<P> }

impl<Y> A<Y> for P {
    type B = Box<P>;
}

fn main() {
    let _t: MaybeBox<P>;
}
