plain
travis_time:end:1a95c454:start=1541168522906086774,finish=1541168579150225529,duration=56244138755
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:45:55] .................................................................................................... 3900/4984
[00:45:57] .................................................................................................... 4000/4984
[00:46:00] .................................................................................................... 4100/4984
[00:46:04] .................................................................................i.................. 4200/4984
[00:46:09] ....................................................................................F............... 4300/4984
[00:46:15] .................................................................................................... 4500/4984
[00:46:18] .......................................................i............................................ 4600/4984
[00:46:22] .................................................................................................... 4700/4984
[00:46:24] .................................................................................................... 4800/4984
---
[00:46:29] ---- [ui] ui/rust-2018/trait-import-suggestions.rs stdout ----
[00:46:29] diff of stderr:
[00:46:29] 
[00:46:29] 25    |
[00:46:29] 26 LL |     x.extern_baz();
[00:46:29] - help: the following trait is implemented but not in scope, perhaps add a `use` for it:
[00:46:29] -    |
[00:46:29] -    |
[00:46:29] - LL | use baz::BazTrait;
[00:46:29] 32 
[00:46:29] 32 
[00:46:29] 33 error[E0599]: no function or associated item named `from_str` found for type `u32` in the current scope
[00:46:29] 
[00:46:29] 
[00:46:29] The actual stderr differed from the expected stderr.
[00:46:29] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/trait-import-suggestions/trait-import-suggestions.stderr
[00:46:29] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/trait-import-suggestions/trait-import-suggestions.stderr
[00:46:29] To update references, rerun the tests and pass the `--bless` flag
[00:46:29] To only update this specific test, also pass `--test-args rust-2018/trait-import-suggestions.rs`
[00:46:29] error: 1 errors occurred comparing output.
[00:46:29] status: exit code: 1
[00:46:29] status: exit code: 1
[00:46:29] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rust-2018/trait-import-suggestions.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/trait-import-suggestions/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "--extern" "baz" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/trait-import-suggestions/auxiliary" "-A" "unused"
[00:46:29] ------------------------------------------
[00:46:29] 
[00:46:29] ------------------------------------------
[00:46:29] stderr:
[00:46:29] stderr:
[00:46:29] ------------------------------------------
[00:46:29] {"message":"no method named `foobar` found for type `u32` in the current scope","code":{"code":"E0599","explanation":"\nThis error occurs when a method is used on a type which doesn't implement it:\n\nErroneous code example:\n\n