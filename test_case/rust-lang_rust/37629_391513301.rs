
error[E0603]: trait `RLINQ` is private
  --> src/main.rs:14:9
   |
14 |     use rlinq::RLINQ;
   |         ^^^^^^^^^^^^

error[E0599]: no method named `is_where` found for type `std::vec::Vec<std::string::String>` in the current scope
  --> src/main.rs:18:11
   |
18 |         x.is_where();
   |           ^^^^^^^^
   |
   = help: items from traits can only be used if the trait is implemented and in scope
   = note: the following trait defines an item `is_where`, perhaps you need to implement it:
           candidate #1: `rlinq::RLINQ`
