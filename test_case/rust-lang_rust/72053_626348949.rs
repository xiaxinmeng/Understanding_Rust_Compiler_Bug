plain
##[section]Starting: Linux x86_64-gnu-llvm-8
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 4'
Agent machine name: 'fv-az619'
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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/2326f6a8-7816-4a5d-8f4e-78710fcd1b06.sh

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
 ---> Running in 85dda28fed39
Removing intermediate container 85dda28fed39
 ---> 0a279946f71f
Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/test/ui --pass=check                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
Removing intermediate container b4c6dd304f3f
 ---> 278761b21a71
Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
 ---> Running in cbb1e6037aae
---
Successfully built c86d05765fa3
Successfully tagged rust-ci:latest
Built container sha256:c86d05765fa33e34209463fff625c4ac70b972773eecf7bd287c603277546761
Uploading finished image to https://rust-lang-ci-sccache2.s3.amazonaws.com/docker/01dc1286a5bbaf76b78ddf30062f4c7b7a18bebd152e57c676d9f6b8dee3e0c0a8f2b50cbbe8f246bd7ff55da78f136be89250a32a848774935682442d28949b
upload failed: - to s3://rust-lang-ci-sccache2/docker/01dc1286a5bbaf76b78ddf30062f4c7b7a18bebd152e57c676d9f6b8dee3e0c0a8f2b50cbbe8f246bd7ff55da78f136be89250a32a848774935682442d28949b An error occurred (InvalidAccessKeyId) when calling the CreateMultipartUpload operation: The AWS Access Key Id you provided does not exist in our records.
[CI_JOB_NAME=x86_64-gnu-llvm-8]
== clock drift check ==
  local time: Sun May 10 14:12:40 UTC 2020
  network time: Sun, 10 May 2020 14:12:40 GMT
---
   Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
   Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
   Compiling chalk-rust-ir v0.10.0
   Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
   Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
   Compiling chalk-solve v0.10.0
   Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
   Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
   Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
   Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
   Compiling chalk-rust-ir v0.10.0
   Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
   Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
   Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
   Compiling chalk-solve v0.10.0
   Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
   Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
   Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
.....................................................i.............................................. 1800/10160
.................................................................................................... 1900/10160
.......................................................................i..i......................... 2000/10160
.................................................................................................... 2100/10160
.............................................................iiiii.................................. 2200/10160
.................................................................................................... 2400/10160
.................................................................................................... 2500/10160
.................................................................................................... 2600/10160
.................................................................................................... 2700/10160
---
.................................................................................................... 5200/10160
.................................................................................................... 5300/10160
........................i........................................................................... 5400/10160
.................i.................................................................................. 5500/10160
........................ii.ii........i...i.......................................................... 5600/10160
.........................................................................i.......................... 5800/10160
.................................................................................................... 5900/10160
....................ii.....................................i........................................ 6000/10160
.................................................................................................... 6100/10160
.................................................................................................... 6100/10160
.................................................................................................... 6200/10160
.................................................................................ii...i..ii......... 6300/10160
.................................................................................................... 6500/10160
.................................................................................................... 6600/10160
.................................................................................................... 6700/10160
.................................................................................................... 6700/10160
..............i..ii................................................................................. 6800/10160
.................................................................................................... 7000/10160
....................................................................i............................... 7100/10160
.................................................................................................... 7200/10160
.................................................................................................... 7300/10160
---
.................................................................................................... 8100/10160
.................................................................................................... 8200/10160
...................................................................................i................ 8300/10160
.................................................................................................... 8400/10160
.....................................iiiiii.iiiii.i................................................. 8500/10160
.................................................................................................... 8700/10160
.................................................................................................... 8800/10160
.................................................................................................... 8900/10160
.................................................................................................... 9000/10160
---
Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 188 tests
iiii......i.............ii.i..........i...............................i..i..................i....i.. 100/188
..........i.i.i...iii..iiiiiiiiiiiiiiii.......................iii...............ii......

 finished in 5.400
Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 9 tests
iiiiiiiii

 finished in 0.155
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 115 tests
iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
...iiii.....ii.

 finished in 14.528
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

 finished in 144.786
Set({"src/libterm"}) not skipped for "bootstrap::test::Crate" -- not in ["src/tools/tidy"]
---

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

 finished in 0.933
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

 finished in 64.335
Set({"src/doc/rustdoc"}) not skipped for "bootstrap::test::RustdocBook" -- not in ["src/tools/tidy"]
doc tests for: /checkout/src/doc/rustdoc/src/advanced-features.md
---
doc tests for: /checkout/src/doc/rustc/src/targets/index.md
doc tests for: /checkout/src/doc/rustc/src/what-is-rustc.md
 finished in 3.889
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

 finished in 0.764
Build completed successfully in 1:34:35
Build completed successfully in 1:34:35
+ python2.7 ../x.py test src/test/mir-opt --pass=build --target=armv5te-unknown-linux-gnueabi
Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.27s
Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Building stage0 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
test result: ok. 96 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

 finished in 7.958
