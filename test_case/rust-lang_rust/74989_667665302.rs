rust
use std::ops::Index;

enum Idx {
    _0,
    _1,
}

impl<T> Index<Idx> for [T] {
    type Output = T;

    fn index(&self, idx: Idx) -> &T {
        Index::index(self, idx as usize)
    }
}

// This PR would cause there to be a clashing std library implementation of `Index<Idx> for [T; 2]`
impl<T> Index<Idx> for [T; 2] {
    type Output = T;

    fn index(&self, idx: Idx) -> &T {
        unsafe { self.get_unchecked(idx as usize) }
    }
}

fn main() {
    let array = [11, 22];
    let slice: &[u8] = &array;
    
    println!("{:?}", array[Idx::_0]);
    println!("{:?}", slice[Idx::_0]);
}
