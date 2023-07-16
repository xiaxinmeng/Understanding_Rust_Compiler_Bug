plain
travis_time:end:193de5cd:start=1545097608276238659,finish=1545097609337887757,duration=1061649098
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/63/dc/c8bfd1bd77113c033161ce31730510d1c479cf9bcc8e99edf3c906f30cce/awscli-1.16.77-py2.py3-none-any.whl (1.4MB)
    0% |▎                               | 10kB 29.4MB/s eta 0:00:01
    1% |▌                               | 20kB 2.1MB/s eta 0:00:01
    2% |▊                               | 30kB 3.0MB/s eta 0:00:01
    2% |█                               | 40kB 2.0MB/s eta 0:00:01
---
[00:02:54]    Compiling libc v0.2.45
[00:02:54]    Compiling core v0.0.0 (/checkout/src/libcore)
[00:02:54]    Compiling unwind v0.0.0 (/checkout/src/libunwind)
[00:02:55]    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
[00:02:58] error[E0432]: unresolved import `self::adapters::flatten_compat`
[00:02:58]     |
[00:02:58]     |
[00:02:58] 362 | use self::adapters::{flatten_compat, ZipImpl};
[00:02:58]     |                      ^^^^^^^^^^^^^^ no `flatten_compat` in `iter::adapters`
[00:03:00] warning: unused import: `flatten_compat`
[00:03:00]    --> src/libcore/iter/mod.rs:362:22
[00:03:00]     |
[00:03:00]     |
[00:03:00] 362 | use self::adapters::{flatten_compat, ZipImpl};
[00:03:00]     |
[00:03:00]     = note: #[warn(unused_imports)] on by default
[00:03:00] 
[00:03:00]    Compiling compiler_builtins v0.1.2
---
[00:03:04]    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
[00:03:09] error[E0308]: mismatched types
[00:03:09]     --> src/libcore/iter/traits/iterator.rs:1111:26
[00:03:09]      |
[00:03:09] 1111 |         FlatMap { inner: FlatMap::new(self, f) }
[00:03:09]      |                          ^^^^^^^^^^^^^^^^^^^^^ expected struct `iter::adapters::flatten::FlattenCompat`, found struct `iter::adapters::flatten::FlatMap`
[00:03:09]      |
[00:03:09]      = note: expected type `iter::adapters::flatten::FlattenCompat<iter::adapters::Map<Self, F>, <U as iter::traits::collect::IntoIterator>::IntoIter>`
[00:03:09]                 found type `iter::adapters::flatten::FlatMap<Self, U, F>`
[00:03:09] error[E0308]: mismatched types
[00:03:09]     --> src/libcore/iter/traits/iterator.rs:1179:26
[00:03:09]      |
[00:03:09]      |
[00:03:09] 1179 |         Flatten { inner: Flatten::new(self) }
[00:03:09]      |                          ^^^^^^^^^^^^^^^^^^ expected struct `iter::adapters::flatten::FlattenCompat`, found struct `iter::adapters::flatten::Flatten`
[00:03:09]      |
[00:03:09]      = note: expected type `iter::adapters::flatten::FlattenCompat<Self, <<Self as iter::traits::iterator::Iterator>::Item as iter::traits::collect::IntoIterator>::IntoIter>`
[00:03:09]                 found type `iter::adapters::flatten::Flatten<Self>`
[00:03:09] error[E0308]: mismatched types
[00:03:09]   --> src/libcore/iter/adapters/flatten.rs:31:9
[00:03:09]    |
[00:03:09]    |
[00:03:09] 30 |     pub(in super::super) fn new(iter: I, f: F) -> FlatMap<I, U, F> {
[00:03:09]    |                                                   ---------------- expected `iter::adapters::flatten::FlatMap<I, U, F>` because of return type
[00:03:09] 31 |         FlattenCompat::new(iter.map(f))
[00:03:09]    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `iter::adapters::flatten::FlatMap`, found struct `iter::adapters::flatten::FlattenCompat`
[00:03:09]    |
[00:03:09]    = note: expected type `iter::adapters::flatten::FlatMap<I, U, F>`
[00:03:09]               found type `iter::adapters::flatten::FlattenCompat<iter::adapters::Map<I, F>, _>`
[00:03:09] error[E0308]: mismatched types
[00:03:09]    --> src/libcore/iter/adapters/flatten.rs:123:9
[00:03:09]     |
[00:03:09]     |
[00:03:09] 122 |     pub(in super::super) fn new(iter: I) -> Flatten<I> {
[00:03:09]     |                                             ---------- expected `iter::adapters::flatten::Flatten<I>` because of return type
[00:03:09] 123 |         FlattenCompat::new(iter)
[00:03:09]     |         ^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `iter::adapters::flatten::Flatten`, found struct `iter::adapters::flatten::FlattenCompat`
[00:03:09]     |
[00:03:09]     = note: expected type `iter::adapters::flatten::Flatten<I>`
[00:03:09]                found type `iter::adapters::flatten::FlattenCompat<I, _>`
1% /dev
tmpfs           1.5G  284K  1.5G   1% /run
/dev/sda1        69G  8.7G   58G  14% /
none            4.0K     0  4.0K   0% /sys/fs/cgroup
---
travis_time:end:29ab7a44:start=1545097810351727494,finish=1545097810357446543,duration=5719049
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:3a7c0540
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:142443b4
travis_time:start:142443b4
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:00608400
$ dmesg | grep -i kill
