rust
#![feature(generators)]

fn wrapper<T>(value: T) -> T {
    value
}

fn func<T>() -> impl Sized {}

trait Trait<'a> {
    type Assoc;

    fn call() {
        let _ = wrapper(|()| {
            let _value = func::<Self::Assoc>();
            yield;
        });
    }
}

impl Trait<'static> for () {
    type Assoc = ();
}

fn main() {
    <()>::call();
}
