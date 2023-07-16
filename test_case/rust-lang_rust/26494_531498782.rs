rust
// Passing small structs by value.
pub fn parameters_by_value(v: (u64, u64)) -> u64 {                                                                                                                                                          
  v.0 + v.1
}

// Returning small structs by value.
pub fn return_by_value() -> (u64, u64) {
  (3, 4)
}
