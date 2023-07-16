plain
[00:53:30] ....................................................................................................
[00:53:33] ....................................................................................................
[00:53:37] ......................................................i.............................................
[00:53:41] ....................................................................................................
[00:53:44] ..........................................................................F.........................
[00:53:48] ........i............................................................
[00:53:48] failures:
[00:53:48] 
[00:53:48] ---- [ui] ui/uninhabited/uninhabited-matches-feature-gated.rs stdout ----
[00:53:48] ---- [ui] ui/uninhabited/uninhabited-matches-feature-gated.rs stdout ----
[00:53:48] diff of stderr:
[00:53:48] 
[00:53:48] 16 LL |     let _ = match x {}; //~ ERROR non-exhaustive
[00:53:48] 18 
[00:53:48] 18 
[00:53:48] - error[E0004]: non-exhaustive patterns: type (Void,) is non-empty
[00:53:48] -    |
[00:53:48] -    |
[00:53:48] - LL |     let _ = match x {}; //~ ERROR non-exhaustive
[00:53:48] -    |
[00:53:48] -    |
[00:53:48] - help: Please ensure that all possible cases are being handled; possibly adding wildcards or more match arms.
[00:53:48] -    |
[00:53:48] -    |
[00:53:48] - LL |     let _ = match x {}; //~ ERROR non-exhaustive
[00:53:48] - 
[00:53:48] - 
[00:53:48] - error[E0004]: non-exhaustive patterns: type [Void; 1] is non-empty
[00:53:48] -    |
[00:53:48] -    |
[00:53:48] - LL |     let _ = match x {}; //~ ERROR non-exhaustive
[00:53:48] -    |
[00:53:48] -    |
[00:53:48] - help: Please ensure that all possible cases are being handled; possibly adding wildcards or more match arms.
[00:53:48] -    |
[00:53:48] -    |
[00:53:48] - LL |     let _ = match x {}; //~ ERROR non-exhaustive
[00:53:48] - 
[00:53:48] - 
[00:53:48] 43 error[E0004]: non-exhaustive patterns: `&[_]` not covered
[00:53:48] 45    |
[00:53:48] 
[00:53:48] 
[00:53:48] 58 LL |     let Ok(x) = x;
[00:53:48] 59    |         ^^^^^ pattern `Err(_)` not covered
[00:53:48] - error: aborting due to 7 previous errors
[00:53:48] + error: aborting due to 5 previous errors
[00:53:48] 62 
[00:53:48] 63 Some errors occurred: E0004, E0005.
[00:53:48] 63 Some errors occurred: E0004, E0005.
[00:53:48] 64 For more information about an error, try `rustc --explain E0004`.
[00:53:48] 
[00:53:48] 
[00:53:48] The actual stderr differed from the expected stderr.
[00:53:48] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/uninhabited/uninhabited-matches-feature-gated/uninhabited-matches-feature-gated.stderr
[00:53:48] To update references, rerun the tests and pass the `--bless` flag
[00:53:48] To only update this specific test, also pass `--test-args uninhabited/uninhabited-matches-feature-gated.rs`
[00:53:48] error: 1 errors occurred comparing output.
[00:53:48] status: exit code: 1
[00:53:48] status: exit code: 1
[00:53:48] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/uninhabited/uninhabited-matches-feature-gated.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/uninhabited/uninhabited-matches-feature-gated/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/uninhabited/uninhabited-matches-feature-gated/auxiliary" "-A" "unused"
[00:53:48] ------------------------------------------
[00:53:48] 
[00:53:48] ------------------------------------------
[00:53:48] stderr:
[00:53:48] stderr:
[00:53:48] ------------------------------------------
[00:53:48] {"message":"non-exhaustive patterns: `Err(_)` not covered","code":{"code":"E0004","explanation":"\nThis error indicates that the compiler cannot guarantee a matching pattern for\none or more possible inputs to a match expression. Guaranteed matches are\nrequired in order to assign values to match expressions, or alternatively,\ndetermine the flow of execution. Erroneous code example:\n\n