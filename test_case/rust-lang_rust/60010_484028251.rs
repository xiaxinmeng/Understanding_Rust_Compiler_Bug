rust
impl<T> SourceDatabase for T 
    where
        T: RefUnwindSafe, // [1]
        T: HasQueryGroup, // [2]
{}
