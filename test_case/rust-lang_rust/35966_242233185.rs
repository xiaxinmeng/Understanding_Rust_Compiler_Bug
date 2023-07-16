
error[E0094]: intrinsic has wrong number of type parameters: found 2, expected 1
  --> src/test/compile-fail/E0094.rs:13:5
   |
13 |     fn size_of<T, U>() -> usize; //~ ERROR E0094
   |     ^^^^^^^^^^------^^^^^^^^^^^^
   |               |
   |               expected 1 type parameter
