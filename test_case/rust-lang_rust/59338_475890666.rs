plain
travis_time:end:06887aba:start=1553360478866281299,finish=1553360481170437161,duration=2304155862
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:50:00]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:50:00] error: incorrect close delimiter: `}`
[00:50:00]     --> src/librustc/ty/context.rs:1082:5
[00:50:00]      |
[00:50:00] 1076 |     pub fn dep_graph(self) -> &'gcx DepGraph {
[00:50:00]      |                                              - close delimiter possibly meant for this
[00:50:00] 1077 |         self.dep_graph.get_or_init(|| {
[00:50:00]      |                                   - un-closed delimiter
[00:50:00] 1082 |     }
[00:50:00]      |     ^ incorrect close delimiter
[00:50:00] 
[00:50:43] error: aborting due to previous error
---
travis_time:end:09c1d9c0:start=1553363536960571825,finish=1553363536965501897,duration=4930072
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:04e657d2
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
trav
