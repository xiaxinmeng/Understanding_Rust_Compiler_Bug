plain
.................................................................................................... 200/3076
.......ii........................................................................................... 300/3076
..........................................................i......................................... 400/3076
.................................................................................................... 500/3076
.........................................F..........F...iiii........................................ 600/3076
.................................................................................................... 800/3076
.................................................................................................... 900/3076
.................................................................................................... 1000/3076
.................................................................................................... 1100/3076
---
---- src/macros/mod.rs - assert_matches (line 127) stdout ----
error[E0432]: unresolved import `std::assert`
 --> src/macros/mod.rs:130:10
  |
6 | use std::assert::assert_matches;
  |          ^^^^^^ could not find `assert` in `std`

error: cannot determine resolution for the macro `assert_matches`
   |
   |
14 | assert_matches!(c, Ok(x) | Err(x) if x.len() < 100);
   |
   |
   = note: import resolution is stuck, try simplifying macro imports

error: cannot determine resolution for the macro `assert_matches`
   |
   |
11 | assert_matches!(b, None);
   |
   |
   = note: import resolution is stuck, try simplifying macro imports

error: cannot determine resolution for the macro `assert_matches`
   |
   |
10 | assert_matches!(a, Some(_));
   |
   |
   = note: import resolution is stuck, try simplifying macro imports
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0432`.
Couldn't compile the test.
Couldn't compile the test.
---- src/macros/mod.rs - debug_assert_matches (line 286) stdout ----
error[E0432]: unresolved import `std::assert`
 --> src/macros/mod.rs:289:10
  |
6 | use std::assert::debug_assert_matches;
  |          ^^^^^^ could not find `assert` in `std`
error: cannot determine resolution for the macro `debug_assert_matches`
  --> src/macros/mod.rs:297:1
   |
   |
14 | debug_assert_matches!(c, Ok(x) | Err(x) if x.len() < 100);
   |
   |
   = note: import resolution is stuck, try simplifying macro imports
error: cannot determine resolution for the macro `debug_assert_matches`
  --> src/macros/mod.rs:294:1
   |
   |
11 | debug_assert_matches!(b, None);
   |
   |
   = note: import resolution is stuck, try simplifying macro imports
error: cannot determine resolution for the macro `debug_assert_matches`
  --> src/macros/mod.rs:293:1
   |
   |
10 | debug_assert_matches!(a, Some(_));
   |
   |
   = note: import resolution is stuck, try simplifying macro imports
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0432`.
Couldn't compile the test.
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


Build completed unsuccessfully in 0:19:17
