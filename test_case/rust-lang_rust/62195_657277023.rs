rust
pub fn leak<'a>(vec: impl Into<Vec<T>>) -> &'a mut [T]
where
    T: 'a, 
