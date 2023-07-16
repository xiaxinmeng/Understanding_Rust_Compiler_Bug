plain
.................................................................................................... 9000/11228
.................................................................................................... 9100/11228
.................................................................................................... 9200/11228
...............i......i............................................................................. 9300/11228
......................................................iiiiii..iiiiii.i.............................. 9400/11228
.................................................................................................... 9600/11228
.................................................................................................... 9700/11228
.................................................................................................... 9800/11228
.................................................................................................... 9900/11228
---
Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 238 tests
iii............iii...ii..i.ii.........i.ii..........i..............i.............i.i...iii.......iii 100/238
..........i.....i.............i.i.i....i.i.iiii.......................................ii..i..i....i. 200/238
test result: ok. 193 passed; 0 failed; 45 ignored; 0 measured; 0 filtered out; finished in 2.88s

 finished in 2.952 seconds
Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 27 tests
iiiiiiiiiiiiiiiiiiiiiiiiiii

 finished in 0.072 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.20s

 finished in 2.274 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
Building stage0 tool linkchecker (x86_64-unknown-linux-gnu)
   Compiling once_cell v1.4.1
   Compiling linkchecker v0.1.0 (/checkout/src/tools/linkchecker)
    Finished release [optimized] target(s) in 1.79s
core/num/struct.ParseIntError.html:10: broken intra-doc link - [<code>str.trim()</code>]
std/num/struct.ParseIntError.html:10: broken intra-doc link - [<code>str.trim()</code>]
thread 'main' panicked at 'found some broken links', src/tools/linkchecker/main.rs:111:9


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
expected success, got: exit code: 101
