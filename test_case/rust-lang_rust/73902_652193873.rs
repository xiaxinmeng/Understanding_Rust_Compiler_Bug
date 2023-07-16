plain
##[section]Starting: Linux mingw-check
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 4'
Agent machine name: 'fv-az578'
Current agent version: '2.171.1'
##[group]Operating System
16.04.6
LTS
LTS
##[endgroup]
##[group]Virtual Environment
Environment: ubuntu-16.04
Version: 20200621.1
Included Software: https://github.com/actions/virtual-environments/blob/ubuntu16/20200621.1/images/linux/Ubuntu1604-README.md
##[endgroup]
Agent running as: 'vsts'
Prepare build directory.
Set build variables.
Download all required tasks.
Download all required tasks.
Downloading task: Bash (3.171.1)
Checking job knob settings.
   Knob: AgentToolsDirectory = /opt/hostedtoolcache Source: ${AGENT_TOOLSDIRECTORY} 
   Knob: AgentPerflog = /home/vsts/perflog Source: ${VSTS_AGENT_PERFLOG} 
Start tracking orphan processes.
##[section]Finishing: Initialize job
##[section]Starting: Configure Job Name
==============================================================================
---
========================== Starting Command Output ===========================
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/9c626b44-c1c3-4307-8eaf-31bf8ae300e6.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/73902/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/73902/merge:refs/remotes/pull/73902/merge
---
 ---> a9ec21d337b3
Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
 ---> Using cache
 ---> 5ff2c13d8dba
Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            python3 ../x.py doc --stage 0 src/libstd &&            /scripts/validate-toolstate.sh
 ---> 6b931e755c7e
Successfully built 6b931e755c7e
Successfully tagged rust-ci:latest
Built container sha256:6b931e755c7ea1f69816640eb9df74fafd40a545d0e5aa8341d35009dabb0f3c
---
    Checking rustc_span v0.0.0 (/checkout/src/librustc_span)
    Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
    Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
    Checking rustc_parse_format v0.0.0 (/checkout/src/librustc_parse_format)
    Checking tracing v0.1.15
    Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
    Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
    Checking rustc_session v0.0.0 (/checkout/src/librustc_session)
    Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
    Checking chalk-solve v0.14.0
---
configure: build.locked-deps    := True
configure: rust.channel         := nightly
configure: llvm.assertions      := True
configure: build.cargo-native-static := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
Hugepagesize:       2048 kB
DirectMap4k:      153536 kB
DirectMap2M:     2992128 kB
DirectMap1G:     6291456 kB
+ python3 ../x.py test src/tools/expand-yaml-anchors
Ensuring the YAML anchors in the GitHub Actions config were expanded
Ensuring the YAML anchors in the GitHub Actions config were expanded
Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
---
   Compiling linked-hash-map v0.5.2
   Compiling lazy_static v1.4.0
   Compiling yaml-rust v0.4.3
   Compiling quote v1.0.2
   Compiling thiserror-impl v1.0.5
   Compiling thiserror v1.0.5
   Compiling yaml-merge-keys v0.4.0
   Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
Build completed successfully in 0:00:31
+ python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
    Finished dev [unoptimized + debuginfo] target(s) in 0.18s
Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
    Checking rustc_span v0.0.0 (/checkout/src/librustc_span)
    Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
    Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
    Checking rustc_parse_format v0.0.0 (/checkout/src/librustc_parse_format)
    Checking tracing v0.1.15
    Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
    Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
    Checking rustc_session v0.0.0 (/checkout/src/librustc_session)
    Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
    Checking chalk-solve v0.14.0
---
    Checking rustfix v0.5.0
    Checking flate2 v1.0.12
The following warnings were emitted during compilation:

warning: libssh2/src/wincng.c:63:28: fatal error: versionhelpers.h: No such file or directory

