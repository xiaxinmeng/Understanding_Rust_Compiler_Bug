plain
travis_time:end:2d0433a0:start=1556706575272918969,finish=1556706576033799254,duration=760880285
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:51:18]    Compiling racer v2.1.22
[01:53:16] error[E0599]: no method named `as_str` found for type `&str` in the current scope
[01:53:16]    --> src/tools/rls/rls/src/build/cargo.rs:203:41
[01:53:16]     |
[01:53:16] 203 |         .map(|pkg_spec| pkg_spec.name().as_str().to_owned())
[01:53:16]     |                                         ^^^^^^ help: there is a method with a similar name: `as_ptr`
[01:53:17] error[E0061]: this function takes 8 parameters but 7 parameters were supplied
[01:53:17]    --> src/tools/rls/rls/src/project_model.rs:218:5
[01:53:17]     |
[01:53:17]     |
[01:53:17] 218 |     ops::resolve_with_previous(registry, ws, Method::Everything, prev, None, &[], true)
[01:53:17] 
[01:53:17] error: aborting due to 2 previous errors
[01:53:17] 
[01:53:17] Some errors have detailed explanations: E0061, E0599.
---
[01:59:50] +
[01:59:50] +
[01:59:50] 
[01:59:50] The actual stderr differed from the expected stderr.
[01:59:50] Actual stderr saved to /tmp/compiletestbiZ3lC/async-fn.stderr
[01:59:50] To update references, run this command from build directory:
[01:59:50] tests/run-pass/update-references.sh '/tmp/compiletestbiZ3lC' 'async-fn.rs'
[01:59:50] error: 1 errors occurred comparing output.
[01:59:50] status: exit code: 1
[01:59:50] status: exit code: 1
[01:59:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/async-fn.rs" "-L" "/tmp/compiletestbiZ3lC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestbiZ3lC/async-fn.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "-L" "/tmp/compiletestbiZ3lC/async-fn.stage-id.aux" "-A" "unused"
[01:59:50] ------------------------------------------
[01:59:50] 
[01:59:50] ------------------------------------------
[01:59:50] stderr:
---
[01:59:55] +For more information about this error, try `rustc --explain E0080`.
[01:59:55] +
[01:59:55] 
[01:59:55] The actual stderr differed from the expected stderr.
[01:59:55] Actual stderr saved to /tmp/compiletestbiZ3lC/hashmap.stderr
[01:59:55] To update references, run this command from build directory:
[01:59:55] tests/run-pass/update-references.sh '/tmp/compiletestbiZ3lC' 'hashmap.rs'
[01:59:55] error: 1 errors occurred comparing output.
[01:59:55] status: exit code: 1
[01:59:55] status: exit code: 1
[01:59:55] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/hashmap.rs" "-L" "/tmp/compiletestbiZ3lC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestbiZ3lC/hashmap.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "-Zmiri-seed=0000000000000000" "-L" "/tmp/compiletestbiZ3lC/hashmap.stage-id.aux" "-A" "unused"
[01:59:55] ------------------------------------------
[01:59:55] 
[01:59:55] ------------------------------------------
[01:59:55] stderr:
---
[02:00:06] Verifying status of edition-guide...
[02:00:06] Verifying status of rls...
[02:00:06] This PR updated 'src/tools/rls', verifying if status is 'test-pass'...
[02:00:06] 
[02:00:06] ⚠️ We detected that this PR updated 'rls', but its tests failed.
[02:00:06] 
[02:00:06] If you do intend to update 'rls', please check the error messages above and
[02:00:06] commit another update.
[02:00:06] 
[02:00:06] If you do NOT intend to update 'rls', please ensure you did not accidentally
[02:00:06] change the submodule at 'src/tools/rls'. You may ask your reviewer for the
[02:00:06] proper steps.
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 3.
travis_time:start:2a0d02c0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed May  1 12:29:54 UTC 2019
