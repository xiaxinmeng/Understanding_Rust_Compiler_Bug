plain
######################                                                    30.8%
##############################################################            86.9%
######################################################################## 100.0%
[00:02:19] extracting /checkout/obj/build/cache/2018-09-23/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:02:19] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:02:19] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:02:19] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:02:19] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:02:19]     Updating crates.io index
[00:02:24]  Downloading libc v0.2.43
[00:02:24]  Downloading num_cpus v1.8.0
[00:02:24]  Downloading filetime v0.2.1
[00:02:24]  Downloading lazy_static v0.2.11
---
[00:03:36] travis_fold:end:log-system-info
travis_fold:start:make-tidy
travis_time:start:0d2d571f
make -j 4 tidy
[00:03:36] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:03:36] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:03:36] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:03:36] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:03:38] travis_fold:start:stage0-tidy
travis_time:start:stage0-tidy
Building stage0 tool tidy (x86_64-unknown-linux-gnu)
Building stage0 tool tidy (x86_64-unknown-linux-gnu)
[00:03:38] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:03:38] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:03:38] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:03:38] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:03:38]    Compiling unicode-xid v0.1.0
[00:03:38]    Compiling serde v1.0.75
[00:03:38]    Compiling ryu v0.2.6
[00:03:39]    Compiling itoa v0.4.2
---
[00:04:23] travis_time:end:0d2d571f:start=1537997688916012475,finish=1537997735613435458,duration=46697422983
travis_fold:start:make-all
travis_time:start:330a8e05
make -j 4 all
[00:04:23] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:04:23] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:04:23] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:04:23] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:04:24] travis_fold:start:stage0-std
travis_time:start:stage0-std
Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:04:24] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:04:24] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:04:24] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:04:24] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:04:25]    Compiling core v0.0.0 (/checkout/src/libcore)
[00:04:25]    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
[00:04:25]    Compiling unwind v0.0.0 (/checkout/src/libunwind)
[00:04:31]    Compiling compiler_builtins v0.0.0 (/checkout/src/rustc/compiler_builtins_shim)
---

[00:05:08] travis_fold:start:stage0-test
travis_time:start:stage0-test
Building stage0 test artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:05:08] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:05:08] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:05:08] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:05:08] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:05:08]    Compiling term v0.0.0 (/checkout/src/libterm)
[00:05:11]    Compiling test v0.0.0 (/checkout/src/libtest)
[00:05:17]     Finished release [optimized] target(s) in 8.99s
[00:05:17] Copying stage0 test from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[00:05:17] Copying stage0 test from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[00:05:17] travis_fold:end:stage0-test

[00:05:17] travis_time:end:stage0-test:start=1537997780423950165,finish=1537997789435903131,duration=9011952966

[00:05:17] travis_fold:start:stage0-rustc
travis_time:start:stage0-rustc
Building stage0 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:05:17] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:05:17] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:05:17] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:05:17] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:05:17]    Compiling cfg-if v0.1.5
[00:05:17]    Compiling nodrop v0.1.12
[00:05:17]    Compiling memoffset v0.2.1
[00:05:17]    Compiling scopeguard v0.3.3
---

[00:18:31] travis_time:end:stage0-rustc:start=1537997789436593359,finish=1537998583277520633,duration=793840927274

[00:18:31] Building stage0 codegen artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu, llvm)
[00:18:31] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:18:31] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:18:31] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:18:31] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:18:31]    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
[00:18:31]    Compiling rustc_codegen_llvm v0.0.0 (/checkout/src/librustc_codegen_llvm)
[00:18:31]    Compiling libc v0.2.43
[00:18:32]    Compiling rustc-demangle v0.1.9
[00:18:34]    Compiling num_cpus v1.8.0
---
[00:19:32] Assembling stage1 compiler (x86_64-unknown-linux-gnu)
[00:19:32] travis_fold:start:stage1-std
travis_time:start:stage1-std
Building stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:19:32] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:19:32] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:19:32] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:19:32] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:19:33]    Compiling core v0.0.0 (/checkout/src/libcore)
[00:19:33]    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
[00:19:33]    Compiling unwind v0.0.0 (/checkout/src/libunwind)
[00:19:39]    Compiling compiler_builtins v0.0.0 (/checkout/src/rustc/compiler_builtins_shim)
---

[00:20:39] travis_fold:start:stage1-test
travis_time:start:stage1-test
Building stage1 test artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:20:39] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:20:39] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:20:39] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:20:39] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:20:39]    Compiling term v0.0.0 (/checkout/src/libterm)
[00:20:43]    Compiling test v0.0.0 (/checkout/src/libtest)
[00:20:49]     Finished release [optimized] target(s) in 10.04s
[00:20:49] Copying stage1 test from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[00:20:49] travis_fold:end:stage1-test
[00:20:49] travis_fold:end:stage1-test

[00:20:49] travis_time:end:stage1-test:start=1537998711566492351,finish=1537998721630706286,duration=10064213935

[00:20:49] travis_fold:start:stage1-rustc
travis_time:start:stage1-rustc
Building stage1 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:20:49] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:20:49] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:20:49] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:20:49] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:20:49]    Compiling cfg-if v0.1.5
[00:20:49]    Compiling nodrop v0.1.12
[00:20:49]    Compiling void v1.0.2
[00:20:50]    Compiling scopeguard v0.3.3
---

[00:35:29] travis_time:end:stage1-rustc:start=1537998721631585748,finish=1537999601045671513,duration=879414085765

