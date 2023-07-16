plain
2019-07-14T02:23:32.3458015Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-14T02:23:32.3614298Z ##[command]git config gc.auto 0
2019-07-14T02:23:32.3681247Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-14T02:23:33.2244605Z ##[command]git config --get-all http.proxy
2019-07-14T02:23:33.2252249Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62669/merge:refs/remotes/pull/62669/merge
---
2019-07-14T02:24:07.2039261Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-14T02:24:07.2039302Z 
2019-07-14T02:24:07.2040346Z   git checkout -b <new-branch-name>
2019-07-14T02:24:07.2040389Z 
2019-07-14T02:24:07.2041067Z HEAD is now at 3f57a9978 Merge 810802188bd0d2a0314f3ee816151a5ee379c2fe into d32a7250dbf797c9a89f56de0842d7ad43bfe85f
2019-07-14T02:24:07.2203900Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-14T02:24:07.2207309Z ==============================================================================
2019-07-14T02:24:07.2207390Z Task         : Bash
2019-07-14T02:24:07.2207442Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-14T02:25:59.3480110Z Attempting with retry: curl -y 30 -Y 10 --connect-timeout 30 -f -L -C - -o /tmp/rustci_docker_cache https://.s3.amazonaws.com/docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624
2019-07-14T02:25:59.3531134Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-14T02:25:59.3531430Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-14T02:25:59.3531590Z 
2019-07-14T02:25:59.3574217Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-14T02:26:00.3648644Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-14T02:26:00.3649142Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-14T02:26:00.3649307Z 
2019-07-14T02:26:00.3649307Z 
2019-07-14T02:26:00.3694173Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-14T02:26:02.3784460Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-14T02:26:02.3786760Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-14T02:26:02.3787096Z 
2019-07-14T02:26:02.3787096Z 
2019-07-14T02:26:02.3791281Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-14T02:26:05.3880711Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-14T02:26:05.3881250Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-14T02:26:05.3881385Z 
2019-07-14T02:26:05.3881385Z 
2019-07-14T02:26:05.3924416Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-14T02:26:09.4026225Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-14T02:26:09.4029004Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-14T02:26:09.4029330Z 
2019-07-14T02:26:09.4029330Z 
2019-07-14T02:26:09.4030376Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-14T02:26:09.4034889Z The command has failed after 5 attempts.
2019-07-14T02:26:09.4614352Z open /tmp/rustci_docker_cache: no such file or directory
2019-07-14T02:26:09.4641313Z Attempting with retry: docker build --rm -t rust-ci -f /home/vsts/work/1/s/src/ci/docker/x86_64-gnu-llvm-6.0/Dockerfile /home/vsts/work/1/s/src/ci/docker
2019-07-14T02:26:09.5697810Z Sending build context to Docker daemon  521.7kB
2019-07-14T02:26:09.5698623Z 
2019-07-14T02:26:09.5857062Z Step 1/8 : FROM ubuntu:16.04
---
2019-07-14T02:26:26.7381985Z Reading package lists...
2019-07-14T02:26:27.8020469Z Reading package lists...
2019-07-14T02:26:27.9821932Z Building dependency tree...
2019-07-14T02:26:27.9822044Z Reading state information...
2019-07-14T02:26:28.1143159Z The following additional packages will be installed:
2019-07-14T02:26:28.1179394Z   binfmt-support binutils bzip2 cmake-data cpp cpp-5 dpkg-dev g++-5 gcc gcc-5
2019-07-14T02:26:28.1179916Z   git-man libarchive13 libasan2 libasn1-8-heimdal libatomic1
2019-07-14T02:26:28.1180501Z   libbabeltrace-ctf1 libbabeltrace1 libbsd-dev libbsd0 libbz2-1.0 libc-dev-bin
2019-07-14T02:26:28.1181045Z   libedit2 liberror-perl libexpat1 libffi-dev libffi6 libgcc-5-dev libgdbm3
2019-07-14T02:26:28.1181349Z   libglib2.0-0 libgmp10 libgnutls30 libgomp1 libgssapi-krb5-2
2019-07-14T02:26:28.1181604Z   libgssapi3-heimdal libhcrypto4-heimdal libheimbase1-heimdal
2019-07-14T02:26:28.1181942Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-14T02:26:28.1181942Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-14T02:26:28.1182282Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-14T02:26:28.1182832Z   libkrb5-3 libkrb5support0 libldap-2.4-2 libllvm6.0 liblsan0 liblzo2-2
2019-07-14T02:26:28.1183099Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-14T02:26:28.1183382Z   libperl5.22 libpipeline1 libpython-stdlib libpython2.7-minimal
2019-07-14T02:26:28.1183653Z   libpython2.7-stdlib libpython3.5 libpython3.5-minimal libpython3.5-stdlib
2019-07-14T02:26:28.1183919Z   libquadmath0 libroken18-heimdal librtmp1 libsasl2-2 libsasl2-modules-db
2019-07-14T02:26:28.1184250Z   libsqlite3-0 libssl1.0.0 libstdc++-5-dev libtasn1-6 libtinfo-dev libtsan0
2019-07-14T02:26:28.1184518Z   libubsan0 libwind0-heimdal libxml2 linux-libc-dev llvm-6.0 llvm-6.0-dev
2019-07-14T02:26:28.1184783Z   llvm-6.0-runtime mime-support openssl patch perl perl-modules-5.22 python
2019-07-14T02:26:28.1185073Z   python-minimal python2.7-minimal
2019-07-14T02:26:28.1185143Z Suggested packages:
2019-07-14T02:26:28.1185417Z   binutils-doc bzip2-doc codeblocks eclipse ninja-build cpp-doc gcc-5-locales
2019-07-14T02:26:28.1185738Z   debian-keyring g++-multilib g++-5-multilib gcc-5-doc libstdc++6-5-dbg
2019-07-14T02:26:28.1186009Z   gcc-multilib manpages-dev autoconf automake libtool flex bison gcc-doc
2019-07-14T02:26:28.1186579Z   libasan2-dbg liblsan0-dbg libtsan0-dbg libubsan0-dbg libcilkrts5-dbg
2019-07-14T02:26:28.1186846Z   libmpx0-dbg libquadmath0-dbg gdb-doc gettext-base git-daemon-run
2019-07-14T02:26:28.1186846Z   libmpx0-dbg libquadmath0-dbg gdb-doc gettext-base git-daemon-run
2019-07-14T02:26:28.1187112Z   | git-daemon-sysvinit git-doc git-el git-email git-gui gitk gitweb git-arch
2019-07-14T02:26:28.1187425Z   git-cvs git-mediawiki git-svn lrzip glibc-doc gnutls-bin krb5-doc krb5-user
2019-07-14T02:26:28.1187690Z   libstdc++-5-doc llvm-6.0-doc make-doc ed diffutils-doc perl-doc
2019-07-14T02:26:28.1187968Z   libterm-readline-gnu-perl | libterm-readline-perl-perl python-doc python-tk
2019-07-14T02:26:28.1188230Z   python2.7-doc
2019-07-14T02:26:28.1188290Z Recommended packages:
2019-07-14T02:26:28.1188556Z   build-essential fakeroot libalgorithm-merge-perl libc-dbg gdbserver less
2019-07-14T02:26:28.1188858Z   rsync ssh-client manpages manpages-dev libfile-fcntllock-perl
2019-07-14T02:26:28.1189262Z   libglib2.0-data shared-mime-info xdg-user-dirs krb5-locales libsasl2-modules
2019-07-14T02:26:28.1189487Z   libssl-doc xml-core netbase rename
2019-07-14T02:26:28.1189538Z The following NEW packages will be installed:
2019-07-14T02:26:28.1189849Z   binfmt-support binutils bzip2 ca-certificates cmake cmake-data cpp cpp-5
2019-07-14T02:26:28.1190297Z   curl dpkg-dev file g++ g++-5 gcc gcc-5 gdb git git-man libarchive13 libasan2
2019-07-14T02:26:28.1190570Z   libasn1-8-heimdal libatomic1 libbabeltrace-ctf1 libbabeltrace1 libbsd-dev
2019-07-14T02:26:28.1191029Z   libbsd0 libc-dev-bin libc6-dev libcc1-0 libcilkrts5 libcurl3 libcurl3-gnutls
2019-07-14T02:26:28.1191342Z   libdpkg-perl libedit-dev libedit2 liberror-perl libexpat1 libffi-dev libffi6
2019-07-14T02:26:28.1191619Z   libgcc-5-dev libgdbm3 libglib2.0-0 libgmp10 libgnutls30 libgomp1
2019-07-14T02:26:28.1192207Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-14T02:26:28.1192476Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-14T02:26:28.1192799Z   libkrb5-3 libkrb5support0 libldap-2.4-2 libllvm6.0 liblsan0 liblzo2-2
2019-07-14T02:26:28.1193062Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-14T02:26:28.1193062Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-14T02:26:28.1193319Z   libperl5.22 libpipeline1 libpython-stdlib libpython2.7-minimal
2019-07-14T02:26:28.1193640Z   libpython2.7-stdlib libpython3.5 libpython3.5-minimal libpython3.5-stdlib
2019-07-14T02:26:28.1193916Z   libquadmath0 libroken18-heimdal librtmp1 libsasl2-2 libsasl2-modules-db
2019-07-14T02:26:28.1194190Z   libsqlite3-0 libssl-dev libssl1.0.0 libstdc++-5-dev libtasn1-6 libtinfo-dev
2019-07-14T02:26:28.1194623Z   libtsan0 libubsan0 libwind0-heimdal libxml2 linux-libc-dev llvm-6.0
2019-07-14T02:26:28.1194894Z   llvm-6.0-dev llvm-6.0-runtime llvm-6.0-tools make mime-support openssl patch
2019-07-14T02:26:28.1195152Z   perl perl-modules-5.22 pkg-config python python-minimal python2.7
2019-07-14T02:26:28.1195450Z   python2.7-minimal sudo xz-utils zlib1g-dev
2019-07-14T02:26:28.1195503Z The following packages will be upgraded:
2019-07-14T02:26:28.7792537Z 1 upgraded, 115 newly installed, 0 to remove and 4 not upgraded.
2019-07-14T02:26:28.7792768Z Need to get 121 MB of archives.
2019-07-14T02:26:28.7792828Z After this operation, 592 MB of additional disk space will be used.
2019-07-14T02:26:28.7794260Z Get:1 http://archive.ubuntu.com/ubuntu xenial/main amd64 libgdbm3 amd64 1.8.3-13.1 [16.9 kB]
---
2019-07-14T02:26:29.5933569Z Get:61 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 sudo amd64 1.8.16-0ubuntu1.7 [390 kB]
2019-07-14T02:26:29.5998398Z Get:62 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 openssl amd64 1.0.2g-1ubuntu4.15 [492 kB]
2019-07-14T02:26:29.6086958Z Get:63 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 ca-certificates all 20170717~16.04.2 [167 kB]
2019-07-14T02:26:29.6119308Z Get:64 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 libcurl3-gnutls amd64 7.47.0-1ubuntu2.13 [184 kB]
2019-07-14T02:26:29.6151871Z Get:65 http://archive.ubuntu.com/ubuntu xenial/main amd64 libedit2 amd64 3.1-20150325-1ubuntu2 [76.5 kB]
2019-07-14T02:26:29.6169374Z Get:66 http://archive.ubuntu.com/ubuntu xenial/main amd64 libpipeline1 amd64 1.4.1-2 [24.6 kB]
2019-07-14T02:26:29.6177642Z Get:67 http://archive.ubuntu.com/ubuntu xenial/main amd64 binfmt-support amd64 2.1.6-1 [50.7 kB]
2019-07-14T02:26:29.6729339Z Get:69 http://archive.ubuntu.com/ubuntu xenial/main amd64 libisl15 amd64 0.16.1-1 [524 kB]
2019-07-14T02:26:29.6878306Z Get:70 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 cpp-5 amd64 5.4.0-6ubuntu1~16.04.11 [7660 kB]
2019-07-14T02:26:29.8186584Z Get:71 http://archive.ubuntu.com/ubuntu xenial/main amd64 cpp amd64 4:5.3.1-1ubuntu1 [27.7 kB]
2019-07-14T02:26:29.8187644Z Get:72 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 curl amd64 7.47.0-1ubuntu2.13 [139 kB]
---
2019-07-14T02:26:47.7596832Z Unpacking ca-certificates (20170717~16.04.2) ...
2019-07-14T02:26:47.9236914Z Selecting previously unselected package libcurl3-gnutls:amd64.
2019-07-14T02:26:47.9249701Z Preparing to unpack .../libcurl3-gnutls_7.47.0-1ubuntu2.13_amd64.deb ...
2019-07-14T02:26:47.9375261Z Unpacking libcurl3-gnutls:amd64 (7.47.0-1ubuntu2.13) ...
2019-07-14T02:26:48.0684388Z Selecting previously unselected package libedit2:amd64.
2019-07-14T02:26:48.0704674Z Preparing to unpack .../libedit2_3.1-20150325-1ubuntu2_amd64.deb ...
2019-07-14T02:26:48.0831907Z Unpacking libedit2:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-14T02:26:48.1925547Z Selecting previously unselected package libpipeline1:amd64.
2019-07-14T02:26:48.1945077Z Preparing to unpack .../libpipeline1_1.4.1-2_amd64.deb ...
2019-07-14T02:26:48.2073880Z Unpacking libpipeline1:amd64 (1.4.1-2) ...
2019-07-14T02:26:48.3147279Z Selecting previously unselected package binfmt-support.
2019-07-14T02:26:48.3165519Z Preparing to unpack .../binfmt-support_2.1.6-1_amd64.deb ...
2019-07-14T02:26:48.3295007Z Unpacking binfmt-support (2.1.6-1) ...
2019-07-14T02:26:48.4438468Z Preparing to unpack .../binutils_2.26.1-1ubuntu1~16.04.8_amd64.deb ...
2019-07-14T02:26:48.4567321Z Unpacking binutils (2.26.1-1ubuntu1~16.04.8) ...
2019-07-14T02:26:48.9420713Z Selecting previously unselected package libisl15:amd64.
2019-07-14T02:26:48.9440988Z Preparing to unpack .../libisl15_0.16.1-1_amd64.deb ...
---
2019-07-14T02:27:00.7883837Z Selecting previously unselected package libtinfo-dev:amd64.
2019-07-14T02:27:00.7903530Z Preparing to unpack .../libtinfo-dev_6.0+20160213-1ubuntu1_amd64.deb ...
2019-07-14T02:27:00.8032335Z Unpacking libtinfo-dev:amd64 (6.0+20160213-1ubuntu1) ...
2019-07-14T02:27:00.9043461Z Selecting previously unselected package libedit-dev:amd64.
2019-07-14T02:27:00.9064921Z Preparing to unpack .../libedit-dev_3.1-20150325-1ubuntu2_amd64.deb ...
2019-07-14T02:27:00.9189334Z Unpacking libedit-dev:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-14T02:27:01.0396508Z Selecting previously unselected package libllvm6.0:amd64.
2019-07-14T02:27:01.0419264Z Preparing to unpack .../libllvm6.0_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-14T02:27:01.0552385Z Unpacking libllvm6.0:amd64 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-14T02:27:04.0893238Z Selecting previously unselected package zlib1g-dev:amd64.
2019-07-14T02:27:04.0914644Z Preparing to unpack .../zlib1g-dev_1%3a1.2.8.dfsg-2ubuntu4.1_amd64.deb ...
2019-07-14T02:27:04.1041537Z Unpacking zlib1g-dev:amd64 (1:1.2.8.dfsg-2ubuntu4.1) ...
2019-07-14T02:27:04.2108881Z Preparing to unpack .../libssl-dev_1.0.2g-1ubuntu4.15_amd64.deb ...
2019-07-14T02:27:04.2254711Z Unpacking libssl-dev:amd64 (1.0.2g-1ubuntu4.15) ...
2019-07-14T02:27:04.5471521Z Selecting previously unselected package llvm-6.0-runtime.
2019-07-14T02:27:04.5471521Z Selecting previously unselected package llvm-6.0-runtime.
2019-07-14T02:27:04.5496555Z Preparing to unpack .../llvm-6.0-runtime_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-14T02:27:04.5636993Z Unpacking llvm-6.0-runtime (1:6.0-1ubuntu2~16.04.1) ...
2019-07-14T02:27:04.6820776Z Selecting previously unselected package llvm-6.0.
2019-07-14T02:27:04.6843866Z Preparing to unpack .../llvm-6.0_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-14T02:27:04.6966817Z Unpacking llvm-6.0 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-14T02:27:05.3804644Z Selecting previously unselected package libffi-dev:amd64.
2019-07-14T02:27:05.3824964Z Preparing to unpack .../libffi-dev_3.2.1-4_amd64.deb ...
2019-07-14T02:27:05.3961609Z Unpacking libffi-dev:amd64 (3.2.1-4) ...
2019-07-14T02:27:05.5133967Z Selecting previously unselected package llvm-6.0-dev.
2019-07-14T02:27:05.5156340Z Preparing to unpack .../llvm-6.0-dev_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-14T02:27:05.5309768Z Unpacking llvm-6.0-dev (1:6.0-1ubuntu2~16.04.1) ...
2019-07-14T02:27:10.2167031Z Selecting previously unselected package llvm-6.0-tools.
2019-07-14T02:27:10.2193566Z Preparing to unpack .../llvm-6.0-tools_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-14T02:27:10.2326122Z Unpacking llvm-6.0-tools (1:6.0-1ubuntu2~16.04.1) ...
2019-07-14T02:27:10.4189688Z Selecting previously unselected package pkg-config.
2019-07-14T02:27:10.4216993Z Preparing to unpack .../pkg-config_0.29.1-0ubuntu1_amd64.deb ...
2019-07-14T02:27:10.4347225Z Unpacking pkg-config (0.29.1-0ubuntu1) ...
2019-07-14T02:27:10.5475006Z Processing triggers for systemd (229-4ubuntu21.21) ...
2019-07-14T02:27:10.8990572Z Setting up libgdbm3:amd64 (1.8.3-13.1) ...
2019-07-14T02:27:10.9665327Z Setting up libffi6:amd64 (3.2.1-4) ...
2019-07-14T02:27:11.0047881Z Setting up libglib2.0-0:amd64 (2.48.2-0ubuntu4.3) ...
---
2019-07-14T02:27:14.6689967Z debconf: unable to initialize frontend: Dialog
2019-07-14T02:27:14.6690939Z debconf: (TERM is not set, so the dialog frontend is not usable.)
2019-07-14T02:27:14.6691292Z debconf: falling back to frontend: Readline
2019-07-14T02:27:15.2054576Z Setting up libcurl3-gnutls:amd64 (7.47.0-1ubuntu2.13) ...
2019-07-14T02:27:15.2429440Z Setting up libedit2:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-14T02:27:15.2840260Z Setting up libpipeline1:amd64 (1.4.1-2) ...
2019-07-14T02:27:15.3218301Z Setting up binfmt-support (2.1.6-1) ...
2019-07-14T02:27:15.3906262Z mount: permission denied
2019-07-14T02:27:15.3907309Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-14T02:27:15.3921392Z mount: permission denied
2019-07-14T02:27:15.3926056Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-14T02:27:15.5759509Z invoke-rc.d: could not determine current runlevel
2019-07-14T02:27:15.5792455Z invoke-rc.d: policy-rc.d denied execution of start.
2019-07-14T02:27:15.6352831Z Setting up libisl15:amd64 (0.16.1-1) ...
2019-07-14T02:27:15.6731260Z Setting up cpp-5 (5.4.0-6ubuntu1~16.04.11) ...
2019-07-14T02:27:15.7116834Z Setting up cpp (4:5.3.1-1ubuntu1) ...
2019-07-14T02:27:15.7586291Z Setting up curl (7.47.0-1ubuntu2.13) ...
---
2019-07-14T02:27:17.3294362Z Setting up libedit-dev:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-14T02:27:17.3671790Z Setting up libllvm6.0:amd64 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-14T02:27:17.4077121Z Setting up zlib1g-dev:amd64 (1:1.2.8.dfsg-2ubuntu4.1) ...
2019-07-14T02:27:17.4477486Z Setting up libssl-dev:amd64 (1.0.2g-1ubuntu4.15) ...
2019-07-14T02:27:17.4935165Z Setting up llvm-6.0-runtime (1:6.0-1ubuntu2~16.04.1) ...
2019-07-14T02:27:17.5238384Z mount: permission denied
2019-07-14T02:27:17.5243766Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-14T02:27:17.5373447Z Setting up llvm-6.0 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-14T02:27:17.5748874Z Setting up libffi-dev:amd64 (3.2.1-4) ...
2019-07-14T02:27:17.6128416Z Setting up llvm-6.0-dev (1:6.0-1ubuntu2~16.04.1) ...
2019-07-14T02:27:17.6498929Z Setting up llvm-6.0-tools (1:6.0-1ubuntu2~16.04.1) ...
2019-07-14T02:27:17.6875435Z Setting up pkg-config (0.29.1-0ubuntu1) ...
2019-07-14T02:27:17.8154229Z Processing triggers for ca-certificates (20170717~16.04.2) ...
2019-07-14T02:27:17.8318123Z Updating certificates in /etc/ssl/certs...
2019-07-14T02:27:19.4839099Z 148 added, 0 removed; done.
2019-07-14T02:27:19.4839845Z Running hooks in /etc/ca-certificates/update.d...
---
2019-07-14T02:27:52.4010457Z Removing intermediate container 65c60f7e0e96
2019-07-14T02:27:52.4011441Z  ---> d632bdf2ec82
2019-07-14T02:27:52.4056012Z Successfully built d632bdf2ec82
2019-07-14T02:27:52.4894832Z Successfully tagged rust-ci:latest
2019-07-14T02:27:52.6050937Z Built container sha256:d632bdf2ec82de7b582cb6a589b063475a2cc39944fe1c4d261cf21065c58833
2019-07-14T02:27:52.6082782Z Uploading finished image to https://.s3.amazonaws.com/docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624
2019-07-14T02:28:56.0800907Z upload failed: - to s3:///docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624 Parameter validation failed:
2019-07-14T02:28:56.0802056Z Invalid bucket name "": Bucket name must match the regex "^[a-zA-Z0-9.\-_]{1,255}$"
2019-07-14T02:28:57.0175585Z [CI_JOB_NAME=x86_64-gnu-llvm-6.0]
2019-07-14T02:28:57.0229803Z Starting sccache server...
2019-07-14T02:28:57.0782286Z configure: processing command line
2019-07-14T02:28:57.0783040Z configure: 
---
2019-07-14T02:32:27.6063450Z    Compiling serde_json v1.0.33
2019-07-14T02:32:31.9431652Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-07-14T02:32:40.7761433Z     Finished release [optimized] target(s) in 1m 32s
2019-07-14T02:32:40.7834403Z tidy check
2019-07-14T02:32:40.9281286Z tidy error: /checkout/src/librustc_resolve/lib.rs:4181: line longer than 100 chars
2019-07-14T02:32:42.7268759Z some tidy checks failed
2019-07-14T02:32:42.7269118Z 
2019-07-14T02:32:42.7269118Z 
2019-07-14T02:32:42.7271992Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-07-14T02:32:42.7272546Z 
2019-07-14T02:32:42.7272969Z 
2019-07-14T02:32:42.7286107Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-07-14T02:32:42.7286330Z Build completed unsuccessfully in 0:01:35
2019-07-14T02:32:42.7286330Z Build completed unsuccessfully in 0:01:35
2019-07-14T02:32:44.1399436Z ##[error]Bash exited with code '1'.
2019-07-14T02:32:44.1434122Z ##[section]Starting: Checkout
2019-07-14T02:32:44.1436346Z ==============================================================================
2019-07-14T02:32:44.1436409Z Task         : Get sources
2019-07-14T02:32:44.1436479Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
