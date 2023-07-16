plain
.................................................................................................... 9400/11754
.................................................................................................... 9500/11754
........................................................................................i......i.... 9600/11754
.................................................................................................... 9700/11754
..................................iiiiiii..iiiiii.i................................................. 9800/11754
.................................................................................................... 10000/11754
.................................................................................................... 10100/11754
.................................................................................................... 10200/11754
.................................................................................................... 10300/11754
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 35 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiiiii....
test result: ok. 4 passed; 0 failed; 31 ignored; 0 measured; 0 filtered out; finished in 0.11s

 finished in 0.169 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.29s

 finished in 2.353 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
tmp.js:1519
});
  ^

SyntaxError: Unexpected end of input
    at createScript (vm.js:80:10)
    at Object.runInThisContext (vm.js:139:10)
    at Module._compile (module.js:616:28)
    at loadContent (/checkout/src/tools/rustdoc-js/tester.js:165:7)
    at loadMainJsAndIndex (/checkout/src/tools/rustdoc-js/tester.js:255:19)
    at load_files (/checkout/src/tools/rustdoc-js/tester.js:390:12)
    at main (/checkout/src/tools/rustdoc-js/tester.js:465:27)
    at Object.<anonymous> (/checkout/src/tools/rustdoc-js/tester.js:485:14)
    at Module._compile (module.js:652:30)
    at Object.Module._extensions..js (module.js:663:10)


command did not execute successfully: "/usr/bin/node" "/checkout/src/tools/rustdoc-js/tester.js" "--crate-name" "std" "--resource-suffix" "1.53.0" "--doc-folder" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc" "--test-folder" "/checkout/src/test/rustdoc-js-std"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:30:58
