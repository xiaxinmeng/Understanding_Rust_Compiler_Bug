plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/ce/50/6789babdceaae0e7d4c43b66a76052c5f8b8ef2416075f5604c8961adb94/awscli-1.15.39-py2.py3-none-any.whl (1.3MB)
    0% |▎                               | 10kB 21.0MB/s eta 0:00:01
    1% |▌                               | 20kB 2.0MB/s eta 0:00:01
    2% |▉                               | 30kB 2.3MB/s eta 0:00:01
    3% |█                               | 40kB 2.0MB/s eta 0:00:01
---
    100% |████████████████████████████████| 61kB 7.0MB/s 
Collecting botocore==1.10.39 (from awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/ac/35/bf811140cc44b45ebe1601b83151e86b57bfed31b06c451a0f87383f9eed/botocore-1.10.39-py2.py3-none-any.whl (4.3MB)
    0% |                                | 10kB 29.3MB/s eta 0:00:01
    0% |▏                               | 20kB 27.5MB/s eta 0:00:01
    0% |▎                               | 30kB 31.2MB/s eta 0:00:01
    0% |▎                               | 40kB 23.3MB/s eta 0:00:01
---
[00:20:54]    Compiling rustc_lsan v0.0.0 (file:///checkout/src/librustc_lsan)
[00:20:56]    Compiling rustc_tsan v0.0.0 (file:///checkout/src/librustc_tsan)
[00:20:56]    Compiling rustc_asan v0.0.0 (file:///checkout/src/librustc_asan)
[00:20:57]    Compiling rustc_msan v0.0.0 (file:///checkout/src/librustc_msan)
[00:21:22] error[E0494]: cannot refer to the interior of another static, use a constant instead
[00:21:22]      |
[00:21:22]      |
[00:21:22] 3781 |         ASCII_UPPERCASE_MAP[*self as usize]
[00:21:22] 
[00:21:22] 
[00:21:22] error[E0494]: cannot refer to the interior of another static, use a constant instead
[00:21:22]      |
[00:21:22]      |
[00:21:22] 3803 |         ASCII_LOWERCASE_MAP[*self as usize]
[00:21:22] 
[00:21:22] 
[00:21:22] error[E0494]: cannot refer to the interior of another static, use a constant instead
[00:21:22]      |
[00:21:22]      |
[00:21:22] 3908 | /         match ASCII_CHARACTER_CLASS[*self as usize] {
[00:21:22] 3909 | |             L | Lx | U | Ux => true,
[00:21:22] 3910 | |             _ => false
[00:21:22]      | |_________^
[00:21:22] 
[00:21:22] 
[00:21:22] error[E0494]: cannot refer to the interior of another static, use a constant instead
[00:21:22]      |
[00:21:22]      |
[00:21:22] 3909 |             L | Lx | U | Ux => true,
[00:21:22] 
[00:21:22] 
[00:21:22] error[E0494]: cannot refer to the interior of another static, use a constant instead
[00:21:22]      |
[00:21:22]      |
[00:21:22] 3946 | /         match ASCII_CHARACTER_CLASS[*self as usize] {
[00:21:22] 3947 | |             U | Ux => true,
[00:21:22] 3948 | |             _ => false
[00:21:22]      | |_________^
[00:21:22] 
[00:21:22] 
[00:21:22] error[E0494]: cannot refer to the interior of another static, use a constant instead
[00:21:22]      |
[00:21:22]      |
[00:21:22] 3947 |             U | Ux => true,
[00:21:22] 
[00:21:22] 
[00:21:22] error[E0494]: cannot refer to the interior of another static, use a constant instead
[00:21:22]      |
[00:21:22]      |
[00:21:22] 3984 | /         match ASCII_CHARACTER_CLASS[*self as usize] {
[00:21:22] 3985 | |             L | Lx => true,
[00:21:22] 3986 | |             _ => false
[00:21:22]      | |_________^
[00:21:22] 
[00:21:22] 
[00:21:22] error[E0494]: cannot refer to the interior of another static, use a constant instead
[00:21:22]      |
[00:21:22]      |
[00:21:22] 3985 |             L | Lx => true,
[00:21:22] 
[00:21:22] 
[00:21:22] error[E0494]: cannot refer to the interior of another static, use a constant instead
[00:21:22]      |
[00:21:22]      |
[00:21:22] 4025 | /         match ASCII_CHARACTER_CLASS[*self as usize] {
[00:21:22] 4026 | |             D | L | Lx | U | Ux => true,
[00:21:22] 4027 | |             _ => false
[00:21:22]      | |_________^
[00:21:22] 
[00:21:22] 
[00:21:22] error[E0494]: cannot refer to the interior of another static, use a constant instead
[00:21:22]      |
[00:21:22]      |
[00:21:22] 4026 |             D | L | Lx | U | Ux => true,
[00:21:22] 
[00:21:22] 
[00:21:22] error[E0494]: cannot refer to the interior of another static, use a constant instead
[00:21:22]      |
[00:21:22]      |
[00:21:22] 4063 | /         match ASCII_CHARACTER_CLASS[*self as usize] {
[00:21:22] 4064 | |             D => true,
[00:21:22] 4065 | |             _ => false
[00:21:22]      | |_________^
[00:21:22] 
[00:21:22] 
[00:21:22] error[E0494]: cannot refer to the interior of another static, use a constant instead
[00:21:22]      |
[00:21:22] 4064 |             D => true,
[00:21:22]      |             ^
[00:21:22] 
[00:21:22] 
[00:21:22] error[E0494]: cannot refer to the interior of another static, use a constant instead
[00:21:22]      |
[00:21:22]      |
[00:21:22] 4104 | /         match ASCII_CHARACTER_CLASS[*self as usize] {
[00:21:22] 4105 | |             D | Lx | Ux => true,
[00:21:22] 4106 | |             _ => false
[00:21:22]      | |_________^
[00:21:22] 
[00:21:22] 
[00:21:22] error[E0494]: cannot refer to the interior of another static, use a constant instead
[00:21:22]      |
[00:21:22]      |
[00:21:22] 4105 |             D | Lx | Ux => true,
[00:21:22] 
[00:21:22] 
[00:21:22] error[E0494]: cannot refer to the interior of another static, use a constant instead
[00:21:22]      |
[00:21:22]      |
[00:21:22] 4146 | /         match ASCII_CHARACTER_CLASS[*self as usize] {
[00:21:22] 4147 | |             P => true,
[00:21:22] 4148 | |             _ => false
[00:21:22]      | |_________^
[00:21:22] 
[00:21:22] 
[00:21:22] error[E0494]: cannot refer to the interior of another static, use a constant instead
[00:21:22]      |
[00:21:22] 4147 |             P => true,
[00:21:22]      |             ^
[00:21:22] 
[00:21:22] 
[00:21:22] error[E0494]: cannot refer to the interior of another static, use a constant instead
[00:21:22]      |
[00:21:22]      |
[00:21:22] 4184 | /         match ASCII_CHARACTER_CLASS[*self as usize] {
[00:21:22] 4185 | |             Ux | U | Lx | L | D | P => true,
[00:21:22] 4186 | |             _ => false
[00:21:22]      | |_________^
[00:21:22] 
[00:21:22] 
[00:21:22] error[E0494]: cannot refer to the interior of another static, use a constant instead
[00:21:22]      |
[00:21:22]      |
[00:21:22] 4185 |             Ux | U | Lx | L | D | P => true,
[00:21:22] 
[00:21:22] 
[00:21:22] error[E0494]: cannot refer to the interior of another static, use a constant instead
[00:21:22]      |
[00:21:22]      |
[00:21:22] 4239 | /         match ASCII_CHARACTER_CLASS[*self as usize] {
[00:21:22] 4240 | |             Cw | W => true,
[00:21:22] 4241 | |             _ => false
[00:21:22]      | |_________^
[00:21:22] 
[00:21:22] 
[00:21:22] error[E0494]: cannot refer to the interior of another static, use a constant instead
[00:21:22]      |
[00:21:22]      |
[00:21:22] 4240 |             Cw | W => true,
[00:21:22] 
[00:21:22] 
[00:21:22] error[E0494]: cannot refer to the interior of another static, use a constant instead
[00:21:22]      |
[00:21:22]      |
[00:21:22] 4279 | /         match ASCII_CHARACTER_CLASS[*self as usize] {
[00:21:22] 4280 | |             C | Cw => true,
[00:21:22] 4281 | |             _ => false
[00:21:22]      | |_________^
[00:21:22] 
[00:21:22] 
[00:21:22] error[E0494]: cannot refer to the interior of another static, use a constant instead
[00:21:22]      |
[00:21:22]      |
[00:21:22] 4280 |             C | Cw => true,
[00:21:22] 
[00:21:22] 
[00:21:22] error[E0494]: cannot refer to the interior of another static, use a constant instead
[00:21:22]    |
[00:21:22]    |
[00:21:22] 45 |     if n &   7 != 0 { x.mul_small(POW10[n & 7]); }
[00:21:22] 
[00:21:22] 
[00:21:22] error[E0494]: cannot refer to the interior of another static, use a constant instead
[00:21:22]    |
[00:21:22]    |
[00:21:22] 46 |     if n &   8 != 0 { x.mul_small(POW10[8]); }
[00:21:22] 
[00:21:22] 
[00:21:22] error[E0494]: cannot refer to the interior of another static, use a constant instead
[00:21:22]    |
[00:21:22]    |
[00:21:22] 58 |         x.div_rem_small(POW10[largest]);
[00:21:22] 
[00:21:22] 
[00:21:22] error[E0494]: cannot refer to the interior of another static, use a constant instead
[00:21:22]    |
[00:21:22]    |
[00:21:22] 61 |     x.div_rem_small(TWOPOW10[n]);
[00:21:22] 
[00:21:22] 
[00:21:22] error[E0494]: cannot refer to the interior of another static, use a constant instead
[00:21:22]     |
[00:21:22]     |
[00:21:22] 131 |     let (f, e, k) = CACHED_POW10[idx as usize];
[00:21:22] 
[00:21:22] 
[00:21:22] error[E0494]: cannot refer to the interior of another static, use a constant instead
[00:21:22]     |
[00:21:22]     |
[00:21:22] 131 |     let (f, e, k) = CACHED_POW10[idx as usize];
[00:21:22] 
[00:21:22] 
[00:21:22] error[E0494]: cannot refer to the interior of another static, use a constant instead
[00:21:22]     |
[00:21:22]     |
[00:21:22] 131 |     let (f, e, k) = CACHED_POW10[idx as usize];
[00:21:22] 
[00:21:22] 
[00:21:23] error[E0494]: cannot refer to the interior of another static, use a constant instead
[00:21:23]     --> libcore/str/mod.rs:1462:21
[00:21:23]      |
[00:21:23] 1462 |             let w = UTF8_CHAR_WIDTH[first as usize];
[00:21:23] 
[00:21:23] 
[00:21:23] error[E0494]: cannot refer to the interior of another static, use a constant instead
[00:21:23]     --> libcore/str/mod.rs:1573:12
[00:21:23]      |
[00:21:23] 1573 |     return UTF8_CHAR_WIDTH[b as usize] as usize;
[00:21:23] 
obj/build/bootstrap/debug/bootstrap build
[00:21:28] Build completed unsuccessfully in 0:16:43
[00:21:28] Build completed unsuccessfully in 0:16:43
[00:21:28] make: *** [all] Error 1
[00:21:28] Makefile:28: recipe for target 'all' failed
