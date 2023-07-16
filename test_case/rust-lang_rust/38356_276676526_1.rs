rust
   struct IntegerLiteral { negative: bool, decimal_digits: String, type_suffix: Option<String> }
   impl TryInto<u32> IntegerLiteral { type Err = OutOfRange; /* … */ }
   // Other impls for integer types supported in this compiler version

   // Something similarly for floats
   