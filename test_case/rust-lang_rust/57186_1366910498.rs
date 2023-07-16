
~/workspace/rust% rustc +dev f57.rs
error[E0308]: mismatched types
  --> f57.rs:11:20
   |
11 |     let x: dyn T = foo(S);
   |            -----   ^^^^^^ expected trait object `dyn T`, found struct `S`
   |            |
   |            expected due to this
   |
   = note: expected trait object `dyn T`
                    found struct `S`

error[E0277]: the size for values of type `dyn T` cannot be known at compilation time
  --> f57.rs:11:9
   |
11 |     let x: dyn T = foo(S);
   |         ^ doesn't have a size known at compile-time
   |
   = help: the trait `Sized` is not implemented for `dyn T`
   = note: all local variables must have a statically known size
   = help: unsized locals are gated as an unstable feature
help: consider borrowing here
   |
11 |     let x: &dyn T = foo(S);
   |            +