Build completed successfully in 0:01:11
+ python2.7 ../x.py test src/test/ui --pass=check --target=armv5te-unknown-linux-gnueabi
Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.22s
Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Building stage0 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
    Finished release [optimized] target(s) in 0.18s
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> armv5te-unknown-linux-gnueabi)

running 10160 tests
ii...............................i.ii.......................................F.......F............... 100/10160
.................................................................................................... 300/10160
.................................................................................................... 400/10160
.............................................................................FF.F................... 500/10160
.................................................................................i.................. 600/10160
---
....................................ii.i.............i.............................................. 1800/10160
..i....................i.i................................................i........................F 1900/10160
.......................................................................i..i......................... 2000/10160
.................................................................................................... 2100/10160
.............................................................iiiii.................................. 2200/10160
.................................................................................................... 2400/10160
.................................................................................................... 2500/10160
.................................................................................................... 2600/10160
............................................................i........i.........i.................... 2700/10160
---
.........FF.............................................F........................................... 5500/10160
..F......................................i.......................................................... 5600/10160
..................................................................i................................i 5700/10160
.........................................................................i.......................... 5800/10160
...............................i.ii...............F.........................FF..FFFFF............... 5900/10160
....................ii.....................................i........................................ 6000/10160
..................................................................FFFFF...........................FF 6100/10160
...F.F...................F.......................................................................... 6200/10160
.....FFF.F........FFFF...FFF.....................................................ii...i..ii......... 6300/10160
.....F.............................................................................................. 6500/10160
.................................................................................................... 6600/10160
.................................................................................................... 6700/10160
.................................................................................................... 6700/10160
...........i..i..ii.......................F.................F..F....FFF..F.....F.................... 6800/10160
.................................................................................................... 6900/10160
.........................................................................FF.FF.....FFF.FF.F...FFFFFF 7000/10160
FF..FFFFFF.F....FF..................................................i............................... 7100/10160
.................................................................................................... 7300/10160
..........i......................................................................................... 7400/10160
.................................................................................................... 7500/10160
.................................................................................................... 7600/10160
.................................................................................................... 7600/10160
.................................................................................................... 7700/10160
.........................i........i.....................i...........F............................... 7800/10160
.................................................................................................... 7900/10160
.................................................................................................... 8000/10160
.................................................................................................... 8100/10160
................................................................FFFF................................ 8200/10160
...................................................................................i................ 8300/10160
.............................................................iiiii.................................. 8400/10160
.....................................iiiiii.iiiii.i................................................. 8500/10160
.................................................................................................... 8700/10160
.................................................................................................... 8800/10160
........................................F........................................................... 8900/10160
....................................................................F............................... 9000/10160
....................................................................F............................... 9000/10160
.................................................................................................... 9100/10160
...............................ii...............................F...F............................... 9200/10160
................FF.................................................................................. 9300/10160
.................................................................................................... 9500/10160
...............................................................i.................................... 9600/10160
.................................................................................................... 9700/10160
.................................................................................................... 9800/10160
---

------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/array-slice-vec/bounds-check-no-overflow/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/array-slice-vec/bounds-check-no-overflow/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/array-slice-vec/dst-raw-slice.rs stdout ----
---

------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/array-slice-vec/dst-raw-slice/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/array-slice-vec/dst-raw-slice/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/async-await/issues/issue-65419/issue-65419-async-fn-resume-after-completion.rs stdout ----
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


---- [ui] ui/binop/binop-fail-3.rs stdout ----
---- [ui] ui/binop/binop-fail-3.rs stdout ----

error: error pattern 'quux' not found!
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/binop-fail-3/a"
stdout:
------------------------------------------


------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/binop-fail-3/a: 6: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/binop-fail-3/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/binop/binop-panic.rs stdout ----
---- [ui] ui/binop/binop-panic.rs stdout ----

error: error pattern 'quux' not found!
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/binop-panic/a"
stdout:
------------------------------------------


------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/binop-panic/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/binop-panic/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/borrowck/borrowck-local-borrow.rs#mir stdout ----
---- [ui] ui/borrowck/borrowck-local-borrow.rs#mir stdout ----

error in revision `mir`: error pattern 'panic 1' not found!
status: exit code: 2
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-local-borrow.mir/a"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-local-borrow.mir/a: 6: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-local-borrow.mir/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/borrowck/borrowck-local-borrow.rs#migrate stdout ----
---- [ui] ui/borrowck/borrowck-local-borrow.rs#migrate stdout ----

error in revision `migrate`: error pattern 'panic 1' not found!
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-local-borrow.migrate/a"
stdout:
------------------------------------------


------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-local-borrow.migrate/a: 6: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-local-borrow.migrate/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/closures/diverging-closure.rs stdout ----
---- [ui] ui/closures/diverging-closure.rs stdout ----

error: error pattern 'oops' not found!
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/diverging-closure/a"
stdout:
------------------------------------------


------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/diverging-closure/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/diverging-closure/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/consts/promoted_div_by_zero.rs stdout ----
---

------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/promoted_div_by_zero/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/promoted_div_by_zero/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/fn/expr-fn-panic.rs stdout ----
---

