rust
pub struct A<T> {
    data: T,
}

impl<T> A<T> {
    const SIZE: usize = std::mem::size_of::<T>();
}

fn main() {
    println!("{}", A::SIZE);
}
