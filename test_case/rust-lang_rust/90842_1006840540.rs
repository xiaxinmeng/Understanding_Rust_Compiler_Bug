plain

######################################################################## 100.0%
extracting /checkout/obj/build/cache/2021-11-30/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.xz
Building rustbuild
warning: patch for `indexmap` uses the features mechanism. default-features and features will not take effect because the patch dependency does not support this mechanism
    Updating git repository `https://github.com/bluss/indexmap`
---
   Compiling ignore v0.4.17
   Compiling toml v0.5.7
    Finished dev [unoptimized] target(s) in 48.50s
Checking stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
warning: patch for `indexmap` uses the features mechanism. default-features and features will not take effect because the patch dependency does not support this mechanism
---
    Checking getopts v0.2.21
    Checking test v0.0.0 (/checkout/library/test)
    Finished release [optimized] target(s) in 18.22s
Checking stage0 std test/bench/example targets (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
warning: patch for `indexmap` uses the features mechanism. default-features and features will not take effect because the patch dependency does not support this mechanism
---
    Checking alloc v0.0.0 (/checkout/library/alloc)
    Checking core v0.0.0 (/checkout/library/core)
    Finished release [optimized] target(s) in 10.82s
Checking stage0 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
warning: patch for `indexmap` uses the features mechanism. default-features and features will not take effect because the patch dependency does not support this mechanism
---
    Checking getopts v0.2.21
   Compiling crossbeam-utils v0.7.2
   Compiling memoffset v0.5.5
   Compiling crossbeam-epoch v0.8.2
   Compiling indexmap v1.7.0 (https://github.com/bluss/indexmap?branch=master#4d6dde35)
    Checking unic-char-property v0.9.0
   Compiling generic-array v0.14.4
    Checking miniz_oxide v0.4.0
    Checking unicode-normalization v0.1.13
---
    Checking rustc_driver v0.0.0 (/checkout/compiler/rustc_driver)
    Checking rustc-main v0.0.0 (/checkout/compiler/rustc)
    Finished release [optimized] target(s) in 52.15s
Checking stage0 rustdoc artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
warning: patch for `indexmap` uses the features mechanism. default-features and features will not take effect because the patch dependency does not support this mechanism
---
    Checking gccjit v1.0.0 (https://github.com/antoyo/gccjit.rs#0672b78d)
    Checking rustc_codegen_gcc v0.1.0 (/checkout/compiler/rustc_codegen_gcc)
    Finished release [optimized] target(s) in 1.62s
Checking stage0 clippy artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
warning: patch for `indexmap` uses the features mechanism. default-features and features will not take effect because the patch dependency does not support this mechanism
---
    Checking rustc-workspace-hack v1.0.0 (/checkout/src/tools/rustc-workspace-hack)
    Checking clippy_lints v0.1.59 (/checkout/src/tools/clippy/clippy_lints)
    Finished release [optimized] target(s) in 20.29s
Checking stage0 rustfmt artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
warning: patch for `indexmap` uses the features mechanism. default-features and features will not take effect because the patch dependency does not support this mechanism
---
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---

######################################################################## 100.0%
extracting /checkout/obj/build/cache/2021-11-30/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.xz
Building rustbuild
warning: patch for `indexmap` uses the features mechanism. default-features and features will not take effect because the patch dependency does not support this mechanism
   Compiling proc-macro2 v1.0.30
   Compiling unicode-xid v0.2.2
   Compiling lazy_static v1.4.0
   Compiling syn v1.0.80
---
DirectMap2M:     4978688 kB
DirectMap1G:    55574528 kB
+ python3 ../x.py --stage 2 test src/tools/expand-yaml-anchors
Building rustbuild
warning: patch for `indexmap` uses the features mechanism. default-features and features will not take effect because the patch dependency does not support this mechanism
Ensuring the YAML anchors in the GitHub Actions config were expanded
Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
warning: patch for `indexmap` uses the features mechanism. default-features and features will not take effect because the patch dependency does not support this mechanism
---
    Finished release [optimized] target(s) in 5.34s
Build completed successfully in 0:00:05
+ python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
Building rustbuild
warning: patch for `indexmap` uses the features mechanism. default-features and features will not take effect because the patch dependency does not support this mechanism
Warning: --all-targets is now on by default and does not need to be passed explicitly.
Checking stage0 std artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
Checking stage0 std artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
warning: patch for `indexmap` uses the features mechanism. default-features and features will not take effect because the patch dependency does not support this mechanism
    Checking core v0.0.0 (/checkout/library/core)
   Compiling libc v0.2.108
   Compiling memchr v2.4.1
   Compiling std v0.0.0 (/checkout/library/std)
---
    Checking getopts v0.2.21
    Checking test v0.0.0 (/checkout/library/test)
    Finished release [optimized] target(s) in 18.11s
Checking stage0 std test/bench/example targets (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
warning: patch for `indexmap` uses the features mechanism. default-features and features will not take effect because the patch dependency does not support this mechanism
    Checking ppv-lite86 v0.2.8
    Checking proc_macro v0.0.0 (/checkout/library/proc_macro)
    Checking test v0.0.0 (/checkout/library/test)
    Checking rand_core v0.5.1
---
    Checking core v0.0.0 (/checkout/library/core)
    Checking std v0.0.0 (/checkout/library/std)
    Finished release [optimized] target(s) in 11.11s
Checking stage0 compiler artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
warning: patch for `indexmap` uses the features mechanism. default-features and features will not take effect because the patch dependency does not support this mechanism
---
    Checking itertools v0.10.1
   Compiling crossbeam-utils v0.7.2
   Compiling memoffset v0.5.5
   Compiling crossbeam-epoch v0.8.2
   Compiling indexmap v1.7.0 (https://github.com/bluss/indexmap?branch=master#4d6dde35)
    Checking unic-char-property v0.9.0
    Checking unic-ucd-version v0.9.0
    Checking miniz_oxide v0.4.0
   Compiling generic-array v0.14.4
---
    Checking rustc_driver v0.0.0 (/checkout/compiler/rustc_driver)
    Checking rustc-main v0.0.0 (/checkout/compiler/rustc)
    Finished release [optimized] target(s) in 51.23s
Checking stage0 rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
warning: patch for `indexmap` uses the features mechanism. default-features and features will not take effect because the patch dependency does not support this mechanism
    Checking lazy_static v1.4.0
   Compiling winapi-i686-pc-windows-gnu v0.4.0
   Compiling winapi v0.3.9
   Compiling proc-macro2 v1.0.30
---
    Checking gccjit v1.0.0 (https://github.com/antoyo/gccjit.rs#0672b78d)
    Checking rustc_codegen_gcc v0.1.0 (/checkout/compiler/rustc_codegen_gcc)
    Finished release [optimized] target(s) in 1.42s
Checking stage0 clippy artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
warning: patch for `indexmap` uses the features mechanism. default-features and features will not take effect because the patch dependency does not support this mechanism
---
    Checking clippy_lints v0.1.59 (/checkout/src/tools/clippy/clippy_lints)
    Checking rustc-workspace-hack v1.0.0 (/checkout/src/tools/rustc-workspace-hack)
    Finished release [optimized] target(s) in 22.03s
Checking stage0 rustfmt artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
warning: patch for `indexmap` uses the features mechanism. default-features and features will not take effect because the patch dependency does not support this mechanism
    Checking vec_map v0.8.2
    Checking strsim v0.8.0
   Compiling rustfmt-nightly v1.4.38 (/checkout/src/tools/rustfmt)
    Checking humantime v2.0.1
---
    Finished release [optimized] target(s) in 7.41s
Build completed successfully in 0:02:21
+ python3 ../x.py build --stage 0 src/tools/build-manifest
Building rustbuild
warning: patch for `indexmap` uses the features mechanism. default-features and features will not take effect because the patch dependency does not support this mechanism
Building stage0 tool build-manifest (x86_64-unknown-linux-gnu)
Building stage0 tool build-manifest (x86_64-unknown-linux-gnu)
warning: patch for `indexmap` uses the features mechanism. default-features and features will not take effect because the patch dependency does not support this mechanism
---
    Finished release [optimized] target(s) in 11.53s
Build completed successfully in 0:00:11
+ python3 ../x.py test --stage 0 src/tools/compiletest
Building rustbuild
warning: patch for `indexmap` uses the features mechanism. default-features and features will not take effect because the patch dependency does not support this mechanism
Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
warning: patch for `indexmap` uses the features mechanism. default-features and features will not take effect because the patch dependency does not support this mechanism
   Compiling core v0.0.0 (/checkout/library/core)
   Compiling libc v0.2.108
   Compiling memchr v2.4.1
   Compiling std v0.0.0 (/checkout/library/std)
---
   Compiling getopts v0.2.21
   Compiling test v0.0.0 (/checkout/library/test)
    Finished release [optimized] target(s) in 50.69s
Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
warning: patch for `indexmap` uses the features mechanism. default-features and features will not take effect because the patch dependency does not support this mechanism
---

Build completed successfully in 0:01:07
+ python3 ../x.py test --stage 2 src/tools/tidy
Building rustbuild
warning: patch for `indexmap` uses the features mechanism. default-features and features will not take effect because the patch dependency does not support this mechanism
Building stage0 tool tidy (x86_64-unknown-linux-gnu)
Building stage0 tool tidy (x86_64-unknown-linux-gnu)
warning: patch for `indexmap` uses the features mechanism. default-features and features will not take effect because the patch dependency does not support this mechanism
---
   Compiling regex v1.5.4
   Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
    Finished release [optimized] target(s) in 7.93s
tidy check
tidy error: invalid source: "git+https://github.com/bluss/indexmap?branch=master#4d6dde35b59009e6097a58c6ebbb0cb9b549709d"
Checking which error codes lack tests...
* 628 error codes
* highest error code: E0786
Found 502 error codes
Found 502 error codes
Found 0 error codes with no tests
Done!
tidy error: invalid license `MIT/Apache-2.0 AND BSD-2-Clause` in `crossbeam-queue 0.2.3 (registry+https://github.com/rust-lang/crates.io-index)`
some tidy checks failed



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rust-tidy" "/checkout" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build" "16"


Build completed unsuccessfully in 0:00:10
