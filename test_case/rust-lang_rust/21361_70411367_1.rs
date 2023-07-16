 rust
impl<'a, T> Iterator for &'a mut (Iterator<Item=T> + 'a) {
//~^ error: the type parameter `T` is not constrained by the impl trait, self type, or predicates
    type Item = T;
}
