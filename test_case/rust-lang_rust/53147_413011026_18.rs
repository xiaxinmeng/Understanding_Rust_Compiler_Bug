\non_applicability":null,"expansion":null}],"children":[{"message":"consider borrowing here","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-17718-static-move.rs","byte_start":552,"byte_end":555,"line_start":16,"line_end":16,"column_start":14,"column_end":17,"is_primary":true,"text":[{"text":"    let _a = FOO; //~ ERROR: cannot move out of static item","highlight_start":14,"highlight_end":17}],"label":null,"suggested_replacement":"&FOO","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0507]: cannot move out of static item\n  --> /checkout/src/test/ui/issues/issue-17718-static-move.rs:16:14\n   |\nLL |     let _a = FOO; //~ ERROR: cannot move out of static item\n   |              ^^^\n   |              |\n   |              cannot move out of static item\n   |              help: consider borrowing here: `&FOO`\n\n"}
[00:52:46] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:52:46] {"message":"For more information about this error, try `rustc --explain E0507`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0507`.\n"}
[00:52:46] ------------------------------------------
[00:52:46] 
[00:52:46] thread '[ui (nll)] ui/issues/issue-17718-static-move.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3166:9
[00:52:46] 
[00:52:46] 
[00:52:46] ---- [ui (nll)] ui/std-uncopyable-atomics.rs stdout ----
[00:52:46] diff of stderr:
[00:52:46] 
[00:52:46] 5    |             ^^^
[00:52:46] 6    |             |
[00:52:46] 7    |             cannot move out of borrowed content
[00:52:46] -    |             help: consider using a reference instead: `&*&x`
[00:52:46] +    |             help: consider removing the `*`: `&x`
[00:52:46] 10 error[E0507]: cannot move out of borrowed content
[00:52:46] 11   --> $DIR/std-uncopyable-atomics.rs:21:13
[00:52:46] 
[00:52:46] 14    |             ^^^
[00:52:46] 14    |             ^^^
[00:52:46] 15    |             |
[00:52:46] 16    |             cannot move out of borrowed content
[00:52:46] -    |             help: consider using a reference instead: `&*&x`
[00:52:46] +    |             help: consider removing the `*`: `&x`
[00:52:46] 19 error[E0507]: cannot move out of borrowed content
[00:52:46] 20   --> $DIR/std-uncopyable-atomics.rs:23:13
[00:52:46] 
[00:52:46] 23    |             ^^^
[00:52:46] 23    |             ^^^
[00:52:46] 24    |             |
[00:52:46] 25    |             cannot move out of borrowed content
[00:52:46] -    |             help: consider using a reference instead: `&*&x`
[00:52:46] +    |             help: consider removing the `*`: `&x`
[00:52:46] 28 error[E0507]: cannot move out of borrowed content
[00:52:46] 29   --> $DIR/std-uncopyable-atomics.rs:25:13
[00:52:46] 
[00:52:46] 32    |             ^^^
[00:52:46] 32    |             ^^^
[00:52:46] 33    |             |
[00:52:46] 34    |             cannot move out of borrowed content
[00:52:46] -    |             help: consider using a reference instead: `&*&x`
[00:52:46] +    |             help: consider removing the `*`: `&x`
[00:52:46] 37 error: aborting due to 4 previous errors
[00:52:46] 38 
[00:52:46] 
[00:52:46] 
[00:52:46] 
[00:52:46] The actual stderr differed from the expected stderr.
[00:52:46] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/std-uncopyable-atomics.nll/std-uncopyable-atomics.nll.stderr
[00:52:46] To update references, rerun the tests and pass the `--bless` flag
[00:52:46] To only update this specific test, also pass `--test-args std-uncopyable-atomics.rs`
[00:52:46] error: 1 errors occurred comparing output.
[00:52:46] status: exit code: 1
[00:52:46] status: exit code: 1
[00:52:46] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/std-uncopyable-atomics.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/std-uncopyable-atomics.nll/a" "-Zborrowck=mir" "-Ztwo-phase-borrows" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/std-uncopyable-atomics.nll/auxiliary" "-A" "unused"
[00:52:46] ------------------------------------------
[00:52:46] 
[00:52:46] ------------------------------------------
[00:52:46] stderr:
[00:52:46] stderr:
[00:52:46] ------------------------------------------
[00:52:46] {"message":"cannot move out of borrowed content","code":{"code":"E0507","explanation":"\nYou tried to move out of a value which was borrowed. Erroneous code example:\n\n