------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fn/expr-fn-panic/a: 6: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fn/expr-fn-panic/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/generator/generator-resume-after-panic.rs stdout ----
---

------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/generator-resume-after-panic/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/generator-resume-after-panic/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/hashmap/hashmap-capacity-overflow.rs stdout ----
---- [ui] ui/hashmap/hashmap-capacity-overflow.rs stdout ----

error: error pattern 'capacity overflow' not found!
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hashmap/hashmap-capacity-overflow/a"
stdout:
------------------------------------------


------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hashmap/hashmap-capacity-overflow/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hashmap/hashmap-capacity-overflow/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/if/expr-if-panic-fn.rs stdout ----
---

------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/if/expr-if-panic-fn/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/if/expr-if-panic-fn/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/if/expr-if-panic.rs stdout ----
---

------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/if/expr-if-panic/a: 6: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/if/expr-if-panic/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/if/if-check-panic.rs stdout ----
---- [ui] ui/if/if-check-panic.rs stdout ----

error: error pattern 'Number is odd' not found!
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/if/if-check-panic/a"
stdout:
------------------------------------------


------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/if/if-check-panic/a: 6: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/if/if-check-panic/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/if/if-cond-bot.rs stdout ----
---- [ui] ui/if/if-cond-bot.rs stdout ----

error: error pattern 'quux' not found!
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/if/if-cond-bot/a"
stdout:
------------------------------------------


------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/if/if-cond-bot/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/if/if-cond-bot/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/imports/glob-use-std.rs stdout ----
---- [ui] ui/imports/glob-use-std.rs stdout ----

error: error pattern 'panic works' not found!
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/glob-use-std/a"
stdout:
------------------------------------------


------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/glob-use-std/a: 6: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/glob-use-std/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/issues/issue-12920.rs stdout ----
---

------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12920/a: 6: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12920/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/issues/issue-13202.rs stdout ----
---

------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-13202/a: 6: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-13202/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/issues/issue-18576.rs stdout ----
---- [ui] ui/issues/issue-18576.rs stdout ----

error: error pattern 'stop' not found!
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-18576/a"
stdout:
------------------------------------------


------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-18576/a: 6: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-18576/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/issues/issue-20971.rs stdout ----
---- [ui] ui/issues/issue-20971.rs stdout ----

error: error pattern 'Hello, world!' not found!
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-20971/a"
stdout:
------------------------------------------


------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-20971/a: 6: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-20971/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/issues/issue-23354-2.rs stdout ----
---- [ui] ui/issues/issue-23354-2.rs stdout ----

error: error pattern 'panic evaluated' not found!
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23354-2/a"
stdout:
------------------------------------------


------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23354-2/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23354-2/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/issues/issue-23354.rs stdout ----
---- [ui] ui/issues/issue-23354.rs stdout ----

error: error pattern 'panic evaluated' not found!
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23354/a"
stdout:
------------------------------------------


------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23354/a: 6: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23354/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/issues/issue-2444.rs stdout ----
---

------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-2444/a: 6: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-2444/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/issues/issue-2470-bounds-check-overflow.rs stdout ----
---

------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-2470-bounds-check-overflow/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-2470-bounds-check-overflow/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/issues/issue-2761.rs stdout ----
---- [ui] ui/issues/issue-2761.rs stdout ----

error: error pattern 'custom message' not found!
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-2761/a"
stdout:
------------------------------------------


------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-2761/a: 6: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-2761/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/issues/issue-28934.rs stdout ----
---

------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-28934/a: 6: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-28934/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/issues/issue-29798.rs stdout ----
---

------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-29798/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-29798/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/issues/issue-3029.rs stdout ----
---- [ui] ui/issues/issue-3029.rs stdout ----

error: error pattern 'so long' not found!
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-3029/a"
stdout:
------------------------------------------


------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-3029/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-3029/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/issues/issue-30380.rs stdout ----
---- [ui] ui/issues/issue-30380.rs stdout ----

error: error pattern 'panicking destructors ftw!' not found!
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-30380/a"
stdout:
------------------------------------------


------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-30380/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-30380/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/issues/issue-44216-add-instant.rs stdout ----
---

------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-44216-add-instant/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-44216-add-instant/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/issues/issue-44216-add-system-time.rs stdout ----
---

------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-44216-add-system-time/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-44216-add-system-time/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/issues/issue-44216-sub-instant.rs stdout ----
---

------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-44216-sub-instant/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-44216-sub-instant/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/issues/issue-44216-sub-system-time.rs stdout ----
---

------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-44216-sub-system-time/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-44216-sub-system-time/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/issues/issue-51345-2.rs stdout ----
---- [ui] ui/issues/issue-51345-2.rs stdout ----

error: error pattern ' thread 'main' panicked at 'explicit panic'' not found!
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-51345-2/a"
stdout:
------------------------------------------


------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-51345-2/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-51345-2/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/issues/issue-6458-1.rs stdout ----
---

------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-6458-1/a: 6: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-6458-1/a: Syntax error: word unexpected (expecting ")")
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


