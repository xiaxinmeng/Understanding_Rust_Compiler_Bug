plain
##[section]Starting: Linux x86_64-gnu-llvm-8
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 4'
Agent machine name: 'fv-az578'
Current agent version: '2.168.2'
##[group]Operating System
16.04.6
LTS
LTS
##[endgroup]
##[group]Virtual Environment
Environment: ubuntu-16.04
Version: 20200430.2
Included Software: https://github.com/actions/virtual-environments/blob/ubuntu16/20200430.2/images/linux/Ubuntu1604-README.md
##[endgroup]
Agent running as: 'vsts'
Prepare build directory.
Set build variables.
Download all required tasks.
Download all required tasks.
Downloading task: Bash (3.163.1)
Checking job knob settings.
   Knob: AgentToolsDirectory = /opt/hostedtoolcache Source: ${AGENT_TOOLSDIRECTORY} 
   Knob: AgentPerflog = /home/vsts/perflog Source: ${VSTS_AGENT_PERFLOG} 
Start tracking orphan processes.
##[section]Finishing: Initialize job
##[section]Starting: Configure Job Name
==============================================================================
---
========================== Starting Command Output ===========================
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/cfa731ae-b8a6-4a32-96e9-30fadca99649.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/72053/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/72053/merge:refs/remotes/pull/72053/merge
---
  libstdc++6-armel-cross libtinfo-dev libtsan0 libubsan0 libubsan0-armel-cross
  libuv1 libwind0-heimdal libxml2 libyaml-0-2 linux-libc-dev
  linux-libc-dev-armel-cross llvm-8 llvm-8-dev llvm-8-runtime mime-support
  multiarch-support openssl patch perl perl-modules-5.26 python python-minimal
  python-pygments python-yaml python2.7-minimal readline-common
  binutils-doc cmake-doc ninja-build cpp-doc gcc-7-locales debian-keyring
  g++-multilib g++-7-multilib gcc-7-doc libstdc++6-7-dbg
  g++-7-multilib-arm-linux-gnueabi libstdc++6-7-dbg-armel-cross gcc-multilib
  manpages-dev autoconf automake libtool flex bison gcc-doc gcc-7-multilib
---
  git-email git-gui gitk gitweb git-cvs git-mediawiki git-svn lrzip glibc-doc
  gnupg | gnupg2 bzr gdbm-l10n krb5-doc krb5-user ncurses-doc libssl-doc
  libstdc++-7-doc llvm-8-doc make-doc ed diffutils-doc perl-doc
  libterm-readline-gnu-perl | libterm-readline-perl-perl python-doc python-tk
  ttf-bitstream-vera python2.7-doc readline-doc
  build-essential fakeroot gnupg | gnupg2 libalgorithm-merge-perl libc-dbg
  gdbserver less ssh-client manpages manpages-dev libfile-fcntllock-perl
  liblocale-gettext-perl libglib2.0-data shared-mime-info xdg-user-dirs
  liblocale-gettext-perl libglib2.0-data shared-mime-info xdg-user-dirs
  krb5-locales publicsuffix libsasl2-modules nodejs-doc netbase python-chardet
  python-pkg-resources
  binfmt-support binutils binutils-arm-linux-gnueabi binutils-common
  binutils-x86-64-linux-gnu ca-certificates cmake cmake-data cpp cpp-7
  cpp-7-arm-linux-gnueabi cpp-arm-linux-gnueabi curl dpkg-dev file g++ g++-7
  g++-7-arm-linux-gnueabi g++-arm-linux-gnueabi gcc gcc-7
---
  libssl-dev libssl1.0.0 libssl1.1 libstdc++-7-dev libstdc++-7-dev-armel-cross
  libstdc++6-armel-cross libtinfo-dev libtsan0 libubsan0 libubsan0-armel-cross
  libuv1 libwind0-heimdal libxml2 libyaml-0-2 linux-libc-dev
  linux-libc-dev-armel-cross llvm-8 llvm-8-dev llvm-8-runtime llvm-8-tools
  make mime-support multiarch-support nodejs openssl patch perl
  perl-modules-5.26 pkg-config python python-minimal python-pygments
  python-yaml python2.7 python2.7-minimal readline-common sudo xz-utils
