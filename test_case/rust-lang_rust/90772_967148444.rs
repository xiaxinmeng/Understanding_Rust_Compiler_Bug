plain
    Checking alloc v0.0.0 (/checkout/library/alloc)
    Checking cfg-if v0.1.10
    Checking adler v0.2.3
    Checking rustc-demangle v0.1.21
error[E0277]: expected a `FnMut<(&T,)>` closure, found `F`
     |
     |
1554 |             if !process_one::<F, T, A, false>(&mut f, &mut g) {
     |                 ----------------------------- ^^^^^^ expected an `FnMut<(&T,)>` closure, found `F`
     |                 required by a bound introduced by this call
     |
note: required by a bound in `process_one`
    --> library/alloc/src/vec/mod.rs:1527:16
    --> library/alloc/src/vec/mod.rs:1527:16
     |
1522 |         fn process_one<F, T, A: Allocator, const DELETED: bool>(
     |            ----------- required by a bound in this
1527 |             F: FnMut(&T) -> bool,
1527 |             F: FnMut(&T) -> bool,
     |                ^^^^^^^^^^^^^^^^^ required by this bound in `process_one`
     |
     |
1474 |         F: FnMut(&mut T) -> bool + for<'r> core::ops::FnMut<(&'r T,)>,


error[E0277]: expected a `FnMut<(&T,)>` closure, found `F`
     |
     |
1561 |             process_one::<F, T, A, true>(&mut f, &mut g);
     |             ---------------------------- ^^^^^^ expected an `FnMut<(&T,)>` closure, found `F`
     |             required by a bound introduced by this call
     |
note: required by a bound in `process_one`
    --> library/alloc/src/vec/mod.rs:1527:16
    --> library/alloc/src/vec/mod.rs:1527:16
     |
1522 |         fn process_one<F, T, A: Allocator, const DELETED: bool>(
     |            ----------- required by a bound in this
1527 |             F: FnMut(&T) -> bool,
1527 |             F: FnMut(&T) -> bool,
     |                ^^^^^^^^^^^^^^^^^ required by this bound in `process_one`
     |
     |
1474 |         F: FnMut(&mut T) -> bool + for<'r> core::ops::FnMut<(&'r T,)>,

For more information about this error, try `rustc --explain E0277`.
error: could not compile `alloc` due to 2 previous errors
Build completed unsuccessfully in 0:01:21
