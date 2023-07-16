 rust
// #[derive]d impl
impl Hash {
   fn hash() { ... } // same-old

   #[if(only_has_one_field)]  // magic
   fn hash_one_shot<H: Hasher>(&self, state: &mut H) -> u64 {
      self.only_field.hash_one_shot(state)
   }
}