---- [ui] ui/issues/issue-811.rs stdout ----
---- [ui] ui/issues/issue-811.rs stdout ----

error: error pattern 'quux' not found!
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-811/a"
stdout:
------------------------------------------


------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-811/a: 6: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-811/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/issues/issue-948.rs stdout ----
---- [ui] ui/issues/issue-948.rs stdout ----

error: error pattern 'beep boop' not found!
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-948/a"
stdout:
------------------------------------------


------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-948/a: 6: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-948/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/loops/for-each-loop-panic.rs stdout ----
---- [ui] ui/loops/for-each-loop-panic.rs stdout ----

error: error pattern 'moop' not found!
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/loops/for-each-loop-panic/a"
stdout:
------------------------------------------


------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/loops/for-each-loop-panic/a: 6: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/loops/for-each-loop-panic/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/macros/assert-as-macro.rs stdout ----
---- [ui] ui/macros/assert-as-macro.rs stdout ----

error: error pattern 'assertion failed: 1 == 2' not found!
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/assert-as-macro/a"
stdout:
------------------------------------------


------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/assert-as-macro/a: 6: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/assert-as-macro/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/macros/assert-eq-macro-panic.rs stdout ----
---- [ui] ui/macros/assert-eq-macro-panic.rs stdout ----

error: error pattern 'assertion failed: `(left == right)`' not found!

error: error pattern ' left: `14`' not found!

error: error pattern 'right: `15`' not found!
error: multiple error patterns not found
status: exit code: 2
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/assert-eq-macro-panic/a"
stdout:
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/assert-eq-macro-panic/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/assert-eq-macro-panic/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/macros/assert-macro-fmt.rs stdout ----
---- [ui] ui/macros/assert-macro-fmt.rs stdout ----

error: error pattern 'panicked at 'test-assert-fmt 42 rust'' not found!
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/assert-macro-fmt/a"
stdout:
------------------------------------------


------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/assert-macro-fmt/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/assert-macro-fmt/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/macros/assert-macro-explicit.rs stdout ----
---- [ui] ui/macros/assert-macro-explicit.rs stdout ----

error: error pattern 'panicked at 'assertion failed: false'' not found!
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/assert-macro-explicit/a"
stdout:
------------------------------------------


------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/assert-macro-explicit/a: 6: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/assert-macro-explicit/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/macros/assert-macro-static.rs stdout ----
---- [ui] ui/macros/assert-macro-static.rs stdout ----

error: error pattern 'panicked at 'test-assert-static'' not found!
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/assert-macro-static/a"
stdout:
------------------------------------------


------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/assert-macro-static/a: 6: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/assert-macro-static/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/macros/assert-macro-owned.rs stdout ----
---- [ui] ui/macros/assert-macro-owned.rs stdout ----

error: error pattern 'panicked at 'test-assert-owned'' not found!
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/assert-macro-owned/a"
stdout:
------------------------------------------


------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/assert-macro-owned/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/assert-macro-owned/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/macros/assert-ne-macro-panic.rs stdout ----
---- [ui] ui/macros/assert-ne-macro-panic.rs stdout ----

error: error pattern 'assertion failed: `(left != right)`' not found!

error: error pattern ' left: `14`' not found!

error: error pattern 'right: `14`' not found!
error: multiple error patterns not found
status: exit code: 2
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/assert-ne-macro-panic/a"
stdout:
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/assert-ne-macro-panic/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/assert-ne-macro-panic/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/macros/unimplemented-macro-panic.rs stdout ----
---- [ui] ui/macros/unimplemented-macro-panic.rs stdout ----

error: error pattern 'not implemented' not found!
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/unimplemented-macro-panic/a"
stdout:
------------------------------------------


------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/unimplemented-macro-panic/a: 6: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/unimplemented-macro-panic/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/macros/unreachable-fmt-msg.rs stdout ----
---- [ui] ui/macros/unreachable-fmt-msg.rs stdout ----

error: error pattern 'internal error: entered unreachable code: 6 is not prime' not found!
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/unreachable-fmt-msg/a"
stdout:
------------------------------------------


------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/unreachable-fmt-msg/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/unreachable-fmt-msg/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/macros/unreachable-macro-panic.rs stdout ----
---

------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/unreachable-macro-panic/a: 4: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/unreachable-macro-panic/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/macros/unreachable-static-msg.rs stdout ----
---- [ui] ui/macros/unreachable-static-msg.rs stdout ----

error: error pattern 'internal error: entered unreachable code: uhoh' not found!
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/unreachable-static-msg/a"
stdout:
------------------------------------------


------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/unreachable-static-msg/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/unreachable-static-msg/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/macros/unreachable.rs stdout ----
---

------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/unreachable/a: 6: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/unreachable/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/match/expr-match-panic-fn.rs stdout ----
---

------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/match/expr-match-panic-fn/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/match/expr-match-panic-fn/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/match/expr-match-panic.rs stdout ----
---

------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/match/expr-match-panic/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/match/expr-match-panic/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/match/match-bot-panic.rs stdout ----
---

