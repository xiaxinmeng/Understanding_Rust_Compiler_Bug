
55 
56 The same applies to transmutes to `*mut fn()`, which were observed in practice.


The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/explain/explain.stdout
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/explain/explain.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args explain.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/explain.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/explain" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--explain" "E0591" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/explain/auxiliary"
------------------------------------------
------------------------------------------
Per [RFC 401][rfc401], if you have a function declaration `foo`:
