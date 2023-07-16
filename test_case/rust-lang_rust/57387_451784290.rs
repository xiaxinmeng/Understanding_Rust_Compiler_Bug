plain
travis_time:end:02050ca4:start=1546812741754778094,finish=1546812742729666673,duration=974888579
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:59:03] 
[00:59:03] 30    |
[00:59:03] 31    = help: add #![feature(non_ascii_idents)] to the crate attributes to enable
[00:59:03] 32 
[00:59:03] - warning: type parameter `γ` should have a camel case name such as `Γ`
[00:59:03] + warning: type parameter `γ` should have a camel case name
[00:59:03] 35    |
[00:59:03] 35    |
[00:59:03] 36 LL |     γ  //~ ERROR non-ascii idents are not fully supported
[00:59:03] -    |     ^
[00:59:03] -    |     ^
[00:59:03] +    |     ^ help: convert the identifier to camel case: `Γ`
[00:59:03] 39    = note: #[warn(non_camel_case_types)] on by default
[00:59:03] 40 
[00:59:03] 
[00:59:03] 
[00:59:03] 
[00:59:03] The actual stderr differed from the expected stderr.
[00:59:03] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/utf8_idents/utf8_idents.stderr
[00:59:03] To update references, rerun the tests and pass the `--bless` flag
[00:59:03] To only update this specific test, also pass `--test-args utf8_idents.rs`
[00:59:03] error: 1 errors occurred comparing output.
[00:59:03] status: exit code: 1
[00:59:03] status: exit code: 1
[00:59:03] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/utf8_idents.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/utf8_idents/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/i/utf8_idents.rs:5:5\n   |\nLL |     γ  //~ ERROR non-ascii idents are not fully supported\n   |     ^\n   |\n   = help: add #![feature(non_ascii_idents)] to the crate attributes to enable\n\n"}
[00:59:03] {"message":"non-ascii idents are not fully supported. (see issue #55467)","code":{"code":"E0658","explanation":"\nAn unstable feature was used.\n\nErroneous code example:\n\n