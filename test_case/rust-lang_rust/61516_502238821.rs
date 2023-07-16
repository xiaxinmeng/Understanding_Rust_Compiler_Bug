
#![feature(const_generics)]

struct Config(usize);
const CONFIG: Config = Config(5);

struct FakeArray<T, const N: Config>(T);

impl<T, const N: Config> FakeArray<T, { N }> {
    const fn new(val: T) -> Self {
        Self(val)
    }
}

fn main() {
    let fa = FakeArray::new::<_, { CONFIG }>(1);
}
