plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/f3/04/ef69353b3bdbc6ddb9de24291039dc06c33feb0e51ed73f5f3be707ae035/awscli-1.15.47-py2.py3-none-any.whl (1.3MB)
    0% |▎                               | 10kB 12.6MB/s eta 0:00:01
    1% |▌                               | 20kB 1.9MB/s eta 0:00:01
    2% |▊                               | 30kB 2.1MB/s eta 0:00:01
    3% |█                               | 40kB 2.0MB/s eta 0:00:01
---
[00:03:37]    Compiling std_unicode v0.0.0 (file:///checkout/src/libstd_unicode)
[00:03:39]    Compiling alloc_system v0.0.0 (file:///checkout/src/liballoc_system)
[00:03:39]    Compiling panic_abort v0.0.0 (file:///checkout/src/libpanic_abort)
[00:03:43]    Compiling panic_unwind v0.0.0 (file:///checkout/src/libpanic_unwind)
[00:03:48] warning: function is never used: `hashmap_random_keys`
[00:03:48]   --> libstd/sys/unix/rand.rs:14:1
[00:03:48]    |
[00:03:48] 14 | pub fn hashmap_random_keys() -> (u64, u64) {
[00:03:48]    |
[00:03:48]    = note: #[warn(dead_code)] on by default
[00:03:48] 
[00:03:48] warning: function is never used: `getrandom`
[00:03:48] warning: function is never used: `getrandom`
[00:03:48]   --> libstd/sys/unix/rand.rs:36:5
[00:03:48]    |
[00:03:48] 36 |     fn getrandom(buf: &mut [u8]) -> libc::c_long {
[00:03:48] 
[00:03:48] warning: function is never used: `getrandom_fill_bytes`
[00:03:48]   --> libstd/sys/unix/rand.rs:45:5
[00:03:48]    |
[00:03:48]    |
[00:03:48] 45 |     fn getrandom_fill_bytes(v: &mut [u8]) -> bool {
[00:03:48] 
[00:03:48] 
[00:03:48] warning: function is never used: `is_getrandom_available`
[00:03:48]   --> libstd/sys/unix/rand.rs:67:5
[00:03:48]    |
[00:03:48] 67 |     fn is_getrandom_available() -> bool {
[00:03:48] 
[00:03:48] warning: function is never used: `fill_bytes`
[00:03:48]   --> libstd/sys/unix/rand.rs:93:5
[00:03:48]    |
[00:03:48]    |
[00:03:48] 93 |     pub fn fill_bytes(v: &mut [u8]) {
[00:03:48] 
[00:03:56]     Finished release [optimized] target(s) in 45.03s
[00:03:56] Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[00:03:56] travis_fold:end:stage0-std
---
[01:10:15] travis_fold:start:test_stage1-std
travis_time:start:test_stage1-std
Testing std stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:10:15]    Compiling std v0.0.0 (file:///checkout/src/libstd)
[01:10:32] error: function is never used: `hashmap_random_keys`
[01:10:32]   --> libstd/sys/unix/rand.rs:14:1
[01:10:32]    |
[01:10:32] 14 | pub fn hashmap_random_keys() -> (u64, u64) {
[01:10:32]    |
[01:10:32]    = note: `-D dead-code` implied by `-D warnings`
[01:10:32] 
[01:10:32] 
[01:10:32] error: function is never used: `getrandom`
[01:10:32]   --> libstd/sys/unix/rand.rs:36:5
[01:10:32]    |
[01:10:32] 36 |     fn getrandom(buf: &mut [u8]) -> libc::c_long {
[01:10:32] 
[01:10:32] 
[01:10:32] error: function is never used: `getrandom_fill_bytes`
[01:10:32]   --> libstd/sys/unix/rand.rs:45:5
[01:10:32]    |
[01:10:32] 45 |     fn getrandom_fill_bytes(v: &mut [u8]) -> bool {
[01:10:32] 
[01:10:32] 
[01:10:32] error: function is never used: `is_getrandom_available`
[01:10:32]   --> libstd/sys/unix/rand.rs:67:5
[01:10:32]    |
[01:10:32] 67 |     fn is_getrandom_available() -> bool {
[01:10:32] 
[01:10:32] 
[01:10:32] error: function is never used: `fill_bytes`
[01:10:32]   --> libstd/sys/unix/rand.rs:93:5
[01:10:32]    |
[01:10:32] 93 |     pub fn fill_bytes(v: &mut [u8]) {
[01:10:32] 
[01:10:33] error: aborting due to 5 previous errors
[01:10:33] 
[01:10:33] error: Could not compile `std`.
[01:10:33] error: Could not compile `std`.
[01:10:33] 
[01:10:33] To learn more, run the command again with --verbose.
[01:10:33] 
[01:10:33] 
[01:10:33] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "std" "--" "--quiet"
[01:10:33] 
[01:10:33] 
[01:10:33] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:10:33] Build completed unsuccessfully in 0:30:26
[01:10:33] Build completed unsuccessfully in 0:30:26
[01:10:33] Makefile:58: recipe for target 'check' failed
[01:10:33] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:224619ab
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
