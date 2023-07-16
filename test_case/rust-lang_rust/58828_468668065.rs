plain
travis_time:end:035aa4fa:start=1551441384323227750,finish=1551441386581982783,duration=2258755033
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=mingw-check
---
[00:06:51] configure: build.locked-deps    := True
[00:06:51] configure: llvm.ccache          := sccache
[00:06:51] configure: build.cargo-native-static := True
[00:06:51] configure: dist.missing-tools   := True
[00:06:51] configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
[00:06:51] configure: writing `config.toml` in current directory
[00:06:51] configure: 
[00:06:51] configure: run `python /checkout/x.py --help`
[00:06:51] configure: 
---
[00:08:38]     Checking panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:08:40] error: hidden lifetime parameters in types are deprecated
[00:08:40]    --> src/libstd/sys/windows/args.rs:173:34
[00:08:40]     |
[00:08:40] 173 |     pub fn inner_debug(&self) -> ArgsInnerDebug {
[00:08:40] 
[00:08:40] error: hidden lifetime parameters in types are deprecated
[00:08:40]    --> src/libstd/sys/windows/os.rs:139:41
[00:08:40]     |
[00:08:40]     |
[00:08:40] 139 | pub fn split_paths(unparsed: &OsStr) -> SplitPaths {
[00:08:40] 
[00:08:40] error: hidden lifetime parameters in types are deprecated
[00:08:40]   --> src/libstd/sys/windows/path.rs:22:52
[00:08:40]    |
[00:08:40]    |
[00:08:40] 22 | pub fn parse_prefix<'a>(path: &'a OsStr) -> Option<Prefix> {
[00:08:40] 
[00:08:42] error: aborting due to 3 previous errors
[00:08:42] 
[00:08:42] error: Could not compile `std`.
---
travis_time:end:02c03a8a:start=1551441921902744268,finish=1551441921909804712,duration=7060444
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:02e69e04
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2f2da88c
travis_time:start:2f2da88c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1ca34e60
$ dmesg | grep -i kill
