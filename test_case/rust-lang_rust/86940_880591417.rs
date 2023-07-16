rust
#![warn(rust_2021_compatibility)]

struct Generic<T>(T);

trait Hey { fn from_iter(_: i32) -> Self; }

impl Hey for Generic<i32> { fn from_iter(x: i32) -> Self { Self(x) } }

impl std::iter::FromIterator<i32> for Generic<i32> { fn from_iter<T: IntoIterator<Item = i32>>(_: T) -> Self { todo!() } }

fn main() {
    <Generic<_>>::from_iter(1);
}
