\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issue-28625.rs","byte_start":610,"byte_end":629,"line_start":20,"line_end":20,"column_start":14,"column_end":33,"is_primary":true,"text":[{"text":"    unsafe { std::mem::transmute(a) } //~ ERROR transmute called with types of different sizes","highlight_start":14,"highlight_end":33}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"source type: &ArrayPeano<T> (32 bits)","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"target type: &[T] (64 bits)","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0512]: transmute called with types of different sizes\n  --> /checkout/src/test/ui/issue-28625.rs:20:14\n   |\nLL |     unsafe { std::mem::transmute(a) } //~ ERROR transmute called with types of different sizes\n   |              ^^^^^^^^^^^^^^^^^^^\n   |\n   = note: source type: &ArrayPeano<T> (32 bits)\n   = note: target type: &[T] (64 bits)\n\n"}
[00:46:07] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:46:07] {"message":"For more information about this error, try `rustc --explain E0512`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0512`.\n"}
[00:46:07] ------------------------------------------
[00:46:07] 
[00:46:07] thread '[ui] ui/issue-28625.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3162:9
[00:46:07] 
[00:46:07] 
[00:46:07] ---- [ui] ui/issue-32377.rs stdout ----
[00:46:07] diff of stderr:
[00:46:07] 
[00:46:07] 4 LL |     unsafe { mem::transmute(x) }
[00:46:07] 6    |
[00:46:07] 6    |
[00:46:07] -    = note: source type: [usize; 2] (128 bits)
[00:46:07] +    = note: source type: [usize; 2] (64 bits)
[00:46:07] 8    = note: target type: Bar<U> (0 bits)
[00:46:07] 10 error: aborting due to previous error
[00:46:07] 
[00:46:07] 
[00:46:07] The actual stderr differed from the expected stderr.
[00:46:07] The actual stderr differed from the expected stderr.
[00:46:07] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-32377/issue-32377.stderr
[00:46:07] To update references, rerun the tests and pass the `--bless` flag
[00:46:07] To only update this specific test, also pass `--test-args issue-32377.rs`
[00:46:07] error: 1 errors occurred comparing output.
[00:46:07] status: exit code: 101
[00:46:07] status: exit code: 101
[00:46:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issue-32377.rs" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-32377/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-32377/auxiliary" "-A" "unused"
[00:46:07] ------------------------------------------
[00:46:07] 
[00:46:07] ------------------------------------------
[00:46:07] stderr:
[00:46:07] stderr:
[00:46:07] ------------------------------------------
[00:46:07] {"message":"transmute called with types of different sizes","code":{"code":"E0512","explanation":"\nTransmute with two differently sized types was attempted. Erroneous code\nexample:\n\n