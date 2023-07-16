 rust
macro_rules! self_calling_method(
  ($name:ident) => (fn x(&self){self.$name()})
)

//...
  self_calling_method(y)
