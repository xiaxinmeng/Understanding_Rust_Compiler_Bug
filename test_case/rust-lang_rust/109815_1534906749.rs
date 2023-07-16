rust
#![feature(min_specialization)]

trait X {}
trait Y: X {}
trait Z {
    type Assoc: Y;
}
struct A<T>(T);

impl<T: X> Z for A<T> {}

trait MyFrom<T> {
    fn from(other: T) -> Self;
}

impl<T> MyFrom<T> for T {
    fn from(other: T) -> T {
        other
    }
}

impl<T: X> MyFrom<<A<T> as Z>::Assoc> for T {}
