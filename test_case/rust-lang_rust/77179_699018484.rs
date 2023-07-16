rust
#![feature(type_alias_impl_trait)]

type Pointer<T> = impl std::ops::Deref<Target=T>;

fn test() -> Pointer<i32> {
    Box::new(1)
}

fn main() {
    println!("{:?}", *test());
}
