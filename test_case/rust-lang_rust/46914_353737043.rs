
[01:12:44] ---- ptr.rs - ptr::*mut T::copy_from (line 1827) stdout ----
[01:12:44] 	error: the type of this value must be known in this context
[01:12:44]   --> ptr.rs:10:22
[01:12:44]    |
[01:12:44] 10 |     dst.as_mut_ptr().copy_from(ptr, elts);
[01:12:44]    |                      ^^^^^^^^^
[01:12:44]    |
[01:12:44] note: lint level defined here
[01:12:44]   --> ptr.rs:1:9
[01:12:44]    |
[01:12:44] 1  | #![deny(warnings)]
[01:12:44]    |         ^^^^^^^^
[01:12:44]    = note: #[deny(tyvar_behind_raw_pointer)] implied by #[deny(warnings)]
[01:12:44]    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
[01:12:44]    = note: for more information, see issue #46906 <https://github.com/rust-lang/rust/issues/46906>
[01:12:44] 
[01:12:44] thread 'rustc' panicked at 'couldn't compile the test', /checkout/src/librustdoc/test.rs:286:12
[01:12:44] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:12:44] 
[01:12:44] ---- ptr.rs - ptr::*mut T::copy_from_nonoverlapping (line 1866) stdout ----
[01:12:44] 	error: the type of this value must be known in this context
[01:12:44]   --> ptr.rs:10:22
[01:12:44]    |
[01:12:44] 10 |     dst.as_mut_ptr().copy_from_nonoverlapping(ptr, elts);
[01:12:44]    |                      ^^^^^^^^^^^^^^^^^^^^^^^^
[01:12:44]    |
[01:12:44] note: lint level defined here
[01:12:44]   --> ptr.rs:1:9
[01:12:44]    |
[01:12:44] 1  | #![deny(warnings)]
[01:12:44]    |         ^^^^^^^^
[01:12:44]    = note: #[deny(tyvar_behind_raw_pointer)] implied by #[deny(warnings)]
[01:12:44]    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
[01:12:44]    = note: for more information, see issue #46906 <https://github.com/rust-lang/rust/issues/46906>
[01:12:44] 
[01:12:44] thread 'rustc' panicked at 'couldn't compile the test', /checkout/src/librustdoc/test.rs:286:12
[01:12:44] 
[01:12:44] 
[01:12:44] failures:
[01:12:44]     ptr.rs - ptr::*mut T::copy_from (line 1827)
[01:12:44]     ptr.rs - ptr::*mut T::copy_from_nonoverlapping (line 1866)
