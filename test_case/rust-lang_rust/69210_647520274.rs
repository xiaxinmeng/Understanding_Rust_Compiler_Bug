rust
pub trait LLRemove<T> {
    fn stabilized_remove(&mut self, at: usize) -> T;
}

impl<T> LLRemove<T> for LinkedList<T>  {
    fn stabilized_remove(&mut self, at: usize) -> T {
        // ...
    }
}
