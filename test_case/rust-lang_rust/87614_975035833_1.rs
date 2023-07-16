
error[E0599]: no method named `count` found for array `[{integer}; 4]` in the current scope
  --> $DIR/count2len.rs:5:11
   |
LL |     slice.count();
   |          --------
   |          |
   |          help: consider using `len` instead
   |
   note: `count` is defined on `std::iter::Iterator`, which `[{integer}; 4]` does not implement
