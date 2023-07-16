rust
trait Foo {
    const A: Self;
}

impl Foo for Option<f32> {
    const A: Self = None;
}

impl Foo for usize {
    const A: Self = 7;
}

struct A<T>(T);

impl<T: Foo> A<T> {
    const VALUE: T = T::A;
}

fn main() {
    match Some(1.0f32) {
        // I do not want to use value based reasoning for `T::A`,
        // so the following should error/warn because `Option<f32>` is not structurally match.
        A::<Option<f32>>::VALUE => (),
        _ => (),
    }

    match 42 {
        // This should be accepted.
        A::<usize>::VALUE => (),
        _ => (),
    }
}

