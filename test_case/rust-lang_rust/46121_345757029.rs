
[00:03:22] error[E0277]: the trait bound `T: core::fmt::Pointer` is not satisfied
[00:03:22]     --> /checkout/src/liballoc/arc.rs:1331:9
[00:03:22]      |
[00:03:22] 1331 |         fmt::Pointer::fmt(&**self, f)
[00:03:22]      |         ^^^^^^^^^^^^^^^^^ the trait `core::fmt::Pointer` is not implemented for `T`
[00:03:22]      |
[00:03:22]      = help: consider adding a `where T: core::fmt::Pointer` bound
[00:03:22]      = note: required by `core::fmt::Pointer::fmt`
[00:03:22] 
[00:03:22] error[E0277]: the trait bound `T: core::fmt::Pointer` is not satisfied
[00:03:22]     --> /checkout/src/liballoc/rc.rs:1075:9
[00:03:22]      |
[00:03:22] 1075 |         fmt::Pointer::fmt(&**self, f)
[00:03:22]      |         ^^^^^^^^^^^^^^^^^ the trait `core::fmt::Pointer` is not implemented for `T`
[00:03:22]      |
[00:03:22]      = help: consider adding a `where T: core::fmt::Pointer` bound
[00:03:22]      = note: required by `core::fmt::Pointer::fmt`
[00:03:22] 
[00:03:23] error: aborting due to 2 previous errors
[00:03:23] 
[00:03:23] error: Could not compile `alloc`.
