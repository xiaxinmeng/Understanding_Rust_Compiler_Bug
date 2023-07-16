rust
#[derive(Default)]
struct MyTypeWrapper<T = i32>(T);

impl<T> MyTypeWrapper<T> {
    const fn new(t: T) -> Self {
        Self(t)
    }
}

fn main() {
    // This does work but is needlessly verbose IMO
    let a: MyTypeWrapper = MyTypeWrapper::default();

    // This does not work but should IMO
    let a = MyTypeWrapper::default();

    // This should work and does work
    let a: MyTypeWrapper<usize> = MyTypeWrapper::default();
}
