rust
pub trait Arbitrary {
    fn arbitrary<'a>(u: &'a mut String); 
}
