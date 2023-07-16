plain
[00:00:46] configure: rust.quiet-tests     := True
---
[00:43:07] F...............................................................................i...................
[00:43:13] .......................i............................................................................
---
[00:43:56] .....................i............................................................................i.
[00:44:02] ....................................................................................................
[00:44:09] ...........ii.......................................................................................
---
[00:44:20] error[E0432]: unresolved import `MY_HEAP`
[00:44:20]   --> $DIR/allocator-submodule.rs:22:5
[00:44:20]    |
[00:44:20] LL |     static MY_HEAP: Global = Global::default(); //~ ERROR
[00:44:20]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `MY_HEAP` in the root
---
[00:44:20] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator-submodule.stderr
[00:44:20] To update references, run this command from build directory:
[00:44:20] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'allocator-submodule.rs'
[00:44:20]
[00:44:20] error: 1 errors occurred comparing output.
[00:44:20] status: exit code: 101
[00:44:20] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/allocator-submodule.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator-submodule.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator-submodule.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
---
[00:44:20] {"message":"unresolved import `MY_HEAP`","code":{"code":"E0432","explanation":"\nAn import was unresolved.\n\nErroneous code example:\n\n