rust
pub trait Arbitrary<'a> {
    fn arbitrary(u: &'a mut String); 
}
