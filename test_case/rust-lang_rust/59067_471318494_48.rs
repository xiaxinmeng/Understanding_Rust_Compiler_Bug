\n"},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/nll/issue-55850.rs","byte_start":576,"byte_end":588,"line_start":28,"line_end":28,"column_start":9,"column_end":21,"is_primary":false,"text":[{"text":"        yield &s[..] //~ ERROR `s` does not live long enough [E0597]","highlight_start":9,"highlight_end":21}],"label":"possible yield occurs here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/issue-55850.rs","byte_start":583,"byte_end":584,"line_start":28,"line_end":28,"column_start":16,"column_end":17,"is_primary":true,"text":[{"text":"        yield &s[..] //~ ERROR `s` does not live long enough [E0597]","highlight_start":16,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this error has been downgraded to a warning for backwards compatibility with previous releases","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"this represents potential undefined behavior in your code and this warning will become a hard error in the future","code":null,"level":"warning","spans":[],"children":[],"rendered":null}],"rendered":"warning[E0626]: borrow may still be in use when generator yields\n  --> /checkout/src/test/ui/nll/issue-55850.rs:28:16\n   |\nLL |         yield &s[..] //~ ERROR `s` does not live long enough [E0597]\n   |         -------^---- possible yield occurs here\n   |\n   = warning: this error has been downgraded to a warning for backwards compatibility with previous releases\n   = warning: this represents potential undefined behavior in your code and this warning will become a hard error in the future\n\n"}
[01:38:31] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:38:31] make: *** [check] Error 1
[01:38:31] ------------------------------------------
[01:38:31] 
[01:38:31] thread '[ui (nll)] ui/nll/issue-55850.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3319:9
[01:38:31] 
[01:38:31] 
[01:38:31] ---- [ui (nll)] ui/regions/regions-ref-in-fn-arg.rs stdout ----
[01:38:31] diff of stderr:
[01:38:31] 
[01:38:31] 6 LL |     x //~^ ERROR borrowed value does not live long enough
[01:38:31] 7    |     ^ returns a value referencing data owned by the current function
[01:38:31] - error[E0515]: cannot return value referencing function parameter
[01:38:31] - error[E0515]: cannot return value referencing function parameter
[01:38:31] + warning[E0515]: cannot return value referencing function parameter
[01:38:31] 11    |
[01:38:31] 11    |
[01:38:31] 12 LL |     with(|box ref x| x) //~ ERROR borrowed value does not live long enough
[01:38:31] 
[01:38:31] 13    |           ---------  ^ returns a value referencing data owned by the current function
[01:38:31] 15    |           function parameter borrowed here
[01:38:31] +    |
[01:38:31] +    = warning: this error has been downgraded to a warning for backwards compatibility with previous releases
[01:38:31] +    = warning: this represents potential undefined behavior in your code and this warning will become a hard error in the future
---
[01:38:31] 20 
[01:38:31] 
[01:38:31] 
[01:38:31] The actual stderr differed from the expected stderr.
[01:38:31] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-ref-in-fn-arg.nll/regions-ref-in-fn-arg.nll.stderr
[01:38:31] To update references, rerun the tests and pass the `--bless` flag
[01:38:31] To only update this specific test, also pass `--test-args regions/regions-ref-in-fn-arg.rs`
[01:38:31] error: 1 errors occurred comparing output.
[01:38:31] status: exit code: 1
[01:38:31] status: exit code: 1
[01:38:31] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-ref-in-fn-arg.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-ref-in-fn-arg.nll/a" "-Zborrowck=migrate" "-Ztwo-phase-borrows" "-Crpath" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-ref-in-fn-arg.nll/auxiliary" "-A" "unused"
[01:38:31] ------------------------------------------
[01:38:31] 
[01:38:31] ------------------------------------------
[01:38:31] stderr:
[01:38:31] stderr:
[01:38:31] ------------------------------------------
[01:38:31] {"message":"cannot return value referencing function parameter","code":{"code":"E0515","explanation":"\nCannot return value that references local variable\n\nLocal variables, function parameters and temporaries are all dropped before the\nend of the function body. So a reference to them cannot be returned.\n\n