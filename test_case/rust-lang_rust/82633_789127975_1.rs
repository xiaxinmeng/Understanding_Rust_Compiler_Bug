
error[E0277]: the size for values of type `str` cannot be known at compilation time
  --> src/main.rs:15:5
   |
4  | trait A { fn a() where Self: Sized; }
   |           ------------------------- required by `A::a`
...
15 |     str::a();
   |     ^^^^^^ doesn't have a size known at compile-time
