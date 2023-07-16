plain
.................................................................................................... 9000/11247
.................................................................................................... 9100/11247
.................................................................................................... 9200/11247
...........................................i......i................................................. 9300/11247
..................................................................................iiiiii..iiiiii.i.. 9400/11247
.................................................................................................... 9600/11247
.................................................................................................... 9700/11247
.................................................................................................... 9800/11247
.................................................................................................... 9900/11247
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 27 tests
iiiiiiiiiiiiiiiiiiiiiiiiiii

Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
 finished in 0.071 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i...ii...i..i...ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.31s

 finished in 2.377 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
Building stage0 tool linkchecker (x86_64-unknown-linux-gnu)
   Compiling once_cell v1.4.1
   Compiling linkchecker v0.1.0 (/checkout/src/tools/linkchecker)
    Finished release [optimized] target(s) in 1.78s
rustdoc/linking-to-items-by-name.html:205: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/std/primitive.slice.html
rustdoc/linking-to-items-by-name.html:206: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/std/primitive.array.html
rustdoc/linking-to-items-by-name.html:207: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/std/primitive.tuple.html
rustdoc/linking-to-items-by-name.html:208: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/std/primitive.unit.html
rustdoc/linking-to-items-by-name.html:209: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/std/primitive.fn.html
rustdoc/linking-to-items-by-name.html:210: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/std/primitive.pointer.html
rustdoc/linking-to-items-by-name.html:211: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/std/primitive.reference.html
rustdoc/linking-to-items-by-name.html:212: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/std/primitive.never.html
rustdoc/print.html:1309: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/std/primitive.slice.html
rustdoc/print.html:1310: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/std/primitive.array.html
rustdoc/print.html:1311: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/std/primitive.tuple.html
rustdoc/print.html:1312: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/std/primitive.unit.html
rustdoc/print.html:1313: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/std/primitive.fn.html
rustdoc/print.html:1314: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/std/primitive.pointer.html
rustdoc/print.html:1315: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/std/primitive.reference.html
rustdoc/print.html:1316: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/std/primitive.never.html
thread 'main' panicked at 'found some broken links', src/tools/linkchecker/main.rs:111:9


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
expected success, got: exit code: 101
