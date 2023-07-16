rust
pub struct MyType<T> {
    val: T,
}

impl<T: ToString> MyType<T> {
    pub fn new(val: T) -> Self {
        Self {
            val
        }
    }
    
    pub fn new_from_self(&self, val: u8) -> MyType<u8> {
        MyType {
            val
        }
    }

    pub fn new_from_self_generic(&self, val: u8) -> Self {
        Self {
            val
        }
    }
}
