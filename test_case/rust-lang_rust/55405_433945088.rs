plain
travis_time:end:096a545e:start=1540824806913893220,finish=1540824873050737394,duration=66136844174
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
---
[00:08:56] 1769 | |         }
[00:08:56] 1770 | |     }
[00:08:56]      | |_____- defined here
[00:08:56] ...
[00:08:56] 3098 |                   let path = P(self.lower_path_extra(def, &prefix, None, ParamMode::Explicit, None));
[00:08:56] 
[00:09:10] error: aborting due to previous error
[00:09:10] 
[00:09:10] For more information about this error, try `rustc --explain E0061`.
---
[00:09:10] travis_time:end:stage0-rustc:start=1540825270115460925,finish=1540825433396964043,duration=163281503118

[00:09:10] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:09:10] expected success, got: exit code: 101
[00:09:10] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1115:9
[00:09:10] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:09:10] Build completed unsuccessfully in 0:03:39
[00:09:10] Makefile:28: recipe for target 'all' failed
[00:09:10] Makefile:28: recipe for target 'all' failed
[00:09:10] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:06645b70
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
