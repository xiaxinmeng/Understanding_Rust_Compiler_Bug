plain
.................................................................................................... 9400/11731
.................................................................................................... 9500/11731
........................................................................i......i.................... 9600/11731
.................................................................................................... 9700/11731
..................iiiiiii..iiiiii.i................................................................. 9800/11731
.................................................................................................... 10000/11731
.................................................................................................... 10100/11731
.................................................................................................... 10200/11731
.................................................................................................... 10300/11731
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 31 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii..

 finished in 0.113 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.56s

 finished in 2.632 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
Building stage0 tool linkchecker (x86_64-unknown-linux-gnu)
   Compiling once_cell v1.7.2
   Compiling linkchecker v0.1.0 (/checkout/src/tools/linkchecker)
    Finished release [optimized] target(s) in 2.01s
nomicon/print.html:314: broken link - std/alloc/trait.GlobalAlloc.html
nomicon/print.html:847: broken link - std/alloc/trait.GlobalAlloc.html
nomicon/safe-unsafe-meaning.html:214: broken link - std/alloc/trait.GlobalAlloc.html
nomicon/exotic-sizes.html:263: broken link - std/alloc/trait.GlobalAlloc.html
reference/type-layout.html:365: broken link - std/alloc/struct.Layout.html
reference/print.html:8230: broken link - std/task/enum.Poll.html
reference/print.html:8231: broken link - std/task/enum.Poll.html
reference/print.html:8231: broken link - std/task/enum.Poll.html
reference/print.html:8237: broken link - std/task/struct.Context.html
reference/print.html:10116: broken link - std/alloc/struct.Layout.html
reference/print.html:13099: broken link - alloc/alloc/trait.GlobalAlloc.html
reference/expressions/await-expr.html:186: broken link - std/task/enum.Poll.html
reference/expressions/await-expr.html:187: broken link - std/task/enum.Poll.html
reference/expressions/await-expr.html:187: broken link - std/task/enum.Poll.html
reference/expressions/await-expr.html:193: broken link - std/task/struct.Context.html
reference/runtime.html:217: broken link - alloc/alloc/trait.GlobalAlloc.html
thread 'main' panicked at 'found some broken links', src/tools/linkchecker/main.rs:95:9


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
expected success, got: exit code: 101
