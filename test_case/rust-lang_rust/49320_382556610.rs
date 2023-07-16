plain
travis_time:start:test_ui
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:58:45] 
[00:58:45] running 1363 tests
[00:58:52] ..F...............................................................................i.................
[00:59:04] ....................................................................................................
[00:59:09] ....................................................................................................
[00:59:13] ....................................................................................................
[00:59:19] ....................................................................................................
---
[00:59:57] ....................................................................................................
[01:00:05] .............ii.....................................................................................
[01:00:15] ..............................................................................................i.....
[01:00:19] To update references, run this command from build directory:
[01:00:19] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'allocator-submodule.rs'
[01:00:19] error: 1 errors occurred comparing output.
[01:00:19] status: exit code: 101
[01:00:19] status: exit code: 101
[01:00:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/allocator-submodule.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator-submodule.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator-submodule.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[01:00:19] ------------------------------------------
[01:00:19] 
[01:00:19] ------------------------------------------
[01:00:19] stderr:
[01:00:19] stderr:
[01:00:19] ------------------------------------------
[01:00:19] {"message":"unresolved import `MY_HEAP`","code":{"code":"E0432","explanation":"\nAn import was unresolved.\n\nErroneous code example:\n\n