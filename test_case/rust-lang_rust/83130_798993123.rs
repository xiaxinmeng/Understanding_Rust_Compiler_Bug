plain
.................................................................................................... 9300/11557
.................................................................................................... 9400/11557
.....................................................................................i......i....... 9500/11557
.................................................................................................... 9600/11557
...............................iiiiiii..iiiiii.i.................................................... 9700/11557
.................................................................................................... 9900/11557
.................................................................................................... 10000/11557
.................................................................................................... 10100/11557
.................................................................................................... 10200/11557
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 29 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]

 finished in 0.075 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
 finished in 11.054 seconds
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.41s

 finished in 2.483 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
Building stage0 tool linkchecker (x86_64-unknown-linux-gnu)
   Compiling once_cell v1.4.1
   Compiling linkchecker v0.1.0 (/checkout/src/tools/linkchecker)
    Finished release [optimized] target(s) in 1.84s
core/slice/struct.EscapeAscii.html:5: broken intra-doc link - [<code>[u8]::escape_ascii</code>]
core/slice/struct.EscapeAscii.html:7: broken intra-doc link - [<code>[u8]::escape_ascii</code>]
thread 'main' panicked at 'found some broken links', src/tools/linkchecker/main.rs:97:9


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
expected success, got: exit code: 101
