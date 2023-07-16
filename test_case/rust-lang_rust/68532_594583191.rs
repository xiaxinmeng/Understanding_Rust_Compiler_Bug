rust
pub struct A<T>(T);

impl<T> A<T> {
    pub const N: usize = 68;

    pub fn foo(&self) {
        let b = [0; Self::N];
        println!("{}", b.len());
    }
}
