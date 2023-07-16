plain
[00:46:07] 
[00:46:07] ---- [ui] ui/issue-15919.rs stdout ----
[00:46:07] diff of stderr:
[00:46:07] 
[00:46:07] - error: the type `[usize; 18446744073709551615]` is too big for the current architecture
[00:46:07] + error: the type `[usize; 4294967295]` is too big for the current architecture
[00:46:07] 3 error: aborting due to previous error
[00:46:07] 4 
[00:46:07] 
[00:46:07] 
[00:46:07] 
[00:46:07] The actual stderr differed from the expected stderr.
[00:46:07] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-15919/issue-15919.stderr
[00:46:07] To update references, rerun the tests and pass the `--bless` flag
[00:46:07] To only update this specific test, also pass `--test-args issue-15919.rs`
[00:46:07] error: 1 errors occurred comparing output.
[00:46:07] status: exit code: 101
[00:46:07] status: exit code: 101
[00:46:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issue-15919.rs" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-15919/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-15919/auxiliary" "-A" "unused"
[00:46:07] ------------------------------------------
[00:46:07] 
[00:46:07] ------------------------------------------
[00:46:07] stderr:
[00:46:07] stderr:
[00:46:07] ------------------------------------------
[00:46:07] {"message":"the type `[usize; 4294967295]` is too big for the current architecture","code":null,"level":"error","spans":[],"children":[],"rendered":"error: the type `[usize; 4294967295]` is too big for the current architecture\n\n"}
[00:46:07] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:46:07] ------------------------------------------
[00:46:07] 
[00:46:07] thread '[ui] ui/issue-15919.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3162:9
[00:46:07] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:46:07] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:46:07] 
[00:46:07] ---- [ui] ui/issue-17913.rs stdout ----
[00:46:07] diff of stderr:
[00:46:07] 
[00:46:07] - error: the type `[&usize; 17293822569102704640]` is too big for the current architecture
[00:46:07] + error: the type `[&usize; 4294967295]` is too big for the current architecture
[00:46:07] 3 error: aborting due to previous error
[00:46:07] 4 
[00:46:07] 
[00:46:07] 
[00:46:07] 
[00:46:07] The actual stderr differed from the expected stderr.
[00:46:07] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-17913/issue-17913.stderr
[00:46:07] To update references, rerun the tests and pass the `--bless` flag
[00:46:07] To only update this specific test, also pass `--test-args issue-17913.rs`
[00:46:07] error: 1 errors occurred comparing output.
[00:46:07] status: exit code: 101
[00:46:07] status: exit code: 101
[00:46:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issue-17913.rs" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-17913/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-17913/auxiliary" "-A" "unused"
[00:46:07] ------------------------------------------
[00:46:07] 
[00:46:07] ------------------------------------------
[00:46:07] stderr:
[00:46:07] stderr:
[00:46:07] ------------------------------------------
[00:46:07] {"message":"the type `[&usize; 4294967295]` is too big for the current architecture","code":null,"level":"error","spans":[],"children":[],"rendered":"error: the type `[&usize; 4294967295]` is too big for the current architecture\n\n"}
[00:46:07] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:46:07] ------------------------------------------
[00:46:07] 
[00:46:07] thread '[ui] ui/issue-17913.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3162:9
[00:46:07] 
[00:46:07] 
[00:46:07] ---- [ui] ui/issue-28625.rs stdout ----
[00:46:07] diff of stderr:
[00:46:07] 
[00:46:07] 4 LL |     unsafe { std::mem::transmute(a) } //~ ERROR transmute called with types of different sizes
[00:46:07] 6    |
[00:46:07] 6    |
[00:46:07] -    = note: source type: &ArrayPeano<T> (64 bits)
[00:46:07] -    = note: target type: &[T] (128 bits)
[00:46:07] +    = note: source type: &ArrayPeano<T> (32 bits)
[00:46:07] +    = note: target type: &[T] (64 bits)
[00:46:07] 10 error: aborting due to previous error
[00:46:07] 11 
[00:46:07] 
[00:46:07] 
[00:46:07] 
[00:46:07] The actual stderr differed from the expected stderr.
[00:46:07] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-28625/issue-28625.stderr
[00:46:07] To update references, rerun the tests and pass the `--bless` flag
[00:46:07] To only update this specific test, also pass `--test-args issue-28625.rs`
[00:46:07] error: 1 errors occurred comparing output.
[00:46:07] status: exit code: 101
[00:46:07] status: exit code: 101
[00:46:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issue-28625.rs" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-28625/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-28625/auxiliary" "-A" "unused"
[00:46:07] ------------------------------------------
[00:46:07] 
[00:46:07] ------------------------------------------
[00:46:07] stderr:
[00:46:07] stderr:
[00:46:07] ------------------------------------------
[00:46:07] {"message":"transmute called with types of different sizes","code":{"code":"E0512","explanation":"\nTransmute with two differently sized types was attempted. Erroneous code\nexample:\n\n