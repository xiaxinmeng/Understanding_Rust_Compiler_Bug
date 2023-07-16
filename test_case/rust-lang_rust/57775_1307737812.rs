rust
use core::marker::PhantomData;

struct A<T>(PhantomData<T>);
impl<T> A<T> {
    const EMPTY: Vec<T> = Vec::new();
    
    fn f() -> [Vec<T>; 2] {
        [Self::EMPTY; 2]
    }
}

fn main() {
    let empty = A::<u32>::f();
    dbg!(empty);
}