------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/match/match-bot-panic/a: 6: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/match/match-bot-panic/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/match/match-disc-bot.rs stdout ----
---- [ui] ui/match/match-disc-bot.rs stdout ----

error: error pattern 'quux' not found!
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/match/match-disc-bot/a"
stdout:
------------------------------------------


------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/match/match-disc-bot/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/match/match-disc-bot/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/match/match-wildcards.rs stdout ----
---- [ui] ui/match/match-wildcards.rs stdout ----

error: error pattern 'squirrelcupcake' not found!
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/match/match-wildcards/a"
stdout:
------------------------------------------


------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/match/match-wildcards/a: 6: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/match/match-wildcards/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/mir/mir_codegen_calls_converging_drops.rs stdout ----
---- [ui] ui/mir/mir_codegen_calls_converging_drops.rs stdout ----

error: error pattern 'converging_fn called' not found!

error: error pattern '0 dropped' not found!
error: error pattern 'exit' not found!

error: multiple error patterns not found
status: exit code: 2
status: exit code: 2
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/mir_codegen_calls_converging_drops/a"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/mir_codegen_calls_converging_drops/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/mir_codegen_calls_converging_drops/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/mir/mir_codegen_calls_converging_drops_2.rs stdout ----
---- [ui] ui/mir/mir_codegen_calls_converging_drops_2.rs stdout ----

error: error pattern 'complex called' not found!

error: error pattern 'dropped' not found!
error: error pattern 'exit' not found!

error: multiple error patterns not found
status: exit code: 2
status: exit code: 2
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/mir_codegen_calls_converging_drops_2/a"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/mir_codegen_calls_converging_drops_2/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/mir_codegen_calls_converging_drops_2/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/mir/mir_codegen_calls_diverging.rs stdout ----
---- [ui] ui/mir/mir_codegen_calls_diverging.rs stdout ----

error: error pattern 'diverging_fn called' not found!
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/mir_codegen_calls_diverging/a"
stdout:
------------------------------------------


------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/mir_codegen_calls_diverging/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/mir_codegen_calls_diverging/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/mir/mir_codegen_calls_diverging_drops.rs stdout ----
---- [ui] ui/mir/mir_codegen_calls_diverging_drops.rs stdout ----

error: error pattern 'diverging_fn called' not found!

error: error pattern '0 dropped' not found!
error: multiple error patterns not found
status: exit code: 2
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/mir_codegen_calls_diverging_drops/a"
stdout:
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/mir_codegen_calls_diverging_drops/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/mir_codegen_calls_diverging_drops/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/mir/mir_drop_panics.rs stdout ----
---- [ui] ui/mir/mir_drop_panics.rs stdout ----

error: error pattern 'panic 1' not found!

error: error pattern 'drop 2' not found!
error: multiple error patterns not found
status: exit code: 2
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/mir_drop_panics/a"
stdout:
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/mir_drop_panics/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/mir_drop_panics/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/mir/mir_dynamic_drops_1.rs stdout ----
---- [ui] ui/mir/mir_dynamic_drops_1.rs stdout ----

error: error pattern 'drop 1' not found!

error: error pattern 'drop 2' not found!
error: multiple error patterns not found
status: exit code: 2
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/mir_dynamic_drops_1/a"
stdout:
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/mir_dynamic_drops_1/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/mir_dynamic_drops_1/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/mir/mir_dynamic_drops_2.rs stdout ----
---- [ui] ui/mir/mir_dynamic_drops_2.rs stdout ----

error: error pattern 'drop 1' not found!
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/mir_dynamic_drops_2/a"
stdout:
------------------------------------------


------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/mir_dynamic_drops_2/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/mir_dynamic_drops_2/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/mir/mir_dynamic_drops_3.rs stdout ----
---- [ui] ui/mir/mir_dynamic_drops_3.rs stdout ----

error: error pattern 'unwind happens' not found!

error: error pattern 'drop 3' not found!

error: error pattern 'drop 2' not found!

error: error pattern 'drop 1' not found!
error: multiple error patterns not found
status: exit code: 2
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/mir_dynamic_drops_3/a"
stdout:
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/mir_dynamic_drops_3/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/mir_dynamic_drops_3/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/mir/mir_indexing_oob_1.rs stdout ----
---

------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/mir_indexing_oob_1/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/mir_indexing_oob_1/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/mir/mir_indexing_oob_2.rs stdout ----
---

------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/mir_indexing_oob_2/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/mir_indexing_oob_2/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/mir/mir_indexing_oob_3.rs stdout ----
---

------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/mir_indexing_oob_3/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/mir_indexing_oob_3/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/never_type/return-never-coerce.rs stdout ----
---- [ui] ui/never_type/return-never-coerce.rs stdout ----

error: error pattern 'aah!' not found!
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/never_type/return-never-coerce/a"
stdout:
------------------------------------------


------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/never_type/return-never-coerce/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/never_type/return-never-coerce/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/numbers-arithmetic/divide-by-zero.rs stdout ----
---- [ui] ui/numbers-arithmetic/divide-by-zero.rs stdout ----

