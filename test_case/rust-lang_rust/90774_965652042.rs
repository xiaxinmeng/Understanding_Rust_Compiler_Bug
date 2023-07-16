plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
133 | |     // empty (base case for the recursion)
134 | |     () => {};
135 | |
...   |
142 | |         $crate::__thread_local_inner!($(#[$attr])* $vis $name, $t, const $init);
...   |
154 | |     );
155 | | }
    | |_- in this expansion of `thread_local!` (#1)
    | |_- in this expansion of `thread_local!` (#1)
...
162 |   macro_rules! __thread_local_inner {
    |  _-
    | |_|
    | |
163 | |     // used to generate the `LocalKey` value for const-initialized thread locals
164 | |     (@key $t:ty, const $init:expr) => {{
165 | |         #[cfg_attr(not(windows), inline)] // see comments below
...   |
168 | |             const INIT_EXPR: $t = $init;
...   |
...   |
321 | |             $crate::__thread_local_inner!(@key $t, $($init)*);
322 | |     }
323 | | }
    | | -
    | |_|
    | |_|
    | |_in this expansion of `$crate::__thread_local_inner!` (#2)
    |   in this expansion of `$crate::__thread_local_inner!` (#3)
   ::: library/std/src/thread/local/tests.rs:20:5
    |
    |
20  |       thread_local!(static FOO2: Cell<i32> = const { Cell::new(1) });
    |
    |
    = note: `-D dead-code` implied by `-D warnings`
error: constant is never used: `INIT_EXPR`
   --> library/std/src/thread/local.rs:168:13
    |
132 | / macro_rules! thread_local {
132 | / macro_rules! thread_local {
133 | |     // empty (base case for the recursion)
134 | |     () => {};
135 | |
...   |
142 | |         $crate::__thread_local_inner!($(#[$attr])* $vis $name, $t, const $init);
...   |
154 | |     );
155 | | }
    | |_- in this expansion of `thread_local!` (#1)
    | |_- in this expansion of `thread_local!` (#1)
...
162 |   macro_rules! __thread_local_inner {
    |  _-
    | |_|
    | |
163 | |     // used to generate the `LocalKey` value for const-initialized thread locals
164 | |     (@key $t:ty, const $init:expr) => {{
165 | |         #[cfg_attr(not(windows), inline)] // see comments below
...   |
168 | |             const INIT_EXPR: $t = $init;
...   |
...   |
321 | |             $crate::__thread_local_inner!(@key $t, $($init)*);
322 | |     }
323 | | }
    | | -
    | |_|
    | |_|
    | |_in this expansion of `$crate::__thread_local_inner!` (#2)
    |   in this expansion of `$crate::__thread_local_inner!` (#3)
   ::: library/std/src/thread/local/tests.rs:52:5
    |
    |
52  |       thread_local!(static FOO2: Foo = const { Foo(&FOO2) });

error: constant is never used: `INIT_EXPR`
   --> library/std/src/thread/local.rs:168:13
    |
    |
132 | / macro_rules! thread_local {
133 | |     // empty (base case for the recursion)
134 | |     () => {};
135 | |
...   |
142 | |         $crate::__thread_local_inner!($(#[$attr])* $vis $name, $t, const $init);
...   |
154 | |     );
155 | | }
    | |_- in this expansion of `thread_local!` (#1)
    | |_- in this expansion of `thread_local!` (#1)
...
162 |   macro_rules! __thread_local_inner {
    |  _-
    | |_|
    | |
163 | |     // used to generate the `LocalKey` value for const-initialized thread locals
164 | |     (@key $t:ty, const $init:expr) => {{
165 | |         #[cfg_attr(not(windows), inline)] // see comments below
...   |
168 | |             const INIT_EXPR: $t = $init;
...   |
...   |
321 | |             $crate::__thread_local_inner!(@key $t, $($init)*);
322 | |     }
323 | | }
    | | -
    | |_|
    | |_|
    | |_in this expansion of `$crate::__thread_local_inner!` (#2)
    |   in this expansion of `$crate::__thread_local_inner!` (#3)
   ::: library/std/src/thread/local/tests.rs:68:5
    |
    |
68  |       thread_local!(static FOO2: UnsafeCell<Option<Foo>> = const { UnsafeCell::new(None) });

error: constant is never used: `INIT_EXPR`
   --> library/std/src/thread/local.rs:168:13
    |
    |
132 | / macro_rules! thread_local {
133 | |     // empty (base case for the recursion)
134 | |     () => {};
135 | |
...   |
142 | |         $crate::__thread_local_inner!($(#[$attr])* $vis $name, $t, const $init);
...   |
154 | |     );
155 | | }
    | |_- in this expansion of `thread_local!` (#1)
    | |_- in this expansion of `thread_local!` (#1)
...
162 |   macro_rules! __thread_local_inner {
    |  _-
    | |_|
    | |
163 | |     // used to generate the `LocalKey` value for const-initialized thread locals
164 | |     (@key $t:ty, const $init:expr) => {{
165 | |         #[cfg_attr(not(windows), inline)] // see comments below
...   |
168 | |             const INIT_EXPR: $t = $init;
...   |
...   |
321 | |             $crate::__thread_local_inner!(@key $t, $($init)*);
322 | |     }
323 | | }
    | | -
    | |_|
    | |_|
    | |_in this expansion of `$crate::__thread_local_inner!` (#2)
    |   in this expansion of `$crate::__thread_local_inner!` (#3)
   ::: library/std/src/thread/local/tests.rs:90:5
    |
    |
90  |       thread_local!(static K3: UnsafeCell<Option<S1>> = const { UnsafeCell::new(None) });

error: constant is never used: `INIT_EXPR`
   --> library/std/src/thread/local.rs:168:13
    |
    |
132 | / macro_rules! thread_local {
133 | |     // empty (base case for the recursion)
134 | |     () => {};
135 | |
...   |
142 | |         $crate::__thread_local_inner!($(#[$attr])* $vis $name, $t, const $init);
...   |
154 | |     );
155 | | }
    | |_- in this expansion of `thread_local!` (#1)
    | |_- in this expansion of `thread_local!` (#1)
...
162 |   macro_rules! __thread_local_inner {
    |  _-
    | |_|
    | |
163 | |     // used to generate the `LocalKey` value for const-initialized thread locals
164 | |     (@key $t:ty, const $init:expr) => {{
165 | |         #[cfg_attr(not(windows), inline)] // see comments below
...   |
168 | |             const INIT_EXPR: $t = $init;
...   |
...   |
321 | |             $crate::__thread_local_inner!(@key $t, $($init)*);
322 | |     }
323 | | }
    | | -
    | |_|
    | |_|
    | |_in this expansion of `$crate::__thread_local_inner!` (#2)
    |   in this expansion of `$crate::__thread_local_inner!` (#3)
   ::: library/std/src/thread/local/tests.rs:91:5
    |
    |
91  |       thread_local!(static K4: UnsafeCell<Option<S2>> = const { UnsafeCell::new(None) });

error: constant is never used: `INIT_EXPR`
   --> library/std/src/thread/local.rs:168:13
    |
    |
132 | / macro_rules! thread_local {
133 | |     // empty (base case for the recursion)
134 | |     () => {};
135 | |
...   |
142 | |         $crate::__thread_local_inner!($(#[$attr])* $vis $name, $t, const $init);
...   |
154 | |     );
155 | | }
    | |_- in this expansion of `thread_local!` (#1)
    | |_- in this expansion of `thread_local!` (#1)
...
162 |   macro_rules! __thread_local_inner {
    |  _-
    | |_|
    | |
163 | |     // used to generate the `LocalKey` value for const-initialized thread locals
164 | |     (@key $t:ty, const $init:expr) => {{
165 | |         #[cfg_attr(not(windows), inline)] // see comments below
...   |
168 | |             const INIT_EXPR: $t = $init;
...   |
...   |
321 | |             $crate::__thread_local_inner!(@key $t, $($init)*);
322 | |     }
323 | | }
    | | -
    | |_|
    | |_|
    | |_in this expansion of `$crate::__thread_local_inner!` (#2)
    |   in this expansion of `$crate::__thread_local_inner!` (#3)
   ::: library/std/src/thread/local/tests.rs:143:5
    |
    |
143 |       thread_local!(static K2: UnsafeCell<Option<S1>> = const { UnsafeCell::new(None) });

error: constant is never used: `INIT_EXPR`
   --> library/std/src/thread/local.rs:168:13
    |
    |
132 | / macro_rules! thread_local {
133 | |     // empty (base case for the recursion)
134 | |     () => {};
135 | |
...   |
142 | |         $crate::__thread_local_inner!($(#[$attr])* $vis $name, $t, const $init);
...   |
154 | |     );
155 | | }
    | |_- in this expansion of `thread_local!` (#1)
    | |_- in this expansion of `thread_local!` (#1)
...
162 |   macro_rules! __thread_local_inner {
    |  _-
    | |_|
    | |
163 | |     // used to generate the `LocalKey` value for const-initialized thread locals
164 | |     (@key $t:ty, const $init:expr) => {{
165 | |         #[cfg_attr(not(windows), inline)] // see comments below
...   |
168 | |             const INIT_EXPR: $t = $init;
...   |
...   |
321 | |             $crate::__thread_local_inner!(@key $t, $($init)*);
322 | |     }
323 | | }
    | | -
    | |_|
    | |_|
    | |_in this expansion of `$crate::__thread_local_inner!` (#2)
    |   in this expansion of `$crate::__thread_local_inner!` (#3)
   ::: library/std/src/thread/local/tests.rs:192:5
    |
    |
192 |       thread_local!(static K1: UnsafeCell<Option<S1>> = const { UnsafeCell::new(None) });

error: constant is never used: `INIT_EXPR`
   --> library/std/src/thread/local.rs:168:13
    |
    |
132 | / macro_rules! thread_local {
133 | |     // empty (base case for the recursion)
134 | |     () => {};
135 | |
...   |
142 | |         $crate::__thread_local_inner!($(#[$attr])* $vis $name, $t, const $init);
...   |
154 | |     );
155 | | }
    | |_- in this expansion of `thread_local!` (#1)
    | |_- in this expansion of `thread_local!` (#1)
...
162 |   macro_rules! __thread_local_inner {
    |  _-
    | |_|
    | |
163 | |     // used to generate the `LocalKey` value for const-initialized thread locals
164 | |     (@key $t:ty, const $init:expr) => {{
165 | |         #[cfg_attr(not(windows), inline)] // see comments below
...   |
168 | |             const INIT_EXPR: $t = $init;
...   |
...   |
321 | |             $crate::__thread_local_inner!(@key $t, $($init)*);
322 | |     }
323 | | }
    | | -
    | |_|
    | |_|
    | |_in this expansion of `$crate::__thread_local_inner!` (#2)
    |   in this expansion of `$crate::__thread_local_inner!` (#3)
   ::: library/std/src/thread/local/tests.rs:193:5
    |
    |
193 |       thread_local!(static K2: UnsafeCell<Option<Foo>> = const { UnsafeCell::new(None) });

error: could not compile `std` due to 8 previous errors
Build completed unsuccessfully in 0:00:34
