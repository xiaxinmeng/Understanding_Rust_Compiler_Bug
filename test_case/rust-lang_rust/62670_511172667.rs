plain
2019-07-14T04:16:52.2769762Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-14T04:16:52.2953302Z ##[command]git config gc.auto 0
2019-07-14T04:16:52.3016880Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-14T04:16:52.3065206Z ##[command]git config --get-all http.proxy
2019-07-14T04:16:52.3199225Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62670/merge:refs/remotes/pull/62670/merge
---
2019-07-14T04:17:25.0819057Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-14T04:17:25.0819083Z 
2019-07-14T04:17:25.0819243Z   git checkout -b <new-branch-name>
2019-07-14T04:17:25.0819267Z 
2019-07-14T04:17:25.0819306Z HEAD is now at fe5f17394 Merge 62f3bb074b63ad262dc6c304975a67f08d1b74f5 into fa6b70658e76ca47e1627308c736edab63a17dc8
2019-07-14T04:17:25.0949325Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-14T04:17:25.0953346Z ==============================================================================
2019-07-14T04:17:25.0953428Z Task         : Bash
2019-07-14T04:17:25.0953624Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-14T04:19:16.9871010Z Attempting with retry: curl -y 30 -Y 10 --connect-timeout 30 -f -L -C - -o /tmp/rustci_docker_cache https://.s3.amazonaws.com/docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624
2019-07-14T04:19:16.9920847Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-14T04:19:16.9934161Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-14T04:19:16.9934438Z 
2019-07-14T04:19:16.9963117Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-14T04:19:18.0046540Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-14T04:19:18.0047815Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-14T04:19:18.0048377Z 
2019-07-14T04:19:18.0048377Z 
2019-07-14T04:19:18.0051246Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-14T04:19:20.0122777Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-14T04:19:20.0123207Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-14T04:19:20.0123274Z 
2019-07-14T04:19:20.0123274Z 
2019-07-14T04:19:20.0167180Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-14T04:19:23.0253276Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-14T04:19:23.0253521Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-14T04:19:23.0253649Z 
2019-07-14T04:19:23.0253649Z 
2019-07-14T04:19:23.0254688Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-14T04:19:27.0343577Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-14T04:19:27.0343971Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-14T04:19:27.0344144Z 
2019-07-14T04:19:27.0344144Z 
2019-07-14T04:19:27.0381772Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-14T04:19:27.0384523Z The command has failed after 5 attempts.
2019-07-14T04:19:27.1468388Z open /tmp/rustci_docker_cache: no such file or directory
2019-07-14T04:19:27.1496831Z Attempting with retry: docker build --rm -t rust-ci -f /home/vsts/work/1/s/src/ci/docker/x86_64-gnu-llvm-6.0/Dockerfile /home/vsts/work/1/s/src/ci/docker
2019-07-14T04:19:27.3091250Z Sending build context to Docker daemon  521.7kB
2019-07-14T04:19:27.3091472Z 
2019-07-14T04:19:27.3335885Z Step 1/8 : FROM ubuntu:16.04
---
2019-07-14T04:19:43.4904448Z Reading package lists...
2019-07-14T04:19:44.3726125Z Reading package lists...
2019-07-14T04:19:44.5100606Z Building dependency tree...
2019-07-14T04:19:44.5100779Z Reading state information...
2019-07-14T04:19:44.6131611Z The following additional packages will be installed:
2019-07-14T04:19:44.6132651Z   binfmt-support binutils bzip2 cmake-data cpp cpp-5 dpkg-dev g++-5 gcc gcc-5
2019-07-14T04:19:44.6132911Z   git-man libarchive13 libasan2 libasn1-8-heimdal libatomic1
2019-07-14T04:19:44.6133132Z   libbabeltrace-ctf1 libbabeltrace1 libbsd-dev libbsd0 libbz2-1.0 libc-dev-bin
2019-07-14T04:19:44.6133615Z   libedit2 liberror-perl libexpat1 libffi-dev libffi6 libgcc-5-dev libgdbm3
2019-07-14T04:19:44.6133823Z   libglib2.0-0 libgmp10 libgnutls30 libgomp1 libgssapi-krb5-2
2019-07-14T04:19:44.6134212Z   libgssapi3-heimdal libhcrypto4-heimdal libheimbase1-heimdal
2019-07-14T04:19:44.6134691Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-14T04:19:44.6134691Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-14T04:19:44.6134937Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-14T04:19:44.6135334Z   libkrb5-3 libkrb5support0 libldap-2.4-2 libllvm6.0 liblsan0 liblzo2-2
2019-07-14T04:19:44.6135615Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-14T04:19:44.6135833Z   libperl5.22 libpipeline1 libpython-stdlib libpython2.7-minimal
2019-07-14T04:19:44.6136057Z   libpython2.7-stdlib libpython3.5 libpython3.5-minimal libpython3.5-stdlib
2019-07-14T04:19:44.6136825Z   libquadmath0 libroken18-heimdal librtmp1 libsasl2-2 libsasl2-modules-db
2019-07-14T04:19:44.6137124Z   libsqlite3-0 libssl1.0.0 libstdc++-5-dev libtasn1-6 libtinfo-dev libtsan0
2019-07-14T04:19:44.6138192Z   libubsan0 libwind0-heimdal libxml2 linux-libc-dev llvm-6.0 llvm-6.0-dev
2019-07-14T04:19:44.6138606Z   llvm-6.0-runtime mime-support openssl patch perl perl-modules-5.22 python
2019-07-14T04:19:44.6138825Z   python-minimal python2.7-minimal
2019-07-14T04:19:44.6138896Z Suggested packages:
2019-07-14T04:19:44.6139183Z   binutils-doc bzip2-doc codeblocks eclipse ninja-build cpp-doc gcc-5-locales
2019-07-14T04:19:44.6139432Z   debian-keyring g++-multilib g++-5-multilib gcc-5-doc libstdc++6-5-dbg
2019-07-14T04:19:44.6140136Z   gcc-multilib manpages-dev autoconf automake libtool flex bison gcc-doc
2019-07-14T04:19:44.6140729Z   libasan2-dbg liblsan0-dbg libtsan0-dbg libubsan0-dbg libcilkrts5-dbg
2019-07-14T04:19:44.6140907Z   libmpx0-dbg libquadmath0-dbg gdb-doc gettext-base git-daemon-run
2019-07-14T04:19:44.6140907Z   libmpx0-dbg libquadmath0-dbg gdb-doc gettext-base git-daemon-run
2019-07-14T04:19:44.6141127Z   | git-daemon-sysvinit git-doc git-el git-email git-gui gitk gitweb git-arch
2019-07-14T04:19:44.6141314Z   git-cvs git-mediawiki git-svn lrzip glibc-doc gnutls-bin krb5-doc krb5-user
2019-07-14T04:19:44.6141492Z   libstdc++-5-doc llvm-6.0-doc make-doc ed diffutils-doc perl-doc
2019-07-14T04:19:44.6141718Z   libterm-readline-gnu-perl | libterm-readline-perl-perl python-doc python-tk
2019-07-14T04:19:44.6141868Z   python2.7-doc
2019-07-14T04:19:44.6141904Z Recommended packages:
2019-07-14T04:19:44.6142292Z   build-essential fakeroot libalgorithm-merge-perl libc-dbg gdbserver less
2019-07-14T04:19:44.6142489Z   rsync ssh-client manpages manpages-dev libfile-fcntllock-perl
2019-07-14T04:19:44.6142989Z   libglib2.0-data shared-mime-info xdg-user-dirs krb5-locales libsasl2-modules
2019-07-14T04:19:44.6143261Z   libssl-doc xml-core netbase rename
2019-07-14T04:19:44.6143306Z The following NEW packages will be installed:
2019-07-14T04:19:44.6143527Z   binfmt-support binutils bzip2 ca-certificates cmake cmake-data cpp cpp-5
2019-07-14T04:19:44.6143745Z   curl dpkg-dev file g++ g++-5 gcc gcc-5 gdb git git-man libarchive13 libasan2
2019-07-14T04:19:44.6144153Z   libasn1-8-heimdal libatomic1 libbabeltrace-ctf1 libbabeltrace1 libbsd-dev
2019-07-14T04:19:44.6144356Z   libbsd0 libc-dev-bin libc6-dev libcc1-0 libcilkrts5 libcurl3 libcurl3-gnutls
2019-07-14T04:19:44.6144891Z   libdpkg-perl libedit-dev libedit2 liberror-perl libexpat1 libffi-dev libffi6
2019-07-14T04:19:44.6145167Z   libgcc-5-dev libgdbm3 libglib2.0-0 libgmp10 libgnutls30 libgomp1
2019-07-14T04:19:44.6145742Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-14T04:19:44.6145985Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-14T04:19:44.6146181Z   libkrb5-3 libkrb5support0 libldap-2.4-2 libllvm6.0 liblsan0 liblzo2-2
2019-07-14T04:19:44.6146777Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-14T04:19:44.6146777Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-14T04:19:44.6147071Z   libperl5.22 libpipeline1 libpython-stdlib libpython2.7-minimal
2019-07-14T04:19:44.6147320Z   libpython2.7-stdlib libpython3.5 libpython3.5-minimal libpython3.5-stdlib
2019-07-14T04:19:44.6147946Z   libquadmath0 libroken18-heimdal librtmp1 libsasl2-2 libsasl2-modules-db
2019-07-14T04:19:44.6148278Z   libsqlite3-0 libssl-dev libssl1.0.0 libstdc++-5-dev libtasn1-6 libtinfo-dev
2019-07-14T04:19:44.6148525Z   libtsan0 libubsan0 libwind0-heimdal libxml2 linux-libc-dev llvm-6.0
2019-07-14T04:19:44.6148775Z   llvm-6.0-dev llvm-6.0-runtime llvm-6.0-tools make mime-support openssl patch
2019-07-14T04:19:44.6149090Z   perl perl-modules-5.22 pkg-config python python-minimal python2.7
2019-07-14T04:19:44.6149311Z   python2.7-minimal sudo xz-utils zlib1g-dev
2019-07-14T04:19:44.6149363Z The following packages will be upgraded:
2019-07-14T04:19:44.8594091Z 1 upgraded, 115 newly installed, 0 to remove and 4 not upgraded.
2019-07-14T04:19:44.8594186Z Need to get 121 MB of archives.
2019-07-14T04:19:44.8595136Z After this operation, 592 MB of additional disk space will be used.
2019-07-14T04:19:44.8596047Z Get:1 http://archive.ubuntu.com/ubuntu xenial/main amd64 libgdbm3 amd64 1.8.3-13.1 [16.9 kB]
---
2019-07-14T04:19:45.9610844Z Get:61 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 sudo amd64 1.8.16-0ubuntu1.7 [390 kB]
2019-07-14T04:19:45.9678285Z Get:62 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 openssl amd64 1.0.2g-1ubuntu4.15 [492 kB]
2019-07-14T04:19:45.9747162Z Get:63 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 ca-certificates all 20170717~16.04.2 [167 kB]
2019-07-14T04:19:45.9772898Z Get:64 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 libcurl3-gnutls amd64 7.47.0-1ubuntu2.13 [184 kB]
2019-07-14T04:19:45.9801691Z Get:65 http://archive.ubuntu.com/ubuntu xenial/main amd64 libedit2 amd64 3.1-20150325-1ubuntu2 [76.5 kB]
2019-07-14T04:19:45.9815320Z Get:66 http://archive.ubuntu.com/ubuntu xenial/main amd64 libpipeline1 amd64 1.4.1-2 [24.6 kB]
2019-07-14T04:19:45.9822312Z Get:67 http://archive.ubuntu.com/ubuntu xenial/main amd64 binfmt-support amd64 2.1.6-1 [50.7 kB]
2019-07-14T04:19:46.0412796Z Get:69 http://archive.ubuntu.com/ubuntu xenial/main amd64 libisl15 amd64 0.16.1-1 [524 kB]
2019-07-14T04:19:46.0483307Z Get:70 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 cpp-5 amd64 5.4.0-6ubuntu1~16.04.11 [7660 kB]
2019-07-14T04:19:46.1716222Z Get:71 http://archive.ubuntu.com/ubuntu xenial/main amd64 cpp amd64 4:5.3.1-1ubuntu1 [27.7 kB]
2019-07-14T04:19:46.1722835Z Get:72 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 curl amd64 7.47.0-1ubuntu2.13 [139 kB]
---
2019-07-14T04:20:04.0629492Z Unpacking ca-certificates (20170717~16.04.2) ...
2019-07-14T04:20:04.2709924Z Selecting previously unselected package libcurl3-gnutls:amd64.
2019-07-14T04:20:04.2723163Z Preparing to unpack .../libcurl3-gnutls_7.47.0-1ubuntu2.13_amd64.deb ...
2019-07-14T04:20:04.2858839Z Unpacking libcurl3-gnutls:amd64 (7.47.0-1ubuntu2.13) ...
2019-07-14T04:20:04.4411807Z Selecting previously unselected package libedit2:amd64.
2019-07-14T04:20:04.4426448Z Preparing to unpack .../libedit2_3.1-20150325-1ubuntu2_amd64.deb ...
2019-07-14T04:20:04.4641155Z Unpacking libedit2:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-14T04:20:04.5963988Z Selecting previously unselected package libpipeline1:amd64.
2019-07-14T04:20:04.5980973Z Preparing to unpack .../libpipeline1_1.4.1-2_amd64.deb ...
2019-07-14T04:20:04.6143490Z Unpacking libpipeline1:amd64 (1.4.1-2) ...
2019-07-14T04:20:04.7589406Z Selecting previously unselected package binfmt-support.
2019-07-14T04:20:04.7607933Z Preparing to unpack .../binfmt-support_2.1.6-1_amd64.deb ...
2019-07-14T04:20:04.7749238Z Unpacking binfmt-support (2.1.6-1) ...
2019-07-14T04:20:04.9112942Z Preparing to unpack .../binutils_2.26.1-1ubuntu1~16.04.8_amd64.deb ...
2019-07-14T04:20:04.9282109Z Unpacking binutils (2.26.1-1ubuntu1~16.04.8) ...
2019-07-14T04:20:05.4225164Z Selecting previously unselected package libisl15:amd64.
2019-07-14T04:20:05.4238632Z Preparing to unpack .../libisl15_0.16.1-1_amd64.deb ...
---
2019-07-14T04:20:17.1998649Z Selecting previously unselected package libtinfo-dev:amd64.
2019-07-14T04:20:17.2012644Z Preparing to unpack .../libtinfo-dev_6.0+20160213-1ubuntu1_amd64.deb ...
2019-07-14T04:20:17.2145522Z Unpacking libtinfo-dev:amd64 (6.0+20160213-1ubuntu1) ...
2019-07-14T04:20:17.3070188Z Selecting previously unselected package libedit-dev:amd64.
2019-07-14T04:20:17.3085097Z Preparing to unpack .../libedit-dev_3.1-20150325-1ubuntu2_amd64.deb ...
2019-07-14T04:20:17.3239939Z Unpacking libedit-dev:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-14T04:20:17.4451766Z Selecting previously unselected package libllvm6.0:amd64.
2019-07-14T04:20:17.4469832Z Preparing to unpack .../libllvm6.0_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-14T04:20:17.4599046Z Unpacking libllvm6.0:amd64 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-14T04:20:20.2347890Z Selecting previously unselected package zlib1g-dev:amd64.
2019-07-14T04:20:20.2364137Z Preparing to unpack .../zlib1g-dev_1%3a1.2.8.dfsg-2ubuntu4.1_amd64.deb ...
2019-07-14T04:20:20.2506944Z Unpacking zlib1g-dev:amd64 (1:1.2.8.dfsg-2ubuntu4.1) ...
2019-07-14T04:20:20.3562531Z Preparing to unpack .../libssl-dev_1.0.2g-1ubuntu4.15_amd64.deb ...
2019-07-14T04:20:20.3692503Z Unpacking libssl-dev:amd64 (1.0.2g-1ubuntu4.15) ...
2019-07-14T04:20:20.6812654Z Selecting previously unselected package llvm-6.0-runtime.
2019-07-14T04:20:20.6812654Z Selecting previously unselected package llvm-6.0-runtime.
2019-07-14T04:20:20.6832598Z Preparing to unpack .../llvm-6.0-runtime_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-14T04:20:20.6969757Z Unpacking llvm-6.0-runtime (1:6.0-1ubuntu2~16.04.1) ...
2019-07-14T04:20:20.8248988Z Selecting previously unselected package llvm-6.0.
2019-07-14T04:20:20.8261756Z Preparing to unpack .../llvm-6.0_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-14T04:20:20.8394981Z Unpacking llvm-6.0 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-14T04:20:21.4233983Z Selecting previously unselected package libffi-dev:amd64.
2019-07-14T04:20:21.4253975Z Preparing to unpack .../libffi-dev_3.2.1-4_amd64.deb ...
2019-07-14T04:20:21.4634226Z Unpacking libffi-dev:amd64 (3.2.1-4) ...
2019-07-14T04:20:21.5870840Z Selecting previously unselected package llvm-6.0-dev.
2019-07-14T04:20:21.5895439Z Preparing to unpack .../llvm-6.0-dev_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-14T04:20:21.6032117Z Unpacking llvm-6.0-dev (1:6.0-1ubuntu2~16.04.1) ...
2019-07-14T04:20:25.9702760Z Selecting previously unselected package llvm-6.0-tools.
2019-07-14T04:20:25.9727181Z Preparing to unpack .../llvm-6.0-tools_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-14T04:20:25.9878484Z Unpacking llvm-6.0-tools (1:6.0-1ubuntu2~16.04.1) ...
2019-07-14T04:20:26.1442357Z Selecting previously unselected package pkg-config.
2019-07-14T04:20:26.1460012Z Preparing to unpack .../pkg-config_0.29.1-0ubuntu1_amd64.deb ...
2019-07-14T04:20:26.1607858Z Unpacking pkg-config (0.29.1-0ubuntu1) ...
2019-07-14T04:20:26.2750014Z Processing triggers for systemd (229-4ubuntu21.21) ...
2019-07-14T04:20:26.6591552Z Setting up libgdbm3:amd64 (1.8.3-13.1) ...
2019-07-14T04:20:26.7295435Z Setting up libffi6:amd64 (3.2.1-4) ...
2019-07-14T04:20:26.7709334Z Setting up libglib2.0-0:amd64 (2.48.2-0ubuntu4.3) ...
---
2019-07-14T04:20:30.4203587Z debconf: unable to initialize frontend: Dialog
2019-07-14T04:20:30.4204194Z debconf: (TERM is not set, so the dialog frontend is not usable.)
2019-07-14T04:20:30.4204579Z debconf: falling back to frontend: Readline
2019-07-14T04:20:30.9081328Z Setting up libcurl3-gnutls:amd64 (7.47.0-1ubuntu2.13) ...
2019-07-14T04:20:30.9479569Z Setting up libedit2:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-14T04:20:30.9905729Z Setting up libpipeline1:amd64 (1.4.1-2) ...
2019-07-14T04:20:31.0346862Z Setting up binfmt-support (2.1.6-1) ...
2019-07-14T04:20:31.1116656Z mount: permission denied
2019-07-14T04:20:31.1122026Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-14T04:20:31.1135667Z mount: permission denied
2019-07-14T04:20:31.1139967Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-14T04:20:31.2536061Z invoke-rc.d: could not determine current runlevel
2019-07-14T04:20:31.2563090Z invoke-rc.d: policy-rc.d denied execution of start.
2019-07-14T04:20:31.3172664Z Setting up libisl15:amd64 (0.16.1-1) ...
2019-07-14T04:20:31.3592405Z Setting up cpp-5 (5.4.0-6ubuntu1~16.04.11) ...
2019-07-14T04:20:31.4007282Z Setting up cpp (4:5.3.1-1ubuntu1) ...
2019-07-14T04:20:31.4521144Z Setting up curl (7.47.0-1ubuntu2.13) ...
---
2019-07-14T04:20:33.1717498Z Setting up libedit-dev:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-14T04:20:33.2199146Z Setting up libllvm6.0:amd64 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-14T04:20:33.2617473Z Setting up zlib1g-dev:amd64 (1:1.2.8.dfsg-2ubuntu4.1) ...
2019-07-14T04:20:33.3083449Z Setting up libssl-dev:amd64 (1.0.2g-1ubuntu4.15) ...
2019-07-14T04:20:33.3561728Z Setting up llvm-6.0-runtime (1:6.0-1ubuntu2~16.04.1) ...
2019-07-14T04:20:33.3898384Z mount: permission denied
2019-07-14T04:20:33.3899857Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-14T04:20:33.4065015Z Setting up llvm-6.0 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-14T04:20:33.4541213Z Setting up libffi-dev:amd64 (3.2.1-4) ...
2019-07-14T04:20:33.4968493Z Setting up llvm-6.0-dev (1:6.0-1ubuntu2~16.04.1) ...
2019-07-14T04:20:33.5492702Z Setting up llvm-6.0-tools (1:6.0-1ubuntu2~16.04.1) ...
2019-07-14T04:20:33.6012968Z Setting up pkg-config (0.29.1-0ubuntu1) ...
2019-07-14T04:20:33.7469453Z Processing triggers for ca-certificates (20170717~16.04.2) ...
2019-07-14T04:20:33.7756138Z Updating certificates in /etc/ssl/certs...
2019-07-14T04:20:35.2445919Z 148 added, 0 removed; done.
2019-07-14T04:20:35.2447543Z Running hooks in /etc/ca-certificates/update.d...
---
2019-07-14T04:21:07.7407786Z  ---> b1f9e6013b29
2019-07-14T04:21:07.7441275Z Successfully built b1f9e6013b29
2019-07-14T04:21:07.8971026Z Successfully tagged rust-ci:latest
2019-07-14T04:21:07.9750850Z Built container sha256:b1f9e6013b296eb4f5678074c79a27f884e26ba218f56ddbd419dc5fd6d61ad3
2019-07-14T04:21:07.9765624Z Uploading finished image to https://.s3.amazonaws.com/docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624
2019-07-14T04:22:06.0862364Z upload failed: - to s3:///docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624 Parameter validation failed:
2019-07-14T04:22:06.0862785Z Invalid bucket name "": Bucket name must match the regex "^[a-zA-Z0-9.\-_]{1,255}$"
2019-07-14T04:22:07.1024672Z [CI_JOB_NAME=x86_64-gnu-llvm-6.0]
2019-07-14T04:22:07.1069836Z Starting sccache server...
2019-07-14T04:22:07.1507633Z configure: processing command line
2019-07-14T04:22:07.1508202Z configure: 
---
2019-07-14T04:25:21.3455701Z    Compiling serde_json v1.0.33
2019-07-14T04:25:25.5421451Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-07-14T04:25:33.5883656Z     Finished release [optimized] target(s) in 1m 25s
2019-07-14T04:25:33.5950278Z tidy check
2019-07-14T04:25:33.7476702Z tidy error: /checkout/src/libsyntax/parse/diagnostics.rs:787: line longer than 100 chars
2019-07-14T04:25:35.1807177Z some tidy checks failed
2019-07-14T04:25:35.1811659Z 
2019-07-14T04:25:35.1811659Z 
2019-07-14T04:25:35.1812888Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-07-14T04:25:35.1813283Z 
2019-07-14T04:25:35.1813487Z 
2019-07-14T04:25:35.1822465Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-07-14T04:25:35.1822609Z Build completed unsuccessfully in 0:01:27
2019-07-14T04:25:35.1822609Z Build completed unsuccessfully in 0:01:27
2019-07-14T04:25:36.5394499Z ##[error]Bash exited with code '1'.
2019-07-14T04:25:36.5423814Z ##[section]Starting: Checkout
2019-07-14T04:25:36.5425309Z ==============================================================================
2019-07-14T04:25:36.5425375Z Task         : Get sources
2019-07-14T04:25:36.5425414Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
