\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-17252.rs","byte_start":486,"byte_end":489,"line_start":11,"line_end":11,"column_start":20,"column_end":23,"is_primary":true,"text":[{"text":"const FOO: usize = FOO; //~ ERROR E0391","highlight_start":20,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"...which again requires processing `FOO`, completing the cycle","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"cycle used when processing `main::{{constant}}`","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-17252.rs","byte_start":537,"byte_end":540,"line_start":14,"line_end":14,"column_start":18,"column_end":21,"is_primary":true,"text":[{"text":"    let _x: [u8; FOO]; // caused stack overflow prior to fix","highlight_start":18,"highlight_end":21}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0391]: cycle detected when processing `FOO`\n  --> /checkout/src/test/ui/issues/issue-17252.rs:11:20\n   |\nLL | const FOO: usize = FOO; //~ ERROR E0391\n   |                    ^^^\n   |\n   = note: ...which again requires processing `FOO`, completing the cycle\nnote: cycle used when processing `main::{{constant}}`\n  --> /checkout/src/test/ui/issues/issue-17252.rs:14:18\n   |\nLL |     let _x: [u8; FOO]; // caused stack overflow prior to fix\n   |                  ^^^\n\n"}
[00:47:21] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:47:21] {"message":"For more information about this error, try `rustc --explain E0391`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0391`.\n"}
[00:47:21] ------------------------------------------
[00:47:21] 
[00:47:21] thread '[ui] ui/issues/issue-17252.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3196:9
[00:47:21] 
[00:47:21] 
[00:47:21] ---- [ui] ui/issues/issue-21177.rs stdout ----
[00:47:21] diff of stderr:
[00:47:21] 
[00:47:21] 5    |                     ^^^^
[00:47:21] 6    |
[00:47:21] 7    = note: ...which again requires computing the bounds for type parameter `T`, completing the cycle
[00:47:21] + note: cycle used when processing `foo`
[00:47:21] +   --> $DIR/issue-21177.rs:16:21
[00:47:21] +    |
[00:47:21] + LL | fn foo<T: Trait<A = T::B>>() { }
[00:47:21] 8 
[00:47:21] 8 
[00:47:21] 9 error[E0220]: associated type `B` not found for `T`
[00:47:21] 
[00:47:21] 
[00:47:21] The actual stderr differed from the expected stderr.
[00:47:21] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-21177/issue-21177.stderr
[00:47:21] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-21177/issue-21177.stderr
[00:47:21] To update references, rerun the tests and pass the `--bless` flag
[00:47:21] To only update this specific test, also pass `--test-args issues/issue-21177.rs`
[00:47:21] error: 1 errors occurred comparing output.
[00:47:21] status: exit code: 1
[00:47:21] status: exit code: 1
[00:47:21] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-21177.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-21177/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-21177/auxiliary" "-A" "unused"
[00:47:21] ------------------------------------------
[00:47:21] 
[00:47:21] ------------------------------------------
[00:47:21] stderr:
[00:47:21] stderr:
[00:47:21] ------------------------------------------
[00:47:21] {"message":"cycle detected when computing the bounds for type  found for `T`\n  --> /checkout/src/test/ui/issues/issue-21177.rs:16:21\n   |\nLL | fn foo<T: Trait<A = T::B>>() { }\n   |                     ^^^^ associated type `B` not found\n\n"}
[00:47:21] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:47:21] {"message":"Some errors occurred: E0220, E0391.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0220, E0391.\n"}
[00:47:21] {"message":"For more information about an error, try `rustc --explain E0220`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0220`.\n"}
[00:47:21] ------------------------------------------
[00:47:21] 
[00:47:21] thread '[ui] ui/issues/issue-21177.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3196:9
[00:47:21] 
[00:47:21] 
[00:47:21] ---- [ui] ui/issues/issue-23302-1.rs stdout ----
[00:47:21] diff of stderr:
[00:47:21] 
[00:47:21] 5    |         ^^^^^^^^^^^^^
[00:47:21] 6    |
[00:47:21] 7    = note: ...which again requires processing `X::A::{{constant}}`, completing the cycle
[00:47:21] + note: cycle used when const-evaluating `X::A::{{constant}}`
[00:47:21] +   --> $DIR/issue-23302-1.rs:14:9
[00:47:21] +    |
[00:47:21] + LL |     A = X::A as isize, //~ ERROR E0391
[00:47:21] 8 
[00:47:21] 9 error: aborting due to previous error
[00:47:21] 10 
[00:47:21] 
[00:47:21] 
[00:47:21] 
[00:47:21] The actual stderr differed from the expected stderr.
[00:47:21] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23302-1/issue-23302-1.stderr
[00:47:21] To update references, rerun the tests and pass the `--bless` flag
[00:47:21] To only update this specific test, also pass `--test-args issues/issue-23302-1.rs`
[00:47:21] error: 1 errors occurred comparing output.
[00:47:21] status: exit code: 1
[00:47:21] status: exit code: 1
[00:47:21] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-23302-1.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23302-1/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23302-1/auxiliary" "-A" "unused"
[00:47:21] ------------------------------------------
[00:47:21] 
[00:47:21] ------------------------------------------
[00:47:21] stderr:
[00:47:21] stderr:
[00:47:21] ------------------------------------------
[00:47:21] {"message":"cycle detected when processing `X::A::{{constant}}`","code":{"code":"E0391","explanation":"\nThis error indicates that some types or traits depend on each other\nand therefore cannot be constructed.\n\nThe following example contains a circular dependency between two traits:\n\n