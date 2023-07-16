
2020-03-06T13:39:14.1294778Z 52     let f: extern "C" fn(*mut i32) = transmute(foo as extern "C" fn(_));
2020-03-06T13:39:14.1295058Z 
2020-03-06T13:39:14.1295225Z The actual stdout differed from the expected stdout.
2020-03-06T13:39:14.1296052Z Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/explain/explain.stdout
2020-03-06T13:39:14.1296052Z Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/explain/explain.stdout
2020-03-06T13:39:14.1296539Z To update references, rerun the tests and pass the `--bless` flag
2020-03-06T13:39:14.1296998Z To only update this specific test, also pass `--test-args explain.rs`
2020-03-06T13:39:14.1297434Z error: 1 errors occurred comparing output.
2020-03-06T13:39:14.1297629Z status: exit code: 0
2020-03-06T13:39:14.1297629Z status: exit code: 0
2020-03-06T13:39:14.1299318Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/explain.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/explain" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "--explain" "E0591" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/explain/auxiliary"
2020-03-06T13:39:14.1300723Z ------------------------------------------
2020-03-06T13:39:14.1300723Z ------------------------------------------
2020-03-06T13:39:14.1301009Z Per [RFC 401][rfc401], if you have a function declaration `foo`:
2020-03-06T13:39:14.1301341Z 