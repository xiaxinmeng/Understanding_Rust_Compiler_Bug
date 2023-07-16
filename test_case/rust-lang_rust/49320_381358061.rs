plain
[00:00:50] configure: rust.quiet-tests     := True
---
[00:46:36] F...............................................................................i...................
[00:46:43] .......................i............................................................................
---
[00:47:26] .....................i...........................................................................i..
[00:47:32] ....................................................................................................
[00:47:38] ...........ii.......................................................................................
---
[00:47:50] error[E0432]: unresolved import `MY_HEAP`
[00:47:50]   --> $DIR/allocator-submodule.rs:22:5
[00:47:50]    |
[00:47:50] LL |     static MY_HEAP: Global = Global::default(); //~ ERROR
[00:47:50]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `MY_HEAP` in the root
---
[00:47:50] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator-submodule.stderr
[00:47:50] To update references, run this command from build directory:
[00:47:50] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'allocator-submodule.rs'
[00:47:50]
[00:47:50] error: 1 errors occurred comparing output.
[00:47:50] status: exit code: 101
[00:47:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/allocator-submodule.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator-submodule.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator-submodule.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
---
[00:47:50] {"message":"unresolved import `MY_HEAP`","code":{"code":"E0432","explanation":"\nAn import was unresolved.\n\nErroneous code example:\n\n