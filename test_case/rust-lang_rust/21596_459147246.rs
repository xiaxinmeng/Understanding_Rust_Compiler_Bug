
error[E0599]: no method named `to_string` found for type `*const u8` in the current scope
 --> src/main.rs:8:11
  |
8 |         z.to_string()  // Not OK
  |           ^^^^^^^^^
  |
  = note: the method `to_string` exists but the following trait bounds were not satisfied:
          `*const u8 : std::string::ToString`
