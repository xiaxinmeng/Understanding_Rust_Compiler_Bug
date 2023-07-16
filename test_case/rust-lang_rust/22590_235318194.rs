
error[E0277]: `C` does not implement required traits
  --> src/test/compile-fail/issue-24356.rs:30:9
   |
9  | impl<T: B> A for T {
   |      ----  note: implementing `B` can satisfy `A` 
...
18 |    A::a(C);
   |         ^ `C` does not impl `A`
