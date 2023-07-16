plain
travis_time:end:167e41b2:start=1557851599470721981,finish=1557851600226578379,duration=755856398
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:08:05]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:08:13] error: unnecessary parentheses around `if` condition
[00:08:13]     --> src/libsyntax/parse/parser.rs:2631:27
[00:08:13]      |
[00:08:13] 2631 |                   } else if (self.span.rust_2018() && self.eat_keyword(keywords::Await)
[00:08:13]      |  ___________________________^
[00:08:13] 2632 | |                            && self.check(&token::Not)) {
[00:08:13]      |
[00:08:13]      = note: `-D unused-parens` implied by `-D warnings`
[00:08:13] help: remove these parentheses
[00:08:13]      |
[00:08:13]      |
[00:08:13] 2631 |                 } else if self.span.rust_2018() && self.eat_keyword(keywords::Await)
[00:08:13] 2632 |                            && self.check(&token::Not) {
[00:08:13] 
[00:08:19] error: aborting due to previous error
[00:08:19] 
[00:08:19] error: Could not compile `syntax`.
---
travis_time:end:1de8a8ea:start=1557852110857480098,finish=1557852110862269841,duration=4789743
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:07d23bcc
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0117d5ab
$ cat ./obj/build/x86_64-un
