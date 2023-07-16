plain
travis_time:start:test_ui-fulldeps
Check compiletest suite=ui-fulldeps mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:51:44] 
[00:51:44] running 22 tests
[00:52:01] ..........F...........
[00:52:01] 
[00:52:01] ---- [ui] ui-fulldeps/lint-plugin-forbid-attrs.rs stdout ----
[00:52:01] diff of stderr:
[00:52:01] 
[00:52:01] 
[00:52:01] + error[E0453]: allow(test_lint) overruled by outer forbid(test_lint)
[00:52:01] +   --> $DIR/lint-plugin-forbid-attrs.rs:20:9
[00:52:01] +    |
[00:52:01] + LL | #![forbid(test_lint)]
[00:52:01] +    |           --------- `forbid` level set here
[00:52:01] + ...
[00:52:01] + LL | #[allow(test_lint)]
[00:52:01] +    |         ^^^^^^^^^ overruled by previous forbid
[00:52:01] + 
[00:52:01] 1 error: item is named 'lintme'
[00:52:01] 3    |
[00:52:01] 
[00:52:01] 9    |
[00:52:01] 9    |
[00:52:01] 10 LL | #![forbid(test_lint)]
[00:52:01] - 
[00:52:01] - 
[00:52:01] - error[E0453]: allow(test_lint) overruled by outer forbid(test_lint)
[00:52:01] -    |
[00:52:01] -    |
[00:52:01] - LL | #![forbid(test_lint)]
[00:52:01] -    |           --------- `forbid` level set here
[00:52:01] - ...
[00:52:01] - LL | #[allow(test_lint)]
[00:52:01] -    |         ^^^^^^^^^ overruled by previous forbid
[00:52:01] 22 error: aborting due to 2 previous errors
[00:52:01] 23 
[00:52:01] 
[00:52:01] 
[00:52:01] 
[00:52:01] The actual stderr differed from the expected stderr.
[00:52:01] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-attrs/lint-plugin-forbid-attrs.stderr
[00:52:01] To update references, rerun the tests and pass the `--bless` flag
[00:52:01] To only update this specific test, also pass `--test-args lint-plugin-forbid-attrs.rs`
[00:52:01] error: 1 errors occurred comparing output.
[00:52:01] status: exit code: 101
[00:52:01] status: exit code: 101
[00:52:01] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-plugin-forbid-attrs.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-attrs/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-attrs/auxiliary" "-A" "unused"
[00:52:01] ------------------------------------------
[00:52:01] 
[00:52:01] ------------------------------------------
[00:52:01] stderr:
[00:52:01] stderr:
[00:52:01] ------------------------------------------
[00:52:01] {"message":"allow(test_lint) overruled by outer forbid(test_lint)","code":{"code":"E0453","explanation":"\nA lint check attribute was overruled by a `forbid` directive set as an\nattribute on an enclosing scope, or on the command line with the `-F` option.\n\nExample of erroneous code:\n\n