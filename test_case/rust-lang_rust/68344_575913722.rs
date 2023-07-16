rust
#![feature(generic_associated_types)]

struct Example<T> {}

impl<T> Example<T> {
    type O<X> = Example<X>;
    
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Self::O<U> {
        unimplemented!();
    }
}
