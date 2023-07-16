plain
[01:38:31] ---- [ui (nll)] ui/borrowck/borrowck-in-static.rs stdout ----
[01:38:31] 
[01:38:31] error: ui test compiled successfully!
[01:38:31] status: exit code: 0
[01:38:31] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-in-static.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-in-static.nll/a" "-Zborrowck=migrate" "-Ztwo-phase-borrows" "-Crpath" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-in-static.nll/auxiliary" "-A" "unused"
[01:38:31] ------------------------------------------
[01:38:31] 
[01:38:31] ------------------------------------------
[01:38:31] stderr:
[01:38:31] stderr:
[01:38:31] ------------------------------------------
[01:38:31] {"message":"cannot move out of captured variable in an `Fn` closure","code":{"code":"E0507","explanation":"\nYou tried to move out of a value which was borrowed. Erroneous code example:\n\n