
error[E0599]: no method named `is_minus_one` found for struct `A` in the current scope
 --> src/main.rs:3:7
  |
2 |     struct A;
  |     --------- method `is_minus_one` not found for this
3 |     A.is_minus_one();
  |       ^^^^^^^^^^^^ method not found in `A`
  |
  = help: items from traits can only be used if the trait is implemented and in scope
  = note: the following trait defines an item `is_minus_one`, perhaps you need to implement it:
          candidate #1: `std::sys::unix::IsMinusOne`
