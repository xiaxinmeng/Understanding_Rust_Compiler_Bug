\nlet mut i = 0;\nlet mut x = &mut i; // ok!\n\n// or:\nlet mut i = 0;\nlet a  |                 ---- ---------  ^^^^^^^ immutable borrow occurs here
[00:48:22] -    |                 |    mutable borrow later used by call
[00:48:22] -    |                 |    mutable borrow later used by call
[00:48:22] +    |                 |    mutable borrow used by call, in later iteration of loop
[00:48:22] 20 
[00:48:22] 20 
[00:48:22] 21 error[E0502]: cannot borrow `*self.cx_mut` as immutable because it is also borrowed as mutable
[00:48:22] 
[00:48:22] 24 LL |                 self.hash_expr(&self.cx_mut.body(eid).value);
[00:48:22] 25    |                 ---- ---------  ^^^^^^^^^^^ immutable borrow occurs here
[00:48:22] -    |                 |    mutable borrow later used by call
[00:48:22] -    |                 |    mutable borrow later used by call
[00:48:22] +    |                 |    mutable borrow used by call, in later iteration of loop
[00:48:22] 29 
[00:48:22] 30 error[E0502]: cannot borrow `reg.sess_mut` as immutable because it is also borrowed as mutable
[00:48:22] 
[00:48:22] 
[00:48:22] 33 LL |     reg.register_static(Box::new(TrivialPass::new(&reg.sess_mut)));
[00:48:22] 34    |     --- ---------------                           ^^^^^^^^^^^^^ immutable borrow occurs here
[00:48:22] -    |     |   mutable borrow later used by call
[00:48:22] -    |     |   mutable borrow later used by call
---
[00:48:22]     [ui] ui/borrowck/borrowck-describe-lvalue.rs#mir
[00:48:22]     [ui] ui/borrowck/borrowck-imm-ref-to-mut-rec-field-issue-3162-c.rs#mir
[00:48:22]     [ui] ui/borrowck/borrowck-match-already-borrowed.rs#mir
[00:48:22]     [ui] ui/borrowck/borrowck-overloaded-index-ref-index.rs#mir
[00:48:22]     [ui] ui/borrowck/borrobin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:48:22] 
[00:48:22] 
[00:48:22] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:48:22] Build completed unsuccessfully in 0:03:36
[00:48:22] Build completed unsuccessfully in 0:03:36
[00:48:22] make: *** [check] Error 1
[00:48:22] Makefile:58: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0af23bc1
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Nov 20 22:29:24 UTC 2018
