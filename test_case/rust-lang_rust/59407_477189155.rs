rust
pub trait TR {
    type A;
}
pub struct P;
pub struct Q;
impl TR for Q {
    type A = P;
}
pub type QA = <Q as TR>::A;
impl From<&str> for QA {
    fn from(_ : &str) -> Self {
        unimplemented!()
    }
}
