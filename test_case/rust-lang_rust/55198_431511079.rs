
error[E0308]: mismatched types
  --> src/main.rs:13:27
   |
13 |       ParsedLine{keyword: k}
   |                           ^
   |                           |
   |                           expected enum `std::option::Option`, found reference
   |                           help: use `as_ref` here: `k.as_ref()`
   |
   = note: expected type `std::option::Option<&Keyword>`
              found type `&'a std::option::Option<Keyword>`
