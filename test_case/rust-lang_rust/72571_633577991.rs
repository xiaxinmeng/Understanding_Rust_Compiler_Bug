plain
##[section]Starting: Linux mingw-check
##[section]Starting: Initialize job
Agent name: 'Hosted Agent'
Agent machine name: 'fv-az578'
Current agent version: '2.169.0'
##[group]Operating System
16.04.6
LTS
LTS
##[endgroup]
##[group]Virtual Environment
Environment: ubuntu-16.04
Version: 20200517.1
Included Software: https://github.com/actions/virtual-environments/blob/ubuntu16/20200517.1/images/linux/Ubuntu1604-README.md
##[endgroup]
Agent running as: 'vsts'
Prepare build directory.
Set build variables.
Download all required tasks.
Download all required tasks.
Downloading task: Bash (3.163.2)
Checking job knob settings.
   Knob: AgentToolsDirectory = /opt/hostedtoolcache Source: ${AGENT_TOOLSDIRECTORY} 
   Knob: AgentPerflog = /home/vsts/perflog Source: ${VSTS_AGENT_PERFLOG} 
Start tracking orphan processes.
##[section]Finishing: Initialize job
##[section]Starting: Configure Job Name
==============================================================================
---
========================== Starting Command Output ===========================
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/ee14b960-27b4-419d-918c-4f57f95ce7ff.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/72571/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/72571/merge:refs/remotes/pull/72571/merge
---
Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
 ---> Running in e58e2155f293
Removing intermediate container e58e2155f293
 ---> 26da3909c36c
Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            python3 ../x.py doc --stage 0 src/libstd &&            /scripts/validate-toolstate.sh
Removing intermediate container 09c9063b4ad6
 ---> a33381f1eb6d
Successfully built a33381f1eb6d
Successfully tagged rust-ci:latest
Successfully tagged rust-ci:latest
Built container sha256:a33381f1eb6da604b1a7d06faf153d3fe2569e7daaf5337eff0293ad32212d9d
Uploading finished image to https://ci-caches.rust-lang.org/docker/5bc2903bfd26f1eecf82af5af297e10aa4b599340206326f40d447874d8599074a279b11a4efdc758357d3333f3bfe9393332c87d1ed40911a83c162f179476b
upload failed: - to s3://rust-lang-ci-sccache2/docker/5bc2903bfd26f1eecf82af5af297e10aa4b599340206326f40d447874d8599074a279b11a4efdc758357d3333f3bfe9393332c87d1ed40911a83c162f179476b An error occurred (InvalidAccessKeyId) when calling the CreateMultipartUpload operation: The AWS Access Key Id you provided does not exist in our records.
[CI_JOB_NAME=mingw-check]
== clock drift check ==
  local time: Mon May 25 13:29:47 UTC 2020
  network time: Mon, 25 May 2020 13:29:47 GMT
  network time: Mon, 25 May 2020 13:29:47 GMT
== end clock drift check ==
/checkout/src/ci/run.sh: line 114: /usr/local/bin/sccache: cannot execute binary file: Exec format error
configure: 
configure: rust.parallel-compiler := True
configure: build.configure-args := ['--enable-parallel-compiler']
configure: 
---
    Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
    Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
    Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
    Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
    Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
    Checking chalk-rust-ir v0.10.0
    Checking chalk-solve v0.10.0
    Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
    Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
    Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
configure: rust.codegen-units-std := 1
configure: rust.verify-llvm-ir  := True
configure: build.cargo-native-static := True
configure: build.submodules     := False
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
Hugepagesize:       2048 kB
DirectMap4k:      169920 kB
DirectMap2M:     4024320 kB
DirectMap1G:     5242880 kB
+ python3 ../x.py test src/tools/expand-yaml-anchors
Ensuring the YAML anchors in the GitHub Actions config were expanded
Ensuring the YAML anchors in the GitHub Actions config were expanded
Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
   Compiling unicode-xid v0.2.0
   Compiling syn v1.0.11
   Compiling linked-hash-map v0.5.2
   Compiling lazy_static v1.4.0
   Compiling lazy_static v1.4.0
   Compiling yaml-rust v0.4.3
   Compiling quote v1.0.2
   Compiling thiserror-impl v1.0.5
   Compiling thiserror v1.0.5
   Compiling yaml-merge-keys v0.4.0
   Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
Build completed successfully in 0:00:29
+ python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
    Finished dev [unoptimized] target(s) in 0.19s
Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
   Compiling backtrace-sys v0.1.37
   Compiling hashbrown v0.6.2
The following warnings were emitted during compilation:

