
   Compiling playground v0.0.1 (file:///playground)
error[E0599]: no function or associated item named `new` found for type `main::T` in the current scope
 --> src/main.rs:3:5
  |
3 |     T::new();
  |     ^^^^^^
  |
  = help: items from traits can only be used if the trait is implemented and in scope
  = note: the following trait defines an item `new`, perhaps you need to implement it:
          candidate #1: `std::iter::ZipImpl`
