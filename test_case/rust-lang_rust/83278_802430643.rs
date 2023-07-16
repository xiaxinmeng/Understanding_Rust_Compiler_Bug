plain
.................................................................................................... 9300/11700
.................................................................................................... 9400/11700
.................................................................................................... 9500/11700
...........................................i......i................................................. 9600/11700
.........................................................................................iiiiiii..ii 9700/11700
.................................................................................................... 9900/11700
.................................................................................................... 10000/11700
.................................................................................................... 10100/11700
.................................................................................................... 10200/11700
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 31 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii..

 finished in 0.104 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.25s

 finished in 2.310 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
running 49 tests
.................................................
test result: ok. 49 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/ppc.rs (build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/ppc-d262d2a95dbbebd4)
running 19 tests
...................
test result: ok. 19 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

---
running 5 tests
.....
test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/json.rs (build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/json-f94db72993a2f5be)
running 70 tests
......................................................................
test result: ok. 70 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

---
Building stage0 tool linkchecker (x86_64-unknown-linux-gnu)
   Compiling once_cell v1.7.2
   Compiling linkchecker v0.1.0 (/checkout/src/tools/linkchecker)
    Finished release [optimized] target(s) in 1.63s
std/cell/index.html:18: broken link - core/sync/atomic/index.html
edition-guide/print.html:544: broken link - core/index.html
edition-guide/rust-2018/module-system/path-clarity.html:232: broken link - core/index.html
nomicon/unchecked-uninit.html:178: broken link - core/mem/union.MaybeUninit.html
nomicon/unchecked-uninit.html:222: broken link - core/mem/union.MaybeUninit.html
nomicon/unchecked-uninit.html:258: broken link - core/ptr/index.html
nomicon/unchecked-uninit.html:260: broken link - core/ptr/fn.write.html
nomicon/panic-handler.html:173: broken link - core/panic/struct.PanicInfo.html
nomicon/print.html:3192: broken link - core/mem/union.MaybeUninit.html
nomicon/print.html:3236: broken link - core/mem/union.MaybeUninit.html
nomicon/print.html:3272: broken link - core/ptr/index.html
nomicon/print.html:3274: broken link - core/ptr/fn.write.html
nomicon/print.html:6780: broken link - core/panic/struct.PanicInfo.html
reference/runtime.html:177: broken link - core/panic/struct.PanicInfo.html
reference/behavior-considered-undefined.html:241: broken link - core/ptr/struct.NonNull.html
reference/behavior-considered-undefined.html:241: broken link - core/num/index.html
reference/print.html:5733: broken link - std/macro.is_x86_feature_detected.html
reference/print.html:5751: broken link - core/panic/struct.Location.html
reference/print.html:5754: broken link - core/panic/struct.Location.html
reference/print.html:5765: broken link - core/panic/struct.Location.html
reference/print.html:5766: broken link - core/intrinsics/fn.caller_location.html
reference/print.html:11752: broken link - core/prelude/index.html
reference/print.html:11757: broken link - core/index.html
reference/print.html:11789: broken link - core/index.html
reference/print.html:11796: broken link - core/prelude/index.html
reference/print.html:11798: broken link - core/index.html
reference/print.html:12758: broken link - core/ptr/struct.NonNull.html
reference/print.html:12758: broken link - core/num/index.html
reference/print.html:13019: broken link - core/panic/struct.PanicInfo.html
reference/attributes/codegen.html:265: broken link - std/macro.is_x86_feature_detected.html
reference/attributes/codegen.html:283: broken link - core/panic/struct.Location.html
reference/attributes/codegen.html:286: broken link - core/panic/struct.Location.html
reference/attributes/codegen.html:297: broken link - core/panic/struct.Location.html
reference/attributes/codegen.html:298: broken link - core/intrinsics/fn.caller_location.html
reference/names/preludes.html:190: broken link - core/prelude/index.html
reference/names/preludes.html:195: broken link - core/index.html
reference/names/preludes.html:227: broken link - core/index.html
reference/names/preludes.html:234: broken link - core/prelude/index.html
reference/names/preludes.html:236: broken link - core/index.html
thread 'main' panicked at 'found some broken links', src/tools/linkchecker/main.rs:97:9


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
expected success, got: exit code: 101
