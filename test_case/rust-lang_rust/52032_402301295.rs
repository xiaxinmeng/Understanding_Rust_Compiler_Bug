plain
[00:44:00] ................................................................................................i.F.
[00:44:06] ............................................................i.......................................
[00:44:11] ....................................................................................................
[00:44:14] ....................................................................................................
[00:44:18] ......................................................................................F.............
[00:44:27] ....................................................................................................
[00:44:31] ....................................................................................................
[00:44:38] ....................................................................................................
[00:44:43] ....................................................................................................
---
[00:45:15] 
[00:45:15] ---- [ui] ui/codemap_tests/unicode.rs stdout ----
[00:45:15] diff of stderr:
[00:45:15] 
[00:45:15] 4 LL | extern "路濫狼á́́" fn foo() {} //~ ERROR invalid ABI
[00:45:15] 5    |        ^^^^^^^^^ invalid ABI
[00:45:15] 6    |
[00:45:15] -    = help: valid ABIs: cdecl, stdcall, fastcall, vectorcall, thiscall, aapcs, win64, sysv64, ptx-kernel, msp430-interrupt, x86-interrupt, Rust, C, system, rust-intrinsic, rust-call, platform-intrinsic, unadjusted
[00:45:15] +    = help: valid ABIs: cdecl, stdcall, fastcall, vectorcall, thiscall, aapcs, win64, sysv64, ptx-kernel, msp430-interrupt, x86-interrupt, amdgpu-kernel, Rust, C, system, rust-intrinsic, rust-call, platform-intrinsic, unadjusted
[00:45:15] 9 error: aborting due to previous error
[00:45:15] 10 
[00:45:15] 
[00:45:15] 
[00:45:15] 
[00:45:15] The actual stderr differed from the expected stderr.
[00:45:15] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/codemap_tests/unicode/unicode.stderr
[00:45:15] To update references, rerun the tests and pass the `--bless` flag
[00:45:15] To only update this specific test, also pass `--test-args codemap_tests/unicode.rs`
[00:45:15] error: 1 errors occurred comparing output.
[00:45:15] status: exit code: 101
[00:45:15] status: exit code: 101
[00:45:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/codemap_tests/unicode.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/codemap_tests/unicode/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/codemap_tests/unicode/auxiliary" "-A" "unused"
[00:45:15] ------------------------------------------
[00:45:15] 
[00:45:15] ------------------------------------------
[00:45:15] stderr:
[00:45:15] stderr:
[00:45:15] ------------------------------------------
[00:45:15] {"message":"invalid ABI: found `路濫狼á́́`","code":{"code":"E0703","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/codemap_tests/unicode.rs","byte_start":474,"byte_end":491,"line_start":11,"line_end":11,"column_start":8,"column_end":16,"is_primary":true,"text":[{"text":"extern \"路濫狼á́́\" fn foo() {} //~ ERROR invalid ABI","highlight_start":8,"highlight_end":16}],"label":"invalid ABI","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"valid ABIs: cdecl, stdcall, fastcall, vectorcall, thiscall, aapcs, win64, sysv64, ptx-kernel, msp430-interrupt, x86-interrupt, amdgpu-kernel, Rust, C, system, rust-intrinsic, rust-call, platform-intrinsic, unadjusted","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0703]: invalid ABI: found `路濫狼á́́`\n  --> /checkout/src/test/ui/codemap_tests/unicode.rs:11:8\n   |\nLL | extern \"路濫狼á́́\" fn foo() {} //~ ERROR invalid ABI\n   |        ^^^^^^^^^ invalid ABI\n   |\n   = help: valid ABIs: cdecl, stdcall, fastcall, vectorcall, thiscall, aapcs, win64, sysv64, ptx-kernel, msp430-interrupt, x86-interrupt, amdgpu-kernel, Rust, C, system, rust-intrinsic, rust-call, platform-intrinsic, unadjusted\n\n"}
[00:45:15] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:45:15] {"message":"For more information about this error, try `rustc --explain E0703`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0703`.\n"}
[00:45:15] ------------------------------------------
[00:45:15] 
[00:45:15] thread '[ui] ui/codemap_tests/unicode.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[00:45:15] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:45:15] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:45:15] 
[00:45:15] ---- [ui] ui/feature-gate-abi.rs stdout ----
[00:45:15] diff of stderr:
[00:45:15] 
[00:45:15] 1 error[E0658]: intrinsics are subject to change
[00:45:15] -   --> $DIR/feature-gate-abi.rs:19:1
[00:45:15] +   --> $DIR/feature-gate-abi.rs:20:1
[00:45:15] 3    |
[00:45:15] 4 LL | extern "rust-intrinsic" fn f1() {} //~ ERROR intrinsics are subject to change
[00658]: msp430-interrupt ABI is experimental and subject to change (see issue #38487)
[00:45:15] -   --> $DIR/feature-gate-abi.rs:23:1
[00:45:15] +   --> $DIR/feature-gate-abi.rs:24:1
[00:45:15] 35    |
[00:45:15] 36 LL | extern "msp430-interrupt" fn f5() {} //~ ERROR msp430-interrupt ABI is experimental
[00:45:15] 
[00:45:15] 
[00:45:15] 39    = help: add #![feature(abi_msp430_interrupt)] to the crate attributes to enable
[00:45:15] 40 
[00:45:15] 41 error[E0658]: PTX ABIs are experimental and subject to change (see issue #38788)
[00:45:15] -   --> $DIR/feature-gate-abi.rs:24:1
[00:45:15] +   --> $DIR/feature-gate-abi.rs:25:1
[00:45:15] 43    |
[00:45:15] 44 LL | extern "ptx-kernel" fn f6() {} //~ ERROR PTX ABIs are experimental and subject to change
[00:45:15] 
[00:45:15] 
[00:45:15] 47    = help: add #![feature(abi_ptx)] to the crate attributes to enable
[00:45:15] 48 
[00:45:15] 49 error[E0658]: x86-interrupt ABI is experimental and subject to change (see issue #40180)
[00:45:15] -   --> $DIR/feature-gate-abi.rs:25:1
[00:45:15] +   --> $DIR/feature-gate-abi.rs:26:1
[00:45:15] 51    |
[00:45:15] 52 LL | extern "x86-interrupt" fn f7() {} //~ ERROR x86-interrupt ABI is experimental
[00:45:15] 
[00:45:15] 
[00:45:15] 55    = help: add #![feature(abi_x86_interrupt)] to the crate attributes to enable
[00:45:15] 57 error[E0658]: thiscall is experimental and subject to change
[00:45:15] -   --> $DIR/feature-gate-abi.rs:26:1
[00:45:15] -   --> $DIR/feature-gate-abi.rs:26:1
[00:45:15] +   --> $DIR/feature-gate-abi.rs0:45:15] 79    = help: add #![feature(platform_intrinsics)] to the crate attributes to enable
[00:45:15] 81 error[E0658]: vectorcall is experimental and subject to change
[00:45:15] -   --> $DIR/feature-gate-abi.rs:32:5
[00:45:15] +   --> $DIR/feature-gate-abi.rs:34:5
[00:45:15] 83    |
[00:45:15] 83    |
[00:45:15] 84 LL |     extern "vectorcall" fn m3(); //~ ERROR vectorcall is experimental and subject to change
[00:45:15] 
[00:45:15] 
[00:45:15] 87    = help: add #![feature(abi_vectorcall)] to the crate attributes to enable
[00:45:15] 88 
[00:45:15] 89 error[E0658]: rust-call ABI is subject to change (see issue #29625)
[00:45:15] -   --> $DIR/feature-gate-abi.rs:33:5
[00:45:15] +   --> $DIR/feature-gate-abi.rs:35:5
[00:45:15] 91    |
[00:45:15] 92 LL |     extern "rust-call" fn m4(); //~ ERROR rust-call ABI is subject to change
[00:45:15] 
[00:45:15] 95    = help: add #![feature(unboxed_closures)] to the crate attributes to enable
[00:45:15] 96 
[00:45:15] 96 
[00:45:15] 97 error[E0658]: msp430-interrupt ABI is experimental and subject to change (see issue #38487)
[00:45:15] -   --> $DIR/feature-gate-abi.rs:34:5
[00:45:15] +   --> $DIR/feature-gate-abi.rs:36:5
[00:45:15] 99    |
[00:45:15] 100 LL |     extern "msp430-interrupt" fn m5(); //~ ERROR msp430-interrupt ABI is experimental
[00:45:15] 
[00:45:15] 
[00:45:15] 103    = help: add #![feature(abi_msp430_interrupt)] to the crate attributes to enable
[00:45:15] 104 
[00:45:15] 105 error[E0658]: PTX ABIs are experimental and subject to change (see issue #38788)
[00:45:15] -   --> $DIR/feature-gate-abi.rs:35:5
[00:45:15] +   --> $DIR/feature-gate-abi.rs:37:5
[00:45:15] 107    |
[00:45:15] 108 LL |     extern "ptx-kernel" fn m6(); //~ ERROR PTX ABIs are experimental and subject to change
[00:45:15] 
[00:45:15] 
[00:45:15] 111    = help: add #![feature(abi_ptx)] to the crate attributes to enable
[00:45:15] 112 
[00:45:15] 113 error[E0658]: x86-interrupt ABI is experimental and subject to change (see issue #40180)
[00:45:15] -   --> $DIR/feature-gate-abi.rs:36:5
[00:45:15] +   --> $DIR/feature-gate-abi.rs:38:5
[00:45:15] 115    |
[00:45:15] 116 LL |     extern "x86-interrupt" fn m7(); //~ ERROR x86-interrupt ABI is experimental
[00:45:15] 
[00:45:15] 
[00:45:15] 119    = help: add #![feature(abi_x86_interrupt)] to the crate attributes to enable
[00:45:15] 121 error[E0658]: thiscall is experimental and subject to change
[00:45:15] -   --> $DIR/feature-gate-abi.rs:37:5
[00:45:15] +   --> $DIR/feature-gate-abi.rs:39:5
[00:45:15] 123    |
[00:45:15] 123    |
[00:45:15] 124 LL |     extern "thiscall" fn m8(); //~ ERROR thiscall is experimental and subject to change
[00:45:15] 
[00:45:15] 126    |
[00:45:15] 126    |
[00:45:15] 127    = help: add #![feature(abi_thiscall)] to the crate attributes to enable
[00:45:15] 128 
[00:45:15] + error[E0658]: amdgpu-kernel ABI is experimental and subject to change (see issue #51575)
[00:45:15] +   --> $DIR/feature-gate-abi.rs:40:5
[00:45:15] +    |
[00:45:15] + LL |     extern "amdgpu-kernel" fn m9() {} //~ ERROR amdgpu-kernel is experimental and subject to change
[00:45:15] +    |
[00:45:15] +    |
[00:45:15] +    = help: add #![feature(abi_amdgpu_kernel)] to the crate attributes to enable
[00:45:15] + 
[00:45:15] 129 error[E0658]: intrinsics are subject to change
[00:45:15] -   --> $DIR/feature-gate-abi.rs:39:5
[00:45:15] +   --> $DIR/feature-gate-abi.rs:42:5
[00:45:15] 131    |
[00:45:15] 132 LL |     extern "rust-intrinsic" fn dm1() {} //~ ERROR intrinsics are subject to change
[00:45:15] 
[00:45:15] 
[00:45:15] 135    = help: add #![feature(intrinsics)] to the crate attributes to enable
[00:45:15] 136 
[00:45:15] 137 error[E0658]: platform intrinsics are experimental and possibly buggy (see issue #27731)
[00:45:15] -   --> $DIR/feature-gate-abi.rs:40:5
[00:45:15] +   --> $DIR/feature-gate-abi.rs:43:5
[00:45:15] 139    |
[00:45:15] 140 LL |     extern "platform-intrinsic" fn dm2() {} //~ ERROR platform intrinsics are experimental
[00:45:15] 
[00:45:15] 143    = help: add #![feature(platform_intrinsics)] to the crate attributes to enable
[00:45:15] 144 
[00:45:15] 145 error[E0658]: vectorcall is experimental and subject to change
[00:45:15] 145 error[E0658]: vectorcall is experimental and subject to change
[00:45:15] -   --> $DIR/feature-gate-abi.rs:41:5
[00:45:15] +   --> $DIR/feature-gate-abi.rs:44:5
[00:45:15] 147    |
[00:45:15] 148 LL |     extern "vectorcall" fn dm3() {} //~ ERROR vectorcall is experimental and subject to change
[00:45:0:45:15] 195    |
[00:45:0:45:15] 195    |
[00:45:15] 196 LL |     extern "rust-intrinsic" fn m1() {} //~ ERROR intrinsics are subject to change
[00:45:15] 
[00:45:15] 
[00:45:15] 199    = help: add #![feature(intrinsics)] to the crate attributes to enable
[00:45:15] 200 
[00:45:15] 201 error[E0658]: platform intrinsics are experimental and possibly buggy (see issue #27731)
[00:45:15] -   --> $DIR/feature-gate-abi.rs:54:5
[00:45:15] +   --> $DIR/feature-gate-abi.rs:58:5
[00:45:15] 203    |
[00:45:15] 204 LL |     extern "platform-intrinsic" fn m2() {} //~ ERROR platform intrinsics are experimental
[00:45:15] 
[00:45:15] 207    = help: add #![feature(platform_intrinsics)] to the crate attributes to enable
[00:45:15] 208 
[00:45:15] 209 error[E0658]: vectorcall is experimental and subject to change
[00:45:15] 209 error[E0658]: vectorcall is experimental and subject to change
[00:45:15] -   --> $DIR/feature-gate-abi.rs:55:5
[00:45:15] +   --> $DIR/feature-gate-abi.rs:59:5
[00:45:15] 211    |
[00:45:15] 212 LL |     extern "vectorcall" fn m3() {} //~ ERROR vectorcall is experimental and subject to change
[00:45:15] 
[00:45:15] 
[00:45:15] 215    = help: add #![feature(abi_vectorcall)] to the crate attributes to enable
[00:45:15] 216 
[00:45:15] 217 error[E0658]: rust-call ABI is subject to change (see issue #29625)
[00:45:15] -   --> $DIR/feature-gate-abi.rs:56:5
[00:45:15] +   --> $DIR/feature-gate-abi.rs:60:5
[00:45:15] 219    |
[00:45:15] 220 LL |     extern "rust-call" fn m4() {} //~ ERROR rust-call ABI is subject to change
[00:45:15] 
[00:45:15] 223    = help: add #![feature(unboxed_closures)] to the crate attributes to enable
[00:45:15] 224 
[00:45:15] 224 
[00:45:15] 225 error[E0658]: msp430-interrupt ABI is experimental and subject to change (see issue #38487)
[00:45:15] -   --> $DIR/feature-gate-abi.rs:57:5
[00:45:15] +   --> $DIR/feature-gate-abi.rs:61:5
[00:45:15] 227    |
[00:45:15] 228 LL |     extern "msp430-interrupt" fn m5() {} //~ ERROR msp430-interrupt ABI is experimental
[00:45:15] 
[00:45:15] 
[00:45:15] 231    = help: add #![feature(abi_msp430_interrupt)] to the crate attributes to enable
[00:45:15] 232 
[00:45:15] 233 error[E0658]: PTX ABIs are experimental and subject to change (see issue #38788)
[00:45:15] -   --> $DIR/feature-gate-abi.rs:58:5
[00:45:15] +   --> $DIR/feature-gate-abi.rs:62:5
[00:45:15] 235    |
[00:45:15] 236 LL |     extern "ptx-kernel" fn m6() {} //~ ERROR PTX ABIs are experimental and subject to change
[00:45:15] 
[00:45:15] 
[00:45:15] 239    = help: add #![feature(abi_ptx)] to the crate attributes to enable
[00:45:15] 240 
[00:45:15] 241 error[E0658]: x86-interrupt ABI is experimental and subject to change (see issue #40180)
[00:45:15] -   --> $DIR/feature-gate-abi.rs:59:5
[00:45:15] +   --> $DIR/feature-gate-abi.rs:63:5
[00:45:15] 243    |
[00:45:15] 244 LL |     extern "x86-interrupt" fn m7() {} //~ ERROR x86-interrupt ABI is experimental
[00:45:15] 
[00:45:15] 
[00:45:15] 247    = help: add #![feature(abi_x86_interrupt)] to the crate attributes to enable
[00:45:15] 249 error[E0658]: thiscall is experimental and subject to change
[00:45:15] -   --> $DIR/feature-gate-abi.rs:60:5
[00:45:15] +   --> $DIR/feature-gate-abi.rs:64:5
[00:45:15] 251    |
[00:45:15] 251    |
[00:45:15] 252 LL |     extern "thiscall" fn m8() {} //~ ERROR thiscall is experimental and subject to change
[00:45:15] 
[00:45:15] 254    |
[00:45:15] 254    |
[00:45:15] 255    = help: add #![feature(abi_thiscall)] to the crate attributes to enable
[00:45:15] 256 
[00:45:15] - error[E0658]: intrinsics are subject to change
[00:45:15] + error[E0658]: amdgpu-kernel ABI is experimental and subject to change (see issue #51575)
[00:45:15] 258   --> $DIR/feature-gate-abi.rs:65:5
[00:45:15] 259    |
[00:45:15] + LL |     extern "amdgpu-kernel" fn m9() {} //~ ERROR amdgpu-kernel is experimental and subject to change
[00:45:15] +    |
[00:45:15] +    |
[00:45:15] +    = help: add #![feature(abi_amdgpu_kernel)] to the crate attributes to enable
[00:45:15] + 
[00:45:15] + error[E0658]: intrinsics are subject to change
[00:45:15] +   --> $DIR/feature-gate-abi.rs:70:5
[00:45:15] +    |
[00:45:15] 260 LL |     extern "rust-intrinsic" fn im1() {} //~ ERROR intrinsics are subject to change
[00:45:15] 262    |
[00:45:15] 
[00:45:15] 
[00:45:15] 263    = help: add #![feature(intrinsics)] to the crate attributes to enable
[00:45:15] 264 
[00:45:15] 265 error[E0658]: platform intrinsics are experimental and possibly buggy (see issue #27731)
[00:45:15] -   --> $DIR/feature-gate-abi.rs:66:5
[00:45:15] +   --> $DIR/feature-gate-abi.rs:71:5
[00:45:15] 267    |
[00:45:15] 268 LL |     extern "platform-intrinsic" fn im2() {} //~ ERROR platform intrinsics are experimental
[00:45:15] 
[00:45:15] 271    = help: add #![feature(platform_intrinsics)] to the crate attributes to enable
[00:45:15] 272 
[00:45:15] 273 error[E0658]: vectorcall is experimental and subject to change
[00:45:15] 273 error[E0658]: vectorcall is experimental and subject to change
[00:45:15] -   --> $DIR/feature-gate-abi.rs:67:5
[00:45:15] +   --> $DIR/feature-gate-abi.rs:72:5
[00:45:15] 275    |
[00:45:15] 276 LL |     extern "vectorcall" fn im3() {} //~ ERROR vectorcall is experimental and subject to change
[00:45:15] 
[00:45:15] 
[00:45:15] 279    = help: add #![feature(abi_vectorcall)] to the crate attributes to enable
[00:45:15] 280 
[00:45:15] 281 error[E0658]: rust-call ABI is subject to change (see issue #29625)
[00:45:15] -   --> $DIR/feature-gate-abi.rs:68:5
[00:45:15] +   --> $DIR/feature-gate-abi.rs:73:5
[00:45:15] 283    |
[00:45:15] 284 LL |     extern "rust-call" fn im4() {} //~ ERROR rust-call ABI is subject to change
[00:45:15] 
[00:45:15] 287    = help: add #![feature(unboxed_closures)] to the crate attributes to enable
[00:45:15] 288 
[00:45:15] 288 
[00:45:15] 289 error[E0658]: msp430-interrupt ABI is experimental and subject to change (see issue #38487)
[00:45:15] -   --> $DIR/feature-gate-abi.rs:69:5
[00:45:15] +   --> $DIR/feature-gate-abi.rs:74:5
[00:45:15] 291    |
[00:45:15] 292 LL |     extern "msp430-interrupt" fn im5() {} //~ ERROR msp430-interrupt ABI is experimental
[00:45:15] 
[00:45:15] 
[00:45:15] 295    = help: add #![feature(abi_msp430_interrupt)] to the crate attributes to enable
[00:45:15] 296 
[00:45:15] 297 error[E0658]: PTX ABIs are experimental and subject to change (see issue #38788)
[00:45:15] -   --> $DIR/feature-gate-abi.rs:70:5
[00:45:15] +   --> $DIR/feature-gate-abi.rs:75:5
[00:45:15] 299    |
[00:45:15] 300 LL |     extern "ptx-kernel" fn im6() {} //~ ERROR PTX ABIs are experimental and subject to change
[00:45:15] 
[00:45:15] 
[00:45:15] 303    = help: add #![feature(abi_ptx)] to the crate attributes to enable
[00:45:15] 304 
[00:45:15] 305 error[E0658]: x86-interrupt ABI is experimental and subject to change (see issue #40180)
[00:45:15] -   --> $DIR/feature-gate-abi.rs:71:5
[00:45:15] +   --> $DIR/feature-gate-abi.rs:76:5
[00:45:15] 307    |
[00:45:15] 308 LL |     extern "x86-interrupt" fn im7() {} //~ ERROR x86-interrupt ABI is experimental
[00:45:15] 
[00:45:15] 
[00:45:15] 311    = help: add #![feature(abi_x86_interrupt)] to the crate attributes to enable
[00:45:15] 313 error[E0658]: thiscall is experimental and subject to change
[00:45:15] -   --> $DIR/feature-gate-abi.rs:72:5
[00:45:15] +   --> $DIR/feature-gate-abi.rs:77:5
[00:45:15] 315    |
[00:45:15] 315    |
[00:45:15] 316 LL |     extern "thiscall" fn im8() {} //~ ERROR thiscall is experimental and subject to change
[00:45:15] 
[00:45:15] 318    |
[00:45:15] 318    |
[00:45:15] 319    = help: add #![feature(abi_thiscall)] to the crate attributes to enable
[00:45:15] 320 
[00:45:15] + error[E0658]: amdgpu-kernel ABI is experimental and subject to change (see issue #51575)
[00:45:15] +   --> $DIR/feature-gate-abi.rs:78:5
[00:45:15] +    |
[00:45:15] + LL |     extern "amdgpu-kernel" fn im9() {} //~ ERROR amdgpu-kernel is experimental and subject to change
[00:45:15] +    |
[00:45:15] +    |
[00:45:15] +    = help: add #![feature(abi_amdgpu_kernel)] to the crate attributes to enable
[00:45:15] + 
[00:45:15] 321 error[E0658]: intrinsics are subject to change
[00:45:15] -   --> $DIR/feature-gate-abi.rs:76:11
[00:45:15] +   --> $DIR/feature-gate-abi.rs:82:11
[00:45:15] 323    |
[00:45:15] 324 LL | type A1 = extern "rust-intrinsic" fn(); //~ ERROR intrinsics are subject to change
[00:45:15] 
[00:45:15] 
[00:45:15] 327    = help: add #![feature(intrinsics)] to the crate attributes to enable
[00:45:15] 328 
[00:45:15] 329 error[E0658]: platform intrinsics are experimental and possibly buggy (see issue #27731)
[00:45:15] -   --> $DIR/feature-gate-abi.rs:77:11
[00:45:15] +   --> $DIR/feature-gate-abi.rs:83:11
[00:45:15] 331    |
[00:45:15] 332 LL | type A2 = extern "platform-intrinsic" fn(); //~ ERROR platform intrinsics are experimental
[00:45:15] 
[00:45:15] 335    = help: add #![feature(platform_intrinsics)] to the crate attributes to enable
[00:45:15] 336 
[00:45:15] 337 error[E0658]:o change
[00:45:15] 337 error[E0658]:o change
[00:45:15] +    |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:45:15] +    |
[00:45:15] +    = help: add #![feature(abi_amdgpu_kernel)] to the crate attributes to enable
[00:45:15] + 
[00:45:15] 385 error[E0658]: intrinsics are subject to change
[00:45:15] -   --> $DIR/feature-gate-abi.rs:86:1
[00:45:15] +   --> $DIR/feature-gate-abi.rs:93:1
[00:45:15] 387    |
[00:45:15] 388 LL | extern "rust-intrinsic" {} //~ ERROR intrinsics are subject to change
[00:45:15] 
[00:45:15] 
[00:45:15] 391    = help: add #![feature(intrinsics)] to the crate attributes to enable
[00:45:15] 392 
[00:45:15] 393 error[E0658]: platform intrinsics are experimental and possibly buggy (see issue #27731)
[00:45:15] -   --> $DIR/feature-gate-abi.rs:87:1
[00:45:15] +   --> $DIR/feature-gate-abi.rs:94:1
[00:45:15] 395    |
[00:45:15] 396 LL | extern "platform-intrinsic" {} //~ ERROR platform intrinsics are experimental
[00:45:15] 
[00:45:15] 399    = help: add #![feature(platform_intrinsics)] to the crate attributes to enable
[00:45:15] 400 
[00:45:15] 401 error[E0658]: vectorcall is experimental and subject to change
[00:45:15] 401 error[E0658]: vectorcall is experimental and subject to change
[00:45:15] -   --> $DIR/feature-gate-abi.rs:88:1
[00:45:15] +   --> $DIR/feature-gate-abi.rs:95:1
[00:45:15] 403    |
[00:45:15] 404 LL | extern "vectorcall" {} //~ ERROR vectorcall is experimental and subject to change
[00:45:15] 
[00:45:15] 
[00:45:15] 407    = help: add #![feature(abi_vectorcall)] to the crate attributes to enable
[00:45:15] 408 
[00:45:15] 409 error[E0658]: rust-call ABI is subject to change (see issue #29625)
[00:45:15] -   --> $DIR/feature-gate-abi.rs:89:1
[00:45:15] +   --> $DIR/feature-gate-abi.rs:96:1
[00:45:15] 411    |
[00:45:15] 412 LL | extern "rust-call" {} //~ ERROR rust-call ABI is subject to change
[00:45:15] 
[00:45:15] 415    = help: add #![feature(unboxed_closures)] to the crate attributes to enable
[00:45:15] 416 
[00:45:15] 416 
[00:45:15] 417 error[E0658]: msp430-interrupt ABI is experimental and subject to change (see issue #38487)
[00:45:15] -   --> $DIR/feature-gate-abi.rs:90:1
[00:45:15] +   --> $DIR/feature-gate-abi.rs:97:1
[00:45:15] 419    |
[00:45:15] 420 LL | extern "msp430-interrupt" {} //~ ERROR msp430-interrupt ABI is experimental
[00:45:15] 
[00:45:15] 
[00:45:15] 423    = help: add #![feature(abi_msp430_interrupt)] to the crate attributes to enable
[00:45:15] 424 
[00:45:15] 425 error[E0658]: PTX ABIs are experimental and subject to change (see issue #38788)
[00:45:15] -   --> $DIR/feature-gate-abi.rs:91:1
[00:45:15] +   --> $DIR/feature-gate-abi.rs:98:1
[00:45:15] 427    |
[00:45:15] 428 LL | extern "ptx-kernel" {} //~ ERROR PTX ABIs are experimental and subject to change
[00:45:15] 
[00:45:15] 
[00:45:15] 431    = help: add #![feature(abi_ptx)] to the crate attributes to enable
[00:45:15] 432 
[00:45:15] 433 error[E0658]: x86-interrupt ABI is experimental and subject to change (see issue #40180)
[00:45:15] -   --> $DIR/feature-gate-abi.rs:92:1
[00:45:15] +   --> $DIR/feature-gate-abi.rs:99:1
[00:45:15] 435    |
[00:45:15] 436 LL | extern "x86-interrupt" {} //~ ERROR x86-interrupt ABI is experimental
[00:45:15] 
[00:45:15] 
[00:45:15] 439    = help: add #![feature(abi_x86_interrupt)] to the crate attributes to enable
[00:45:15] 441 error[E0658]: thiscall is experimental and subject to change
[00:45:15] -   --> $DIR/feature-gate-abi.rs:93:1
[00:45:15] +   --> $DIR/feature-gate-abi.rs:100:1
[00:45:15] 443    |
[00:45:15] 443    |
[00:45:15] 444 LL | extern "thiscall" {} //~ ERROR thiscall is experimental and subject to change
[00:45:15] 
[00:45:15] 446    |
[00:45:15] 446    |
[00:45:15] 447    = help: add #![feature(abi_thiscall)] to the crate attributes to enable
[00:45:15] - error: aborting due to 56 previous errors
[00:45:15] - error: aborting due to 56 previous errors
[00:45:15] + error[E0658]: amdgpu-kernel ABI is experimental and subject to change (see issue #51575)
[00:45:15] +   --> $DIR/feature-gate-abi.rs:101:1
[00:45:15] +    |
[00:45:15] + LL | extern "amdgpu-kernel" {} //~ ERROR amdgpu-kernel is experimental and subject to change
[00:45:15] +    |
[00:45:15] +    |
[00:45:15] +    = help: add #![feature(abi_amdgpu_kernel)] to the crate attributes to enable
[00:45:15] + error: aborting due to 63 previous errors
[00:45:15] 450 
[00:45:15] 451 For more information about this error, try `rustc --explain E0658`.
[00:45:15] 452 
[00:45:15] 452 
[00:45:15] 
[00:45:15] 
[00:45:15] The actual stderr differed from the expected stderr.
[00:45:15] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gate-abi/feature-gate-abi.stderr
[00:45:15] To update referlevel":"error","spans":[{"file_name":"/checkout/src/test/ui/feature-gate-abi.rs","byte_start":675,"byte_end":709,"line_start":20,"line_end":20,"column_start":1,"column_end":35,"is_primary":true,"text":[{"text":"extern \"rust-intrinsic\" fn f1() {} //~ ERROR intrinsics are subject to change","highlight_start":1,"highlight_end":35}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"add #![feature(intrinsics)] to the crate attributes to enable","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0658]: intrinsics are subject to change\n  --> /checkout/src/test/ui/feature-gate-abi.rs:20:1\n   |\nLL | extern \"rust-intrinsic\" fn f1() {} //~ ERROR intrinsics are subject to change\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: add #![feature(intrinsics)] to the crate attributes to enable\n\n"}
[00:45:15] {"message":"platform intrinsics are experimental and possibly buggy (see issue #27731)","code":{"code":"E0658","explanation":"\nAn unstable feature was used.\n\nErroneous code example:\n\n