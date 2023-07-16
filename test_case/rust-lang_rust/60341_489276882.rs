rust
match &self.opt {             // calculate address of discriminant
  Some(ref val) => Some(val), // calculate address of value storage
  None => self.slow_path(),   // returns Option<&'static T>
}
