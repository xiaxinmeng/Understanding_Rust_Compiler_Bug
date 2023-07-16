rust
   // maybe poorly named, just an example
   pub enum SetResult<'a, T> { Set(&'a T), WasSet(&'a T, T) }
   pub fn set(&self, value: T) -> SetResult<'_, T>);
   