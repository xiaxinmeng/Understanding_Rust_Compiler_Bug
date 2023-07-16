plain
travis_time:end:1176d3fe:start=1549042244360760462,finish=1549042386939791067,duration=142579030605
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:59:27] .................................................................................................... 2600/5359
[00:59:30] .................................................................................................... 2700/5359
[00:59:35] .................................................................................................... 2800/5359
[00:59:39] .................................................................................................... 2900/5359
[00:59:42] ........F........................................................................................... 3000/5359
[00:59:49] .................................................................................................... 3200/5359
[00:59:52] ...................i................................................................................ 3300/5359
[00:59:56] ....................................................................................ii...i..ii...... 3400/5359
[00:59:59] .................................................................................................... 3500/5359
---
[01:01:07] failures:
[01:01:07] 
[01:01:07] ---- [ui] ui/issues/issue-57979.rs stdout ----
[01:01:07] normalized stderr:
[01:01:07] error[E0666]: nested `impl Trait` is not allowed
[01:01:07]    |
[01:01:07]    |
[01:01:07] LL | pub fn collect(_: impl IntoIterator<Item = impl Borrow<Data<impl AsRef<[u8]>>>>) {
[01:01:07]    |                                            |                |
[01:01:07]    |                                            |                |
[01:01:07]    |                                            |                nested `impl Trait` here
[01:01:07]    |                                            outer `impl Trait`
[01:01:07] 
[01:01:07] error[E0601]: `main` function not found in crate `issue_57979`
[01:01:07]    |
[01:01:07]    = note: consider adding a `main` function to `$DIR/issue-57979.rs`
[01:01:07] error: aborting due to 2 previous errors
[01:01:07] 
[01:01:07] Some errors occurred: E0601, E0666.
[01:01:07] For more information about an error, try `rustc --explain E0601`.
[01:01:07] For more information about an error, try `rustc --explain E0601`.
[01:01:07] 
[01:01:07] 
[01:01:07] 
[01:01:07] The actual stderr differed from the expected stderr.
[01:01:07] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-57979/issue-57979.stderr
[01:01:07] To update references, rerun the tests and pass the `--bless` flag
[01:01:07] To only update this specific test, also pass `--test-args issues/issue-57979.rs`
[01:01:07] error: 1 errors occurred comparing output.
[01:01:07] status: exit code: 1
[01:01:07] status: exit code: 1
[01:01:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-57979.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-57979/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-57979/auxiliary" "-A" "unused"
[01:01:07] ------------------------------------------
[01:01:07] 
[01:01:07] ------------------------------------------
[01:01:07] stderr:
[01:01:07] stderr:
[01:01:07] ------------------------------------------
[01:01:07] {"message":"nested `impl Trait` is not allowed","code":{"code":"E0666","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-57979.rs","byte_start":1209,"byte_end":1244,"line_start":39,"line_end":39,"column_start":44,"column_end":79,"is_primary":false,"text":[{"text":"pub fn collect(_: impl IntoIterator<Item = impl Borrow<Data<impl AsRef<[u8]>>>>) {","highlight_start":44,"highlight_end":79}],"label":"outer `impl Trait`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/issues/issue-57979.rs","byte_start":1226,"byte_end":1242,"line_start":39,"line_end":39,"column_start":61,"column_end":77,"is_primary":true,"text":[{"text":"pub fn collect(_: impl IntoIterator<Item = impl Borrow<Data<impl AsRef<[u8]>>>>) {","highlight_start":61,"highlight_end":77}],"label":"nested `impl Trait` here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0666]: nested `impl Trait` is not allowed\n  --> /checkout/src/test/ui/issues/issue-57979.rs:39:61\n   |\nLL | pub fn collect(_: impl IntoIterator<Item = impl Borrow<Data<impl AsRef<[u8]>>>>) {\n   |                                            -----------------^^^^^^^^^^^^^^^^--\n   |                                            |                |\n   |                                            |                nested `impl Trait` here\n   |                                            outer `impl Trait`\n\n"}
[01:01:07] {"message":"`main` function not found in crate `issue_57979`","code":{"code":"E0601","explanation":"\nNo `main` function was found in a binary crate. To fix this error, add a\n`main` function. For example:\n\n