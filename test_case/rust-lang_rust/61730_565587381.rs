
error[E0599]: no associated item named `BAR` found for type `S` in the current scope
  --> file12.rs:12:19
   |
12 |     data: [u8; S::BAR],
   |                   ^^^ associated item not found in `S`
   |
   = help: items from traits can only be used if the type parameter is bounded by the trait
help: the following trait defines an item `BAR`, perhaps you need to restrict type parameter `S` with it:
   |
11 | struct Array<S: Bar> where S: Bar {
   |              ^^^^^^
