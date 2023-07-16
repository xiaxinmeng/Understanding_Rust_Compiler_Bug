
impl<T:Simple> Clone for T {
    fn clone(&self) -> T { *self } // compiler knows this is a copy, not move, due to Simple bound
}
