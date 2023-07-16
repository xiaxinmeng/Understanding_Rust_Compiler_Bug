rust
impl<T> Option<T> {
    pub fn as_ref(&self) -> Option<&T> {
        match *self {
            Option::Some(ref x) => Option::Some(x),
            Option::None => Option::None,
        }
    }
    
    pub fn unwrap(self) -> T {
        match self {
            Option::Some(x) => x,
            Option::None => panic!("unwrap on None"),
        }
    }
    
    pub fn unwrap_or_default(self) -> T
    where T: Default, {
        match self {
            Option::Some(x) => x,
            Option::None => Default::default(),
        }
    }
}
