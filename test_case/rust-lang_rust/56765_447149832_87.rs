\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-42060.rs","byte_start":514,"byte_end":527,"line_start":13,"line_end":13,"column_start":16,"column_end":29,"is_primary":true,"text":[{"text":"    let other: typeof(thing) = thing; //~ ERROR attempt to use a non-constant value in a constant","highlight_start":16,"highlight_end":29}],"label":"reserved keyword","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0516]: `typeof` is a reserved keyword but unimplemented\n  --> /checkout/src/test/ui/issues/issue-42060.rs:13:16\n   |\nLL |     let other: typeof(thing) = thing; //~ ERROR attempt to use a non-constant value in a constant\n   |                ^^^^^^^^^^^^^ reserved keyword\n\n"}
[00:49:31] {"message":"aborting due to 4 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 4 previous errors\n\n"}
[00:49:31] {"message":"Some errors occurred: E0435, E0516.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0435, E0516.\n"}
[00:49:31] {"message":"For more information about an error, try `rustc --explain E0435`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0435`.\n"}
[00:49:31] ------------------------------------------
[00:49:31] 
[00:49:31] thread '[ui] ui/issues/issue-42060.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[00:49:31] 
[00:49:31] 
[00:49:31] ---- [ui] ui/issues/issue-4335.rs stdout ----
[00:49:31] diff of stderr:
[00:49:31] 
[00:49:31] + error[E0507]: cannot move out of borrowed content
[00:49:31] +   --> $DIR/issue-4335.rs:16:20
[00:49:31] +    |
[00:49:31] + LL |     id(Box::new(|| *v))
[00:49:31] +    |                    ^^ cannot move out of borrowed content
[00:49:31] + 
[00:49:31] 1 error[E0373]: closure may outlive the current function, but it borrows `v`, which is owned by the current function
[00:49:31] 3    |
[00:49:31] 
[00:49:31] 9    |
[00:49:31] 9    |
[00:49:31] 10 LL |     id(Box::new(move || *v))
[00:49:31] - 
[00:49:31] - error[E0507]: cannot move out of borrowed content
[00:49:31] -   --> $DIR/issue-4335.rs:16:20
[00:49:31] -    |
[00:49:31] -    |
[00:49:31] - LL |     id(Box::new(|| *v))
[00:49:31] -    |                    ^^ cannot move out of borrowed content
[00:49:31] 19 error: aborting due to 2 previous errors
[00:49:31] 20 
[00:49:31] 
[00:49:31] 
[00:49:31] 
[00:49:31] The actual stderr differed from the expected stderr.
[00:49:31] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-4335/issue-4335.stderr
[00:49:31] To update references, rerun the tests and pass the `--bless` flag
[00:49:31] To only update this specific test, also pass `--test-args issues/issue-4335.rs`
[00:49:31] error: 1 errors occurred comparing output.
[00:49:31] status: exit code: 1
[00:49:31] status: exit code: 1
[00:49:31] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-4335.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-4335/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-4335/auxiliary" "-A" "unused"
[00:49:31] ------------------------------------------
[00:49:31] 
[00:49:31] ------------------------------------------
[00:49:31] stderr:
[00:49:31] stderr:
[00:49:31] ------------------------------------------
[00:49:31] {"message":"cannot move out of borrowed content","code":{"code":"E0507","explanation":"\nYou tried to move out of a value which was borrowed. Erroneous code example:\n\n