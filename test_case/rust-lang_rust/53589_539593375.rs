rust
pub fn value<U>(&mut self, arg: U) -> &R
    where T: FnMut(U) -> R
{
    if self.value.as_ref().is_none() {
        self.value = Some((self.calculation)(arg));
    }
    
    self.value.as_ref().unwrap()
}
