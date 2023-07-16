\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/consts/const-match-check.rs","byte_start":1137,"byte_end":1138,et 0 = 0; 0 };
[00:51:08] 
[00:51:08] -    |                      ^ pattern `_` not covered
[00:51:08] +    |                      ^ pattern `-2147483648i32..=-1i32` not covered
[00:51:08] 6 
[00:51:08] - error[E0005]: refutable pattern in local binding: `_` not covered
[00:51:08] + error[E0005]: refutable pattern in local binding: `-2147483648i32..=-1i32` not covered
[00:51:08] 9    |
[00:51:08] 9    |
[00:51:08] 10 LL | static Y: i32 = { let 0 = 0; 0 };
[00:51:08] 
[00:51:08] -    |                       ^ pattern `_` not covered
[00:51:08] +    |                       ^ pattern `-2147483648i32..=-1i32` not covered
[00:51:08] 12 
[00:51:08] - error[E0005]: refutable pattern in local binding: `_` not covered
[00:51:08] + error[E0005]: refutable pattern in local binding: `-2147483648i32..=-1i32` not covered
[00:51:08] 15    |
[00:51:08] 15    |
[00:51:08] 16 LL |     const X: i32 = { let 0 = 0; 0 };
[00:51:08] 
[00:51:08] -    |                          ^ pattern `_` not covered
[00:51:08] +    |                          ^ pattern `-2147483648i32..=-1i32` not covered
[00:51:08] 18 
[00:51:08] - error[E0005]: refutable pattern in local binding: `_` not covered
[00:51:08] + error[E0005]: refutable pattern in local binding: `-2147483648i32..=-1i32` not covered
[00:51:08] 21    |
[00:51:08] 21    |
[00:51:08] 22 LL |     const X: i32 = { let 0 = 0; 0 };
[00:51:08] 
[00:51:08] -    |                          ^ pattern `_` not covered
[00:51:08] +    |                          ^ pattern `-2147483648i32..=-1i32` not covered
[00:51:08] 25 error: aborting due to 4 previous errors
[00:51:08] 26 
[00:51:08] 
[00:51:08] 
[00:51:08] 
[00:51:08] The actual stderr differed from the expected stderr.
[00:51:08] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-match-check.matchck/const-match-check.matchck.stderr
[00:51:08] To update references, rerun the tests and pass the `--bless` flag
[00:51:08] To only update this specific test, also pass `--test-args consts/const-match-check.rs`
[00:51:08] 
[00:51:08] error in revision `matchck`: 1 errors occurred comparing output.
[00:51:08] status: exit code: 1
[00:51:08] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-match-check.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "matchck" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-match-check.matchck/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-match-check.matchck/auxiliary" "-A" "unused"
[00:51:08] ------------------------------------------
[00:51:08] 
[00:51:08] ------------------------------------------
[00:51:08] stderr:
[00:51:08] stderr:
[00:51:08] ------------------------------------------
[00:51:08] {"message":"refutable pattern in local binding: `-2147483648i32..=-1i32` not covered","code":{"code":"E0005","explanation":"\nPatterns used to bind names must be irrefutable, that is, they mot covered\n