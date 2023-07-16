
pub struct Repeat<T> { val: T }

impl<T:Clone> Iterator<T> for Repeat<T> {
    fn next(&self) -> Option<T> { Some(self.val.clone()) }
} 
