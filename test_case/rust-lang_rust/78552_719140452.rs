
error[E0277]: a value of type `HashMap<std::string::String, cargo_metadata::Package>` cannot be built from an iterator over elements of type `(std::string::String, cargo_metadata::Package)`
  --> src/utils/cargo_metadata.rs:46:18
   |
46 |                 .collect(),
   |                  ^^^^^^^ value of type `HashMap<std::string::String, cargo_metadata::Package>` cannot be built from `std::iter::Iterator<Item=(std::string::String, cargo_metadata::Package)>`
   |
   = help: the trait `FromIterator<(std::string::String, cargo_metadata::Package)>` is not implemented for `HashMap<std::string::String, cargo_metadata::Package>`
