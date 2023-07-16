plain
travis_time:end:2c99647f:start=1548806593387522263,finish=1548806594376967118,duration=989444855
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[01:07:49] .................................................................................................... 600/2947
[01:08:01] .................................................................................................... 700/2947
[01:08:10] .................................................................................................... 800/2947
[01:08:18] .................................................................................................... 900/2947
[01:08:34] ...........................................F........................................................ 1000/2947
[01:08:56] .................................................................................................... 1200/2947
[01:09:06] .................................................................................................... 1300/2947
[01:09:19] .................................................................................................... 1400/2947
[01:09:30] .................................................................................................... 1500/2947
---
[01:13:04] failures:
[01:13:04] 
[01:13:04] ---- [run-pass] run-pass/impl-trait/nested.rs stdout ----
[01:13:04] 
[01:13:04] error: test compilation failed although it shouldn't!
[01:13:04] status: exit code: 1
[01:13:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/impl-trait/nested.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/impl-trait/nested/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/impl-trait/nested/auxiliary"
[01:13:04] ------------------------------------------
[01:13:04] 
[01:13:04] ------------------------------------------
[01:13:04] stderr:
[01:13:04] stderr:
[01:13:04] ------------------------------------------
[01:13:04] {"message":"nested `impl Trait` is not allowed","code":{"code":"E0666","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/run-pass/impl-trait/nested.rs","byte_start":92,"byte_end":115,"line_start":6,"line_end":6,"column_start":12,"column_end":35,"is_primary":false,"text":[{"text":"fn test(_: impl Borrow<impl Debug>) {}","highlight_start":12,"highlight_end":35}],"label":"outer `impl Trait`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/run-pass/impl-trait/nested.rs","byte_start":104,"byte_end":114,"line_start":6,"line_end":6,"column_start":24,"column_end":34,"is_primary":true,"text":[{"text":"fn test(_: impl Borrow<impl Debug>) {}","highlight_start":24,"highlight_end":34}],"label":"nested `impl Trait` here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0666]: nested `impl Trait` is not allowed\n  --> /checkout/src/test/run-pass/impl-trait/nested.rs:6:24\n   |\nLL | fn test(_: impl Borrow<impl Debug>) {}\n   |            ------------^^^^^^^^^^-\n   |            |           |\n   |            |           nested `impl Trait` here\n   |            outer `impl Trait`\n\n"}
[01:13:04] {"message":"failed to resolve: maybe a missing `extern crate core;`?","code":{"code":"E0433","explanation":"\nAn undeclared type or module was used.\n\nErroneous code example:\n\n