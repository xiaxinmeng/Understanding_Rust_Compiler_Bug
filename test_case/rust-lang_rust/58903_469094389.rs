plain
travis_time:end:012dcdb5:start=1551661260287692129,finish=1551661332861818742,duration=72574126613
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[01:07:27] ............................................ii...................................................... 3800/5430
[01:07:29] ..............................................................i..................................... 3900/5430
[01:07:31] .................................................................................................... 4000/5430
[01:07:33] ....................i............................................................................... 4100/5430
[01:07:36] .................F.................................................................................. 4200/5430
[01:07:50] .................................................................................................... 4400/5430
[01:07:53] .................................................................................................... 4500/5430
[01:07:57] ...............................................................................i.................... 4600/5430
[01:08:03] .................................................................................................... 4700/5430
---
[01:08:27] 1 error: incorrect close delimiter: `}`
[01:08:27] -   --> $DIR/unclosed_delim_mod.rs:6:1
[01:08:27] +   --> $DIR/unclosed_delim_mod.rs:5:1
[01:08:27] 3    |
[01:08:27] 4 LL | pub fn new() -> Result<Value, ()> {
[01:08:27] 
[01:08:27] 
[01:08:27] The actual stderr differed from the expected stderr.
[01:08:27] The actual stderr differed from the expected stderr.
[01:08:27] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/unclosed-delimiter-in-dep/unclosed-delimiter-in-dep.stderr
[01:08:27] To update references, rerun the tests and pass the `--bless` flag
[01:08:27] To only update this specific test, also pass `--test-args parser/unclosed-delimiter-in-dep.rs`
[01:08:27] error: 1 errors occurred comparing output.
[01:08:27] status: exit code: 1
[01:08:27] status: exit code: 1
[01:08:27] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/unclosed-delimiter-in-dep.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/busted_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: incorrect close delimiter: `}`\n  --> /checkout/src/test/ui/parser/unclosed_delim_mod.rs:5:1\n   |\nLL | pub fn new() -> Result<Value, ()> {\n   |                                   - close delimiter possibly meant for this\nLL |     Ok(Value {\n   |       - un-closed delimiter\nLL |     }\nLL | }\n   | ^ incorrect close delimiter\n\n"}
[01:08:27] {"message":"mismatched types","code":{"code":"E0308","explanation":"\nThis error occurs when the compiler was unable to infer the concrete type of a\nvariable. It can occur for several cases, the most common of which is a\nmismatch in the expected type that the compiler inferred for a variable's\ninitializing expression, and the actual type explicitly assigned to the\nvariable.\n\nFor example:\n\n