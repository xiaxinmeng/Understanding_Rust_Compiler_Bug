plain
   Compiling addr2line v0.16.0
error[E0255]: the name `sys` is defined multiple times
   --> library/std/src/lib.rs:572:1
    |
488 | pub use crate::sys;
    |         ---------- previous import of the module `sys` here
572 | mod sys;
572 | mod sys;
    | ^^^^^^^^ `sys` redefined here
    |
    = note: `sys` must be defined only once in the type namespace of this module
help: you can use `as` to change the binding name of the import
    |
488 | pub use crate::sys as other_sys;


error[E0365]: `sys` is only public within the crate, and cannot be re-exported outside
    |
488 | pub use crate::sys;
488 | pub use crate::sys;
    |         ^^^^^^^^^^ re-export of crate public `sys`
    |
    = note: consider declaring type or module `sys` with `pub`
Some errors have detailed explanations: E0255, E0365.
For more information about an error, try `rustc --explain E0255`.
error: could not compile `std` due to 2 previous errors
Build completed unsuccessfully in 0:00:18