[00:35:29] Building stage1 codegen artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu, llvm)
[00:35:29] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:35:29] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:35:29] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:35:29] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:35:29]    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
[00:35:29]    Compiling libc v0.2.43
[00:35:29]    Compiling rustc_codegen_llvm v0.0.0 (/checkout/src/librustc_codegen_llvm)
[00:35:30]    Compiling rustc-demangle v0.1.9
[00:35:31]    Compiling num_cpus v1.8.0
---
[00:36:36] Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[00:36:36] travis_fold:start:stage2-rustdoc
travis_time:start:stage2-rustdoc
Building rustdoc for stage2 (x86_64-unknown-linux-gnu)
[00:36:36] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:36:36] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:36:36] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:36:36] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:36:36]    Compiling rand_core v0.2.1
[00:36:36]    Compiling stable_deref_trait v1.1.0
[00:36:36]    Compiling libc v0.2.43
[00:36:37]    Compiling pulldown-cmark v0.1.2
---

[00:38:11] travis_time:end:stage2-rustdoc:start=1537999668591626168,finish=1537999763289447362,duration=94697821194

[00:38:11] Build completed successfully in 0:33:47
[00:38:11] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:38:11] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:38:11] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:38:11] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:38:12] travis_fold:start:stage0-std
travis_time:start:stage0-std
Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:38:12] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:38:12] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:38:12] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:38:12] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:38:12] Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[00:38:12] travis_fold:end:stage0-std

[00:38:12] travis_time:end:stage0-std:start=1537999764592653382,finish=1537999764861552653,duration=268899271
[00:38:12] travis_time:end:stage0-std:start=1537999764592653382,finish=1537999764861552653,duration=268899271

[00:38:12] travis_fold:start:stage0-test
travis_time:start:stage0-test
Building stage0 test artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:38:12] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:38:12] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:38:12] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:38:12] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:38:13] Copying stage0 test from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[00:38:13] travis_fold:end:stage0-test

[00:38:13] travis_time:end:stage0-test:start=1537999764862185444,finish=1537999765110814699,duration=248629255
[00:38:13] travis_time:end:stage0-test:start=1537999764862185444,finish=1537999765110814699,duration=248629255

[00:38:13] travis_fold:start:stage0-rustc
travis_time:start:stage0-rustc
Building stage0 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:38:13] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:38:13] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:38:13] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:38:13] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:38:13] Copying stage0 rustc from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[00:38:13] travis_fold:end:stage0-rustc

[00:38:13] travis_time:end:stage0-rustc:start=1537999765111561543,finish=1537999765420732026,duration=309170483
[00:38:13] travis_time:end:stage0-rustc:start=1537999765111561543,finish=1537999765420732026,duration=309170483

[00:38:13] Building stage0 codegen artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu, llvm)
[00:38:13] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:38:13] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:38:13] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:38:13] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:38:13] travis_fold:start:stage0-rustc_codegen_llvm
travis_time:start:stage0-rustc_codegen_llvm
travis_fold:end:stage0-rustc_codegen_llvm


[00:38:13] travis_time:end:stage0-rustc_codegen_llvm:start=1537999765430622728,finish=1537999765689569284,duration=258946556

[00:38:13] Assembling stage1 compiler (x86_64-unknown-linux-gnu)
[00:38:13] travis_fold:start:stage1-std
travis_time:start:stage1-std
Building stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:38:13] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:38:13] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:38:13] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:38:13] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:38:13] Copying stage1 std from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[00:38:13] travis_fold:end:stage1-std

[00:38:13] travis_time:end:stage1-std:start=1537999765691750140,finish=1537999765964002268,duration=272252128
[00:38:13] travis_time:end:stage1-std:start=1537999765691750140,finish=1537999765964002268,duration=272252128

[00:38:13] travis_fold:start:stage1-test
travis_time:start:stage1-test
Building stage1 test artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:38:13] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:38:13] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:38:13] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:38:13] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:38:14] Copying stage1 test from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[00:38:14] travis_fold:end:stage1-test

[00:38:14] travis_time:end:stage1-test:start=1537999765964606369,finish=1537999766213364185,duration=248757816
[00:38:14] travis_time:end:stage1-test:start=1537999765964606369,finish=1537999766213364185,duration=248757816

[00:38:14] travis_fold:start:stage1-rustc
travis_time:start:stage1-rustc
Building stage1 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:38:14] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:38:14] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:38:14] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:38:14] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:38:14] Copying stage1 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[00:38:14] travis_fold:end:stage1-rustc

[00:38:14] travis_time:end:stage1-rustc:start=1537999766214039996,finish=1537999766531887816,duration=317847820
[00:38:14] travis_time:end:stage1-rustc:start=1537999766214039996,finish=1537999766531887816,duration=317847820

[00:38:14] Building stage1 codegen artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu, llvm)
[00:38:14] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:38:14] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:38:14] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:38:14] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:38:14] travis_fold:start:stage1-rustc_codegen_llvm
travis_time:start:stage1-rustc_codegen_llvm
travis_fold:end:stage1-rustc_codegen_llvm

---
[00:38:14] Generating unstable book md files (x86_64-unknown-linux-gnu)
[00:38:14] travis_fold:start:stage0-unstable-book-gen
travis_time:start:stage0-unstable-book-gen
Building stage0 tool unstable-book-gen (x86_64-unknown-linux-gnu)
[00:38:14] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:38:14] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:38:14] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:38:14] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:38:17]    Compiling unstable-book-gen v0.1.0 (/checkout/src/tools/unstable-book-gen)
[00:38:19]     Finished release [optimized] target(s) in 4.28s
[00:38:19] travis_fold:end:stage0-unstable-book-gen


[00:38:19] travis_time:end:stage0-unstable-book-gen:start=1537999766792982640,finish=1537999771097651134,duration=4304668494

