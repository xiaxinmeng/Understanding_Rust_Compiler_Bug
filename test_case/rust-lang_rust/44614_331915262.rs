Rust
struct Newtype<T>(T);

impl<T> Deref for Newtype<T> {
    type Target = T;
    fn deref(&self) -> &T { &self.0 }
}
impl<T> DerefMut for Newtype<T> {
    fn deref_mut(&mut self) -> &mut T { &mut self.0 }
}
unsafe impl<T> DerefPure for Newtype<T> { /* ... */ }

fn main() {
    let n = Newtype(("hello",));
    {
        let s = "hi there".to_string();
        let (mut d,) = n;
        *d = &s; // create a temporary? fail? both options are unsatisfying
    }
}
