plain
2020-04-10T20:41:38.9083520Z ========================== Starting Command Output ===========================
2020-04-10T20:41:38.9091191Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/2c022d24-952c-4948-8643-18bdb784441c.sh
2020-04-10T20:41:38.9091762Z 
2020-04-10T20:41:38.9099164Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-10T20:41:38.9121111Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70989/merge to s
2020-04-10T20:41:38.9125234Z Task         : Get sources
2020-04-10T20:41:38.9125585Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-10T20:41:38.9125900Z Version      : 1.0.0
2020-04-10T20:41:38.9126114Z Author       : Microsoft
---
2020-04-10T20:41:39.8979138Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-10T20:41:39.8984833Z ##[command]git config gc.auto 0
2020-04-10T20:41:39.8989004Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-10T20:41:39.8992724Z ##[command]git config --get-all http.proxy
2020-04-10T20:41:39.8999857Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70989/merge:refs/remotes/pull/70989/merge
---
2020-04-10T20:43:37.5008487Z   libwind0-heimdal libx32asan4 libx32atomic1 libx32cilkrts5 libx32gcc-7-dev
2020-04-10T20:43:37.5009433Z   libx32gcc1 libx32gomp1 libx32itm1 libx32quadmath0 libx32stdc++-7-dev
2020-04-10T20:43:37.5010092Z   libx32stdc++6 libx32ubsan0 libxml2 libyaml-0-2 linux-libc-dev mime-support
2020-04-10T20:43:37.5010755Z   multiarch-support openssl patch perl perl-modules-5.26 python python-minimal
2020-04-10T20:43:37.5011418Z   python-pygments python-yaml python2.7-minimal python3 python3-minimal
2020-04-10T20:43:37.5012072Z   python3-pygments python3-yaml python3.6 python3.6-minimal readline-common
2020-04-10T20:43:37.5013050Z   binutils-doc cmake-doc ninja-build cpp-doc gcc-7-locales debian-keyring
2020-04-10T20:43:37.5013734Z   gcc-7-doc libstdc++6-7-dbg lib32stdc++6-7-dbg libx32stdc++6-7-dbg
2020-04-10T20:43:37.5014360Z   manpages-dev autoconf automake libtool flex bison gcc-doc libgcc1-dbg
2020-04-10T20:43:37.5014994Z   libgomp1-dbg libitm1-dbg libatomic1-dbg libasan4-dbg liblsan0-dbg
2020-04-10T20:43:37.5014994Z   libgomp1-dbg libitm1-dbg libatomic1-dbg libasan4-dbg liblsan0-dbg
2020-04-10T20:43:37.5015628Z   libtsan0-dbg libubsan0-dbg libcilkrts5-dbg libmpx2-dbg libquadmath0-dbg
2020-04-10T20:43:37.5016274Z   gdb-doc gettext-base git-daemon-run | git-daemon-sysvinit git-doc git-el
2020-04-10T20:43:37.5016943Z   git-email git-gui gitk gitweb git-cvs git-mediawiki git-svn lrzip glibc-doc
2020-04-10T20:43:37.5017590Z   gnupg | gnupg2 bzr gdbm-l10n krb5-doc krb5-user ncurses-doc libssl-doc
2020-04-10T20:43:37.5018239Z   libstdc++-7-doc make-doc ed diffutils-doc perl-doc libterm-readline-gnu-perl
2020-04-10T20:43:37.5018891Z   | libterm-readline-perl-perl python-doc python-tk ttf-bitstream-vera
2020-04-10T20:43:37.5020063Z   python3.6-venv python3.6-doc readline-doc
2020-04-10T20:43:37.5020344Z Recommended packages:
2020-04-10T20:43:37.5020889Z   build-essential fakeroot gnupg | gnupg2 libalgorithm-merge-perl libc-dbg
2020-04-10T20:43:37.5021522Z   gdbserver less ssh-client manpages manpages-dev libfile-fcntllock-perl
2020-04-10T20:43:37.5021522Z   gdbserver less ssh-client manpages manpages-dev libfile-fcntllock-perl
2020-04-10T20:43:37.5022168Z   liblocale-gettext-perl libglib2.0-data shared-mime-info xdg-user-dirs
2020-04-10T20:43:37.5022829Z   krb5-locales publicsuffix libsasl2-modules nodejs-doc netbase python-chardet
2020-04-10T20:43:37.5023391Z   python-pkg-resources python3-pkg-resources
2020-04-10T20:43:37.7477287Z   binutils binutils-common binutils-x86-64-linux-gnu ca-certificates cmake
2020-04-10T20:43:37.7477938Z   cmake-data cpp cpp-7 curl dpkg-dev file g++ g++-7 g++-7-multilib
2020-04-10T20:43:37.7478595Z   g++-multilib gcc gcc-7 gcc-7-base gcc-7-multilib gcc-multilib gdb git
2020-04-10T20:43:37.7479277Z   git-man lib32asan4 lib32atomic1 lib32cilkrts5 lib32gcc-7-dev lib32gcc1
---
2020-04-10T20:43:37.7490101Z   libsasl2-modules-db libsqlite3-0 libssl-dev libssl1.0.0 libssl1.1
2020-04-10T20:43:37.7490732Z   libstdc++-7-dev libtinfo-dev libtsan0 libubsan0 libuv1 libwind0-heimdal
2020-04-10T20:43:37.7491368Z   libx32asan4 libx32atomic1 libx32cilkrts5 libx32gcc-7-dev libx32gcc1
2020-04-10T20:43:37.7492031Z   libx32gomp1 libx32itm1 libx32quadmath0 libx32stdc++-7-dev libx32stdc++6
2020-04-10T20:43:37.7492778Z   libx32ubsan0 libxml2 libyaml-0-2 linux-libc-dev llvm-9-tools make
2020-04-10T20:43:37.7493465Z   mime-support multiarch-support nodejs openssl patch perl perl-modules-5.26
2020-04-10T20:43:37.7494140Z   pkg-config python python-minimal python-pygments python-yaml python2.7
2020-04-10T20:43:37.7494783Z   python2.7-minimal python3 python3-minimal python3-pygments python3-yaml
2020-04-10T20:43:37.7495804Z The following packages will be upgraded:
2020-04-10T20:43:37.7496262Z   gcc-8-base libgcc1 libstdc++6
2020-04-10T20:43:38.0195657Z 3 upgraded, 160 newly installed, 0 to remove and 9 not upgraded.
2020-04-10T20:43:38.0195949Z Need to get 114 MB of archives.
---
2020-04-10T20:43:49.0752298Z Unpacking libicu60:amd64 (60.2-3ubuntu3.1) ...
2020-04-10T20:43:50.0858723Z Selecting previously unselected package libxml2:amd64.
2020-04-10T20:43:50.0871354Z Preparing to unpack .../009-libxml2_2.9.4+dfsg1-6.1ubuntu1.3_amd64.deb ...
2020-04-10T20:43:50.0883089Z Unpacking libxml2:amd64 (2.9.4+dfsg1-6.1ubuntu1.3) ...
2020-04-10T20:43:50.1834367Z Selecting previously unselected package libyaml-0-2:amd64.
2020-04-10T20:43:50.1849684Z Preparing to unpack .../010-libyaml-0-2_0.1.7-2ubuntu3_amd64.deb ...
2020-04-10T20:43:50.1871695Z Unpacking libyaml-0-2:amd64 (0.1.7-2ubuntu3) ...
2020-04-10T20:43:50.2087657Z Selecting previously unselected package python3-yaml.
2020-04-10T20:43:50.2099752Z Preparing to unpack .../011-python3-yaml_3.12-1build2_amd64.deb ...
2020-04-10T20:43:50.2112827Z Unpacking python3-yaml (3.12-1build2) ...
2020-04-10T20:43:50.2441393Z Preparing to unpack .../012-sudo_1.8.21p2-3ubuntu1.2_amd64.deb ...
2020-04-10T20:43:50.2465334Z Unpacking sudo (1.8.21p2-3ubuntu1.2) ...
2020-04-10T20:43:50.3106309Z Selecting previously unselected package xz-utils.
2020-04-10T20:43:50.3118440Z Preparing to unpack .../013-xz-utils_5.2.2-1.3_amd64.deb ...
---
2020-04-10T20:44:01.1491116Z Unpacking libedit-dev:amd64 (3.1-20170329-1) ...
2020-04-10T20:44:01.1735963Z Selecting previously unselected package libssl-dev:amd64.
2020-04-10T20:44:01.1753571Z Preparing to unpack .../122-libssl-dev_1.1.1-1ubuntu2.1~18.04.5_amd64.deb ...
2020-04-10T20:44:01.1771567Z Unpacking libssl-dev:amd64 (1.1.1-1ubuntu2.1~18.04.5) ...
2020-04-10T20:44:01.3529759Z Selecting previously unselected package python-pygments.
2020-04-10T20:44:01.3557200Z Preparing to unpack .../123-python-pygments_2.2.0+dfsg-1_all.deb ...
2020-04-10T20:44:01.3637323Z Unpacking python-pygments (2.2.0+dfsg-1) ...
2020-04-10T20:44:01.4442997Z Selecting previously unselected package python-yaml.
2020-04-10T20:44:01.4465299Z Preparing to unpack .../124-python-yaml_3.12-1build2_amd64.deb ...
2020-04-10T20:44:01.4480900Z Unpacking python-yaml (3.12-1build2) ...
2020-04-10T20:44:01.4787394Z Selecting previously unselected package python3-pygments.
2020-04-10T20:44:01.4804352Z Preparing to unpack .../125-python3-pygments_2.2.0+dfsg-1_all.deb ...
2020-04-10T20:44:01.4816503Z Unpacking python3-pygments (2.2.0+dfsg-1) ...
2020-04-10T20:44:01.5647156Z Preparing to unpack .../126-llvm-9-tools_1%3a9-2~ubuntu18.04.2_amd64.deb ...
2020-04-10T20:44:01.5656271Z Unpacking llvm-9-tools (1:9-2~ubuntu18.04.2) ...
2020-04-10T20:44:01.6368377Z Selecting previously unselected package pkg-config.
2020-04-10T20:44:01.6387748Z Preparing to unpack .../127-pkg-config_0.29.1-0ubuntu2_amd64.deb ...
---
2020-04-10T20:44:30.0718908Z Successfully built aff367096a80
2020-04-10T20:44:30.0756791Z Successfully tagged rust-ci:latest
2020-04-10T20:44:30.1072790Z Built container sha256:aff367096a80069c4223b20cd88911bad54f881b39d9e11c87aa8887b84f8370
2020-04-10T20:44:30.1093273Z Uploading finished image to https://rust-lang-ci-sccache2.s3.amazonaws.com/docker/4e9138cad10f948b4966d6ee14fb349e05416faf4283b234c6f8952e3235a5f7d2fc083400d8e61d68cc1d932a36e6e0de3b489b2a33a11ef8315761abb72726
2020-04-10T20:45:04.6626055Z upload failed: - to s3://rust-lang-ci-sccache2/docker/4e9138cad10f948b4966d6ee14fb349e05416faf4283b234c6f8952e3235a5f7d2fc083400d8e61d68cc1d932a36e6e0de3b489b2a33a11ef8315761abb72726 An error occurred (InvalidAccessKeyId) when calling the CreateMultipartUpload operation: The AWS Access Key Id you provided does not exist in our records.
2020-04-10T20:45:05.2761539Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-10T20:45:05.2793690Z == clock drift check ==
2020-04-10T20:45:05.2793690Z == clock drift check ==
2020-04-10T20:45:05.2807526Z   local time: Fri Apr 10 20:45:05 UTC 2020
2020-04-10T20:45:05.4687022Z   network time: Fri, 10 Apr 2020 20:45:05 GMT
2020-04-10T20:45:05.4710574Z Starting sccache server...
2020-04-10T20:45:05.5717283Z configure: processing command line
2020-04-10T20:45:05.5717840Z configure: 
2020-04-10T20:45:05.5718946Z configure: build.build          := x86_64-unknown-linux-gnu
---
2020-04-10T20:46:58.1130197Z Makefile:67: recipe for target 'prepare' failed
2020-04-10T20:46:58.1130580Z make: *** [prepare] Error 1
2020-04-10T20:46:58.1136387Z The command has failed after 5 attempts.
2020-04-10T20:46:58.1136663Z == clock drift check ==
2020-04-10T20:46:58.1143778Z   local time: Fri Apr 10 20:46:58 UTC 2020
2020-04-10T20:46:58.3021881Z   network time: Fri, 10 Apr 2020 20:46:58 GMT
2020-04-10T20:47:04.0560091Z 
2020-04-10T20:47:04.0560091Z 
2020-04-10T20:47:04.0659607Z ##[error]Bash exited with code '1'.
2020-04-10T20:47:04.0678014Z ##[section]Finishing: Run build
2020-04-10T20:47:04.0727700Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70989/merge to s
2020-04-10T20:47:04.0733551Z Task         : Get sources
2020-04-10T20:47:04.0733962Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-10T20:47:04.0734542Z Version      : 1.0.0
2020-04-10T20:47:04.0734808Z Author       : Microsoft
2020-04-10T20:47:04.0734808Z Author       : Microsoft
2020-04-10T20:47:04.0735231Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-10T20:47:04.0735694Z ==============================================================================
2020-04-10T20:47:04.4069665Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-10T20:47:04.4115382Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70989/merge to s
2020-04-10T20:47:04.4215036Z Cleaning up task key
2020-04-10T20:47:04.4216689Z Start cleaning up orphan processes.
2020-04-10T20:47:04.4436642Z Terminate orphan process: pid (3950) (python)
2020-04-10T20:47:04.4607696Z ##[section]Finishing: Finalize Job
