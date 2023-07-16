plain
.................................................................................................... 8900/11099
.................................................................................................... 9000/11099
.................................................................................................... 9100/11099
.......i............................................................................................ 9200/11099
......................................iiiiii..iiiiii.i.............................................. 9300/11099
.................................................................................................... 9500/11099
.................................................................................................... 9600/11099
.................................................................................................... 9700/11099
.................................................................................................... 9800/11099
---
Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 229 tests
iiiiiiii......iii...ii..i.ii.........i...........i.............i.............i.i...iii.......iii.... 100/229
......i....i.............i.i.i...iii...iiii....................................ii..i...i..i......... 200/229
....iii.ii...................

 finished in 3.150 seconds
Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 26 tests
iiiiiiiiiiiiiiiiiiiiiiiiii

 finished in 0.143 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii.....ii....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out

 finished in 2.818 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
   Compiling tempfile v3.1.0
   Compiling serde_json v1.0.59
   Compiling lint-docs v0.1.0 (/checkout/src/tools/lint-docs)
    Finished release [optimized] target(s) in 9.35s
error: failed to test example in lint docs for `missing_fragment_specifier` in /checkout/compiler/rustc_lint_defs/src/builtin.rs:1231: lint docs should contain the line `### Example`


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/lint-docs" "--src" "/checkout/compiler" "--out" "/checkout/obj/build/x86_64-unknown-linux-gnu/md-doc/rustc/src/lints" "--rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustc-target" "x86_64-unknown-linux-gnu"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:32:02
