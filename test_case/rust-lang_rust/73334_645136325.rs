plain
##[section]Starting: Linux x86_64-gnu-llvm-8
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 4'
Agent machine name: 'fv-az578'
Current agent version: '2.170.1'
##[group]Operating System
16.04.6
LTS
LTS
##[endgroup]
##[group]Virtual Environment
Environment: ubuntu-16.04
Version: 20200604.1
Included Software: https://github.com/actions/virtual-environments/blob/ubuntu16/20200604.1/images/linux/Ubuntu1604-README.md
##[endgroup]
Agent running as: 'vsts'
Prepare build directory.
Set build variables.
Download all required tasks.
Download all required tasks.
Downloading task: Bash (3.163.3)
Checking job knob settings.
   Knob: AgentToolsDirectory = /opt/hostedtoolcache Source: ${AGENT_TOOLSDIRECTORY} 
   Knob: AgentPerflog = /home/vsts/perflog Source: ${VSTS_AGENT_PERFLOG} 
Start tracking orphan processes.
##[section]Finishing: Initialize job
##[section]Starting: Configure Job Name
==============================================================================
---
========================== Starting Command Output ===========================
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/818bf3cf-f532-49e7-9c2b-e680089f2a37.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/73334/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/73334/merge:refs/remotes/pull/73334/merge
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
 ---> Running in ab31b7cdb748
Removing intermediate container ab31b7cdb748
 ---> 0357870531b8
Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
Removing intermediate container e54c5e90a2ea
 ---> 95ebefbd6e5c
Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
 ---> Running in 493ca100e0e7
---
 ---> a8baf24e9197
Successfully built a8baf24e9197
Successfully tagged rust-ci:latest
Built container sha256:a8baf24e9197bd7f82dc842c306c8885971058d2d996eb88693a5f833a4acade
Uploading finished image to https://ci-caches.rust-lang.org/docker/87a645642495c6f9e906b1a36380bc6b3ad356e76eb7f8bdd34e85f4cbbfc6dcf3af6863e5d1fd0bd7e6ee6a4366f910edc55d64e7aa71fdca1219a85ff1bec8
upload failed: - to s3://rust-lang-ci-sccache2/docker/87a645642495c6f9e906b1a36380bc6b3ad356e76eb7f8bdd34e85f4cbbfc6dcf3af6863e5d1fd0bd7e6ee6a4366f910edc55d64e7aa71fdca1219a85ff1bec8 An error occurred (InvalidAccessKeyId) when calling the CreateMultipartUpload operation: The AWS Access Key Id you provided does not exist in our records.
[CI_JOB_NAME=x86_64-gnu-llvm-8]
== clock drift check ==
  local time: Wed Jun 17 03:11:01 UTC 2020
  network time: Wed, 17 Jun 2020 03:11:01 GMT
---
   Compiling rustc_parse_format v0.0.0 (/checkout/src/librustc_parse_format)
   Compiling chalk-rust-ir v0.10.0
   Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
   Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
   Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
   Compiling chalk-solve v0.10.0
   Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
   Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
   Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
   Compiling rustc_parse_format v0.0.0 (/checkout/src/librustc_parse_format)
   Compiling chalk-rust-ir v0.10.0
   Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
   Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
   Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
   Compiling chalk-solve v0.10.0
   Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
   Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
   Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
.................................................................................................... 1900/10320
.................................................................................................... 2000/10320
...............i..i................................................................................. 2100/10320
.................................................................................................... 2200/10320
.....iiiii.......................................................................................... 2300/10320
.................................................................................................... 2500/10320
.................................................................................................... 2600/10320
.................................................................................................... 2700/10320
.................................................................................................... 2800/10320
---
.................................................................................................... 6000/10320
.......ii.....................................i..................................................... 6100/10320
.................................................................................................... 6200/10320
.................................................................................................... 6300/10320
......................................................................ii...i..ii...........i........ 6400/10320
.................................................................................................... 6600/10320
.................................................................................................... 6700/10320
.................................................................................................... 6800/10320
.................................................................................................... 6800/10320
....i..ii........................................................................................... 6900/10320
.................................................................................................... 7100/10320
............................................................i....................................... 7200/10320
.................................................................................................... 7300/10320
.................................................................................................... 7400/10320
---
...............................................................................F.................... 8200/10320
.................................................................................................... 8300/10320
.................................................................................................... 8400/10320
..i................................................................................................. 8500/10320
........................................................iiiiii.iiiiii.i............................. 8600/10320
.............i...................................................................................... 8800/10320
.................................................................................................... 8900/10320
.................................................................................................... 9000/10320
.................................................................................................... 9100/10320
---

