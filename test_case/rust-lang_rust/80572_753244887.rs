plain
.................................................................................................... 9000/11231
.................................................................................................... 9100/11231
.................................................................................................... 9200/11231
..................i......i.......................................................................... 9300/11231
.........................................................iiiiii..iiiiii.i........................... 9400/11231
.................................................................................................... 9600/11231
.................................................................................................... 9700/11231
.................................................................................................... 9800/11231
.................................................................................................... 9900/11231
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 27 tests
iiiiiiiiiiiiiiiiiiiiiiiiiii

 finished in 0.071 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i...ii...i..i...ii..........iiii.........i.....i....i......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.25s

 finished in 2.331 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
Building stage0 tool linkchecker (x86_64-unknown-linux-gnu)
   Compiling once_cell v1.4.1
   Compiling linkchecker v0.1.0 (/checkout/src/tools/linkchecker)
    Finished release [optimized] target(s) in 1.66s
core/result/enum.Result.html:487: broken link - core/primitive.slice.html
thread 'main' panicked at 'found some broken links', src/tools/linkchecker/main.rs:111:9


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
expected success, got: exit code: 101
