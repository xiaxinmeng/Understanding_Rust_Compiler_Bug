plain

running 1404 tests
.................................................................................................... 100/1404
.................................................................................................... 200/1404
.....................................F.............................................................. 300/1404
...............................F.F.................................................................. 400/1404
...........................................................ii....................................... 600/1404
.................................................................................................... 700/1404
.................................................................................................... 800/1404
.................................................................................................... 900/1404
---
.................................................................................................... 1400/1404
....
failures:

---- iter::adapters::cloned::test_cloned_side_effects stdout ----
thread 'iter::adapters::cloned::test_cloned_side_effects' panicked at 'assertion failed: `(left == right)`
 right: `2`', library/core/tests/iter/adapters/cloned.rs:34:5

---- iter::adapters::zip::test_zip_cloned_sideffectful stdout ----
---- iter::adapters::zip::test_zip_cloned_sideffectful stdout ----
thread 'iter::adapters::zip::test_zip_cloned_sideffectful' panicked at 'assertion failed: `(left == right)`
  left: `[CountClone(Cell { value: 1 }), CountClone(Cell { value: 1 }), CountClone(Cell { value: 0 }), CountClone(Cell { value: 0 })]`,
 right: `[1, 1, 1, 0]`', library/core/tests/iter/adapters/zip.rs:122:5
---- iter::adapters::zip::test_zip_map_sideffectful stdout ----
---- iter::adapters::zip::test_zip_map_sideffectful stdout ----
thread 'iter::adapters::zip::test_zip_map_sideffectful' panicked at 'assertion failed: `(left == right)`
  left: `[1, 1, 1, 1, 0, 0]`,
 right: `[1, 1, 1, 1, 1, 0]`', library/core/tests/iter/adapters/zip.rs:141:5
error: test failed, to rerun pass '-p core --test coretests'

failures:
    iter::adapters::cloned::test_cloned_side_effects
    iter::adapters::zip::test_zip_cloned_sideffectful
    iter::adapters::zip::test_zip_cloned_sideffectful
    iter::adapters::zip::test_zip_map_sideffectful

test result: FAILED. 1399 passed; 3 failed; 2 ignored; 0 measured; 0 filtered out; finished in 1.69s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


Build completed unsuccessfully in 0:17:20
