plain
.................................................................................................... 9400/11795
.................................................................................................... 9500/11795
.................................................................................................... 9600/11795
.....................i......i....................................................................... 9700/11795
...................................................................iiiiiii..iiiiii.i................ 9800/11795
.................................................................................................... 10000/11795
.................................................................................................... 10100/11795
.................................................................................................... 10200/11795
.................................................................................................... 10300/11795
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 35 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiiiii....

 finished in 0.158 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i...i.ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.30s

 finished in 2.367 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
    Finished release [optimized] target(s) in 0.54s
rustdoc: [check-theme] Starting tests! (Ignoring all other arguments)
 - Checking "/checkout/src/librustdoc/html/static/themes/dark.css"... OK
 - Checking "/checkout/src/librustdoc/html/static/themes/ayu.css"... FAILED
  Missing ".search-input:focus" rule

command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rustdoc-themes" "/checkout/obj/build/bootstrap/debug/rustdoc" "/checkout/src/librustdoc/html/static/themes"
expected success, got: exit code: 1

