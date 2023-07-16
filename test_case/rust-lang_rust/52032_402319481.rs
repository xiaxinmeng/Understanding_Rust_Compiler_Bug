plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/b5/dd/84d32d2275ea16daf09d561858dd0e615c56c9e8afb2e9b42d02bc45e417/awscli-1.15.51-py2.py3-none-any.whl (1.3MB)
    0% |▎                               | 10kB 11.2MB/s eta 0:00:01
    1% |▌                               | 20kB 1.8MB/s eta 0:00:01
    2% |▊                               | 30kB 2.0MB/s eta 0:00:01
    3% |█                               | 40kB 1.9MB/s eta 0:00:01
---
    100% |████████████████████████████████| 552kB 2.0MB/s 
Collecting botocore==1.10.50 (from awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/d5/9f/2e701a365b5ff0e8b664d6c393f3c61c20e52bb5148bbc2e27d737b890db/botocore-1.10.50-py2.py3-none-any.whl (4.4MB)
    0% |                                | 10kB 45.9MB/s eta 0:00:01
    0% |▏                               | 20kB 48.9MB/s eta 0:00:01
    0% |▎                               | 30kB 56.2MB/s eta 0:00:01
    0% |▎                               | 40kB 34.7MB/s eta 0:00:01
---
[00:39:55] ................................................................................................i...
[00:40:01] ............................................................i.......................................
[00:40:06] ....................................................................................................
[00:40:09] ....................................................................................................
[00:40:12] .....................................................................................F..............
[00:40:21] ....................................................................................................
[00:40:25] ....................................................................................................
[00:40:32] ....................................................................................................
[00:40:36] ....................................................................................................
---
[00:41:08] 
[00:41:08] ---- [ui] ui/feature-gate-abi.rs stdout ----
[00:41:08] diff of stderr:
[00:41:08] 
[00:41:08] 1 error[E0658]: intrinsics are subject to change
[00:41:08] -   --> $DIR/feature-gate-abi.rs:19:1
[00:41:08] +   --> $DIR/feature-gate-abi.rs:20:1
[00:41:08] 3    |
[00:41:08] 4 LL | extern "rust-intrinsic" fn f1() {} //~ ERROR intrinsics are subject to change
[00:41:08] 
[00:41:08] 
[00:41:08] 7    = help: add #![feature(intrinsics)] to the crate attributes to enable
[00:41:08] 8 
[00:41:08] 9 error[E0658]: platform intrinsics are experimental and possibly buggy (see issue #27731)
[00:41:08] -   --> $DIR/feature-gate-abi.rs:20:1
[00:41:08] +   --> $DIR/feature-gate-abi.rs:21:1
[00:41:08] 11    |
[00:41:08] 12 LL | extern "platform-intrinsic" fn f2() {} //~ ERROR platform intrinsics are experimental
[00:41:08] 
[00:41:08] 15    = help: add #![feature(platform_intrinsics)] to the crate attributes to enable
[00:41:08] 16 
[00:41:08] 16 
[00:41:08] 17 error[E0658]: vectorcall is ege (see issue #40180)
[00:41:08] -   --> $DIR/feature-gate-abi.rs:36:5
[00:41:08] +   --> $DIR/feature-gate-abi.rs:38:5
[00:41:08] 123    |
[00:41:08] 124 LL |     extern "x86-interrupt" fn m7(); //~ ERROR x86-interrupt ABI is experimental
[00:41:08] 
[00:41:08] 
[00:41:08] 127    = help: add #![feature(abi_x86_interrupt)] to the crate attributes to enable
[00:41:08] 129 error[E0658]: thiscall is experimental and subject to change
[00:41:08] -   --> $DIR/feature-gate-abi.rs:37:5
[00:41:08] +   --> $DIR/feature-gate-abi.rs:39:5
[00:41:08] 131    |
[00:41:08] 131    |
[00:41:08] 132 LL |     extern "thiscall" fn m8(); //~ ERROR thiscall is experimental and subject to change
[00:41:08] 
[00:41:08] 134    |
[00:41:08] 134    |
[00:41:08] 135    = help: add #![feature(abi_thiscall)] to the crate attributes to enable
[00:41:08] 136 
[00:41:08] - error[E0658]: amdgpu-kernel is experimental and subject to change
[00:41:08] -   --> $DIR/feature-gate-abi.rs:26:1
[00:41:08] + error[E0658]: amdgpu-kernel ABI is experimental and subject to change (see issue #51575)
[00:41:08] +   --> $DIR/feature-gate-abi.rs:40:5
[00:41:08] 139    |
[00:41:08] - LL | extern "amdgpu-kernel" fn m9() {} //~ ERROR amdgpu-kernel is experimental and subject to change
[00:41:08] -    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:41:08] + LL |     extern "amdgpu-kernel" fn m9() {} //~ ERROR amdgpu-kernel is experimental and subject to change
[00:41:08] 142    |
[00:41:08] 142    |
[00:41:08] 143    = help: add #![feature(abi_amdgpu_kernel)] to the crate attributes to enable
[00:41:08] 
[00:41:08] 
[00:41:08] 145 error[E0658]: intrinsics are subject to change
[00:41:08] -   --> $DIR/feature-gate-abi.rs:39:5
[00:41:08] +   --> $DIR/feature-gate-abi.rs:42:5
[00:41:08] 147    |
[00:41:08] 148 LL |     extern "rust-intrinsic" fn dm1() {} //~ ERROR intrinsics are subject to change
[00:41:08] 
[00:41:08] 
[00:41:08] 151    = help: add #![feature(intrinsics)] to the crate attributes to enable
[00:41:08] 152 
[00:41:08] 153 error[E0658]: platform intrinsics are experimental and possibly buggy (see issue #27731)
[00:41:08] -   --> $DIR/feature-gate-abi.rs:40:5
[00:41:08] +   --> $DIR/feature-gate-abi.rs:43:5
[00:41:08] 155    |
[00:41:08] 156 LL |     extern "platform-intrinsic" fn dm2() {} //~ ERROR platform intrinsics are experimental
[00:41:08] 
[00:41:08] 159    = help: add #![feature(platform_intrinsics)] to the crate attributes to enable
[00:41:08] 160 
[00:41:08] 161 error[E0658]: vectorcall is experimental and subject to change
[00:41:08] 161 error[E0658]: vectorcall is experimental and subject to change
[00:41:08] -   --> $DIR/feature-gate-abi.rs:41:5
[00:41:08] +   --> $DIR/feature-gate-abi.rs:44:5
[00:41:08] 163    |
[00:41:08] 164 LL |     extern "vectorcall" fn dm3() {} //~ ERROR vectorcall is experimental and subject to change
[00:41:08] 
[00:41:08] 
[00:41:08] 167    = help: add #![feature(abi_vectorcall)] to the crate attributes to enable
[00:41:08] 168 
[00:41:08] 169 error[E0658]: rust-call ABI is subject to change (see issue #29625)
[00:41:08] -   --> $DIR/feature-gate-abi.rs:42:5
[00:41:08] +   --> $DIR/feature-gate-abi.rs:45:5
[00:41:08] 171    |
[00:41:08] 172 LL |     extern "rust-call" fn dm4() {} //~ ERROR rust-call ABI is subject to change
[00:41:08] 
[00:41:08] 175    = help: add #![feature(unboxed_closures)] to the crate attributes to enable
[00:41:08] 176 
[00:41:08] 176 
[00:41:08] 177 error[E0658]: msp430-interrupt ABI is experimental and subject to change (see issue #38487)
[00:41:08] -   --> $DIR/feature-gate-abi.rs:43:5
[00:41:08] +   --> $DIR/feature-gate-abi.rs:46:5
[00:41:08] 179    |
[00:41:08] 180 LL |     extern "msp430-interrupt" fn dm5() {} //~ ERROR msp430-interrupt ABI is experimental
[00:41:08] 
[00:41:08] 
[00:41:08] 183    = help: add #![feature(abi_msp430_interrupt)] to the crate attributes to enable
[00:41:08] 184 
[00:41:08] 185 error[E0658]: PTX ABIs are experimental and subject to change (see issue #38788)
[00:41:08] -   --> $DIR/feature-gate-abi.rs:44:5
[00:41:08] +   --> $DIR/feature-gate-abi.rs:47:5
[00:41:08] 187    |
[00:41:08] 188 LL |     extern "ptx-kernel" fn dm6() {} //~ ERROR PTX ABIs are experimental and subject to change
[00:41:08] 
[00:41:08] 
[00:41:08] 191    = help: add #![feature(abi_ptx)] to the crate attributes to enable
[00:41:08] 192 
[00:41:08] 193 error[E0658]: x86-interrupt ABI is experimental and subject to change (see issue #40180)
[00:41:08] -   --> $DIR/feature-gate-abi.rs:45:5
[00:41:08] +   --> $DIR/feature-gate-abi.rs:48:5
[00:41:08] 195    |
[00:41:08] 196 LL |     extern "x86-interrupt" fn dm7() {} //~ ERROR x86-interrupt ABI is experimental
[00:41:08] 
[00:41:08] 
[00:41:08] 199    = help: add #![feature(abi_x86_interrupt)] to the crate attributes to enable
[00:41:08] 201 error[E0658]: thiscall is experimental and subject to change
[00:41:08] -   --> $DIR/feature-gate-abi.rs:46:5
[00:41:08] +   --> $DIR/feature-gate-abi.rs:49:5
[00:41:08] 203    |
[00:41:08] 203    |
[00:41:08] 204 LL |     extern "thiscall" fn dm8() {} //~ ERROR thiscall is experimental and subject to change
[00:41:08] 
[00:41:08] 206    |
[00:41:08] 206    |
[00:41:08] 207    = help: add #![feature(abi_thiscall)] to the crate attributes to enable
[00:41:08] 208 
[00:41:08] - error[E0658]: amdgpu-kernel is experimental and subject to change
[00:41:08] -   --> $DIR/feature-gate-abi.rs:26:1
[00:41:08] + error[E0658]: amdgpu-kernel ABI is experimental and subject to change (see issue #51575)
[00:41:08] +   --> $DIR/feature-gate-abi.rs:50:5
[00:41:08] 211    |
[00:41:08] 212 LL |     extern "amdgpu-kernel" fn dm9() {} //~ ERROR amdgpu-kernel is experimental and subject to change
[00:41:08] +    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:41:08] 214    |
[00:41:08] 214    |
[00:41:08] 215    = help: add #![feature(abi_amdgpu_kernel)] to the crate attributes to enable
[00:41:08] 
[00:41:08] 
[00:41:08] 217 error[E0658]: intrinsics are subject to change
[00:41:08] -   --> $DIR/feature-gate-abi.rs:53:5
[00:41:08] +   --> $DIR/feature-gate-abi.rs:57:5
[00:4e issue #38487)
[00:41:08] -   --> $DIR/feature-gate-abi.rs:69:5
[00:41:08] +   --> $DIR/feature-gate-abi.rs:74:5
[00:41:08] 323    |
[00:41:08] 324 LL |     extern "msp430-interrupt" fn im5() {} //~ ERROR msp430-interrupt ABI is experimental
[00:41:08] 
[00:41:08] 
[00:41:08] 327    = help: add #![feature(abi_msp430_interrupt)] to the crate attributes to enable
[00:41:08] 328 
[00:41:08] 329 error[E0658]: PTX ABIs are experimental and subject to change (see issue #38788)
[00:41:08] -   --> $DIR/feature-gate-abi.rs:70:5
[00:41:08] +   --> $DIR/feature-gate-abi.rs:75:5
[00:41:08] 331    |
[00:41:08] 332 LL |     extern "ptx-kernel" fn im6() {} //~ ERROR PTX ABIs are experimental and subject to change
[00:41:08] 
[00:41:08] 
[00:41:08] 335    = help: add #![feature(abi_ptx)] to the crate attributes to enable
[00:41:08] 336 
[00:41:08] 337 error[E0658]: x86-interrupt ABI is experimental and subject to change (see issue #40180)
[00:41:08] -   --> $DIR/feature-gate-abi.rs:71:5
[00:41:08] +   --> $DIR/feature-gate-abi.rs:76:5
[00:41:08] 339    |
[00:41:08] 340 LL |     extern "x86-interrupt" fn im7() {} //~ ERROR x86-interrupt ABI is experimental
[00:41:08] 
[00:41:08] 
[00:41:08] 343    = help: add #![feature(abi_x86_interrupt)] to the crate attributes to enable
[00:41:08] 345 error[E0658]: thiscall is experimental and subject to change
[00:41:08] -   --> $DIR/feature-gate-abi.rs:72:5
[00:41:08] +   --> $DIR/feature-gate-abi.rs:77:5
[00:41:08] 347    |
[00:41:08] 347    |
[00:41:08] 348 LL |     extern "thiscall" fn im8() {} //~ ERROR thiscall is experimental and subject to change
[00:41:08] 
[00:41:08] 350    |
[00:41:08] 350    |
[00:41:08] 351    = help: add #![feature(abi_thiscall)] to the crate attributes to enable
[00:41:08] 352 
[00:41:08] - error[E0658]: amdgpu-kernel is experimental and subject to change
[00:41:08] -   --> $DIR/feature-gate-abi.rs:26:1
[00:41:08] + error[E0658]: amdgpu-kernel ABI is experimental and subject to change (see issue #51575)
[00:41:08] +   --> $DIR/feature-gate-abi.rs:78:5
[00:41:08] 355    |
[00:41:08] 356 LL |     extern "amdgpu-kernel" fn im9() {} //~ ERROR amdgpu-kernel is experimental and subject to change
[00:41:08] +    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:41:08] 358    |
[00:41:08] 358    |
[00:41:08] 359    = help: add #![feature(abi_amdgpu_kernel)] to the crate attributes to enable
[00:41:08] 
[00:41:08] 
[00:41:08] 361 error[E0658]: intrinsics are subject to change
[00:41:08] -   --> $DIR/feature-gate-abi.rs:76:11
[00:41:08] +   --> $DIR/feature-gate-abi.rs:82:11
[00:41:08] 363    |
[00:41:08] 364 LL | type A1 = extern "rust-intrinsic" fn(); //~ ERROR intrinsics are subject to change
[00:41:08] 
[00:41:08] 
[00:41:08] 367    = help: add #![feature(intrinsics)] to the crate attributes to enable
[00:41:08] 368 
[00:41:08] 369 error[E0658]: platform intrinsics are experimental and possibly buggy (see issue #27731)
[00:41:08] -   --> $DIR/feature-gate-abi.rs:77:11
[00:41:08] +   --> $DIR/feature-gate-abi.rs:83:11
[00:41:08] 371    |
[00:41:08] 372 LL | type A2 = extern "platform-intrinsic" fn(); //~ ERROR platform intrinsics are experimental
[00:41:08] 
[00:41:08] 375    = help: add #![feature(platform_intrinsics)] to the crate attributes to enable
[00:41:08] 376 
[00:41:08] 377 error[E0658]: vectorcall is experimental and subject to change
[00:41:08] 377 error[E0658]: vectorcall is experimental and subject to change
[00:41:08] -   --> $DIR/feature-gate-abi.rs:78:11
[00:41:08] +   --> $DIR/feature-gate-abi.rs:84:11
[00:41:08] 379    |
[00:41:08] 380 LL | type A3 = extern "vectorcall" fn(); //~ ERROR vectorcall is experimental and subject to change
[00:41:08] 
[00:41:08] 
[00:41:08] 383    = help: add #![feature(abi_vectorcall)] to the crate attributes to enable
[00:41:08] 384 
[00:41:08] 385 error[E0658]: rust-call ABI is subject to change (see issue #29625)
[00:41:08] -   --> $DIR/feature-gate-abi.rs:79:11
[00:41:08] +   --> $DIR/feature-gate-abi.rs:85:11
[00:41:08] 387    |
[00:41:08] 388 LL | type A4 = extern "rust-call" fn(); //~ ERROR rust-call ABI is subject to change
[00:41:08] 
[00:41:08] 391    = help: add #![feature(unboxed_closures)] to the crate attributes to enable
[00:41:08] 392 
[00:41:08] 392 
[00:41:08] 393 error[E0658]: msp430-interrupt ABI is experimental and subject to change (see issue #38487)
[00:41:08] -   --> $DIR/feature-gate-abi.rs:80:11
[00:41:08] +   --> $DIR/feature-gate-abi.rs:86:11
[00:41:08] 395    |
[00:41:08] 396 LL | type A5 = extern "msp430-interrupt" fn(); //~ ERROR msp430-interrupt ABI is experimental
[00:41:08] 
[00:41:08] 
[00:41:08] 399    = help: add #![feature(abi_msp430_interrupt)] to the crate attributes to enable
[00:41:08] 400 
[00:41:08] 401 error[E0658]: PTX ABIs are experimental and subject to change (see issue #38788)
[00:41:08] -   --> $DIR/feature-gate-abi.rs:81:11
[00:41:08] +   --> $DIR/feature-gate-abi.rs:87:11
[00:41:08] 403    |
[00:41:08] 404 LL | type A6 = extern "ptx-kernel" fn (); //~ ERROR PTX ABIs are experimental and subject to change
[00:41:08] 
[00:41:08] 
[00:41:08] 407    = help: add #![feature(abi_ptx)] to the crate attributes to enable
[00:41:08] 408 
[00:41:08] 409 error[E0658]: x86-interrupt ABI is experimental and subject to change (see issue #40180)
[00:41:08] -   --> $DIR/feature-gate-abi.rs:82:11
[00:41:08] +   --> $DIR/feature-gate-abi.rs:88:11
[00:41:08] 411    |
[00:41:08] 412 LL | type A7 = extern "x86-interrupt" fn(); //~ ERROR x86-interrupt ABI is experimental
[00:41:08] 
[00:41:08] 
[00:41:08] 415    = help: add #![feature(abi_x86_interrupt)] to the crate attributes to enable
[00:41:08] 417 error[E0658]: thiscall is experimental and subject to change
[00:41:08] -   --> $DIR/feature-gate-abi.rs:83:11
[00:41:08] +   --> $DIR/feature-gate-abi.rs:89:11
[00:41:08] 419    |
[00:41:08] 419    |
[00:41:08] 420 LL | type A8 = extern "thiscall" fn(); //~ ERROR thiscall is experimental and subject to change
[00:41:08] 
[00:41:08] 422    |
[00:41:08] 422    |
[00:41:08] 423  0:41:08] {"message":"intrinsics are subject to change","code":{"code":"E0658","explanation":"\nAn unstable feature was used.\n\nErroneous code example:\n\n