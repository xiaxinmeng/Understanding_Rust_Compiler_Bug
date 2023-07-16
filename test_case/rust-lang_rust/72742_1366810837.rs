
error[E0277]: the size for values of type `[u8]` cannot be known at compilation time
 --> src/test/ui/unsized-locals/suggest-borrow.rs:2:9
  |
2 |     let x: [u8] = vec!(1, 2, 3)[..]; //~ ERROR E0277
  |         ^ doesn't have a size known at compile-time
  |
  = help: the trait `Sized` is not implemented for `[u8]`
  = note: all local variables must have a statically known size
  = help: unsized locals are gated as an unstable feature
help: consider borrowing here
  |
2 |     let x: [u8] = &vec!(1, 2, 3)[..]; //~ ERROR E0277
  |                   +

error[E0308]: mismatched types
 --> src/test/ui/unsized-locals/suggest-borrow.rs:3:20
  |
3 |     let x: &[u8] = vec!(1, 2, 3)[..]; //~ ERROR E0308
  |            -----   ^^^^^^^^^^^^^^^^^
  |            |       |
  |            |       expected `&[u8]`, found slice `[{integer}]`
  |            |       help: consider borrowing here: `&vec!(1, 2, 3)[..]`
  |            expected due to this

error[E0308]: mismatched types
 --> src/test/ui/unsized-locals/suggest-borrow.rs:4:19
  |
4 |     let x: [u8] = &vec!(1, 2, 3)[..]; //~ ERROR E0308
  |            ----   ^^^^^^^^^^^^^^^^^^ expected slice `[u8]`, found `&[{integer}]`
  |            |
  |            expected due to this
  |
help: consider removing the borrow
  |
4 -     let x: [u8] = &vec!(1, 2, 3)[..]; //~ ERROR E0308
4 +     let x: [u8] = vec!(1, 2, 3)[..]; //~ ERROR E0308
  |
help: alternatively, consider changing the type annotation
  |
4 |     let x: &[u8] = &vec!(1, 2, 3)[..]; //~ ERROR E0308
  |            +

error[E0277]: the size for values of type `[u8]` cannot be known at compilation time
 --> src/test/ui/unsized-locals/suggest-borrow.rs:4:9
  |
4 |     let x: [u8] = &vec!(1, 2, 3)[..]; //~ ERROR E0308
  |         ^ doesn't have a size known at compile-time
  |
  = help: the trait `Sized` is not implemented for `[u8]`
  = note: all local variables must have a statically known size
  = help: unsized locals are gated as an unstable feature
