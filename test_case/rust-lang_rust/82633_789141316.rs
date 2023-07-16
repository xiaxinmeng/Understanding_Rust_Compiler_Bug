
error[E0277]: the size for values of type `R` cannot be known at compilation time
   --> <source>:1:11
    |
1   | fn foo<F: FnOnce() -> R, R: ?Sized>() {}
    |           ^^^^^^^^^^^^^  - this type parameter needs to be `Sized`
    |           |
    |           doesn't have a size known at compile-time
