plain
   Compiling libc v0.2.93
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.46
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0107]: missing generics for trait `Unsize`
    |
note: trait defined here, with 1 generic parameter: `T`
    |
    |
125 | pub trait Unsize<T: ?Sized> {
help: add missing generic argument
    |
    |
1   | <T>//! # The Rust Core Library

error: aborting due to previous error

For more information about this error, try `rustc --explain E0107`.
