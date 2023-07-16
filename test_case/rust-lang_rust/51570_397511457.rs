plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:11e909f6
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/ce/50/6789babdceaae0e7d4c43b66a76052c5f8b8ef2416075f5604c8961adb94/awscli-1.15.39-py2.py3-none-any.whl (1.3MB)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/db/c8/7dcf9dbcb22429512708fe3a547f8b6101c0d02137acbd892505aee57adf/colorama-0.3.9-py2.py3-none-any.whl
Collecting rsa<=3.5.0,>=3.1.2 (from awscli)
---
  Downloading https://files.pythonhosted.org/packages/d7/14/2a0004d487464d120c9fb85313a75cd3d71a7506955be458eebfe19a6b1d/s3transfer-0.1.13-py2.py3-none-any.whl (59kB)
Collecting botocore==1.10.39 (from awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/ac/35/bf811140cc44b45ebe1601b83151e86b57bfed31b06c451a0f87383f9eed/botocore-1.10.39-py2.py3-none-any.whl (4.3MB)
Collecting docutils>=0.10 (from awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/50/09/c53398e0005b11f7ffb27b7aa720c617aba53be4fb4f4f3f06b9b5c60f28/docutils-0.14-py2-none-any.whl (543kB)
---
[00:32:02] warning: ../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingValue.c:109:48: warning: unused parameter 'Value' [-Wunused-parameter]
[00:32:02] warning:                                       uint64_t Value) {
[00:32:02] warning:                                                ^
[00:32:02] warning: 3 warnings generated.
[00:32:30] error[E0494]: cannot refer to the interior of another static, use a constant instead
[00:32:30]      |
[00:32:30]      |
[00:32:30] 3781 |         ASCII_UPPERCASE_MAP[*self as usize]
[00:32:30] 
[00:32:30] 
[00:32:30] error[E0494]: cannot refer to the interior of another static, use a constant instead
[00:32:30]      |
[00:32:30]      |
[00:32:30] 3803 |         ASCII_LOWERCASE_MAP[*self as usize]
[00:32:30] 
[00:32:30] 
[00:32:30] error[E0494]: cannot refer to the interior of another static, use a constant instead
[00:32:30]      |
[00:32:30]      |
[00:32:30] 3908 | /         match ASCII_CHARACTER_CLASS[*self as usize] {
[00:32:30] 3909 | |             L | Lx | U | Ux => true,
[00:32:30] 3910 | |             _ => false
[00:32:30]      | |_________^
[00:32:30] 
[00:32:30] 
[00:32:30] error[E0494]: cannot refer to the interior of another static, use a constant instead
[00:32:30]      |
[00:32:30]      |
[00:32:30] 3909 |             L | Lx | U | Ux => true,
[00:32:30] 
[00:32:30] 
[00:32:30] error[E0494]: cannot refer to the interior of another static, use a constant instead
[00:32:30]      |
[00:32:30]      |
[00:32:30] 3946 | /         match ASCII_CHARACTER_CLASS[*self as usize] {
[00:32:30] 3947 | |             U | Ux => true,
[00:32:30] 3948 | |             _ => false
[00:32:30]      | |_________^
[00:32:30] 
[00:32:30] 
[00:32:30] error[E0494]: cannot refer to the interior of another static, use a constant instead
[00:32:30]      |
[00:32:30]      |
[00:32:30] 3947 |             U | Ux => true,
[00:32:30] 
[00:32:30] 
[00:32:30] error[E0494]: cannot refer to the interior of another static, use a constant instead
[00:32:30]      |
[00:32:30]      |
[00:32:30] 3984 | /         match ASCII_CHARACTER_CLASS[*self as usize] {
[00:32:30] 3985 | |             L | Lx => true,
[00:32:30] 3986 | |             _ => false
[00:32:30]      | |_________^
[00:32:30] 
[00:32:30] 
[00:32:30] error[E0494]: cannot refer to the interior of another static, use a constant instead
[00:32:30]      |
[00:32:30]      |
[00:32:30] 3985 |             L | Lx => true,
[00:32:30] 
[00:32:30] 
[00:32:30] error[E0494]: cannot refer to the interior of another static, use a constant instead
[00:32:30]      |
[00:32:30]      |
[00:32:30] 4025 | /         match ASCII_CHARACTER_CLASS[*self as usize] {
[00:32:30] 4026 | |             D | L | Lx | U | Ux => true,
[00:32:30] 4027 | |             _ => false
[00:32:30]      | |_________^
[00:32:30] 
[00:32:30] 
[00:32:30] error[E0494]: cannot refer to the interior of another static, use a constant instead
[00:32:30]      |
[00:32:30]      |
[00:32:30] 4026 |             D | L | Lx | U | Ux => true,
[00:32:30] 
[00:32:30] 
[00:32:30] error[E0494]: cannot refer to the interior of another static, use a constant instead
[00:32:30]      |
[00:32:30]      |
[00:32:30] 4063 | /         match ASCII_CHARACTER_CLASS[*self as usize] {
[00:32:30] 4064 | |             D => true,
[00:32:30] 4065 | |             _ => false
[00:32:30]      | |_________^
[00:32:30] 
[00:32:30] 
[00:32:30] error[E0494]: cannot refer to the interior of another static, use a constant instead
[00:32:30]      |
[00:32:30] 4064 |             D => true,
[00:32:30]      |             ^
[00:32:30] 
[00:32:30] 
[00:32:30] error[E0494]: cannot refer to the interior of another static, use a constant instead
[00:32:30]      |
[00:32:30]      |
[00:32:30] 4104 | /         match ASCII_CHARACTER_CLASS[*self as usize] {
[00:32:30] 4105 | |             D | Lx | Ux => true,
[00:32:30] 4106 | |             _ => false
[00:32:30]      | |_________^
[00:32:30] 
[00:32:30] 
[00:32:30] error[E0494]: cannot refer to the interior of another static, use a constant instead
[00:32:30]      |
[00:32:30]      |
[00:32:30] 4105 |             D | Lx | Ux => true,
[00:32:30] 
[00:32:30] 
[00:32:30] error[E0494]: cannot refer to the interior of another static, use a constant instead
[00:32:30]      |
[00:32:30]      |
[00:32:30] 4146 | /         match ASCII_CHARACTER_CLASS[*self as usize] {
[00:32:30] 4147 | |             P => true,
[00:32:30] 4148 | |             _ => false
[00:32:30]      | |_________^
[00:32:30] 
[00:32:30] 
[00:32:30] error[E0494]: cannot refer to the interior of another static, use a constant instead
[00:32:30]      |
[00:32:30] 4147 |             P => true,
[00:32:30]      |             ^
[00:32:30] 
[00:32:30] 
[00:32:30] error[E0494]: cannot refer to the interior of another static, use a constant instead
[00:32:30]      |
[00:32:30]      |
[00:32:30] 4184 | /         match ASCII_CHARACTER_CLASS[*self as usize] {
[00:32:30] 4185 | |             Ux | U | Lx | L | D | P => true,
[00:32:30] 4186 | |             _ => false
[00:32:30]      | |_________^
[00:32:30] 
[00:32:30] 
[00:32:30] error[E0494]: cannot refer to the interior of another static, use a constant instead
[00:32:30]      |
[00:32:30]      |
[00:32:30] 4185 |             Ux | U | Lx | L | D | P => true,
[00:32:30] 
[00:32:30] 
[00:32:30] error[E0494]: cannot refer to the interior of another static, use a constant instead
[00:32:30]      |
[00:32:30]      |
[00:32:30] 4239 | /         match ASCII_CHARACTER_CLASS[*self as usize] {
[00:32:30] 4240 | |             Cw | W => true,
[00:32:30] 4241 | |             _ => false
[00:32:30]      | |_________^
[00:32:30] 
[00:32:30] 
[00:32:30] error[E0494]: cannot refer to the interior of another static, use a constant instead
[00:32:30]      |
[00:32:30]      |
[00:32:30] 4240 |             Cw | W => true,
[00:32:30] 
[00:32:30] 
[00:32:30] error[E0494]: cannot refer to the interior of another static, use a constant instead
[00:32:30]      |
[00:32:30]      |
[00:32:30] 4279 | /         match ASCII_CHARACTER_CLASS[*self as usize] {
[00:32:30] 4280 | |             C | Cw => true,
[00:32:30] 4281 | |             _ => false
[00:32:30]      | |_________^
[00:32:30] 
[00:32:30] 
[00:32:30] error[E0494]: cannot refer to the interior of another static, use a constant instead
[00:32:30]      |
[00:32:30]      |
[00:32:30] 4280 |             C | Cw => true,
[00:32:30] 
[00:32:30] 
[00:32:30] error[E0494]: cannot refer to the interior of another static, use a constant instead
[00:32:30]    |
[00:32:30]    |
[00:32:30] 45 |     if n &   7 != 0 { x.mul_small(POW10[n & 7]); }
[00:32:30] 
[00:32:30] 
[00:32:30] error[E0494]: cannot refer to the interior of another static, use a constant instead
[00:32:30]    |
[00:32:30]    |
[00:32:30] 46 |     if n &   8 != 0 { x.mul_small(POW10[8]); }
[00:32:30] 
[00:32:30] 
[00:32:30] error[E0494]: cannot refer to the interior of another static, use a constant instead
[00:32:30]    |
[00:32:30]    |
[00:32:30] 58 |         x.div_rem_small(POW10[largest]);
[00:32:30] 
[00:32:30] 
[00:32:30] error[E0494]: cannot refer to the interior of another static, use a constant instead
[00:32:30]    |
[00:32:30]    |
[00:32:30] 61 |     x.div_rem_small(TWOPOW10[n]);
[00:32:30] 
[00:32:30] 
[00:32:30] error[E0494]: cannot refer to the interior of another static, use a constant instead
[00:32:30]     |
[00:32:30]     |
[00:32:30] 131 |     let (f, e, k) = CACHED_POW10[idx as usize];
[00:32:30] 
[00:32:30] 
[00:32:30] error[E0494]: cannot refer to the interior of another static, use a constant instead
[00:32:30]     |
[00:32:30]     |
[00:32:30] 131 |     let (f, e, k) = CACHED_POW10[idx as usize];
[00:32:30] 
[00:32:30] 
[00:32:30] error[E0494]: cannot refer to the interior of another static, use a constant instead
[00:32:30]     |
[00:32:30]     |
[00:32:30] 131 |     let (f, e, k) = CACHED_POW10[idx as usize];
[00:32:30] 
[00:32:30] 
[00:32:32] error[E0494]: cannot refer to the interior of another static, use a constant instead
[00:32:32]     --> libcore/str/mod.rs:1462:21
[00:32:32]      |
[00:32:32] 1462 |             let w = UTF8_CHAR_WIDTH[first as usize];
[00:32:32] 
[00:32:32] 
[00:32:32] error[E0494]: cannot refer to the interior of another static, use a constant instead
[00:32:32]     --> libcore/str/mod.rs:1573:12
[00:32:32]      |
[00:32:32] 1573 |     return UTF8_CHAR_WIDTH[b as usize] as usize;
[00:32:32] 
[00:32:42] error: aborting due to 31 previous errors
[00:32:42] 
[00:32:42] For more information about this error, try `rustc --explain E0494`.
[00:32:42] For more information about this error, try `rustc --explain E0494`.
[00:32:42] error: Could not compile `core`.
[00:32:42] 
[00:32:42] Caused by:
[00:32:42]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core libcore/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=9cebfd5492943f2f -C extra-filename=-9cebfd5492943f2f --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -C linker=clang -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps` (exit code: 101)
[00:32:42] warning: build failed, waiting for other jobs to finish...
[00:33:07] error: build failed
[00:33:07] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace profiler" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:33:07] expected success, got: exit code: 101
[00:33:07] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:33:07] travis_fold:end:stage1-std

[00:33:07] travis_time:end:stage1-std:start=1529037394722906629,finish=1529037480114169387,duration=85391262758

---
travis_time:end:0cd05e0e:start=1529037480955071483,finish=1529037480962571626,duration=7500143
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:24300b46
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
global:
global:
  _ZdaPv;
  _ZdaPvRKSt9nothrow_t;
  _ZdaPvSt11align_val_t;
  _ZdaPvSt11align_val_tRKSt9nothrow_t;
  _ZdaPvj;
  _ZdaPvjSt11align_val_t;
  _ZdlPv;
  _ZdlPvRKSt9nothrow_t;
  _ZdlPvSt11align_val_t;
  _ZdlPvSt11align_val_tRKSt9nothrow_t;
  _ZdlPvj;
  _ZdlPvjSt11align_val_t;
  _Znaj;
  _ZnajRKSt9nothrow_t;
  _ZnajSt11align_val_t;
  _ZnajSt11align_val_tRKSt9nothrow_t;
  _Znwj;
  _ZnwjRKSt9nothrow_t;
  _ZnwjSt11align_val_t;
  _ZnwjSt11align_val_tRKSt9nothrow_t;
  __asan_*;
  __cxa_atexit;
  __cxa_throw;
  __fprintf_chk;
  __getdelim;
  __interceptor___cxa_atexit;
  __interceptor___cxa_throw;
  __interceptor___fprintf_chk;
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:12b69e78
$ dmesg | grep -i kill
