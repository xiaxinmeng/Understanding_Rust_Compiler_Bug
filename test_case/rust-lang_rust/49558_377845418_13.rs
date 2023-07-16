\n\nMust always be called with exactly two arguments, e.g. `f(2, \"test\")`.\n\nNote that Rust does not have a notion of optional function arguments or\nvariadic functions (except for its C-FFI).\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/method-call-err-msg.rs","byte_start":616,"byte_end":645,"line_start":16,"line_end":16,"column_start":5,"column_end":34,"is_primary":false,"text":[{"text":"    fn one(self, _: isize) -> Foo { self }","highlight_start":5,"highlight_end":34}],"label":"definee following traits define an item `take`, perhaps you need to implement one of them:\n           candidate #1: `std::iter::Iterator`\n           candidate #2: `std::io::Read`\n\n"}
[00:40:47] {"message":"aborting due to 4 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 4 previous errors\n\n"}
[00:40:47] {"message":"Some errors occurred: E0061, E0599.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0061, E0599.\n"}
[00:40:47] {"message":"For more information about an error, try `rustc --explain E0061`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0061`.\n"}
---
[00:40:47] 26    = help: to disambiguate the method call, write `UnusedTrait::f9(u, 342)` instead
[00:40:47] 27    = help: items from traits can only be used if the trait is implemented and in scope
[00:40:47] 28    = note: the following traits define an item `f9`, perhaps you need to implement one of them:
[00:40:47] -            candidate #1: `CtxtFn`
[00:40:47] +            candidate #1: `UnusedTrait`
[00:40:47] 30            candidate #2: `OtherTrait`
[00:40:47] -            candidate #3: `UnusedTrait`
[00:40:47] +            candidate #3: `CtxtFn`
[00:40:47] 32
[00:40:47] 33 error[E0599]: no method named `fff` found for type `Myisize` in the current scope
[00:40:47] 34   --> $DIR/issue-7575.rs:74:30
[00:40:47]
[00:40:47]
[00:40:47] The actual stderr differed from the expected stderr.
[00:40:47] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-7575.stderr
[00:40:47] To update references, run this command from build directory:
[00:40:47] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'span/issue-7575.rs'
[00:40:47]
[00:40:47] error: 1 errors occurred comparing output.
[00:40:47] status: exit code: 101
[00:40:47] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/issue-7575.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-7575.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-7575.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
---
[00:40:47] {"message":"no method named `f9` found for type `usize` in the current scope","code":{"code":"E0599","explanation":"\nThis error occurs when a methonown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zmiri -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zmiri -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:40:47] expected success, got: exit code: 101
[00:40:47]
[00:40:47]
[00:40:47] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:40:47] Build completed unsuccessfully in 0:02:00
[00:40:47] Makefile:58: recipe for target 'check' failed
[00:40:47] make: *** [check] Error 1
---
$ cat obj/tmp/sccache.log
