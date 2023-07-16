plain
[00:49:12] 
[00:49:12] ---- [ui] ui/consts/static_mut_containing_mut_ref2.rs stdout ----
[00:49:12] diff of stderr:
[00:49:12] 
[00:49:12] 4 LL | pub static mut STDERR_BUFFER: () = unsafe { *(&mut STDERR_BUFFER_SPACE) = 42; };
[00:49:12] 5    |                                              ^^^^^^^^^^^^^^^^^^^^^^^^^^ statics require immutable values
[00:49:12] 6 
[00:49:12] - error[E0019]: static contains unimplemented expression type
[00:49:12] -    |
[00:49:12] -    |
[00:49:12] - LL | pub static mut STDERR_BUFFER: () = unsafe { *(&mut STDERR_BUFFER_SPACE) = 42; };
[00:49:12] + error: aborting due to previous error
[00:49:12] 12 
[00:49:12] - error: aborting due to 2 previous errors
[00:49:12] - 
[00:49:12] - 
[00:49:12] - Some errors occurred: E0017, E0019.
[00:49:12] - For more information about an error, try `rustc --explain E0017`.
[00:49:12] + For more information about this error, try `rustc --explain E0017`.
[00:49:12] 17 
[00:49:12] 
[00:49:12] 
[00:49:12] The actual stderr differed from the expected stderr.
[00:49:12] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/static_mut_containing_mut_ref2/static_mut_containing_mut_ref2.stderr
[00:49:12] To update references, rerun the tests and pass the `--bless` flag
[00:49:12] To only update this specific test, also pass `--test-args consts/static_mut_containing_mut_ref2.rs`
[00:49:12] error: 1 errors occurred comparing output.
[00:49:12] status: exit code: 1
[00:49:12] status: exit code: 1
[00:49:12] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/static_mut_containing_mut_ref2.rs" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/static_mut_containing_mut_ref2/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/static_mut_containing_mut_ref2/auxiliary" "-A" "unused"
[00:49:12] ------------------------------------------
[00:49:12] 
[00:49:12] ------------------------------------------
[00:49:12] stderr:
[00:49:12] stderr:
[00:49:12] ------------------------------------------
[00:49:12] {"message":"references in statics may only refer to immutable values","code":{"code":"E0017","explanation":"\nReferences in statics and constants may only refer to immutable values.\nErroneous code example:\n\n