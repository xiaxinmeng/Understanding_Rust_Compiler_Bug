
error[E0599]: no method named `test` found for type parameter `impl Test` in the current scope
 --> file5.rs:8:7
  |
4 |     fn test(self: Pin<&mut Self>);
  |                   -------------- the method might not be found because of this arbitrary self type
...
8 |     t.test()
  |       ^^^^ method not found in `impl Test`
