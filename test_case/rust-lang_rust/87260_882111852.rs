plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
tidy: Skipping binary file check, read-only filesystem
Checking which error codes lack tests...
* 625 error codes
* highest error code: E0783
tidy error: /checkout/compiler/rustc_codegen_gcc/Readme.md:74: tab character
tidy error: /checkout/compiler/rustc_codegen_gcc/Readme.md:76: tab character
tidy error: /checkout/compiler/rustc_codegen_gcc/Readme.md:78: tab character
tidy error: /checkout/compiler/rustc_codegen_gcc/Readme.md:80: tab character
tidy error: /checkout/compiler/rustc_codegen_gcc/Readme.md:82: tab character
tidy error: /checkout/compiler/rustc_codegen_gcc/Readme.md:84: tab character
tidy error: /checkout/compiler/rustc_codegen_gcc/Readme.md:86: tab character
tidy error: /checkout/compiler/rustc_codegen_gcc/Readme.md:88: tab character
tidy error: /checkout/compiler/rustc_codegen_gcc/Readme.md:90: tab character
tidy error: /checkout/compiler/rustc_codegen_gcc/Readme.md:92: tab character
tidy error: /checkout/compiler/rustc_codegen_gcc/test.sh:33: line longer than 100 chars
tidy error: /checkout/compiler/rustc_codegen_gcc/test.sh:39: line longer than 100 chars
tidy error: /checkout/compiler/rustc_codegen_gcc/test.sh:41: line longer than 100 chars
tidy error: /checkout/compiler/rustc_codegen_gcc/test.sh:47: line longer than 100 chars
tidy error: /checkout/compiler/rustc_codegen_gcc/test.sh:60: line longer than 100 chars
tidy error: /checkout/compiler/rustc_codegen_gcc/test.sh:67: line longer than 100 chars
tidy error: /checkout/compiler/rustc_codegen_gcc/test.sh:75: line longer than 100 chars
tidy error: /checkout/compiler/rustc_codegen_gcc/test.sh:119: line longer than 100 chars
tidy error: /checkout/compiler/rustc_codegen_gcc/test.sh:129: line longer than 100 chars
tidy error: /checkout/compiler/rustc_codegen_gcc/test.sh:130: line longer than 100 chars
tidy error: /checkout/compiler/rustc_codegen_gcc/test.sh:131: line longer than 100 chars
tidy error: /checkout/compiler/rustc_codegen_gcc/test.sh:132: line longer than 100 chars
tidy error: /checkout/compiler/rustc_codegen_gcc/test.sh:133: line longer than 100 chars
tidy error: /checkout/compiler/rustc_codegen_gcc/test.sh:136: line longer than 100 chars
tidy error: /checkout/compiler/rustc_codegen_gcc/test.sh:169: line longer than 100 chars
tidy error: /checkout/compiler/rustc_codegen_gcc/test.sh:180: line longer than 100 chars
tidy error: /checkout/compiler/rustc_codegen_gcc/test.sh:183: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/test.sh:184: line longer than 100 chars
tidy error: /checkout/compiler/rustc_codegen_gcc/prepare.sh:14: line longer than 100 chars
tidy error: /checkout/compiler/rustc_codegen_gcc/config.sh:33: line longer than 100 chars
tidy error: /checkout/compiler/rustc_codegen_gcc/config.sh:34: line longer than 100 chars
tidy error: /checkout/compiler/rustc_codegen_gcc/config.sh:44: line longer than 100 chars
tidy error: /checkout/compiler/rustc_codegen_gcc/example/alloc_system.rs:1: copyright notices attributed to the Rust Project Developers are deprecated
tidy error: /checkout/compiler/rustc_codegen_gcc/example/mini_core_hello_world.rs:256: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/tests/run/mut_ref.rs: leading newline
tidy error: /checkout/compiler/rustc_codegen_gcc/cargo.sh:16: line longer than 100 chars
tidy error: /checkout/compiler/rustc_codegen_gcc/rustup.sh:10: line longer than 100 chars
tidy error: /checkout/compiler/rustc_codegen_gcc/rustup.sh:14: line longer than 100 chars
tidy error: /checkout/compiler/rustc_codegen_gcc/src/consts.rs:35: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/consts.rs:47: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/consts.rs:112: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/consts.rs:129: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/consts.rs:141: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/consts.rs:234: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/consts.rs:245: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/consts.rs:257: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/consts.rs:258: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/consts.rs:259: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/allocator.rs:56: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/builder.rs:50: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/builder.rs:53: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/builder.rs:101: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/builder.rs:293: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/builder.rs:325: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/builder.rs:402: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/builder.rs:587: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/builder.rs:592: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/builder.rs:593: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/builder.rs:598: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/builder.rs:603: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/builder.rs:659: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/builder.rs:680: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/builder.rs:725: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/builder.rs:757: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/builder.rs:825: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/builder.rs:880: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/builder.rs:897: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/builder.rs:926: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/builder.rs:943: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/builder.rs:1052: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/builder.rs:1077: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/builder.rs:1115: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/builder.rs:1140: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/builder.rs:1146: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/builder.rs:1181: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/builder.rs:1187: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/builder.rs:1191: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/builder.rs:1227: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/builder.rs:1302: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/builder.rs:1321: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/builder.rs:1330: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/builder.rs:1541: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/builder.rs:1550: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/builder.rs:1600: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/builder.rs:1611: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/builder.rs:1616: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/builder.rs:1756: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/builder.rs:1799: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/builder.rs:1801: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/mono_item.rs:29: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/mono_item.rs:46: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/mono_item.rs:54: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/debuginfo.rs:71: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/debuginfo.rs:90: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/debuginfo.rs:133: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/base.rs:37: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/base.rs:76: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/base.rs:137: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/back/write.rs:56: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/intrinsic/simd.rs:160: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/intrinsic/llvm.rs:14: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/intrinsic/mod.rs:164: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/intrinsic/mod.rs:277: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/intrinsic/mod.rs:292: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/intrinsic/mod.rs:359: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/intrinsic/mod.rs:366: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/intrinsic/mod.rs:374: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/intrinsic/mod.rs:597: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/intrinsic/mod.rs:644: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/intrinsic/mod.rs:678: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/intrinsic/mod.rs:698: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/intrinsic/mod.rs:836: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/intrinsic/mod.rs:840: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/type_of.rs:212: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/type_of.rs:258: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/type_of.rs:284: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/callee.rs:35: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/callee.rs:66: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/callee.rs:81: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/callee.rs:86: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/abi.rs:14: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/abi.rs:90: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/lib.rs:2: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/lib.rs:3: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/lib.rs:5: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/lib.rs:6: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/lib.rs:7: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/lib.rs:157: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/lib.rs:169: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/lib.rs:207: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/lib.rs:255: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/lib.rs:328: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/type_.rs:29: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/type_.rs:152: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/type_.rs:154: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/type_.rs:170: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/type_.rs:176: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/type_.rs:184: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/type_.rs:220: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/type_.rs:266: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/type_.rs:274: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/declare.rs:85: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/declare.rs:150: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/declare.rs:155: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/declare.rs:183: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/declare.rs:190: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/declare.rs:193: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/declare.rs:203: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/common.rs:30: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/common.rs:42: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/common.rs:65: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/common.rs:198: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/common.rs:212: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/common.rs:218: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/common.rs:264: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/asm.rs:18: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/asm.rs:256: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/asm.rs:299: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/asm.rs:305: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/asm.rs:393: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/asm.rs:398: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/asm.rs:407: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/asm.rs:408: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/context.rs:45: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/context.rs:108: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/context.rs:137: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/context.rs:147: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/context.rs:152: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/context.rs:280: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/context.rs:282: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/context.rs:283: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/context.rs:284: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/context.rs:305: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/context.rs:383: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/context.rs:388: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/coverageinfo.rs:55: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/coverageinfo.rs:75: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/coverageinfo.rs:94: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_codegen_gcc/src/coverageinfo.rs:101: TODO is deprecated; use FIXME
Found 0 error codes with no tests
Done!
Done!
tidy error: /checkout/compiler/rustc_codegen_gcc/build_sysroot/Cargo.toml doesn't have `edition = "2018"` on a separate line
some tidy checks failed



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rust-tidy" "/checkout" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build" "16"


Build completed unsuccessfully in 0:00:10