warning: /usr/local/bin/sccache: 1: /usr/local/bin/sccache: Syntax error: word unexpected (expecting ")")
error: failed to run custom build command for `compiler_builtins v0.1.28`

Caused by:
  process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/build/compiler_builtins-5ee743d605184fb9/build-script-build` (exit code: 1)
  process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/build/compiler_builtins-5ee743d605184fb9/build-script-build` (exit code: 1)
--- stdout
cargo:rerun-if-changed=build.rs
cargo:compiler-rt=/cargo/registry/src/github.com-1ecc6299db9ec823/compiler_builtins-0.1.28/compiler-rt
cargo:rustc-cfg=feature="unstable"
cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/absvdi2.c
cargo:rustc-cfg=__absvdi2="optimized-c"
cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/absvsi2.c
cargo:rustc-cfg=__absvsi2="optimized-c"
cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/absvti2.c
cargo:rustc-cfg=__absvti2="optimized-c"
cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/addvdi3.c
cargo:rustc-cfg=__addvdi3="optimized-c"
cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/addvsi3.c
cargo:rustc-cfg=__addvsi3="optimized-c"
cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/addvti3.c
cargo:rustc-cfg=__addvti3="optimized-c"
cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/i386/ashldi3.S
cargo:rustc-cfg=__ashldi3="optimized-c"
cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/i386/ashrdi3.S
cargo:rustc-cfg=__ashrdi3="optimized-c"
cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/clzdi2.c
cargo:rustc-cfg=__clzdi2="optimized-c"
cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/clzsi2.c
cargo:rustc-cfg=__clzsi2="optimized-c"
cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/clzti2.c
cargo:rustc-cfg=__clzti2="optimized-c"
cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/cmpdi2.c
cargo:rustc-cfg=__cmpdi2="optimized-c"
cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/cmpti2.c
cargo:rustc-cfg=__cmpti2="optimized-c"
cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/ctzdi2.c
cargo:rustc-cfg=__ctzdi2="optimized-c"
cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/ctzsi2.c
cargo:rustc-cfg=__ctzsi2="optimized-c"
cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/ctzti2.c
cargo:rustc-cfg=__ctzti2="optimized-c"
cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/divdc3.c
cargo:rustc-cfg=__divdc3="optimized-c"
cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/i386/divdi3.S
cargo:rustc-cfg=__divdi3="optimized-c"
cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/divsc3.c
cargo:rustc-cfg=__divsc3="optimized-c"
cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/divxc3.c
cargo:rustc-cfg=__divxc3="optimized-c"
cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/extendhfsf2.c
cargo:rustc-cfg=__extendhfsf2="optimized-c"
cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/ffsti2.c
cargo:rustc-cfg=__ffsti2="optimized-c"
cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/i386/floatdidf.S
cargo:rustc-cfg=__floatdidf="optimized-c"
cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/i386/floatdisf.S
cargo:rustc-cfg=__floatdisf="optimized-c"
cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/i386/floatdixf.S
cargo:rustc-cfg=__floatdixf="optimized-c"
cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/i386/floatundidf.S
cargo:rustc-cfg=__floatundidf="optimized-c"
cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/i386/floatundisf.S
cargo:rustc-cfg=__floatundisf="optimized-c"
cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/i386/floatundixf.S
cargo:rustc-cfg=__floatundixf="optimized-c"
cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/int_util.c
cargo:rustc-cfg=__int_util="optimized-c"
cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/i386/lshrdi3.S
cargo:rustc-cfg=__lshrdi3="optimized-c"
cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/i386/moddi3.S
cargo:rustc-cfg=__moddi3="optimized-c"
cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/muldc3.c
cargo:rustc-cfg=__muldc3="optimized-c"
cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/i386/muldi3.S
cargo:rustc-cfg=__muldi3="optimized-c"
cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/mulsc3.c
cargo:rustc-cfg=__mulsc3="optimized-c"
cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/mulvdi3.c
cargo:rustc-cfg=__mulvdi3="optimized-c"
cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/mulvsi3.c
cargo:rustc-cfg=__mulvsi3="optimized-c"
cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/mulvti3.c
cargo:rustc-cfg=__mulvti3="optimized-c"
cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/mulxc3.c
cargo:rustc-cfg=__mulxc3="optimized-c"
cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/negdf2.c
cargo:rustc-cfg=__negdf2="optimized-c"
cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/negdi2.c
cargo:rustc-cfg=__negdi2="optimized-c"
cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/negsf2.c
cargo:rustc-cfg=__negsf2="optimized-c"
cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/negti2.c
cargo:rustc-cfg=__negti2="optimized-c"
cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/negvdi2.c
cargo:rustc-cfg=__negvdi2="optimized-c"
cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/negvsi2.c
cargo:rustc-cfg=__negvsi2="optimized-c"
cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/negvti2.c
cargo:rustc-cfg=__negvti2="optimized-c"
cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/paritydi2.c
cargo:rustc-cfg=__paritydi2="optimized-c"
cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/paritysi2.c
cargo:rustc-cfg=__paritysi2="optimized-c"
cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/parityti2.c
cargo:rustc-cfg=__parityti2="optimized-c"
cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/popcountdi2.c
cargo:rustc-cfg=__popcountdi2="optimized-c"
cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/popcountsi2.c
cargo:rustc-cfg=__popcountsi2="optimized-c"
cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/popcountti2.c
cargo:rustc-cfg=__popcountti2="optimized-c"
cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/powixf2.c
cargo:rustc-cfg=__powixf2="optimized-c"
cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/subvdi3.c
cargo:rustc-cfg=__subvdi3="optimized-c"
cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/subvsi3.c
cargo:rustc-cfg=__subvsi3="optimized-c"
cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/subvti3.c
cargo:rustc-cfg=__subvti3="optimized-c"
cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/truncdfhf2.c
cargo:rustc-cfg=__truncdfhf2="optimized-c"
cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/truncdfsf2.c
cargo:rustc-cfg=__truncdfsf2="optimized-c"
cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/truncsfhf2.c
cargo:rustc-cfg=__truncsfhf2="optimized-c"
cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/ucmpdi2.c
cargo:rustc-cfg=__ucmpdi2="optimized-c"
cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/ucmpti2.c
cargo:rustc-cfg=__ucmpti2="optimized-c"
cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/i386/udivdi3.S
cargo:rustc-cfg=__udivdi3="optimized-c"
cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/i386/umoddi3.S
cargo:rustc-cfg=__umoddi3="optimized-c"
cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/apple_versioning.c
cargo:rustc-cfg=apple_versioning="optimized-c"
TARGET = Some("i686-pc-windows-gnu")
OPT_LEVEL = Some("3")
HOST = Some("x86_64-unknown-linux-gnu")
CC_i686-pc-windows-gnu = Some("sccache i686-w64-mingw32-gcc")
CFLAGS_i686-pc-windows-gnu = Some("-ffunction-sections -fdata-sections -m32 -fno-omit-frame-pointer")
CRATE_CC_NO_DEFAULTS = None
DEBUG = Some("false")
CARGO_CFG_TARGET_FEATURE = Some("fxsr,mmx,sse,sse2")
CC_i686-pc-windows-gnu = Some("sccache i686-w64-mingw32-gcc")
CFLAGS_i686-pc-windows-gnu = Some("-ffunction-sections -fdata-sections -m32 -fno-omit-frame-pointer")
CRATE_CC_NO_DEFAULTS = None
CARGO_CFG_TARGET_FEATURE = Some("fxsr,mmx,sse,sse2")
CC_i686-pc-windows-gnu = Some("sccache i686-w64-mingw32-gcc")
CFLAGS_i686-pc-windows-gnu = Some("-ffunction-sections -fdata-sections -m32 -fno-omit-frame-pointer")
CRATE_CC_NO_DEFAULTS = None
CARGO_CFG_TARGET_FEATURE = Some("fxsr,mmx,sse,sse2")
running: "sccache" "i686-w64-mingw32-gcc" "-O3" "-ffunction-sections" "-fdata-sections" "-m32" "-ffunction-sections" "-fdata-sections" "-m32" "-fno-omit-frame-pointer" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/i686-pc-windows-gnu/release/build/compiler_builtins-75f8d5bc7b61f73b/out/absvdi2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/absvdi2.c"
cargo:warning=/usr/local/bin/sccache: 1: /usr/local/bin/sccache: Syntax error: word unexpected (expecting ")")

--- stderr



error occurred: Command "sccache" "i686-w64-mingw32-gcc" "-O3" "-ffunction-sections" "-fdata-sections" "-m32" "-ffunction-sections" "-fdata-sections" "-m32" "-fno-omit-frame-pointer" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/i686-pc-windows-gnu/release/build/compiler_builtins-75f8d5bc7b61f73b/out/absvdi2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/absvdi2.c" with args "i686-w64-mingw32-gcc" did not execute successfully (status code exit code: 2).


warning: build failed, waiting for other jobs to finish...
error: build failed
---
  local time: Mon May 25 13:39:44 UTC 2020
  network time: Mon, 25 May 2020 13:39:44 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/72571/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/72571/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (4078) (python)
##[section]Finishing: Finalize Job
