
error[E0433]: failed to resolve: could not find `__OsLocalKeyInner` in `thread`
 --> src/lib.rs:3:1
  |
3 | thread_local!(static LOCK_HELD: Cell<bool> = Cell::new(false));
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ could not find `__OsLocalKeyInner` in `thread`
  |
  = note: this error originates in the macro `$crate::__thread_local_inner` which comes from the expansion of the macro `thread_local` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0412]: cannot find type `__OsLocalKeyInner` in module `$crate::thread`
   --> src/lib.rs:3:1
    |
3   | thread_local!(static LOCK_HELD: Cell<bool> = Cell::new(false));
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: a struct with a similar name exists: `__FastLocalKeyInner`
    |
   ::: /Users/sumukhgovindaraju/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/std/src/thread/local.rs:924:5
    |
924 |     pub struct Key<T> {
    |     ----------------- similarly named struct `__FastLocalKeyInner` defined here
    |
    = note: this error originates in the macro `$crate::__thread_local_inner` which comes from the expansion of the macro `thread_local` (in Nightly builds, run with -Z macro-backtrace for more info)

#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use ::std::prelude::rust_2015::*;
#[macro_use]
extern crate std;
use std::cell::Cell;


const LOCK_HELD: ::std::thread::LocalKey<Cell<bool>> =
    {
        #[inline]
        fn __init() -> Cell<bool> { Cell::new(false) }
        #[inline]
        unsafe fn __getit(init:
                ::std::option::Option<&mut ::std::option::Option<Cell<bool>>>)
            -> ::std::option::Option<&'static Cell<bool>> {
            #[cfg(all(not(target_thread_local),
            not(all(target_family = "wasm",
            not(target_feature = "atomics"))),))]
            static __KEY: ::std::thread::__OsLocalKeyInner<Cell<bool>> =
                ::std::thread::__OsLocalKeyInner::new();

            #[allow(unused_unsafe)]
            unsafe {
                __KEY.get(move ||
                        {
                            if let ::std::option::Option::Some(init) = init {
                                    if let ::std::option::Option::Some(value) = init.take() {
                                            return value;
                                        } else if true {
                                           ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(&["internal error: entered unreachable code: "],
                                                   &[::core::fmt::ArgumentV1::new_display(&::core::fmt::Arguments::new_v1(&["missing default value"],
                                                                           &[]))]));
                                       }
                                }
                            __init()
                        })
            }
        }
        unsafe { ::std::thread::LocalKey::new(__getit) }
    };
fn main() {}
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0412, E0433.
For more information about an error, try `rustc --explain E0412`.
