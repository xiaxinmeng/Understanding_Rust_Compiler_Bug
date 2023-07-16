rust
trait Foo {
    const X: u8;
}

struct Generic<T>(T);

impl<T> Foo for Generic<T> {
    const X: u8 = 10;
}

const fn hello<T>() -> u8 {
    Generic::<T>::X
}
