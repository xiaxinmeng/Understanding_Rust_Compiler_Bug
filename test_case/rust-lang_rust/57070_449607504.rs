plain
travis_time:end:033dc400:start=1545527083925505633,finish=1545527138241635218,duration=54316129585
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:05:05]    Compiling arena v0.0.0 (/checkout/src/libarena)
[00:05:05]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:05:09]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:06:22]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:06:25] error[E0425]: cannot find value `fingerprints` in this scope
[00:06:25]    --> src/librustc/dep_graph/graph.rs:512:44
[00:06:25]     |
[00:06:25] 512 |         if current_dep_graph.nodes.len() > fingerprints.len() {
[00:06:25]     |                                            ^^^^^^^^^^^^ help: try: `self.fingerprints`
[00:06:25] error[E0425]: cannot find value `fingerprints` in this scope
[00:06:25] error[E0425]: cannot find value `fingerprints` in this scope
[00:06:25]    --> src/librustc/dep_graph/graph.rs:513:13
[00:06:25]     |
[00:06:25] 513 |             fingerprints.resize(current_dep_graph.nodes.len(), Fingerprint::ZERO);
[00:06:25]     |             ^^^^^^^^^^^^ help: try: `self.fingerprints`
[00:06:25] error[E0425]: cannot find value `fingerprints` in this scope
[00:06:25] error[E0425]: cannot find value `fingerprints` in this scope
[00:06:25]    --> src/librustc/dep_graph/graph.rs:518:24
[00:06:25]     |
[00:06:25] 518 |             (dep_node, fingerprints[idx])
[00:06:25]     |                        ^^^^^^^^^^^^ help: try: `self.fingerprints`
| sort -nr | head -n100
