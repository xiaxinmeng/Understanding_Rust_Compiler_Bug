rust
impl<'a, T> Add<&&'a T> for String 
where
    String: Add<&'a T>
