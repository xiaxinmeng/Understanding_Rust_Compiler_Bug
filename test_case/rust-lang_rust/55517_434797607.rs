plain
travis_time:end:03b94990:start=1541007419406398813,finish=1541007484369787706,duration=64963388893
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:47:25] .................................................................................................... 1200/4989
[00:47:27] .................................................................................................... 1300/4989
[00:47:30] .................................................................................................... 1400/4989
[00:47:32] ..................................................................................i................. 1500/4989
[00:47:35] ........................................................i..F........................................ 1600/4989
[00:47:42] .................................................................................................... 1800/4989
[00:47:46] ................................................................................................i... 1900/4989
[00:47:49] .................................................................................................... 2000/4989
[00:47:53] .................................................................................................... 2100/4989
---
[00:49:16] ............................................................i....................................... 4600/4989
[00:49:20] .................................................................................................... 4700/4989
[00:49:23] .................................................................................................... 4800/4989
[00:49:26] .................................................................................................... 4900/4989
, 'a) })...
[00:49:28] 8    = note: ...so that the types are compatible:
[00:49:28] 9            expected Foo<&'a isize>
[00:49:28] 10               found Foo<&isize>
[00:49:28] 
[00:49:28] The actual stderr differed from the expected stderr.
[00:49:28] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hrtb/hrtb-perfect-forwarding/hrtb-perfect-forwarding.stderr
[00:49:28] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hrtb/hrtb-perfect-forwarding/hrtb-perfect-forwarding.stderr
[00:49:28] To update references, rerun the tests and pass the `--bless` flag
[00:49:28] To only update this specific test, also pass `--test-args hrtb/hrtb-perfect-forwarding.rs`
[00:49:28] error: 1 errors occurred comparing output.
[00:49:28] status: exit code: 1
[00:49:28] status: exit code: 1
[00:49:28] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/hrtb/hrtb-perfect-forwarding.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hrtb/hrtb-perfect-forwarding/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hrtb/hrtb-perfect-forwarding/auxiliary" "-A" "unused"
[00:49:28] ------------------------------------------
[00:49:28] 
[00:49:28] ------------------------------------------
[00:49:28] stderr:
[00:49:28] stderr:
[00:49:28] ------------------------------------------
[00:49:28] {"message":"cannot infer an appropriate lifetime due to conflicting requirements","code":{"code":"E0495","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/hrtb/hrtb-perfect-forwarding.rs","byte_start":1640,"byte_end":1656,"line_start":56,"line_end":56,"column_start":5,"column_end":21,"is_primary":true,"text":[{"text":"    foo_hrtb_bar_not(&mut t); //~ ERROR E0495","highlight_start":5,"highlight_end":21}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"first, the lifetime cannot outlive lifetime RePlaceholder(Placeholder { universe: U2, name: BrNamed(crate0:DefIndex(1:23), 'a) })...","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"...so that the types are compatible:\nexpected Foo<&'a isize>\n   found Foo<&isize>","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"but, the lifetime must be valid for the lifetime 'b as defined on the function body at 49:21...","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/hrtb/hrtb-perfect-forwarding.rs","byte_start":1305,"byte_end":1307,"line_start":49,"line_end":49,"column_start":21,"column_end":23,"is_primary":true,"text":[{"text":"fn foo_hrtb_bar_not<'b,T>(mut t: T)","highlight_start":21,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"...so that the types are compatible:\nexpected Bar<&isize>\n   found Bar<&'b isize>","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0495]: cannot infer an appropriate lifetime due to conflicting requirements\n  --> /checkout/src/test/ui/hrtb/hrtb-perfect-forwarding.rs:56:5\n   |\nLL |     foo_hrtlinux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib
70532 ./obj/build/x86_64-unknown-linux-gnu/native
70300 ./obj/build/x86_64-unknown-linux-gnu/native/jemalloc
67720 ./src/test
67612 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps
