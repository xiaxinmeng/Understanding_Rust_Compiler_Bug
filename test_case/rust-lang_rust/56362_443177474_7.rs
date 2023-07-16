\nlet x = Some(1);\n\nmatch x {\n    Some(y) [00:51:08] 3    |
[00:51:08] 4 LL |     let a = 4; //~ ERROR refutable pattern in local binding: `_` not covered
[00:51:08] 
[00:51:08] 5    |         ^ interpreted as a constant pattern, not new variable
[00:51:08] 6 
[00:51:08] - error[E0005]: refutable pattern in local binding: `_` not covered
[00:51:08] + error[E0005]: refutable pattern in local binding: `0u8..=1u8` not covered
[00:51:08] 9    |
[00:51:08] 9    |
[00:51:08] 10 LL |     let c = 4; //~ ERROR refutable pattern in local binding: `_` not covered
[00:51:08] 
[00:51:08] 11    |         ^ interpreted as a constant pattern, not new variable
[00:51:08] 12 
[00:51:08] - error[E0005]: refutable pattern in local binding: `_` not covered
[00:51:08] + error[E0005]: refutable pattern in local binding: `0u8..=1u8` not covered
[00:51:08] 15    |
[00:51:08] 15    |
[00:51:08] 16 LL |     let d = 4; //~ ERROR refutable pattern in local binding: `_` not covered
[00:51:08] 
[00:51:08] The actual stderr differed from the expected stderr.
[00:51:08] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-pattern-irrefutable/const-pattern-irrefutable.stderr
[00:51:08] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-pattern-irrefutable/const-pattern-irrefutable.stderr
[00:51:08] To update references, rerun the tests and pass the `--bless` flag
[00:51:08] To only update this specific test, also pass `--test-args consts/const-pattern-irrefutable.rs`
[00:51:08] error: 1 errors occurred comparing output.
[00:51:08] status: exit code: 1
[00:51:08] status: exit code: 1
[00:51:08] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/utable pattern in local binding: `_` not covered","highlight_start":9,"highlight_end":10}],"label":"interpreted as a constant pattern, not new variable","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0005]: refutable pattern in local binding: `0u8..=1u8` not covered\n  --> /checkout/src/test/ui/consts/const-pattern-irrefutable.rs:22:9\n   |\nLL |     let a = 4; //~ ERROR refutable pattern in local binding: `_` not covered\n   |         ^ interpreted as a constant pattern, not new variable\n\n"}
[00:51:08] {"message":"refutable pattern in local binding: `0u8..=1u8` not covered","code":{"code":"E0005","explanation":"\nPatterns used to bind names must be irrefutable, that is, they must guarantee\nthat a name will be extracted in all cases. Erroneous code example:\n\n