plain
.................................................................................................... 9000/11242
.................................................................................................... 9100/11242
.................................................................................................... 9200/11242
......................................i......i...................................................... 9300/11242
.............................................................................iiiiii..iiiiii.i....... 9400/11242
.................................................................................................... 9600/11242
.................................................................................................... 9700/11242
.................................................................................................... 9800/11242
.................................................................................................... 9900/11242
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 27 tests
iiiiiiiiiiiiiiiiiiiiiiiiiii

Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
 finished in 0.087 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
 finished in 11.790 seconds
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i..i...ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.53s

 finished in 2.619 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
running 80 tests
i.iF...i......i...........ii......................i.............i...............
failures:

---- src/builtin.rs - builtin::DISJOINT_CAPTURE_DROP_REORDER (line 2953) stdout ----
error: Drop order affected for closure because of `capture_disjoint_fields`
   |
17 |     let c = || {
   |  ___________^
18 | |      let x = p.x;
18 | |      let x = p.x;
19 | |   };
   | |___^
   |
note: the lint level is defined here
  --> src/builtin.rs:2954:9
   |
2  | #![deny(disjoint_capture_drop_reorder)]
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: let (p) = (p);
error: aborting due to previous error

Couldn't compile the test.

---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc_lint_defs" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:27:10
