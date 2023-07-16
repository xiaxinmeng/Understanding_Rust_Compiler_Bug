rust
impl RangeArgument<T> for RangeToInclusive<T> {
   fn end(&self) {
     Some(self.end+1)
   }
}
