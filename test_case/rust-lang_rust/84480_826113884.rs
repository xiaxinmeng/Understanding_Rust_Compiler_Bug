plain
Set({"src/test/rustdoc-gui"}) not skipped for "bootstrap::test::RustdocGUI" -- not in ["src/tools/tidy"]
Set({"src/tools/rustdoc-themes"}) not skipped for "bootstrap::test::RustdocTheme" -- not in ["src/tools/tidy"]
Suite("src/test/rustdoc-ui") not skipped for "bootstrap::test::RustdocUi" -- not in ["src/tools/tidy"]
Suite("src/test/rustdoc-json") not skipped for "bootstrap::test::RustdocJson" -- not in ["src/tools/tidy"]
Set({"src/tools/html-checker"}) not skipped for "bootstrap::test::HtmlCheck" -- not in ["src/tools/tidy"]
not running HTML-check tool because `tidy` is missing
Suite("src/test/run-make") not skipped for "bootstrap::test::RunMake" -- not in ["src/tools/tidy"]
Skipping Set({"src/tools/tidy"}) because it is excluded
Suite("src/test/ui") not skipped for "bootstrap::test::Ui" -- not in ["src/tools/tidy"]
Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
.................................................................................................... 9400/11775
.................................................................................................... 9500/11775
.................................................................................................... 9600/11775
.i.......i.......................................................................................... 9700/11775
...............................................iiiiiii..iiiiii.i.................................... 9800/11775
.................................................................................................... 10000/11775
.................................................................................................... 10100/11775
.................................................................................................... 10200/11775
.................................................................................................... 10300/11775
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 35 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiiiii....
test result: ok. 4 passed; 0 failed; 31 ignored; 0 measured; 0 filtered out; finished in 0.13s

 finished in 0.193 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.49s

 finished in 2.554 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
    Finished release [optimized] target(s) in 0.65s
rustdoc: [check-theme] Starting tests! (Ignoring all other arguments)
 - Checking "/checkout/src/librustdoc/html/static/themes/dark.css"... OK
 - Checking "/checkout/src/librustdoc/html/static/themes/ayu.css"... FAILED
  Missing "h2,h3,h4" rule

command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rustdoc-themes" "/checkout/obj/build/bootstrap/debug/rustdoc" "/checkout/src/librustdoc/html/static/themes"
expected success, got: exit code: 1