error: failed to run custom build command for `libssh2-sys v0.2.14`

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/release/build/libssh2-sys-2c5c02bb6d6e9018/build-script-build` (exit code: 1)
--- stdout
cargo:include=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/i686-pc-windows-gnu/release/build/libssh2-sys-b2a2826acdf9099f/out/include
cargo:root=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/i686-pc-windows-gnu/release/build/libssh2-sys-b2a2826acdf9099f/out
TARGET = Some("i686-pc-windows-gnu")
OPT_LEVEL = Some("3")
HOST = Some("x86_64-unknown-linux-gnu")
CC_i686-pc-windows-gnu = Some("sccache i686-w64-mingw32-gcc")
CFLAGS_i686-pc-windows-gnu = Some("-ffunction-sections -fdata-sections -m32 -fno-omit-frame-pointer")
CRATE_CC_NO_DEFAULTS = None
DEBUG = Some("false")
CARGO_CFG_TARGET_FEATURE = Some("fxsr,mmx,sse,sse2")
running: "sccache" "i686-w64-mingw32-gcc" "-O3" "-ffunction-sections" "-fdata-sections" "-m32" "-ffunction-sections" "-fdata-sections" "-m32" "-fno-omit-frame-pointer" "-I" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/i686-pc-windows-gnu/release/build/libssh2-sys-b2a2826acdf9099f/out/include" "-I" "libssh2/src" "-I" "libssh2/win32" "-I" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/i686-pc-windows-gnu/release/build/libz-sys-6da1b538ea6718da/out/include" "-DHAVE_LONGLONG" "-DLIBSSH2_WINCNG" "-DLIBSSH2_DH_GEX_NEW" "-DLIBSSH2_HAVE_ZLIB" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/i686-pc-windows-gnu/release/build/libssh2-sys-b2a2826acdf9099f/out/build/libssh2/src/agent.o" "-c" "libssh2/src/agent.c"
running: "sccache" "i686-w64-mingw32-gcc" "-O3" "-ffunction-sections" "-fdata-sections" "-m32" "-ffunction-sections" "-fdata-sections" "-m32" "-fno-omit-frame-pointer" "-I" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/i686-pc-windows-gnu/release/build/libssh2-sys-b2a2826acdf9099f/out/include" "-I" "libssh2/src" "-I" "libssh2/win32" "-I" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/i686-pc-windows-gnu/release/build/libz-sys-6da1b538ea6718da/out/include" "-DHAVE_LONGLONG" "-DLIBSSH2_WINCNG" "-DLIBSSH2_DH_GEX_NEW" "-DLIBSSH2_HAVE_ZLIB" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/i686-pc-windows-gnu/release/build/libssh2-sys-b2a2826acdf9099f/out/build/libssh2/src/bcrypt_pbkdf.o" "-c" "libssh2/src/bcrypt_pbkdf.c"
exit code: 0
exit code: 0
running: "sccache" "i686-w64-mingw32-gcc" "-O3" "-ffunction-sections" "-fdata-sections" "-m32" "-ffunction-sections" "-fdata-sections" "-m32" "-fno-omit-frame-pointer" "-I" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/i686-pc-windows-gnu/release/build/libssh2-sys-b2a2826acdf9099f/out/include" "-I" "libssh2/src" "-I" "libssh2/win32" "-I" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/i686-pc-windows-gnu/release/build/libz-sys-6da1b538ea6718da/out/include" "-DHAVE_LONGLONG" "-DLIBSSH2_WINCNG" "-DLIBSSH2_DH_GEX_NEW" "-DLIBSSH2_HAVE_ZLIB" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/i686-pc-windows-gnu/release/build/libssh2-sys-b2a2826acdf9099f/out/build/libssh2/src/blowfish.o" "-c" "libssh2/src/blowfish.c"
running: "sccache" "i686-w64-mingw32-gcc" "-O3" "-ffunction-sections" "-fdata-sections" "-m32" "-ffunction-sections" "-fdata-sections" "-m32" "-fno-omit-frame-pointer" "-I" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/i686-pc-windows-gnu/release/build/libssh2-sys-b2a2826acdf9099f/out/include" "-I" "libssh2/src" "-I" "libssh2/win32" "-I" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/i686-pc-windows-gnu/release/build/libz-sys-6da1b538ea6718da/out/include" "-DHAVE_LONGLONG" "-DLIBSSH2_WINCNG" "-DLIBSSH2_DH_GEX_NEW" "-DLIBSSH2_HAVE_ZLIB" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/i686-pc-windows-gnu/release/build/libssh2-sys-b2a2826acdf9099f/out/build/libssh2/src/channel.o" "-c" "libssh2/src/channel.c"
exit code: 0
running: "sccache" "i686-w64-mingw32-gcc" "-O3" "-ffunction-sections" "-fdata-sections" "-m32" "-ffunction-sections" "-fdata-sections" "-m32" "-fno-omit-frame-pointer" "-I" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/i686-pc-windows-gnu/release/build/libssh2-sys-b2a2826acdf9099f/out/include" "-I" "libssh2/src" "-I" "libssh2/win32" "-I" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/i686-pc-windows-gnu/release/build/libz-sys-6da1b538ea6718da/out/include" "-DHAVE_LONGLONG" "-DLIBSSH2_WINCNG" "-DLIBSSH2_DH_GEX_NEW" "-DLIBSSH2_HAVE_ZLIB" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/i686-pc-windows-gnu/release/build/libssh2-sys-b2a2826acdf9099f/out/build/libssh2/src/comp.o" "-c" "libssh2/src/comp.c"
exit code: 0
running: "sccache" "i686-w64-mingw32-gcc" "-O3" "-ffunction-sections" "-fdata-sections" "-m32" "-ffunction-sections" "-fdata-sections" "-m32" "-fno-omit-frame-pointer" "-I" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/i686-pc-windows-gnu/release/build/libssh2-sys-b2a2826acdf9099f/out/include" "-I" "libssh2/src" "-I" "libssh2/win32" "-I" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/i686-pc-windows-gnu/release/build/libz-sys-6da1b538ea6718da/out/include" "-DHAVE_LONGLONG" "-DLIBSSH2_WINCNG" "-DLIBSSH2_DH_GEX_NEW" "-DLIBSSH2_HAVE_ZLIB" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/i686-pc-windows-gnu/release/build/libssh2-sys-b2a2826acdf9099f/out/build/libssh2/src/crypt.o" "-c" "libssh2/src/crypt.c"
exit code: 0
running: "sccache" "i686-w64-mingw32-gcc" "-O3" "-ffunction-sections" "-fdata-sections" "-m32" "-ffunction-sections" "-fdata-sections" "-m32" "-fno-omit-frame-pointer" "-I" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/i686-pc-windows-gnu/release/build/libssh2-sys-b2a2826acdf9099f/out/include" "-I" "libssh2/src" "-I" "libssh2/win32" "-I" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/i686-pc-windows-gnu/release/build/libz-sys-6da1b538ea6718da/out/include" "-DHAVE_LONGLONG" "-DLIBSSH2_WINCNG" "-DLIBSSH2_DH_GEX_NEW" "-DLIBSSH2_HAVE_ZLIB" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/i686-pc-windows-gnu/release/build/libssh2-sys-b2a2826acdf9099f/out/build/libssh2/src/global.o" "-c" "libssh2/src/global.c"
exit code: 0
running: "sccache" "i686-w64-mingw32-gcc" "-O3" "-ffunction-sections" "-fdata-sections" "-m32" "-ffunction-sections" "-fdata-sections" "-m32" "-fno-omit-frame-pointer" "-I" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/i686-pc-windows-gnu/release/build/libssh2-sys-b2a2826acdf9099f/out/include" "-I" "libssh2/src" "-I" "libssh2/win32" "-I" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/i686-pc-windows-gnu/release/build/libz-sys-6da1b538ea6718da/out/include" "-DHAVE_LONGLONG" "-DLIBSSH2_WINCNG" "-DLIBSSH2_DH_GEX_NEW" "-DLIBSSH2_HAVE_ZLIB" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/i686-pc-windows-gnu/release/build/libssh2-sys-b2a2826acdf9099f/out/build/libssh2/src/hostkey.o" "-c" "libssh2/src/hostkey.c"
exit code: 0
running: "sccache" "i686-w64-mingw32-gcc" "-O3" "-ffunction-sections" "-fdata-sections" "-m32" "-ffunction-sections" "-fdata-sections" "-m32" "-fno-omit-frame-pointer" "-I" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/i686-pc-windows-gnu/release/build/libssh2-sys-b2a2826acdf9099f/out/include" "-I" "libssh2/src" "-I" "libssh2/win32" "-I" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/i686-pc-windows-gnu/release/build/libz-sys-6da1b538ea6718da/out/include" "-DHAVE_LONGLONG" "-DLIBSSH2_WINCNG" "-DLIBSSH2_DH_GEX_NEW" "-DLIBSSH2_HAVE_ZLIB" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/i686-pc-windows-gnu/release/build/libssh2-sys-b2a2826acdf9099f/out/build/libssh2/src/keepalive.o" "-c" "libssh2/src/keepalive.c"
exit code: 0
running: "sccache" "i686-w64-mingw32-gcc" "-O3" "-ffunction-sections" "-fdata-sections" "-m32" "-ffunction-sections" "-fdata-sections" "-m32" "-fno-omit-frame-pointer" "-I" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/i686-pc-windows-gnu/release/build/libssh2-sys-b2a2826acdf9099f/out/include" "-I" "libssh2/src" "-I" "libssh2/win32" "-I" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/i686-pc-windows-gnu/release/build/libz-sys-6da1b538ea6718da/out/include" "-DHAVE_LONGLONG" "-DLIBSSH2_WINCNG" "-DLIBSSH2_DH_GEX_NEW" "-DLIBSSH2_HAVE_ZLIB" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/i686-pc-windows-gnu/release/build/libssh2-sys-b2a2826acdf9099f/out/build/libssh2/src/kex.o" "-c" "libssh2/src/kex.c"
exit code: 0
running: "sccache" "i686-w64-mingw32-gcc" "-O3" "-ffunction-sections" "-fdata-sections" "-m32" "-ffunction-sections" "-fdata-sections" "-m32" "-fno-omit-frame-pointer" "-I" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/i686-pc-windows-gnu/release/build/libssh2-sys-b2a2826acdf9099f/out/include" "-I" "libssh2/src" "-I" "libssh2/win32" "-I" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/i686-pc-windows-gnu/release/build/libz-sys-6da1b538ea6718da/out/include" "-DHAVE_LONGLONG" "-DLIBSSH2_WINCNG" "-DLIBSSH2_DH_GEX_NEW" "-DLIBSSH2_HAVE_ZLIB" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/i686-pc-windows-gnu/release/build/libssh2-sys-b2a2826acdf9099f/out/build/libssh2/src/knownhost.o" "-c" "libssh2/src/knownhost.c"
exit code: 0
running: "sccache" "i686-w64-mingw32-gcc" "-O3" "-ffunction-sections" "-fdata-sections" "-m32" "-ffunction-sections" "-fdata-sections" "-m32" "-fno-omit-frame-pointer" "-I" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/i686-pc-windows-gnu/release/build/libssh2-sys-b2a2826acdf9099f/out/include" "-I" "libssh2/src" "-I" "libssh2/win32" "-I" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/i686-pc-windows-gnu/release/build/libz-sys-6da1b538ea6718da/out/include" "-DHAVE_LONGLONG" "-DLIBSSH2_WINCNG" "-DLIBSSH2_DH_GEX_NEW" "-DLIBSSH2_HAVE_ZLIB" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/i686-pc-windows-gnu/release/build/libssh2-sys-b2a2826acdf9099f/out/build/libssh2/src/mac.o" "-c" "libssh2/src/mac.c"
exit code: 0
running: "sccache" "i686-w64-mingw32-gcc" "-O3" "-ffunction-sections" "-fdata-sections" "-m32" "-ffunction-sections" "-fdata-sections" "-m32" "-fno-omit-frame-pointer" "-I" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/i686-pc-windows-gnu/release/build/libssh2-sys-b2a2826acdf9099f/out/include" "-I" "libssh2/src" "-I" "libssh2/win32" "-I" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/i686-pc-windows-gnu/release/build/libz-sys-6da1b538ea6718da/out/include" "-DHAVE_LONGLONG" "-DLIBSSH2_WINCNG" "-DLIBSSH2_DH_GEX_NEW" "-DLIBSSH2_HAVE_ZLIB" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/i686-pc-windows-gnu/release/build/libssh2-sys-b2a2826acdf9099f/out/build/libssh2/src/misc.o" "-c" "libssh2/src/misc.c"
exit code: 0
running: "sccache" "i686-w64-mingw32-gcc" "-O3" "-ffunction-sections" "-fdata-sections" "-m32" "-ffunction-sections" "-fdata-sections" "-m32" "-fno-omit-frame-pointer" "-I" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/i686-pc-windows-gnu/release/build/libssh2-sys-b2a2826acdf9099f/out/include" "-I" "libssh2/src" "-I" "libssh2/win32" "-I" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/i686-pc-windows-gnu/release/build/libz-sys-6da1b538ea6718da/out/include" "-DHAVE_LONGLONG" "-DLIBSSH2_WINCNG" "-DLIBSSH2_DH_GEX_NEW" "-DLIBSSH2_HAVE_ZLIB" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/i686-pc-windows-gnu/release/build/libssh2-sys-b2a2826acdf9099f/out/build/libssh2/src/packet.o" "-c" "libssh2/src/packet.c"
exit code: 0
running: "sccache" "i686-w64-mingw32-gcc" "-O3" "-ffunction-sections" "-fdata-sections" "-m32" "-ffunction-sections" "-fdata-sections" "-m32" "-fno-omit-frame-pointer" "-I" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/i686-pc-windows-gnu/release/build/libssh2-sys-b2a2826acdf9099f/out/include" "-I" "libssh2/src" "-I" "libssh2/win32" "-I" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/i686-pc-windows-gnu/release/build/libz-sys-6da1b538ea6718da/out/include" "-DHAVE_LONGLONG" "-DLIBSSH2_WINCNG" "-DLIBSSH2_DH_GEX_NEW" "-DLIBSSH2_HAVE_ZLIB" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/i686-pc-windows-gnu/release/build/libssh2-sys-b2a2826acdf9099f/out/build/libssh2/src/pem.o" "-c" "libssh2/src/pem.c"
exit code: 0
running: "sccache" "i686-w64-mingw32-gcc" "-O3" "-ffunction-sections" "-fdata-sections" "-m32" "-ffunction-sections" "-fdata-sections" "-m32" "-fno-omit-frame-pointer" "-I" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/i686-pc-windows-gnu/release/build/libssh2-sys-b2a2826acdf9099f/out/include" "-I" "libssh2/src" "-I" "libssh2/win32" "-I" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/i686-pc-windows-gnu/release/build/libz-sys-6da1b538ea6718da/out/include" "-DHAVE_LONGLONG" "-DLIBSSH2_WINCNG" "-DLIBSSH2_DH_GEX_NEW" "-DLIBSSH2_HAVE_ZLIB" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/i686-pc-windows-gnu/release/build/libssh2-sys-b2a2826acdf9099f/out/build/libssh2/src/publickey.o" "-c" "libssh2/src/publickey.c"
exit code: 0
running: "sccache" "i686-w64-mingw32-gcc" "-O3" "-ffunction-sections" "-fdata-sections" "-m32" "-ffunction-sections" "-fdata-sections" "-m32" "-fno-omit-frame-pointer" "-I" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/i686-pc-windows-gnu/release/build/libssh2-sys-b2a2826acdf9099f/out/include" "-I" "libssh2/src" "-I" "libssh2/win32" "-I" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/i686-pc-windows-gnu/release/build/libz-sys-6da1b538ea6718da/out/include" "-DHAVE_LONGLONG" "-DLIBSSH2_WINCNG" "-DLIBSSH2_DH_GEX_NEW" "-DLIBSSH2_HAVE_ZLIB" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/i686-pc-windows-gnu/release/build/libssh2-sys-b2a2826acdf9099f/out/build/libssh2/src/scp.o" "-c" "libssh2/src/scp.c"
exit code: 0
running: "sccache" "i686-w64-mingw32-gcc" "-O3" "-ffunction-sections" "-fdata-sections" "-m32" "-ffunction-sections" "-fdata-sections" "-m32" "-fno-omit-frame-pointer" "-I" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/i686-pc-windows-gnu/release/build/libssh2-sys-b2a2826acdf9099f/out/include" "-I" "libssh2/src" "-I" "libssh2/win32" "-I" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/i686-pc-windows-gnu/release/build/libz-sys-6da1b538ea6718da/out/include" "-DHAVE_LONGLONG" "-DLIBSSH2_WINCNG" "-DLIBSSH2_DH_GEX_NEW" "-DLIBSSH2_HAVE_ZLIB" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/i686-pc-windows-gnu/release/build/libssh2-sys-b2a2826acdf9099f/out/build/libssh2/src/session.o" "-c" "libssh2/src/session.c"
exit code: 0
running: "sccache" "i686-w64-mingw32-gcc" "-O3" "-ffunction-sections" "-fdata-sections" "-m32" "-ffunction-sections" "-fdata-sections" "-m32" "-fno-omit-frame-pointer" "-I" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/i686-pc-windows-gnu/release/build/libssh2-sys-b2a2826acdf9099f/out/include" "-I" "libssh2/src" "-I" "libssh2/win32" "-I" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/i686-pc-windows-gnu/release/build/libz-sys-6da1b538ea6718da/out/include" "-DHAVE_LONGLONG" "-DLIBSSH2_WINCNG" "-DLIBSSH2_DH_GEX_NEW" "-DLIBSSH2_HAVE_ZLIB" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/i686-pc-windows-gnu/release/build/libssh2-sys-b2a2826acdf9099f/out/build/libssh2/src/sftp.o" "-c" "libssh2/src/sftp.c"
exit code: 0
running: "sccache" "i686-w64-mingw32-gcc" "-O3" "-ffunction-sections" "-fdata-sections" "-m32" "-ffunction-sections" "-fdata-sections" "-m32" "-fno-omit-frame-pointer" "-I" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/i686-pc-windows-gnu/release/build/libssh2-sys-b2a2826acdf9099f/out/include" "-I" "libssh2/src" "-I" "libssh2/win32" "-I" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/i686-pc-windows-gnu/release/build/libz-sys-6da1b538ea6718da/out/include" "-DHAVE_LONGLONG" "-DLIBSSH2_WINCNG" "-DLIBSSH2_DH_GEX_NEW" "-DLIBSSH2_HAVE_ZLIB" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/i686-pc-windows-gnu/release/build/libssh2-sys-b2a2826acdf9099f/out/build/libssh2/src/transport.o" "-c" "libssh2/src/transport.c"
exit code: 0
running: "sccache" "i686-w64-mingw32-gcc" "-O3" "-ffunction-sections" "-fdata-sections" "-m32" "-ffunction-sections" "-fdata-sections" "-m32" "-fno-omit-frame-pointer" "-I" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/i686-pc-windows-gnu/release/build/libssh2-sys-b2a2826acdf9099f/out/include" "-I" "libssh2/src" "-I" "libssh2/win32" "-I" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/i686-pc-windows-gnu/release/build/libz-sys-6da1b538ea6718da/out/include" "-DHAVE_LONGLONG" "-DLIBSSH2_WINCNG" "-DLIBSSH2_DH_GEX_NEW" "-DLIBSSH2_HAVE_ZLIB" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/i686-pc-windows-gnu/release/build/libssh2-sys-b2a2826acdf9099f/out/build/libssh2/src/userauth.o" "-c" "libssh2/src/userauth.c"
exit code: 0
running: "sccache" "i686-w64-mingw32-gcc" "-O3" "-ffunction-sections" "-fdata-sections" "-m32" "-ffunction-sections" "-fdata-sections" "-m32" "-fno-omit-frame-pointer" "-I" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/i686-pc-windows-gnu/release/build/libssh2-sys-b2a2826acdf9099f/out/include" "-I" "libssh2/src" "-I" "libssh2/win32" "-I" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/i686-pc-windows-gnu/release/build/libz-sys-6da1b538ea6718da/out/include" "-DHAVE_LONGLONG" "-DLIBSSH2_WINCNG" "-DLIBSSH2_DH_GEX_NEW" "-DLIBSSH2_HAVE_ZLIB" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/i686-pc-windows-gnu/release/build/libssh2-sys-b2a2826acdf9099f/out/build/libssh2/src/wincng.o" "-c" "libssh2/src/wincng.c"
cargo:warning=libssh2/src/wincng.c:63:28: fatal error: versionhelpers.h: No such file or directory
cargo:warning=compilation terminated.
exit code: 0

--- stderr
fatal: Not a git repository (or any parent up to mount point /cargo)
fatal: Not a git repository (or any parent up to mount point /cargo)
Stopping at filesystem boundary (GIT_DISCOVERY_ACROSS_FILESYSTEM not set).


error occurred: Command "sccache" "i686-w64-mingw32-gcc" "-O3" "-ffunction-sections" "-fdata-sections" "-m32" "-ffunction-sections" "-fdata-sections" "-m32" "-fno-omit-frame-pointer" "-I" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/i686-pc-windows-gnu/release/build/libssh2-sys-b2a2826acdf9099f/out/include" "-I" "libssh2/src" "-I" "libssh2/win32" "-I" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/i686-pc-windows-gnu/release/build/libz-sys-6da1b538ea6718da/out/include" "-DHAVE_LONGLONG" "-DLIBSSH2_WINCNG" "-DLIBSSH2_DH_GEX_NEW" "-DLIBSSH2_HAVE_ZLIB" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/i686-pc-windows-gnu/release/build/libssh2-sys-b2a2826acdf9099f/out/build/libssh2/src/wincng.o" "-c" "libssh2/src/wincng.c" with args "i686-w64-mingw32-gcc" did not execute successfully (status code exit code: 1).


warning: build failed, waiting for other jobs to finish...
error: build failed
---
  local time: Wed Jul  1 03:42:57 UTC 2020
  network time: Wed, 01 Jul 2020 03:42:57 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/73902/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/73902/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3847) (python)
##[section]Finishing: Finalize Job
