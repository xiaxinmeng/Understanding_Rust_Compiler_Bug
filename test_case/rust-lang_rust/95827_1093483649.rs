plain
[RUSTC-TIMING] gimli test:false 4.406
[RUSTC-TIMING] object test:false 4.421
warning: dropping unsupported crate type `dylib` for target `wasm32-unknown-unknown`

error: use of mutable static is unsafe and requires unsafe block (error E0133)
    |
148 | / macro_rules! thread_local {
149 | |     // empty (base case for the recursion)
150 | |     () => {};
150 | |     () => {};
151 | |
...   |
158 | |         $crate::__thread_local_inner!($(#[$attr])* $vis $name, $t, const $init);
...   |
170 | |     );
171 | | }
    | |_- in this expansion of `thread_local!` (#1)
    | |_- in this expansion of `thread_local!` (#1)
...
178 |   macro_rules! __thread_local_inner {
    |  _-
    | |_|
    | |
179 | |     // used to generate the `LocalKey` value for const-initialized thread locals
180 | |     (@key $t:ty, const $init:expr) => {{
181 | |         #[cfg_attr(not(windows), inline(always))] // see comments below
...   |
197 | |                 $crate::option::Option::Some(&VAL)
    | |                                              ^^^^ use of mutable static
...   |
363 | |             $crate::__thread_local_inner!(@key $t, $($init)*);
364 | |     }
365 | | }
    | | -
    | |_|
    | |_|
    | |_in this expansion of `$crate::__thread_local_inner!` (#2)
    |   in this expansion of `$crate::__thread_local_inner!` (#3)
   ::: library/std/src/sys_common/thread_info.rs:13:1
    |
    |
13  |   thread_local! { static THREAD_INFO: RefCell<Option<ThreadInfo>> = const { RefCell::new(None) } }
    |
note: the lint level is defined here
   --> library/std/src/thread/local.rs:182:16
    |
    |
148 | / macro_rules! thread_local {
149 | |     // empty (base case for the recursion)
150 | |     () => {};
151 | |
...   |
158 | |         $crate::__thread_local_inner!($(#[$attr])* $vis $name, $t, const $init);
...   |
170 | |     );
171 | | }
    | |_- in this expansion of `thread_local!` (#1)
    | |_- in this expansion of `thread_local!` (#1)
...
178 |   macro_rules! __thread_local_inner {
    |  _-
    | |_|
    | |
179 | |     // used to generate the `LocalKey` value for const-initialized thread locals
180 | |     (@key $t:ty, const $init:expr) => {{
181 | |         #[cfg_attr(not(windows), inline(always))] // see comments below
182 | |         #[deny(unsafe_op_in_unsafe_fn)]
...   |
...   |
363 | |             $crate::__thread_local_inner!(@key $t, $($init)*);
364 | |     }
365 | | }
    | | -
    | |_|
    | |_|
    | |_in this expansion of `$crate::__thread_local_inner!` (#2)
    |   in this expansion of `$crate::__thread_local_inner!` (#3)
   ::: library/std/src/sys_common/thread_info.rs:13:1
    |
    |
13  |   thread_local! { static THREAD_INFO: RefCell<Option<ThreadInfo>> = const { RefCell::new(None) } }
    |   ------------------------------------------------------------------------------------------------ in this macro invocation (#1)
    = note: mutable statics can be mutated by multiple threads: aliasing violations or data races will cause undefined behavior
[RUSTC-TIMING] std test:false 2.655
warning: `std` (lib) generated 1 warning
error: could not compile `std` due to previous error; 1 warning emitted
Build completed unsuccessfully in 0:10:51
