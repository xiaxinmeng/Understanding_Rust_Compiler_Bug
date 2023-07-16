plain
---- src/collections/vec_deque/mod.rs - collections::vec_deque::VecDeque<T,A>::reserve_exact (line 669) stdout ----
error: expected type, found `1`
 --> src/collections/vec_deque/mod.rs:672:30
  |
6 | let mut buf: VecDeque::from([1]);
  |                              ^ expected type

error[E0214]: parenthesized type parameters may only be used with a `Fn` trait
  |
  |
6 | let mut buf: VecDeque::from([1]);
  |                        |
  |                        |
  |                        only `Fn` traits may use parentheses
  |                        help: use angle brackets instead: `from<[1]>`
error[E0107]: missing generics for struct `VecDeque`
  --> src/collections/vec_deque/mod.rs:672:14
   |
   |
6  | let mut buf: VecDeque::from([1]);
   |              ^^^^^^^^ expected at least 1 generic argument
   |
note: struct defined here, with at least 1 generic parameter: `T`
   |
94 | pub struct VecDeque<
   |            ^^^^^^^^
95 |     T,
95 |     T,
   |     -
help: add missing generic argument
   |
6  | let mut buf: VecDeque<T>::from([1]);

error: aborting due to 3 previous errors

Some errors have detailed explanations: E0107, E0214.
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "alloc" "--" "--quiet"


Build completed unsuccessfully in 0:19:17
