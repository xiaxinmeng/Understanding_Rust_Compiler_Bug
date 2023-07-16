plain
travis_time:end:15f77c2e:start=1541261676620823550,finish=1541261678798888953,duration=2178065403
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:53:10] .................................................................................................... 4400/4984
[00:53:13] .................................................................................................... 4500/4984
[00:53:17] .......................................................i............................................ 4600/4984
[00:53:21] .................................................................................................... 4700/4984
user-annotations/dump-fn-method.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/dump-fn-method/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/dump-fn-method/auxiliary" "-A" "unused"
[00:53:28] ------------------------------------------
[00:53:28] 
[00:53:28] ------------------------------------------
[00:53:28] stderr:
[00:53:28] stderr:
[00:53:28] ------------------------------------------
[00:53:28] {"message":"user substs: Canonical { max_universe: U0, variables: [], value: UserSubsts { substs: [u32], user_self_ty: None } }","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/nll/user-annotations/dump-fn-method.rs","byte_start":975,"byte_end":985,"line_start":36,"line_end":36,"column_start":13,"column_end":23,"is_primary":true,"text":[{"text":"    let x = foo::<u32>; //~ ERROR [u32]","highlight_start":13,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: user substs: Canonical { max_universe: U0, variables: [], value: UserSubsts { substs: [u32], user_self_ty: None } }\n  --> /checkout/src/test/ui/nll/user-annotations/dump-fn-method.rs:36:13\n   |\nLL |     let x = foo::<u32>; //~ ERROR [u32]\n   |             ^^^^^^^^^^\n\n"}
[00:53:28] {"message":"user substs: Canonical { max_universe: U0, variables: [Canon
