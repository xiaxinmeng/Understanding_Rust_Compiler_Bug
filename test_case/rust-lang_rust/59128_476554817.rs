plain
travis_time:end:172c67eb:start=1553587918897901069,finish=1553587993156616906,duration=74258715837
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:20:51] 
[01:20:51] running 9 tests
[01:20:51] iiiiiiiii
[01:20:51] 
[01:20:51]  finished in 0.161
[01:20:51] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:21:08] 
[01:21:08] running 120 tests
[01:21:34] .iiiii...i.....i..i...i..i.i...i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i 100/120
[01:21:39] .i......iii.i.....ii
[01:21:39] 
[01:21:39]  finished in 31.825
[01:21:39] travis_fold:end:test_debuginfo

---
travis_time:start:test_rustdoc-ui
Check compiletest suite=rustdoc-ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:51:00] 
[01:51:00] running 24 tests
[01:51:12] .......FFFFFFFFFFFF..FF.
[01:51:12] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:51:12] 
[01:51:12] ---- [ui] rustdoc-ui/deny-intra-link-resolution-failure.rs stdout ----
[01:51:12] diff of stderr:
[01:51:12] diff of stderr:
[01:51:12] 
[01:51:12] - error: `[v2]` cannot be resolved, ignoring it...
[01:51:12] -   --> $DIR/deny-intra-link-resolution-failure.rs:3:6
[01:51:12] -    |
[01:51:12] - LL | /// [v2]
[01:51:12] -    |
[01:51:12] - note: lint level defined here
[01:51:12] -   --> $DIR/deny-intra-link-resolution-failure.rs:1:9
[01:51:12] -    |
[01:51:12] -    |
[01:51:12] - LL | #![deny(intra_doc_link_resolution_failure)]
[01:51:12] -    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:51:12] -    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:51:12] + error: `[v2]` cannot be resolved, ignoring it...
[01:51:12] +   --> $DIR/deny-intra-link-resolution-failure.rs:3:6
[01:51:12] + LL | /// [v2]
[01:51:12] +    |      ^^ cannot be resolved, ignoring
[01:51:12] +    |
[01:51:12] + note: lint level defined here
[01:51:12] + note: lint level defined here
[01:51:12] +   --> $DIR/deny-intra-link-resolution-failure.rs:1:9
[01:51:12] +    |
[01:51:12] + LL | #![deny(intra_doc_link_resolution_failure)]
[01:51:12] +    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:51:12] +    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:51:12] - error: aborting due to previous error
[01:51:12] + error: aborting due to previous error
[01:51:12] 15 
[01:51:12] 16 
[01:51:12] 16 
[01:51:12] 
[01:51:12] 
[01:51:12] The actual stderr differed from the expected stderr.
[01:51:12] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/deny-intra-link-resolution-failure/deny-intra-link-resolution-failure.stderr
[01:51:12] To update references, rerun the tests and pass the `--bless` flag
[01:51:12] To only update this specific test, also pass `--test-args deny-intra-link-resolution-failure.rs`
[01:51:12] error: 1 errors occurred comparing output.
[01:51:12] status: exit code: 1
[01:51:12] status: exit code: 1
[01:51:12] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/deny-intra-link-resolution-failure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/deny-intra-link-resolution-failure/a" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/deny-intra-link-resolution-failure/auxiliary"
[01:51:12] ------------------------------------------
[01:51:12] 
[01:51:12] ------------------------------------------
[01:51:12] stderr:
[01:51:12] stderr:
[01:51:12] ------------------------------------------
[01:51:12] error: `[v2]` cannot be resolved, ignoring it...
[01:51:12]    |
[01:51:12]    |
[01:51:12] LL | /// [v2] //~ ERROR
[01:51:12]    |
[01:51:12] note: lint level defined here
[01:51:12]   --> /checkout/src/test/rustdoc-ui/deny-intra-link-resolution-failure.rs:1:9
[01:51:12]    |
---
[01:51:12] 
[01:51:12] - error: missing documentation for macro
[01:51:12] -   --> $DIR/deny-missing-docs-macro.rs:6:1
[01:51:12] -    |
[01:51:12] - LL | macro_rules! foo {
[01:51:12] -    |
[01:51:12] - note: lint level defined here
[01:51:12] -   --> $DIR/deny-missing-docs-macro.rs:3:9
[01:51:12] -    |
[01:51:12] -    |
[01:51:12] - LL | #![deny(missing_docs)]
[01:51:12] + error: missing documentation for macro
[01:51:12] +   --> $DIR/deny-missing-docs-macro.rs:6:1
[01:51:12] +    |
[01:51:12] +    |
[01:51:12] + LL | macro_rules! foo {
[01:51:12] +    |
[01:51:12] + note: lint level defined here
[01:51:12] +   --> $DIR/deny-missing-docs-macro.rs:3:9
[01:51:12] +    |
[01:51:12] +    |
[01:51:12] + LL | #![deny(missing_docs)]
[01:51:12] 12 
[01:51:12] - error: aborting due to previous error
[01:51:12] + error: aborting due to previous error
[01:51:12] 14 
[01:51:12] 14 
[01:51:12] 15 
[01:51:12] 
[01:51:12] 
[01:51:12] The actual stderr differed from the expected stderr.
[01:51:12] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/deny-missing-docs-macro/deny-missing-docs-macro.stderr
[01:51:12] To update references, rerun the tests and pass the `--bless` flag
[01:51:12] To only update this specific test, also pass `--test-args deny-missing-docs-macro.rs`
[01:51:12] error: 1 errors occurred comparing output.
[01:51:12] status: exit code: 1
[01:51:12] status: exit code: 1
[01:51:12] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/deny-missing-docs-macro.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/deny-missing-docs-macro/a" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/deny-missing-docs-macro/auxiliary"
[01:51:12] ------------------------------------------
[01:51:12] 
[01:51:12] ------------------------------------------
[01:51:12] stderr:
[01:51:12] stderr:
[01:51:12] ------------------------------------------
[01:51:12] error: missing documentation for macro
[01:51:12]   --> /checkout/src/test/rustdoc-ui/deny-missing-docs-macro.rs:6:1
[01:51:12]    |
[01:51:12] LL | macro_rules! foo { //~ ERROR
[01:51:12]    |
[01:51:12] note: lint level defined here
[01:51:12]   --> /checkout/src/test/rustdoc-ui/deny-missing-docs-macro.rs:3:9
[01:51:12]    |
[01:51:12]    |
[01:51:12] LL | #![deny(missing_docs)]
[01:51:12] 
[01:51:12] error: aborting due to previous error
[01:51:12] 
[01:51:12] 
---
[01:51:12] 
[01:51:12] - error: missing documentation for crate
[01:51:12] -   --> $DIR/deny-missing-docs-crate.rs:1:1
[01:51:12] -    |
[01:51:12] - LL | / #![deny(missing_docs)]
[01:51:12] - LL | |
[01:51:12] - LL | | pub struct Foo;
[01:51:12] -    |
[01:51:12] - note: lint level defined here
[01:51:12] -   --> $DIR/deny-missing-docs-crate.rs:1:9
[01:51:12] -    |
[01:51:12] -    |
[01:51:12] - LL | #![deny(missing_docs)]
[01:51:12] + error: missing documentation for crate
[01:51:12] +   --> $DIR/deny-missing-docs-crate.rs:1:1
[01:51:12] +    |
[01:51:12] +    |
[01:51:12] + LL | / #![deny(missing_docs)]
[01:51:12] + LL | |
[01:51:12] + LL | | pub struct Foo;
[01:51:12] +    |
[01:51:12] + note: lint level defined here
[01:51:12] +   --> $DIR/deny-missing-docs-crate.rs:1:9
[01:51:12] +    |
[01:51:12] +    |
[01:51:12] + LL | #![deny(missing_docs)]
[01:51:12] 14 
[01:51:12] - error: missing documentation for a struct
[01:51:12] -   --> $DIR/deny-missing-docs-crate.rs:3:1
[01:51:12] -    |
---
[01:51:12] 23 
[01:51:12] 
[01:51:12] 
[01:51:12] The actual stderr differed from the expected stderr.
[01:51:12] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/deny-missing-docs-crate/deny-missing-docs-crate.stderr
[01:51:12] To update references, rerun the tests and pass the `--bless` flag
[01:51:12] To only update this specific test, also pass `--test-args deny-missing-docs-crate.rs`
[01:51:12] error: 1 errors occurred comparing output.
[01:51:12] status: exit code: 1
[01:51:12] status: exit code: 1
[01:51:12] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/deny-missing-docs-crate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/deny-missing-docs-crate/a" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/deny-missing-docs-crate/auxiliary"
[01:51:12] ------------------------------------------
[01:51:12] 
[01:51:12] ------------------------------------------
[01:51:12] stderr:
[01:51:12] stderr:
[01:51:12] ------------------------------------------
[01:51:12] error: missing documentation for crate
[01:51:12]   --> /checkout/src/test/rustdoc-ui/deny-missing-docs-crate.rs:1:1
[01:51:12]    |
[01:51:12] LL | / #![deny(missing_docs)] //~ ERROR
[01:51:12] LL | |
[01:51:12] LL | | pub struct Foo; //~ ERROR
[01:51:12]    |
[01:51:12] note: lint level defined here
[01:51:12]   --> /checkout/src/test/rustdoc-ui/deny-missing-docs-crate.rs:1:9
[01:51:12]    |
[01:51:12]    |
[01:51:12] LL | #![deny(missing_docs)] //~ ERROR
[01:51:12] 
[01:51:12] error: missing documentation for a struct
[01:51:12]   --> /checkout/src/test/rustdoc-ui/deny-missing-docs-crate.rs:3:1
[01:51:12]    |
[01:51:12]    |
[01:51:12] LL | pub struct Foo; //~ ERROR
[01:51:12] 
[01:51:12] error: aborting due to 2 previous errors
[01:51:12] 
[01:51:12] 
[01:51:12] 
[01:51:12] ------------------------------------------
[01:51:12] 
[01:51:12] thread '[ui] rustdoc-ui/deny-missing-docs-crate.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3331:9
[01:51:12] 
[01:51:12] ---- [ui] rustdoc-ui/deprecated-attrs.rs stdout ----
[01:51:12] diff of stderr:
[01:51:12] 
[01:51:12] - warning: the `#![doc(no_default_passes)]` attribute is considered deprecated
[01:51:12] -    = warning: please see https://github.com/rust-lang/rust/issues/44136
[01:51:12] -    = warning: please see https://github.com/rust-lang/rust/issues/44136
[01:51:12] -    = help: you may want to use `#![doc(document_private_items)]`
[01:51:12] + warning: the `#![doc(no_default_passes)]` attribute is considered deprecated
[01:51:12] +    = warning: please see https://github.com/rust-lang/rust/issues/44136
[01:51:12] +    = warning: please see https://github.com/rust-lang/rust/issues/44136
[01:51:12] +    = help: you may want to use `#![doc(document_private_items)]`
[01:51:12] 5 
[01:51:12] - warning: the `#![doc(passes = "...")]` attribute is considered deprecated
[01:51:12] -    = warning: please see https://github.com/rust-lang/rust/issues/44136
[01:51:12] -    = warning: please see https://github.com/rust-lang/rust/issues/44136
[01:51:12] + warning: the `#![doc(passes = "...")]` attribute is considered deprecated
[01:51:12] +    = warning: please see https://github.com/rust-lang/rust/issues/44136
[01:51:12] 9 
[01:51:12] 10 
[01:51:12] 
[01:51:12] 
[01:51:12] 
[01:51:12] The actual stderr differed from the expected stderr.
[01:51:12] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/deprecated-attrs/deprecated-attrs.stderr
[01:51:12] To update references, rerun the tests and pass the `--bless` flag
[01:51:12] To only update this specific test, also pass `--test-args deprecated-attrs.rs`
[01:51:12] error: 1 errors occurred comparing output.
[01:51:12] status: exit code: 0
[01:51:12] status: exit code: 0
[01:51:12] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/deprecated-attrs.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/deprecated-attrs/a" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/deprecated-attrs/auxiliary"
[01:51:12] ------------------------------------------
[01:51:12] 
[01:51:12] ------------------------------------------
[01:51:12] stderr:
[01:51:12] stderr:
[01:51:12] ------------------------------------------
[01:51:12] warning: the `#![doc(no_default_passes)]` attribute is considered deprecated
[01:51:12]    = warning: please see https://github.com/rust-lang/rust/issues/44136
[01:51:12]    = warning: please see https://github.com/rust-lang/rust/issues/44136
[01:51:12]    = help: you may want to use `#![doc(document_private_items)]`
[01:51:12] 
[01:51:12] warning: the `#![doc(passes = "...")]` attribute is considered deprecated
[01:51:12]    = warning: please see https://github.com/rust-lang/rust/issues/44136
[01:51:12] 
[01:51:12] 
[01:51:12] ------------------------------------------
---
[01:51:12] diff of stdout:
[01:51:12] 
[01:51:12] 6 failures:
[01:51:12] 7 
[01:51:12] 8 ---- $DIR/failed-doctest-output.rs - OtherStruct (line 17) stdout ----
[01:51:12] - error[E0425]: cannot find value `no` in this scope
[01:51:12] -   |
[01:51:12] - 3 | no
[01:51:12] -   | ^^ not found in this scope
[01:51:12] -   | ^^ not found in this scope
[01:51:12] + error[E0425]: cannot find value `no` in this scope
[01:51:12] +   |
[01:51:12] + 3 | no
[01:51:12] +   | ^^ not found in this scope
[01:51:12] 14 
[01:51:12] 14 
[01:51:12] - error: aborting due to previous error
[01:51:12] + error: aborting due to previous error
[01:51:12] 16 
[01:51:12] - For more information about this error, try `rustc --explain E0425`.
[01:51:12] + For more information about this error, try `rustc --explain E0425`.
[01:51:12] 18 thread '$DIR/failed-doctest-output.rs - OtherStruct (line 17)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:310:13
[01:51:12] 20 
[01:51:12] 
[01:51:12] 
[01:51:12] The actual stdout differed from the expected stdout.
[01:51:12] The actual stdout differed from the expected stdout.
[01:51:12] Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/failed-doctest-output.stdout
[01:51:12] To update references, rerun the tests and pass the `--bless` flag
[01:51:12] To only update this specific test, also pass `--test-args failed-doctest-output.rs`
[01:51:12] error: 1 errors occurred comparing output.
[01:51:12] status: exit code: 101
[01:51:12] status: exit code: 101
[01:51:12] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/failed-doctest-output.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/a" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/auxiliary"
[01:51:12] ------------------------------------------
[01:51:12] 
[01:51:12] ------------------------------------------
[01:51:12] stderr:
---
[01:51:12] -    |
[01:51:12] - note: lint level defined here
[01:51:12] -   --> $DIR/doc-without-codeblock.rs:3:9
[01:51:12] -    |
[01:51:12] - LL | #![deny(missing_doc_code_examples)]
[01:51:12] + error: Missing code example in this documentation
[01:51:12] +    |
[01:51:12] + note: lint level defined here
[01:51:12] +   --> $DIR/doc-without-codeblock.rs:3:9
[01:51:12] +   --> $DIR/doc-without-codeblock.rs:3:9
[01:51:12] +    |
[01:51:12] + LL | #![deny(missing_doc_code_examples)]
[01:51:12] 8 
[01:51:12] - error: Missing code example in this documentation
[01:51:12] -   --> $DIR/doc-without-codeblock.rs:5:1
[01:51:12] -    |
---
[01:51:12] 14 
[01:51:12] - error: Missing code example in this documentation
[01:51:12] -   --> $DIR/doc-without-codeblock.rs:9:1
[01:51:12] -    |
[01:51:12] - LL | /// And then, the princess died.
[01:51:12] + error: Missing code example in this documentation
[01:51:12] +   --> $DIR/doc-without-codeblock.rs:9:1
[01:51:12] +    |
[01:51:12] +    |
[01:51:12] + LL | /// And then, the princess died.
[01:51:12] 20 
[01:51:12] - error: Missing code example in this documentation
[01:51:12] -   --> $DIR/doc-without-codeblock.rs:12:5
[01:51:12] -    |
[01:51:12] -    |
[01:51:12] - LL |     /// Or maybe not because she saved herself!
[01:51:12] + error: Missing code example in this documentation
[01:51:12] +   --> $DIR/doc-without-codeblock.rs:12:5
[01:51:12] +    |
[01:51:12] +    |
[01:51:12] + LL |     /// Or maybe not because she saved herself!
[01:51:12] 26 
[01:51:12] - error: aborting due to 4 previous errors
[01:51:12] + error: aborting due to 4 previous errors
[01:51:12] 28 
[01:51:12] 28 
[01:51:12] 29 
[01:51:12] 
[01:51:12] 
[01:51:12] The actual stderr differed from the expected stderr.
[01:51:12] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doc-without-codeblock/doc-without-codeblock.stderr
[01:51:12] To update references, rerun the tests and pass the `--bless` flag
[01:51:12] To only update this specific test, also pass `--test-args doc-without-codeblock.rs`
[01:51:12] error: 1 errors occurred comparing output.
[01:51:12] status: exit code: 1
[01:51:12] status: exit code: 1
[01:51:12] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/doc-without-codeblock.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doc-without-codeblock/a" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doc-without-codeblock/auxiliary"
[01:51:12] ------------------------------------------
[01:51:12] 
[01:51:12] ------------------------------------------
[01:51:12] stderr:
---
[01:51:12] 
[01:51:12] error: Missing code example in this documentation
[01:51:12]   --> /checkout/src/test/rustdoc-ui/doc-without-codeblock.rs:9:1
[01:51:12]    |
[01:51:12] LL | /// And then, the princess died.
[01:51:12] 
[01:51:12] error: Missing code example in this documentation
[01:51:12]   --> /checkout/src/test/rustdoc-ui/doc-without-codeblock.rs:12:5
[01:51:12]    |
[01:51:12]    |
[01:51:12] LL |     /// Or maybe not because she saved herself!
[01:51:12] 
[01:51:12] error: aborting due to 4 previous errors
[01:51:12] 
[01:51:12] 
[01:51:12] 
[01:51:12] ------------------------------------------
[01:51:12] 
[01:51:12] thread '[ui] rustdoc-ui/doc-without-codeblock.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3331:9
[01:51:12] 
[01:51:12] ---- [ui] rustdoc-ui/intra-doc-alias-ice.rs stdout ----
[01:51:12] diff of stderr:
[01:51:12] 
[01:51:12] - error: `[TypeAlias::hoge]` cannot be resolved, ignoring it...
[01:51:12] -    |
[01:51:12] -    |
[01:51:12] - LL | /// [broken cross-reference](TypeAlias::hoge)
[01:51:12] -    |
[01:51:12] - note: lint level defined here
[01:51:12] -   --> $DIR/intra-doc-alias-ice.rs:1:9
[01:51:12] -    |
[01:51:12] -    |
[01:51:12] - LL | #![deny(intra_doc_link_resolution_failure)]
[01:51:12] -    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:51:12] -    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:51:12] + error: `[TypeAlias::hoge]` cannot be resolved, ignoring it...
[01:51:12] +    |
[01:51:12] +    |
[01:51:12] + LL | /// [broken cross-reference](TypeAlias::hoge)
[01:51:12] +    |
[01:51:12] + note: lint level defined here
[01:51:12] +   --> $DIR/intra-doc-alias-ice.rs:1:9
[01:51:12] +    |
[01:51:12] +    |
[01:51:12] + LL | #![deny(intra_doc_link_resolution_failure)]
[01:51:12] +    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:51:12] +    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:51:12] - error: aborting due to previous error
[01:51:12] + error: aborting due to previous error
[01:51:12] 15 
[01:51:12] 16 
[01:51:12] 16 
[01:51:12] 
[01:51:12] 
[01:51:12] The actual stderr differed from the expected stderr.
[01:51:12] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-doc-alias-ice/intra-doc-alias-ice.stderr
[01:51:12] To update references, rerun the tests and pass the `--bless` flag
[01:51:12] To only update this specific test, also pass `--test-args intra-doc-alias-ice.rs`
[01:51:12] error: 1 errors occurred comparing output.
[01:51:12] status: exit code: 1
[01:51:12] status: exit code: 1
[01:51:12] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/intra-doc-alias-ice.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-doc-alias-ice/a" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-doc-alias-ice/auxiliary"
[01:51:12] ------------------------------------------
[01:51:12] 
[01:51:12] ------------------------------------------
[01:51:12] stderr:
[01:51:12] stderr:
[01:51:12] ------------------------------------------
[01:51:12] error: `[TypeAlias::hoge]` cannot be resolved, ignoring it...
[01:51:12]    |
[01:51:12]    |
[01:51:12] LL | /// [broken cross-reference](TypeAlias::hoge) //~ ERROR
[01:51:12]    |
[01:51:12] note: lint level defined here
[01:51:12]   --> /checkout/src/test/rustdoc-ui/intra-doc-alias-ice.rs:1:9
[01:51:12]    |
---
[01:51:12] 
[01:51:12] - error: `[i]` cannot be resolved, ignoring it...
[01:51:12] -   --> $DIR/intra-link-span-ice-55723.rs:11:10
[01:51:12] -    |
[01:51:12] - LL | /// （arr[i]）
[01:51:12] -    |
[01:51:12] - note: lint level defined here
[01:51:12] -   --> $DIR/intra-link-span-ice-55723.rs:3:9
[01:51:12] -    |
[01:51:12] -    |
[01:51:12] - LL | #![deny(intra_doc_link_resolution_failure)]
[01:51:12] -    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:51:12] -    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:51:12] + error: `[i]` cannot be resolved, ignoring it...
[01:51:12] +    |
[01:51:12] +    |
[01:51:12] + LL | /// （arr[i]）
[01:51:12] +    |
[01:51:12] + note: lint level defined here
[01:51:12] +   --> $DIR/intra-link-span-ice-55723.rs:3:9
[01:51:12] +    |
[01:51:12] +    |
[01:51:12] + LL | #![deny(intra_doc_link_resolution_failure)]
[01:51:12] +    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:51:12] +    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:51:12] - error: aborting due to previous error
[01:51:12] + error: aborting due to previous error
[01:51:12] 15 
[01:51:12] 16 
[01:51:12] 16 
[01:51:12] 
[01:51:12] 
[01:51:12] The actual stderr differed from the expected stderr.
[01:51:12] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-link-span-ice-55723/intra-link-span-ice-55723.stderr
[01:51:12] To update references, rerun the tests and pass the `--bless` flag
[01:51:12] To only update this specific test, also pass `--test-args intra-link-span-ice-55723.rs`
[01:51:12] error: 1 errors occurred comparing output.
[01:51:12] status: exit code: 1
[01:51:12] status: exit code: 1
[01:51:12] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/intra-link-span-ice-55723.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-link-span-ice-55723/a" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-link-span-ice-55723/auxiliary"
[01:51:12] ------------------------------------------
[01:51:12] 
[01:51:12] ------------------------------------------
[01:51:12] stderr:
---
[01:51:12] 
[01:51:12] ---- [ui] rustdoc-ui/intra-links-ambiguity.rs stdout ----
[01:51:12] diff of stderr:
[01:51:12] 
[01:51:12] - error: `ambiguous` is both a struct and a function
[01:51:12] -    |
[01:51:12] -    |
[01:51:12] - LL | /// [`ambiguous`] is ambiguous.
[01:51:12] -    |
[01:51:12] - note: lint level defined here
[01:51:12] -   --> $DIR/intra-links-ambiguity.rs:1:9
[01:51:12] -    |
[01:51:12] -    |
[01:51:12] - LL | #![deny(intra_doc_link_resolution_failure)]
[01:51:12] - help: to link to the struct, prefix with the item type
[01:51:12] -    |
[01:51:12] -    |
[01:51:12] - LL | /// [`struct@ambiguous`] is ambiguous.
[01:51:12] - help: to link to the function, add parentheses
[01:51:12] -    |
[01:51:12] -    |
[01:51:12] - LL | /// [`ambiguous()`] is ambiguous.
[01:51:12] -    |      ^^^^^^^^^^^^^
[01:51:12] + error: `ambiguous` is both a struct and a function
[01:51:12] +    |
[01:51:12] +    |
[01:51:12] + LL | /// [`ambiguous`] is ambiguous.
[01:51:12] +    |
[01:51:12] + note: lint level defined here
[01:51:12] +   --> $DIR/intra-links-ambiguity.rs:1:9
[01:51:12] +    |
[01:51:12] +    |
[01:51:12] + LL | #![deny(intra_doc_link_resolution_failure)]
[01:51:12] +    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:51:12] + help: to link to the struct, prefix with the item type
[01:51:12] +    |
[01:51:12] + LL | /// [`struct@ambiguous`] is ambiguous.
[01:51:12] + help: to link to the function, add parentheses
[01:51:12] +    |
[01:51:12] +    |
[01:51:12] + LL | /// [`ambiguous()`] is ambiguous.
[01:51:12] 20 
[01:51:12] 20 
[01:51:12] - error: `ambiguous` is both a struct and a function
[01:51:12] -    |
[01:51:12] -    |
[01:51:12] - LL | /// [ambiguous] is ambiguous.
[01:51:12] - help: to link to the struct, prefix with the item type
[01:51:12] -    |
[01:51:12] -    |
[01:51:12] - LL | /// [struct@ambiguous] is ambiguous.
[01:51:12] - help: to link to the function, add parentheses
[01:51:12] -    |
[01:51:12] -    |
[01:51:12] - LL | /// [ambiguous()] is ambiguous.
[01:51:12] -    |      ^^^^^^^^^^^
[01:51:12] + error: `ambiguous` is both a struct and a function
[01:51:12] +    |
[01:51:12] +    |
[01:51:12] + LL | /// [ambiguous] is ambiguous.
[01:51:12] + help: to link to the struct, prefix with the item type
[01:51:12] +    |
[01:51:12] +    |
[01:51:12] + LL | /// [struct@ambiguous] is ambiguous.
[01:51:12] + help: to link to the function, add parentheses
[01:51:12] +    |
[01:51:12] +    |
[01:51:12] + LL | /// [ambiguous()] is ambiguous.
[01:51:12] 34 
[01:51:12] 34 
[01:51:12] - error: `multi_conflict` is a struct, a function, and a macro
[01:51:12] -    |
[01:51:12] -    |
[01:51:12] - LL | /// [`multi_conflict`] is a three-way conflict.
[01:51:12] - help: to link to the struct, prefix with the item type
[01:51:12] -    |
[01:51:12] -    |
[01:51:12] - LL | /// [`struct@multi_conflict`] is a three-way conflict.
[01:51:12] - help: to link to the function, add parentheses
[01:51:12] -    |
[01:51:12] -    |
[01:51:12] - LL | /// [`multi_conflict()`] is a three-way conflict.
[01:51:12] - help: to link to the macro, add an exclamation mark
[01:51:12] -    |
[01:51:12] -    |
[01:51:12] - LL | /// [`multi_conflict!`] is a three-way conflict.
[01:51:12] -    |      ^^^^^^^^^^^^^^^^^
[01:51:12] + error: `multi_conflict` is a struct, a function, and a macro
[01:51:12] +    |
[01:51:12] +    |
[01:51:12] + LL | /// [`multi_conflict`] is a three-way conflict.
[01:51:12] + help: to link to the struct, prefix with the item type
[01:51:12] +    |
[01:51:12] +    |
[01:51:12] + LL | /// [`struct@multi_conflict`] is a three-way conflict.
[01:51:12] + help: to link to the function, add parentheses
[01:51:12] +    |
[01:51:12] +    |
[01:51:12] + LL | /// [`multi_conflict()`] is a three-way conflict.
[01:51:12] + help: to link to the macro, add an exclamation mark
[01:51:12] +    |
[01:51:12] +    |
[01:51:12] + LL | /// [`multi_conflict!`] is a three-way conflict.
[01:51:12] 52 
[01:51:12] 52 
[01:51:12] - error: `type_and_value` is both a module and a constant
[01:51:12] -    |
[01:51:12] -    |
[01:51:12] - LL | /// Ambiguous [type_and_value].
[01:51:12] - help: to link to the module, prefix with the item type
[01:51:12] -    |
[01:51:12] -    |
[01:51:12] - LL | /// Ambiguous [module@type_and_value].
[01:51:12] - help: to link to the constant, prefix with the item type
[01:51:12] -    |
[01:51:12] -    |
[01:51:12] - LL | /// Ambiguous [const@type_and_value].
[01:51:12] -    |                ^^^^^^^^^^^^^^^^^^^^
[01:51:12] + error: `type_and_value` is both a module and a constant
[01:51:12] +    |
[01:51:12] +    |
[01:51:12] + LL | /// Ambiguous [type_and_value].
[01:51:12] + help: to link to the module, prefix with the item type
[01:51:12] +    |
[01:51:12] +    |
[01:51:12] + LL | /// Ambiguous [module@type_and_value].
[01:51:12] + help: to link to the constant, prefix with the item type
[01:51:12] +    |
[01:51:12] +    |
[01:51:12] + LL | /// Ambiguous [const@type_and_value].
[01:51:12] 66 
[01:51:12] 66 
[01:51:12] - error: `foo::bar` is both an enum and a function
[01:51:12] -    |
[01:51:12] -    |
[01:51:12] - LL | /// Ambiguous non-implied shortcut link [`foo::bar`].
[01:51:12] -    |                                          ^^^^^^^^^^ ambiguous link
[01:51:12] - help: to link to the enum, prefix with the item type
[01:51:12] -    |
[01:51:12] - LL | /// Ambiguous non-implied shortcut link [`enum@foo::bar`].
[01:51:12] - help: to link to the function, add parentheses
[01:51:12] -    |
[01:51:12] -    |
[01:51:12] - LL | /// Ambiguous non-implied shortcut link [`foo::bar()`].
[01:51:12] -    |                                          ^^^^^^^^^^^^
[01:51:12] + error: `foo::bar` is both an enum and a function
[01:51:12] +    |
[01:51:12] +    |
[01:51:12] + LL | /// Ambiguous non-implied shortcut link [`foo::bar`].
[01:51:12] +    |                                          ^^^^^^^^^^ ambiguous link
[01:51:12] + help: to link to the enum, prefix with the item type
[01:51:12] +    |
[01:51:12] + LL | /// Ambiguous non-implied shortcut link [`enum@foo::bar`].
[01:51:12] + help: to link to the function, add parentheses
[01:51:12] +    |
[01:51:12] +    |
[01:51:12] + LL | /// Ambiguous non-implied shortcut link [`foo::bar()`].
[01:51:12] 80 
[01:51:12] - error: aborting due to 5 previous errors
[01:51:12] + error: aborting due to 5 previous errors
[01:51:12] 82 
[01:51:12] 82 
[01:51:12] 83 
[01:51:12] 
[01:51:12] 
[01:51:12] The actual stderr differed from the expected stderr.
[01:51:12] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-links-ambiguity/intra-links-ambiguity.stderr
[01:51:12] To update references, rerun the tests and pass the `--bless` flag
[01:51:12] To only update this specific test, also pass `--test-args intra-links-ambiguity.rs`
[01:51:12] error: 1 errors occurred comparing output.
[01:51:12] status: exit code: 1
[01:51:12] status: exit code: 1
[01:51:12] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/intra-links-ambiguity.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-links-ambiguity/a" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-links-ambiguity/auxiliary"
[01:51:12] ------------------------------------------
[01:51:12] 
[01:51:12] ------------------------------------------
[01:51:12] stderr:
[01:51:12] stderr:
[01:51:12] ------------------------------------------
[01:51:12] error: `ambiguous` is both a struct and a function
[01:51:12]    |
[01:51:12]    |
[01:51:12] LL | /// [`ambiguous`] is ambiguous. //~ERROR `ambiguous`
[01:51:12]    |
[01:51:12] note: lint level defined here
[01:51:12]   --> /checkout/src/test/rustdoc-ui/intra-links-ambiguity.rs:1:9
[01:51:12]    |
[01:51:12]    |
[01:51:12] LL | #![deny(intra_doc_link_resolution_failure)]
[01:51:12]    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:51:12] help: to link to the struct, prefix with the item type
[01:51:12]    |
[01:51:12] LL | /// [`struct@ambiguous`] is ambiguous. //~ERROR `ambiguous`
[01:51:12] help: to link to the function, add parentheses
[01:51:12]    |
[01:51:12]    |
[01:51:12] LL | /// [`ambiguous()`] is ambiguous. //~ERROR `ambiguous`
[01:51:12] 
[01:51:12] 
[01:51:12] error: `ambiguous` is both a struct and a function
[01:51:12]    |
[01:51:12]    |
[01:51:12] LL | /// [ambiguous] is ambiguous. //~ERROR ambiguous
[01:51:12] help: to link to the struct, prefix with the item type
[01:51:12]    |
[01:51:12]    |
[01:51:12] LL | /// [struct@ambiguous] is ambiguous. //~ERROR ambiguous
[01:51:12] help: to link to the function, add parentheses
[01:51:12]    |
[01:51:12]    |
[01:51:12] LL | /// [ambiguous()] is ambiguous. //~ERROR ambiguous
[01:51:12] 
[01:51:12] 
[01:51:12] error: `multi_conflict` is a struct, a function, and a macro
[01:51:12]    |
[01:51:12]    |
[01:51:12] LL | /// [`multi_conflict`] is a three-way conflict. //~ERROR `multi_conflict`
[01:51:12] help: to link to the struct, prefix with the item type
[01:51:12]    |
[01:51:12]    |
[01:51:12] LL | /// [`struct@multi_conflict`] is a three-way conflict. //~ERROR `multi_conflict`
[01:51:12] help: to link to the function, add parentheses
[01:51:12]    |
[01:51:12]    |
[01:51:12] LL | /// [`multi_conflict()`] is a three-way conflict. //~ERROR `multi_conflict`
[01:51:12] help: to link to the macro, add an exclamation mark
[01:51:12]    |
[01:51:12]    |
[01:51:12] LL | /// [`multi_conflict!`] is a three-way conflict. //~ERROR `multi_conflict`
[01:51:12] 
[01:51:12] 
[01:51:12] error: `type_and_value` is both a module and a constant
[01:51:12]    |
[01:51:12]    |
[01:51:12] LL | /// Ambiguous [type_and_value]. //~ERROR type_and_value
[01:51:12] help: to link to the module, prefix with the item type
[01:51:12]    |
[01:51:12]    |
[01:51:12] LL | /// Ambiguous [module@type_and_value]. //~ERROR type_and_value
[01:51:12] help: to link to the constant, prefix with the item type
[01:51:12]    |
[01:51:12]    |
[01:51:12] LL | /// Ambiguous [const@type_and_value]. //~ERROR type_and_value
[01:51:12] 
[01:51:12] 
[01:51:12] error: `foo::bar` is both an enum and a function
[01:51:12]    |
[01:51:12]    |
[01:51:12] LL | /// Ambiguous non-implied shortcut link [`foo::bar`]. //~ERROR `foo::bar`
[01:51:12] help: to link to the enum, prefix with the item type
[01:51:12]    |
[01:51:12]    |
[01:51:12] LL | /// Ambiguous non-implied shortcut link [`enum@foo::bar`]. //~ERROR `foo::bar`
[01:51:12] help: to link to the function, add parentheses
[01:51:12]    |
[01:51:12]    |
[01:51:12] LL | /// Ambiguous non-implied shortcut link [`foo::bar()`]. //~ERROR `foo::bar`
[01:51:12] 
[01:51:12] error: aborting due to 5 previous errors
[01:51:12] 
[01:51:12] 
---
[01:51:12] 
[01:51:12] - warning: `[error]` cannot be resolved, ignoring it...
[01:51:12] -   --> $DIR/intra-links-warning-crlf.rs:8:6
[01:51:12] -    |
[01:51:12] - LL | /// [error]
[01:51:12] -    |
[01:51:12] -    = note: #[warn(intra_doc_link_resolution_failure)] on by default
[01:51:12] -    = note: #[warn(intra_doc_link_resolution_failure)] on by default
[01:51:12] -    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:51:12] + warning: `[error]` cannot be resolved, ignoring it...
[01:51:12] +    |
[01:51:12] + LL | /// [error]

