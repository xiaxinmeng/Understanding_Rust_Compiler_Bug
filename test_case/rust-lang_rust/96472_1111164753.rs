
170 | |     );
171 | | }
    | |_- in this expansion of `thread_local!` (#1)
...
178 |   macro_rules! __thread_local_inner {
    |  _-
    | |_|
    | |
179 | |     // used to generate the `LocalKey` value for const-initialized thread locals
180 | |     (@key $t:ty, const $init:expr) => {{
181 | |         #[cfg_attr(not(windows), inline)] // see comments below
...   |
259 | |                 not(all(target_family = "wasm", not(target_feature = "atomics"))),
    | |                                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^
...   |
363 | |             $crate::__thread_local_inner!(@key $t, $($init)*);
    | |             ------------------------------------------------- in this macro invocation (#3)
364 | |     }
365 | | }
    | | -
    | |_|
    | |_in this expansion of `$crate::__thread_local_inner!` (#2)
    |   in this expansion of `$crate::__thread_local_inner!` (#3)
    |
   ::: library/std/src/sys_common/thread_info.rs:13:1
    |
13  |   thread_local! { static THREAD_INFO: RefCell<Option<ThreadInfo>> = const { RefCell::new(None) } }
    |   ------------------------------------------------------------------------------------------------ in this macro invocation (#1)

warning: unexpected `cfg` condition name
 --> library/std/src/sys_common/thread_parker/mod.rs:5:37
  |
5 |         all(target_arch = "wasm32", target_feature = "atomics"),
  |                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unexpected `cfg` condition name
   --> library/std/src/thread/local.rs:319:55
    |
148 | / macro_rules! thread_local {
149 | |     // empty (base case for the recursion)
150 | |     () => {};
151 | |
...   |
169 | |         $crate::__thread_local_inner!($(#[$attr])* $vis $name, $t, $init);
    | |         ----------------------------------------------------------------- in this macro invocation (#2)
170 | |     );
171 | | }
    | |_- in this expansion of `thread_local!` (#1)
...
178 |   macro_rules! __thread_local_inner {
    |  _-
    | |_|
    | |
179 | |     // used to generate the `LocalKey` value for const-initialized thread locals
180 | |     (@key $t:ty, const $init:expr) => {{
181 | |         #[cfg_attr(not(windows), inline)] // see comments below
...   |
319 | |                 #[cfg(all(target_family = "wasm", not(target_feature = "atomics")))]
    | |                                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^
...   |
363 | |             $crate::__thread_local_inner!(@key $t, $($init)*);
    | |             ------------------------------------------------- in this macro invocation (#3)
364 | |     }
365 | | }
    | | -
    | |_|
    | |_in this expansion of `$crate::__thread_local_inner!` (#2)
    |   in this expansion of `$crate::__thread_local_inner!` (#3)
    |
   ::: library/std/src/panicking.rs:328:5
    |
328 |       thread_local! { static LOCAL_PANIC_COUNT: Cell<usize> = Cell::new(0) }
    |       ---------------------------------------------------------------------- in this macro invocation (#1)

warning: unexpected `cfg` condition name
   --> library/std/src/thread/local.rs:326:57
    |
148 | / macro_rules! thread_local {
149 | |     // empty (base case for the recursion)
150 | |     () => {};
151 | |
...   |
169 | |         $crate::__thread_local_inner!($(#[$attr])* $vis $name, $t, $init);
    | |         ----------------------------------------------------------------- in this macro invocation (#2)
170 | |     );
171 | | }
    | |_- in this expansion of `thread_local!` (#1)
...
178 |   macro_rules! __thread_local_inner {
    |  _-
    | |_|
    | |
179 | |     // used to generate the `LocalKey` value for const-initialized thread locals
180 | |     (@key $t:ty, const $init:expr) => {{
181 | |         #[cfg_attr(not(windows), inline)] // see comments below
...   |
326 | |                     not(all(target_family = "wasm", not(target_feature = "atomics"))),
    | |                                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^
...   |
363 | |             $crate::__thread_local_inner!(@key $t, $($init)*);
    | |             ------------------------------------------------- in this macro invocation (#3)
364 | |     }
365 | | }
    | | -
    | |_|
    | |_in this expansion of `$crate::__thread_local_inner!` (#2)
    |   in this expansion of `$crate::__thread_local_inner!` (#3)
    |
   ::: library/std/src/panicking.rs:328:5
    |
328 |       thread_local! { static LOCAL_PANIC_COUNT: Cell<usize> = Cell::new(0) }
    |       ---------------------------------------------------------------------- in this macro invocation (#1)

warning: unexpected `cfg` condition name
   --> library/std/src/thread/local.rs:333:57
    |
148 | / macro_rules! thread_local {
149 | |     // empty (base case for the recursion)
150 | |     () => {};
151 | |
...   |
169 | |         $crate::__thread_local_inner!($(#[$attr])* $vis $name, $t, $init);
    | |         ----------------------------------------------------------------- in this macro invocation (#2)
170 | |     );
171 | | }
    | |_- in this expansion of `thread_local!` (#1)
...
178 |   macro_rules! __thread_local_inner {
    |  _-
    | |_|
    | |
179 | |     // used to generate the `LocalKey` value for const-initialized thread locals
180 | |     (@key $t:ty, const $init:expr) => {{
181 | |         #[cfg_attr(not(windows), inline)] // see comments below
...   |
333 | |                     not(all(target_family = "wasm", not(target_feature = "atomics"))),
    | |                                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^
...   |
363 | |             $crate::__thread_local_inner!(@key $t, $($init)*);
    | |             ------------------------------------------------- in this macro invocation (#3)
364 | |     }
365 | | }
    | | -
    | |_|
    | |_in this expansion of `$crate::__thread_local_inner!` (#2)
    |   in this expansion of `$crate::__thread_local_inner!` (#3)
    |
   ::: library/std/src/panicking.rs:328:5
    |
328 |       thread_local! { static LOCAL_PANIC_COUNT: Cell<usize> = Cell::new(0) }
    |       ---------------------------------------------------------------------- in this macro invocation (#1)