[00:38:19] travis_fold:start:stage0-rustbook
travis_time:start:stage0-rustbook
Building stage0 tool rustbook (x86_64-unknown-linux-gnu)
[00:38:19] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:38:19] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:38:19] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:38:19] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:38:19]    Compiling libc v0.2.43
[00:38:19]    Compiling version_check v0.1.4
[00:38:19]    Compiling string_cache_shared v0.3.0
[00:38:19]    Compiling cc v1.0.25
---
[00:42:00] Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[00:42:00] travis_fold:start:stage2-rustdoc
travis_time:start:stage2-rustdoc
Building rustdoc for stage2 (x86_64-unknown-linux-gnu)
[00:42:00] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:42:00] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:42:00] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:42:00] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:42:00] travis_fold:end:stage2-rustdoc

[00:42:00] travis_time:end:stage2-rustdoc:start=1537999992261700087,finish=1537999992534185841,duration=272485754


[00:42:00] Documenting book index (x86_64-unknown-linux-gnu)
[00:42:00] Documenting book redirect pages (x86_64-unknown-linux-gnu)
[00:42:02] Documenting stage2 std (x86_64-unknown-linux-gnu)
[00:42:02] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:42:02] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:42:02] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:42:02] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:42:02]     Checking core v0.0.0 (/checkout/src/libcore)
[00:42:02]  Documenting core v0.0.0 (/checkout/src/libcore)
[00:42:02]    Compiling std v0.0.0 (/checkout/src/libstd)
[00:42:36]     Checking compiler_builtins v0.0.0 (/checkout/src/rustc/compiler_builtins_shim)
[00:42:37]     Checking libc v0.0.0 (/checkout/src/rustc/libc_shim)
[00:42:37]     Checking alloc v0.0.0 (/checkout/src/liballoc)
[00:42:37]  Documenting alloc v0.0.0 (/checkout/src/liballoc)
[00:42:38]     Checking unwind v0.0.0 (/checkout/src/libunwind)
[00:42:38]     Checking alloc_system v0.0.0 (/checkout/src/liballoc_system)
[00:42:38]     Checking panic_abort v0.0.0 (/checkout/src/libpanic_abort)
[00:42:38]     Checking alloc_jemalloc v0.0.0 (/checkout/src/liballoc_jemalloc)
[00:42:42]     Checking rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[00:42:42]     Checking rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
[00:42:42]     Checking rustc_msan v0.0.0 (/checkout/src/librustc_msan)
[00:42:42]     Checking rustc_asan v0.0.0 (/checkout/src/librustc_asan)
[00:42:43]     Checking panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:42:43]  Documenting std v0.0.0 (/checkout/src/libstd)
[00:42:48]     --> /checkout/src/liballoc/rc.rs:1180:29
[00:42:48]      |
[00:42:48] 1180 |     /// Calling [`upgrade`][Weak::upgrade] on the return value always gives [`None`].
[00:42:48]      |                             ^^^^^^^^^^^^^ cannot be resolved, ignoring
---
[00:42:48]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[00:42:48] 
[00:42:52]     Finished release [optimized] target(s) in 50.50s
[00:42:52] Documenting stage2 test (x86_64-unknown-linux-gnu)
[00:42:52] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:42:52] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:42:52] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:42:52] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:42:53]     Checking term v0.0.0 (/checkout/src/libterm)
[00:42:53]  Documenting test v0.0.0 (/checkout/src/libtest)
[00:42:55]     Finished release [optimized] target(s) in 2.58s
[00:42:55] Documenting stage2 whitelisted compiler (x86_64-unknown-linux-gnu)
[00:42:55] Documenting stage2 whitelisted compiler (x86_64-unknown-linux-gnu)
[00:42:55] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:42:55] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:42:55] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:42:55] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:42:55]     Checking nodrop v0.1.12
[00:42:55]     Checking void v1.0.2
[00:42:55]     Checking memoffset v0.2.1
[00:42:56]     Checking scopeguard v0.3.3
---
[00:43:28] Documenting error index (x86_64-unknown-linux-gnu)
[00:43:28] travis_fold:start:stage2-error_index_generator
travis_time:start:stage2-error_index_generator
Building stage2 tool error_index_generator (x86_64-unknown-linux-gnu)
[00:43:28] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:43:28] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:43:28] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:43:28] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:43:28]    Compiling error_index_generator v0.0.0 (/checkout/src/tools/error_index_generator)
[00:43:31] travis_fold:end:stage2-error_index_generator

[00:43:31] travis_time:end:stage2-error_index_generator:start=1538000080647785247,finish=1538000083433754052,duration=2785968805

---
[00:43:33] travis_time:end:330a8e05:start=1537997735615224402,finish=1538000085872198932,duration=2350256974530
travis_fold:start:make-check
travis_time:start:02deb37a
make -j 4 check
[00:43:33] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:43:33] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:43:33] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:43:33] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:43:35] travis_fold:start:stage0-tidy
travis_time:start:stage0-tidy
Building stage0 tool tidy (x86_64-unknown-linux-gnu)
Building stage0 tool tidy (x86_64-unknown-linux-gnu)
[00:43:35] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:43:35] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:43:35] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:43:35] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:43:35] travis_fold:end:stage0-tidy

[00:43:35] travis_time:end:stage0-tidy:start=1538000087465522825,finish=1538000087717204207,duration=251681382

---

[00:43:37] travis_fold:start:stage0-std
travis_time:start:stage0-std
Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:43:37] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:43:37] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:43:37] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:43:37] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:43:37] Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[00:43:37] travis_fold:end:stage0-std

[00:43:37] travis_time:end:stage0-std:start=1538000089369491644,finish=1538000089631720736,duration=262229092
[00:43:37] travis_time:end:stage0-std:start=1538000089369491644,finish=1538000089631720736,duration=262229092

[00:43:37] travis_fold:start:stage0-test
travis_time:start:stage0-test
Building stage0 test artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:43:37] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:43:37] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:43:37] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:43:37] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:43:37] Copying stage0 test from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[00:43:37] travis_fold:end:stage0-test