0 upgraded, 151 newly installed, 0 to remove and 0 not upgraded.
Need to get 167 MB of archives.
After this operation, 782 MB of additional disk space will be used.
Get:1 http://archive.ubuntu.com/ubuntu bionic/main amd64 multiarch-support amd64 2.27-3ubuntu1 [6916 B]
---
Unpacking libpython3.6-stdlib:amd64 (3.6.9-1~18.04ubuntu1) ...
Selecting previously unselected package libxml2:amd64.
Preparing to unpack .../013-libxml2_2.9.4+dfsg1-6.1ubuntu1.3_amd64.deb ...
Unpacking libxml2:amd64 (2.9.4+dfsg1-6.1ubuntu1.3) ...
Selecting previously unselected package libyaml-0-2:amd64.
Preparing to unpack .../014-libyaml-0-2_0.1.7-2ubuntu3_amd64.deb ...
Unpacking libyaml-0-2:amd64 (0.1.7-2ubuntu3) ...
Preparing to unpack .../015-sudo_1.8.21p2-3ubuntu1.2_amd64.deb ...
Unpacking sudo (1.8.21p2-3ubuntu1.2) ...
Selecting previously unselected package xz-utils.
Preparing to unpack .../016-xz-utils_5.2.2-1.3_amd64.deb ...
---
Unpacking libffi-dev:amd64 (3.2.1-8) ...
Selecting previously unselected package llvm-8-dev.
Preparing to unpack .../123-llvm-8-dev_1%3a8-3~ubuntu18.04.2_amd64.deb ...
Unpacking llvm-8-dev (1:8-3~ubuntu18.04.2) ...
Selecting previously unselected package python-pygments.
Preparing to unpack .../124-python-pygments_2.2.0+dfsg-1_all.deb ...
Unpacking python-pygments (2.2.0+dfsg-1) ...
Selecting previously unselected package python-yaml.
Preparing to unpack .../125-python-yaml_3.12-1build2_amd64.deb ...
Unpacking python-yaml (3.12-1build2) ...
Preparing to unpack .../126-llvm-8-tools_1%3a8-3~ubuntu18.04.2_amd64.deb ...
Unpacking llvm-8-tools (1:8-3~ubuntu18.04.2) ...
Selecting previously unselected package pkg-config.
Preparing to unpack .../127-pkg-config_0.29.1-0ubuntu2_amd64.deb ...
---
Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
 ---> Running in 17ccf963fd99
Removing intermediate container 17ccf963fd99
 ---> 86bbf6fe3109
Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/test/ui --pass=check                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
Removing intermediate container cb6320bce437
 ---> 3d96cce15780
Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
 ---> Running in 77f1a5035ee0
---
Successfully built c52a24040a89
Successfully tagged rust-ci:latest
Built container sha256:c52a24040a89f7a40b9040103874e35b265e50d96e1e9b088c2179f371f894de
Uploading finished image to https://rust-lang-ci-sccache2.s3.amazonaws.com/docker/fc773962efc3c429597943dc46b03e2249d7b404de513953c19c4bcbb5a19a6e1e16973a0a3027dc03c71ffe9f5a70f4bf6d3f9417d3b9e44433977916ce13c7
upload failed: - to s3://rust-lang-ci-sccache2/docker/fc773962efc3c429597943dc46b03e2249d7b404de513953c19c4bcbb5a19a6e1e16973a0a3027dc03c71ffe9f5a70f4bf6d3f9417d3b9e44433977916ce13c7 An error occurred (InvalidAccessKeyId) when calling the CreateMultipartUpload operation: The AWS Access Key Id you provided does not exist in our records.
[CI_JOB_NAME=x86_64-gnu-llvm-8]
== clock drift check ==
  local time: Sat May  9 17:33:24 UTC 2020
  network time: Sat, 09 May 2020 17:33:24 GMT
---
   Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
   Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
   Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
   Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
   Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
   Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
   Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
   Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
   Compiling rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
---
   Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
   Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
   Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
   Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
   Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
   Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
   Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
   Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
   Compiling rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
