plain
2019-07-10T14:10:26.3733944Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-10T14:10:26.3915994Z ##[command]git config gc.auto 0
2019-07-10T14:10:26.3981612Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-10T14:10:26.4053338Z ##[command]git config --get-all http.proxy
2019-07-10T14:10:26.4200925Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62077/merge:refs/remotes/pull/62077/merge
---
2019-07-10T14:11:01.0064009Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-10T14:11:01.0065688Z 
2019-07-10T14:11:01.0069918Z   git checkout -b <new-branch-name>
2019-07-10T14:11:01.0071840Z 
2019-07-10T14:11:01.0073049Z HEAD is now at b097bf0fa Merge 00cc9de41efe825fd494c5e81ed46df79b308a41 into d4e15655092d1bdae79619eb0ff2c3cb5468fc36
2019-07-10T14:11:01.0201322Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-10T14:11:01.0203709Z ==============================================================================
2019-07-10T14:11:01.0203756Z Task         : Bash
2019-07-10T14:11:01.0203808Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-10T14:12:51.3243171Z Attempting with retry: curl -y 30 -Y 10 --connect-timeout 30 -f -L -C - -o /tmp/rustci_docker_cache https://.s3.amazonaws.com/docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624
2019-07-10T14:12:51.3302309Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-10T14:12:51.3302745Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-10T14:12:51.3302989Z 
2019-07-10T14:12:51.3303815Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-10T14:12:52.3368638Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-10T14:12:52.3368969Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-10T14:12:52.3369105Z 
2019-07-10T14:12:52.3369105Z 
2019-07-10T14:12:52.3416607Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-10T14:12:54.3479316Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-10T14:12:54.3479616Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-10T14:12:54.3479731Z 
2019-07-10T14:12:54.3479731Z 
2019-07-10T14:12:54.3520522Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-10T14:12:57.3594110Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-10T14:12:57.3594185Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-10T14:12:57.3594217Z 
2019-07-10T14:12:57.3594217Z 
2019-07-10T14:12:57.3627041Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-10T14:13:01.3701395Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-10T14:13:01.3701573Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-10T14:13:01.3701607Z 
2019-07-10T14:13:01.3701607Z 
2019-07-10T14:13:01.3737327Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-10T14:13:01.3740199Z The command has failed after 5 attempts.
2019-07-10T14:13:01.4095484Z open /tmp/rustci_docker_cache: no such file or directory
2019-07-10T14:13:01.4114675Z Attempting with retry: docker build --rm -t rust-ci -f /home/vsts/work/1/s/src/ci/docker/x86_64-gnu-llvm-6.0/Dockerfile /home/vsts/work/1/s/src/ci/docker
2019-07-10T14:13:01.5737249Z Sending build context to Docker daemon  521.7kB
2019-07-10T14:13:01.5737958Z 
2019-07-10T14:13:01.5995125Z Step 1/8 : FROM ubuntu:16.04
---
2019-07-10T14:13:17.0927314Z Reading package lists...
2019-07-10T14:13:17.8884880Z Reading package lists...
2019-07-10T14:13:18.0227010Z Building dependency tree...
2019-07-10T14:13:18.0227093Z Reading state information...
2019-07-10T14:13:18.1214963Z The following additional packages will be installed:
2019-07-10T14:13:18.1215877Z   binfmt-support binutils bzip2 cmake-data cpp cpp-5 dpkg-dev g++-5 gcc gcc-5
2019-07-10T14:13:18.1216101Z   git-man libarchive13 libasan2 libasn1-8-heimdal libatomic1
2019-07-10T14:13:18.1218156Z   libbabeltrace-ctf1 libbabeltrace1 libbsd-dev libbsd0 libbz2-1.0 libc-dev-bin
2019-07-10T14:13:18.1219091Z   libedit2 liberror-perl libexpat1 libffi-dev libffi6 libgcc-5-dev libgdbm3
2019-07-10T14:13:18.1219513Z   libglib2.0-0 libgmp10 libgnutls30 libgomp1 libgssapi-krb5-2
2019-07-10T14:13:18.1219713Z   libgssapi3-heimdal libhcrypto4-heimdal libheimbase1-heimdal
2019-07-10T14:13:18.1219930Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-10T14:13:18.1219930Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-10T14:13:18.1220137Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-10T14:13:18.1220390Z   libkrb5-3 libkrb5support0 libldap-2.4-2 libllvm6.0 liblsan0 liblzo2-2
2019-07-10T14:13:18.1222541Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-10T14:13:18.1223321Z   libperl5.22 libpipeline1 libpython-stdlib libpython2.7-minimal
2019-07-10T14:13:18.1223699Z   libpython2.7-stdlib libpython3.5 libpython3.5-minimal libpython3.5-stdlib
2019-07-10T14:13:18.1223959Z   libquadmath0 libroken18-heimdal librtmp1 libsasl2-2 libsasl2-modules-db
2019-07-10T14:13:18.1224355Z   libsqlite3-0 libssl1.0.0 libstdc++-5-dev libtasn1-6 libtinfo-dev libtsan0
2019-07-10T14:13:18.1224591Z   libubsan0 libwind0-heimdal libxml2 linux-libc-dev llvm-6.0 llvm-6.0-dev
2019-07-10T14:13:18.1224780Z   llvm-6.0-runtime mime-support openssl patch perl perl-modules-5.22 python
2019-07-10T14:13:18.1224945Z   python-minimal python2.7-minimal
2019-07-10T14:13:18.1225032Z Suggested packages:
2019-07-10T14:13:18.1225225Z   binutils-doc bzip2-doc codeblocks eclipse ninja-build cpp-doc gcc-5-locales
2019-07-10T14:13:18.1225412Z   debian-keyring g++-multilib g++-5-multilib gcc-5-doc libstdc++6-5-dbg
2019-07-10T14:13:18.1256498Z   gcc-multilib manpages-dev autoconf automake libtool flex bison gcc-doc
2019-07-10T14:13:18.1257157Z   libasan2-dbg liblsan0-dbg libtsan0-dbg libubsan0-dbg libcilkrts5-dbg
2019-07-10T14:13:18.1257386Z   libmpx0-dbg libquadmath0-dbg gdb-doc gettext-base git-daemon-run
2019-07-10T14:13:18.1257386Z   libmpx0-dbg libquadmath0-dbg gdb-doc gettext-base git-daemon-run
2019-07-10T14:13:18.1257586Z   | git-daemon-sysvinit git-doc git-el git-email git-gui gitk gitweb git-arch
2019-07-10T14:13:18.1257781Z   git-cvs git-mediawiki git-svn lrzip glibc-doc gnutls-bin krb5-doc krb5-user
2019-07-10T14:13:18.1257995Z   libstdc++-5-doc llvm-6.0-doc make-doc ed diffutils-doc perl-doc
2019-07-10T14:13:18.1258198Z   libterm-readline-gnu-perl | libterm-readline-perl-perl python-doc python-tk
2019-07-10T14:13:18.1258353Z   python2.7-doc
2019-07-10T14:13:18.1258418Z Recommended packages:
2019-07-10T14:13:18.1258702Z   build-essential fakeroot libalgorithm-merge-perl libc-dbg gdbserver less
2019-07-10T14:13:18.1258888Z   rsync ssh-client manpages manpages-dev libfile-fcntllock-perl
2019-07-10T14:13:18.1259117Z   libglib2.0-data shared-mime-info xdg-user-dirs krb5-locales libsasl2-modules
2019-07-10T14:13:18.1259302Z   libssl-doc xml-core netbase rename
2019-07-10T14:13:18.1259341Z The following NEW packages will be installed:
2019-07-10T14:13:18.1259563Z   binfmt-support binutils bzip2 ca-certificates cmake cmake-data cpp cpp-5
2019-07-10T14:13:18.1259765Z   curl dpkg-dev file g++ g++-5 gcc gcc-5 gdb git git-man libarchive13 libasan2
2019-07-10T14:13:18.1260145Z   libasn1-8-heimdal libatomic1 libbabeltrace-ctf1 libbabeltrace1 libbsd-dev
2019-07-10T14:13:18.1260383Z   libbsd0 libc-dev-bin libc6-dev libcc1-0 libcilkrts5 libcurl3 libcurl3-gnutls
2019-07-10T14:13:18.1261281Z   libdpkg-perl libedit-dev libedit2 liberror-perl libexpat1 libffi-dev libffi6
2019-07-10T14:13:18.1261624Z   libgcc-5-dev libgdbm3 libglib2.0-0 libgmp10 libgnutls30 libgomp1
2019-07-10T14:13:18.1262213Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-10T14:13:18.1262480Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-10T14:13:18.1262892Z   libkrb5-3 libkrb5support0 libldap-2.4-2 libllvm6.0 liblsan0 liblzo2-2
2019-07-10T14:13:18.1263161Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-10T14:13:18.1263161Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-10T14:13:18.1263413Z   libperl5.22 libpipeline1 libpython-stdlib libpython2.7-minimal
2019-07-10T14:13:18.1263720Z   libpython2.7-stdlib libpython3.5 libpython3.5-minimal libpython3.5-stdlib
2019-07-10T14:13:18.1263991Z   libquadmath0 libroken18-heimdal librtmp1 libsasl2-2 libsasl2-modules-db
2019-07-10T14:13:18.1264405Z   libsqlite3-0 libssl-dev libssl1.0.0 libstdc++-5-dev libtasn1-6 libtinfo-dev
2019-07-10T14:13:18.1264635Z   libtsan0 libubsan0 libwind0-heimdal libxml2 linux-libc-dev llvm-6.0
2019-07-10T14:13:18.1264846Z   llvm-6.0-dev llvm-6.0-runtime llvm-6.0-tools make mime-support openssl patch
2019-07-10T14:13:18.1265043Z   perl perl-modules-5.22 pkg-config python python-minimal python2.7
2019-07-10T14:13:18.1265245Z   python2.7-minimal sudo xz-utils zlib1g-dev
2019-07-10T14:13:18.1265295Z The following packages will be upgraded:
2019-07-10T14:13:18.9724643Z 1 upgraded, 115 newly installed, 0 to remove and 4 not upgraded.
2019-07-10T14:13:18.9725690Z Need to get 121 MB of archives.
2019-07-10T14:13:18.9725801Z After this operation, 592 MB of additional disk space will be used.
2019-07-10T14:13:18.9726931Z Get:1 http://archive.ubuntu.com/ubuntu xenial/main amd64 libgdbm3 amd64 1.8.3-13.1 [16.9 kB]
---
2019-07-10T14:13:20.5482065Z Get:61 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 sudo amd64 1.8.16-0ubuntu1.7 [390 kB]
2019-07-10T14:13:20.5523791Z Get:62 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 openssl amd64 1.0.2g-1ubuntu4.15 [492 kB]
2019-07-10T14:13:20.6137919Z Get:63 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 ca-certificates all 20170717~16.04.2 [167 kB]
2019-07-10T14:13:20.6164998Z Get:64 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 libcurl3-gnutls amd64 7.47.0-1ubuntu2.13 [184 kB]
2019-07-10T14:13:20.6195255Z Get:65 http://archive.ubuntu.com/ubuntu xenial/main amd64 libedit2 amd64 3.1-20150325-1ubuntu2 [76.5 kB]
2019-07-10T14:13:20.6210002Z Get:66 http://archive.ubuntu.com/ubuntu xenial/main amd64 libpipeline1 amd64 1.4.1-2 [24.6 kB]
2019-07-10T14:13:20.6217862Z Get:67 http://archive.ubuntu.com/ubuntu xenial/main amd64 binfmt-support amd64 2.1.6-1 [50.7 kB]
2019-07-10T14:13:20.7325637Z Get:69 http://archive.ubuntu.com/ubuntu xenial/main amd64 libisl15 amd64 0.16.1-1 [524 kB]
2019-07-10T14:13:20.7431657Z Get:70 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 cpp-5 amd64 5.4.0-6ubuntu1~16.04.11 [7660 kB]
2019-07-10T14:13:21.0228754Z Get:71 http://archive.ubuntu.com/ubuntu xenial/main amd64 cpp amd64 4:5.3.1-1ubuntu1 [27.7 kB]
2019-07-10T14:13:21.0237424Z Get:72 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 curl amd64 7.47.0-1ubuntu2.13 [139 kB]
---
2019-07-10T14:13:45.7574091Z Unpacking ca-certificates (20170717~16.04.2) ...
2019-07-10T14:13:46.0676436Z Selecting previously unselected package libcurl3-gnutls:amd64.
2019-07-10T14:13:46.0691557Z Preparing to unpack .../libcurl3-gnutls_7.47.0-1ubuntu2.13_amd64.deb ...
2019-07-10T14:13:46.0856615Z Unpacking libcurl3-gnutls:amd64 (7.47.0-1ubuntu2.13) ...
2019-07-10T14:13:46.2276204Z Selecting previously unselected package libedit2:amd64.
2019-07-10T14:13:46.2288768Z Preparing to unpack .../libedit2_3.1-20150325-1ubuntu2_amd64.deb ...
2019-07-10T14:13:46.2433924Z Unpacking libedit2:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-10T14:13:46.3678233Z Selecting previously unselected package libpipeline1:amd64.
2019-07-10T14:13:46.3689250Z Preparing to unpack .../libpipeline1_1.4.1-2_amd64.deb ...
2019-07-10T14:13:46.3833831Z Unpacking libpipeline1:amd64 (1.4.1-2) ...
2019-07-10T14:13:46.5014306Z Selecting previously unselected package binfmt-support.
2019-07-10T14:13:46.5026228Z Preparing to unpack .../binfmt-support_2.1.6-1_amd64.deb ...
2019-07-10T14:13:46.5172758Z Unpacking binfmt-support (2.1.6-1) ...
2019-07-10T14:13:46.6448120Z Preparing to unpack .../binutils_2.26.1-1ubuntu1~16.04.8_amd64.deb ...
2019-07-10T14:13:46.6595649Z Unpacking binutils (2.26.1-1ubuntu1~16.04.8) ...
2019-07-10T14:13:47.1350563Z Selecting previously unselected package libisl15:amd64.
2019-07-10T14:13:47.1363129Z Preparing to unpack .../libisl15_0.16.1-1_amd64.deb ...
---
2019-07-10T14:13:58.2985474Z Selecting previously unselected package libtinfo-dev:amd64.
2019-07-10T14:13:58.3004620Z Preparing to unpack .../libtinfo-dev_6.0+20160213-1ubuntu1_amd64.deb ...
2019-07-10T14:13:58.3145489Z Unpacking libtinfo-dev:amd64 (6.0+20160213-1ubuntu1) ...
2019-07-10T14:13:58.4200050Z Selecting previously unselected package libedit-dev:amd64.
2019-07-10T14:13:58.4213798Z Preparing to unpack .../libedit-dev_3.1-20150325-1ubuntu2_amd64.deb ...
2019-07-10T14:13:58.4365510Z Unpacking libedit-dev:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-10T14:13:58.5528532Z Selecting previously unselected package libllvm6.0:amd64.
2019-07-10T14:13:58.5544417Z Preparing to unpack .../libllvm6.0_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-10T14:13:58.5688004Z Unpacking libllvm6.0:amd64 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-10T14:14:01.4579061Z Selecting previously unselected package zlib1g-dev:amd64.
2019-07-10T14:14:01.4599240Z Preparing to unpack .../zlib1g-dev_1%3a1.2.8.dfsg-2ubuntu4.1_amd64.deb ...
2019-07-10T14:14:01.4749311Z Unpacking zlib1g-dev:amd64 (1:1.2.8.dfsg-2ubuntu4.1) ...
2019-07-10T14:14:01.5819896Z Preparing to unpack .../libssl-dev_1.0.2g-1ubuntu4.15_amd64.deb ...
2019-07-10T14:14:01.5966106Z Unpacking libssl-dev:amd64 (1.0.2g-1ubuntu4.15) ...
2019-07-10T14:14:01.9260192Z Selecting previously unselected package llvm-6.0-runtime.
2019-07-10T14:14:01.9260192Z Selecting previously unselected package llvm-6.0-runtime.
2019-07-10T14:14:01.9278232Z Preparing to unpack .../llvm-6.0-runtime_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-10T14:14:01.9419666Z Unpacking llvm-6.0-runtime (1:6.0-1ubuntu2~16.04.1) ...
2019-07-10T14:14:02.0661096Z Selecting previously unselected package llvm-6.0.
2019-07-10T14:14:02.0677129Z Preparing to unpack .../llvm-6.0_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-10T14:14:02.0829830Z Unpacking llvm-6.0 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-10T14:14:02.7187338Z Selecting previously unselected package libffi-dev:amd64.
2019-07-10T14:14:02.7206969Z Preparing to unpack .../libffi-dev_3.2.1-4_amd64.deb ...
2019-07-10T14:14:02.7374975Z Unpacking libffi-dev:amd64 (3.2.1-4) ...
2019-07-10T14:14:02.8627902Z Selecting previously unselected package llvm-6.0-dev.
2019-07-10T14:14:02.8643673Z Preparing to unpack .../llvm-6.0-dev_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-10T14:14:02.8805967Z Unpacking llvm-6.0-dev (1:6.0-1ubuntu2~16.04.1) ...
2019-07-10T14:14:07.2166511Z Selecting previously unselected package llvm-6.0-tools.
2019-07-10T14:14:07.2190056Z Preparing to unpack .../llvm-6.0-tools_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-10T14:14:07.2329853Z Unpacking llvm-6.0-tools (1:6.0-1ubuntu2~16.04.1) ...
2019-07-10T14:14:07.4239397Z Selecting previously unselected package pkg-config.
2019-07-10T14:14:07.4259981Z Preparing to unpack .../pkg-config_0.29.1-0ubuntu1_amd64.deb ...
2019-07-10T14:14:07.4397434Z Unpacking pkg-config (0.29.1-0ubuntu1) ...
2019-07-10T14:14:07.5597139Z Processing triggers for systemd (229-4ubuntu21.21) ...
2019-07-10T14:14:08.1470652Z Setting up libgdbm3:amd64 (1.8.3-13.1) ...
2019-07-10T14:14:08.2252993Z Setting up libffi6:amd64 (3.2.1-4) ...
2019-07-10T14:14:08.2675864Z Setting up libglib2.0-0:amd64 (2.48.2-0ubuntu4.3) ...
---
2019-07-10T14:14:12.1983821Z debconf: unable to initialize frontend: Dialog
2019-07-10T14:14:12.1984117Z debconf: (TERM is not set, so the dialog frontend is not usable.)
2019-07-10T14:14:12.1984911Z debconf: falling back to frontend: Readline
2019-07-10T14:14:12.7068524Z Setting up libcurl3-gnutls:amd64 (7.47.0-1ubuntu2.13) ...
2019-07-10T14:14:12.7670144Z Setting up libedit2:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-10T14:14:12.8132602Z Setting up libpipeline1:amd64 (1.4.1-2) ...
2019-07-10T14:14:12.8638627Z Setting up binfmt-support (2.1.6-1) ...
2019-07-10T14:14:12.9436905Z mount: permission denied
2019-07-10T14:14:12.9437667Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-10T14:14:12.9450407Z mount: permission denied
2019-07-10T14:14:12.9451543Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-10T14:14:13.0971443Z invoke-rc.d: could not determine current runlevel
2019-07-10T14:14:13.0999728Z invoke-rc.d: policy-rc.d denied execution of start.
2019-07-10T14:14:13.1694647Z Setting up libisl15:amd64 (0.16.1-1) ...
2019-07-10T14:14:13.2218994Z Setting up cpp-5 (5.4.0-6ubuntu1~16.04.11) ...
2019-07-10T14:14:13.2683871Z Setting up cpp (4:5.3.1-1ubuntu1) ...
2019-07-10T14:14:13.3327173Z Setting up curl (7.47.0-1ubuntu2.13) ...
---
2019-07-10T14:14:15.5127938Z Setting up libedit-dev:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-10T14:14:15.5556813Z Setting up libllvm6.0:amd64 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-10T14:14:15.5983595Z Setting up zlib1g-dev:amd64 (1:1.2.8.dfsg-2ubuntu4.1) ...
2019-07-10T14:14:15.6413662Z Setting up libssl-dev:amd64 (1.0.2g-1ubuntu4.15) ...
2019-07-10T14:14:15.6843035Z Setting up llvm-6.0-runtime (1:6.0-1ubuntu2~16.04.1) ...
2019-07-10T14:14:15.7163696Z mount: permission denied
2019-07-10T14:14:15.7167861Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-10T14:14:15.7311337Z Setting up llvm-6.0 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-10T14:14:15.7797847Z Setting up libffi-dev:amd64 (3.2.1-4) ...
2019-07-10T14:14:15.8256878Z Setting up llvm-6.0-dev (1:6.0-1ubuntu2~16.04.1) ...
2019-07-10T14:14:15.8678302Z Setting up llvm-6.0-tools (1:6.0-1ubuntu2~16.04.1) ...
2019-07-10T14:14:15.9125967Z Setting up pkg-config (0.29.1-0ubuntu1) ...
2019-07-10T14:14:16.0459886Z Processing triggers for ca-certificates (20170717~16.04.2) ...
2019-07-10T14:14:16.0627423Z Updating certificates in /etc/ssl/certs...
2019-07-10T14:14:17.4609277Z 148 added, 0 removed; done.
2019-07-10T14:14:17.4614654Z Running hooks in /etc/ca-certificates/update.d...
---
2019-07-10T14:14:50.1307755Z  ---> 3a7c7dc66699
2019-07-10T14:14:50.1341385Z Successfully built 3a7c7dc66699
2019-07-10T14:14:50.3272548Z Successfully tagged rust-ci:latest
2019-07-10T14:14:50.4173398Z Built container sha256:3a7c7dc6669942739877f1fa44b6912c68134b8644cd4fe45f48a2399b24b204
2019-07-10T14:14:50.4187500Z Uploading finished image to https://.s3.amazonaws.com/docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624
2019-07-10T14:15:47.2792500Z upload failed: - to s3:///docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624 Parameter validation failed:
2019-07-10T14:15:47.2793451Z Invalid bucket name "": Bucket name must match the regex "^[a-zA-Z0-9.\-_]{1,255}$"
2019-07-10T14:15:48.3513199Z [CI_JOB_NAME=x86_64-gnu-llvm-6.0]
2019-07-10T14:15:48.3557654Z Starting sccache server...
2019-07-10T14:15:48.3995063Z configure: processing command line
2019-07-10T14:15:48.3995865Z configure: 
---
2019-07-10T15:06:19.4880475Z .................................................................................................... 200/5771
2019-07-10T15:06:23.2247726Z .................................................................................................... 300/5771
2019-07-10T15:06:26.6144036Z .................................................................................................... 400/5771
2019-07-10T15:06:30.0039112Z .................................................................................................... 500/5771
2019-07-10T15:06:33.5714094Z ..................................................................i................................. 600/5771
2019-07-10T15:06:41.4519017Z .................................................................................................... 800/5771
2019-07-10T15:06:46.3591704Z .................................................................................................... 900/5771
2019-07-10T15:06:46.3591704Z .................................................................................................... 900/5771
2019-07-10T15:06:51.7384915Z .....................................................................................i...........i.. 1000/5771
2019-07-10T15:06:55.5300088Z .................................................................................................... 1100/5771
2019-07-10T15:06:59.2402602Z ..............iiiii................................................................................. 1200/5771
2019-07-10T15:07:04.3606622Z .................................................................................................... 1400/5771
2019-07-10T15:07:07.0897976Z .................................................................................................... 1500/5771
2019-07-10T15:07:10.2180646Z .................................................................................................... 1600/5771
2019-07-10T15:07:12.5851886Z .................................................................................................... 1700/5771
2019-07-10T15:07:12.5851886Z .................................................................................................... 1700/5771
2019-07-10T15:07:15.7773219Z ....................................................i............................................... 1800/5771
2019-07-10T15:07:23.5382913Z .................................................................................................... 2000/5771
2019-07-10T15:07:27.2119741Z .................................................................................................... 2100/5771
2019-07-10T15:07:30.9510409Z .................................................................................................... 2200/5771
2019-07-10T15:07:30.9510409Z .................................................................................................... 2200/5771
2019-07-10T15:07:34.3540684Z .............i...................................................................................... 2300/5771
2019-07-10T15:07:43.3078879Z .................................................................................................... 2500/5771
2019-07-10T15:07:48.0686888Z .................................................................................................... 2600/5771
2019-07-10T15:07:51.2491546Z .................................................................................................... 2700/5771
2019-07-10T15:07:55.4281506Z .................................................................................................... 2800/5771
2019-07-10T15:07:55.4281506Z .................................................................................................... 2800/5771
2019-07-10T15:07:59.7280333Z .................................................................................................... 2900/5771
2019-07-10T15:08:04.2784956Z .................................................................................................... 3000/5771
2019-07-10T15:08:08.6297660Z .................................................................................................... 3100/5771
2019-07-10T15:08:12.7846316Z .................................................................................................... 3200/5771
2019-07-10T15:08:15.4932979Z .................................................................................................... 3300/5771
2019-07-10T15:08:20.1839325Z .................................................................................................... 3400/5771
2019-07-10T15:08:23.9671100Z ....................................................................i............................... 3500/5771
2019-07-10T15:08:27.4293464Z .................................................................................................... 3600/5771
2019-07-10T15:08:31.1592399Z ..........................................ii...i..ii................................................ 3700/5771
2019-07-10T15:08:39.2793182Z .................................................................................................... 3900/5771
2019-07-10T15:08:39.2793182Z .................................................................................................... 3900/5771
2019-07-10T15:08:42.8356382Z ........................................................ii.......................................... 4000/5771
2019-07-10T15:08:45.0782159Z .............................................................................i...................... 4100/5771
2019-07-10T15:08:47.1381133Z .................................................................................................... 4200/5771
2019-07-10T15:08:49.0213686Z .........................................i.......................................................... 4300/5771
2019-07-10T15:09:05.8032734Z .................................................................................................... 4500/5771
2019-07-10T15:09:12.2711478Z .................................................................................................... 4600/5771
2019-07-10T15:09:15.2357085Z .................................................................................................... 4700/5771
2019-07-10T15:09:19.2986101Z .................................................................................................... 4800/5771
---
2019-07-10T15:09:49.0768807Z .................................................................................................... 5400/5771
2019-07-10T15:09:53.1614818Z .................................................................................................... 5500/5771
2019-07-10T15:09:56.0992320Z .................................................................................................... 5600/5771
2019-07-10T15:09:58.7682477Z .................................................................................................... 5700/5771
2019-07-10T15:10:00.9658634Z ...........i...........................................................
2019-07-10T15:10:00.9659514Z 
2019-07-10T15:10:00.9741996Z  finished in 235.171
2019-07-10T15:10:00.9938431Z Check compiletest suite=run-pass mode=run-pass (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-07-10T15:10:01.2320877Z 
2019-07-10T15:10:01.2320877Z 
2019-07-10T15:10:01.2321144Z running 2921 tests
2019-07-10T15:10:14.6421099Z .................................................................................................... 100/2921
2019-07-10T15:10:29.0050731Z ...............................................................................i.................... 200/2921
2019-07-10T15:10:51.5620086Z .................................................................................................... 400/2921
2019-07-10T15:11:03.2940824Z .................................................................................................... 500/2921
2019-07-10T15:11:15.6815162Z .................................................................................................... 600/2921
2019-07-10T15:11:34.9660347Z .................................................................................................... 700/2921
2019-07-10T15:11:34.9660347Z .................................................................................................... 700/2921
2019-07-10T15:11:47.6463167Z .................................................................................................... 800/2921
2019-07-10T15:11:58.0724840Z .................................................................................................... 900/2921
2019-07-10T15:12:14.2298744Z .................................................................................................... 1000/2921
2019-07-10T15:12:27.1031677Z .................................................................................................... 1100/2921
2019-07-10T15:12:37.5879917Z .................................................................................................... 1200/2921
2019-07-10T15:12:49.0928794Z .................................................................................................... 1300/2921
2019-07-10T15:13:04.3379776Z .........................ii......................................................................... 1400/2921
2019-07-10T15:13:16.3727047Z .................................................................................................... 1500/2921
2019-07-10T15:13:26.9224834Z .............................................................................i.......i.............. 1600/2921
2019-07-10T15:13:44.1289256Z .................................................................................................... 1700/2921
2019-07-10T15:13:57.6514927Z .........................................................................F.F........................ 1800/2921
2019-07-10T15:14:10.1322406Z ................................F................................................................... 1900/2921
2019-07-10T15:14:25.3324833Z .......i.......................................................................i.................... 2000/2921
2019-07-10T15:15:08.9477301Z .................................................................................................... 2200/2921
2019-07-10T15:15:19.7294001Z .................................................................................................... 2300/2921
2019-07-10T15:15:19.7294001Z .................................................................................................... 2300/2921
2019-07-10T15:15:39.3704769Z ..............ii.................................................................................... 2400/2921
2019-07-10T15:16:29.8725352Z .................................................................................................... 2600/2921
2019-07-10T15:16:41.0808531Z .................................................................................................... 2700/2921
2019-07-10T15:16:52.5198184Z .................................................................................................... 2800/2921
2019-07-10T15:17:06.5189329Z .................................................................................................... 2900/2921
2019-07-10T15:17:06.5189329Z .................................................................................................... 2900/2921
2019-07-10T15:17:10.4724932Z .....................
2019-07-10T15:17:10.4726317Z failures:
2019-07-10T15:17:10.4740098Z 
2019-07-10T15:17:10.4740499Z ---- [run-pass] run-pass/macros/macro-comma-support.rs#core stdout ----
2019-07-10T15:17:10.4743558Z normalized stderr:
2019-07-10T15:17:10.4744199Z warning: use of deprecated item 'try': use the `?` operator instead
2019-07-10T15:17:10.4744983Z   --> $DIR/macro-comma-support.rs:264:9
2019-07-10T15:17:10.4745244Z    |
2019-07-10T15:17:10.4745430Z LL |         try!(Ok(()));
2019-07-10T15:17:10.4745930Z    |
2019-07-10T15:17:10.4746129Z    = note: #[warn(deprecated)] on by default
2019-07-10T15:17:10.4746285Z 
2019-07-10T15:17:10.4746285Z 
2019-07-10T15:17:10.4746668Z warning: use of deprecated item 'try': use the `?` operator instead
2019-07-10T15:17:10.4747090Z   --> $DIR/macro-comma-support.rs:265:9
2019-07-10T15:17:10.4747317Z    |
2019-07-10T15:17:10.4747505Z LL |         try!(Ok(()),);
2019-07-10T15:17:10.4747828Z 
2019-07-10T15:17:10.4747828Z 
2019-07-10T15:17:10.4748679Z warning: use of deprecated item '$crate::try': use the `?` operator instead
2019-07-10T15:17:10.4750259Z   --> $DIR/macro-comma-support.rs:265:9
2019-07-10T15:17:10.4751538Z    |
2019-07-10T15:17:10.4751750Z LL |         try!(Ok(()),);
2019-07-10T15:17:10.4752248Z    |
2019-07-10T15:17:10.4752857Z    = note: this warning originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-07-10T15:17:10.4753094Z 
2019-07-10T15:17:10.4753242Z 
2019-07-10T15:17:10.4753242Z 
2019-07-10T15:17:10.4753371Z 
2019-07-10T15:17:10.4753520Z 
2019-07-10T15:17:10.4753686Z The actual stderr differed from the expected stderr.
2019-07-10T15:17:10.4754217Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/macros/macro-comma-support.core/macro-comma-support.core.stderr
2019-07-10T15:17:10.4754821Z To update references, rerun the tests and pass the `--bless` flag
2019-07-10T15:17:10.4755255Z To only update this specific test, also pass `--test-args macros/macro-comma-support.rs`
2019-07-10T15:17:10.4755417Z 
2019-07-10T15:17:10.4755589Z error in revision `core`: 1 errors occurred comparing output.
2019-07-10T15:17:10.4755729Z status: exit code: 0
2019-07-10T15:17:10.4757072Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/macros/macro-comma-support.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "core" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/macros/macro-comma-support.core/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-C" "debug_assertions=yes" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/macros/macro-comma-support.core/auxiliary"
2019-07-10T15:17:10.4757914Z ------------------------------------------
2019-07-10T15:17:10.4758048Z 
2019-07-10T15:17:10.4758326Z ------------------------------------------
2019-07-10T15:17:10.4758485Z stderr:
2019-07-10T15:17:10.4758485Z stderr:
2019-07-10T15:17:10.4758755Z ------------------------------------------
2019-07-10T15:17:10.4759072Z warning: use of deprecated item 'try': use the `?` operator instead
2019-07-10T15:17:10.4759756Z    |
2019-07-10T15:17:10.4759756Z    |
2019-07-10T15:17:10.4759865Z LL |         try!(Ok(()));
2019-07-10T15:17:10.4760096Z    |
2019-07-10T15:17:10.4760222Z    = note: #[warn(deprecated)] on by default
2019-07-10T15:17:10.4760320Z 
2019-07-10T15:17:10.4760320Z 
2019-07-10T15:17:10.4760982Z warning: use of deprecated item 'try': use the `?` operator instead
2019-07-10T15:17:10.4761860Z    |
2019-07-10T15:17:10.4761860Z    |
2019-07-10T15:17:10.4762004Z LL |         try!(Ok(()),);
2019-07-10T15:17:10.4762297Z 
2019-07-10T15:17:10.4762297Z 
2019-07-10T15:17:10.4762693Z warning: use of deprecated item '$crate::try': use the `?` operator instead
2019-07-10T15:17:10.4763321Z    |
2019-07-10T15:17:10.4763321Z    |
2019-07-10T15:17:10.4763465Z LL |         try!(Ok(()),);
2019-07-10T15:17:10.4763788Z    |
2019-07-10T15:17:10.4764550Z    = note: this warning originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-07-10T15:17:10.4764706Z 
2019-07-10T15:17:10.4764823Z 
2019-07-10T15:17:10.4764823Z 
2019-07-10T15:17:10.4765096Z ------------------------------------------
2019-07-10T15:17:10.4765222Z 
2019-07-10T15:17:10.4765336Z 
2019-07-10T15:17:10.4765638Z ---- [run-pass] run-pass/macros/macro-comma-support.rs#std stdout ----
2019-07-10T15:17:10.4765780Z normalized stderr:
2019-07-10T15:17:10.4766087Z warning: use of deprecated item 'try': use the `?` operator instead
2019-07-10T15:17:10.4766385Z   --> $DIR/macro-comma-support.rs:264:9
2019-07-10T15:17:10.4766525Z    |
2019-07-10T15:17:10.4766652Z LL |         try!(Ok(()));
2019-07-10T15:17:10.4766859Z    |
2019-07-10T15:17:10.4766983Z    = note: #[warn(deprecated)] on by default
2019-07-10T15:17:10.4767076Z 
2019-07-10T15:17:10.4767076Z 
2019-07-10T15:17:10.4767370Z warning: use of deprecated item 'try': use the `?` operator instead
2019-07-10T15:17:10.4767694Z   --> $DIR/macro-comma-support.rs:265:9
2019-07-10T15:17:10.4767833Z    |
2019-07-10T15:17:10.4767940Z LL |         try!(Ok(()),);
2019-07-10T15:17:10.4768157Z 
2019-07-10T15:17:10.4768157Z 
2019-07-10T15:17:10.4768450Z warning: use of deprecated item '$crate::try': use the `?` operator instead
2019-07-10T15:17:10.4768781Z   --> $DIR/macro-comma-support.rs:265:9
2019-07-10T15:17:10.4768918Z    |
2019-07-10T15:17:10.4769025Z LL |         try!(Ok(()),);
2019-07-10T15:17:10.4769267Z    |
2019-07-10T15:17:10.4769651Z    = note: this warning originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-07-10T15:17:10.4769793Z 
2019-07-10T15:17:10.4769891Z 
2019-07-10T15:17:10.4769891Z 
2019-07-10T15:17:10.4769988Z 
2019-07-10T15:17:10.4770082Z 
2019-07-10T15:17:10.4770223Z The actual stderr differed from the expected stderr.
2019-07-10T15:17:10.4771083Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/macros/macro-comma-support.std/macro-comma-support.std.stderr
2019-07-10T15:17:10.4771615Z To update references, rerun the tests and pass the `--bless` flag
2019-07-10T15:17:10.4772072Z To only update this specific test, also pass `--test-args macros/macro-comma-support.rs`
2019-07-10T15:17:10.4772259Z 
2019-07-10T15:17:10.4772529Z error in revision `std`: 1 errors occurred comparing output.
2019-07-10T15:17:10.4772686Z status: exit code: 0
2019-07-10T15:17:10.4773756Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/macros/macro-comma-support.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "std" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/macros/macro-comma-support.std/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-C" "debug_assertions=yes" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/macros/macro-comma-support.std/auxiliary"
2019-07-10T15:17:10.4774730Z ------------------------------------------
2019-07-10T15:17:10.4774865Z 
2019-07-10T15:17:10.4775186Z ------------------------------------------
2019-07-10T15:17:10.4775436Z stderr:
2019-07-10T15:17:10.4775436Z stderr:
2019-07-10T15:17:10.4775756Z ------------------------------------------
2019-07-10T15:17:10.4776090Z warning: use of deprecated item 'try': use the `?` operator instead
2019-07-10T15:17:10.4776594Z    |
2019-07-10T15:17:10.4776594Z    |
2019-07-10T15:17:10.4776710Z LL |         try!(Ok(()));
2019-07-10T15:17:10.4776941Z    |
2019-07-10T15:17:10.4777227Z    = note: #[warn(deprecated)] on by default
2019-07-10T15:17:10.4777336Z 
2019-07-10T15:17:10.4777336Z 
2019-07-10T15:17:10.4777648Z warning: use of deprecated item 'try': use the `?` operator instead
2019-07-10T15:17:10.4778187Z    |
2019-07-10T15:17:10.4778187Z    |
2019-07-10T15:17:10.4778305Z LL |         try!(Ok(()),);
2019-07-10T15:17:10.4778533Z 
2019-07-10T15:17:10.4778533Z 
2019-07-10T15:17:10.4778957Z warning: use of deprecated item '$crate::try': use the `?` operator instead
2019-07-10T15:17:10.4779524Z    |
2019-07-10T15:17:10.4779524Z    |
2019-07-10T15:17:10.4779650Z LL |         try!(Ok(()),);
2019-07-10T15:17:10.4779885Z    |
2019-07-10T15:17:10.4780272Z    = note: this warning originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-07-10T15:17:10.4780848Z 
2019-07-10T15:17:10.4780985Z 
2019-07-10T15:17:10.4780985Z 
2019-07-10T15:17:10.4781404Z ------------------------------------------
2019-07-10T15:17:10.4781577Z 
2019-07-10T15:17:10.4781708Z 
2019-07-10T15:17:10.4782124Z ---- [run-pass] run-pass/macros/try-macro.rs stdout ----
2019-07-10T15:17:10.4782320Z normalized stderr:
2019-07-10T15:17:10.4782732Z warning: use of deprecated item 'try': use the `?` operator instead
2019-07-10T15:17:10.4783133Z   --> $DIR/try-macro.rs:13:8
2019-07-10T15:17:10.4783335Z    |
2019-07-10T15:17:10.4783482Z LL |     Ok(try!("1".parse()))
2019-07-10T15:17:10.4783974Z    |
2019-07-10T15:17:10.4784275Z    = note: #[warn(deprecated)] on by default
2019-07-10T15:17:10.4784369Z 
2019-07-10T15:17:10.4784369Z 
2019-07-10T15:17:10.4784664Z warning: use of deprecated item 'try': use the `?` operator instead
2019-07-10T15:17:10.4784995Z   --> $DIR/try-macro.rs:17:8
2019-07-10T15:17:10.4785140Z    |
2019-07-10T15:17:10.4785254Z LL |     Ok(try!(try!("2".parse::<i32>()).to_string().parse::<i32>()))
2019-07-10T15:17:10.4785496Z 
2019-07-10T15:17:10.4785496Z 
2019-07-10T15:17:10.4786220Z warning: use of deprecated item 'try': use the `?` operator instead
2019-07-10T15:17:10.4786596Z   --> $DIR/try-macro.rs:17:13
2019-07-10T15:17:10.4786738Z    |
2019-07-10T15:17:10.4786849Z LL |     Ok(try!(try!("2".parse::<i32>()).to_string().parse::<i32>()))
2019-07-10T15:17:10.4787108Z 
2019-07-10T15:17:10.4787108Z 
2019-07-10T15:17:10.4787402Z warning: use of deprecated item 'try': use the `?` operator instead
2019-07-10T15:17:10.4787730Z   --> $DIR/try-macro.rs:21:8
2019-07-10T15:17:10.4787874Z    |
2019-07-10T15:17:10.4787988Z LL |     Ok(try!("1".parse::<i32>()) as f32 + try!("2.0".parse::<f32>()))
2019-07-10T15:17:10.4788193Z 
2019-07-10T15:17:10.4788193Z 
2019-07-10T15:17:10.4788525Z warning: use of deprecated item 'try': use the `?` operator instead
2019-07-10T15:17:10.4788831Z   --> $DIR/try-macro.rs:21:42
2019-07-10T15:17:10.4789077Z    |
2019-07-10T15:17:10.4789226Z LL |     Ok(try!("1".parse::<i32>()) as f32 + try!("2.0".parse::<f32>()))
2019-07-10T15:17:10.4789454Z 
2019-07-10T15:17:10.4789454Z 
2019-07-10T15:17:10.4789795Z warning: use of deprecated item 'try': use the `?` operator instead
2019-07-10T15:17:10.4790107Z   --> $DIR/try-macro.rs:25:8
2019-07-10T15:17:10.4790373Z    |
2019-07-10T15:17:10.4790487Z LL |     Ok(try!("a".parse::<i32>()) as f32 + try!("2.0".parse::<f32>()))
2019-07-10T15:17:10.4791126Z 
2019-07-10T15:17:10.4791126Z 
2019-07-10T15:17:10.4791614Z warning: use of deprecated item 'try': use the `?` operator instead
2019-07-10T15:17:10.4792025Z   --> $DIR/try-macro.rs:25:42
2019-07-10T15:17:10.4792209Z    |
2019-07-10T15:17:10.4792366Z LL |     Ok(try!("a".parse::<i32>()) as f32 + try!("2.0".parse::<f32>()))
2019-07-10T15:17:10.4792694Z 
2019-07-10T15:17:10.4792694Z 
2019-07-10T15:17:10.4793672Z warning: use of deprecated item 'try': use the `?` operator instead
2019-07-10T15:17:10.4794092Z   --> $DIR/try-macro.rs:29:8
2019-07-10T15:17:10.4794277Z    |
2019-07-10T15:17:10.4794433Z LL |     Ok(try!("1".parse::<i32>()) as f32 + try!("b".parse::<f32>()))
2019-07-10T15:17:10.4794755Z 
2019-07-10T15:17:10.4794755Z 
2019-07-10T15:17:10.4795144Z warning: use of deprecated item 'try': use the `?` operator instead
2019-07-10T15:17:10.4795561Z   --> $DIR/try-macro.rs:29:42
2019-07-10T15:17:10.4795751Z    |
2019-07-10T15:17:10.4795906Z LL |     Ok(try!("1".parse::<i32>()) as f32 + try!("b".parse::<f32>()))
2019-07-10T15:17:10.4796531Z 
2019-07-10T15:17:10.4796627Z 
2019-07-10T15:17:10.4796742Z 
2019-07-10T15:17:10.4796837Z 
2019-07-10T15:17:10.4796837Z 
2019-07-10T15:17:10.4796960Z The actual stderr differed from the expected stderr.
2019-07-10T15:17:10.4800774Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/macros/try-macro/try-macro.stderr
2019-07-10T15:17:10.4801487Z To update references, rerun the tests and pass the `--bless` flag
2019-07-10T15:17:10.4801762Z To only update this specific test, also pass `--test-args macros/try-macro.rs`
2019-07-10T15:17:10.4801866Z error: 1 errors occurred comparing output.
2019-07-10T15:17:10.4801928Z status: exit code: 0
2019-07-10T15:17:10.4801928Z status: exit code: 0
2019-07-10T15:17:10.4802654Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/macros/try-macro.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/macros/try-macro/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/macros/try-macro/auxiliary"
2019-07-10T15:17:10.4802996Z ------------------------------------------
2019-07-10T15:17:10.4803049Z 
2019-07-10T15:17:10.4803275Z ------------------------------------------
2019-07-10T15:17:10.4803322Z stderr:
2019-07-10T15:17:10.4803322Z stderr:
2019-07-10T15:17:10.4803558Z ------------------------------------------
2019-07-10T15:17:10.4803809Z warning: use of deprecated item 'try': use the `?` operator instead
2019-07-10T15:17:10.4804296Z    |
2019-07-10T15:17:10.4804296Z    |
2019-07-10T15:17:10.4804338Z LL |     Ok(try!("1".parse()))
2019-07-10T15:17:10.4804421Z    |
2019-07-10T15:17:10.4804483Z    = note: #[warn(deprecated)] on by default
2019-07-10T15:17:10.4804512Z 
2019-07-10T15:17:10.4804512Z 
2019-07-10T15:17:10.4804751Z warning: use of deprecated item 'try': use the `?` operator instead
2019-07-10T15:17:10.4805196Z    |
2019-07-10T15:17:10.4805196Z    |
2019-07-10T15:17:10.4805242Z LL |     Ok(try!(try!("2".parse::<i32>()).to_string().parse::<i32>()))
2019-07-10T15:17:10.4805339Z 
2019-07-10T15:17:10.4805339Z 
2019-07-10T15:17:10.4805608Z warning: use of deprecated item 'try': use the `?` operator instead
2019-07-10T15:17:10.4806305Z    |
2019-07-10T15:17:10.4806305Z    |
2019-07-10T15:17:10.4806347Z LL |     Ok(try!(try!("2".parse::<i32>()).to_string().parse::<i32>()))
2019-07-10T15:17:10.4806621Z 
2019-07-10T15:17:10.4806621Z 
2019-07-10T15:17:10.4806858Z warning: use of deprecated item 'try': use the `?` operator instead
2019-07-10T15:17:10.4807099Z    |
2019-07-10T15:17:10.4807099Z    |
2019-07-10T15:17:10.4807154Z LL |     Ok(try!("1".parse::<i32>()) as f32 + try!("2.0".parse::<f32>()))
2019-07-10T15:17:10.4807225Z 
2019-07-10T15:17:10.4807225Z 
2019-07-10T15:17:10.4807426Z warning: use of deprecated item 'try': use the `?` operator instead
2019-07-10T15:17:10.4807682Z    |
2019-07-10T15:17:10.4807682Z    |
2019-07-10T15:17:10.4807720Z LL |     Ok(try!("1".parse::<i32>()) as f32 + try!("2.0".parse::<f32>()))
2019-07-10T15:17:10.4807814Z 
2019-07-10T15:17:10.4807814Z 
2019-07-10T15:17:10.4811879Z warning: use of deprecated item 'try': use the `?` operator instead
2019-07-10T15:17:10.4812342Z    |
2019-07-10T15:17:10.4812342Z    |
2019-07-10T15:17:10.4812390Z LL |     Ok(try!("a".parse::<i32>()) as f32 + try!("2.0".parse::<f32>()))
2019-07-10T15:17:10.4812488Z 
2019-07-10T15:17:10.4812488Z 
2019-07-10T15:17:10.4812756Z warning: use of deprecated item 'try': use the `?` operator instead
2019-07-10T15:17:10.4813074Z    |
2019-07-10T15:17:10.4813074Z    |
2019-07-10T15:17:10.4813121Z LL |     Ok(try!("a".parse::<i32>()) as f32 + try!("2.0".parse::<f32>()))
2019-07-10T15:17:10.4813205Z 
2019-07-10T15:17:10.4813205Z 
2019-07-10T15:17:10.4813471Z warning: use of deprecated item 'try': use the `?` operator instead
2019-07-10T15:17:10.4813829Z    |
2019-07-10T15:17:10.4813829Z    |
2019-07-10T15:17:10.4813892Z LL |     Ok(try!("1".parse::<i32>()) as f32 + try!("b".parse::<f32>()))
2019-07-10T15:17:10.4814151Z 
2019-07-10T15:17:10.4814151Z 
2019-07-10T15:17:10.4814593Z warning: use of deprecated item 'try': use the `?` operator instead
2019-07-10T15:17:10.4814841Z    |
2019-07-10T15:17:10.4814841Z    |
2019-07-10T15:17:10.4814895Z LL |     Ok(try!("1".parse::<i32>()) as f32 + try!("b".parse::<f32>()))
2019-07-10T15:17:10.4814963Z 
2019-07-10T15:17:10.4814984Z 
2019-07-10T15:17:10.4815184Z ------------------------------------------
2019-07-10T15:17:10.4815211Z 
---
2019-07-10T15:17:10.4816162Z test result: FAILED. 2909 passed; 3 failed; 9 ignored; 0 measured; 0 filtered out
2019-07-10T15:17:10.4816194Z 
2019-07-10T15:17:10.4816219Z 
2019-07-10T15:17:10.4816240Z 
2019-07-10T15:17:10.4817898Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-07-10T15:17:10.4818162Z 
2019-07-10T15:17:10.4818200Z 
2019-07-10T15:17:10.4818446Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:535:22
2019-07-10T15:17:10.4818502Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-07-10T15:17:10.4818502Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-07-10T15:17:10.4818561Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-07-10T15:17:10.4818600Z Build completed unsuccessfully in 0:58:00
2019-07-10T15:17:10.9943464Z ##[error]Bash exited with code '1'.
2019-07-10T15:17:10.9972780Z ##[section]Starting: Checkout
2019-07-10T15:17:10.9974811Z ==============================================================================
2019-07-10T15:17:10.9974871Z Task         : Get sources
2019-07-10T15:17:10.9974906Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