[00:43:37] travis_time:end:stage0-test:start=1538000089632237899,finish=1538000089875654141,duration=243416242
[00:43:37] travis_time:end:stage0-test:start=1538000089632237899,finish=1538000089875654141,duration=243416242

[00:43:37] travis_fold:start:stage0-rustc
travis_time:start:stage0-rustc
Building stage0 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:43:37] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:43:37] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:43:37] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:43:37] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:43:38] Copying stage0 rustc from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[00:43:38] travis_fold:end:stage0-rustc

[00:43:38] travis_time:end:stage0-rustc:start=1538000089876313427,finish=1538000090186911084,duration=310597657
[00:43:38] travis_time:end:stage0-rustc:start=1538000089876313427,finish=1538000090186911084,duration=310597657

[00:43:38] Building stage0 codegen artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu, llvm)
[00:43:38] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:43:38] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:43:38] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:43:38] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:43:38] travis_fold:start:stage0-rustc_codegen_llvm
travis_time:start:stage0-rustc_codegen_llvm
travis_fold:end:stage0-rustc_codegen_llvm


[00:43:38] travis_time:end:stage0-rustc_codegen_llvm:start=1538000090196607329,finish=1538000090455954430,duration=259347101

[00:43:38] Assembling stage1 compiler (x86_64-unknown-linux-gnu)
[00:43:38] travis_fold:start:stage1-std
travis_time:start:stage1-std
Building stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:43:38] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:43:38] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:43:38] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:43:38] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:43:38] Copying stage1 std from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[00:43:38] travis_fold:end:stage1-std

[00:43:38] travis_time:end:stage1-std:start=1538000090457913297,finish=1538000090726302279,duration=268388982
[00:43:38] travis_time:end:stage1-std:start=1538000090457913297,finish=1538000090726302279,duration=268388982

[00:43:38] travis_fold:start:stage1-test
travis_time:start:stage1-test
Building stage1 test artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:43:38] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:43:38] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:43:38] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:43:38] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:43:38] Copying stage1 test from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[00:43:38] travis_fold:end:stage1-test

[00:43:38] travis_time:end:stage1-test:start=1538000090726984078,finish=1538000090975532314,duration=248548236
[00:43:38] travis_time:end:stage1-test:start=1538000090726984078,finish=1538000090975532314,duration=248548236

[00:43:38] travis_fold:start:stage1-rustc
travis_time:start:stage1-rustc
Building stage1 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:43:38] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:43:38] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:43:38] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:43:38] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:43:39] Copying stage1 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[00:43:39] travis_fold:end:stage1-rustc

[00:43:39] travis_time:end:stage1-rustc:start=1538000090976180027,finish=1538000091280602100,duration=304422073
[00:43:39] travis_time:end:stage1-rustc:start=1538000090976180027,finish=1538000091280602100,duration=304422073

[00:43:39] Building stage1 codegen artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu, llvm)
[00:43:39] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:43:39] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:43:39] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:43:39] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:43:39] travis_fold:start:stage1-rustc_codegen_llvm
travis_time:start:stage1-rustc_codegen_llvm
travis_fold:end:stage1-rustc_codegen_llvm

---

