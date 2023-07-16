plain
travis_time:end:1b8ced6a:start=1553829335198258045,finish=1553829408523302533,duration=73325044488
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:13:33]     |
[00:13:33] 274 |         let parent_id = self.tcx.hir().get_parent_node_by_hir_id(expr.hir_id);
[00:13:33]     |                                                                  ^^^^ not found in this scope
[00:13:33] 
[00:13:33] error[E0425]: cannot find value `cm` in this scope
[00:13:33]    --> src/librustc_typeck/check/demand.rs:281:34
[00:13:33]     |
[00:13:33] 281 |                 if let Ok(src) = cm.span_to_snippet(sp) {
[00:13:33]     |                                  ^^ not found in this scope
[00:13:33] error[E0425]: cannot find value `sp` in this scope
[00:13:33]    --> src/librustc_typeck/check/demand.rs:281:53
[00:13:33]     |
[00:13:33]     |
[00:13:33] 281 |                 if let Ok(src) = cm.span_to_snippet(sp) {
[00:13:33]     |                                                     ^^ not found in this scope
[00:13:37] error[E0308]: mismatched types
[00:13:37]    --> src/librustc_typeck/check/demand.rs:273:84
[00:13:37]     |
[00:13:37]     |
[00:13:37] 273 |     fn is_hir_id_from_struct_pattern_shorthand_field(&self, hir_id: hir::HirId) -> bool {
[00:13:37]     |        ---------------------------------------------                               ^^^^ expected bool, found ()
[00:13:37]     |        this function's body doesn't return
[00:13:37] ...
[00:13:37] 290 |         };
[00:13:37]     |          - help: consider removing this semicolon
---
travis_time:end:2f12fdca:start=1553830494951591891,finish=1553830494957714886,duration=6122995
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0473d95e
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:18c78f00
travis_time:start:18c78f00
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1c8437e4
$ dmesg | grep -i kill
