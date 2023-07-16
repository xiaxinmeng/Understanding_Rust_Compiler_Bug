plain
2020-04-11T02:28:23.0300676Z ========================== Starting Command Output ===========================
2020-04-11T02:28:23.0303116Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/43cb7401-2de5-4d12-ba37-ca0b3d74e74d.sh
2020-04-11T02:28:23.0303388Z 
2020-04-11T02:28:23.0307437Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-11T02:28:23.0325996Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70989/merge to s
2020-04-11T02:28:23.0329258Z Task         : Get sources
2020-04-11T02:28:23.0329578Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-11T02:28:23.0329874Z Version      : 1.0.0
2020-04-11T02:28:23.0330075Z Author       : Microsoft
---
2020-04-11T02:28:24.0245884Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-11T02:28:24.0255721Z ##[command]git config gc.auto 0
2020-04-11T02:28:24.0262044Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-11T02:28:24.0268094Z ##[command]git config --get-all http.proxy
2020-04-11T02:28:24.0278095Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70989/merge:refs/remotes/pull/70989/merge
---
2020-04-11T02:30:26.8203463Z   libx32gcc-7-dev libx32gcc1 libx32gomp1 libx32itm1 libx32quadmath0
2020-04-11T02:30:26.8204017Z   libx32stdc++-7-dev libx32stdc++6 libx32ubsan0 libxml2 libyaml-0-2
2020-04-11T02:30:26.8204541Z   linux-libc-dev llvm-8 llvm-8-dev llvm-8-runtime mime-support
2020-04-11T02:30:26.8205095Z   multiarch-support openssl patch perl perl-modules-5.26 python python-minimal
2020-04-11T02:30:26.8205917Z   python-pygments python-yaml python2.7-minimal readline-common
2020-04-11T02:30:26.8210991Z   binutils-doc cmake-doc ninja-build cpp-doc gcc-7-locales debian-keyring
2020-04-11T02:30:26.8211571Z   gcc-7-doc libstdc++6-7-dbg lib32stdc++6-7-dbg libx32stdc++6-7-dbg
2020-04-11T02:30:26.8212117Z   manpages-dev autoconf automake libtool flex bison gcc-doc libgcc1-dbg
2020-04-11T02:30:26.8212701Z   libgomp1-dbg libitm1-dbg libatomic1-dbg libasan4-dbg liblsan0-dbg
2020-04-11T02:30:26.8212701Z   libgomp1-dbg libitm1-dbg libatomic1-dbg libasan4-dbg liblsan0-dbg
2020-04-11T02:30:26.8213272Z   libtsan0-dbg libubsan0-dbg libcilkrts5-dbg libmpx2-dbg libquadmath0-dbg
2020-04-11T02:30:26.8213832Z   gdb-doc gettext-base git-daemon-run | git-daemon-sysvinit git-doc git-el
2020-04-11T02:30:26.8214397Z   git-email git-gui gitk gitweb git-cvs git-mediawiki git-svn lrzip glibc-doc
2020-04-11T02:30:26.8214980Z   gnupg | gnupg2 bzr gdbm-l10n krb5-doc krb5-user ncurses-doc libssl-doc
2020-04-11T02:30:26.8215511Z   libstdc++-7-doc llvm-8-doc make-doc ed diffutils-doc perl-doc
2020-04-11T02:30:26.8216072Z   libterm-readline-gnu-perl | libterm-readline-perl-perl python-doc python-tk
2020-04-11T02:30:26.8216591Z   ttf-bitstream-vera python2.7-doc readline-doc
2020-04-11T02:30:26.8217297Z   build-essential fakeroot gnupg | gnupg2 libalgorithm-merge-perl libc-dbg
2020-04-11T02:30:26.8217871Z   gdbserver less ssh-client manpages manpages-dev libfile-fcntllock-perl
2020-04-11T02:30:26.8218421Z   liblocale-gettext-perl libglib2.0-data shared-mime-info xdg-user-dirs
2020-04-11T02:30:26.8218421Z   liblocale-gettext-perl libglib2.0-data shared-mime-info xdg-user-dirs
2020-04-11T02:30:26.8218990Z   krb5-locales publicsuffix libsasl2-modules nodejs-doc netbase python-chardet
2020-04-11T02:30:26.8219440Z   python-pkg-resources
2020-04-11T02:30:27.1105732Z   binfmt-support binutils binutils-common binutils-x86-64-linux-gnu
2020-04-11T02:30:27.1106333Z   ca-certificates cmake cmake-data cpp cpp-7 curl dpkg-dev file g++ g++-7
2020-04-11T02:30:27.1106920Z   g++-7-multilib g++-multilib gcc gcc-7 gcc-7-base gcc-7-multilib gcc-multilib
2020-04-11T02:30:27.1107532Z   gdb git git-man lib32asan4 lib32atomic1 lib32cilkrts5 lib32gcc-7-dev
---
2020-04-11T02:30:27.1118891Z   libubsan0 libuv1 libwind0-heimdal libx32asan4 libx32atomic1 libx32cilkrts5
2020-04-11T02:30:27.1119467Z   libx32gcc-7-dev libx32gcc1 libx32gomp1 libx32itm1 libx32quadmath0
2020-04-11T02:30:27.1120326Z   libx32stdc++-7-dev libx32stdc++6 libx32ubsan0 libxml2 libyaml-0-2
2020-04-11T02:30:27.1120876Z   linux-libc-dev llvm-8 llvm-8-dev llvm-8-runtime llvm-8-tools make
2020-04-11T02:30:27.1121453Z   mime-support multiarch-support nodejs openssl patch perl perl-modules-5.26
2020-04-11T02:30:27.1122013Z   pkg-config python python-minimal python-pygments python-yaml python2.7
2020-04-11T02:30:27.1122849Z The following packages will be upgraded:
2020-04-11T02:30:27.1123250Z   gcc-8-base libgcc1 libstdc++6
2020-04-11T02:30:27.3775380Z 3 upgraded, 160 newly installed, 0 to remove and 9 not upgraded.
2020-04-11T02:30:27.3776536Z Need to get 155 MB of archives.
---
2020-04-11T02:30:39.9573017Z Unpacking libpython3.6-stdlib:amd64 (3.6.9-1~18.04) ...
2020-04-11T02:30:40.1381196Z Selecting previously unselected package libxml2:amd64.
2020-04-11T02:30:40.1397153Z Preparing to unpack .../012-libxml2_2.9.4+dfsg1-6.1ubuntu1.3_amd64.deb ...
2020-04-11T02:30:40.1408542Z Unpacking libxml2:amd64 (2.9.4+dfsg1-6.1ubuntu1.3) ...
2020-04-11T02:30:40.2201010Z Selecting previously unselected package libyaml-0-2:amd64.
2020-04-11T02:30:40.2201801Z Preparing to unpack .../013-libyaml-0-2_0.1.7-2ubuntu3_amd64.deb ...
2020-04-11T02:30:40.2226193Z Unpacking libyaml-0-2:amd64 (0.1.7-2ubuntu3) ...
2020-04-11T02:30:40.2433600Z Preparing to unpack .../014-sudo_1.8.21p2-3ubuntu1.2_amd64.deb ...
2020-04-11T02:30:40.2460309Z Unpacking sudo (1.8.21p2-3ubuntu1.2) ...
2020-04-11T02:30:40.3023621Z Selecting previously unselected package xz-utils.
2020-04-11T02:30:40.3024202Z Preparing to unpack .../015-xz-utils_5.2.2-1.3_amd64.deb ...
---
2020-04-11T02:30:54.5375166Z Unpacking libffi-dev:amd64 (3.2.1-8) ...
2020-04-11T02:30:54.5706956Z Selecting previously unselected package llvm-8-dev.
2020-04-11T02:30:54.5728347Z Preparing to unpack .../131-llvm-8-dev_1%3a8-3~ubuntu18.04.2_amd64.deb ...
2020-04-11T02:30:54.5737007Z Unpacking llvm-8-dev (1:8-3~ubuntu18.04.2) ...
2020-04-11T02:30:57.9899874Z Selecting previously unselected package python-pygments.
2020-04-11T02:30:57.9923378Z Preparing to unpack .../132-python-pygments_2.2.0+dfsg-1_all.deb ...
2020-04-11T02:30:57.9996307Z Unpacking python-pygments (2.2.0+dfsg-1) ...
2020-04-11T02:30:58.0785574Z Selecting previously unselected package python-yaml.
2020-04-11T02:30:58.0806374Z Preparing to unpack .../133-python-yaml_3.12-1build2_amd64.deb ...
2020-04-11T02:30:58.0817466Z Unpacking python-yaml (3.12-1build2) ...
2020-04-11T02:30:58.1126205Z Preparing to unpack .../134-llvm-8-tools_1%3a8-3~ubuntu18.04.2_amd64.deb ...
2020-04-11T02:30:58.1138601Z Unpacking llvm-8-tools (1:8-3~ubuntu18.04.2) ...
2020-04-11T02:30:58.1704268Z Selecting previously unselected package pkg-config.
2020-04-11T02:30:58.1724114Z Preparing to unpack .../135-pkg-config_0.29.1-0ubuntu2_amd64.deb ...
---
2020-04-11T02:31:28.0541431Z Successfully built 1f29f1bc5fc6
2020-04-11T02:31:28.0609248Z Successfully tagged rust-ci:latest
2020-04-11T02:31:28.0889704Z Built container sha256:1f29f1bc5fc602faf7e731aab595604aeed1d36642108c947951712715ad5691
2020-04-11T02:31:28.0904991Z Uploading finished image to https://rust-lang-ci-sccache2.s3.amazonaws.com/docker/336bdfb8a50774a3deeea378ad09e5b077dfd51a49891789d75d975811a7db5bf5fbd00e1a4d8d870e59556146765bd3a16d274dda991fe21bad88d7ae3745cc
2020-04-11T02:32:20.6269948Z upload failed: - to s3://rust-lang-ci-sccache2/docker/336bdfb8a50774a3deeea378ad09e5b077dfd51a49891789d75d975811a7db5bf5fbd00e1a4d8d870e59556146765bd3a16d274dda991fe21bad88d7ae3745cc An error occurred (InvalidAccessKeyId) when calling the CreateMultipartUpload operation: The AWS Access Key Id you provided does not exist in our records.
2020-04-11T02:32:21.0995645Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-11T02:32:21.0995958Z == clock drift check ==
2020-04-11T02:32:21.0996220Z   local time: Sat Apr 11 02:32:21 UTC 2020
2020-04-11T02:32:21.0996220Z   local time: Sat Apr 11 02:32:21 UTC 2020
2020-04-11T02:32:21.2350263Z   network time: Sat, 11 Apr 2020 02:32:21 GMT
2020-04-11T02:32:21.2379222Z Starting sccache server...
2020-04-11T02:32:21.3231330Z configure: processing command line
2020-04-11T02:32:21.3232516Z configure: 
2020-04-11T02:32:21.3234674Z configure: rust.dist-src        := False
---
2020-04-11T02:37:29.2919011Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-11T02:37:30.7794129Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-11T02:37:32.4251134Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-11T02:37:33.1570535Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-11T02:37:41.8985928Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-11T02:37:43.9455782Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-11T02:37:48.0130123Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-11T02:37:51.7630491Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-11T02:38:00.7806791Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-11T02:55:09.0801110Z    Compiling unwind v0.0.0 (/checkout/src/libunwind)
2020-04-11T02:55:09.8568642Z    Compiling backtrace-sys v0.1.35
2020-04-11T02:55:10.6917546Z    Compiling hashbrown v0.6.2
2020-04-11T02:55:19.9963968Z    Compiling rustc-std-workspace-core v1.99.0 (/checkout/src/tools/rustc-std-workspace-core)
2020-04-11T02:55:24.7110191Z cmpxchg instructions must be atomic.
2020-04-11T02:55:24.7118477Z   %10 = cmpxchg i64* %9, i64 0, i64 0 unordered not_atomic
2020-04-11T02:55:24.7124350Z in function __llvm_memcpy_element_unordered_atomic_8
2020-04-11T02:55:24.7130202Z LLVM ERROR: Broken function found, compilation aborted!
2020-04-11T02:55:24.7191479Z error: could not compile `compiler_builtins`.
2020-04-11T02:55:24.7196639Z To learn more, run the command again with --verbose.
2020-04-11T02:55:24.7216745Z warning: build failed, waiting for other jobs to finish...
2020-04-11T02:55:24.7688390Z error: build failed
2020-04-11T02:55:24.7714099Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "i586-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-04-11T02:55:24.7714099Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "i586-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-04-11T02:55:24.7727654Z expected success, got: exit code: 101
2020-04-11T02:55:24.7733683Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build --stage 1 src/libstd --target=i586-unknown-linux-gnu
2020-04-11T02:55:24.7734303Z Build completed unsuccessfully in 0:21:26
2020-04-11T02:55:24.7780508Z == clock drift check ==
2020-04-11T02:55:24.7788628Z   local time: Sat Apr 11 02:55:24 UTC 2020
2020-04-11T02:55:24.9019082Z   network time: Sat, 11 Apr 2020 02:55:24 GMT
2020-04-11T02:55:27.2918738Z 
2020-04-11T02:55:27.2918738Z 
2020-04-11T02:55:27.2977321Z ##[error]Bash exited with code '1'.
2020-04-11T02:55:27.3006237Z ##[section]Finishing: Run build
2020-04-11T02:55:27.3050022Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70989/merge to s
2020-04-11T02:55:27.3054861Z Task         : Get sources
2020-04-11T02:55:27.3055203Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-11T02:55:27.3055514Z Version      : 1.0.0
2020-04-11T02:55:27.3055752Z Author       : Microsoft
2020-04-11T02:55:27.3055752Z Author       : Microsoft
2020-04-11T02:55:27.3056116Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-11T02:55:27.3056514Z ==============================================================================
2020-04-11T02:55:27.6124192Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-11T02:55:27.6167703Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70989/merge to s
2020-04-11T02:55:27.6253443Z Cleaning up task key
2020-04-11T02:55:27.6254671Z Start cleaning up orphan processes.
2020-04-11T02:55:27.6462455Z Terminate orphan process: pid (3638) (python)
2020-04-11T02:55:27.6666339Z ##[section]Finishing: Finalize Job
