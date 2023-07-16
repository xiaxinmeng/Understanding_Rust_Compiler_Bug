plain
[00:43:47] ..................................................................................................i.
[00:43:51] ...............................................................i....................................
[00:43:54] ....................................................................................................
[00:43:57] ....................................................................................................
[00:43:59] .......................................................................................F............
[00:44:05] ....................................................................................................
[00:44:08] ....................................................................................................
[00:44:11] ....................................................................................................
[00:44:14] ....................................................................................................
---
[00:44:35] 
[00:44:35] ---- [ui] ui/feature-gate-abi.rs stdout ----
[00:44:35] diff of stderr:
[00:44:35] 
[00:44:35] 1 error[E0658]: intrinsics are subject to change
[00:44:35] -   --> $DIR/feature-gate-abi.rs:19:1
[00:44:35] +   --> $DIR/feature-gate-abi.rs:20:1
[00:44:35] 3    |
[00:44:35] 4 LL | extern "rust-intrinsic" fn f1() {} //~ ERROR intrinsics are subject to change
[00:44:35] 
[00:44:35] 
[00:44:35] 7    = help: add #![feature(intrinsics)] to the crate attributes to enable
[00:44:35] 8 
[00:44:35] 9 error[E0658]: platform intrinsics are experimental and possibly buggy (see issue #27731)
[00:44:35] -   --> $DIR/feature-gate-abi.rs:20:1
[00:44:35] +   --> $DIR/feature-gate-abi.rs:21:1
[00:44:35] 11    |
[00:44:35] 12 LL | extern "platform-intrinsic" fn f2() {} //~ ERROR platform intrinsics are experimental
[00:44:35] 
[00:44:35] 15    = help: add #![feature(platform_intrinsics)] to the crate attributes to enable
[00:44:35] 16 
[00:44:35] 17 error[E0658]: vectorcall is experimental and subject to change
[00:44:35] 17 error[E0658]: vectorcall is experimental and subject to change
[00:44:35] -   --> $DIR/feature-gate-abi.rs:21:1
[00:44:35] +   --> $DIR/feature-gate-abi.rs:22:1
[00:44:35] 19    |
[00:44:35] 20 LL | extern "vectorcall" fn f3() {} //~ ERROR vectorcall is experimental and subject to change
[00:44:35] 
[00:44:35] 
[00:44:35] 23    = help: add #![feature(abi_vectorcall)] to the crate attributes to enable
[00:44:35] 24 
[00:44:35] 25 error[E0658]: rust-call ABI is subject to change (see issue #29625)
[00:44:35] -   --> $DIR/feature-gate-abi.rs:22:1
[00:44:35] +   --> $DIR/feature-gate-abi.rs:23:1
[00:44:35] 27    |
[00:44:35] 28 LL | extern "rust-call" fn f4() {} //~ ERROR rust-call ABI is subject to change
[00:44:35] 
[00:44:35] 31    = help: add #![feature(unboxed_closures)] to the crate attributes to enable
[00:44:35] 32 
[00:44:35] 32 
[00:44:35] 33 error[E0658]: msp430-interrupt ABI is experimental and subject to change (see issue #38487)
[00:44:35] -   --> $DIR/feature-gate-abi.rs:23:1
[00:44:35] +   --> $DIR/feature-gate-abi.rs:24:1
[00:44:35] 35    |
[00:44:35] 36 LL | extern "msp430-interrupt" fn f5() {} //~ ERROR msp430-interrupt ABI is experimental
[00:44:35] 
[00:44:35] 
[00:44:35] 39    = help: add #![feature(abi_msp430_interrupt)] to the crate attributes to enable
[00:44:35] 40 
[00:44:35] 41 error[E0658]: PTX ABIs are experimental and subject to change (see issue #38788)
[00:44:35] -   --> $DIR/feature-gate-abi.rs:24:1
[00:44:35] +   --> $DIR/feature-gate-abi.rs:25:1
[00:44:35] 43    |
[00:44:35] 44 LL | extern "ptx-kernel" fn f6() {} //~ ERROR PTX ABIs are experimental and subject to change
[00:44:35] 
[00:44:35] 
[00:44:35] 47    = help: add #![feature(abi_ptx)] to the crate attributes to enable
[00:44:35] 48 
[00:44:35] 49 error[E0658]: x86-interrupt ABI is experimental and subject to change (see issue #40180)
[00:44:35] -   --> $DIR/feature-gate-abi.rs:25:1
[00:44:35] +   --> $DIR/feature-gate-abi.rs:26:1
[00:44:35] 51    |
[00:44:35] 52 LL | extern "x86-interrupt" fn f7() {} //~ ERROR x86-interrupt ABI is experimental
[00:44:35] 
[00:44:35] 
[00:44:35] 55    = help: add #![feature(abi_x86_interrupt)] to the crate attributes to enable
[00:44:35] 57 error[E0658]: thiscall is experimental and subject to change
[00:44:35] -   --> $DIR/feature-gate-abi.rs:26:1
[00:44:35] +   --> $DIR/feature-gate-abi.rs:27:1
[00:44:35] 59    |
[00:44:35] 59    |
[00:44:35] 60 LL | extern "thiscall" fn f8() {} //~ ERROR thiscall is experimental and subject to change
[00:44:35] 
[00:44:35] 62    |
[00:44:35] 62    |
[00:44:35] 63    = help: add #![feature(abi_thiscall)] to the crate attributes to enable
[00:44:35] 64 
[00:44:35] - error[E0658]: amdgpu-kernel is experimental and subject to change
[00:44:35] -   --> $DIR/feature-gate-abi.rs:26:1
[00:44:35] + error[E0658]: amdgpu-kernel ABI is experimental and subject to change (see issue #51575)
[00:44:35] +   --> $DIR/feature-gate-abi.rs:28:1
[00:44:35] 67    |
[00:44:35] 68 LL | extern "amdgpu-kernel" fn f9() {} //~ ERROR amdgpu-kernel is experimental and subject to change
[00:44:35] +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:44:35] 70    |
[00:44:35] 70    |
[00:44:35] 71    = help: add #![feature(abi_amdgpu_kernel)] to the crate attributes to enable
[00:44:35] 
[00:44:35] 
[00:44:35] 73 error[E0658]: intrinsics are subject to change
[00:44:35] -   --> $DIR/feature-gate-abi.rs:30:5
[00:44:35] +   --> $DIR/feature-gate-abi.rs:32:5
[00:44:35] 75    |
[00:44:35] 76 LL |     extern "rust-intrinsic" fn m1(); //~ ERROR intrinsics are subject to change
[00:44:35] 
[00:44:35] 
[00:44:35] 79    = help: add #![feature(intrinsics)] to the crate attributes to enable
[00:44:35] 80 
[00:44:35] 81 error[E0658]: platform intrinsics are experimental and possibly buggy (see issue #27731)
[00:44:35] -   --> $DIR/feature-gate-abi.rs:31:5
[00:44:35] +   --> $DIR/feature-gate-abi.rs:33:5
[00:44:35] 83    |
[00:44:35] 84 LL |     extern "platform-intrinsic" fn m2(); //~ ERROR platform intrinsics are experimental
[00:44:35] 
[00:44:35] 87    = help: add #![feature(platform_intrinsics)] to the crate attributes to enable
[00:44:35] 88 
[00:44:35] 89 error[E0658]: vectorcall is experimental and subject to change
[00:44:35] 89 error[E0658]: vectorcall is experimental and subject to change
[00:44:35] -   --> $DIR/feature-gate-abi.rs:32:5
[00:44:35] +   --> $DIR/feature-gate-abi.rs:34:5
[00:44:35] 91    |
[00:44:35] 92 LL |     extern "vectorcall" fn m3(); //~ ERROR vectorcall is experimental and subject to change
[00:44:35] 
[00:44:35] 
[00:44:35] 95    = help: add #![feature(abi_vectorcall)] to the crate attributes to enable
[00:44:35] 96 
[00:44:35] 97 error[E0658]: rust-call ABI is subject to change (see issue #29625)
[00:44:35] -   --> $DIR/feature-gate-abi.rs:33:5
[00:44:35] +   --> $DIR/feature-gate-abi.rs:35:5
[00:44:35] 99    |
[00:44:35] 100 LL |     extern "rust-call" fn m4(); //~ ERROR rust-call ABI is subject to change
[00:44:35] 
[00:44:35] 103    = help: add #![feature(unboxed_closures)] to the crate attributes to enable
[00:44:35] 104 
[00:44:35] 104 
[00:44:35] 105 error[E0658]: msp430-interrupt ABI is experimental and subject to change (see issue #38487)
[00:44:35] -   --> $DIR/feature-gate-abi.rs:34:5
[00:44:35] +   --> $DIR/feature-gate-abi.rs:36:5
[00:44:35] 107    |
[00:44:35] 108 LL |     extern "msp430-interrupt" fn m5(); //~ ERROR msp430-interrupt ABI is experimental
[00:44:35] 
[00:44:35] 
[00:44:35] 111    = help: add #![feature(abi_msp430_interrupt)] to the crate attributes to enable
[00:44:35] 112 
[00:44:35] 113 error[E0658]: PTX ABIs are experimental and subject to change (see issue #38788)
[00:44:35] -   --> $DIR/feature-gate-abi.rs:35:5
[00:44:35] +   --> $DIR/feature-gate-abi.rs:37:5
[00:44:35] 115    |
[00:44:35] 116 LL |     extern "ptx-kernel" fn m6(); //~ ERROR PTX ABIs are experimental and subject to change
[00:44:35] 
[00:44:35] 
[00:44:35] 119    = help: add #![feature(abi_ptx)] to the crate attributes to enable
[00:44:35] 120 
[00:44:35] 121 error[E0658]: x86-interrupt ABI is experimental and subject to change (see issue #40180)
[00:44:35] -   --> $DIR/feature-gate-abi.rs:36:5
[00:44:35] +   --> $DIR/feature-gate-abi.rs:38:5
[00:44:35] 123    |
[00:44:35] 124 LL |     extern "x86-interrupt" fn m7(); //~ ERROR x86-interrupt ABI is experimental
[00:44:35] 
[00:44:35] 
[00:44:35] 127    = help: add #![feature(abi_x86_interrupt)] to the crate attributes to enable
[00:44:35] 129 error[E0658]: thiscall is experimental and subject to change
[00:44:35] -   --> $DIR/feature-gate-abi.rs:37:5
[00:44:35] +   --> $DIR/feature-gate-abi.rs:39:5
[00:44:35] 131    |
[00:44:35] 131    |
[00:44:35] 132 LL |     extern "thiscall" fn m8(); //~ ERROR thiscall is experimental and subject to change
[00:44:35] 
[00:44:35] 134    |
[00:44:35] 134    |
[00:44:35] 135    = help: add #![feature(abi_thiscall)] to the crate attributes to enable
[00:44:35] 136 
[00:44:35] - error[E0658]: amdgpu-kernel is experimental and subject to change
[00:44:35] -   --> $DIR/feature-gate-abi.rs:26:1
[00:44:35] + error[E0658]: amdgpu-kernel ABI is experimental and subject to change (see issue #51575)
[00:44:35] +   --> $DIR/feature-gate-abi.rs:40:5
[00:44:35] 139    |
[00:44:35] - LL | extern "amdgpu-kernel" fn m9() {} //~ ERROR amdgpu-kernel is experimental and subject to change
[00:44:35] -    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:44:35] + LL |     extern "amdgpu-kernel" fn m9() {} //~ ERROR amdgpu-kernel is experimental and subject to change
[00:44:35] 142    |
[00:44:35] 142    |
[00:44:35] 143    = help: add #![feature(abi_amdgpu_kernel)] to the crate attributes to enable
[00:44:35] 
[00:44:35] 
[00:44:35] 145 error[E0658]: intrinsics are subject to change
[00:44:35] -   --> $DIR/feature-gate-abi.rs:39:5
[00:44:35] +   --> $DIR/feature-gate-abi.rs:42:5
[00:44:35] 147    |
[00:44:35] 148 LL |     extern "rust-intrinsic" fn dm1() {} //~ ERROR intrinsics are subject to change
[00:44:35] 
[00:44:35] 
[00:44:35] 151    = help: add #![feature(intrinsics)] to the crate attributes to enable
[00:44:35] 152 
[00:44:35] 153 error[E0658]: platform intrinsics are experimental and possibly buggy (see issue #27731)
[00:44:35] -   --> $DIR/feature-gate-abi.rs:40:5
[00:44:35] +   --> $DIR/feature-gate-abi.rs:43:5
[00:44:35] 155    |
[00:44:35] 156 LL |     extern "platform-intrinsic" fn dm2() {} //~ ERROR platform intrinsics are experimental
[00:44:35] 
[00:44:35] 159    = help: add #![feature(platform_intrinsics)] to the crate attributes to enable
[00:44:35] 160 
[00:44:35] 161 error[E0658]: vectorcall is experimental and subject to change
[00:44:35] 161 error[E0658]: vectorcall is experimental and subject to change
[00:44:35] -   --> $DIR/feature-gate-abi.rs:41:5
[00:44:35] +   --> $DIR/feature-gate-abi.rs:44:5
[00:44:35] 163    |
[00:44:35] 164 LL |     extern "vectorcall" fn dm3() {} //~ ERROR vectorcall is experimental and subject to change
[00:44:35] 
[00:44:35] 
[00:44:35] 167    = help: add #![feature(abi_vectorcall)] to the crate attributes to enable
[00:44:35] 168 
[00:44:35] 169 error[E0658]: rust-call ABI is subject to change (see issue #29625)
[00:44:35] -   --> $DIR/feature-gate-abi.rs:42:5
[00:44:35] +   --> $DIR/feature-gate-abi.rs:45:5
[00:44:35] 171    |
[00:44:35] 172 LL |     extern "rust-call" fn dm4() {} //~ ERROR rust-call ABI is subject to change
[00:44:35] 
[00:44:35] 175    = help: add #![feature(unboxed_closures)] to the crate attributes to enable
[00:44:35] 176 
[00:44:35] 176 
[00:44:35] 177 error[E0658]: msp430-interrupt ABI is experimental and subject to change (see issue #38487)
[00:44:35] -   --> $DIR/feature-gate-abi.rs:43:5
[00:44:35] +   --> $DIR/feature-gate-abi.rs:46:5
[00:44:35] 179    |
[00:44:35] 180 LL |     extern "msp430-interrupt" fn dm5() {} //~ ERROR msp430-interrupt ABI is experimental
[00:44:35] 
[00:44:35] 
[00:44:35] 183    = help: add #![feature(abi_msp430_interrupt)] to the crate attributes to enable
[00:44:35] 184 
[00:44:35] 185 error[E0658]: PTX ABIs are experimental and subject to change (see issue #38788)
[00:44:35] -   --> $DIR/feature-gate-abi.rs:44:5
[00:44:35] +   --> $DIR/feature-gate-abi.rs:47:5
[00:44:35] 187    |
[00:44:35] 188 LL |     extern "ptx-kernel" fn dm6() {} //~ ERROR PTX ABIs are experimental and subject to change
[00:44:35] 
[00:44:35] 
[00:44:35] 191    = help: add #![feature(abi_ptx)] to the crate attributes to enable
[00:44:35] 192 
[00:44:35] 193 error[E0658]: x86-interrupt ABI is experimental and subject to change (see issue #40180)
[00:44:35] -   --> $DIR/feature-gate-abi.rs:45:5
[00:44:35] +   --> $DIR/feature-gate-abi.rs:48:5
[00:44:35] 195    |
[00:44:35] 196 LL |     extern "x86-interrupt" fn dm7() {} //~ ERROR x86-interrupt ABI is experimental
[00:44:35] 
[00:44:35] 
[00:44:35] 199    = help: add #![feature(abi_x86_interrupt)] to the crate attributes to enable
[00:44:35] 201 error[E0658]: thiscall is experimental and subject to change
[00:44:35] -   --> $DIR/feature-gate-abi.rs:46:5
[00:44:35] +   --> $DIR/feature-gate-abi.rs:49:5
[00:44:35] 203    |
[00:44:35] 203    |
[00:44:35] 204 LL |     extern "thiscall" fn dm8() {} //~ ERROR thiscall is experimental and subject to change
[00:44:35] 
[00:44:35] 206    |
[00:44:35] 206    |
[00:44:35] 207    = help: add #![feature(abi_thiscall)] to the crate attributes to enable
[00:44:35] 208 
[00:44:35] - error[E0658]: amdgpu-kernel is experimental and subject to change
[00:44:35] -   --> $DIR/feature-gate-abi.rs:26:1
[00:44:35] + error[E0658]: amdgpu-kernel ABI is experimental and subject to change (see issue #51575)
[00:44:35] +   --> $DIR/feature-gate-abi.rs:50:5
[00:44:35] 211    |
[00:44:35] 212 LL |     extern "amdgpu-kernel" fn dm9() {} //~ ERROR amdgpu-kernel is experimental and subject to change
[00:44:35] +    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:44:35] 214    |
[00:44:35] 214    |
[00:44:35] 215    = help: add #![feature(abi_amdgpu_kernel)] to the crate attributes to enable
[00:44:35] 
[00:44:35] 
[00:44:35] 217 error[E0658]: intrinsics are subject to change
[00:44:35] -   --> $DIR/feature-gate-abi.rs:53:5
[00:44:35] +   --> $DIR/feature-gate-abi.rs:57:5
[00:44:35] 219    |
[00:44:35] 220 LL |     extern "rust-intrinsic" fn m1() {} //~ ERROR intrinsics are subject to change
[00:44:35] 
[00:44:35] 
[00:44:35] 223    = help: add #![feature(intrinsics)] to the crate attributes to enable
[00:44:35] 224 
[00:44:35] 225 error[E0658]: platform intrinsics are experimental and possibly buggy (see issue #27731)
[00:44:35] -   --> $DIR/feature-gate-abi.rs:54:5
[00:44:35] +   --> $DIR/feature-gate-abi.rs:58:5
[00:44:35] 227    |
[00:44:35] 228 LL |     extern "platform-intrinsic" fn m2() {} //~ ERROR platform intrinsics are experimental
[00:44:35] 
[00:44:35] 231    = help: add #![feature(platform_intrinsics)] to the crate attributes to enable
[00:44:35] 232 
[00:44:35] 233 error[E0658]: vectorcall is experimental and subject to change
[00:44:35] 233 error[E0658]: vectorcall is experimental and subject to change
[00:44:35] -   --> $DIR/feature-gate-abi.rs:55:5
[00:44:35] +   --> $DIR/feature-gate-abi.rs:59:5
[00:44:35] 235    |
[00:44:35] 236 LL |     extern "vectorcall" fn m3() {} //~ ERROR vectorcall is experimental and subject to change
[00:44:35] 
[00:44:35] 
[00:44:35] 239    = help: add #![feature(abi_vectorcall)] to the crate attributes to enable
[00:44:35] 240 
[00:44:35] 241 error[E0658]: rust-call ABI is subject to change (see issue #29625)
[00:44:35] -   --> $DIR/feature-gate-abi.rs:56:5
[00:44:35] +   --> $DIR/feature-gate-abi.rs:60:5
[00:44:35] 243    |
[00:44:35] 244 LL |     extern "rust-call" fn m4() {} //~ ERROR rust-call ABI is subject to change
[00:44:35] 
[00:44:35] 247    = help: add #![feature(unboxed_closures)] to the crate attributes to enable
[00:44:35] 248 
[00:44:35] 248 
[00:44:35] 249 error[E0658]: msp430-interrupt ABI is experimental and subject to change (see issue #38487)
[00:44:35] -   --> $DIR/feature-gate-abi.rs:57:5
[00:44:35] +   --> $DIR/feature-gate-abi.rs:61:5
[00:44:35] 251    |
[00:44:35] 252 LL |     extern "msp430-interrupt" fn m5() {} //~ ERROR msp430-interrupt ABI is experimental
[00:44:35] 
[00:44:35] 
[00:44:35] 255    = help: add #![feature(abi_msp430_interrupt)] to the crate attributes to enable
[00:44:35] 256 
[00:44:35] 257 error[E0658]: PTX ABIs are experimental and subject to change (see issue #38788)
[00:44:35] -   --> $DIR/feature-gate-abi.rs:58:5
[00:44:35] +   --> $DIR/feature-gate-abi.rs:62:5
[00:44:35] 259    |
[00:44:35] 260 LL |     extern "ptx-kernel" fn m6() {} //~ ERROR PTX ABIs are experimental and subject to change
[00:44:35] 
[00:44:35] 
[00:44:35] 263    = help: add #![feature(abi_ptx)] to the crate attributes to enable
[00:44:35] 264 
[00:44:35] 265 error[E0658]: x86-interrupt ABI is experimental and subject to change (see issue #40180)
[00:44:35] -   --> $DIR/feature-gate-abi.rs:59:5
[00:44:35] +   --> $DIR/feature-gate-abi.rs:63:5
[00:44:35] 267    |
[00:44:35] 268 LL |     extern "x86-interrupt" fn m7() {} //~ ERROR x86-interrupt ABI is experimental
[00:44:35] 
[00:44:35] 
[00:44:35] 271    = help: add #![feature(abi_x86_interrupt)] to the crate attributes to enable
[00:44:35] 273 error[E0658]: thiscall is experimental and subject to change
[00:44:35] -   --> $DIR/feature-gate-abi.rs:60:5
[00:44:35] +   --> $DIR/feature-gate-abi.rs:64:5
[00:44:35] 275    |
[00:44:35] 275    |
[00:44:35] 276 LL |     extern "thiscall" fn m8() {} //~ ERROR thiscall is experimental and subject to change
[00:44:35] 
[00:44:35] 278    |
[00:44:35] 278    |
[00:44:35] 279    = help: add #![feature(abi_thiscall)] to the crate attributes to enable
[00:44:35] 280 
[00:44:35] - error[E0658]: amdgpu-kernel is experimental and subject to change
[00:44:35] -   --> $DIR/feature-gate-abi.rs:26:1
[00:44:35] + error[E0658]: amdgpu-kernel ABI is experimental and subject to change (see issue #51575)
[00:44:35] +   --> $DIR/feature-gate-abi.rs:65:5
[00:44:35] 283    |
[00:44:35] 284 LL |     extern "amdgpu-kernel" fn m9() {} //~ ERROR amdgpu-kernel is experimental and subject to change
[00:44:35] +    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:44:35] 286    |
[00:44:35] 286    |
[00:44:35] 287    = help: add #![feature(abi_amdgpu_kernel)] to the crate attributes to enable
[00:44:35] 
[00:44:35] 
[00:44:35] 289 error[E0658]: intrinsics are subject to change
[00:44:35] -   --> $DIR/feature-gate-abi.rs:65:5
[00:44:35] +   --> $DIR/feature-gate-abi.rs:70:5
[00:44:35] 291    |
[00:44:35] 292 LL |     extern "rust-intrinsic" fn im1() {} //~ ERROR intrinsics are subject to change
[00:44:35] 
[00:44:35] 
[00:44:35] 295    = help: add #![feature(intrinsics)] to the crate attributes to enable
[00:44:35] 296 
[00:44:35] 297 error[E0658]: platform intrinsics are experimental and possibly buggy (see issue #27731)
[00:44:35] -   --> $DIR/feature-gate-abi.rs:66:5
[00:44:35] +   --> $DIR/feature-gate-abi.rs:71:5
[00:44:35] 299    |
[00:44:35] 300 LL |     extern "platform-intrinsic" fn im2() {} //~ ERROR platform intrinsics are experimental
[00:44:35] 
[00:44:35] 303    = help: add #![feature(platform_intrinsics)] to the crate attributes to enable
[00:44:35] 304 
[00:44:35] 305 error[E0658]: vectorcall is experimental and subject to change
[00:44:35] 305 error[E0658]: vectorcall is experimental and subject to change
[00:44:35] -   --> $DIR/feature-gate-abi.rs:67:5
[00:44:35] +   --> $DIR/feature-gate-abi.rs:72:5
[00:44:35] 307    |
[00:44:35] 308 LL |     extern "vectorcall" fn im3() {} //~ ERROR vectorcall is experimental and subject to change
[00:44:35] 
[00:44:35] 
[00:44:35] 311    = help: add #![feature(abi_vectorcall)] to the crate attributes to enable
[00:44:35] 312 
[00:44:35] 313 error[E0658]: rust-call ABI is subject to change (see issue #29625)
[00:44:35] -   --> $DIR/feature-gate-abi.rs:68:5
[00:44:35] +   --> $DIR/feature-gate-abi.rs:73:5
[00:44:35] 315    |
[00:44:35] 316 LL |     extern "rust-call" fn im4() {} //~ ERROR rust-call ABI is subject to change
[00:44:35] 
[00:44:35] 319    = help: add #![feature(unboxed_closures)] to the crate attributes to enable
[00:44:35] 320 
[00:44:35] 320 
[00:44:35] 321 error[E0658]: msp430-interrupt ABI is experimental and subject to change (see issue #38487)
[00:44:35] -   --> $DIR/feature-gate-abi.rs:69:5
[00:44:35] +   --> $DIR/feature-gate-abi.rs:74:5
[00:44:35] 323    |
[00:44:35] 324 LL |     extern "msp430-interrupt" fn im5() {} //~ ERROR msp430-interrupt ABI is experimental
[00:44:35] 
[00:44:35] 
[00:44:35] 327    = help: add #![feature(abi_msp430_interrupt)] to the crate attributes to enable
[00:44:35] 328 
[00:44:35] 329 error[E0658]: PTX ABIs are experimental and subject to change (see issue #38788)
[00:44:35] -   --> $DIR/feature-gate-abi.rs:70:5
[00:44:35] +   --> $DIR/feature-gate-abi.rs:75:5
[00:44:35] 331    |
[00:44:35] 332 LL |     extern "ptx-kernel" fn im6() {} //~ ERROR PTX ABIs are experimental and subject to change
[00:44:35] 
[00:44:35] 
[00:44:35] 335    = help: add #![feature(abi_ptx)] to the crate attributes to enable
[00:44:35] 336 
[00:44:35] 337 error[E0658]: x86-interrupt ABI is experimental and subject to change (see issue #40180)
[00:44:35] -   --> $DIR/feature-gate-abi.rs:71:5
[00:44:35] +   --> $DIR/feature-gate-abi.rs:76:5
[00:44:35] 339    |
[00:44:35] 340 LL |     extern "x86-interrupt" fn im7() {} //~ ERROR x86-interrupt ABI is experimental
[00:44:35] 
[00:44:35] 
[00:44:35] 343    = help: add #![feature(abi_x86_interrupt)] to the crate attributes to enable
[00:44:35] 345 error[E0658]: thiscall is experimental and subject to change
[00:44:35] -   --> $DIR/feature-gate-abi.rs:72:5
[00:44:35] +   --> $DIR/feature-gate-abi.rs:77:5
[00:44:35] 347    |
[00:44:35] 347    |
[00:44:35] 348 LL |     extern "thiscall" fn im8() {} //~ ERROR thiscall is experimental and subject to change
[00:44:35] 
[00:44:35] 350    |
[00:44:35] 350    |
[00:44:35] 351    = help: add #![feature(abi_thiscall)] to the crate attributes to enable
[00:44:35] 352 
[00:44:35] - error[E0658]: amdgpu-kernel is experimental and subject to change
[00:44:35] -   --> $DIR/feature-gate-abi.rs:26:1
[00:44:35] + error[E0658]: amdgpu-kernel ABI is experimental and subject to change (see issue #51575)
[00:44:35] +   --> $DIR/feature-gate-abi.rs:78:5
[00:44:35] 355    |
[00:44:35] 356 LL |     extern "amdgpu-kernel" fn im9() {} //~ ERROR amdgpu-kernel is experimental and subject to change
[00:44:35] +    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:44:35] 358    |
[00:44:35] 358    |
[00:44:35] 359    = help: add #![feature(abi_amdgpu_kernel)] to the crate attributes to enable
[00:44:35] 
[00:44:35] 
[00:44:35] 361 error[E0658]: intrinsics are subject to change
[00:44:35] -   --> $DIR/feature-gate-abi.rs:76:11
[00:44:35] +   --> $DIR/feature-gate-abi.rs:82:11
[00:44:35] 363    |
[00:44:35] 364 LL | type A1 = extern "rust-intrinsic" fn(); //~ ERROR intrinsics are subject to change
[00:44:35] 
[00:44:35] 
[00:44:35] 367    = help: add #![feature(intrinsics)] to the crate attributes to enable
[00:44:35] 368 
[00:44:35] 369 error[E0658]: platform intrinsics are experimental and possibly buggy (see issue #27731)
[00:44:35] -   --> $DIR/feature-gate-abi.rs:77:11
[00:44:35] +   --> $DIR/feature-gate-abi.rs:83:11
[00:44:35] 371    |
[00:44:35] 372 LL | type A2 = extern "platform-intrinsic" fn(); //~ ERROR platform intrinsics are experimental
[00:44:35] 
[00:44:35] 375    = help: add #![feature(platform_intrinsics)] to the crate attributes to enable
[00:44:35] 376 
[00:44:35] 377 error[E0658]: vectorcall is experimental and subject to change
[00:44:35] 377 error[E0658]: vectorcall is experimental and subject to change
[00:44:35] -   --> $DIR/feature-gate-abi.rs:78:11
[00:44:35] +   --> $DIR/feature-gate-abi.rs:84:11
[00:44:35] 379    |
[00:44:35] 380 LL | type A3 = extern "vectorcall" fn(); //~ ERROR vectorcall is experimental and subject to change
[00:44:35] 
[00:44:35] 
[00:44:35] 383    = help: add #![feature(abi_vectorcall)] to the crate attributes to enable
[00:44:35] 384 
[00:44:35] 385 error[E0658]: rust-call ABI is subject to change (see issue #29625)
[00:44:35] -   --> $DIR/feature-gate-abi.rs:79:11
[00:44:35] +   --> $DIR/feature-gate-abi.rs:85:11
[00:44:35] 387    |
[00:44:35] 388 LL | type A4 = extern "rust-call" fn(); //~ ERROR rust-call ABI is subject to change
[00:44:35] 
[00:44:35] 391    = help: add #![feature(unboxed_closures)] to the crate attributes to enable
[00:44:35] 392 
[00:44:35] 392 
[00:44:35] 393 error[E0658]: msp430-interrupt ABI is experimental and subject to change (see issue #38487)
[00:44:35] -   --> $DIR/feature-gate-abi.rs:80:11
[00:44:35] +   --> $DIR/feature-gate-abi.rs:86:11
[00:44:35] 395    |
[00:44:35] 396 LL | type A5 = extern "msp430-interrupt" fn(); //~ ERROR msp430-interrupt ABI is experimental
[00:44:35] 
[00:44:35] 
[00:44:35] 399    = help: add #![feature(abi_msp430_interrupt)] to the crate attributes to enable
[00:44:35] 400 
[00:44:35] 401 error[E0658]: PTX ABIs are experimental and subject to change (see issue #38788)
[00:44:35] -   --> $DIR/feature-gate-abi.rs:81:11
[00:44:35] +   --> $DIR/feature-gate-abi.rs:87:11
[00:44:35] 403    |
[00:44:35] 404 LL | type A6 = extern "ptx-kernel" fn (); //~ ERROR PTX ABIs are experimental and subject to change
[00:44:35] 
[00:44:35] 
[00:44:35] 407    = help: add #![feature(abi_ptx)] to the crate attributes to enable
[00:44:35] 408 
[00:44:35] 409 error[E0658]: x86-interrupt ABI is experimental and subject to change (see issue #40180)
[00:44:35] -   --> $DIR/feature-gate-abi.rs:82:11
[00:44:35] +   --> $DIR/feature-gate-abi.rs:88:11
[00:44:35] 411    |
[00:44:35] 412 LL | type A7 = extern "x86-interrupt" fn(); //~ ERROR x86-interrupt ABI is experimental
[00:44:35] 
[00:44:35] 
[00:44:35] 415    = help: add #![feature(abi_x86_interrupt)] to the crate attributes to enable
[00:44:35] 417 error[E0658]: thiscall is experimental and subject to change
[00:44:35] -   --> $DIR/feature-gate-abi.rs:83:11
[00:44:35] +   --> $DIR/feature-gate-abi.rs:89:11
[00:44:35] 419    |
[00:44:35] 419    |
[00:44:35] 420 LL | type A8 = extern "thiscall" fn(); //~ ERROR thiscall is experimental and subject to change
[00:44:35] 
[00:44:35] 422    |
---
[00:44:35] test result: FAILED. 1555 passed; 1 failed; 5 ignored; 0 measured; 0 filtered out
[00:44:35] 
[00:44:35] 
[00:44:35] 
[00:44:35] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:44:35] 
[00:44:35] 
[00:44:35] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:44:35] Build completed unsuccessfully in 0:01:26
[00:44:35] Build completed unsuccessfully in 0:01:26
[00:44:35] make: *** [check] Error 1
[00:44:35] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:123a8dae
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
