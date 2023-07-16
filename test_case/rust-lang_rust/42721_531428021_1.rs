
error[E0119]: conflicting implementations of trait `PrintAnything`:
  --> src/main.rs:27:1
   |
21 | impl<T: Integer> PrintAnything for T {
   | ------------------------------------ first implementation here
...
27 | impl<T: Collection> PrintAnything for T {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation

error: aborting due to previous error
