plain
.................................................................................................... 200/3066
.......ii........................................................................................... 300/3066
..........................................................i......................................... 400/3066
.................................................................................................... 500/3066
............................................F......F....iiii........................................ 600/3066
.................................................................................................... 800/3066
.................................................................................................... 900/3066
.................................................................................................... 1000/3066
.................................................................................................... 1100/3066
---
........i.....................i.....................i......................i........................ 3000/3066
..................................................................
failures:

---- src/macros/mod.rs - assert_matches (line 126) stdout ----
error: cannot find macro `assert_matches` in this scope
   |
   |
12 | assert_matches!(c, Ok(x) | Err(x) if x.len() < 100);
   |
   = note: consider importing one of these items:
           core::assert::assert_matches
           std::assert::assert_matches
           std::assert::assert_matches

error: cannot find macro `assert_matches` in this scope
  |
  |
9 | assert_matches!(b, None);
  |
  = note: consider importing one of these items:
          core::assert::assert_matches
          std::assert::assert_matches
          std::assert::assert_matches

error: cannot find macro `assert_matches` in this scope
  |
  |
8 | assert_matches!(a, Some(_));
  |
  = note: consider importing one of these items:
          core::assert::assert_matches
          std::assert::assert_matches
---
---- src/macros/mod.rs - debug_assert_matches (line 283) stdout ----
error: cannot find macro `debug_assert_matches` in this scope
   --> src/macros/mod.rs:292:1
    |
12  | debug_assert_matches!(c, Ok(x) | Err(x) if x.len() < 100);
    | ^^^^^^^^^^^^^^^^^^^^ help: a macro with a similar name exists: `debug_assert_eq`
   ::: /checkout/library/core/src/macros/mod.rs:236:1
    |
236 | macro_rules! debug_assert_eq {
236 | macro_rules! debug_assert_eq {
    | ---------------------------- similarly named macro `debug_assert_eq` defined here
    = note: consider importing one of these items:
            core::assert::debug_assert_matches
            std::assert::debug_assert_matches


error: cannot find macro `debug_assert_matches` in this scope
   --> src/macros/mod.rs:289:1
    |
9   | debug_assert_matches!(b, None);
    | ^^^^^^^^^^^^^^^^^^^^ help: a macro with a similar name exists: `debug_assert_eq`
   ::: /checkout/library/core/src/macros/mod.rs:236:1
    |
236 | macro_rules! debug_assert_eq {
236 | macro_rules! debug_assert_eq {
    | ---------------------------- similarly named macro `debug_assert_eq` defined here
    = note: consider importing one of these items:
            core::assert::debug_assert_matches
            std::assert::debug_assert_matches


error: cannot find macro `debug_assert_matches` in this scope
   --> src/macros/mod.rs:288:1
    |
8   | debug_assert_matches!(a, Some(_));
    | ^^^^^^^^^^^^^^^^^^^^ help: a macro with a similar name exists: `debug_assert_eq`
   ::: /checkout/library/core/src/macros/mod.rs:236:1
    |
236 | macro_rules! debug_assert_eq {
236 | macro_rules! debug_assert_eq {
    | ---------------------------- similarly named macro `debug_assert_eq` defined here
    = note: consider importing one of these items:
            core::assert::debug_assert_matches
            std::assert::debug_assert_matches

---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


Build completed unsuccessfully in 0:17:36
