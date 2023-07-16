plain
.................................................................................................... 9300/11526
.................................................................................................... 9400/11526
...........................................................................i......i................. 9500/11526
.................................................................................................... 9600/11526
..............iiiiiii..iiiiii.i..................................................................... 9700/11526
.................................................................................................... 9900/11526
.................................................................................................... 10000/11526
.................................................................................................... 10100/11526
.................................................................................................... 10200/11526
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 29 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii

Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
 finished in 0.067 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i...i.i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.56s

 finished in 2.637 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
.................................................................................................... 200/2842
..ii................................................................................................ 300/2842
...................................................i................................................ 400/2842
.................................................................................................... 500/2842
........................i..i...........F.....F.iiii................................................. 600/2842
.................................................................................................... 800/2842
.................................................................................................... 900/2842
.................................................................................................... 1000/2842
.................................................................................................... 1100/2842
---
---- src/macros/mod.rs - assert_matches (line 126) stdout ----
error[E0658]: use of unstable library feature 'assert_matches'
 --> src/macros/mod.rs:129:1
  |
6 | assert_matches!(a, Some(_));
  |
  |
  = help: add `#![feature(assert_matches)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'assert_matches'
 --> src/macros/mod.rs:130:1
  |
  |
7 | assert_matches!(b, None);
  |
  |
  = help: add `#![feature(assert_matches)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'assert_matches'
  --> src/macros/mod.rs:133:1
   |
   |
10 | assert_matches!(a, Ok(x) | Err(x) if x.len() < 100);
   |
   |
   = help: add `#![feature(assert_matches)]` to the crate attributes to enable
error[E0308]: mismatched types
  --> src/macros/mod.rs:133:20
   |
   |
10 | assert_matches!(a, Ok(x) | Err(x) if x.len() < 100);
   |                 -  ^^^^^ expected enum `Option`, found enum `Result`
   |                 this expression has type `Option<u32>`
   |
   = note: expected enum `Option<u32>`
              found enum `Result<_, _>`
              found enum `Result<_, _>`

error[E0308]: mismatched types
  --> src/macros/mod.rs:133:28
   |
10 | assert_matches!(a, Ok(x) | Err(x) if x.len() < 100);
   |                 -          ^^^^^^ expected enum `Option`, found enum `Result`
   |                 this expression has type `Option<u32>`
   |
   = note: expected enum `Option<u32>`
              found enum `Result<_, _>`
---
---- src/macros/mod.rs - debug_assert_matches (line 281) stdout ----
error[E0658]: use of unstable library feature 'assert_matches'
 --> src/macros/mod.rs:284:1
  |
6 | debug_assert_matches!(a, Some(_));
  |
  |
  = help: add `#![feature(assert_matches)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'assert_matches'
 --> src/macros/mod.rs:285:1
  |
  |
7 | debug_assert_matches!(b, None);
  |
  |
  = help: add `#![feature(assert_matches)]` to the crate attributes to enable
error[E0658]: use of unstable library feature 'assert_matches'
  --> src/macros/mod.rs:288:1
   |
   |
10 | debug_assert_matches!(a, Ok(x) | Err(x) if x.len() < 100);
   |
   |
   = help: add `#![feature(assert_matches)]` to the crate attributes to enable
error[E0308]: mismatched types
  --> src/macros/mod.rs:288:26
   |
   |
10 | debug_assert_matches!(a, Ok(x) | Err(x) if x.len() < 100);
   |                       -  ^^^^^ expected enum `Option`, found enum `Result`
   |                       this expression has type `Option<u32>`
   |
   = note: expected enum `Option<u32>`
              found enum `Result<_, _>`
              found enum `Result<_, _>`

error[E0308]: mismatched types
  --> src/macros/mod.rs:288:34
   |
10 | debug_assert_matches!(a, Ok(x) | Err(x) if x.len() < 100);
   |                       -          ^^^^^^ expected enum `Option`, found enum `Result`
   |                       this expression has type `Option<u32>`
   |
   = note: expected enum `Option<u32>`
              found enum `Result<_, _>`
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:20:57
