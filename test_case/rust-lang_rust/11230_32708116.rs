 rust
trait CopyOnWrite<T> {
    /// check to see if there is one reference to the container
    fn is_owned(&self) -> bool;

    /// clone the data self points to, then modify self to point
    /// to point to the newly cloned data.
    fn to_owned(&mut self);

    /// get a mutable reference to the data self points to. 
    /// if the data structure is shared with multiple pointers
    /// clone the data first, and return a reference to the cloned data
    fn cow_get_mut(&mut self) -> &'a mut T {
        if !self.is_owned {
            self.to_owned();
        }
        unsafe { &mut *self.get() }
    }
}
