plain
.................................................................................................... 9500/11877
.................................................................................................... 9600/11877
..............................................................................i......i.............. 9700/11877
.................................................................................................... 9800/11877
.......................iiiiiii..iiiiii.i............................................................ 9900/11877
.................................................................................................... 10100/11877
.................................................................................................... 10200/11877
.................................................................................................... 10300/11877
.................................................................................................... 10400/11877
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 36 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii....

 finished in 0.169 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i...i.ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 3.54s

 finished in 3.607 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
Building stage0 tool linkchecker (x86_64-unknown-linux-gnu)
   Compiling once_cell v1.7.2
   Compiling linkchecker v0.1.0 (/checkout/src/tools/linkchecker)
    Finished release [optimized] target(s) in 1.73s
alloc/collections/binary_heap/struct.IntoIter.html:139: broken link fragment `#tymethod.as_inner` pointing to `alloc/collections/binary_heap/struct.IntoIter.html`
alloc/collections/binary_heap/struct.IntoIter.html:139: broken link fragment `#tymethod.as_inner` pointing to `alloc/collections/binary_heap/struct.IntoIter.html`
alloc/vec/struct.IntoIter.html:164: broken link fragment `#tymethod.as_inner` pointing to `alloc/vec/struct.IntoIter.html`
alloc/vec/struct.IntoIter.html:164: broken link fragment `#tymethod.as_inner` pointing to `alloc/vec/struct.IntoIter.html`
thread 'main' panicked at 'found some broken links', src/tools/linkchecker/main.rs:95:9


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
expected success, got: exit code: 101
