 rust
pub struct G;

impl<T, U> FnMut<(U,), T> for G {
    extern "rust-call" fn call_mut(&mut self, (i,):(U,)) -> T {
        panic!()
    }
}

pub fn main() {} 
