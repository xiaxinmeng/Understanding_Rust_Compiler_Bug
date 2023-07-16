plain
[RUSTC-TIMING] object test:false 4.681
[RUSTC-TIMING] gimli test:false 4.860
warning: dropping unsupported crate type `dylib` for target `wasm32-unknown-unknown`

error: use of mutable static is unsafe and requires unsafe block (error E0133)
    |
146 | / macro_rules! thread_local {
147 | |     // empty (base case for the recursion)
148 | |     () => {};
148 | |     () => {};
149 | |
...   |
156 | |         $crate::__thread_local_inner!($(#[$attr])* $vis $name, $t, const $init);
...   |
168 | |     );
169 | | }
    | |_- in this expansion of `thread_local!` (#1)
    | |_- in this expansion of `thread_local!` (#1)
...
176 |   macro_rules! __thread_local_inner {
    |  _-
    | |_|
    | |
177 | |     // used to generate the `LocalKey` value for const-initialized thread locals
178 | |     (@key $t:ty, const $init:expr) => {{
179 | |         #[cfg_attr(not(windows), inline)] // see comments below
192 | |                 Some(&VAL)
    | |                      ^^^^ use of mutable static
...   |
...   |
334 | |             $crate::__thread_local_inner!(@key $t, $($init)*);
335 | |     }
336 | | }
    | | -
    | |_|
    | |_|
    | |_in this expansion of `$crate::__thread_local_inner!` (#2)
    |   in this expansion of `$crate::__thread_local_inner!` (#3)
   ::: library/std/src/panicking.rs:242:5
    |
    |
242 |       thread_local! { static LOCAL_PANIC_COUNT: Cell<usize> = const { Cell::new(0) } }
    |
note: the lint level is defined here
   --> library/std/src/panicking.rs:10:9
    |
    |
10  | #![deny(unsafe_op_in_unsafe_fn)]
    |         ^^^^^^^^^^^^^^^^^^^^^^
    = note: mutable statics can be mutated by multiple threads: aliasing violations or data races will cause undefined behavior
[RUSTC-TIMING] std test:false 2.472
warning: `std` (lib) generated 1 warning
error: could not compile `std` due to previous error; 1 warning emitted
Build completed unsuccessfully in 0:13:29