---- [ui] ui/repeat_count.rs stdout ----
diff of stderr:

40 LL |     let f = [0; -4_isize];
41    |                 ^^^^^^^^ expected `usize`, found `isize`
- note: `-4_isize` cannot fit into type `usize`
-   --> $DIR/repeat_count.rs:19:17
-    |
-    |
- LL |     let f = [0; -4_isize];
+    = note: `-4_isize` cannot fit into type `usize`
48 
49 error[E0308]: mismatched types
50   --> $DIR/repeat_count.rs:22:23
50   --> $DIR/repeat_count.rs:22:23

52 LL |     let f = [0_usize; -1_isize];
53    |                       ^^^^^^^^ expected `usize`, found `isize`
- note: `-1_isize` cannot fit into type `usize`
-   --> $DIR/repeat_count.rs:22:23
-    |
-    |
- LL |     let f = [0_usize; -1_isize];
+    = note: `-1_isize` cannot fit into type `usize`
60 
61 error[E0308]: mismatched types
62   --> $DIR/repeat_count.rs:25:17
62   --> $DIR/repeat_count.rs:25:17


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/repeat_count/repeat_count.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args repeat_count.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/repeat_count.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/repeat_count" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/repeat_count/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0435]: attempt to use a non-constant value in a constant
  --> /checkout/src/test/ui/repeat_count.rs:5:17
   |
LL |     let a = [0; n];
   |                 ^ non-constant value
error[E0308]: mismatched types
  --> /checkout/src/test/ui/repeat_count.rs:7:17
   |
   |
LL |     let b = [0; ()];
   |                 ^^ expected `usize`, found `()`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/repeat_count.rs:10:17
   |
   |
LL |     let c = [0; true];
   |                 ^^^^ expected `usize`, found `bool`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/repeat_count.rs:13:17
   |
   |
LL |     let d = [0; 0.5];
   |                 ^^^ expected `usize`, found floating-point number
error[E0308]: mismatched types
  --> /checkout/src/test/ui/repeat_count.rs:16:17
   |
   |
LL |     let e = [0; "foo"];
   |                 ^^^^^ expected `usize`, found `&str`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/repeat_count.rs:31:17
   |
   |
LL |     let g = [0; G { g: () }];
   |                 ^^^^^^^^^^^ expected `usize`, found struct `main::G`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/repeat_count.rs:19:17
   |
   |
LL |     let f = [0; -4_isize];
   |                 ^^^^^^^^ expected `usize`, found `isize`
   = note: `-4_isize` cannot fit into type `usize`

error[E0308]: mismatched types
  --> /checkout/src/test/ui/repeat_count.rs:22:23
  --> /checkout/src/test/ui/repeat_count.rs:22:23
   |
LL |     let f = [0_usize; -1_isize];
   |                       ^^^^^^^^ expected `usize`, found `isize`
   = note: `-1_isize` cannot fit into type `usize`

error[E0308]: mismatched types
  --> /checkout/src/test/ui/repeat_count.rs:25:17
  --> /checkout/src/test/ui/repeat_count.rs:25:17
   |
LL |     let f = [0; 4u8];
   |                 ^^^ expected `usize`, found `u8`
help: change the type of the numeric literal from `u8` to `usize`
   |
   |
LL |     let f = [0; 4usize];

error: aborting due to 9 previous errors

Some errors have detailed explanations: E0308, E0435.
---
thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
Build completed unsuccessfully in 0:57:05
Build completed unsuccessfully in 0:57:05
== clock drift check ==
  local time: Wed Jun 17 04:09:40 UTC 2020
  network time: Wed, 17 Jun 2020 04:09:40 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/73334/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/73334/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3486) (python)
##[section]Finishing: Finalize Job
