plain
.................................................................................................... 9200/11497
.................................................................................................... 9300/11497
.................................................................................................... 9400/11497
..................................................i......i.......................................... 9500/11497
.........................................................................................iiiiiii..ii 9600/11497
.................................................................................................... 9800/11497
.................................................................................................... 9900/11497
.................................................................................................... 10000/11497
.................................................................................................... 10100/11497
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 29 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii

Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
 finished in 0.073 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.36s

 finished in 2.438 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
.................................................................................................... 1800/2840
.................................................................................................... 1900/2840
.................................................................................................... 2000/2840
.................................................................................................... 2100/2840
.........................................................................F......F................... 2200/2840
.................................................................................................... 2400/2840
....................................................................................i............... 2500/2840
................................i................i.....................i.....................i...... 2600/2840
...............i.....................i................................i.....................i....... 2700/2840
...............i.....................i................................i.....................i....... 2700/2840
..............i.....................i.....................i......................................... 2800/2840
........................................
failures:

---- src/ptr/mod.rs - ptr::read_unaligned (line 762) stdout ----
error: unknown lint: `unaligned_reads`
   |
   |
15 | #[allow(unaligned_reads)]
   |
note: the lint level is defined here
  --> src/ptr/mod.rs:760:9
   |
   |
1  | #![deny(warnings)]
   |         ^^^^^^^^
   = note: `#[deny(unknown_lints)]` implied by `#[deny(warnings)]`

error: unknown lint: `unaligned_reads`
   |
   |
15 | #[allow(unaligned_reads)]

error: aborting due to 2 previous errors

Couldn't compile the test.
Couldn't compile the test.
---- src/ptr/mod.rs - ptr::write_unaligned (line 958) stdout ----
error: unknown lint: `unaligned_reads`
   |
   |
13 | #[allow(unaligned_reads)]
   |
note: the lint level is defined here
  --> src/ptr/mod.rs:956:9
   |
   |
1  | #![deny(warnings)]
   |         ^^^^^^^^
   = note: `#[deny(unknown_lints)]` implied by `#[deny(warnings)]`

error: unknown lint: `unaligned_reads`
   |
   |
13 | #[allow(unaligned_reads)]

error: aborting due to 2 previous errors

Couldn't compile the test.
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:18:29
