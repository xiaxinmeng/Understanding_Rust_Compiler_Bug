plain
2020-04-10T19:40:00.9470459Z ========================== Starting Command Output ===========================
2020-04-10T19:40:00.9472942Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/eede6f85-7752-480c-8231-550fd53a11e2.sh
2020-04-10T19:40:00.9473236Z 
2020-04-10T19:40:00.9477569Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-10T19:40:00.9496874Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70989/merge to s
2020-04-10T19:40:00.9501106Z Task         : Get sources
2020-04-10T19:40:00.9501590Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-10T19:40:00.9501849Z Version      : 1.0.0
2020-04-10T19:40:00.9502023Z Author       : Microsoft
---
2020-04-10T19:40:02.1492812Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-10T19:40:02.1502777Z ##[command]git config gc.auto 0
2020-04-10T19:40:02.1510022Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-10T19:40:02.1518363Z ##[command]git config --get-all http.proxy
2020-04-10T19:40:02.1531123Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70989/merge:refs/remotes/pull/70989/merge
---
2020-04-10T19:42:13.9287899Z   libwind0-heimdal libx32asan4 libx32atomic1 libx32cilkrts5 libx32gcc-7-dev
2020-04-10T19:42:13.9288601Z   libx32gcc1 libx32gomp1 libx32itm1 libx32quadmath0 libx32stdc++-7-dev
2020-04-10T19:42:13.9289151Z   libx32stdc++6 libx32ubsan0 libxml2 libyaml-0-2 linux-libc-dev mime-support
2020-04-10T19:42:13.9290130Z   multiarch-support openssl patch perl perl-modules-5.26 python python-minimal
2020-04-10T19:42:13.9290646Z   python-pygments python-yaml python2.7-minimal python3 python3-minimal
2020-04-10T19:42:13.9291631Z   python3-pygments python3-yaml python3.6 python3.6-minimal readline-common
2020-04-10T19:42:13.9292413Z   binutils-doc cmake-doc ninja-build cpp-doc gcc-7-locales debian-keyring
2020-04-10T19:42:13.9293110Z   gcc-7-doc libstdc++6-7-dbg lib32stdc++6-7-dbg libx32stdc++6-7-dbg
2020-04-10T19:42:13.9293719Z   manpages-dev autoconf automake libtool flex bison gcc-doc libgcc1-dbg
2020-04-10T19:42:13.9294459Z   libgomp1-dbg libitm1-dbg libatomic1-dbg libasan4-dbg liblsan0-dbg
2020-04-10T19:42:13.9294459Z   libgomp1-dbg libitm1-dbg libatomic1-dbg libasan4-dbg liblsan0-dbg
2020-04-10T19:42:13.9295294Z   libtsan0-dbg libubsan0-dbg libcilkrts5-dbg libmpx2-dbg libquadmath0-dbg
2020-04-10T19:42:13.9295798Z   gdb-doc gettext-base git-daemon-run | git-daemon-sysvinit git-doc git-el
2020-04-10T19:42:13.9296772Z   git-email git-gui gitk gitweb git-cvs git-mediawiki git-svn lrzip glibc-doc
2020-04-10T19:42:13.9297327Z   gnupg | gnupg2 bzr gdbm-l10n krb5-doc krb5-user ncurses-doc libssl-doc
2020-04-10T19:42:13.9299043Z   libstdc++-7-doc make-doc ed diffutils-doc perl-doc libterm-readline-gnu-perl
2020-04-10T19:42:13.9299675Z   | libterm-readline-perl-perl python-doc python-tk ttf-bitstream-vera
2020-04-10T19:42:13.9300738Z   python3.6-venv python3.6-doc readline-doc
2020-04-10T19:42:13.9301008Z Recommended packages:
2020-04-10T19:42:13.9301508Z   build-essential fakeroot gnupg | gnupg2 libalgorithm-merge-perl libc-dbg
2020-04-10T19:42:13.9302590Z   gdbserver less ssh-client manpages manpages-dev libfile-fcntllock-perl
2020-04-10T19:42:13.9302590Z   gdbserver less ssh-client manpages manpages-dev libfile-fcntllock-perl
2020-04-10T19:42:13.9303310Z   liblocale-gettext-perl libglib2.0-data shared-mime-info xdg-user-dirs
2020-04-10T19:42:13.9303863Z   krb5-locales publicsuffix libsasl2-modules nodejs-doc netbase python-chardet
2020-04-10T19:42:13.9304349Z   python-pkg-resources python3-pkg-resources
2020-04-10T19:42:14.1948983Z   binutils binutils-common binutils-x86-64-linux-gnu ca-certificates cmake
2020-04-10T19:42:14.1949684Z   cmake-data cpp cpp-7 curl dpkg-dev file g++ g++-7 g++-7-multilib
2020-04-10T19:42:14.1950395Z   g++-multilib gcc gcc-7 gcc-7-base gcc-7-multilib gcc-multilib gdb git
2020-04-10T19:42:14.1951367Z   git-man lib32asan4 lib32atomic1 lib32cilkrts5 lib32gcc-7-dev lib32gcc1
---
2020-04-10T19:42:14.1962864Z   libsasl2-modules-db libsqlite3-0 libssl-dev libssl1.0.0 libssl1.1
2020-04-10T19:42:14.1963470Z   libstdc++-7-dev libtinfo-dev libtsan0 libubsan0 libuv1 libwind0-heimdal
2020-04-10T19:42:14.1966484Z   libx32asan4 libx32atomic1 libx32cilkrts5 libx32gcc-7-dev libx32gcc1
2020-04-10T19:42:14.1967175Z   libx32gomp1 libx32itm1 libx32quadmath0 libx32stdc++-7-dev libx32stdc++6
2020-04-10T19:42:14.1967774Z   libx32ubsan0 libxml2 libyaml-0-2 linux-libc-dev llvm-9-tools make
2020-04-10T19:42:14.1968825Z   mime-support multiarch-support nodejs openssl patch perl perl-modules-5.26
2020-04-10T19:42:14.1970099Z   pkg-config python python-minimal python-pygments python-yaml python2.7
2020-04-10T19:42:14.1971058Z   python2.7-minimal python3 python3-minimal python3-pygments python3-yaml
2020-04-10T19:42:14.1971967Z The following packages will be upgraded:
2020-04-10T19:42:14.1972421Z   gcc-8-base libgcc1 libstdc++6
2020-04-10T19:42:14.4607376Z 3 upgraded, 160 newly installed, 0 to remove and 9 not upgraded.
2020-04-10T19:42:14.4607875Z Need to get 114 MB of archives.
---
2020-04-10T19:42:25.6572994Z Unpacking libicu60:amd64 (60.2-3ubuntu3.1) ...
2020-04-10T19:42:26.7203055Z Selecting previously unselected package libxml2:amd64.
2020-04-10T19:42:26.7212884Z Preparing to unpack .../009-libxml2_2.9.4+dfsg1-6.1ubuntu1.3_amd64.deb ...
2020-04-10T19:42:26.7228834Z Unpacking libxml2:amd64 (2.9.4+dfsg1-6.1ubuntu1.3) ...
2020-04-10T19:42:26.8043305Z Selecting previously unselected package libyaml-0-2:amd64.
2020-04-10T19:42:26.8060475Z Preparing to unpack .../010-libyaml-0-2_0.1.7-2ubuntu3_amd64.deb ...
2020-04-10T19:42:26.8072226Z Unpacking libyaml-0-2:amd64 (0.1.7-2ubuntu3) ...
2020-04-10T19:42:26.8286307Z Selecting previously unselected package python3-yaml.
2020-04-10T19:42:26.8296070Z Preparing to unpack .../011-python3-yaml_3.12-1build2_amd64.deb ...
2020-04-10T19:42:26.8312773Z Unpacking python3-yaml (3.12-1build2) ...
2020-04-10T19:42:26.8684463Z Preparing to unpack .../012-sudo_1.8.21p2-3ubuntu1.2_amd64.deb ...
2020-04-10T19:42:26.8711684Z Unpacking sudo (1.8.21p2-3ubuntu1.2) ...
2020-04-10T19:42:26.9316647Z Selecting previously unselected package xz-utils.
2020-04-10T19:42:26.9329902Z Preparing to unpack .../013-xz-utils_5.2.2-1.3_amd64.deb ...
---
2020-04-10T19:42:38.1770721Z Unpacking libedit-dev:amd64 (3.1-20170329-1) ...
2020-04-10T19:42:38.2028799Z Selecting previously unselected package libssl-dev:amd64.
2020-04-10T19:42:38.2045923Z Preparing to unpack .../122-libssl-dev_1.1.1-1ubuntu2.1~18.04.5_amd64.deb ...
2020-04-10T19:42:38.2059630Z Unpacking libssl-dev:amd64 (1.1.1-1ubuntu2.1~18.04.5) ...
2020-04-10T19:42:38.3858616Z Selecting previously unselected package python-pygments.
2020-04-10T19:42:38.3876943Z Preparing to unpack .../123-python-pygments_2.2.0+dfsg-1_all.deb ...
2020-04-10T19:42:38.3955873Z Unpacking python-pygments (2.2.0+dfsg-1) ...
2020-04-10T19:42:38.4796981Z Selecting previously unselected package python-yaml.
2020-04-10T19:42:38.4797819Z Preparing to unpack .../124-python-yaml_3.12-1build2_amd64.deb ...
2020-04-10T19:42:38.4806227Z Unpacking python-yaml (3.12-1build2) ...
2020-04-10T19:42:38.5094272Z Selecting previously unselected package python3-pygments.
2020-04-10T19:42:38.5110668Z Preparing to unpack .../125-python3-pygments_2.2.0+dfsg-1_all.deb ...
2020-04-10T19:42:38.5122067Z Unpacking python3-pygments (2.2.0+dfsg-1) ...
2020-04-10T19:42:38.5987450Z Preparing to unpack .../126-llvm-9-tools_1%3a9-2~ubuntu18.04.2_amd64.deb ...
2020-04-10T19:42:38.6000418Z Unpacking llvm-9-tools (1:9-2~ubuntu18.04.2) ...
2020-04-10T19:42:38.6717014Z Selecting previously unselected package pkg-config.
2020-04-10T19:42:38.6735912Z Preparing to unpack .../127-pkg-config_0.29.1-0ubuntu2_amd64.deb ...
---
2020-04-10T19:43:03.0951528Z 100 13.3M  100 13.3M    0     0  8397k      0  0:00:01  0:00:01 --:--:-- 8392k
2020-04-10T19:43:03.0983843Z + chmod +x /usr/local/bin/sccache
2020-04-10T19:43:04.4215905Z Removing intermediate container 03e0c573a171
2020-04-10T19:43:04.4216505Z  ---> 6b54cffb5584
2020-04-10T19:43:04.4217503Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-config=llvm-config-9       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-10T19:43:05.1285758Z Removing intermediate container b27f99eea01c
2020-04-10T19:43:05.1287197Z  ---> 32a95bc70237
2020-04-10T19:43:05.1288308Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --target=i686-unknown-linux-gnu &&            python2.7 ../x.py test src/tools/tidy
2020-04-10T19:43:05.1758478Z  ---> Running in 495d8d2b8f3a
---
2020-04-10T19:43:08.0521055Z Successfully built 4275b8875265
2020-04-10T19:43:08.0610111Z Successfully tagged rust-ci:latest
2020-04-10T19:43:08.0927081Z Built container sha256:4275b8875265054fb2715035cb530ace675dfc35e81b3f9d4ede6e2a7a689080
2020-04-10T19:43:08.0942195Z Uploading finished image to https://rust-lang-ci-sccache2.s3.amazonaws.com/docker/6c20e639f7bc78e86170f4867ecce8ebc76c8fa9338e504259a13eed4ee8e6c4b6681f4a6857ff1141fb321a8c80c58239e3d498bac4776cdf41f2943715a0b0
2020-04-10T19:43:43.5694944Z upload failed: - to s3://rust-lang-ci-sccache2/docker/6c20e639f7bc78e86170f4867ecce8ebc76c8fa9338e504259a13eed4ee8e6c4b6681f4a6857ff1141fb321a8c80c58239e3d498bac4776cdf41f2943715a0b0 An error occurred (InvalidAccessKeyId) when calling the CreateMultipartUpload operation: The AWS Access Key Id you provided does not exist in our records.
2020-04-10T19:43:44.0347581Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-10T19:43:44.0374402Z == clock drift check ==
2020-04-10T19:43:44.0374402Z == clock drift check ==
2020-04-10T19:43:44.0382156Z   local time: Fri Apr 10 19:43:44 UTC 2020
2020-04-10T19:43:44.2245656Z   network time: Fri, 10 Apr 2020 19:43:44 GMT
2020-04-10T19:43:44.2273066Z Starting sccache server...
2020-04-10T19:43:44.3242325Z configure: processing command line
2020-04-10T19:43:44.3243473Z configure: 
2020-04-10T19:43:44.3244357Z configure: build.build          := x86_64-unknown-linux-gnu
---
2020-04-10T19:45:33.3157319Z make: *** [prepare] Error 1
2020-04-10T19:45:33.3158008Z Makefile:67: recipe for target 'prepare' failed
2020-04-10T19:45:33.3161865Z The command has failed after 5 attempts.
2020-04-10T19:45:33.3165009Z == clock drift check ==
2020-04-10T19:45:33.3172841Z   local time: Fri Apr 10 19:45:33 UTC 2020
2020-04-10T19:45:33.6336865Z   network time: Fri, 10 Apr 2020 19:45:33 GMT
2020-04-10T19:45:39.3316404Z 
2020-04-10T19:45:39.3316404Z 
2020-04-10T19:45:39.3351182Z ##[error]Bash exited with code '1'.
2020-04-10T19:45:39.3373947Z ##[section]Finishing: Run build
2020-04-10T19:45:39.3420819Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70989/merge to s
2020-04-10T19:45:39.3425925Z Task         : Get sources
2020-04-10T19:45:39.3426229Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-10T19:45:39.3426736Z Version      : 1.0.0
2020-04-10T19:45:39.3426958Z Author       : Microsoft
2020-04-10T19:45:39.3426958Z Author       : Microsoft
2020-04-10T19:45:39.3427276Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-10T19:45:39.3427648Z ==============================================================================
2020-04-10T19:45:39.6603820Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-10T19:45:39.6646200Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70989/merge to s
2020-04-10T19:45:39.6737491Z Cleaning up task key
2020-04-10T19:45:39.6738662Z Start cleaning up orphan processes.
2020-04-10T19:45:39.6928587Z Terminate orphan process: pid (3941) (python)
2020-04-10T19:45:39.7063199Z ##[section]Finishing: Finalize Job
