
error[E0599]: no method named `count` found for array `[{integer}; 4]` in the current scope
  --> $DIR/count2len.rs:5:11
   |
LL |     slice.count();
   |          -^^^^^--
   |          ||
   |          |method cannot be called on `[{integer}; 4]` due to unsatisfied trait bounds
   |          help: consider using `len` instead
