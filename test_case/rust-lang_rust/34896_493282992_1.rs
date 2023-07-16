error[E0308]: mismatched types
  --> src/main.rs:12:16
   |
12 |     let h: H = h.clone();
   |                ^^^^^^^^^ expected struct `std::collections::HashMap`, found reference
   |
   = note: expected type `std::collections::HashMap<std::string::String, std::vec::Vec<(Foo, Bar)>>`
              found type `&std::collections::HashMap<std::string::String, std::vec::Vec<(Foo, Bar)>>`

error: aborting due to previous error
