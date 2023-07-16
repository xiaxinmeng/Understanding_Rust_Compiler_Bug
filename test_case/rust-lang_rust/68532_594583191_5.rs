rust
pub struct A<T>(T);

impl<T> A<T> {
    pub const N: usize = 68;

    pub fn foo(&self) {
        let b = Self::N;
        println!("{}", b);
    }
}
