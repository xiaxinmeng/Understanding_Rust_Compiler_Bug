rust
impl<T, Cond, F> Iterator for OnChange<T, Cond, F>
where
    Cond: Fn(&Option<Self::Item>) -> bool,   // (a) hangs the compiler
    Cond: Fn(&Option<T>) -> bool,   // (b) does not hang
    F: Fn((Option<T>, Option<T>)) -> Option<T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