---
.......................i............................................................................ 1800/9999
.................................................................................................... 1900/9999
........................................i........................................................... 2000/9999
.................................................................................................... 2100/9999
..............................iiiii................................................................. 2200/9999
.................................................................................................... 2400/9999
.................................................................................................... 2500/9999
.................................................................................................... 2600/9999
.................................................................................................... 2700/9999
---
.....................i...............i.............................................................. 5100/9999
.................................................................................................... 5200/9999
...................................................................i................................ 5300/9999
...........................................................i........................................ 5400/9999
................................................................ii.ii........i...i.................. 5500/9999
......i............................................................................................. 5700/9999
.............i...................................................................................... 5800/9999
.................................................ii.....................................i........... 5900/9999
.................................................................................................... 6000/9999
.................................................................................................... 6000/9999
.................................................................................................... 6100/9999
.....................................................................................ii...i..ii..... 6200/9999
.................................................................................................... 6400/9999
.................................................................................................... 6500/9999
.................................................................................................... 6600/9999
.................................................................................................... 6600/9999
.................i..ii.............................................................................. 6700/9999
.................................................................................................... 6900/9999
..................i................................................................................. 7000/9999
.................................................................................................... 7100/9999
............................................................i....................................... 7200/9999
---
.................................................................................................... 7900/9999
.................................................................................................... 8000/9999
.................................................................................................... 8100/9999
............................i....................................................................... 8200/9999
..................................................................................iiiiii.iiiii.i.... 8300/9999
...................................i................................................................ 8500/9999
.................................................................................................... 8600/9999
.................................................................................................... 8700/9999
.................................................................................................... 8800/9999
---
Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 188 tests
iiii......i.............ii.i..........i...............................i..i..................i....i.. 100/188
..........i.i.i...iii..iiiiiiiiiiiiiiii.......................iii...............ii......

 finished in 5.964
Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 9 tests
iiiiiiiii

 finished in 0.179
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 115 tests
iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
...iiii.....ii.

 finished in 16.011
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---

   Doc-tests core

running 2554 tests
......iiiii......................................................................................... 100/2554
.......................................................................................ii........... 200/2554
.........................i.......................................................................... 400/2554
...............................................................................i..i................. 500/2554
.iiii............................................................................................... 600/2554
.................................................................................................... 700/2554
---

running 1020 tests
i................................................................................................... 100/1020
.................................................................................................... 200/1020
...................iii......i......i...i......i..................................................... 300/1020
.................................................................................................... 400/1020
....................................................i....i......................................ii.. 500/1020
.................................................................................................... 700/1020
.................................................................................................... 700/1020
..............................................iiii.................................................. 800/1020
.................................................................................................... 900/1020
....................................................................iiii............................ 1000/1020
test result: ok. 1000 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out

 finished in 166.699
Set({"src/libterm"}) not skipped for "bootstrap::test::Crate" -- not in ["src/tools/tidy"]
---

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

 finished in 1.135
Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
Testing rustc_query_system stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
     Running build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_query_system-d2b3159181a74606

running 0 tests

