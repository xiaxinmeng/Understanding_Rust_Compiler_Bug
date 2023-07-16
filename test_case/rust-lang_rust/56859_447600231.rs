plain
travis_time:end:08a6f346:start=1544907423729478235,finish=1544907478003460520,duration=54273982285
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:49:00] .................................................................................................... 2400/5108
[00:49:04] .................................................................................................... 2500/5108
[00:49:07] .................................................................................................... 2600/5108
[00:49:11] .................................................................................................... 2700/5108
[00:49:15] ......................................................................................F...F......... 2800/5108
[00:49:21] .................................................................................................... 3000/5108
[00:49:24] ...............................................................................i.................... 3100/5108
[00:49:27] .................................................................................................... 3200/5108
[00:49:31] ..........................................ii..i..ii................................................. 3300/5108
---
[00:50:14] .................................................................................................... 4600/5108
[00:50:18] .......................................................................i............................ 4700/5108
[00:50:20] .................................................................................................... 4800/5108
[00:50:24] .................................................................................................... 4900/5108
_end":21}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"did you mean to use one of the enum's variants?","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: the `Self` constructor can only be used with tuple or unit structs\n  --> /checkout/src/test/ui/issues/issue-56199.rs:7:17\n   |\nLL |         let _ = Self;\n   |                 ^^^^\n   |\n   = note: did you mean to use one of the enum's variants?\n\n"}
[00:50:29] {"message":"the `Self` constructor can only be used with tuple or unit structs","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-56199.rs","byte_start":178,"byte_end":182,"line_start":9,"line_end":9,"column_start":17,"column_end":21,"is_primary":true,"text":[{"text":"        let _ = Self();","highlight_start":17,"highlight_end":21}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"did you mean to use one of the enum's variants?","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: the `Self` constructor can only be used with tuple or unit structs\n  --> /checkout/src/test/ui/issues/issue-56199.rs:9:17\n   |\nLL |         let _ = Self();\n   |                 ^^^^\n   |\n   = note: did you mean to use one of the enum's variants?\n\n"}
[00:50:29] {"message":"the `Self` constructor can only be used with tuple or unit structs","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-56199.rs","byte_start":323,"byte_end":327,"line_start":1stc/x86_64-unknown-linux-gnu
123888 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps
122820 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps
115800 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/release
115348 ./src/llvm/test/CodeGen
