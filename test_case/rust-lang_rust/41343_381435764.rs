rust
use std::ops::Index;

enum J_List<T> {
    Cons(T, Box<J_List<T>>),
    Nil,
}

struct List<T> {
    value: J_List<T>,
}

impl<T> Index<usize> for List<T> {
    type Output = T;

    fn index(self, x: usize) -> &T {
        unimplemented!();
    }
}
