plain
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:41:21] 
[00:41:21] running 1569 tests
[00:41:25] ..................................................................................................i.
[00:41:29] ..............................................................F..i..................................
[00:41:34] ....................................................................................................
[00:41:36] ....................................................................................................
[00:41:38] ....................................................................................................
[00:41:41] ....................................................................................................
---
[00:42:11] 
[00:42:11] 5    |     ^^^ not found in this scope
[00:42:11] 6 help: possible candidate is found in another module, you can import it into scope
[00:42:11] 7    |
[00:42:11] - LL | use crate::bar::Foo;
[00:42:11] + LL | use bar::Foo;
[00:42:11] 10 
[00:42:11] 11 error: aborting due to previous error
[00:42:11] 
[00:42:11] 
[00:42:11] 
[00:42:11] The actual stderr differed from the expected stderr.
[00:42:11] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:42:11] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/crate-in-paths/crate-in-paths.stderr
[00:42:11] To update references, rerun the tests and pass the `--bless` flag
[00:42:11] To only update this specific test, also pass `--test-args crate-in-paths.rs`
[00:42:11] error: 1 errors occurred comparing output.
[00:42:11] status: exit code: 101
[00:42:11] status: exit code: 101
[00:42:11] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/crate-in-paths.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/crate-in-paths/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/crate-in-paths/auxiliary" "-A" "unused"
[00:42:11] ------------------------------------------
[00:42:11] 
[00:42:11] ------------------------------------------
[00:42:11] stderr:
[00:42:11] stderr:
[00:42:11] ------------------------------------------
[00:42:11] {"message":"cannot find value `Foo` in this scope","code":{"code":"E0425","explanation":"\nAn unresolved name was used.\n\nErroneous code examples:\n\n