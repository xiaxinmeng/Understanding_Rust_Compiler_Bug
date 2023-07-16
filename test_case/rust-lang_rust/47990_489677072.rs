rust
trait Trait {
    fn f<T>(&self) where Self: Sized {}
}

struct Param;

fn f(receiver: &dyn Trait) {
    receiver.f::<Param>();
}

fn main() {}
