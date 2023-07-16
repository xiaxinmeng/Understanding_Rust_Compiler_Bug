\n\nIf the item is not defined in the current module, itissue-10636-2.rs:18:1
[00:50:34] 
[00:50:34] The actual stderr differed from the expected stderr.
[00:50:34] The actual stderr differed from the expected stderr.
[00:50:34] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/token/issue-10636-2/issue-10636-2.stderr
[00:50:34] To update references, rerun the tests and pass the `--bless` flag
[00:50:34] To only update this specific test, also pass `--test-args token/issue-10636-2.rs`
[00:50:34] error: 1 errors occurred comparing output.
[00:50:34] status: exit code: 101
[00:50:34] status: exit code: 101
[00:50:34] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/token/issue-10636-2.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/token/issue-10636-2/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/token/issue-10636-2/auxiliary" "-A" "unused"
[00:50:34] ------------------------------------------
[00:50:34] 
[00:50:34] ------------------------------------------
[00:50:34] stderr:
[00:50:34] stderr:
[00:50:34] ------------------------------------------
[00:50:34] {"message":"incorrect close delimiter: `}`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/token/issue-10636-2.rs","byte_start":763,"byte_end":764,"line_start":18,"line_end":18,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"} //~ ERROR: incorrect close delimiter","highlight_start":1,"highlight_end":2}],"label":null,"suggested_
travis_fold:start:after_failure.4
travis_time:start:0f7f852c
$ dmesg | grep -i kill
[   10.743173] init: failsafe main process (1094) killed by TERM signal