[00:43:39] travis_fold:start:stage0-compiletest
travis_time:start:stage0-compiletest
Building stage0 tool compiletest (x86_64-unknown-linux-gnu)
[00:43:39] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:43:39] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:43:39] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:43:39] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:43:39]    Compiling diff v0.1.11
[00:43:39]    Compiling filetime v0.2.1
[00:43:39]    Compiling env_logger v0.5.12
[00:43:41]    Compiling syn v0.14.9
---
[00:45:29] .................................................................................................... 1900/6843
[00:45:31] .................................................................................................... 2000/6843
[00:45:34] .................................................................................................... 2100/6843
[00:45:37] .................................................................................................... 2200/6843
[00:45:40] ..........................................................................F......................... 2300/6843
[00:45:46] .................................................................................................... 2500/6843
[00:45:49] .................................................................................................... 2600/6843
[00:45:49] .................................................................................................... 2600/6843
[00:45:53] ......................................FF.......................................................i.... 2700/6843
[00:45:58] .......................................................i.i..ii...................................... 2900/6843
[00:46:02] .................................................................................................... 3000/6843
[00:46:05] .......................................................................................i............ 3100/6843
[00:46:08] .................................................................................................... 3200/6843
---
[00:50:57] ............................i....................................................................... 6500/6843
[00:51:01] .................................................................................................... 6600/6843
[00:51:04] .................................................................................................... 6700/6843
[00:51:06] ..................................................................................i................. 6800/6843
refer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-42755/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-42755/auxiliary" "-A" "unused"
[00:51:07] ------------------------------------------
[00:51:07] 
[00:51:07] ------------------------------------------
[00:51:07] stderr:
[00:51:07] stderr:
[00:51:07] ------------------------------------------
[00:51:07] {"message":"repetition matches empty token tree","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-42755.rs","byte_start":493,"byte_end":501,"line_start":13,"line_end":13,"column_start":7,"column_end":15,"is_primary":true,"text":[{"text":"    ($($p:vis)*) => {} //~ ERROR repetition matches empty token tree","highlight_start":7,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: repetition matches empty token tree\n  --> /checkout/src/test/ui/issues/issue-42755.rs:13:7\n   |\nLL |     ($($p:vis)*) => {} //~ ERROR repetition matches empty token tree\n   |       ^^^^^^^^\n\n"}
[00:51:07] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:51:07] ------------------------------------------
[00:51:07] 
[00:51:07] thread '[ui] ui/issues/issue-42755.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3238:9
[00:51:07] thread '[ui] ui/issues/issue-42755.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3238:9
[00:51:07] note: Run with ` 34    = help: or consider exporting it for use by other crates
[00:51:07] 35 
[00:51:07] 36 warning: unreachable `pub` field
[00:51:07] +   --> $DIR/unreachable_pub-pub_crate.rs:30:9
[00:51:07] 38    |
[00:51:07] 38    |
[00:51:07] 39 LL |         pub neutrons: usize,
[00:51:07] 40    |         ---^^^^^^^^^^^^^^^^
[00:51:07] 
[00:51:07] 42    |         help: consider restricting its visibility: `pub(crate)`
[00:51:07] 43 
[00:51:07] 44 warning: unreachable `pub` item
[00:51:07] +   --> $DIR/unreachable_pub-pub_crate.rs:36:9
[00:51:07] 46    |
[00:51:07] 46    |
[00:51:07] 47 LL |         pub fn count_neutrons(&self) -> usize { self.neutrons }
[00:51:07] 48    |         ---^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:51:07] 
[00:51:07] 50    |         help: consider restricting its visibility: `pub(crate)`
[00:51:07] 51 
[00:51:07] 52 warning: unreachable `pub` item
[00:51:07] +   --> $DIR/unreachable_pub-pub_crate.rs:40:5
[00:51:07] 54    |
[00:51:07] 54    |
[00:51:07] 55 LL |     pub enum Helium {}
[00:51:07] 56    |     ---^^^^^^^^^^^^
[00:51:07] 
[00:51:07] 60    = help: or consider exporting it for use by other crates
[00:51:07] 61 
[00:51:07] 62 warning: unreachable `pub` item
[00:51:07] +   --> $DIR/unreachable_pub-pub_crate.rs:41:5
[00:51:07] 64    |
[00:51:07] 64    |
[00:51:07] 65 LL |     pub union Lithium { c1: usize, c2: u8 }
[00:51:07] 66    |     ---^^^^^^^^^^^^^^
[00:51:07] 
[00:51:07] 70    = help: or consider exporting it foommand: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/unreachable_pub-pub_crate.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unreachable_pub-pub_crate/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unreachable_pub-pub_crate/auxiliary" "-A" "unused"
[00:51:07] ------------------------------------------
[00:51:07] 
[00:51:07] ------------------------------------------
[00:51:07] stderr:
[00:51:07] stderr:
[00:51:07] ------------------------------------------
[00:51:07] {"message":"unreachable `pub` item","code":{"code":"unreachable_pub","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/lint/unreachable_pub-pub_crate.rs","byte_start":948,"byte_end":965,"line_start":25,"line_end":25,"column_start":5,"column_end":22,"is_primary":true,"text":[{"text":"    pub use std::fmt;","highlight_start":5,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"lint level defined here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/lint/unreachable_pub-pub_crate.rs","byte_start":842,"byte_end":857,"line_start":21,"line_end":21,"column_start":9,"column_end":24,"is_primary":true,"text":[{"text":"#![warn(unreachable_pub)]","highlight_start":9,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"or consider exporting it for use by other crates","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"consider restricting its visibility","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/lint/unreachable_pub-pub_crate.rs","byte_start":948,"byte_end":951,"line_start":25,"line_end":25,"column_start":5,"column_end":8,"is_primary":true,"text":[{"text":"    pub use std::fmt;","highlight_start":5,"highlight_end":8}],"label":null,"suggested_replacement":"pub(crate)","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unreachable `pub` item\n  --> /checkout/src/test/ui/lint/unreachable_pub-pub_crate.rs:25:5\n   |\nLL |     pub use std::fmt;\n   |     ---^^^^^^^^^^^^^^\n   |     |\n   |     help: consider restricting its visibility: `pub(crate)`\n   |\nnote: lint level defined here\n  --> /checkout/src/test/ui/lint/unreachable_pub-pub_crate.rs:21:9\n   |\nLL | #![warn(unreachable_pub)]\n   |         ^^^^^^^^^^^^^^^\n   = help: or consider exporting it for use by other crates\n\n"}
[00:51:07] {"message":"unreachable `pub` item","code":{"code":"unreachable_pub","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/lint/unreachable_pub-pub_crate.rs","byte_start":989,"byte_end":993,"line_start":26,"line_end":26,"column_start":24,"column_end":28,"is_primary":true,"text":[{"text":"    pub use std::env::{Args}; // braced-use has different item spans than unbraced","highlight_start":24,"highlight_end":28_start":5,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"or consider exporting it for use by other crates","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"consider restricting its visibility","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/lint/unreachable_pub-pub_crate.rs","byte_start":1054,"byte_end":1057,"line_start":28,"line_end":28,"column_start":5,"column_end":8,"is_primary":true,"text":[{"text":"    pub struct Hydrogen {","highlight_start":5,"highlight_end":8}],"label":null,"suggested_replacement":"pub(crate)","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unreachable `pub` item\n  --> /checkout/src/test/ui/lint/unreachable_pub-pub_crate.rs:28:5\n   |\nLL |     pub struct Hydrogen {\n   |     ---^^^^^^^^^^^^^^^^\n   |     |\n   |     help: consider restricting its visibility: `pub(crate)`\n   |\n   = help: or consider exporting it for use by other crates\n\n"}
[00:51:07] {"message":"unreachable `pub` field","code":{"code":"unreachable_pub","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/lint/unreachable_pub-pub_crate.rs","byte_start":1120,"byte_end":1139,"line_start":30,"line_end":30,"column_start":9,"column_end":28,"is_primary":true,"text":[{"text":"        pub neutrons: usize,","highlight_start":9,"highlight_end":28}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider restricting its visibiling: unreachable `pub` item\n  --> /checkout/src/test/ui/lint/unreachable_pub-pub_crate.rs:40:5\n   |\nLL |     pub enum Helium {}\n   |     ---^^^^^^^^^^^^\n   |     |\n   |     help: consider restricting its visibility: `pub(crate)`\n   |\n   = help: or consider exporting it for use by other crates\n\n"}
[00:51:07] {"message":"unreachable `pub` item","code":{"code":"unreachable_pub","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/lint/unreachable_pub-pub_crate.rs","byte_start":1444,"byte_end":1461,"line_start":41,"line_end":41,"column_start":5,"column_end":22,"is_primary":true,"text":[{"text":"    pub union Lithium { c1: usize, c2: u8 }","highlight_start":5,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"or consider exporting it for use by other crates","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"consider restricting its visibility","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/lint/unreachable_pub-pub_crate.rs","byte_start":1444,"byte_end":1447,"line_start":41,"line_end":41,"column_start":5,"column_end":8,"is_primary":true,"text":[{"text":"    pub union Lithium { c1: usize, c2: u8 }","highlight_start":5,"highlight_end":8}],"label":null,"suggested_replacement":"pub(crate)","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unreachable `pub` item\n  --> /checkout/src/test/ui/lint/unreachable_pub-pub_crate.rs:41:5\n   |\nLL |     pub union Lithium { c1: usize, c2: u8 }\n   |     ---^^^^^^^^^^^^^^\n   |     |\n   |     help: consider restricting its visibility: `pub(crate)`\n   |\n   = help: or consider exporting it for use by other crates\n\n"}
[00:51:07] {"message":"unreachable `pub` item","code":{"code":"unreachable_pub","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/lint/unreachable_pub-pub_crate.rs","byte_start":1488,"byte_end":1506,"line_start":42,"line_end":42,"column_start":5,"column_end":23,"is_primary":true,"text":[{"text":"    pub fn beryllium() {}","highlight_start":5,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"or consider exporting it for use by other crates","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"consider restricting its visibility","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/lint/unreachable_pub-pub_crate.rs","byte_start":1488,"byte_end":1491,"line_start":42,"line_end":42,"column_start":5,"column_end":8,"is_primary":true,"text":[{"text":"    pub fn beryllium() {}","highlight_start":5,"highlight_end":8}],"label":null,"suggested_replacement":"pub(crate)","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unreachable `pub` item\n  --> /checkout/src/test/ui/lint/unreachable_pub-pub_crate.rs:42:5\n   |\nLL |     pub fn beryllium() {}\n   |     ---^^^^^^^^^^^^^^^\n   |     |\n   |     help: consider restricting its visibility: `pub(crate)`\n   |\n   = help: or consider exporting it for use by other crates\n\n"}
[00:51:07] ;","highlight_start":5,"highlight_end":36}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"or consider exporting it for use by other crates","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"consider restricting its visibility","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/lint/unreachable_pub-pub_crate.rs","byte_start":1570,"byte_end":1573,"line_start":45,"line_end":45,"column_start":5,"column_end":8,"is_primary":true,"text":[{"text":"    pub static NITROGEN: usize = 2;","highlight_start":5,"highlight_end":8}],"label":null,"suggested_replacement":"pub(crate)","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unreachable `pub` item\n  --> /checkout/src/test/ui/lint/unreachable_pub-pub_crate.rs:45:5\n   |\nLL |     pub static NITROGEN: usize = 2;\n   |     ---^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |     |\n   |     help: consider restricting its visibility: `pub(crate)`\n   |\n   = help: or consider exporting it for use by other crates\n\n"}
[00:51:07] {"message":"unreachable `pub` item","code":{"code":"unreachable_pub","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/lint/unreachable_pub-pub_crate.rs","byte_start":1606,"byte_end":1629,"line_start":46,"line_end":46,"column_start":5,"column_end":28,"is_primary":true,"text":[{"text":"    pub type Oxygen = bool;","highlight_start":5,"highlight_end":28}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"lumn_start":5,"column_end":57,"is_primary":false,"text":[{"text":"    define_empty_struct_with_visibility!(pub, Fluorine);","highlight_start":5,"highlight_end":57}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"define_empty_struct_with_visibility!","def_site_span":{"file_name":"/checkout/src/test/ui/lint/unreachable_pub-pub_crate.rs","byte_start":1635,"byte_end":1767,"line_start":48,"line_end":50,"column_start":5,"column_end":6,"is_primary":false,"text":[{"text":"    macro_rules! define_empty_struct_with_visibility {","highlight_start":5,"highlight_end":55},{"text":"        ($visibility: vis, $name: ident) => { $visibility struct $name {} }","highlight_start":1,"highlight_end":76},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"or consider exporting it for use by other crates","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"consider restricting its visibility","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/lint/unreachable_pub-pub_crate.rs","byte_start":1809,"byte_end":1812,"line_start":51,"line_end":51,"column_start":42,"column_end":45,"is_primary":true,"text":[{"text":"    define_empty_struct_with_visibility!(pub, Fluorine);","highlight_start":42,"highlight_end":45}],"label":null,"suggested_replacement":"pub(crate)","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unreachable `pub` item\n  --> /checkout/src/test/[00:51:07] 18    |
[00:51:07] 19 LL |     pub use std::env::{Args}; // braced-use has different item spans than unbraced
[00:51:07] 
[00:51:07] 
[00:51:07] 24    = help: or consider exporting it for use by other crates
[00:51:07] 25 
[00:51:07] 26 warning: unreachable `pub` item
[00:51:07] +   --> $DIR/unreachable_pub.rs:23:5
[00:51:07] 28    |
[00:51:07] 28    |
[00:51:07] 29 LL |     pub struct Hydrogen {
[00:51:07] 30    |     ---^^^^^^^^^^^^^^^^
[00:51:07] 
[00:51:07] 34    = help: or consider exporting it for use by other crates
[00:51:07] 35 
[00:51:07] 36 warning: unreachable `pub` field
[00:51:07] +   --> $DIR/unreachable_pub.rs:25:9
[00:51:07] 38    |
[00:51:07] 38    |
[00:51:07] 39 LL |         pub neutrons: usize,
[00:51:07] 40    |         ---^^^^^^^^^^^^^^^^
[00:51:07] 
[00:51:07] 42    |         help: consider restricting its visibility: `crate`
[00:51:07] 43 
[00:51:07] 44 warning: unreachable `pub` item
[00:51:07] +   --> $DIR/unreachable_pub.rs:31:9
[00:51:07] 46    |
[00:51:07] 46    |
[00:51:07] 47 LL |         pub fn count_neutrons(&self) -> usize { self.neutrons }
[00:51:07] 48    |         ---^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:51:07] 
[00:51:07] 50    |         help: consider restricting its visibility: `crate`
[00:51:07] 51 
[00:51:07] 52 warning: unreachable `pub` item
[00:51:07] +   --> $DIR/unreachable_pub.rs:35:5
[00:51:07] 54    |
[00:51:07] 54    |
[00:51:07] 55 LL |     pub enum Helium {}
[00:51:07] 56    |     ---^^^^^^^^^^^^
[00:51:07] 
[00:51:07] 60    = help: or consider exporting it for use by other crates
[00:51:07] 61 
[00:51:07] 62 warning: unreachable `pub` item
[00:51:07] +   --> $DIR/unreachable_pub.rs:36:5
[00:51:07] 64    |
[00:51:07] 64    |
[00:51:07] 65 LL |     pub union Lithium { c1: usize, c2: u8 }
[00:51:07] 66    |     ---^^^^^^^^^^^^^^
[00:51:07] 
[00:51:07] 70    = help: or consider exporting it for use by other crates
[00:51:07] 71 
[00:51:07] 72 warning: unreachable `pub` item
[00:51:07] +   --> $DIR/unreachable_pub.rs:37:5
[00:51:07] 74    |
[00:51:07] 74    |
[00:51:07] 75 LL |     pub fn beryllium() {}
[00:51:07] 76    |     ---^^^^^^^^^^^^^^^
[00:51:07] 
[00:51:07] 80    = help: or consider exporting it for use by other crates
[00:51:07] 81 
[00:51:07] 82 warning: unreachable `pub` item
[00:51:07] +   --> $DIR/unreachable_pub.rs:38:5
[00:51:07] 84    |
[00:51:07] 84    |
[00:51:07] 85 LL |     pub trait Boron {}
[00:51:07] 86    |     ---^^^^^^^^^^^^
[00:51:07] 
[00:51:07] 90    = help: or consider exporting it for use by other crates
[00:51:07] 91 
[00:51:07] 92 warning: unreachable `pub` item
[00:51:07] +   --> $DIR/unreachable_pub.rs:39:5
[00:51:07] 94    |
[00:51:07] 94    |
[00:51:07] 95 LL |     pub const CARBON: usize = 1;
[00:51:07] 96    |     ---^^^^^^^^^^^^^^^^^^^^^^^^^
[00:51:07] 
[00:51:07] 100    = help: or consider exporting it for use by other crates
[00:51:07] 101 
[00:51:07] 102 warning: unreachable `pub` item
[00:51:07] -   -","byte_start":550,"byte_end":565,"line_start":16,"line_end":16,"column_start":9,"column_end":24,"is_primary":true,"text":[{"text":"#![warn(unreachable_pub)]","highlight_start":9,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"or consider exporting it for use by other crates","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"consider restricting its visibility","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/lint/unreachable_pub.rs","byte_start":656,"byte_end":659,"line_start":20,"line_end":20,"column_start":5,"column_end":8,"is_primary":true,"text":[{"text":"    pub use std::fmt;","highlight_start":5,"highlight_end":8}],"label":null,"suggested_replacement":"crate","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unreachable `pub` item\n  --> /checkout/src/test/ui/lint/unreachable_pub.rs:20:5\n   |\nLL |     pub use std::fmt;\n   |     ---^^^^^^^^^^^^^^\n   |     |\n   |     help: consider restricting its visibility: `crate`\n   |\nnote: lint level defined here\n  --> /checkout/src/test/ui/lint/unreachable_pub.rs:16:9\n   |\nLL | #![warn(unreachable_pub)]\n   |         ^^^^^^^^^^^^^^^\n   = help: or consider exporting it for use by other crates\n\n"}
[00:51:07] {"message":"unreachable `pub` item","code":{"code":"unreachable_pub","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/lint/unreachable_pub.rs","byte_start":697,"byte_end":701,"line_start":21,"line_end":21,"text":[{"text":"    pub struct Hydrogen {","highlight_start":5,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"or consider exporting it for use by other crates","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"consider restricting its visibility","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/lint/unreachable_pub.rs","byte_start":762,"byte_end":765,"line_start":23,"line_end":23,"column_start":5,"column_end":8,"is_primary":true,"text":[{"text":"    pub struct Hydrogen {","highlight_start":5,"highlight_end":8}],"label":null,"suggested_replacement":"crate","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unreachable `pub` item\n  --> /checkout/src/test/ui/lint/unreachable_pub.rs:23:5\n   |\nLL |     pub struct Hydrogen {\n   |     ---^^^^^^^^^^^^^^^^\n   |     |\n   |     help: consider restricting its visibility: `crate`\n   |\n   = help: or consider exporting it for use by other crates\n\n"}
[00:51:07] {"message":"unreachable `pub` field","code":{"code":"unreachable_pub","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/lint/unreachable_pub.rs","byte_start":828,"byte_end":847,"line_start":25,"line_end":25,"column_start":9,"column_end":28,"is_primary":true,"text":[{"text":"        pub neutrons: usize,","highlight_start":9,"highlight_end":28}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider restricting ifor use by other crates\n\n"}
[00:51:07] {"message":"unreachable `pub` item","code":{"code":"unreachable_pub","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/lint/unreachable_pub.rs","byte_start":1186,"byte_end":1204,"line_start":37,"line_end":37,"column_start":5,"column_end":23,"is_primary":true,"text":[{"text":"    pub fn beryllium() {}","highlight_start":5,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"or consider exporting it for use by other crates","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"consider restricting its visibility","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/lint/unreachable_pub.rs","byte_start":1186,"byte_end":1189,"line_start":37,"line_end":37,"column_start":5,"column_end":8,"is_primary":true,"text":[{"text":"    pub fn beryllium() {}","highlight_start":5,"highlight_end":8}],"label":null,"suggested_replacement":"crate","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unreachable `pub` item\n  --> /checkout/src/test/ui/lint/unreachable_pub.rs:37:5\n   |\nLL |     pub fn beryllium() {}\n   |     ---^^^^^^^^^^^^^^^\n   |     |\n   |     help: consider restricting its visibility: `crate`\n   |\n   = help: or consider exporting it for use by other crates\n\n"}
[00:51:07] {"message":"unreachable `pub` item","code":{"code":"unreachable_pub","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/lint/unreachable_pub.rs","byte_start":1212,"byte_end":1227,"line_start":38,"line_end":38,"column_start":5,"column_end":20,"is_primary":true,"text":[{"text":"    pub trait Boron {}","highlight_start":5,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"or consider exporting it for use by other crates","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"consider restricting its visibility","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/lint/unreachable_pub.rs","byte_start":1212,"byte_end":1215,"line_start":38,"line_end":38,"column_start":5,"column_end":8,"is_primary":true,"text":[{"text":"    pub trait Boron {}","highlight_start":5,"highlight_end":8}],"label":null,"suggested_replacement":"crate","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unreachable `pub` item\n  --> /checkout/src/test/ui/lint/unreachable_pub.rs:38:5\n   |\nLL |     pub trait Boron {}\n   |     ---^^^^^^^^^^^^\n   |     |\n   |     help: consider restricting its visibility: `crate`\n   |\n   = help: or consider exporting it for use by other crates\n\n"}
[00:51:07] {"message":"unreachable `pub` item","code":{"code":"unreachable_pub","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/lint/unreachable_pub.rs","byte_start":1235,"byte_end":1263,"line_start":39,"line_end":39,"column_start":5,"column_end":33,"is_primary":true,"text":[{"text":"    pub const CARBON: usize = 1;","highlight_start":5,"highlight_end":33}],"label":null,"suggested_replacement":n_start":41,"line_end":41,"column_start":5,"column_end":8,"is_primary":true,"text":[{"text":"    pub type Oxygen = bool;","highlight_start":5,"highlight_end":8}],"label":null,"suggested_replacement":"crate","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unreachable `pub` item\n  --> /checkout/src/test/ui/lint/unreachable_pub.rs:41:5\n   |\nLL |     pub type Oxygen = bool;\n   |     ---^^^^^^^^^^^^^^^^^^^^\n   |     |\n   |     help: consider restricting its visibility: `crate`\n   |\n   = help: or consider exporting it for use by other crates\n\n"}
[00:51:07] {"message":"unreachable `pub` item","code":{"code":"unreachable_pub","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/lint/unreachable_pub.rs","byte_start":1430,"byte_end":1454,"line_start":44,"line_end":44,"column_start":47,"column_end":71,"is_primary":true,"text":[{"text":"        ($visibility: vis, $name: ident) => { $visibility struct $name {} }","highlight_start":47,"highlight_end":71}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/ui/lint/unreachable_pub.rs","byte_start":1470,"byte_end":1522,"line_start":46,"line_end":46,"column_start":5,"column_end":57,"is_primary":false,"text":[{"text":"    define_empty_struct_with_visibility!(pub, Fluorine);","highlight_start":5,"highlight_end":57}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"define_empty_struct_with_visibility!","def_site_span":{"file_name":"/checkout/src/test/u|     |                                    help: consider restricting its visibility: `crate`\n   |     in this macro invocation\n   |\n   = help: or consider exporting it for use by other crates\n\n"}
[00:51:07] {"message":"unreachable `pub` item","code":{"code":"unreachable_pub","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/lint/unreachable_pub.rs","byte_start":1545,"byte_end":1571,"line_start":49,"line_end":49,"column_start":9,"column_end":35,"is_primary":true,"text":[{"text":"        pub fn catalyze() -> bool;","highlight_start":9,"highlight_end":35}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"or consider exporting it for use by other crates","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"consider restricting its visibility","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/lint/unreachable_pub.rs","byte_start":1545,"byte_end":1548,"line_start":49,"line_end":49,"column_start":9,"column_end":12,"is_primary":true,"text":[{"text":"        pub fn catalyze() -> bool;","highlight_start":9,"highlight_end":12}],"label":null,"suggested_replacement":"crate","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unreachable `pub` item\n  --> /checkout/src/test/ui/lint/unreachable_pub.rs:49:9\n   |\nLL |         pub fn catalyze() -> bool;\n   |         ---^^^^^^^^^^^^^^^^^^^^^^^\n   |         |\n   |         help: consider restricting its visibility: `crate`\n   |\n   = help: or consider exporting it for u

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:16260794
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:02364219:start=1538000541243450840,finish=1538000541265770800,duration=22319960
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1f176506
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0b6b26f4
$ dmesg | grep -i kill
