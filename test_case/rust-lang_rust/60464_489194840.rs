plain
travis_time:end:00f828cf:start=1556901413060378383,finish=1556901413853884658,duration=793506275
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:24:57] 
[01:24:57] running 9 tests
[01:24:57] iiiiiiiii
[01:24:57] 
[01:24:57]  finished in 0.167
[01:24:57] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:25:13] 
[01:25:13] running 121 tests
[01:25:40] .iiiii...i.....i..i...i..i.i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i. 100/121
[01:25:45] i.i......iii.i.....ii
[01:25:45] 
[01:25:45]  finished in 31.491
[01:25:45] travis_fold:end:test_debuginfo

---
[01:50:57] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:519:22
[01:50:57] ---- [ui] rustdoc-ui/cfg-test.rs stdout ----
[01:50:57] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:50:57] 
[01:50:57] error: test compilation failed although it shouldn't!
[01:50:57] status: exit code: 1
[01:50:57] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/cfg-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/cfg-test" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/cfg-test/auxiliary"
[01:50:57] ------------------------------------------
[01:50:57] 
[01:50:57] ------------------------------------------
[01:50:57] stderr:
[01:50:57] stderr:
[01:50:57] ------------------------------------------
[01:50:57] error: Unrecognized option: 'out-dir'
[01:50:57] 
[01:50:57] ------------------------------------------
[01:50:57] 
[01:50:57] 
[01:50:57] 
[01:50:57] ---- [ui] rustdoc-ui/coverage/basic.rs stdout ----
[01:50:57] 
[01:50:57] error: test compilation failed although it shouldn't!
[01:50:57] status: exit code: 1
[01:50:57] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/coverage/basic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/coverage/basic" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unstable-options" "--show-coverage" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/coverage/basic/auxiliary"
[01:50:57] ------------------------------------------
[01:50:57] 
[01:50:57] ------------------------------------------
[01:50:57] stderr:
[01:50:57] stderr:
[01:50:57] ------------------------------------------
[01:50:57] error: Unrecognized option: 'out-dir'
[01:50:57] 
[01:50:57] ------------------------------------------
[01:50:57] 
[01:50:57] 
[01:50:57] 
[01:50:57] ---- [ui] rustdoc-ui/coverage/enums.rs stdout ----
[01:50:57] 
[01:50:57] error: test compilation failed although it shouldn't!
[01:50:57] status: exit code: 1
[01:50:57] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/coverage/enums.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/coverage/enums" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unstable-options" "--show-coverage" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/coverage/enums/auxiliary"
[01:50:57] ------------------------------------------
[01:50:57] 
[01:50:57] ------------------------------------------
[01:50:57] stderr:
[01:50:57] stderr:
[01:50:57] ------------------------------------------
[01:50:57] error: Unrecognized option: 'out-dir'
[01:50:57] 
[01:50:57] ------------------------------------------
[01:50:57] 
[01:50:57] 
[01:50:57] 
[01:50:57] ---- [ui] rustdoc-ui/coverage/empty.rs stdout ----
[01:50:57] 
[01:50:57] error: test compilation failed although it shouldn't!
[01:50:57] status: exit code: 1
[01:50:57] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/coverage/empty.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/coverage/empty" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unstable-options" "--show-coverage" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/coverage/empty/auxiliary"
[01:50:57] ------------------------------------------
[01:50:57] 
[01:50:57] ------------------------------------------
[01:50:57] stderr:
[01:50:57] stderr:
[01:50:57] ------------------------------------------
[01:50:57] error: Unrecognized option: 'out-dir'
[01:50:57] 
[01:50:57] ------------------------------------------
[01:50:57] 
[01:50:57] 
[01:50:57] 
[01:50:57] ---- [ui] rustdoc-ui/coverage/private.rs stdout ----
[01:50:57] 
[01:50:57] error: test compilation failed although it shouldn't!
[01:50:57] status: exit code: 1
[01:50:57] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/coverage/private.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/coverage/private" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unstable-options" "--show-coverage" "--document-private-items" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/coverage/private/auxiliary"
[01:50:57] ------------------------------------------
[01:50:57] 
[01:50:57] ------------------------------------------
[01:50:57] stderr:
[01:50:57] stderr:
[01:50:57] ------------------------------------------
[01:50:57] error: Unrecognized option: 'out-dir'
[01:50:57] 
[01:50:57] ------------------------------------------
[01:50:57] 
[01:50:57] 
[01:50:57] 
[01:50:57] ---- [ui] rustdoc-ui/coverage/exotic.rs stdout ----
[01:50:57] 
[01:50:57] error: test compilation failed although it shouldn't!
[01:50:57] status: exit code: 1
[01:50:57] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/coverage/exotic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/coverage/exotic" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unstable-options" "--show-coverage" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/coverage/exotic/auxiliary"
[01:50:57] ------------------------------------------
[01:50:57] 
[01:50:57] ------------------------------------------
[01:50:57] stderr:
[01:50:57] stderr:
[01:50:57] ------------------------------------------
[01:50:57] error: Unrecognized option: 'out-dir'
[01:50:57] 
[01:50:57] ------------------------------------------
[01:50:57] 
[01:50:57] 
[01:50:57] 
[01:50:57] ---- [ui] rustdoc-ui/coverage/statics-consts.rs stdout ----
[01:50:57] 
[01:50:57] error: test compilation failed although it shouldn't!
[01:50:57] status: exit code: 1
[01:50:57] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/coverage/statics-consts.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/coverage/statics-consts" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unstable-options" "--show-coverage" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/coverage/statics-consts/auxiliary"
[01:50:57] ------------------------------------------
[01:50:57] 
[01:50:57] ------------------------------------------
[01:50:57] stderr:
[01:50:57] stderr:
[01:50:57] ------------------------------------------
[01:50:57] error: Unrecognized option: 'out-dir'
[01:50:57] 
[01:50:57] ------------------------------------------
[01:50:57] 
[01:50:57] 
[01:50:57] 
[01:50:57] ---- [ui] rustdoc-ui/coverage/traits.rs stdout ----
[01:50:57] 
[01:50:57] error: test compilation failed although it shouldn't!
[01:50:57] status: exit code: 1
[01:50:57] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/coverage/traits.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/coverage/traits" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unstable-options" "--show-coverage" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/coverage/traits/auxiliary"
[01:50:57] ------------------------------------------
[01:50:57] 
[01:50:57] ------------------------------------------
[01:50:57] stderr:
[01:50:57] stderr:
[01:50:57] ------------------------------------------
[01:50:57] error: Unrecognized option: 'out-dir'
[01:50:57] 
[01:50:57] ------------------------------------------
[01:50:57] 
[01:50:57] 
[01:50:57] 
[01:50:57] ---- [ui] rustdoc-ui/deny-missing-docs-crate.rs stdout ----
[01:50:57] diff of stderr:
[01:50:57] 
[01:50:57] - error: missing documentation for crate
[01:50:57] -   --> $DIR/deny-missing-docs-crate.rs:1:1
[01:50:57] -    |
[01:50:57] - LL | / #![deny(missing_docs)]
[01:50:57] - LL | |
[01:50:57] - LL | | pub struct Foo;
[01:50:57] -    |
[01:50:57] - note: lint level defined here
[01:50:57] -   --> $DIR/deny-missing-docs-crate.rs:1:9
[01:50:57] -    |
[01:50:57] -    |
[01:50:57] - LL | #![deny(missing_docs)]
[01:50:57] - 
[01:50:57] - error: missing documentation for a struct
[01:50:57] -   --> $DIR/deny-missing-docs-crate.rs:3:1
[01:50:57] -    |
[01:50:57] -    |
[01:50:57] - LL | pub struct Foo;
[01:50:57] -    | ^^^^^^^^^^^^^^^
[01:50:57] - 
[01:50:57] - error: aborting due to 2 previous errors
[01:50:57] + error: Unrecognized option: 'out-dir'
[01:50:57] 23 
[01:50:57] 
[01:50:57] 
[01:50:57] The actual stderr differed from the expected stderr.
[01:50:57] The actual stderr differed from the expected stderr.
[01:50:57] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/deny-missing-docs-crate/deny-missing-docs-crate.stderr
[01:50:57] To update references, rerun the tests and pass the `--bless` flag
[01:50:57] To only update this specific test, also pass `--test-args deny-missing-docs-crate.rs`
[01:50:57] error: 1 errors occurred comparing output.
[01:50:57] status: exit code: 1
[01:50:57] status: exit code: 1
[01:50:57] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/deny-missing-docs-crate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/deny-missing-docs-crate" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/deny-missing-docs-crate/auxiliary"
[01:50:57] ------------------------------------------
[01:50:57] 
[01:50:57] ------------------------------------------
[01:50:57] stderr:
[01:50:57] stderr:
[01:50:57] ------------------------------------------
[01:50:57] error: Unrecognized option: 'out-dir'
[01:50:57] 
[01:50:57] ------------------------------------------
[01:50:57] 
[01:50:57] 
[01:50:57] 
[01:50:57] ---- [ui] rustdoc-ui/deny-missing-docs-macro.rs stdout ----
[01:50:57] diff of stderr:
[01:50:57] 
[01:50:57] - error: missing documentation for macro
[01:50:57] -   --> $DIR/deny-missing-docs-macro.rs:6:1
[01:50:57] -    |
[01:50:57] - LL | macro_rules! foo {
[01:50:57] -    |
[01:50:57] - note: lint level defined here
[01:50:57] -   --> $DIR/deny-missing-docs-macro.rs:3:9
[01:50:57] -    |
[01:50:57] -    |
[01:50:57] - LL | #![deny(missing_docs)]
[01:50:57] - 
[01:50:57] - error: aborting due to previous error
[01:50:57] - error: aborting due to previous error
[01:50:57] + error: Unrecognized option: 'out-dir'
[01:50:57] 15 
[01:50:57] 
[01:50:57] 
[01:50:57] The actual stderr differed from the expected stderr.
[01:50:57] The actual stderr differed from the expected stderr.
[01:50:57] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/deny-missing-docs-macro/deny-missing-docs-macro.stderr
[01:50:57] To update references, rerun the tests and pass the `--bless` flag
[01:50:57] To only update this specific test, also pass `--test-args deny-missing-docs-macro.rs`
[01:50:57] error: 1 errors occurred comparing output.
[01:50:57] status: exit code: 1
[01:50:57] status: exit code: 1
[01:50:57] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/deny-missing-docs-macro.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/deny-missing-docs-macro" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/deny-missing-docs-macro/auxiliary"
[01:50:57] ------------------------------------------
[01:50:57] 
[01:50:57] ------------------------------------------
[01:50:57] stderr:
[01:50:57] stderr:
[01:50:57] ------------------------------------------
[01:50:57] error: Unrecognized option: 'out-dir'
[01:50:57] 
[01:50:57] ------------------------------------------
[01:50:57] 
[01:50:57] 
[01:50:57] 
[01:50:57] ---- [ui] rustdoc-ui/deny-intra-link-resolution-failure.rs stdout ----
[01:50:57] diff of stderr:
[01:50:57] 
[01:50:57] - error: `[v2]` cannot be resolved, ignoring it...
[01:50:57] -   --> $DIR/deny-intra-link-resolution-failure.rs:3:6
[01:50:57] - LL | /// [v2]
[01:50:57] -    |      ^^ cannot be resolved, ignoring
[01:50:57] -    |
[01:50:57] - note: lint level defined here
[01:50:57] - note: lint level defined here
[01:50:57] -   --> $DIR/deny-intra-link-resolution-failure.rs:1:9
[01:50:57] -    |
[01:50:57] - LL | #![deny(intra_doc_link_resolution_failure)]
[01:50:57] -    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:50:57] -    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:50:57] - 
[01:50:57] - error: aborting due to previous error
[01:50:57] + error: Unrecognized option: 'out-dir'
[01:50:57] 16 
[01:50:57] 
[01:50:57] 
[01:50:57] The actual stderr differed from the expected stderr.
[01:50:57] The actual stderr differed from the expected stderr.
[01:50:57] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/deny-intra-link-resolution-failure/deny-intra-link-resolution-failure.stderr
[01:50:57] To update references, rerun the tests and pass the `--bless` flag
[01:50:57] To only update this specific test, also pass `--test-args deny-intra-link-resolution-failure.rs`
[01:50:57] error: 1 errors occurred comparing output.
[01:50:57] status: exit code: 1
[01:50:57] status: exit code: 1
[01:50:57] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/deny-intra-link-resolution-failure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/deny-intra-link-resolution-failure" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/deny-intra-link-resolution-failure/auxiliary"
[01:50:57] ------------------------------------------
[01:50:57] 
[01:50:57] ------------------------------------------
[01:50:57] stderr:
[01:50:57] stderr:
[01:50:57] ------------------------------------------
[01:50:57] error: Unrecognized option: 'out-dir'
[01:50:57] 
[01:50:57] ------------------------------------------
[01:50:57] 
[01:50:57] 
[01:50:57] 
[01:50:57] ---- [ui] rustdoc-ui/deprecated-attrs.rs stdout ----
[01:50:57] 
[01:50:57] error: test compilation failed although it shouldn't!
[01:50:57] status: exit code: 1
[01:50:57] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/deprecated-attrs.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/deprecated-attrs" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/deprecated-attrs/auxiliary"
[01:50:57] ------------------------------------------
[01:50:57] 
[01:50:57] ------------------------------------------
[01:50:57] stderr:
[01:50:57] stderr:
[01:50:57] ------------------------------------------
[01:50:57] error: Unrecognized option: 'out-dir'
[01:50:57] 
[01:50:57] ------------------------------------------
[01:50:57] 
[01:50:57] 
---
[01:50:57] -    |
[01:50:57] - note: lint level defined here
[01:50:57] -   --> $DIR/doc-without-codeblock.rs:3:9
[01:50:57] -    |
[01:50:57] - LL | #![deny(missing_doc_code_examples)]
[01:50:57] - 
[01:50:57] - error: Missing code example in this documentation
[01:50:57] -   --> $DIR/doc-without-codeblock.rs:5:1
[01:50:57] -    |
[01:50:57] -    |
[01:50:57] - LL | /// Some docs.
[01:50:57] -    | ^^^^^^^^^^^^^^
[01:50:57] - 
[01:50:57] - error: Missing code example in this documentation
[01:50:57] -   --> $DIR/doc-without-codeblock.rs:9:1
[01:50:57] -    |
[01:50:57] - LL | /// And then, the princess died.
[01:50:57] - 
[01:50:57] - error: Missing code example in this documentation
[01:50:57] -   --> $DIR/doc-without-codeblock.rs:12:5
[01:50:57] -    |
[01:50:57] -    |
[01:50:57] - LL |     /// Or maybe not because she saved herself!
[01:50:57] - 
[01:50:57] - error: aborting due to 4 previous errors
[01:50:57] - error: aborting due to 4 previous errors
[01:50:57] + error: Unrecognized option: 'out-dir'
[01:50:57] 29 
[01:50:57] 
[01:50:57] 
[01:50:57] The actual stderr differed from the expected stderr.
[01:50:57] The actual stderr differed from the expected stderr.
[01:50:57] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doc-without-codeblock/doc-without-codeblock.stderr
[01:50:57] To update references, rerun the tests and pass the `--bless` flag
[01:50:57] To only update this specific test, also pass `--test-args doc-without-codeblock.rs`
[01:50:57] error: 1 errors occurred comparing output.
[01:50:57] status: exit code: 1
[01:50:57] status: exit code: 1
[01:50:57] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/doc-without-codeblock.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doc-without-codeblock" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doc-without-codeblock/auxiliary"
[01:50:57] ------------------------------------------
[01:50:57] 
[01:50:57] ------------------------------------------
[01:50:57] stderr:
[01:50:57] stderr:
[01:50:57] ------------------------------------------
[01:50:57] error: Unrecognized option: 'out-dir'
[01:50:57] 
[01:50:57] ------------------------------------------
[01:50:57] 
[01:50:57] 
[01:50:57] 
[01:50:57] ---- [ui] rustdoc-ui/intra-link-span-ice-55723.rs stdout ----
[01:50:57] diff of stderr:
[01:50:57] 
[01:50:57] - error: `[i]` cannot be resolved, ignoring it...
[01:50:57] -   --> $DIR/intra-link-span-ice-55723.rs:9:10
[01:50:57] -    |
[01:50:57] - LL | /// （arr[i]）
[01:50:57] -    |
[01:50:57] - note: lint level defined here
[01:50:57] -   --> $DIR/intra-link-span-ice-55723.rs:1:9
[01:50:57] -    |
[01:50:57] -    |
[01:50:57] - LL | #![deny(intra_doc_link_resolution_failure)]
[01:50:57] -    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:50:57] -    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:50:57] - 
[01:50:57] - error: aborting due to previous error
[01:50:57] + error: Unrecognized option: 'out-dir'
[01:50:57] 16 
[01:50:57] 
[01:50:57] 
[01:50:57] The actual stderr differed from the expected stderr.
[01:50:57] The actual stderr differed from the expected stderr.
[01:50:57] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-link-span-ice-55723/intra-link-span-ice-55723.stderr
[01:50:57] To update references, rerun the tests and pass the `--bless` flag
[01:50:57] To only update this specific test, also pass `--test-args intra-link-span-ice-55723.rs`
[01:50:57] error: 1 errors occurred comparing output.
[01:50:57] status: exit code: 1
[01:50:57] status: exit code: 1
[01:50:57] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/intra-link-span-ice-55723.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-link-span-ice-55723" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-link-span-ice-55723/auxiliary"
[01:50:57] ------------------------------------------
[01:50:57] 
[01:50:57] ------------------------------------------
[01:50:57] stderr:
[01:50:57] stderr:
[01:50:57] ------------------------------------------
[01:50:57] error: Unrecognized option: 'out-dir'
[01:50:57] 
[01:50:57] ------------------------------------------
[01:50:57] 
[01:50:57] 
[01:50:57] 
[01:50:57] ---- [ui] rustdoc-ui/failed-doctest-output.rs stdout ----
[01:50:57] 
[01:50:57] error: Error: expected failure status (Some(101)) but received status Some(1).
[01:50:57] status: exit code: 1
[01:50:57] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/failed-doctest-output.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/auxiliary"
[01:50:57] ------------------------------------------
[01:50:57] 
[01:50:57] ------------------------------------------
[01:50:57] stderr:
[01:50:57] stderr:
[01:50:57] ------------------------------------------
[01:50:57] error: Unrecognized option: 'out-dir'
[01:50:57] 
[01:50:57] ------------------------------------------
[01:50:57] 
[01:50:57] 
[01:50:57] 
[01:50:57] ---- [ui] rustdoc-ui/intra-doc-alias-ice.rs stdout ----
[01:50:57] diff of stderr:
[01:50:57] 
[01:50:57] - error: `[TypeAlias::hoge]` cannot be resolved, ignoring it...
[01:50:57] -    |
[01:50:57] -    |
[01:50:57] - LL | /// [broken cross-reference](TypeAlias::hoge)
[01:50:57] -    |
[01:50:57] - note: lint level defined here
[01:50:57] -   --> $DIR/intra-doc-alias-ice.rs:1:9
[01:50:57] -    |
[01:50:57] -    |
[01:50:57] - LL | #![deny(intra_doc_link_resolution_failure)]
[01:50:57] -    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:50:57] -    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:50:57] - 
[01:50:57] - error: aborting due to previous error
[01:50:57] + error: Unrecognized option: 'out-dir'
[01:50:57] 16 
[01:50:57] 
[01:50:57] 
[01:50:57] The actual stderr differed from the expected stderr.
[01:50:57] The actual stderr differed from the expected stderr.
[01:50:57] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-doc-alias-ice/intra-doc-alias-ice.stderr
[01:50:57] To update references, rerun the tests and pass the `--bless` flag
[01:50:57] To only update this specific test, also pass `--test-args intra-doc-alias-ice.rs`
[01:50:57] error: 1 errors occurred comparing output.
[01:50:57] status: exit code: 1
[01:50:57] status: exit code: 1
[01:50:57] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/intra-doc-alias-ice.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-doc-alias-ice" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-doc-alias-ice/auxiliary"
[01:50:57] ------------------------------------------
[01:50:57] 
[01:50:57] ------------------------------------------
[01:50:57] stderr:
[01:50:57] stderr:
[01:50:57] ------------------------------------------
[01:50:57] error: Unrecognized option: 'out-dir'
[01:50:57] 
[01:50:57] ------------------------------------------
[01:50:57] 
[01:50:57] 
[01:50:57] 
[01:50:57] ---- [ui] rustdoc-ui/intra-links-warning-crlf.rs stdout ----
[01:50:57] 
[01:50:57] error: test compilation failed although it shouldn't!
[01:50:57] status: exit code: 1
[01:50:57] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/intra-links-warning-crlf.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-links-warning-crlf" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-links-warning-crlf/auxiliary"
[01:50:57] ------------------------------------------
[01:50:57] 
[01:50:57] ------------------------------------------
[01:50:57] stderr:
[01:50:57] stderr:
[01:50:57] ------------------------------------------
[01:50:57] error: Unrecognized option: 'out-dir'
[01:50:57] 
[01:50:57] ------------------------------------------
[01:50:57] 
[01:50:57] 
[01:50:57] 
[01:50:57] ---- [ui] rustdoc-ui/intra-links-warning.rs stdout ----
[01:50:57] 
[01:50:57] error: test compilation failed although it shouldn't!
[01:50:57] status: exit code: 1
[01:50:57] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/intra-links-warning.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-links-warning" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-links-warning/auxiliary"
[01:50:57] ------------------------------------------
[01:50:57] 
[01:50:57] ------------------------------------------
[01:50:57] stderr:
[01:50:57] stderr:
[01:50:57] ------------------------------------------
[01:50:57] error: Unrecognized option: 'out-dir'
[01:50:57] 
[01:50:57] ------------------------------------------
[01:50:57] 
[01:50:57] 
[01:50:57] 
[01:50:57] ---- [ui] rustdoc-ui/invalid-syntax.rs stdout ----
[01:50:57] 
[01:50:57] error: test compilation failed although it shouldn't!
[01:50:57] status: exit code: 1
[01:50:57] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/invalid-syntax.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/invalid-syntax" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/invalid-syntax/auxiliary"
[01:50:57] ------------------------------------------
[01:50:57] 
[01:50:57] ------------------------------------------
[01:50:57] stderr:
[01:50:57] stderr:
[01:50:57] ------------------------------------------
[01:50:57] error: Unrecognized option: 'out-dir'
[01:50:57] 
[01:50:57] ------------------------------------------
[01:50:57] 
[01:50:57] 
[01:50:57] 
[01:50:57] ---- [ui] rustdoc-ui/intra-links-ambiguity.rs stdout ----
[01:50:57] diff of stderr:
[01:50:57] 
[01:50:57] - error: `ambiguous` is both a struct and a function
[01:50:57] -    |
[01:50:57] -    |
[01:50:57] - LL | /// [`ambiguous`] is ambiguous.
[01:50:57] -    |
[01:50:57] - note: lint level defined here
[01:50:57] -   --> $DIR/intra-links-ambiguity.rs:1:9
[01:50:57] -    |
[01:50:57] -    |
[01:50:57] - LL | #![deny(intra_doc_link_resolution_failure)]
[01:50:57] -    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:50:57] - help: to link to the struct, prefix with the item type
[01:50:57] -    |
[01:50:57] - LL | /// [`struct@ambiguous`] is ambiguous.
[01:50:57] - help: to link to the function, add parentheses
[01:50:57] -    |
[01:50:57] -    |
[01:50:57] - LL | /// [`ambiguous()`] is ambiguous.
[01:50:57] - 
[01:50:57] - 
[01:50:57] - error: `ambiguous` is both a struct and a function
[01:50:57] -    |
[01:50:57] -    |
[01:50:57] - LL | /// [ambiguous] is ambiguous.
[01:50:57] - help: to link to the struct, prefix with the item type
[01:50:57] -    |
[01:50:57] -    |
[01:50:57] - LL | /// [struct@ambiguous] is ambiguous.
[01:50:57] - help: to link to the function, add parentheses
[01:50:57] -    |
[01:50:57] -    |
[01:50:57] - LL | /// [ambiguous()] is ambiguous.
[01:50:57] - 
[01:50:57] - 
[01:50:57] - error: `multi_conflict` is a struct, a function, and a macro
[01:50:57] -    |
[01:50:57] -    |
[01:50:57] - LL | /// [`multi_conflict`] is a three-way conflict.
[01:50:57] - help: to link to the struct, prefix with the item type
[01:50:57] -    |
[01:50:57] -    |
[01:50:57] - LL | /// [`struct@multi_conflict`] is a three-way conflict.
[01:50:57] - help: to link to the function, add parentheses
[01:50:57] -    |
[01:50:57] -    |
[01:50:57] - LL | /// [`multi_conflict()`] is a three-way conflict.
[01:50:57] - help: to link to the macro, add an exclamation mark
[01:50:57] -    |
[01:50:57] -    |
[01:50:57] - LL | /// [`multi_conflict!`] is a three-way conflict.
[01:50:57] - 
[01:50:57] - 
[01:50:57] - error: `type_and_value` is both a module and a constant
[01:50:57] -    |
[01:50:57] -    |
[01:50:57] - LL | /// Ambiguous [type_and_value].
[01:50:57] - help: to link to the module, prefix with the item type
[01:50:57] -    |
[01:50:57] -    |
[01:50:57] - LL | /// Ambiguous [module@type_and_value].
[01:50:57] - help: to link to the constant, prefix with the item type
[01:50:57] -    |
[01:50:57] -    |
[01:50:57] - LL | /// Ambiguous [const@type_and_value].
[01:50:57] - 
[01:50:57] - 
[01:50:57] - error: `foo::bar` is both an enum and a function
[01:50:57] -    |
[01:50:57] -    |
[01:50:57] - LL | /// Ambiguous non-implied shortcut link [`foo::bar`].
[01:50:57] -    |                                          ^^^^^^^^^^ ambiguous link
[01:50:57] - help: to link to the enum, prefix with the item type
[01:50:57] -    |
[01:50:57] - LL | /// Ambiguous non-implied shortcut link [`enum@foo::bar`].
[01:50:57] - help: to link to the function, add parentheses
[01:50:57] -    |
[01:50:57] -    |
[01:50:57] - LL | /// Ambiguous non-implied shortcut link [`foo::bar()`].
[01:50:57] - 
[01:50:57] - error: aborting due to 5 previous errors
[01:50:57] - error: aborting due to 5 previous errors
[01:50:57] + error: Unrecognized option: 'out-dir'
[01:50:57] 83 
[01:50:57] 
[01:50:57] 
[01:50:57] The actual stderr differed from the expected stderr.
[01:50:57] The actual stderr differed from the expected stderr.
[01:50:57] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-links-ambiguity/intra-links-ambiguity.stderr
[01:50:57] To update references, rerun the tests and pass the `--bless` flag
[01:50:57] To only update this specific test, also pass `--test-args intra-links-ambiguity.rs`
[01:50:57] error: 1 errors occurred comparing output.
[01:50:57] status: exit code: 1
[01:50:57] status: exit code: 1
[01:50:57] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/intra-links-ambiguity.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-links-ambiguity" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-links-ambiguity/auxiliary"
[01:50:57] ------------------------------------------
[01:50:57] 
[01:50:57] ------------------------------------------
[01:50:57] stderr:
[01:50:57] stderr:
[01:50:57] ------------------------------------------
[01:50:57] error: Unrecognized option: 'out-dir'
[01:50:57] 
[01:50:57] ------------------------------------------
[01:50:57] 
[01:50:57] 
[01:50:57] 
[01:50:57] ---- [ui] rustdoc-ui/issue-58473.rs stdout ----
[01:50:57] 
[01:50:57] error: test compilation failed although it shouldn't!
[01:50:57] status: exit code: 1
[01:50:57] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/issue-58473.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/issue-58473" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/issue-58473/auxiliary"
[01:50:57] ------------------------------------------
[01:50:57] 
[01:50:57] ------------------------------------------
[01:50:57] stderr:
[01:50:57] stderr:
[01:50:57] ------------------------------------------
[01:50:57] error: Unrecognized option: 'out-dir'
[01:50:57] 
[01:50:57] ------------------------------------------
[01:50:57] 
[01:50:57] 
[01:50:57] 
[01:50:57] ---- [ui] rustdoc-ui/issue-58473-2.rs stdout ----
[01:50:57] 
[01:50:57] error: test compilation failed although it shouldn't!
[01:50:57] status: exit code: 1
[01:50:57] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/issue-58473-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/issue-58473-2" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/issue-58473-2/auxiliary"
[01:50:57] ------------------------------------------
[01:50:57] 
[01:50:57] ------------------------------------------
[01:50:57] stderr:
[01:50:57] stderr:
[01:50:57] ------------------------------------------
[01:50:57] error: Unrecognized option: 'out-dir'
[01:50:57] 
[01:50:57] ------------------------------------------
[01:50:57] 
[01:50:57] 
[01:50:57] 
[01:50:57] ---- [ui] rustdoc-ui/lint-group.rs stdout ----
[01:50:57] diff of stderr:
[01:50:57] 
[01:50:57] - error: Documentation test in private item
[01:50:57] -   --> $DIR/lint-group.rs:19:1
[01:50:57] -    |
[01:50:57] - LL | / /// wait, this *does* have a doctest?
[01:50:57] - LL | | ///
[01:50:57] - LL | | /// 