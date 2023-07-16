plain
travis_time:end:0f2cb20f:start=1556765899309998961,finish=1556765900131893490,duration=821894529
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
[01:18:06] 
[01:18:06] running 9 tests
[01:18:06] iiiiiiiii
[01:18:06] 
[01:18:06]  finished in 0.150
[01:18:06] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:18:22] 
[01:18:22] running 121 tests
[01:18:46] .iiiii...i.....i..i...i..i.i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i. 100/121
[01:18:51] i.i......iii.i.....ii
[01:18:51] 
[01:18:51]  finished in 29.288
[01:18:51] travis_fold:end:test_debuginfo

---
[01:42:22] failures:
[01:42:22] 
[01:42:22] ---- [ui] rustdoc-ui/coverage/enums.rs stdout ----
[01:42:22] 
[01:42:22] error: test compilation failed although it shouldn't!
[01:42:22] status: exit code: 1
[01:42:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/coverage/enums.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/coverage/enums" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unstable-options" "--show-coverage" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/coverage/enums/auxiliary"
[01:42:22] ------------------------------------------
[01:42:22] 
[01:42:22] ------------------------------------------
[01:42:22] stderr:
[01:42:22] stderr:
[01:42:22] ------------------------------------------
[01:42:22] error: Unrecognized option: 'out-dir'
[01:42:22] 
[01:42:22] ------------------------------------------
[01:42:22] 
[01:42:22] 
[01:42:22] 
[01:42:22] ---- [ui] rustdoc-ui/coverage/basic.rs stdout ----
[01:42:22] 
[01:42:22] error: test compilation failed although it shouldn't!
[01:42:22] status: exit code: 1
[01:42:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/coverage/basic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/coverage/basic" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unstable-options" "--show-coverage" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/coverage/basic/auxiliary"
[01:42:22] ------------------------------------------
[01:42:22] 
[01:42:22] ------------------------------------------
[01:42:22] stderr:
[01:42:22] stderr:
[01:42:22] ------------------------------------------
[01:42:22] error: Unrecognized option: 'out-dir'
[01:42:22] 
[01:42:22] ------------------------------------------
[01:42:22] 
[01:42:22] 
[01:42:22] 
[01:42:22] ---- [ui] rustdoc-ui/coverage/empty.rs stdout ----
[01:42:22] 
[01:42:22] error: test compilation failed although it shouldn't!
[01:42:22] status: exit code: 1
[01:42:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/coverage/empty.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/coverage/empty" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unstable-options" "--show-coverage" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/coverage/empty/auxiliary"
[01:42:22] ------------------------------------------
[01:42:22] 
[01:42:22] ------------------------------------------
[01:42:22] stderr:
[01:42:22] stderr:
[01:42:22] ------------------------------------------
[01:42:22] error: Unrecognized option: 'out-dir'
[01:42:22] 
[01:42:22] ------------------------------------------
[01:42:22] 
[01:42:22] 
[01:42:22] 
[01:42:22] ---- [ui] rustdoc-ui/cfg-test.rs stdout ----
[01:42:22] 
[01:42:22] error: test compilation failed although it shouldn't!
[01:42:22] status: exit code: 1
[01:42:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/cfg-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/cfg-test" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/cfg-test/auxiliary"
[01:42:22] ------------------------------------------
[01:42:22] 
[01:42:22] ------------------------------------------
[01:42:22] stderr:
[01:42:22] stderr:
[01:42:22] ------------------------------------------
[01:42:22] error: Unrecognized option: 'out-dir'
[01:42:22] 
[01:42:22] ------------------------------------------
[01:42:22] 
[01:42:22] 
[01:42:22] 
[01:42:22] ---- [ui] rustdoc-ui/coverage/exotic.rs stdout ----
[01:42:22] 
[01:42:22] error: test compilation failed although it shouldn't!
[01:42:22] status: exit code: 1
[01:42:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/coverage/exotic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/coverage/exotic" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unstable-options" "--show-coverage" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/coverage/exotic/auxiliary"
[01:42:22] ------------------------------------------
[01:42:22] 
[01:42:22] ------------------------------------------
[01:42:22] stderr:
[01:42:22] stderr:
[01:42:22] ------------------------------------------
[01:42:22] error: Unrecognized option: 'out-dir'
[01:42:22] 
[01:42:22] ------------------------------------------
[01:42:22] 
[01:42:22] 
[01:42:22] 
[01:42:22] ---- [ui] rustdoc-ui/coverage/private.rs stdout ----
[01:42:22] 
[01:42:22] error: test compilation failed although it shouldn't!
[01:42:22] status: exit code: 1
[01:42:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/coverage/private.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/coverage/private" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unstable-options" "--show-coverage" "--document-private-items" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/coverage/private/auxiliary"
[01:42:22] ------------------------------------------
[01:42:22] 
[01:42:22] ------------------------------------------
[01:42:22] stderr:
[01:42:22] stderr:
[01:42:22] ------------------------------------------
[01:42:22] error: Unrecognized option: 'out-dir'
[01:42:22] 
[01:42:22] ------------------------------------------
[01:42:22] 
[01:42:22] 
[01:42:22] 
[01:42:22] ---- [ui] rustdoc-ui/coverage/statics-consts.rs stdout ----
[01:42:22] 
[01:42:22] error: test compilation failed although it shouldn't!
[01:42:22] status: exit code: 1
[01:42:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/coverage/statics-consts.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/coverage/statics-consts" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unstable-options" "--show-coverage" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/coverage/statics-consts/auxiliary"
[01:42:22] ------------------------------------------
[01:42:22] 
[01:42:22] ------------------------------------------
[01:42:22] stderr:
[01:42:22] stderr:
[01:42:22] ------------------------------------------
[01:42:22] error: Unrecognized option: 'out-dir'
[01:42:22] 
[01:42:22] ------------------------------------------
[01:42:22] 
[01:42:22] 
[01:42:22] 
[01:42:22] ---- [ui] rustdoc-ui/coverage/traits.rs stdout ----
[01:42:22] 
[01:42:22] error: test compilation failed although it shouldn't!
[01:42:22] status: exit code: 1
[01:42:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/coverage/traits.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/coverage/traits" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unstable-options" "--show-coverage" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/coverage/traits/auxiliary"
[01:42:22] ------------------------------------------
[01:42:22] 
[01:42:22] ------------------------------------------
[01:42:22] stderr:
[01:42:22] stderr:
[01:42:22] ------------------------------------------
[01:42:22] error: Unrecognized option: 'out-dir'
[01:42:22] 
[01:42:22] ------------------------------------------
[01:42:22] 
[01:42:22] 
[01:42:22] 
[01:42:22] ---- [ui] rustdoc-ui/deny-missing-docs-macro.rs stdout ----
[01:42:22] diff of stderr:
[01:42:22] 
[01:42:22] - error: missing documentation for macro
[01:42:22] -   --> $DIR/deny-missing-docs-macro.rs:6:1
[01:42:22] -    |
[01:42:22] - LL | macro_rules! foo {
[01:42:22] -    |
[01:42:22] - note: lint level defined here
[01:42:22] -   --> $DIR/deny-missing-docs-macro.rs:3:9
[01:42:22] -    |
[01:42:22] -    |
[01:42:22] - LL | #![deny(missing_docs)]
[01:42:22] - 
[01:42:22] - error: aborting due to previous error
[01:42:22] - error: aborting due to previous error
[01:42:22] + error: Unrecognized option: 'out-dir'
[01:42:22] 15 
[01:42:22] 
[01:42:22] 
[01:42:22] The actual stderr differed from the expected stderr.
[01:42:22] The actual stderr differed from the expected stderr.
[01:42:22] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/deny-missing-docs-macro/deny-missing-docs-macro.stderr
[01:42:22] To update references, rerun the tests and pass the `--bless` flag
[01:42:22] To only update this specific test, also pass `--test-args deny-missing-docs-macro.rs`
[01:42:22] error: 1 errors occurred comparing output.
[01:42:22] status: exit code: 1
[01:42:22] status: exit code: 1
[01:42:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/deny-missing-docs-macro.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/deny-missing-docs-macro" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/deny-missing-docs-macro/auxiliary"
[01:42:22] ------------------------------------------
[01:42:22] 
[01:42:22] ------------------------------------------
[01:42:22] stderr:
[01:42:22] stderr:
[01:42:22] ------------------------------------------
[01:42:22] error: Unrecognized option: 'out-dir'
[01:42:22] 
[01:42:22] ------------------------------------------
[01:42:22] 
[01:42:22] 
[01:42:22] 
[01:42:22] ---- [ui] rustdoc-ui/deny-missing-docs-crate.rs stdout ----
[01:42:22] diff of stderr:
[01:42:22] 
[01:42:22] - error: missing documentation for crate
[01:42:22] -   --> $DIR/deny-missing-docs-crate.rs:1:1
[01:42:22] -    |
[01:42:22] - LL | / #![deny(missing_docs)]
[01:42:22] - LL | |
[01:42:22] - LL | | pub struct Foo;
[01:42:22] -    |
[01:42:22] - note: lint level defined here
[01:42:22] -   --> $DIR/deny-missing-docs-crate.rs:1:9
[01:42:22] -    |
[01:42:22] -    |
[01:42:22] - LL | #![deny(missing_docs)]
[01:42:22] - 
[01:42:22] - error: missing documentation for a struct
[01:42:22] -   --> $DIR/deny-missing-docs-crate.rs:3:1
[01:42:22] -    |
[01:42:22] -    |
[01:42:22] - LL | pub struct Foo;
[01:42:22] -    | ^^^^^^^^^^^^^^^
[01:42:22] - 
[01:42:22] - error: aborting due to 2 previous errors
[01:42:22] + error: Unrecognized option: 'out-dir'
[01:42:22] 23 
[01:42:22] 
[01:42:22] 
[01:42:22] The actual stderr differed from the expected stderr.
[01:42:22] The actual stderr differed from the expected stderr.
[01:42:22] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/deny-missing-docs-crate/deny-missing-docs-crate.stderr
[01:42:22] To update references, rerun the tests and pass the `--bless` flag
[01:42:22] To only update this specific test, also pass `--test-args deny-missing-docs-crate.rs`
[01:42:22] error: 1 errors occurred comparing output.
[01:42:22] status: exit code: 1
[01:42:22] status: exit code: 1
[01:42:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/deny-missing-docs-crate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/deny-missing-docs-crate" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/deny-missing-docs-crate/auxiliary"
[01:42:22] ------------------------------------------
[01:42:22] 
[01:42:22] ------------------------------------------
[01:42:22] stderr:
[01:42:22] stderr:
[01:42:22] ------------------------------------------
[01:42:22] error: Unrecognized option: 'out-dir'
[01:42:22] 
[01:42:22] ------------------------------------------
[01:42:22] 
[01:42:22] 
[01:42:22] 
[01:42:22] ---- [ui] rustdoc-ui/deny-intra-link-resolution-failure.rs stdout ----
[01:42:22] diff of stderr:
[01:42:22] 
[01:42:22] - error: `[v2]` cannot be resolved, ignoring it...
[01:42:22] -   --> $DIR/deny-intra-link-resolution-failure.rs:3:6
[01:42:22] - LL | /// [v2]
[01:42:22] -    |      ^^ cannot be resolved, ignoring
[01:42:22] -    |
[01:42:22] - note: lint level defined here
[01:42:22] - note: lint level defined here
[01:42:22] -   --> $DIR/deny-intra-link-resolution-failure.rs:1:9
[01:42:22] -    |
[01:42:22] - LL | #![deny(intra_doc_link_resolution_failure)]
[01:42:22] -    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:42:22] -    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:42:22] - 
[01:42:22] - error: aborting due to previous error
[01:42:22] + error: Unrecognized option: 'out-dir'
[01:42:22] 16 
[01:42:22] 
[01:42:22] 
[01:42:22] The actual stderr differed from the expected stderr.
[01:42:22] The actual stderr differed from the expected stderr.
[01:42:22] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/deny-intra-link-resolution-failure/deny-intra-link-resolution-failure.stderr
[01:42:22] To update references, rerun the tests and pass the `--bless` flag
[01:42:22] To only update this specific test, also pass `--test-args deny-intra-link-resolution-failure.rs`
[01:42:22] error: 1 errors occurred comparing output.
[01:42:22] status: exit code: 1
[01:42:22] status: exit code: 1
[01:42:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/deny-intra-link-resolution-failure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/deny-intra-link-resolution-failure" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/deny-intra-link-resolution-failure/auxiliary"
[01:42:22] ------------------------------------------
[01:42:22] 
[01:42:22] ------------------------------------------
[01:42:22] stderr:
[01:42:22] stderr:
[01:42:22] ------------------------------------------
[01:42:22] error: Unrecognized option: 'out-dir'
[01:42:22] 
[01:42:22] ------------------------------------------
[01:42:22] 
[01:42:22] 
[01:42:22] 
[01:42:22] ---- [ui] rustdoc-ui/deprecated-attrs.rs stdout ----
[01:42:22] 
[01:42:22] error: test compilation failed although it shouldn't!
[01:42:22] status: exit code: 1
[01:42:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/deprecated-attrs.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/deprecated-attrs" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/deprecated-attrs/auxiliary"
[01:42:22] ------------------------------------------
[01:42:22] 
[01:42:22] ------------------------------------------
[01:42:22] stderr:
[01:42:22] stderr:
[01:42:22] ------------------------------------------
[01:42:22] error: Unrecognized option: 'out-dir'
[01:42:22] 
[01:42:22] ------------------------------------------
[01:42:22] 
[01:42:22] 
[01:42:22] 
[01:42:22] ---- [ui] rustdoc-ui/failed-doctest-output.rs stdout ----
[01:42:22] 
[01:42:22] error: Error: expected failure status (Some(101)) but received status Some(1).
[01:42:22] status: exit code: 1
[01:42:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/failed-doctest-output.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/auxiliary"
[01:42:22] ------------------------------------------
[01:42:22] 
[01:42:22] ------------------------------------------
[01:42:22] stderr:
[01:42:22] stderr:
[01:42:22] ------------------------------------------
[01:42:22] error: Unrecognized option: 'out-dir'
[01:42:22] 
[01:42:22] ------------------------------------------
[01:42:22] 
[01:42:22] 
[01:42:22] 
[01:42:22] ---- [ui] rustdoc-ui/intra-doc-alias-ice.rs stdout ----
[01:42:22] diff of stderr:
[01:42:22] 
[01:42:22] - error: `[TypeAlias::hoge]` cannot be resolved, ignoring it...
[01:42:22] -    |
[01:42:22] -    |
[01:42:22] - LL | /// [broken cross-reference](TypeAlias::hoge)
[01:42:22] -    |
[01:42:22] - note: lint level defined here
[01:42:22] -   --> $DIR/intra-doc-alias-ice.rs:1:9
[01:42:22] -    |
[01:42:22] -    |
[01:42:22] - LL | #![deny(intra_doc_link_resolution_failure)]
[01:42:22] -    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:42:22] -    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:42:22] - 
[01:42:22] - error: aborting due to previous error
[01:42:22] + error: Unrecognized option: 'out-dir'
[01:42:22] 16 
[01:42:22] 
[01:42:22] 
[01:42:22] The actual stderr differed from the expected stderr.
[01:42:22] The actual stderr differed from the expected stderr.
[01:42:22] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-doc-alias-ice/intra-doc-alias-ice.stderr
[01:42:22] To update references, rerun the tests and pass the `--bless` flag
[01:42:22] To only update this specific test, also pass `--test-args intra-doc-alias-ice.rs`
[01:42:22] error: 1 errors occurred comparing output.
[01:42:22] status: exit code: 1
[01:42:22] status: exit code: 1
[01:42:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/intra-doc-alias-ice.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-doc-alias-ice" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-doc-alias-ice/auxiliary"
[01:42:22] ------------------------------------------
[01:42:22] 
[01:42:22] ------------------------------------------
[01:42:22] stderr:
[01:42:22] stderr:
[01:42:22] ------------------------------------------
[01:42:22] error: Unrecognized option: 'out-dir'
[01:42:22] 
[01:42:22] ------------------------------------------
[01:42:22] 
[01:42:22] 
---
[01:42:22] -    |
[01:42:22] - note: lint level defined here
[01:42:22] -   --> $DIR/doc-without-codeblock.rs:3:9
[01:42:22] -    |
[01:42:22] - LL | #![deny(missing_doc_code_examples)]
[01:42:22] - 
[01:42:22] - error: Missing code example in this documentation
[01:42:22] -   --> $DIR/doc-without-codeblock.rs:5:1
[01:42:22] -    |
[01:42:22] -    |
[01:42:22] - LL | /// Some docs.
[01:42:22] -    | ^^^^^^^^^^^^^^
[01:42:22] - 
[01:42:22] - error: Missing code example in this documentation
[01:42:22] -   --> $DIR/doc-without-codeblock.rs:9:1
[01:42:22] -    |
[01:42:22] - LL | /// And then, the princess died.
[01:42:22] - 
[01:42:22] - error: Missing code example in this documentation
[01:42:22] -   --> $DIR/doc-without-codeblock.rs:12:5
[01:42:22] -    |
[01:42:22] -    |
[01:42:22] - LL |     /// Or maybe not because she saved herself!
[01:42:22] - 
[01:42:22] - error: aborting due to 4 previous errors
[01:42:22] - error: aborting due to 4 previous errors
[01:42:22] + error: Unrecognized option: 'out-dir'
[01:42:22] 29 
[01:42:22] 
[01:42:22] 
[01:42:22] The actual stderr differed from the expected stderr.
[01:42:22] The actual stderr differed from the expected stderr.
[01:42:22] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doc-without-codeblock/doc-without-codeblock.stderr
[01:42:22] To update references, rerun the tests and pass the `--bless` flag
[01:42:22] To only update this specific test, also pass `--test-args doc-without-codeblock.rs`
[01:42:22] error: 1 errors occurred comparing output.
[01:42:22] status: exit code: 1
[01:42:22] status: exit code: 1
[01:42:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/doc-without-codeblock.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doc-without-codeblock" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doc-without-codeblock/auxiliary"
[01:42:22] ------------------------------------------
[01:42:22] 
[01:42:22] ------------------------------------------
[01:42:22] stderr:
[01:42:22] stderr:
[01:42:22] ------------------------------------------
[01:42:22] error: Unrecognized option: 'out-dir'
[01:42:22] 
[01:42:22] ------------------------------------------
[01:42:22] 
[01:42:22] 
[01:42:22] 
[01:42:22] ---- [ui] rustdoc-ui/intra-link-span-ice-55723.rs stdout ----
[01:42:22] diff of stderr:
[01:42:22] 
[01:42:22] - error: `[i]` cannot be resolved, ignoring it...
[01:42:22] -   --> $DIR/intra-link-span-ice-55723.rs:9:10
[01:42:22] -    |
[01:42:22] - LL | /// （arr[i]）
[01:42:22] -    |
[01:42:22] - note: lint level defined here
[01:42:22] -   --> $DIR/intra-link-span-ice-55723.rs:1:9
[01:42:22] -    |
[01:42:22] -    |
[01:42:22] - LL | #![deny(intra_doc_link_resolution_failure)]
[01:42:22] -    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:42:22] -    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:42:22] - 
[01:42:22] - error: aborting due to previous error
[01:42:22] + error: Unrecognized option: 'out-dir'
[01:42:22] 16 
[01:42:22] 
[01:42:22] 
[01:42:22] The actual stderr differed from the expected stderr.
[01:42:22] The actual stderr differed from the expected stderr.
[01:42:22] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-link-span-ice-55723/intra-link-span-ice-55723.stderr
[01:42:22] To update references, rerun the tests and pass the `--bless` flag
[01:42:22] To only update this specific test, also pass `--test-args intra-link-span-ice-55723.rs`
[01:42:22] error: 1 errors occurred comparing output.
[01:42:22] status: exit code: 1
[01:42:22] status: exit code: 1
[01:42:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/intra-link-span-ice-55723.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-link-span-ice-55723" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-link-span-ice-55723/auxiliary"
[01:42:22] ------------------------------------------
[01:42:22] 
[01:42:22] ------------------------------------------
[01:42:22] stderr:
[01:42:22] stderr:
[01:42:22] ------------------------------------------
[01:42:22] error: Unrecognized option: 'out-dir'
[01:42:22] 
[01:42:22] ------------------------------------------
[01:42:22] 
[01:42:22] 
[01:42:22] 
[01:42:22] ---- [ui] rustdoc-ui/intra-links-warning.rs stdout ----
[01:42:22] 
[01:42:22] error: test compilation failed although it shouldn't!
[01:42:22] status: exit code: 1
[01:42:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/intra-links-warning.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-links-warning" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-links-warning/auxiliary"
[01:42:22] ------------------------------------------
[01:42:22] 
[01:42:22] ------------------------------------------
[01:42:22] stderr:
[01:42:22] stderr:
[01:42:22] ------------------------------------------
[01:42:22] error: Unrecognized option: 'out-dir'
[01:42:22] 
[01:42:22] ------------------------------------------
[01:42:22] 
[01:42:22] 
[01:42:22] 
[01:42:22] ---- [ui] rustdoc-ui/intra-links-warning-crlf.rs stdout ----
[01:42:22] 
[01:42:22] error: test compilation failed although it shouldn't!
[01:42:22] status: exit code: 1
[01:42:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/intra-links-warning-crlf.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-links-warning-crlf" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-links-warning-crlf/auxiliary"
[01:42:22] ------------------------------------------
[01:42:22] 
[01:42:22] ------------------------------------------
[01:42:22] stderr:
[01:42:22] stderr:
[01:42:22] ------------------------------------------
[01:42:22] error: Unrecognized option: 'out-dir'
[01:42:22] 
[01:42:22] ------------------------------------------
[01:42:22] 
[01:42:22] 
[01:42:22] 
[01:42:22] ---- [ui] rustdoc-ui/intra-links-ambiguity.rs stdout ----
[01:42:22] diff of stderr:
[01:42:22] 
[01:42:22] - error: `ambiguous` is both a struct and a function
[01:42:22] -    |
[01:42:22] -    |
[01:42:22] - LL | /// [`ambiguous`] is ambiguous.
[01:42:22] -    |
[01:42:22] - note: lint level defined here
[01:42:22] -   --> $DIR/intra-links-ambiguity.rs:1:9
[01:42:22] -    |
[01:42:22] -    |
[01:42:22] - LL | #![deny(intra_doc_link_resolution_failure)]
[01:42:22] -    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:42:22] - help: to link to the struct, prefix with the item type
[01:42:22] -    |
[01:42:22] - LL | /// [`struct@ambiguous`] is ambiguous.
[01:42:22] - help: to link to the function, add parentheses
[01:42:22] -    |
[01:42:22] -    |
[01:42:22] - LL | /// [`ambiguous()`] is ambiguous.
[01:42:22] - 
[01:42:22] - 
[01:42:22] - error: `ambiguous` is both a struct and a function
[01:42:22] -    |
[01:42:22] -    |
[01:42:22] - LL | /// [ambiguous] is ambiguous.
[01:42:22] - help: to link to the struct, prefix with the item type
[01:42:22] -    |
[01:42:22] -    |
[01:42:22] - LL | /// [struct@ambiguous] is ambiguous.
[01:42:22] - help: to link to the function, add parentheses
[01:42:22] -    |
[01:42:22] -    |
[01:42:22] - LL | /// [ambiguous()] is ambiguous.
[01:42:22] - 
[01:42:22] - 
[01:42:22] - error: `multi_conflict` is a struct, a function, and a macro
[01:42:22] -    |
[01:42:22] -    |
[01:42:22] - LL | /// [`multi_conflict`] is a three-way conflict.
[01:42:22] - help: to link to the struct, prefix with the item type
[01:42:22] -    |
[01:42:22] -    |
[01:42:22] - LL | /// [`struct@multi_conflict`] is a three-way conflict.
[01:42:22] - help: to link to the function, add parentheses
[01:42:22] -    |
[01:42:22] -    |
[01:42:22] - LL | /// [`multi_conflict()`] is a three-way conflict.
[01:42:22] - help: to link to the macro, add an exclamation mark
[01:42:22] -    |
[01:42:22] -    |
[01:42:22] - LL | /// [`multi_conflict!`] is a three-way conflict.
[01:42:22] - 
[01:42:22] - 
[01:42:22] - error: `type_and_value` is both a module and a constant
[01:42:22] -    |
[01:42:22] -    |
[01:42:22] - LL | /// Ambiguous [type_and_value].
[01:42:22] - help: to link to the module, prefix with the item type
[01:42:22] -    |
[01:42:22] -    |
[01:42:22] - LL | /// Ambiguous [module@type_and_value].
[01:42:22] - help: to link to the constant, prefix with the item type
[01:42:22] -    |
[01:42:22] -    |
[01:42:22] - LL | /// Ambiguous [const@type_and_value].
[01:42:22] - 
[01:42:22] - 
[01:42:22] - error: `foo::bar` is both an enum and a function
[01:42:22] -    |
[01:42:22] -    |
[01:42:22] - LL | /// Ambiguous non-implied shortcut link [`foo::bar`].
[01:42:22] -    |                                          ^^^^^^^^^^ ambiguous link
[01:42:22] - help: to link to the enum, prefix with the item type
[01:42:22] -    |
[01:42:22] - LL | /// Ambiguous non-implied shortcut link [`enum@foo::bar`].
[01:42:22] - help: to link to the function, add parentheses
[01:42:22] -    |
[01:42:22] -    |
[01:42:22] - LL | /// Ambiguous non-implied shortcut link [`foo::bar()`].
[01:42:22] - 
[01:42:22] - error: aborting due to 5 previous errors
[01:42:22] - error: aborting due to 5 previous errors
[01:42:22] + error: Unrecognized option: 'out-dir'
[01:42:22] 83 
[01:42:22] 
[01:42:22] 
[01:42:22] The actual stderr differed from the expected stderr.
[01:42:22] The actual stderr differed from the expected stderr.
[01:42:22] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-links-ambiguity/intra-links-ambiguity.stderr
[01:42:22] To update references, rerun the tests and pass the `--bless` flag
[01:42:22] To only update this specific test, also pass `--test-args intra-links-ambiguity.rs`
[01:42:22] error: 1 errors occurred comparing output.
[01:42:22] status: exit code: 1
[01:42:22] status: exit code: 1
[01:42:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/intra-links-ambiguity.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-links-ambiguity" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-links-ambiguity/auxiliary"
[01:42:22] ------------------------------------------
[01:42:22] 
[01:42:22] ------------------------------------------
[01:42:22] stderr:
[01:42:22] stderr:
[01:42:22] ------------------------------------------
[01:42:22] error: Unrecognized option: 'out-dir'
[01:42:22] 
[01:42:22] ------------------------------------------
[01:42:22] 
[01:42:22] 
[01:42:22] 
[01:42:22] ---- [ui] rustdoc-ui/invalid-syntax.rs stdout ----
[01:42:22] 
[01:42:22] error: test compilation failed although it shouldn't!
[01:42:22] status: exit code: 1
[01:42:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/invalid-syntax.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/invalid-syntax" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/invalid-syntax/auxiliary"
[01:42:22] ------------------------------------------
[01:42:22] 
[01:42:22] ------------------------------------------
[01:42:22] stderr:
[01:42:22] stderr:
[01:42:22] ------------------------------------------
[01:42:22] error: Unrecognized option: 'out-dir'
[01:42:22] 
[01:42:22] ------------------------------------------
[01:42:22] 
[01:42:22] 
[01:42:22] 
[01:42:22] ---- [ui] rustdoc-ui/issue-58473.rs stdout ----
[01:42:22] 
[01:42:22] error: test compilation failed although it shouldn't!
[01:42:22] status: exit code: 1
[01:42:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/issue-58473.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/issue-58473" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/issue-58473/auxiliary"
[01:42:22] ------------------------------------------
[01:42:22] 
[01:42:22] ------------------------------------------
[01:42:22] stderr:
[01:42:22] stderr:
[01:42:22] ------------------------------------------
[01:42:22] error: Unrecognized option: 'out-dir'
[01:42:22] 
[01:42:22] ------------------------------------------
[01:42:22] 
[01:42:22] 
[01:42:22] 
[01:42:22] ---- [ui] rustdoc-ui/lint-group.rs stdout ----
[01:42:22] diff of stderr:
[01:42:22] 
[01:42:22] - error: Documentation test in private item
[01:42:22] -   --> $DIR/lint-group.rs:19:1
[01:42:22] -    |
[01:42:22] - LL | / /// wait, this *does* have a doctest?
[01:42:22] - LL | | ///
[01:42:22] - LL | | /// 