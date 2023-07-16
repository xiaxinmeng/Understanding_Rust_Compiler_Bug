plain

error[E0277]: the size for values of type `[u8]` cannot be known at compilation time
  --> library/std/../backtrace/build.rs:13:9
   |
13 |     let expansion = match cc::Build::new().file("src/android-api.c").try_expand() {
   |
   = help: the trait `Sized` is not implemented for `[u8]`
   = note: all local variables must have a statically known size
   = help: unsized locals are gated as an unstable feature
