plain
[00:45:01] ....................................................................................................
[00:45:05] ....................................................................................................
[00:45:08] ....................................................................................................
[00:45:13] ....................................................................................................
[00:45:18] .......................................F............................................................
[00:45:27] ............................................................i.......................................
[00:45:32] .....................................i..............................................................
[00:45:37] .........................................................ii.........................................
[00:45:42] ....................................................................................................
[00:45:42] ....................................................................................................
[00:45:48] ..........................................................i.........................................
urn type contains a borrowed value, but there is no value for it to be borrowed from
[00:45:50] 25    = help: consider giving it a 'static lifetime
[00:45:50] - error: aborting due to 3 previous errors
[00:45:50] + error: aborting due to 2 previous errors
[00:45:50] 28 
[00:45:50] 29 For more information about this error, try `rustc --explain E0106`.
[00:45:50] 29 For more information about this error, try `rustc --explain E0106`.
[00:45:50] 30 
[00:45:50] 
[00:45:50] 
[00:45:50] The actual stderr differed from the expected stderr.
[00:45:50] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-26638/issue-26638.stderr
[00:45:50] To update references, rerun the tests and pass the `--bless` flag
[00:45:50] To only update this specific test, also pass `--test-args issue-26638.rs`
[00:45:50] error: 1 errors occurred comparing output.
[00:45:50] status: exit code: 101
[00:45:50] status: exit code: 101
[00:45:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issue-26638.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-26638/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-26638/auxiliary" "-A" "unused"
[00:45:50] ------------------------------------------
[00:45:50] 
[00:45:50] ------------------------------------------
[00:45:50] stderr:
[00:45:50] stderr:
[00:45:50] ------------------------------------------
[00:45:50] {"message":"missing lifetime specifier","code":{"code":"E0106","explanation":"\nThis error indicates that a lifetime is missing from a type. If it is an error\ninside a function signature, the problem may be with failing to adhere to the\nlifetime elision rules (see below).\n\nHere are some simple examples of where you'll run into this error:\n\n