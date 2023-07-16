
error[E0412]: cannot find type `Hashmap` in this scope
 --> src/lib.rs:2:8
  |
2 |     m: Hashmap<String, ()>,
  |        ^^^^^^^ not found in this scope
  |
help: did you mean `std::collections::HashMap`?
  |
2 |     m: std::collections::HashMap<String, ()>,
  |        ^^^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to previous error