error: error pattern 'attempt to divide by zero' not found!
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numbers-arithmetic/divide-by-zero/a"
stdout:
------------------------------------------


------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numbers-arithmetic/divide-by-zero/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numbers-arithmetic/divide-by-zero/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/numbers-arithmetic/mod-zero.rs stdout ----
---- [ui] ui/numbers-arithmetic/mod-zero.rs stdout ----

error: error pattern 'attempt to calculate the remainder with a divisor of zero' not found!
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numbers-arithmetic/mod-zero/a"
stdout:
------------------------------------------


------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numbers-arithmetic/mod-zero/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numbers-arithmetic/mod-zero/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/numbers-arithmetic/overflowing-add.rs stdout ----
---- [ui] ui/numbers-arithmetic/overflowing-add.rs stdout ----

error: error pattern 'thread 'main' panicked at 'attempt to add with overflow'' not found!
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numbers-arithmetic/overflowing-add/a"
stdout:
------------------------------------------


------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numbers-arithmetic/overflowing-add/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numbers-arithmetic/overflowing-add/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/numbers-arithmetic/overflowing-mul.rs stdout ----
---- [ui] ui/numbers-arithmetic/overflowing-mul.rs stdout ----

error: error pattern 'thread 'main' panicked at 'attempt to multiply with overflow'' not found!
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numbers-arithmetic/overflowing-mul/a"
stdout:
------------------------------------------


------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numbers-arithmetic/overflowing-mul/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numbers-arithmetic/overflowing-mul/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/numbers-arithmetic/overflowing-neg.rs stdout ----
---- [ui] ui/numbers-arithmetic/overflowing-neg.rs stdout ----

error: error pattern 'thread 'main' panicked at 'attempt to negate with overflow'' not found!
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numbers-arithmetic/overflowing-neg/a"
stdout:
------------------------------------------


------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numbers-arithmetic/overflowing-neg/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numbers-arithmetic/overflowing-neg/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/numbers-arithmetic/overflowing-pow-signed.rs stdout ----
---- [ui] ui/numbers-arithmetic/overflowing-pow-signed.rs stdout ----

error: error pattern 'thread 'main' panicked at 'attempt to multiply with overflow'' not found!
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numbers-arithmetic/overflowing-pow-signed/a"
stdout:
------------------------------------------


------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numbers-arithmetic/overflowing-pow-signed/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numbers-arithmetic/overflowing-pow-signed/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/numbers-arithmetic/overflowing-pow-unsigned.rs stdout ----
---- [ui] ui/numbers-arithmetic/overflowing-pow-unsigned.rs stdout ----

error: error pattern 'thread 'main' panicked at 'attempt to multiply with overflow'' not found!
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numbers-arithmetic/overflowing-pow-unsigned/a"
stdout:
------------------------------------------


------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numbers-arithmetic/overflowing-pow-unsigned/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numbers-arithmetic/overflowing-pow-unsigned/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/numbers-arithmetic/overflowing-sub.rs stdout ----
---- [ui] ui/numbers-arithmetic/overflowing-sub.rs stdout ----

error: error pattern 'thread 'main' panicked at 'attempt to subtract with overflow'' not found!
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numbers-arithmetic/overflowing-sub/a"
stdout:
------------------------------------------


------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numbers-arithmetic/overflowing-sub/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numbers-arithmetic/overflowing-sub/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/panic-runtime/unwind-interleaved.rs stdout ----
---

------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/unwind-interleaved/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/unwind-interleaved/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/panic-runtime/unwind-rec.rs stdout ----
---

------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/unwind-rec/a: 6: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/unwind-rec/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/panic-runtime/unwind-rec2.rs stdout ----
---

------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/unwind-rec2/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/unwind-rec2/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/panic-runtime/unwind-unique.rs stdout ----
---

------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/unwind-unique/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/unwind-unique/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/panics/args-panic.rs stdout ----
---- [ui] ui/panics/args-panic.rs stdout ----

error: error pattern 'meep' not found!
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/args-panic/a"
stdout:
------------------------------------------


------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/args-panic/a: 7: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/args-panic/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/panics/doublepanic.rs stdout ----
---- [ui] ui/panics/doublepanic.rs stdout ----

error: error pattern 'One' not found!
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/doublepanic/a"
stdout:
------------------------------------------


------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/doublepanic/a: 6: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/doublepanic/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/panics/explicit-panic-msg.rs stdout ----
---- [ui] ui/panics/explicit-panic-msg.rs stdout ----

error: error pattern 'wooooo' not found!
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/explicit-panic-msg/a"
stdout:
------------------------------------------


------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/explicit-panic-msg/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/explicit-panic-msg/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/panics/main-panic.rs stdout ----
---- [ui] ui/panics/main-panic.rs stdout ----

error: error pattern 'thread 'main' panicked at' not found!
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/main-panic/a"
stdout:
------------------------------------------


------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/main-panic/a: 6: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/main-panic/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/panics/fmt-panic.rs stdout ----
---- [ui] ui/panics/fmt-panic.rs stdout ----

error: error pattern 'meh' not found!
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/fmt-panic/a"
stdout:
------------------------------------------


