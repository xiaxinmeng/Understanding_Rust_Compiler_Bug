Rust
use std::marker::PhantomData;

struct Cacher<T, U: Copy>
    where T: Fn(U) -> U
{
    calculation: T,
    value: Option<u32>,
    __phantom: PhantomData<U> // the name "__phantom" doesn't matter
}

struct Cacher<T, U: Copy>
    where T: Fn(U) -> U
{
    fn new(calculation: T) -> Self {
        Self { calculation, value: None, __phantom: PhantomData }
    }
}

fn main() {
    let c = Cacher::new(|x| x);
    println!("Hello, world!");
}
