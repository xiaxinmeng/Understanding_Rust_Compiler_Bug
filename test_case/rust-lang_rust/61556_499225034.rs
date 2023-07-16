plain
travis_time:end:194ed07c:start=1559762903914052507,finish=1559762904893530249,duration=979477742
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:07:14]    Compiling synstructure v0.10.2
[00:07:26]    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
[00:07:33]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:07:37]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:07:37] error[E0583]: file not found for module `annotate_rs_emitter`
[00:07:37]    |
[00:07:37]    |
[00:07:37] 36 | pub mod annotate_rs_emitter;
[00:07:37]    |
[00:07:37]    |
[00:07:37]    = help: name the file either annotate_rs_emitter.rs or annotate_rs_emitter/mod.rs inside the directory "src/librustc_errors"
[00:07:37] error: aborting due to previous error
[00:07:37] 
[00:07:37] For more information about this error, try `rustc --explain E0583`.
[00:07:37] error: Could not compile `rustc_errors`.
---
travis_time:end:2e692142:start=1559763385826411687,finish=1559763385830893201,duration=4481514
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:075417a2
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:331eae96
