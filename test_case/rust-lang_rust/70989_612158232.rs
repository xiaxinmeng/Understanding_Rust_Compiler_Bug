plain
2020-04-10T18:12:02.9080801Z ========================== Starting Command Output ===========================
2020-04-10T18:12:02.9083282Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/6c013612-3af0-4fc2-8d80-aeefaa99268b.sh
2020-04-10T18:12:02.9083555Z 
2020-04-10T18:12:02.9088669Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-10T18:12:02.9106456Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70989/merge to s
2020-04-10T18:12:02.9109486Z Task         : Get sources
2020-04-10T18:12:02.9109771Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-10T18:12:02.9110064Z Version      : 1.0.0
2020-04-10T18:12:02.9110249Z Author       : Microsoft
---
2020-04-10T18:12:04.1788730Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-10T18:12:04.1795609Z ##[command]git config gc.auto 0
2020-04-10T18:12:04.1800012Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-10T18:12:04.1804114Z ##[command]git config --get-all http.proxy
2020-04-10T18:12:04.1812928Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70989/merge:refs/remotes/pull/70989/merge
---
2020-04-10T18:13:59.6442353Z   libwind0-heimdal libx32asan4 libx32atomic1 libx32cilkrts5 libx32gcc-7-dev
2020-04-10T18:13:59.6443300Z   libx32gcc1 libx32gomp1 libx32itm1 libx32quadmath0 libx32stdc++-7-dev
2020-04-10T18:13:59.6443992Z   libx32stdc++6 libx32ubsan0 libxml2 libyaml-0-2 linux-libc-dev mime-support
2020-04-10T18:13:59.6444718Z   multiarch-support openssl patch perl perl-modules-5.26 python python-minimal
2020-04-10T18:13:59.6445413Z   python-pygments python-yaml python2.7-minimal python3 python3-minimal
2020-04-10T18:13:59.6446125Z   python3-pygments python3-yaml python3.6 python3.6-minimal readline-common
2020-04-10T18:13:59.6449050Z   binutils-doc cmake-doc ninja-build cpp-doc gcc-7-locales debian-keyring
2020-04-10T18:13:59.6449970Z   gcc-7-doc libstdc++6-7-dbg lib32stdc++6-7-dbg libx32stdc++6-7-dbg
2020-04-10T18:13:59.6450805Z   manpages-dev autoconf automake libtool flex bison gcc-doc libgcc1-dbg
2020-04-10T18:13:59.6451648Z   libgomp1-dbg libitm1-dbg libatomic1-dbg libasan4-dbg liblsan0-dbg
2020-04-10T18:13:59.6451648Z   libgomp1-dbg libitm1-dbg libatomic1-dbg libasan4-dbg liblsan0-dbg
2020-04-10T18:13:59.6452491Z   libtsan0-dbg libubsan0-dbg libcilkrts5-dbg libmpx2-dbg libquadmath0-dbg
2020-04-10T18:13:59.6453335Z   gdb-doc gettext-base git-daemon-run | git-daemon-sysvinit git-doc git-el
2020-04-10T18:13:59.6454213Z   git-email git-gui gitk gitweb git-cvs git-mediawiki git-svn lrzip glibc-doc
2020-04-10T18:13:59.6455068Z   gnupg | gnupg2 bzr gdbm-l10n krb5-doc krb5-user ncurses-doc libssl-doc
2020-04-10T18:13:59.6455927Z   libstdc++-7-doc make-doc ed diffutils-doc perl-doc libterm-readline-gnu-perl
2020-04-10T18:13:59.6456788Z   | libterm-readline-perl-perl python-doc python-tk ttf-bitstream-vera
2020-04-10T18:13:59.6458403Z   python3.6-venv python3.6-doc readline-doc
2020-04-10T18:13:59.6458832Z Recommended packages:
2020-04-10T18:13:59.6459556Z   build-essential fakeroot gnupg | gnupg2 libalgorithm-merge-perl libc-dbg
2020-04-10T18:13:59.6460418Z   gdbserver less ssh-client manpages manpages-dev libfile-fcntllock-perl
2020-04-10T18:13:59.6460418Z   gdbserver less ssh-client manpages manpages-dev libfile-fcntllock-perl
2020-04-10T18:13:59.6461258Z   liblocale-gettext-perl libglib2.0-data shared-mime-info xdg-user-dirs
2020-04-10T18:13:59.6462430Z   krb5-locales publicsuffix libsasl2-modules nodejs-doc netbase python-chardet
2020-04-10T18:13:59.6463225Z   python-pkg-resources python3-pkg-resources
2020-04-10T18:13:59.9043979Z   binutils binutils-common binutils-x86-64-linux-gnu ca-certificates cmake
2020-04-10T18:13:59.9044537Z   cmake-data cpp cpp-7 curl dpkg-dev file g++ g++-7 g++-7-multilib
2020-04-10T18:13:59.9045084Z   g++-multilib gcc gcc-7 gcc-7-base gcc-7-multilib gcc-multilib gdb git
2020-04-10T18:13:59.9045624Z   git-man lib32asan4 lib32atomic1 lib32cilkrts5 lib32gcc-7-dev lib32gcc1
---
2020-04-10T18:13:59.9054410Z   libsasl2-modules-db libsqlite3-0 libssl-dev libssl1.0.0 libssl1.1
2020-04-10T18:13:59.9054933Z   libstdc++-7-dev libtinfo-dev libtsan0 libubsan0 libuv1 libwind0-heimdal
2020-04-10T18:13:59.9055461Z   libx32asan4 libx32atomic1 libx32cilkrts5 libx32gcc-7-dev libx32gcc1
2020-04-10T18:13:59.9056074Z   libx32gomp1 libx32itm1 libx32quadmath0 libx32stdc++-7-dev libx32stdc++6
2020-04-10T18:13:59.9056610Z   libx32ubsan0 libxml2 libyaml-0-2 linux-libc-dev llvm-9-tools make
2020-04-10T18:13:59.9057145Z   mime-support multiarch-support nodejs openssl patch perl perl-modules-5.26
2020-04-10T18:13:59.9057702Z   pkg-config python python-minimal python-pygments python-yaml python2.7
2020-04-10T18:13:59.9058239Z   python2.7-minimal python3 python3-minimal python3-pygments python3-yaml
2020-04-10T18:13:59.9059084Z The following packages will be upgraded:
2020-04-10T18:13:59.9059466Z   gcc-8-base libgcc1 libstdc++6
2020-04-10T18:14:00.1230478Z 3 upgraded, 160 newly installed, 0 to remove and 9 not upgraded.
2020-04-10T18:14:00.1230981Z Need to get 114 MB of archives.
---
2020-04-10T18:14:11.7166511Z Unpacking libicu60:amd64 (60.2-3ubuntu3.1) ...
2020-04-10T18:14:12.3404389Z Selecting previously unselected package libxml2:amd64.
2020-04-10T18:14:12.3416631Z Preparing to unpack .../009-libxml2_2.9.4+dfsg1-6.1ubuntu1.3_amd64.deb ...
2020-04-10T18:14:12.3435958Z Unpacking libxml2:amd64 (2.9.4+dfsg1-6.1ubuntu1.3) ...
2020-04-10T18:14:12.4279999Z Selecting previously unselected package libyaml-0-2:amd64.
2020-04-10T18:14:12.4291005Z Preparing to unpack .../010-libyaml-0-2_0.1.7-2ubuntu3_amd64.deb ...
2020-04-10T18:14:12.4304891Z Unpacking libyaml-0-2:amd64 (0.1.7-2ubuntu3) ...
2020-04-10T18:14:12.4489789Z Selecting previously unselected package python3-yaml.
2020-04-10T18:14:12.4501072Z Preparing to unpack .../011-python3-yaml_3.12-1build2_amd64.deb ...
2020-04-10T18:14:12.4513697Z Unpacking python3-yaml (3.12-1build2) ...
2020-04-10T18:14:12.4832873Z Preparing to unpack .../012-sudo_1.8.21p2-3ubuntu1.2_amd64.deb ...
2020-04-10T18:14:12.4853493Z Unpacking sudo (1.8.21p2-3ubuntu1.2) ...
2020-04-10T18:14:12.5436043Z Selecting previously unselected package xz-utils.
2020-04-10T18:14:12.5450388Z Preparing to unpack .../013-xz-utils_5.2.2-1.3_amd64.deb ...
---
2020-04-10T18:14:23.7654761Z Unpacking libedit-dev:amd64 (3.1-20170329-1) ...
2020-04-10T18:14:23.7937261Z Selecting previously unselected package libssl-dev:amd64.
2020-04-10T18:14:23.7953637Z Preparing to unpack .../122-libssl-dev_1.1.1-1ubuntu2.1~18.04.5_amd64.deb ...
2020-04-10T18:14:23.7986297Z Unpacking libssl-dev:amd64 (1.1.1-1ubuntu2.1~18.04.5) ...
2020-04-10T18:14:23.9626405Z Selecting previously unselected package python-pygments.
2020-04-10T18:14:23.9644339Z Preparing to unpack .../123-python-pygments_2.2.0+dfsg-1_all.deb ...
2020-04-10T18:14:23.9716319Z Unpacking python-pygments (2.2.0+dfsg-1) ...
2020-04-10T18:14:24.0563835Z Selecting previously unselected package python-yaml.
2020-04-10T18:14:24.0585090Z Preparing to unpack .../124-python-yaml_3.12-1build2_amd64.deb ...
2020-04-10T18:14:24.0596262Z Unpacking python-yaml (3.12-1build2) ...
2020-04-10T18:14:24.0873218Z Selecting previously unselected package python3-pygments.
2020-04-10T18:14:24.0893553Z Preparing to unpack .../125-python3-pygments_2.2.0+dfsg-1_all.deb ...
2020-04-10T18:14:24.0906190Z Unpacking python3-pygments (2.2.0+dfsg-1) ...
2020-04-10T18:14:24.1688300Z Preparing to unpack .../126-llvm-9-tools_1%3a9-2~ubuntu18.04.2_amd64.deb ...
2020-04-10T18:14:24.1698952Z Unpacking llvm-9-tools (1:9-2~ubuntu18.04.2) ...
2020-04-10T18:14:24.2314630Z Selecting previously unselected package pkg-config.
2020-04-10T18:14:24.2335064Z Preparing to unpack .../127-pkg-config_0.29.1-0ubuntu2_amd64.deb ...
---
2020-04-10T18:14:51.0506228Z Successfully built c43ae52995b7
2020-04-10T18:14:51.0540849Z Successfully tagged rust-ci:latest
2020-04-10T18:14:51.0918338Z Built container sha256:c43ae52995b72addcb2fbc914f9f769ac476ebf2f27d97f877a8bd36c1d5e89e
2020-04-10T18:14:51.0934249Z Uploading finished image to https://rust-lang-ci-sccache2.s3.amazonaws.com/docker/d5b941e9c2b34af4d4e593955f01a9f1cfaa046ac886c771be93a7604394a4ae7970932f520e99eb2995ffd0e57d6e7129446568c1844c92011622fada2abb20
2020-04-10T18:15:28.3991795Z upload failed: - to s3://rust-lang-ci-sccache2/docker/d5b941e9c2b34af4d4e593955f01a9f1cfaa046ac886c771be93a7604394a4ae7970932f520e99eb2995ffd0e57d6e7129446568c1844c92011622fada2abb20 An error occurred (InvalidAccessKeyId) when calling the CreateMultipartUpload operation: The AWS Access Key Id you provided does not exist in our records.
2020-04-10T18:15:28.9126193Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-10T18:15:28.9156890Z == clock drift check ==
2020-04-10T18:15:28.9156890Z == clock drift check ==
2020-04-10T18:15:28.9160908Z   local time: Fri Apr 10 18:15:28 UTC 2020
2020-04-10T18:15:29.0615423Z   network time: Fri, 10 Apr 2020 18:15:29 GMT
2020-04-10T18:15:29.0642397Z Starting sccache server...
2020-04-10T18:15:29.1417952Z configure: processing command line
2020-04-10T18:15:29.1418279Z configure: 
2020-04-10T18:15:29.1419270Z configure: rust.dist-src        := False
---
2020-04-10T18:17:03.0050546Z make: *** [prepare] Error 1
2020-04-10T18:17:03.0051283Z Makefile:67: recipe for target 'prepare' failed
2020-04-10T18:17:03.0055985Z The command has failed after 5 attempts.
2020-04-10T18:17:03.0056217Z == clock drift check ==
2020-04-10T18:17:03.0066420Z   local time: Fri Apr 10 18:17:03 UTC 2020
2020-04-10T18:17:03.2024461Z   network time: Fri, 10 Apr 2020 18:17:03 GMT
2020-04-10T18:17:09.0127508Z 
2020-04-10T18:17:09.0127508Z 
2020-04-10T18:17:09.0193598Z ##[error]Bash exited with code '1'.
2020-04-10T18:17:09.0205902Z ##[section]Finishing: Run build
2020-04-10T18:17:09.0246769Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70989/merge to s
2020-04-10T18:17:09.0251273Z Task         : Get sources
2020-04-10T18:17:09.0251599Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-10T18:17:09.0251913Z Version      : 1.0.0
2020-04-10T18:17:09.0252125Z Author       : Microsoft
2020-04-10T18:17:09.0252125Z Author       : Microsoft
2020-04-10T18:17:09.0252463Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-10T18:17:09.0252864Z ==============================================================================
2020-04-10T18:17:09.3233657Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-10T18:17:09.3275165Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70989/merge to s
2020-04-10T18:17:09.3362431Z Cleaning up task key
2020-04-10T18:17:09.3363794Z Start cleaning up orphan processes.
2020-04-10T18:17:09.3545469Z Terminate orphan process: pid (3796) (python)
2020-04-10T18:17:09.3668083Z ##[section]Finishing: Finalize Job