------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/fmt-panic/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/fmt-panic/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/panics/panic-arg.rs stdout ----
---- [ui] ui/panics/panic-arg.rs stdout ----

error: error pattern 'woe' not found!
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/panic-arg/a"
stdout:
------------------------------------------


------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/panic-arg/a: 7: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/panic-arg/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/panics/panic-macro-any-wrapped.rs stdout ----
---- [ui] ui/panics/panic-macro-any-wrapped.rs stdout ----

error: error pattern 'panicked at 'Box<Any>'' not found!
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/panic-macro-any-wrapped/a"
stdout:
------------------------------------------


------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/panic-macro-any-wrapped/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/panic-macro-any-wrapped/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/panics/panic-macro-any.rs stdout ----
---- [ui] ui/panics/panic-macro-any.rs stdout ----

error: error pattern 'panicked at 'Box<Any>'' not found!
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/panic-macro-any/a"
stdout:
------------------------------------------


------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/panic-macro-any/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/panic-macro-any/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/panics/panic-macro-fmt.rs stdout ----
---- [ui] ui/panics/panic-macro-fmt.rs stdout ----

error: error pattern 'panicked at 'test-fail-fmt 42 rust'' not found!
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/panic-macro-fmt/a"
stdout:
------------------------------------------


------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/panic-macro-fmt/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/panic-macro-fmt/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/panics/panic-macro-explicit.rs stdout ----
---- [ui] ui/panics/panic-macro-explicit.rs stdout ----

error: error pattern 'panicked at 'explicit panic'' not found!
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/panic-macro-explicit/a"
stdout:
------------------------------------------


------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/panic-macro-explicit/a: 6: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/panic-macro-explicit/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/panics/panic-macro-owned.rs stdout ----
---- [ui] ui/panics/panic-macro-owned.rs stdout ----

error: error pattern 'panicked at 'test-fail-owned'' not found!
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/panic-macro-owned/a"
stdout:
------------------------------------------


------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/panic-macro-owned/a: 6: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/panic-macro-owned/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/panics/panic-macro-static.rs stdout ----
---- [ui] ui/panics/panic-macro-static.rs stdout ----

error: error pattern 'panicked at 'test-fail-static'' not found!
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/panic-macro-static/a"
stdout:
------------------------------------------


------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/panic-macro-static/a: 6: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/panic-macro-static/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/panics/panic-main.rs stdout ----
---- [ui] ui/panics/panic-main.rs stdout ----

error: error pattern 'moop' not found!
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/panic-main/a"
stdout:
------------------------------------------


------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/panic-main/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/panic-main/a: Syntax error: "(" unexpected
------------------------------------------


---- [ui] ui/panics/panic-parens.rs stdout ----
---- [ui] ui/panics/panic-parens.rs stdout ----

error: error pattern 'oops' not found!
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/panic-parens/a"
stdout:
------------------------------------------


------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/panic-parens/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/panic-parens/a: Syntax error: Unterminated quoted string
------------------------------------------


---- [ui] ui/panics/panic-set-handler.rs stdout ----
---- [ui] ui/panics/panic-set-handler.rs stdout ----

error: error pattern 'greetings from the panic handler' not found!
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/panic-set-handler/a"
stdout:
------------------------------------------


------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/panic-set-handler/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/panic-set-handler/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/panics/panic-set-unset-handler.rs stdout ----
---- [ui] ui/panics/panic-set-unset-handler.rs stdout ----

error: error pattern 'thread 'main' panicked at 'foobar'' not found!
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/panic-set-unset-handler/a"
stdout:
------------------------------------------


------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/panic-set-unset-handler/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/panic-set-unset-handler/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/panics/panic-take-handler-nop.rs stdout ----
---- [ui] ui/panics/panic-take-handler-nop.rs stdout ----

error: error pattern 'thread 'main' panicked at 'foobar'' not found!
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/panic-take-handler-nop/a"
stdout:
------------------------------------------


------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/panic-take-handler-nop/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/panic-take-handler-nop/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/panics/panic-task-name-none.rs stdout ----
---- [ui] ui/panics/panic-task-name-none.rs stdout ----

error: error pattern 'thread '<unnamed>' panicked at 'test'' not found!
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/panic-task-name-none/a"
stdout:
------------------------------------------


------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/panic-task-name-none/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/panic-task-name-none/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/panics/panic-task-name-owned.rs stdout ----
---- [ui] ui/panics/panic-task-name-owned.rs stdout ----

error: error pattern 'thread 'owned name' panicked at 'test'' not found!
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/panic-task-name-owned/a"
stdout:
------------------------------------------


------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/panic-task-name-owned/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/panic-task-name-owned/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/panics/panic.rs stdout ----
---- [ui] ui/panics/panic.rs stdout ----

error: error pattern '1 == 2' not found!
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/panic/a"
stdout:
------------------------------------------


------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/panic/a: 6: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/panic/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/panics/result-get-panic.rs stdout ----
---- [ui] ui/panics/result-get-panic.rs stdout ----

