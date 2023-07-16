plain
2020-04-24T04:05:41.0626124Z ========================== Starting Command Output ===========================
2020-04-24T04:05:41.0630245Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/b802dfc6-53c1-4ecb-8236-f1be4dcec776.sh
2020-04-24T04:05:41.0630885Z 
2020-04-24T04:05:41.0636271Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-24T04:05:41.0655116Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71489/merge to s
2020-04-24T04:05:41.0658690Z Task         : Get sources
2020-04-24T04:05:41.0658970Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-24T04:05:41.0659261Z Version      : 1.0.0
2020-04-24T04:05:41.0659449Z Author       : Microsoft
---
2020-04-24T04:05:42.0521485Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-24T04:05:42.0527557Z ##[command]git config gc.auto 0
2020-04-24T04:05:42.0531285Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-24T04:05:42.0534403Z ##[command]git config --get-all http.proxy
2020-04-24T04:05:42.0541658Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71489/merge:refs/remotes/pull/71489/merge
---
2020-04-24T04:08:00.5480281Z   libstdc++6-armel-cross libtinfo-dev libtsan0 libubsan0 libubsan0-armel-cross
2020-04-24T04:08:00.5480809Z   libuv1 libwind0-heimdal libxml2 libyaml-0-2 linux-libc-dev
2020-04-24T04:08:00.5481324Z   linux-libc-dev-armel-cross llvm-8 llvm-8-dev llvm-8-runtime mime-support
2020-04-24T04:08:00.5481866Z   multiarch-support openssl patch perl perl-modules-5.26 python python-minimal
2020-04-24T04:08:00.5482398Z   python-pygments python-yaml python2.7-minimal readline-common
2020-04-24T04:08:00.5483188Z   binutils-doc cmake-doc ninja-build cpp-doc gcc-7-locales debian-keyring
2020-04-24T04:08:00.5483697Z   g++-multilib g++-7-multilib gcc-7-doc libstdc++6-7-dbg
2020-04-24T04:08:00.5484206Z   g++-7-multilib-arm-linux-gnueabi libstdc++6-7-dbg-armel-cross gcc-multilib
2020-04-24T04:08:00.5484745Z   manpages-dev autoconf automake libtool flex bison gcc-doc gcc-7-multilib
---
2020-04-24T04:08:00.5489622Z   git-email git-gui gitk gitweb git-cvs git-mediawiki git-svn lrzip glibc-doc
2020-04-24T04:08:00.5490229Z   gnupg | gnupg2 bzr gdbm-l10n krb5-doc krb5-user ncurses-doc libssl-doc
2020-04-24T04:08:00.5490754Z   libstdc++-7-doc llvm-8-doc make-doc ed diffutils-doc perl-doc
2020-04-24T04:08:00.5491276Z   libterm-readline-gnu-perl | libterm-readline-perl-perl python-doc python-tk
2020-04-24T04:08:00.5491778Z   ttf-bitstream-vera python2.7-doc readline-doc
2020-04-24T04:08:00.5492468Z   build-essential fakeroot gnupg | gnupg2 libalgorithm-merge-perl libc-dbg
2020-04-24T04:08:00.5493013Z   gdbserver less ssh-client manpages manpages-dev libfile-fcntllock-perl
2020-04-24T04:08:00.5493533Z   liblocale-gettext-perl libglib2.0-data shared-mime-info xdg-user-dirs
2020-04-24T04:08:00.5493533Z   liblocale-gettext-perl libglib2.0-data shared-mime-info xdg-user-dirs
2020-04-24T04:08:00.5494069Z   krb5-locales publicsuffix libsasl2-modules nodejs-doc netbase python-chardet
2020-04-24T04:08:00.5494499Z   python-pkg-resources
2020-04-24T04:08:00.8438991Z   binfmt-support binutils binutils-arm-linux-gnueabi binutils-common
2020-04-24T04:08:00.8439581Z   binutils-x86-64-linux-gnu ca-certificates cmake cmake-data cpp cpp-7
2020-04-24T04:08:00.8440121Z   cpp-7-arm-linux-gnueabi cpp-arm-linux-gnueabi curl dpkg-dev file g++ g++-7
2020-04-24T04:08:00.8440633Z   g++-7-arm-linux-gnueabi g++-arm-linux-gnueabi gcc gcc-7
---
2020-04-24T04:08:00.8450834Z   libssl-dev libssl1.0.0 libssl1.1 libstdc++-7-dev libstdc++-7-dev-armel-cross
2020-04-24T04:08:00.8451379Z   libstdc++6-armel-cross libtinfo-dev libtsan0 libubsan0 libubsan0-armel-cross
2020-04-24T04:08:00.8452166Z   libuv1 libwind0-heimdal libxml2 libyaml-0-2 linux-libc-dev
2020-04-24T04:08:00.8452682Z   linux-libc-dev-armel-cross llvm-8 llvm-8-dev llvm-8-runtime llvm-8-tools
2020-04-24T04:08:00.8453187Z   make mime-support multiarch-support nodejs openssl patch perl
2020-04-24T04:08:00.8453707Z   perl-modules-5.26 pkg-config python python-minimal python-pygments
2020-04-24T04:08:00.8454227Z   python-yaml python2.7 python2.7-minimal readline-common sudo xz-utils
2020-04-24T04:08:01.1701757Z 0 upgraded, 151 newly installed, 0 to remove and 0 not upgraded.
2020-04-24T04:08:01.1702625Z Need to get 167 MB of archives.
2020-04-24T04:08:01.1703331Z After this operation, 782 MB of additional disk space will be used.
2020-04-24T04:08:01.1704890Z Get:1 http://archive.ubuntu.com/ubuntu bionic/main amd64 multiarch-support amd64 2.27-3ubuntu1 [6916 B]
---
2020-04-24T04:08:13.9705990Z Unpacking libpython3.6-stdlib:amd64 (3.6.9-1~18.04ubuntu1) ...
2020-04-24T04:08:14.1741270Z Selecting previously unselected package libxml2:amd64.
2020-04-24T04:08:14.1751740Z Preparing to unpack .../013-libxml2_2.9.4+dfsg1-6.1ubuntu1.3_amd64.deb ...
2020-04-24T04:08:14.1763285Z Unpacking libxml2:amd64 (2.9.4+dfsg1-6.1ubuntu1.3) ...
2020-04-24T04:08:14.2601302Z Selecting previously unselected package libyaml-0-2:amd64.
2020-04-24T04:08:14.2609528Z Preparing to unpack .../014-libyaml-0-2_0.1.7-2ubuntu3_amd64.deb ...
2020-04-24T04:08:14.2613507Z Unpacking libyaml-0-2:amd64 (0.1.7-2ubuntu3) ...
2020-04-24T04:08:14.2883130Z Preparing to unpack .../015-sudo_1.8.21p2-3ubuntu1.2_amd64.deb ...
2020-04-24T04:08:14.2907955Z Unpacking sudo (1.8.21p2-3ubuntu1.2) ...
2020-04-24T04:08:14.3494224Z Selecting previously unselected package xz-utils.
2020-04-24T04:08:14.3506516Z Preparing to unpack .../016-xz-utils_5.2.2-1.3_amd64.deb ...
---
2020-04-24T04:08:31.6217056Z Unpacking libffi-dev:amd64 (3.2.1-8) ...
2020-04-24T04:08:31.6665580Z Selecting previously unselected package llvm-8-dev.
2020-04-24T04:08:31.6694057Z Preparing to unpack .../123-llvm-8-dev_1%3a8-3~ubuntu18.04.2_amd64.deb ...
2020-04-24T04:08:31.6701487Z Unpacking llvm-8-dev (1:8-3~ubuntu18.04.2) ...
2020-04-24T04:08:34.7507188Z Selecting previously unselected package python-pygments.
2020-04-24T04:08:34.7534740Z Preparing to unpack .../124-python-pygments_2.2.0+dfsg-1_all.deb ...
2020-04-24T04:08:34.7615118Z Unpacking python-pygments (2.2.0+dfsg-1) ...
2020-04-24T04:08:34.8485264Z Selecting previously unselected package python-yaml.
2020-04-24T04:08:34.8508077Z Preparing to unpack .../125-python-yaml_3.12-1build2_amd64.deb ...
2020-04-24T04:08:34.8522707Z Unpacking python-yaml (3.12-1build2) ...
2020-04-24T04:08:34.8880419Z Preparing to unpack .../126-llvm-8-tools_1%3a8-3~ubuntu18.04.2_amd64.deb ...
2020-04-24T04:08:34.8891443Z Unpacking llvm-8-tools (1:8-3~ubuntu18.04.2) ...
2020-04-24T04:08:34.9501196Z Selecting previously unselected package pkg-config.
2020-04-24T04:08:34.9527093Z Preparing to unpack .../127-pkg-config_0.29.1-0ubuntu2_amd64.deb ...
---
2020-04-24T04:09:02.0516513Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-24T04:09:02.0939695Z  ---> Running in 6fc94777159d
2020-04-24T04:09:03.0474606Z Removing intermediate container 6fc94777159d
2020-04-24T04:09:03.0475584Z  ---> bf23fb4fa562
2020-04-24T04:09:03.0476542Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-24T04:09:04.0511737Z Removing intermediate container f530cb1d8f6c
2020-04-24T04:09:04.0513056Z  ---> 69ccd6968fd6
2020-04-24T04:09:04.0513304Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-24T04:09:04.1066890Z  ---> Running in 898b1e11fcfc
---
2020-04-24T04:09:06.0647938Z  ---> 8f4b5ea07222
2020-04-24T04:09:06.0649268Z Successfully built 8f4b5ea07222
2020-04-24T04:09:06.0697938Z Successfully tagged rust-ci:latest
2020-04-24T04:09:06.1000498Z Built container sha256:8f4b5ea07222324024945ab1e7bd40d47f777c37de783dd8110aa340ec9010c3
2020-04-24T04:09:06.1020421Z Uploading finished image to https://rust-lang-ci-sccache2.s3.amazonaws.com/docker/01e88a9474ca8c41c36c8a029a6801ef6e4467f6692e430c9b9e402f5cfd9048278b0d8774bfcb209a220c8a74d64668f696c701e8a37663f9dff484d63e08f7
2020-04-24T04:10:03.2653993Z upload failed: - to s3://rust-lang-ci-sccache2/docker/01e88a9474ca8c41c36c8a029a6801ef6e4467f6692e430c9b9e402f5cfd9048278b0d8774bfcb209a220c8a74d64668f696c701e8a37663f9dff484d63e08f7 An error occurred (InvalidAccessKeyId) when calling the CreateMultipartUpload operation: The AWS Access Key Id you provided does not exist in our records.
2020-04-24T04:10:03.8024429Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-24T04:10:03.8053639Z == clock drift check ==
2020-04-24T04:10:03.8053639Z == clock drift check ==
2020-04-24T04:10:03.8064947Z   local time: Fri Apr 24 04:10:03 UTC 2020
2020-04-24T04:10:04.0976898Z   network time: Fri, 24 Apr 2020 04:10:04 GMT
2020-04-24T04:10:04.1005106Z Starting sccache server...
2020-04-24T04:10:04.1939172Z configure: processing command line
2020-04-24T04:10:04.1939553Z configure: 
2020-04-24T04:10:04.1940467Z configure: rust.dist-src        := False
---
2020-04-24T04:15:16.6889447Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-24T04:15:18.1752321Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-24T04:15:19.8087650Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-24T04:15:20.3760011Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-24T04:15:29.6054731Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-24T04:15:31.5176349Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-24T04:15:35.9177721Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-24T04:15:40.0636306Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-24T04:15:49.9704446Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-24T04:37:46.2450793Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-24T04:37:47.8071685Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-24T04:37:49.5644711Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-24T04:37:49.7006705Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-24T04:37:59.7487305Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-24T04:38:02.0828373Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-24T04:38:06.6959952Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-24T04:38:11.0584212Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-24T04:38:20.7176430Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-24T05:01:03.8700031Z .................................................................................................... 1700/9919
2020-04-24T05:01:08.4310041Z .................................................................................................... 1800/9919
2020-04-24T05:01:16.9571742Z .................................................................................................... 1900/9919
2020-04-24T05:01:25.0396072Z .....i.............................................................................................. 2000/9919
2020-04-24T05:01:31.3796012Z ...............................................................................................iiiii 2100/9919
2020-04-24T05:01:51.2251880Z .................................................................................................... 2300/9919
2020-04-24T05:01:53.4735511Z .................................................................................................... 2400/9919
2020-04-24T05:01:55.7772205Z .................................................................................................... 2500/9919
2020-04-24T05:02:01.5607896Z .................................................................................................... 2600/9919
---
2020-04-24T05:04:56.2268459Z .................................................................................................... 5100/9919
2020-04-24T05:05:03.3155830Z .................................................................................................... 5200/9919
2020-04-24T05:05:07.8303802Z ..................i................................................................................. 5300/9919
2020-04-24T05:05:17.9658698Z ........i........................................................................................... 5400/9919
2020-04-24T05:05:23.6280143Z ........ii.ii........i...i.......................................................................... 5500/9919
2020-04-24T05:05:31.7791674Z .......................................................i............................................ 5700/9919
2020-04-24T05:05:40.7409796Z ........................................................................................ii.......... 5800/9919
2020-04-24T05:05:48.0590725Z ...........................i........................................................................ 5900/9919
2020-04-24T05:05:53.7589047Z .................................................................................................... 6000/9919
2020-04-24T05:05:53.7589047Z .................................................................................................... 6000/9919
2020-04-24T05:06:04.6967512Z .................................................................................................... 6100/9919
2020-04-24T05:06:14.9997596Z .....................ii...i..ii...........i......................................................... 6200/9919
2020-04-24T05:06:31.3366425Z .................................................................................................... 6400/9919
2020-04-24T05:06:35.4262689Z .................................................................................................... 6500/9919
2020-04-24T05:06:35.4262689Z .................................................................................................... 6500/9919
2020-04-24T05:06:44.1777031Z ...................................................i..ii............................................ 6600/9919
2020-04-24T05:07:07.2804708Z .................................................................................................... 6800/9919
2020-04-24T05:07:09.8544059Z ....................................................i............................................... 6900/9919
2020-04-24T05:07:11.9173120Z .................................................................................................... 7000/9919
2020-04-24T05:07:14.0280118Z ..............................................................................................i..... 7100/9919
---
2020-04-24T05:08:50.8239619Z .................................................................................................... 7900/9919
2020-04-24T05:08:56.0884647Z .................................................................................................... 8000/9919
2020-04-24T05:09:02.8130884Z .............................................................i...................................... 8100/9919
2020-04-24T05:09:12.7260165Z .................................................................................................... 8200/9919
2020-04-24T05:09:18.0829928Z ..........iiiiii.iiiii.i............................................................................ 8300/9919
2020-04-24T05:09:31.8336909Z .................................................................................................... 8500/9919
2020-04-24T05:09:39.4913221Z .................................................................................................... 8600/9919
2020-04-24T05:09:52.9717455Z .................................................................................................... 8700/9919
2020-04-24T05:09:59.5915701Z .................................................................................................... 8800/9919
---
2020-04-24T05:12:14.7306718Z Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
2020-04-24T05:12:14.7502636Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-24T05:12:14.9530103Z 
2020-04-24T05:12:14.9531028Z running 186 tests
2020-04-24T05:12:17.9352380Z iiii......i.............ii.i..........i.............................i..i..................i....i.... 100/186
2020-04-24T05:12:20.5127373Z ........i.i.i...iii..iiiiiiiiiiiiiiii.......................iii...............ii......
2020-04-24T05:12:20.5129847Z 
2020-04-24T05:12:20.5135855Z  finished in 5.763
2020-04-24T05:12:20.5142883Z Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
2020-04-24T05:12:20.5346771Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-24T05:12:22.8211265Z Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
2020-04-24T05:12:22.8412625Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-24T05:12:23.0053989Z 
2020-04-24T05:12:23.0054336Z running 9 tests
2020-04-24T05:12:23.0055358Z iiiiiiiii
2020-04-24T05:12:23.0056232Z 
2020-04-24T05:12:23.0056394Z  finished in 0.163
2020-04-24T05:12:23.0056950Z Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
2020-04-24T05:12:23.0251866Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-24T05:12:44.1233727Z Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
2020-04-24T05:12:44.1458492Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-24T05:12:44.3424821Z 
2020-04-24T05:12:44.3425184Z running 115 tests
2020-04-24T05:12:58.2766524Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
2020-04-24T05:12:59.3501182Z ...iiii.....ii.
2020-04-24T05:12:59.3502520Z 
2020-04-24T05:12:59.3502694Z  finished in 15.203
2020-04-24T05:12:59.3503369Z Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
2020-04-24T05:12:59.3504121Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-24T05:24:34.3873377Z warning: 2 warnings emitted
2020-04-24T05:24:34.3873824Z 
2020-04-24T05:24:34.3931419Z 
2020-04-24T05:24:34.3931711Z running 2499 tests
2020-04-24T05:24:43.2647840Z ......iiiii......................................................................................... 100/2499
2020-04-24T05:24:51.9128390Z ......................................................................................ii............ 200/2499
2020-04-24T05:25:11.3507466Z ......................i............................................................................. 400/2499
2020-04-24T05:25:20.9754693Z ............................................................................i..i..................ii 500/2499
2020-04-24T05:25:28.3300165Z ii.................................................................................................. 600/2499
2020-04-24T05:25:36.8296796Z .................................................................................................... 700/2499
---
2020-04-24T05:29:26.0115290Z 
2020-04-24T05:29:26.0115679Z running 1020 tests
2020-04-24T05:29:42.0797661Z i................................................................................................... 100/1020
2020-04-24T05:29:52.0010144Z .................................................................................................... 200/1020
2020-04-24T05:29:59.4734102Z ...................iii......i......i...i......i..................................................... 300/1020
2020-04-24T05:30:04.0996000Z .................................................................................................... 400/1020
2020-04-24T05:30:10.3880970Z ....................................................i....i......................................ii.. 500/1020
2020-04-24T05:30:22.6120719Z .................................................................................................... 700/1020
2020-04-24T05:30:22.6120719Z .................................................................................................... 700/1020
2020-04-24T05:30:29.4252562Z ..............................................iiii.................................................. 800/1020
2020-04-24T05:30:42.2784124Z .................................................................................................... 900/1020
2020-04-24T05:30:48.0330865Z ....................................................................iiii............................ 1000/1020
2020-04-24T05:30:49.2429944Z test result: ok. 1000 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out
2020-04-24T05:30:49.2430184Z 
2020-04-24T05:30:49.2531149Z  finished in 154.418
2020-04-24T05:30:49.2537925Z Set({"src/libterm"}) not skipped for "bootstrap::test::Crate" -- not in ["src/tools/tidy"]
---
2020-04-24T05:33:46.1823688Z 
2020-04-24T05:33:46.1824731Z test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
2020-04-24T05:33:46.1825054Z 
2020-04-24T05:33:46.1883029Z  finished in 0.968
2020-04-24T05:33:46.1894141Z Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
2020-04-24T05:33:46.1912743Z Testing rustc_query_system stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-24T05:33:46.3874663Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-24T05:33:47.4031056Z      Running build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_query_system-c5771041bac2aee4
2020-04-24T05:33:47.4060485Z 
2020-04-24T05:33:47.4061145Z running 0 tests
2020-04-24T05:33:47.4061580Z 
---
2020-04-24T05:47:11.5386807Z Set({"/checkout/src/librustc_parse"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-24T05:47:11.5387527Z Set({"/checkout/src/librustc_passes"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-24T05:47:11.5388426Z Set({"/checkout/src/librustc_plugin_impl"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-24T05:47:11.5389149Z Set({"/checkout/src/librustc_privacy"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-24T05:47:11.5389901Z Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-24T05:47:11.5391358Z Set({"/checkout/src/librustc_save_analysis"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-24T05:47:11.5392185Z Set({"/checkout/src/librustc_session"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-24T05:47:11.5392909Z Set({"/checkout/src/librustc_span"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-24T05:47:11.5393665Z Set({"/checkout/src/librustc_symbol_mangling"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
---
2020-04-24T05:48:13.6282203Z Suite("src/test/run-make-fulldeps") not skipped for "bootstrap::test::RunMakeFullDeps" -- not in ["src/tools/tidy"]
2020-04-24T05:48:13.6664412Z Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-24T05:48:13.8924425Z 
2020-04-24T05:48:13.8926482Z running 211 tests
2020-04-24T05:48:43.0654805Z ......................i...ii.......................................................................i 100/211
2020-04-24T05:49:16.5662207Z ........................................iiiiii......i..............iii.............................F 200/211
2020-04-24T05:49:21.8544032Z .......ii..
2020-04-24T05:49:21.8544594Z 
2020-04-24T05:49:21.8553004Z ---- [run-make] run-make-fulldeps/treat-err-as-bug stdout ----
2020-04-24T05:49:21.8555054Z 
2020-04-24T05:49:21.8555696Z error: make failed
2020-04-24T05:49:21.8555696Z error: make failed
2020-04-24T05:49:21.8556034Z status: exit code: 2
2020-04-24T05:49:21.8556325Z command: "make"
2020-04-24T05:49:21.8556588Z stdout:
2020-04-24T05:49:21.8557453Z ------------------------------------------
2020-04-24T05:49:21.8561833Z LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/treat-err-as-bug/treat-err-as-bug:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/treat-err-as-bug/treat-err-as-bug -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/treat-err-as-bug/treat-err-as-bug  err.rs -Z treat-err-as-bug 2>&1 \
2020-04-24T05:49:21.8563524Z     | "/checkout/src/etc/cat-and-grep.sh" "panicked at 'aborting due to \`-Z treat-err-as-bug=1\`'"
2020-04-24T05:49:21.8563829Z [[[ begin stdout ]]]
2020-04-24T05:49:21.8564291Z error[E0080]: could not evaluate static initializer
2020-04-24T05:49:21.8564851Z   |
2020-04-24T05:49:21.8564851Z   |
2020-04-24T05:49:21.8565197Z 3 | pub static C: u32 = 0-1;
2020-04-24T05:49:21.8565641Z 
2020-04-24T05:49:21.8565903Z 
2020-04-24T05:49:21.8565903Z 
2020-04-24T05:49:21.8566068Z [[[ end stdout ]]]
2020-04-24T05:49:21.8566565Z Error: cannot match: panicked at 'aborting due to `-Z treat-err-as-bug=1`'
2020-04-24T05:49:21.8567063Z Makefile:4: recipe for target 'all' failed
2020-04-24T05:49:21.8567589Z ------------------------------------------
2020-04-24T05:49:21.8567777Z stderr:
2020-04-24T05:49:21.8568131Z ------------------------------------------
2020-04-24T05:49:21.8568131Z ------------------------------------------
2020-04-24T05:49:21.8568360Z make: *** [all] Error 1
2020-04-24T05:49:21.8568832Z ------------------------------------------
2020-04-24T05:49:21.8568988Z 
2020-04-24T05:49:21.8569095Z 
2020-04-24T05:49:21.8569186Z 
---
2020-04-24T05:49:21.8571061Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-24T05:49:21.8571446Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-24T05:49:21.8571657Z 
2020-04-24T05:49:21.8571747Z 
2020-04-24T05:49:21.8587406Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpuasmprinter amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrasmprinter avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitwriter bpf bpfasmparser bpfasmprinter bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader lanai lanaiasmparser lanaiasmprinter lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430asmprinter msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option optremarks orcjit passes perfjitevents powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblyasmprinter webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xcore xcoreasmprinter xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--ar" "ar" "--llvm-bin-dir" "/usr/lib/llvm-8/bin" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-24T05:49:21.8593911Z 
2020-04-24T05:49:21.8594117Z 
2020-04-24T05:49:21.8599377Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-24T05:49:21.8600055Z Build completed unsuccessfully in 1:37:41
2020-04-24T05:49:21.8600055Z Build completed unsuccessfully in 1:37:41
2020-04-24T05:49:21.8647411Z == clock drift check ==
2020-04-24T05:49:21.8676668Z   local time: Fri Apr 24 05:49:21 UTC 2020
2020-04-24T05:49:21.9669530Z   network time: Fri, 24 Apr 2020 05:49:21 GMT
2020-04-24T05:49:23.6589792Z 
2020-04-24T05:49:23.6589792Z 
2020-04-24T05:49:23.6657960Z ##[error]Bash exited with code '1'.
2020-04-24T05:49:23.6673155Z ##[section]Finishing: Run build
2020-04-24T05:49:23.6721578Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71489/merge to s
2020-04-24T05:49:23.6726313Z Task         : Get sources
2020-04-24T05:49:23.6726630Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-24T05:49:23.6726934Z Version      : 1.0.0
2020-04-24T05:49:23.6727140Z Author       : Microsoft
2020-04-24T05:49:23.6727140Z Author       : Microsoft
2020-04-24T05:49:23.6727613Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-24T05:49:23.6727996Z ==============================================================================
2020-04-24T05:49:24.0107312Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-24T05:49:24.0165412Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71489/merge to s
2020-04-24T05:49:24.0259096Z Cleaning up task key
2020-04-24T05:49:24.0260393Z Start cleaning up orphan processes.
2020-04-24T05:49:24.0446528Z Terminate orphan process: pid (3325) (python)
2020-04-24T05:49:24.0637714Z ##[section]Finishing: Finalize Job
