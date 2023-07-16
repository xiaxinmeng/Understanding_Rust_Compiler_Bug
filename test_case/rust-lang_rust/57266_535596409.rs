
error[E0308]: mismatched types
  --> src/main.rs:10:9
   |
10 |     x = f2();
   |         ^^^^ expected opaque type, found a different opaque type
   |
   = note: expected type `impl std::marker::Copy` (opaque type at <src/main.rs:1:12>)
              found type `impl std::marker::Copy` (opaque type at <src/main.rs:4:12>)
   = note: distinct uses of `impl Trait` result in different opaque types
