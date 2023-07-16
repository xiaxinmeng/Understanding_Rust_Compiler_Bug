plain
travis_time:end:1000b6d2:start=1553769190612465621,finish=1553769193490534518,duration=2878068897
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
####################################                                      51.2%
######################################################################## 100.0%
[00:01:57] extracting /checkout/obj/build/cache/2019-03-20/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:57]     Updating crates.io index
[00:02:11]     Updating git repository `https://github.com/gnzlbg/libtest`
[00:02:13]   Downloaded petgraph v0.4.13
[00:02:13]   Downloaded toml v0.4.10
[00:02:13]   Downloaded libc v0.2.50
[00:02:13]   Downloaded getopts v0.2.17
---
tidy check
[00:04:03] * 569 error codes
[00:04:03] * highest error code: E0725
[00:04:03] * 252 features
[00:04:04] invalid source: "git+https://github.com/gnzlbg/libtest?branch=clippy_ci#2cb20561a6162879801a08a606cec9c72dbc3f10"

[00:04:04] travis_time:end:tidy:start=1553769445712601321,finish=1553769447491328413,duration=1778727092

[00:04:04] Build completed successfully in 0:00:46
---
[00:05:01]    Compiling libc v0.2.50
[00:05:01]    Compiling getopts v0.2.17
[00:05:01]    Compiling termcolor v1.0.4
[00:05:01]    Compiling proc_macro v0.0.0 (/checkout/src/libproc_macro)
[00:05:03]    Compiling libtest v0.0.2 (https://github.com/gnzlbg/libtest?branch=clippy_ci#2cb20561)
[00:05:13]     Finished release [optimized] target(s) in 12.33s
[00:05:13] Copying stage0 test from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[00:05:13] travis_fold:end:stage0-test

---
[00:28:16]    Compiling libc v0.2.50
[00:28:16]    Compiling getopts v0.2.17
[00:28:16]    Compiling termcolor v1.0.4
[00:28:16]    Compiling proc_macro v0.0.0 (/checkout/src/libproc_macro)
[00:28:18]    Compiling libtest v0.0.2 (https://github.com/gnzlbg/libtest?branch=clippy_ci#2cb20561)
[00:28:30]     Finished release [optimized] target(s) in 14.64s
[00:28:30] Copying stage1 test from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[00:28:30] travis_fold:end:stage1-test

---
[00:55:16]    Compiling parking_lot_core v0.4.0
[00:55:20]    Compiling tempfile v3.0.5
[00:55:22]    Compiling parking_lot v0.7.1
[00:55:23]    Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
[00:55:32] error[E0507]: cannot move out of captured variable in an `FnMut` closure
[00:55:32]     |
[00:55:32]     |
[00:55:32] 676 |         let cfgs = self.cfgs.clone();
[00:55:32]     |             ---- captured outer variable
[00:55:32] 702 |                     cfgs,
[00:55:32] 702 |                     cfgs,
[00:55:32]     |                     ^^^^ cannot move out of captured variable in an `FnMut` closure
[00:55:32] 
[00:55:32] error[E0507]: cannot move out of captured variable in an `FnMut` closure
[00:55:32]     |
[00:55:32]     |
[00:55:32] 677 |         let libs = self.libs.clone();
[00:55:32]     |             ---- captured outer variable
[00:55:32] 703 |                     libs,
[00:55:32] 703 |                     libs,
[00:55:32]     |                     ^^^^ cannot move out of captured variable in an `FnMut` closure
[00:55:32] 
[00:55:32] error[E0507]: cannot move out of captured variable in an `FnMut` closure
[00:55:32]     |
[00:55:32]     |
[00:55:32] 678 |         let cg = self.cg.clone();
[00:55:32]     |             -- captured outer variable
[00:55:32] 704 |                     cg,
[00:55:32] 704 |                     cg,
[00:55:32]     |                     ^^ cannot move out of captured variable in an `FnMut` closure
[00:55:32] 
[00:55:32] error[E0507]: cannot move out of captured variable in an `FnMut` closure
[00:55:32]     |
[00:55:32]     |
[00:55:32] 679 |         let externs = self.externs.clone();
[00:55:32]     |             ------- captured outer variable
[00:55:32] 705 |                     externs,
[00:55:32] 705 |                     externs,
[00:55:32]     |                     ^^^^^^^ cannot move out of captured variable in an `FnMut` closure
[00:55:32] 
[00:55:32] error[E0507]: cannot move out of captured variable in an `FnMut` closure
[00:55:32]     |
[00:55:32]     |
[00:55:32] 673 |     fn add_test(&mut self, test: String, config: LangString, line: usize) {
[00:55:32]     |                                          ------ captured outer variable
[00:55:32] 710 |                     config.error_codes,
[00:55:32] 710 |                     config.error_codes,
[00:55:32]     |                     ^^^^^^^^^^^^^^^^^^ cannot move out of captured variable in an `FnMut` closure
[00:55:32] 
[00:55:32] error[E0507]: cannot move out of captured variable in an `FnMut` closure
[00:55:32]     |
[00:55:32]     |
[00:55:32] 682 |         let maybe_sysroot = self.maybe_sysroot.clone();
[00:55:32]     |             ------------- captured outer variable
[00:55:32] 712 |                     maybe_sysroot,
[00:55:32] 712 |                     maybe_sysroot,
[00:55:32]     |                     ^^^^^^^^^^^^^ cannot move out of captured variable in an `FnMut` closure
[00:55:32] 
[00:55:32] error[E0507]: cannot move out of captured variable in an `FnMut` closure
[00:55:32]     |
[00:55:32] 683 |         let linker = self.linker.clone();
[00:55:32]     |             ------ captured outer variable
[00:55:32] ...
[00:55:32] ...
[00:55:32] 713 |                     linker,
[00:55:32]     |                     ^^^^^^ cannot move out of captured variable in an `FnMut` closure
[00:55:32] 
[00:55:32] error[E0507]: cannot move out of captured variable in an `FnMut` closure
[00:55:32]     |
[00:55:32]     |
[00:55:32] 685 |         let persist_doctests = self.persist_doctests.clone();
[00:55:32]     |             ---------------- captured outer variable
[00:55:32] 715 |                     persist_doctests
[00:55:32] 715 |                     persist_doctests
[00:55:32]     |                     ^^^^^^^^^^^^^^^^ cannot move out of captured variable in an `FnMut` closure
[00:55:33] error: aborting due to 8 previous errors
[00:55:33] 
[00:55:33] For more information about this error, try `rustc --explain E0507`.
[00:55:33] error: Could not compile `rustdoc`.
---
travis_time:end:0092968b:start=1553772537824788517,finish=1553772537831294751,duration=6506234
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:260ac35d
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:040e6128
travis_time:start:040e6128
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0f74f910
$ dmesg | grep -i kill