error: error pattern 'called `Result::unwrap()` on an `Err` value' not found!
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/result-get-panic/a"
stdout:
------------------------------------------


------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/result-get-panic/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/result-get-panic/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/panics/while-body-panics.rs stdout ----
---- [ui] ui/panics/while-body-panics.rs stdout ----

error: error pattern 'quux' not found!
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/while-body-panics/a"
stdout:
------------------------------------------


------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/while-body-panics/a: 6: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/while-body-panics/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/panics/while-panic.rs stdout ----
---- [ui] ui/panics/while-panic.rs stdout ----

error: error pattern 'giraffe' not found!
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/while-panic/a"
stdout:
------------------------------------------


------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/while-panic/a: 6: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/while-panic/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/process/tls-exit-status.rs stdout ----
---- [ui] ui/process/tls-exit-status.rs stdout ----

error: error pattern 'nonzero' not found!
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/process/tls-exit-status/a"
stdout:
------------------------------------------


------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/process/tls-exit-status/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/process/tls-exit-status/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/rfc-1937-termination-trait/termination-trait-for-never.rs stdout ----
---- [ui] ui/rfc-1937-termination-trait/termination-trait-for-never.rs stdout ----

error: error pattern 'oh, dear' not found!
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/termination-trait-for-never/a"
stdout:
------------------------------------------


------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/termination-trait-for-never/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/termination-trait-for-never/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/rfc-1937-termination-trait/termination-trait-for-box-dyn-error.rs stdout ----
---- [ui] ui/rfc-1937-termination-trait/termination-trait-for-box-dyn-error.rs stdout ----

error: error pattern 'returned Box<dyn Error> from main()' not found!
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/termination-trait-for-box-dyn-error/a"
stdout:
------------------------------------------


------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/termination-trait-for-box-dyn-error/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/termination-trait-for-box-dyn-error/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/rfc-1937-termination-trait/termination-trait-for-str.rs stdout ----
---- [ui] ui/rfc-1937-termination-trait/termination-trait-for-str.rs stdout ----

error: error pattern ' An error message for you' not found!
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/termination-trait-for-str/a"
stdout:
------------------------------------------


------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/termination-trait-for-str/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/termination-trait-for-str/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/rfc-1937-termination-trait/termination-trait-for-result-box-error_err.rs stdout ----
---- [ui] ui/rfc-1937-termination-trait/termination-trait-for-result-box-error_err.rs stdout ----

error: error pattern 'returned Box<Error> from main()' not found!
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/termination-trait-for-result-box-error_err/a"
stdout:
------------------------------------------


------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/termination-trait-for-result-box-error_err/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/termination-trait-for-result-box-error_err/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/str/str-overrun.rs stdout ----
---

------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/str/str-overrun/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/str/str-overrun/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/structs/rhs-type.rs stdout ----
---- [ui] ui/structs/rhs-type.rs stdout ----

error: error pattern 'bye' not found!
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/structs/rhs-type/a"
stdout:
------------------------------------------


------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/structs/rhs-type/a: 6: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/structs/rhs-type/a: Syntax error: word unexpected (expecting ")")
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


---- [ui] ui/threads-sendsync/task-spawn-barefn.rs stdout ----
---- [ui] ui/threads-sendsync/task-spawn-barefn.rs stdout ----

error: error pattern 'Ensure that the child thread runs by panicking' not found!
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/threads-sendsync/task-spawn-barefn/a"
stdout:
------------------------------------------


------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/threads-sendsync/task-spawn-barefn/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/threads-sendsync/task-spawn-barefn/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/threads-sendsync/test-tasks-invalid-value.rs stdout ----
---- [ui] ui/threads-sendsync/test-tasks-invalid-value.rs stdout ----

error: error pattern 'should be a positive integer' not found!
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/threads-sendsync/test-tasks-invalid-value/a"
stdout:
------------------------------------------


------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/threads-sendsync/test-tasks-invalid-value/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/threads-sendsync/test-tasks-invalid-value/a: Syntax error: word unexpected (expecting ")")
------------------------------------------


---- [ui] ui/vec/vec-overrun.rs stdout ----
---

------------------------------------------
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/vec/vec-overrun/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/vec/vec-overrun/a: Syntax error: word unexpected (expecting ")")
------------------------------------------



---
test result: FAILED. 9949 passed; 119 failed; 92 ignored; 0 measured; 0 filtered out



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/armv5te-unknown-linux-gnueabi/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-armv5te-unknown-linux-gnueabi" "--mode" "ui" "--target" "armv5te-unknown-linux-gnueabi" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--pass" "check" "--nodejs" "/usr/bin/node" "--linker" "arm-linux-gnueabi-gcc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/armv5te-unknown-linux-gnueabi/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"



failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/ui --pass=check --target=armv5te-unknown-linux-gnueabi
thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
== clock drift check ==
  local time: Sun May 10 15:56:36 UTC 2020
  local time: Sun May 10 15:56:36 UTC 2020
  network time: Sun, 10 May 2020 15:56:36 GMT
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
Terminate orphan process: pid (3804) (python)
##[section]Finishing: Finalize Job
