plain
.................................................................................................... 9400/11770
.................................................................................................... 9500/11770
...................................................................................................i 9600/11770
......i............................................................................................. 9700/11770
..............................................iiiiiii.iiiiii.i...................................... 9800/11770
.................................................................................................... 10000/11770
.................................................................................................... 10100/11770
.................................................................................................... 10200/11770
.................................................................................................... 10300/11770
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 35 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiiiii....
test result: ok. 4 passed; 0 failed; 31 ignored; 0 measured; 0 filtered out; finished in 0.12s

 finished in 0.184 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.55s

 finished in 2.622 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
   Compiling rustdoc-themes v0.1.0 (/checkout/src/tools/rustdoc-themes)
    Finished release [optimized] target(s) in 0.68s
rustdoc: [check-theme] Starting tests! (Ignoring all other arguments)
 - Checking "/checkout/src/librustdoc/html/static/themes/dark.css"... FAILED
  Missing ".content a.result-fn:focus,.content a.result-method:focus,.content a.result-tymethod:focus" rule
  Missing ".content a.result-type:focus" rule
  Missing ".content a.result-keyword:focus" rule
  Missing ".content a.result-constant:focus,.content a.result-static:focus" rule
  Missing ".content a.result-mod:focus,.content a.result-externcrate:focus" rule
  Missing ".content a.result-attr:focus,.content a.result-derive:focus,.content a.result-macro:focus" rule
  Missing ".content a.result-primitive:focus" rule
  Missing ".content a.result-struct:focus" rule
  Missing ".content a.result-union:focus" rule
  Missing ".content a:focus" rule
  Missing ".content a.result-traitalias:focus" rule
  Missing ".content a.result-foreigntype:focus" rule
  Missing ".content a.result-trait:focus" rule
  Missing ".content a.result-enum:focus" rule
  Missing ".content a:focus,.content span:focus" rule
  Missing ".content a:hover" rule
 - Checking "/checkout/src/librustdoc/html/static/themes/ayu.css"... FAILED
  Missing ".content a.result-fn:focus,.content a.result-method:focus,.content a.result-tymethod:focus" rule
  Missing ".content a.result-type:focus" rule
  Missing ".content a.result-keyword:focus" rule
  Missing ".content a.result-constant:focus,.content a.result-static:focus" rule
  Missing ".content a.result-mod:focus,.content a.result-externcrate:focus" rule
  Missing ".content a.result-attr:focus,.content a.result-derive:focus,.content a.result-macro:focus" rule
  Missing ".content a.result-primitive:focus" rule
  Missing ".content a.result-struct:focus" rule
  Missing ".content a.result-union:focus" rule
  Missing ".content a:focus" rule
  Missing ".content a.result-traitalias:focus" rule
  Missing ".content a.result-foreigntype:focus" rule
  Missing ".content a.result-trait:focus" rule
  Missing ".content a.result-enum:focus" rule
  Missing ".content a:focus,.content span:focus" rule
  Missing ".content a:hover" rule

command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rustdoc-themes" "/checkout/obj/build/bootstrap/debug/rustdoc" "/checkout/src/librustdoc/html/static/themes"
expected success, got: exit code: 1

