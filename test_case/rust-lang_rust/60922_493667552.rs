plain
travis_time:end:0be7a369:start=1558172050423029777,finish=1558172137716894751,duration=87293864974
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:09:20] .................................................................................................... 2700/5534
[01:09:25] .................................................................................................... 2800/5534
[01:09:29] .................................................................................................... 2900/5534
[01:09:33] .................................................................................................... 3000/5534
[01:09:37] ..............................F..................................................................... 3100/5534
[01:09:45] .................................................................................................... 3300/5534
[01:09:49] ................................i................................................................... 3400/5534
[01:09:53] .................................................................................................... 3500/5534
[01:09:56] ......ii...i..ii.................................................................................... 3600/5534
---
[01:11:13] 
[01:11:13] 
[01:11:13] The actual stderr differed from the expected stderr.
[01:11:13] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-60283/issue-60283.stderr
[01:11:13] To update references, rerun the tests and pass the `--bless` flag
[01:11:13] To only update this specific test, also pass `--test-args issues/issue-60283.rs`
[01:11:13] error: 1 errors occurred comparing output.
[01:11:13] status: exit code: 1
[01:11:13] status: exit code: 1
[01:11:13] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-60283.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-60283/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-60283/auxiliary" "-A" "unused"
[01:11:13] ------------------------------------------
[01:11:13] 
[01:11:13] ------------------------------------------
[01:11:13] stderr:
[01:11:13] stderr:
[01:11:13] ------------------------------------------
[01:11:13] {"message":"type mismatch in function arguments","code":{"code":"E0631","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-60283.rs","byte_start":215,"byte_end":218,"line_start":14,"line_end":14,"column_start":5,"column_end":8,"is_primary":true,"text":[{"text":"    foo((), drop)","highlight_start":5,"highlight_end":8}],"label":"expected signature of `for<'a> fn(<() as Trait<'a>>::Item) -> _`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/issues/issue-60283.rs","byte_start":215,"byte_end":218,"line_start":14,"line_end":14,"column_start":5,"column_end":8,"is_primary":true,"text":[{"text":"    foo((), drop)","highlight_start":5,"highlight_end":8}],"label":"found signature of `fn(_) -> _`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"required by `foo`","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-60283.rs","byte_start":91,"byte_end":197,"line_start":9,"line_end":11,"column_start":1,"column_end":50,"is_primary":true,"text":[{"text":"pub fn foo<T, F>(_: T, _: F)","highlight_start":1,"highlight_end":29},{"text":"where T: for<'a> Trait<'a>,","highlight_start":1,"highlight_end":28},{"text":"      F: for<'a> FnMut(<T as Trait<'a>>::Item) {}","highlight_start":1,"highlight_end":50}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0631]: type mismatch in function arguments\n  --> /checkout/src/test/ui/issues/issue-60283.rs:14:5\n   |\nLL |     foo((), drop)\n   |     ^^^\n   |     |\n   |     expected signature of `for<'a> fn(<() as Trait<'a>>::Item) -> _`\n   |     found signature of `fn(_) -> _`\n   |\nnote: required by `foo`\n  --> /checkout/src/test/ui/issues/issue-60283.rs:9:1\n   |\nLL | / pub fn foo<T, F>(_: T, _: F)\nLL | | where T: for<'a> Trait<'a>,\nLL | |       F: for<'a> FnMut(<T as Trait<'a>>::Item) {}\n   | |_________________________________________________^\n\n"}
[01:11:13] {"message":"type mismatch resolving `for<'a> <fn(_) {std::mem::drop::<_>} as std::ops::FnOnce<(<() as Trait<'a>>::Item,)>>::Output == ()`","code":{"code":"E0271","explanation":"\nThis is because of a type mismatch between the associated type of some\ntrait (e.g., `T::Bar`, where `T` implements `trait Quux { type Bar; }`)\nand another type `U` that is required to be equal to `T::Bar`, but is not.\nExamples follow.\n\nHere is a basic example:\n\n