---
Set({"/checkout/src/librustc_parse"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"/checkout/src/librustc_passes"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"/checkout/src/librustc_plugin_impl"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"/checkout/src/librustc_privacy"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"/checkout/src/librustc_save_analysis"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"/checkout/src/librustc_session"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"/checkout/src/librustc_span"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"/checkout/src/librustc_symbol_mangling"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
---
Suite("src/test/run-make-fulldeps") not skipped for "bootstrap::test::RunMakeFullDeps" -- not in ["src/tools/tidy"]
Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 211 tests
......................i...ii.......................................................................i 100/211
........................................iiiiii......i..............iii.............................. 200/211
.......ii..

 finished in 77.356
Set({"src/doc/rustdoc"}) not skipped for "bootstrap::test::RustdocBook" -- not in ["src/tools/tidy"]
doc tests for: /checkout/src/doc/rustdoc/src/advanced-features.md
---
doc tests for: /checkout/src/doc/rustc/src/targets/index.md
doc tests for: /checkout/src/doc/rustc/src/what-is-rustc.md
 finished in 4.545
Set({"src/test/rustdoc-js-std"}) not skipped for "bootstrap::test::RustdocJSStd" -- not in ["src/tools/tidy"]
Checking "alias-1" ... OK
Checking "alias-2" ... OK
Checking "alias-3" ... OK
Checking "alias" ... OK
Checking "basic" ... OK
Checking "deduplication" ... OK
Checking "enum-option" ... OK
Checking "filter-crate" ... OK
Checking "fn-forget" ... OK
Checking "from_u" ... OK
Checking "keyword" ... OK
Checking "macro-check" ... OK
Checking "macro-print" ... OK
Checking "multi-query" ... OK
Checking "never" ... OK
Checking "quoted" ... OK
Checking "return-specific-literal" ... OK
Checking "return-specific" ... OK
Checking "should-fail" ... OK
Checking "string-from_ut" ... OK
Checking "struct-vec" ... OK
Checking "vec-new" ... OK
Check compiletest suite=rustdoc-js mode=js-doc-test (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 6 tests
......
---
Suite("src/test/run-make") not skipped for "bootstrap::test::RunMake" -- not in ["src/tools/tidy"]
Check compiletest suite=run-make mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 13 tests
.iiiiiii.iii.

 finished in 0.641
Build completed successfully in 1:51:49
Build completed successfully in 1:51:49
+ python2.7 ../x.py test src/test/mir-opt --pass=build --target=armv5te-unknown-linux-gnueabi
Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.28s
Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Building stage0 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
test result: ok. 96 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

 finished in 8.857
Build completed successfully in 0:01:26
+ python2.7 ../x.py test src/test/ui --pass=check --target=armv5te-unknown-linux-gnueabi
Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.26s
Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Building stage0 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
    Finished release [optimized] target(s) in 0.20s
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> armv5te-unknown-linux-gnueabi)

running 9999 tests
ii...............................i.ii............................................................... 100/9999
.................................................................................................... 300/9999
.................................................................................................... 400/9999
...........................................................................FF.F..................... 500/9999
...............................................................................i.................... 600/9999
---
.................................................................................................... 1400/9999
.................................................................................................... 1500/9999
...................i................................................................................ 1600/9999
.................................................................................................... 1700/9999
......ii.i.............i................................................i....................i.i.... 1800/9999
........................................i........................................................... 2000/9999
.................................................................................................... 2100/9999
.................................................................................................... 2100/9999
..............................iiiii................................................................. 2200/9999
.................................................................................................... 2400/9999
.................................................................................................... 2500/9999
.................................................................................................... 2600/9999
.............................i........i.........i................................................... 2700/9999
---
.............i.........................................................i.ii......................... 5800/9999
.................................................ii.....................................i........... 5900/9999
.................................................................................................... 6000/9999
.................................................................................................... 6100/9999
.....................................................................................ii...i..ii..... 6200/9999
.................................................................................................... 6400/9999
.................................................................................................... 6500/9999
.................................................................................................... 6600/9999
.................................................................................................... 6600/9999
..............i..i..ii.............................................................................. 6700/9999
.................................................................................................... 6900/9999
..................i................................................................................. 7000/9999
.................................................................................................... 7100/9999
............................................................i....................................... 7200/9999
---
.................................................................................................... 7900/9999
.................................................................................................... 8000/9999
.................................................................................................... 8100/9999
............................i....................................................................... 8200/9999
......iiiii.......................................................................iiiiii.iiiii.i.... 8300/9999
.................................................................................................... 8500/9999
.................................................................................................... 8600/9999
.................................................................................................... 8700/9999
.................................................................................................... 8800/9999
---
failures:

---- [ui] ui/async-await/issues/issue-65419/issue-65419-async-fn-resume-after-completion.rs stdout ----

error: error pattern ' thread 'main' panicked at '`async fn` resumed after completion'' not found!
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-65419/issue-65419-async-fn-resume-after-completion/a"
stdout:
------------------------------------------


------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-65419/issue-65419-async-fn-resume-after-completion/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-65419/issue-65419-async-fn-resume-after-completion/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/async-await/issues/issue-65419/issue-65419-async-fn-resume-after-panic.rs stdout ----
---- [ui] ui/async-await/issues/issue-65419/issue-65419-async-fn-resume-after-panic.rs stdout ----

error: error pattern ' thread 'main' panicked at '`async fn` resumed after panicking'' not found!
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-65419/issue-65419-async-fn-resume-after-panic/a"
stdout:
------------------------------------------


------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-65419/issue-65419-async-fn-resume-after-panic/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-65419/issue-65419-async-fn-resume-after-panic/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/async-await/issues/issue-65419/issue-65419-generator-resume-after-completion.rs stdout ----
---

------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-65419/issue-65419-generator-resume-after-completion/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-65419/issue-65419-generator-resume-after-completion/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/issues/issue-69225-SCEVAddExpr-wrap-flag.rs stdout ----
---- [ui] ui/issues/issue-69225-SCEVAddExpr-wrap-flag.rs stdout ----

error: error pattern ' index out of bounds: the len is 0 but the index is 16777216' not found!
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-69225-SCEVAddExpr-wrap-flag/a"
stdout:
------------------------------------------


------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-69225-SCEVAddExpr-wrap-flag/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-69225-SCEVAddExpr-wrap-flag/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/issues/issue-69225-layout-repeated-checked-add.rs stdout ----
---- [ui] ui/issues/issue-69225-layout-repeated-checked-add.rs stdout ----

error: error pattern ' index out of bounds: the len is 0 but the index is 16777216' not found!
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-69225-layout-repeated-checked-add/a"
stdout:
------------------------------------------


------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-69225-layout-repeated-checked-add/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-69225-layout-repeated-checked-add/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/test-panic-abort-nocapture.rs stdout ----
---
- 
- 


The actual run.stdout differed from the expected run.stdout.
Actual run.stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-panic-abort-nocapture/test-panic-abort-nocapture.run.stdout
diff of run.stderr:
- thread 'main' panicked at 'assertion failed: `(left == right)`
-   left: `2`,
-  right: `4`', $DIR/test-panic-abort-nocapture.rs:32:5
- note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
- note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
- thread 'main' panicked at 'assertion failed: `(left == right)`
-   left: `2`,
-  right: `4`', $DIR/test-panic-abort-nocapture.rs:26:5
- note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
- testing321
+ $TEST_BUILD_DIR/test-panic-abort-nocapture/a: 1: $TEST_BUILD_DIR/test-panic-abort-nocapture/a: Syntax error: word unexpected (expecting ")")


The actual run.stderr differed from the expected run.stderr.
The actual run.stderr differed from the expected run.stderr.
Actual run.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-panic-abort-nocapture/test-panic-abort-nocapture.run.stderr
error: 2 errors occurred comparing run output.
status: exit code: 2
status: exit code: 2
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-panic-abort-nocapture/a" "--test-threads=1" "--nocapture"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-panic-abort-nocapture/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-panic-abort-nocapture/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/test-panic-abort.rs stdout ----
---
- 
- 


The actual run.stdout differed from the expected run.stdout.
Actual run.stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-panic-abort/test-panic-abort.run.stdout
normalized run.stderr:
$TEST_BUILD_DIR/test-panic-abort/a: 1: $TEST_BUILD_DIR/test-panic-abort/a: Syntax error: word unexpected (expecting ")")


The actual run.stderr differed from the expected run.stderr.
The actual run.stderr differed from the expected run.stderr.
Actual run.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-panic-abort/test-panic-abort.run.stderr
error: 2 errors occurred comparing run output.
status: exit code: 2
status: exit code: 2
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-panic-abort/a" "--test-threads=1"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-panic-abort/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-panic-abort/a: Syntax error: word unexpected (expecting ")")
------------------------------------------



---
thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/armv5te-unknown-linux-gnueabi/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-armv5te-unknown-linux-gnueabi" "--mode" "ui" "--target" "armv5te-unknown-linux-gnueabi" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--pass" "check" "--nodejs" "/usr/bin/node" "--linker" "arm-linux-gnueabi-gcc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/armv5te-unknown-linux-gnueabi/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"



failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/ui --pass=check --target=armv5te-unknown-linux-gnueabi
== clock drift check ==
  local time: Sat May  9 19:35:43 UTC 2020
  network time: Sat, 09 May 2020 19:35:44 GMT
== end clock drift check ==
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/72053/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/72053/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (4139) (python)
##[section]Finishing: Finalize Job
