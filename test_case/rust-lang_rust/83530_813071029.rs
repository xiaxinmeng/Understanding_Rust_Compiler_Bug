plain
.................................................................................................... 9400/11734
.................................................................................................... 9500/11734
...........................................................................i......i................. 9600/11734
.................................................................................................... 9700/11734
.....................iiiiiii..iiiiii.i.............................................................. 9800/11734
.................................................................................................... 10000/11734
.................................................................................................... 10100/11734
.................................................................................................... 10200/11734
.................................................................................................... 10300/11734
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 31 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii..
test result: ok. 2 passed; 0 failed; 29 ignored; 0 measured; 0 filtered out; finished in 0.04s

 finished in 0.105 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.33s

 finished in 2.393 seconds
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

     Running tests/json.rs (build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/json-c81972d45b6c6baa)
running 70 tests
......................................................................
test result: ok. 70 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

---
Building stage0 tool linkchecker (x86_64-unknown-linux-gnu)
   Compiling once_cell v1.7.2
   Compiling linkchecker v0.1.0 (/checkout/src/tools/linkchecker)
    Finished release [optimized] target(s) in 1.69s
edition-guide/print.html:544: broken link - core/index.html
edition-guide/rust-2018/module-system/path-clarity.html:232: broken link - core/index.html
nomicon/print.html:3192: broken link - core/mem/union.MaybeUninit.html
nomicon/print.html:3236: broken link - core/mem/union.MaybeUninit.html
nomicon/print.html:3272: broken link - core/ptr/index.html
nomicon/print.html:3274: broken link - core/ptr/fn.write.html
nomicon/print.html:6780: broken link - core/panic/struct.PanicInfo.html
nomicon/panic-handler.html:173: broken link - core/panic/struct.PanicInfo.html
nomicon/unchecked-uninit.html:178: broken link - core/mem/union.MaybeUninit.html
nomicon/unchecked-uninit.html:222: broken link - core/mem/union.MaybeUninit.html
nomicon/unchecked-uninit.html:258: broken link - core/ptr/index.html
nomicon/unchecked-uninit.html:260: broken link - core/ptr/fn.write.html
reference/print.html:5751: broken link - core/panic/struct.Location.html
reference/print.html:5754: broken link - core/panic/struct.Location.html
reference/print.html:5765: broken link - core/panic/struct.Location.html
reference/print.html:5766: broken link - core/intrinsics/fn.caller_location.html
reference/print.html:11792: broken link - core/prelude/index.html
reference/print.html:11797: broken link - core/index.html
reference/print.html:11829: broken link - core/index.html
reference/print.html:11836: broken link - core/prelude/index.html
reference/print.html:11838: broken link - core/index.html
reference/print.html:12798: broken link - core/ptr/struct.NonNull.html
reference/print.html:12798: broken link - core/num/index.html
reference/print.html:13059: broken link - core/panic/struct.PanicInfo.html
reference/names/preludes.html:190: broken link - core/prelude/index.html
reference/names/preludes.html:195: broken link - core/index.html
reference/names/preludes.html:227: broken link - core/index.html
reference/names/preludes.html:234: broken link - core/prelude/index.html
reference/names/preludes.html:236: broken link - core/index.html
reference/behavior-considered-undefined.html:241: broken link - core/ptr/struct.NonNull.html
reference/behavior-considered-undefined.html:241: broken link - core/num/index.html
reference/runtime.html:177: broken link - core/panic/struct.PanicInfo.html
reference/attributes/codegen.html:283: broken link - core/panic/struct.Location.html
reference/attributes/codegen.html:286: broken link - core/panic/struct.Location.html
reference/attributes/codegen.html:297: broken link - core/panic/struct.Location.html
reference/attributes/codegen.html:298: broken link - core/intrinsics/fn.caller_location.html
thread 'main' panicked at 'found some broken links', src/tools/linkchecker/main.rs:95:9


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
expected success, got: exit code: 101