[01:51:12] +    |      ^^^^^ cannot be resolved, ignoring
[01:51:12] +    |      ^^^^^ cannot be resolved, ignoring
[01:51:12] +    |
[01:51:12] +    = note: #[warn(intra_doc_link_resolution_failure)] on by default
[01:51:12] +    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:51:12] - warning: `[error1]` cannot be resolved, ignoring it...
[01:51:12] -   --> $DIR/intra-links-warning-crlf.rs:12:11
[01:51:12] -    |
[01:51:12] -    |
[01:51:12] - LL | /// docs [error1]
[01:51:12] -    |
[01:51:12] -    |
[01:51:12] -    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:51:12] + warning: `[error1]` cannot be resolved, ignoring it...
[01:51:12] +    |
[01:51:12] +    |
[01:51:12] + LL | /// docs [error1]
[01:51:12] +    |           ^^^^^^ cannot be resolved, ignoring
[01:51:12] +    |
[01:51:12] +    |
[01:51:12] +    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:51:12] - warning: `[error2]` cannot be resolved, ignoring it...
[01:51:12] -   --> $DIR/intra-links-warning-crlf.rs:14:11
[01:51:12] -    |
[01:51:12] -    |
[01:51:12] - LL | /// docs [error2]
[01:51:12] -    |
[01:51:12] -    |
[01:51:12] -    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:51:12] + warning: `[error2]` cannot be resolved, ignoring it...
[01:51:12] +    |
[01:51:12] +    |
[01:51:12] + LL | /// docs [error2]
[01:51:12] +    |           ^^^^^^ cannot be resolved, ignoring
[01:51:12] +    |
[01:51:12] +    |
[01:51:12] +    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:51:12] - warning: `[error]` cannot be resolved, ignoring it...
[01:51:12] -   --> $DIR/intra-links-warning-crlf.rs:21:20
[01:51:12] -    |
[01:51:12] -    |
[01:51:12] - LL |  * It also has an [error].
[01:51:12] -    |
[01:51:12] -    |
[01:51:12] -    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:51:12] + warning: `[error]` cannot be resolved, ignoring it...
[01:51:12] +    |
[01:51:12] +    |
[01:51:12] + LL |  * It also has an [error].
[01:51:12] +    |                    ^^^^^ cannot be resolved, ignoring
[01:51:12] +    |
[01:51:12] +    |
[01:51:12] +    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:51:12] 34 
[01:51:12] 
[01:51:12] 
[01:51:12] The actual stderr differed from the expected stderr.
[01:51:12] The actual stderr differed from the expected stderr.
[01:51:12] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-links-warning-crlf/intra-links-warning-crlf.stderr
[01:51:12] To update references, rerun the tests and pass the `--bless` flag
[01:51:12] To only update this specific test, also pass `--test-args intra-links-warning-crlf.rs`
[01:51:12] error: 1 errors occurred comparing output.
[01:51:12] status: exit code: 0
[01:51:12] status: exit code: 0
[01:51:12] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/intra-links-warning-crlf.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-links-warning-crlf/a" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-links-warning-crlf/auxiliary"
[01:51:12] ------------------------------------------
[01:51:12] 
[01:51:12] ------------------------------------------
[01:51:12] stderr:
---
[01:51:12] 
[01:51:12] warning: `[error1]` cannot be resolved, ignoring it...
[01:51:12]   --> /checkout/src/test/rustdoc-ui/intra-links-warning-crlf.rs:12:11
[01:51:12]    |
[01:51:12] LL | /// docs [error1]
[01:51:12]    |           ^^^^^^ cannot be resolved, ignoring
[01:51:12]    |
[01:51:12]    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:51:12] 
[01:51:12] 
[01:51:12] warning: `[error2]` cannot be resolved, ignoring it...
[01:51:12]   --> /checkout/src/test/rustdoc-ui/intra-links-warning-crlf.rs:14:11
[01:51:12]    |
[01:51:12] LL | /// docs [error2]
[01:51:12]    |           ^^^^^^ cannot be resolved, ignoring
[01:51:12]    |
[01:51:12]    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:51:12] 
[01:51:12] 
[01:51:12] warning: `[error]` cannot be resolved, ignoring it...
[01:51:12]   --> /checkout/src/test/rustdoc-ui/intra-links-warning-crlf.rs:21:20
[01:51:12]    |
[01:51:12] LL |  * It also has an [error].
[01:51:12]    |                    ^^^^^ cannot be resolved, ignoring
[01:51:12]    |
[01:51:12]    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:51:12] 
---
[01:51:12] 
[01:51:12] ---- [ui] rustdoc-ui/intra-links-warning.rs stdout ----
[01:51:12] diff of stderr:
[01:51:12] 
[01:51:12] - warning: `[Foo::baz]` cannot be resolved, ignoring it...
[01:51:12] -    |
[01:51:12] -    |
[01:51:12] - LL |        //! Test with [Foo::baz], [Bar::foo], ...
[01:51:12] -    |
[01:51:12] -    = note: #[warn(intra_doc_link_resolution_failure)] on by default
[01:51:12] -    = note: #[warn(intra_doc_link_resolution_failure)] on by default
[01:51:12] -    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:51:12] + warning: `[Foo::baz]` cannot be resolved, ignoring it...
[01:51:12] +    |
[01:51:12] +    |
[01:51:12] + LL |        //! Test with [Foo::baz], [Bar::foo], ...
[01:51:12] +    |
[01:51:12] +    = note: #[warn(intra_doc_link_resolution_failure)] on by default
[01:51:12] +    = note: #[warn(intra_doc_link_resolution_failure)] on by default
[01:51:12] +    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:51:12] 9 
[01:51:12] - warning: `[Bar::foo]` cannot be resolved, ignoring it...
[01:51:12] -    |
[01:51:12] -    |
[01:51:12] - LL |        //! Test with [Foo::baz], [Bar::foo], ...
[01:51:12] -    |
[01:51:12] -    |
[01:51:12] -    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:51:12] + warning: `[Bar::foo]` cannot be resolved, ignoring it...
[01:51:12] +    |
[01:51:12] +    |
[01:51:12] + LL |        //! Test with [Foo::baz], [Bar::foo], ...
[01:51:12] +    |
[01:51:12] +    |
[01:51:12] +    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:51:12] 17 
[01:51:12] - warning: `[Uniooon::X]` cannot be resolved, ignoring it...
[01:51:12] -    |
[01:51:12] -    |
[01:51:12] - LL |      //! , [Uniooon::X] and [Qux::Z].
[01:51:12] -    |
[01:51:12] -    |
[01:51:12] -    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:51:12] + warning: `[Uniooon::X]` cannot be resolved, ignoring it...
[01:51:12] +    |
[01:51:12] +    |
[01:51:12] + LL |      //! , [Uniooon::X] and [Qux::Z].
[01:51:12] +    |
[01:51:12] +    |
[01:51:12] +    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:51:12] 25 
[01:51:12] - warning: `[Qux::Z]` cannot be resolved, ignoring it...
[01:51:12] -    |
[01:51:12] -    |
[01:51:12] - LL |      //! , [Uniooon::X] and [Qux::Z].
[01:51:12] -    |
[01:51:12] -    |
[01:51:12] -    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:51:12] + warning: `[Qux::Z]` cannot be resolved, ignoring it...
[01:51:12] +    |
[01:51:12] +    |
[01:51:12] + LL |      //! , [Uniooon::X] and [Qux::Z].
[01:51:12] +    |
[01:51:12] +    |
[01:51:12] +    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:51:12] 33 
[01:51:12] - warning: `[Uniooon::X]` cannot be resolved, ignoring it...
[01:51:12] -    |
[01:51:12] -    |
[01:51:12] - LL |       //! , [Uniooon::X] and [Qux::Z].
[01:51:12] -    |
[01:51:12] -    |
[01:51:12] -    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:51:12] + warning: `[Uniooon::X]` cannot be resolved, ignoring it...
[01:51:12] +    |
[01:51:12] +    |
[01:51:12] + LL |       //! , [Uniooon::X] and [Qux::Z].
[01:51:12] +    |
[01:51:12] +    |
[01:51:12] +    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:51:12] 41 
[01:51:12] - warning: `[Qux::Z]` cannot be resolved, ignoring it...
[01:51:12] -    |
[01:51:12] -    |
[01:51:12] - LL |       //! , [Uniooon::X] and [Qux::Z].
[01:51:12] -    |
[01:51:12] -    |
[01:51:12] -    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:51:12] + warning: `[Qux::Z]` cannot be resolved, ignoring it...
[01:51:12] +    |
[01:51:12] +    |
[01:51:12] + LL |       //! , [Uniooon::X] and [Qux::Z].
[01:51:12] +    |
[01:51:12] +    |
[01:51:12] +    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:51:12] 49 
[01:51:12] - warning: `[Qux:Y]` cannot be resolved, ignoring it...
[01:51:12] -    |
[01:51:12] -    |
[01:51:12] - LL |        /// [Qux:Y]
[01:51:12] -    |
[01:51:12] -    |
[01:51:12] -    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:51:12] + warning: `[Qux:Y]` cannot be resolved, ignoring it...
[01:51:12] +    |
[01:51:12] +    |
[01:51:12] + LL |        /// [Qux:Y]
[01:51:12] +    |
[01:51:12] +    |
[01:51:12] +    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:51:12] - warning: `[error]` cannot be resolved, ignoring it...
[01:51:12] -   --> $DIR/intra-links-warning.rs:51:30
[01:51:12] -    |
[01:51:12] -    |
[01:51:12] - LL |  * time to introduce a link [error]*/
[01:51:12] -    |
[01:51:12] -    |
[01:51:12] -    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:51:12] + warning: `[error]` cannot be resolved, ignoring it...
[01:51:12] +    |
[01:51:12] +    |
[01:51:12] + LL |  * time to introduce a link [error]*/
[01:51:12] +    |
[01:51:12] +    |
[01:51:12] +    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:51:12] - warning: `[error]` cannot be resolved, ignoring it...
[01:51:12] -   --> $DIR/intra-links-warning.rs:57:30
[01:51:12] -    |
[01:51:12] -    |
[01:51:12] - LL |  * time to introduce a link [error]
[01:51:12] -    |
[01:51:12] -    |
[01:51:12] -    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:51:12] + warning: `[error]` cannot be resolved, ignoring it...
[01:51:12] +    |
[01:51:12] +    |
[01:51:12] + LL |  * time to introduce a link [error]
[01:51:12] +    |
[01:51:12] +    |
[01:51:12] +    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:51:12] - warning: `[error]` cannot be resolved, ignoring it...
[01:51:12] -   --> $DIR/intra-links-warning.rs:61:1
[01:51:12] -    |
[01:51:12] -    |
[01:51:12] - LL | #[doc = "single line [error]"]
[01:51:12] -    |
[01:51:12] -    = note: the link appears in this line:
[01:51:12] -            
[01:51:12] -            single line [error]
[01:51:12] -            single line [error]
[01:51:12] -                         ^^^^^
[01:51:12] -    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:51:12] + warning: `[error]` cannot be resolved, ignoring it...
[01:51:12] +    |
[01:51:12] +    |
[01:51:12] + LL | #[doc = "single line [error]"]
[01:51:12] +    |
[01:51:12] +    = note: the link appears in this line:
[01:51:12] +            
[01:51:12] +            single line [error]
[01:51:12] +            single line [error]
[01:51:12] +                         ^^^^^
[01:51:12] +    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:51:12] - warning: `[error]` cannot be resolved, ignoring it...
[01:51:12] -   --> $DIR/intra-links-warning.rs:64:1
[01:51:12] -    |
[01:51:12] -    |
[01:51:12] - LL | #[doc = "single line with /"escaping/" [error]"]
[01:51:12] -    |
[01:51:12] -    = note: the link appears in this line:
[01:51:12] -            
[01:51:12] -            
[01:51:12] -            single line with "escaping" [error]
[01:51:12] -                                         ^^^^^
[01:51:12] -    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:51:12] + warning: `[error]` cannot be resolved, ignoring it...
[01:51:12] +    |
[01:51:12] +    |
[01:51:12] + LL | #[doc = "single line with /"escaping/" [error]"]
[01:51:12] +    |
[01:51:12] +    = note: the link appears in this line:
[01:51:12] +            
[01:51:12] +            
[01:51:12] +            single line with "escaping" [error]
[01:51:12] +                                         ^^^^^
[01:51:12] +    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:51:12] - warning: `[error]` cannot be resolved, ignoring it...
[01:51:12] -   --> $DIR/intra-links-warning.rs:67:1
[01:51:12] -    |
[01:51:12] - LL | / /// Item docs.
[01:51:12] - LL | / /// Item docs.
[01:51:12] - LL | | #[doc="Hello there!"]
[01:51:12] - LL | | /// [error]
[01:51:12] -    |
[01:51:12] -    = note: the link appears in this line:
[01:51:12] -            
[01:51:12] -            [error]
[01:51:12] -            [error]
[01:51:12] -             ^^^^^
[01:51:12] -    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:51:12] + warning: `[error]` cannot be resolved, ignoring it...
[01:51:12] +    |
[01:51:12] + LL | / /// Item docs.
[01:51:12] + LL | / /// Item docs.
[01:51:12] + LL | | #[doc="Hello there!"]
[01:51:12] + LL | | /// [error]
[01:51:12] +    |
[01:51:12] +    = note: the link appears in this line:
[01:51:12] +            
[01:51:12] +            [error]
[01:51:12] +            [error]
[01:51:12] +             ^^^^^
[01:51:12] +    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:51:12] - warning: `[error1]` cannot be resolved, ignoring it...
[01:51:12] -   --> $DIR/intra-links-warning.rs:73:11
[01:51:12] -    |
[01:51:12] -    |
[01:51:12] - LL | /// docs [error1]
[01:51:12] -    |
[01:51:12] -    |
[01:51:12] -    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:51:12] + warning: `[error1]` cannot be resolved, ignoring it...
[01:51:12] +    |
[01:51:12] +    |
[01:51:12] + LL | /// docs [error1]
[01:51:12] +    |
[01:51:12] +    |
[01:51:12] +    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:51:12] - warning: `[error2]` cannot be resolved, ignoring it...
[01:51:12] -   --> $DIR/intra-links-warning.rs:75:11
[01:51:12] -    |
[01:51:12] -    |
[01:51:12] - LL | /// docs [error2]
[01:51:12] -    |
[01:51:12] -    |
[01:51:12] -    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:51:12] + warning: `[error2]` cannot be resolved, ignoring it...
[01:51:12] +    |
[01:51:12] +    |
[01:51:12] + LL | /// docs [error2]
[01:51:12] +    |
[01:51:12] +    |
[01:51:12] +    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:51:12] 127 
[01:51:12] - warning: `[BarA]` cannot be resolved, ignoring it...
[01:51:12] -    |
[01:51:12] -    |
[01:51:12] - LL | /// bar [BarA] bar
[01:51:12] -    |
[01:51:12] -    |
[01:51:12] -    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:51:12] + warning: `[BarA]` cannot be resolved, ignoring it...
[01:51:12] +    |
[01:51:12] +    |
[01:51:12] + LL | /// bar [BarA] bar
[01:51:12] +    |
[01:51:12] +    |
[01:51:12] +    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:51:12] 135 
[01:51:12] - warning: `[BarB]` cannot be resolved, ignoring it...
[01:51:12] -    |
[01:51:12] -    |
[01:51:12] - LL |  * bar [BarB] bar
[01:51:12] -    |
[01:51:12] -    |
[01:51:12] -    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:51:12] + warning: `[BarB]` cannot be resolved, ignoring it...
[01:51:12] +    |
[01:51:12] +    |
[01:51:12] + LL |  * bar [BarB] bar
[01:51:12] +    |
[01:51:12] +    |
[01:51:12] +    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:51:12] 143 
[01:51:12] - warning: `[BarC]` cannot be resolved, ignoring it...
[01:51:12] -    |
[01:51:12] -    |
[01:51:12] - LL | bar [BarC] bar
[01:51:12] -    |
[01:51:12] -    |
[01:51:12] -    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:51:12] + warning: `[BarC]` cannot be resolved, ignoring it...
[01:51:12] +    |
[01:51:12] +    |
[01:51:12] + LL | bar [BarC] bar
[01:51:12] +    |
[01:51:12] +    |
[01:51:12] +    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:51:12] 151 
[01:51:12] - warning: `[BarD]` cannot be resolved, ignoring it...
[01:51:12] -    |
[01:51:12] -    |
[01:51:12] - LL | #[doc = "Foo/nbar [BarD] bar/nbaz"]
[01:51:12] -    |
[01:51:12] -    = note: the link appears in this line:
[01:51:12] -            
[01:51:12] -            
[01:51:12] -            bar [BarD] bar
[01:51:12] -                 ^^^^
[01:51:12] -    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:51:12] + warning: `[BarD]` cannot be resolved, ignoring it...
[01:51:12] +    |
[01:51:12] +    |
[01:51:12] + LL | #[doc = "Foo/nbar [BarD] bar/nbaz"]
[01:51:12] +    |
[01:51:12] +    = note: the link appears in this line:
[01:51:12] +            
[01:51:12] +            
[01:51:12] +            bar [BarD] bar
[01:51:12] +                 ^^^^
[01:51:12] +    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:51:12] 163 
[01:51:12] - warning: `[BarF]` cannot be resolved, ignoring it...
[01:51:12] -    |
[01:51:12] -    |
[01:51:12] - LL |         #[doc = $f]
[01:51:12] - ...
[01:51:12] - ...
[01:51:12] - LL | f!("Foo/nbar [BarF] bar/nbaz");
[01:51:12] -    | ------------------------------- in this macro invocation
[01:51:12] -    = note: the link appears in this line:
[01:51:12] -            
[01:51:12] -            
[01:51:12] -            bar [BarF] bar
[01:51:12] -                 ^^^^
[01:51:12] -    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:51:12] + warning: `[BarF]` cannot be resolved, ignoring it...
[01:51:12] +    |
[01:51:12] +    |
[01:51:12] + LL |         #[doc = $f]
[01:51:12] + ...
[01:51:12] + ...
[01:51:12] + LL | f!("Foo/nbar [BarF] bar/nbaz");
[01:51:12] +    | ------------------------------- in this macro invocation
[01:51:12] +    = note: the link appears in this line:
[01:51:12] +            
[01:51:12] +            
[01:51:12] +            bar [BarF] bar
[01:51:12] +                 ^^^^
[01:51:12] +    = help: to escape `[` and `]` characters, just add '/' before them like `/[` or `/]`
[01:51:12] 179 
[01:51:12] 
[01:51:12] 
[01:51:12] The actual stderr differed from the expected stderr.
[01:51:12] The actual stderr differed from the expected stderr.
[01:51:12] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-links-warning/intra-links-warning.stderr
[01:51:12] To update references, rerun the tests and pass the `--bless` flag
[01:51:12] To only update this specific test, also pass `--test-args intra-links-warning.rs`
[01:51:12] error: 1 errors occurred comparing output.
[01:51:12] status: exit code: 0
[01:51:12] status: exit code: 0
[01:51:12] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/intra-links-warning.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-links-warning/a" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-links-warning/auxiliary"
[01:51:12] ------------------------------------------
[01:51:12] 
[01:51:12] ------------------------------------------
[01:51:12] stderr:
[01:51:12] stderr:
[01:51:12] ------------------------------------------
[01:51:12] warning: `[Foo::baz]` cannot be resolved, ignoring it...
[01:51:12]    |
[01:51:12]    |
[01:51:12] LL |        //! Test with [Foo::baz], [Bar::foo], ...
[01:51:12]    |
[01:51:12]    = note: #[warn(intra_doc_link_resolution_failure)] on by default
[01:51:12]    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:51:12] 
[01:51:12] 
[01:51:12] warning: `[Bar::foo]` cannot be resolved, ignoring it...
[01:51:12]   --> /checkout/src/test/rustdoc-ui/intra-links-warning.rs:3:35
[01:51:12]    |
[01:51:12] LL |        //! Test with [Foo::baz], [Bar::foo], ...
[01:51:12]    |
[01:51:12]    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:51:12] 
[01:51:12] 
[01:51:12] warning: `[Uniooon::X]` cannot be resolved, ignoring it...
[01:51:12]    |
[01:51:12]    |
[01:51:12] LL |      //! , [Uniooon::X] and [Qux::Z].
[01:51:12]    |
[01:51:12]    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:51:12] 
[01:51:12] 
[01:51:12] warning: `[Qux::Z]` cannot be resolved, ignoring it...
[01:51:12]    |
[01:51:12]    |
[01:51:12] LL |      //! , [Uniooon::X] and [Qux::Z].
[01:51:12]    |
[01:51:12]    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:51:12] 
[01:51:12] 
[01:51:12] warning: `[Uniooon::X]` cannot be resolved, ignoring it...
[01:51:12]    |
[01:51:12]    |
[01:51:12] LL |       //! , [Uniooon::X] and [Qux::Z].
[01:51:12]    |
[01:51:12]    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:51:12] 
[01:51:12] 
[01:51:12] warning: `[Qux::Z]` cannot be resolved, ignoring it...
[01:51:12]    |
[01:51:12]    |
[01:51:12] LL |       //! , [Uniooon::X] and [Qux::Z].
[01:51:12]    |
[01:51:12]    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:51:12] 
[01:51:12] 
[01:51:12] warning: `[Qux:Y]` cannot be resolved, ignoring it...
[01:51:12]    |
[01:51:12]    |
[01:51:12] LL |        /// [Qux:Y]
[01:51:12]    |
[01:51:12]    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:51:12] 
[01:51:12] warning: `[error]` cannot be resolved, ignoring it...
[01:51:12] warning: `[error]` cannot be resolved, ignoring it...
[01:51:12]   --> /checkout/src/test/rustdoc-ui/intra-links-warning.rs:51:30
[01:51:12]    |
[01:51:12] LL |  * time to introduce a link [error]*/
[01:51:12]    |
[01:51:12]    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:51:12] 
[01:51:12] warning: `[error]` cannot be resolved, ignoring it...
[01:51:12] warning: `[error]` cannot be resolved, ignoring it...
[01:51:12]   --> /checkout/src/test/rustdoc-ui/intra-links-warning.rs:57:30
[01:51:12]    |
[01:51:12] LL |  * time to introduce a link [error]
[01:51:12]    |
[01:51:12]    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:51:12] 
[01:51:12] warning: `[error]` cannot be resolved, ignoring it...
[01:51:12] warning: `[error]` cannot be resolved, ignoring it...
[01:51:12]   --> /checkout/src/test/rustdoc-ui/intra-links-warning.rs:61:1
[01:51:12]    |
[01:51:12] LL | #[doc = "single line [error]"]
[01:51:12]    |
[01:51:12]    = note: the link appears in this line:
[01:51:12]            
[01:51:12]            single line [error]
[01:51:12]            single line [error]
[01:51:12]                         ^^^^^
[01:51:12]    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:51:12] 
[01:51:12] warning: `[error]` cannot be resolved, ignoring it...
[01:51:12]   --> /checkout/src/test/rustdoc-ui/intra-links-warning.rs:64:1
[01:51:12]    |
[01:51:12] LL | #[doc = "single line with \"escaping\" [error]"]
[01:51:12]    |
[01:51:12]    = note: the link appears in this line:
[01:51:12]            
[01:51:12]            
[01:51:12]            single line with "escaping" [error]
[01:51:12]    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:51:12] 
[01:51:12] warning: `[error]` cannot be resolved, ignoring it...
[01:51:12]   --> /checkout/src/test/rustdoc-ui/intra-links-warning.rs:67:1
[01:51:12]   --> /checkout/src/test/rustdoc-ui/intra-links-warning.rs:67:1
[01:51:12]    |
[01:51:12] LL | / /// Item docs.
[01:51:12] LL | | #[doc="Hello there!"]
[01:51:12] LL | | /// [error]
[01:51:12]    |
[01:51:12]    = note: the link appears in this line:
[01:51:12]            
[01:51:12]            [error]
[01:51:12]            [error]
[01:51:12]             ^^^^^
[01:51:12]    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:51:12] 
[01:51:12] warning: `[error1]` cannot be resolved, ignoring it...
[01:51:12]   --> /checkout/src/test/rustdoc-ui/intra-links-warning.rs:73:11
[01:51:12]    |
[01:51:12] LL | /// docs [error1]
[01:51:12]    |
[01:51:12]    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:51:12] 
[01:51:12] warning: `[error2]` cannot be resolved, ignoring it...
[01:51:12] warning: `[error2]` cannot be resolved, ignoring it...
[01:51:12]   --> /checkout/src/test/rustdoc-ui/intra-links-warning.rs:75:11
[01:51:12]    |
[01:51:12] LL | /// docs [error2]
[01:51:12]    |
[01:51:12]    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:51:12] 
[01:51:12] 
[01:51:12] warning: `[BarA]` cannot be resolved, ignoring it...
[01:51:12]    |
[01:51:12]    |
[01:51:12] LL | /// bar [BarA] bar
[01:51:12]    |
[01:51:12]    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:51:12] 
[01:51:12] 
[01:51:12] warning: `[BarB]` cannot be resolved, ignoring it...
[01:51:12]    |
[01:51:12]    |
[01:51:12] LL |  * bar [BarB] bar
[01:51:12]    |
[01:51:12]    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:51:12] 
---
[01:51:12] -   --> $DIR/invalid-syntax.rs:3:5
[01:51:12] -    |
[01:51:12] - LL |   /// 