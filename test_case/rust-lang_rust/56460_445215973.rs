plain
[01:07:48] 
[01:07:48] ---- [ui (nll)] ui/nll/issue-55850.rs stdout ----
[01:07:48] diff of stderr:
[01:07:48] 
[01:07:48] 1 error[E0597]: `s` does not live long enough
[01:07:48] 3    |
[01:07:48] 3    |
[01:07:48] - LL |         yield &s[..]
[01:07:48] + LL |         yield &s[..] //~ ERROR `s` does not live long enough [E0597]
[01:07:48] 5    |                ^ borrowed value does not live long enough
[01:07:48] 6 LL |     })
[01:07:48] 7    |     - `s` dropped here while still borrowed
[01:07:48] 
[01:07:48] 9 error[E0626]: borrow may still be in use when generator yields
[01:07:48] 11    |
[01:07:48] 11    |
[01:07:48] - LL |         yield &s[..]
[01:07:48] + LL |         yield &s[..] //~ ERROR `s` does not live long enough [E0597]
[01:07:48] 13    |         -------^---- possible yield occurs here
[01:07:48] 15 error: aborting due to 2 previous errors
[01:07:48] 
[01:07:48] 
[01:07:48] The actual stderr differed from the expected stderr.
[01:07:48] The actual stderr differed from the expected stderr.
[01:07:48] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-55850.nll/issue-55850.nll.stderr
[01:07:48] To update references, rerun the tests and pass the `--bless` flag
[01:07:48] To only update this specific test, also pass `--test-args nll/issue-55850.rs`
[01:07:48] error: 1 errors occurred comparing output.
[01:07:48] status: exit code: 1
[01:07:48] status: exit code: 1
[01:07:48] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-55850.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-55850.nll/a" "-Zborrowck=migrate" "-Ztwo-phase-borrows" "-Crpath" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-55850.nll/auxiliary" "-A" "unused"
[01:07:48] ------------------------------------------
[01:07:48] 
[01:07:48] ------------------------------------------
[01:07:48] stderr:
[01:07:48] stderr:
[01:07:48] ------------------------------------------
[01:07:48] {"message":"`s` does not live long enough","code":{"code":"E0597","explanation":"\nThis error occurs because a borrow was made inside a variable which has a\ngreater lifetime than the borrowed one.\n\nExample of erroneous code:\n\n