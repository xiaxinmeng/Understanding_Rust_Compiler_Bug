plain
2019-07-17T13:01:19.8511569Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-17T13:01:19.8682565Z ##[command]git config gc.auto 0
2019-07-17T13:01:19.8758157Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-17T13:01:19.8827655Z ##[command]git config --get-all http.proxy
2019-07-17T13:01:19.8972314Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62749/merge:refs/remotes/pull/62749/merge
---
2019-07-17T13:01:55.7124007Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-17T13:01:55.7124039Z 
2019-07-17T13:01:55.7124255Z   git checkout -b <new-branch-name>
2019-07-17T13:01:55.7124285Z 
2019-07-17T13:01:55.7124351Z HEAD is now at 83fed9504 Merge 8a9d9a414824e333be7765f4ba33452a5973129d into d56128d2919132aceaf74cc3c68a4554f5445fce
2019-07-17T13:01:55.7269175Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-17T13:01:55.7272851Z ==============================================================================
2019-07-17T13:01:55.7272912Z Task         : Bash
2019-07-17T13:01:55.7272957Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-17T13:03:45.2103014Z Attempting with retry: curl -y 30 -Y 10 --connect-timeout 30 -f -L -C - -o /tmp/rustci_docker_cache https://.s3.amazonaws.com/docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624
2019-07-17T13:03:45.2165284Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-17T13:03:45.2165792Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-17T13:03:45.2166204Z 
2019-07-17T13:03:45.2198730Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-17T13:03:46.2270048Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-17T13:03:46.2270252Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-17T13:03:46.2270391Z 
2019-07-17T13:03:46.2270391Z 
2019-07-17T13:03:46.2311830Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-17T13:03:48.2382603Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-17T13:03:48.2382843Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-17T13:03:48.2382879Z 
2019-07-17T13:03:48.2382879Z 
2019-07-17T13:03:48.2425841Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-17T13:03:51.2497143Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-17T13:03:51.2497905Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-17T13:03:51.2498013Z 
2019-07-17T13:03:51.2498013Z 
2019-07-17T13:03:51.2540454Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-17T13:03:55.2618544Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-17T13:03:55.2618736Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-17T13:03:55.2619308Z 
2019-07-17T13:03:55.2619308Z 
2019-07-17T13:03:55.2662031Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-17T13:03:55.2666466Z The command has failed after 5 attempts.
2019-07-17T13:03:55.4027543Z open /tmp/rustci_docker_cache: no such file or directory
2019-07-17T13:03:55.4063536Z Attempting with retry: docker build --rm -t rust-ci -f /home/vsts/work/1/s/src/ci/docker/x86_64-gnu-llvm-6.0/Dockerfile /home/vsts/work/1/s/src/ci/docker
2019-07-17T13:03:55.5506096Z Sending build context to Docker daemon  521.2kB
2019-07-17T13:03:55.5506318Z 
2019-07-17T13:03:55.5722707Z Step 1/8 : FROM ubuntu:16.04
---
2019-07-17T13:04:12.2012885Z Reading package lists...
2019-07-17T13:04:13.1813208Z Reading package lists...
2019-07-17T13:04:13.3540799Z Building dependency tree...
2019-07-17T13:04:13.3541852Z Reading state information...
2019-07-17T13:04:13.4774332Z The following additional packages will be installed:
2019-07-17T13:04:13.4775734Z   binfmt-support binutils bzip2 cmake-data cpp cpp-5 dpkg-dev g++-5 gcc gcc-5
2019-07-17T13:04:13.4776402Z   git-man libarchive13 libasan2 libasn1-8-heimdal libatomic1
2019-07-17T13:04:13.4777494Z   libbabeltrace-ctf1 libbabeltrace1 libbsd-dev libbsd0 libbz2-1.0 libc-dev-bin
2019-07-17T13:04:13.4778233Z   libedit2 liberror-perl libexpat1 libffi-dev libffi6 libgcc-5-dev libgdbm3
2019-07-17T13:04:13.4779093Z   libglib2.0-0 libgmp10 libgnutls30 libgomp1 libgssapi-krb5-2
2019-07-17T13:04:13.4779346Z   libgssapi3-heimdal libhcrypto4-heimdal libheimbase1-heimdal
2019-07-17T13:04:13.4779633Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-17T13:04:13.4779633Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-17T13:04:13.4779974Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-17T13:04:13.4780232Z   libkrb5-3 libkrb5support0 libldap-2.4-2 libllvm6.0 liblsan0 liblzo2-2
2019-07-17T13:04:13.4780483Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-17T13:04:13.4780785Z   libperl5.22 libpipeline1 libpython-stdlib libpython2.7-minimal
2019-07-17T13:04:13.4781043Z   libpython2.7-stdlib libpython3.5 libpython3.5-minimal libpython3.5-stdlib
2019-07-17T13:04:13.4781300Z   libquadmath0 libroken18-heimdal librtmp1 libsasl2-2 libsasl2-modules-db
2019-07-17T13:04:13.4781607Z   libsqlite3-0 libssl1.0.0 libstdc++-5-dev libtasn1-6 libtinfo-dev libtsan0
2019-07-17T13:04:13.4782020Z   libubsan0 libwind0-heimdal libxml2 linux-libc-dev llvm-6.0 llvm-6.0-dev
2019-07-17T13:04:13.4782257Z   llvm-6.0-runtime mime-support openssl patch perl perl-modules-5.22 python
2019-07-17T13:04:13.4782514Z   python-minimal python2.7-minimal
2019-07-17T13:04:13.4804224Z Suggested packages:
2019-07-17T13:04:13.4804876Z   binutils-doc bzip2-doc codeblocks eclipse ninja-build cpp-doc gcc-5-locales
2019-07-17T13:04:13.4807139Z   debian-keyring g++-multilib g++-5-multilib gcc-5-doc libstdc++6-5-dbg
2019-07-17T13:04:13.4807776Z   gcc-multilib manpages-dev autoconf automake libtool flex bison gcc-doc
2019-07-17T13:04:13.4811339Z   libasan2-dbg liblsan0-dbg libtsan0-dbg libubsan0-dbg libcilkrts5-dbg
2019-07-17T13:04:13.4811974Z   libmpx0-dbg libquadmath0-dbg gdb-doc gettext-base git-daemon-run
2019-07-17T13:04:13.4811974Z   libmpx0-dbg libquadmath0-dbg gdb-doc gettext-base git-daemon-run
2019-07-17T13:04:13.4812225Z   | git-daemon-sysvinit git-doc git-el git-email git-gui gitk gitweb git-arch
2019-07-17T13:04:13.4812540Z   git-cvs git-mediawiki git-svn lrzip glibc-doc gnutls-bin krb5-doc krb5-user
2019-07-17T13:04:13.4812770Z   libstdc++-5-doc llvm-6.0-doc make-doc ed diffutils-doc perl-doc
2019-07-17T13:04:13.4813031Z   libterm-readline-gnu-perl | libterm-readline-perl-perl python-doc python-tk
2019-07-17T13:04:13.4813291Z   python2.7-doc
2019-07-17T13:04:13.4813340Z Recommended packages:
2019-07-17T13:04:13.4813578Z   build-essential fakeroot libalgorithm-merge-perl libc-dbg gdbserver less
2019-07-17T13:04:13.4813870Z   rsync ssh-client manpages manpages-dev libfile-fcntllock-perl
2019-07-17T13:04:13.4814115Z   libglib2.0-data shared-mime-info xdg-user-dirs krb5-locales libsasl2-modules
2019-07-17T13:04:13.4814325Z   libssl-doc xml-core netbase rename
2019-07-17T13:04:13.4814417Z The following NEW packages will be installed:
2019-07-17T13:04:13.4814697Z   binfmt-support binutils bzip2 ca-certificates cmake cmake-data cpp cpp-5
2019-07-17T13:04:13.4814964Z   curl dpkg-dev file g++ g++-5 gcc gcc-5 gdb git git-man libarchive13 libasan2
2019-07-17T13:04:13.4815253Z   libasn1-8-heimdal libatomic1 libbabeltrace-ctf1 libbabeltrace1 libbsd-dev
2019-07-17T13:04:13.4815681Z   libbsd0 libc-dev-bin libc6-dev libcc1-0 libcilkrts5 libcurl3 libcurl3-gnutls
2019-07-17T13:04:13.4815988Z   libdpkg-perl libedit-dev libedit2 liberror-perl libexpat1 libffi-dev libffi6
2019-07-17T13:04:13.4816278Z   libgcc-5-dev libgdbm3 libglib2.0-0 libgmp10 libgnutls30 libgomp1
2019-07-17T13:04:13.4816765Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-17T13:04:13.4817053Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-17T13:04:13.4817293Z   libkrb5-3 libkrb5support0 libldap-2.4-2 libllvm6.0 liblsan0 liblzo2-2
2019-07-17T13:04:13.4817530Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-17T13:04:13.4817530Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-17T13:04:13.4817807Z   libperl5.22 libpipeline1 libpython-stdlib libpython2.7-minimal
2019-07-17T13:04:13.4818226Z   libpython2.7-stdlib libpython3.5 libpython3.5-minimal libpython3.5-stdlib
2019-07-17T13:04:13.4818796Z   libquadmath0 libroken18-heimdal librtmp1 libsasl2-2 libsasl2-modules-db
2019-07-17T13:04:13.4819118Z   libsqlite3-0 libssl-dev libssl1.0.0 libstdc++-5-dev libtasn1-6 libtinfo-dev
2019-07-17T13:04:13.4819371Z   libtsan0 libubsan0 libwind0-heimdal libxml2 linux-libc-dev llvm-6.0
2019-07-17T13:04:13.4819632Z   llvm-6.0-dev llvm-6.0-runtime llvm-6.0-tools make mime-support openssl patch
2019-07-17T13:04:13.4819931Z   perl perl-modules-5.22 pkg-config python python-minimal python2.7
2019-07-17T13:04:13.4820162Z   python2.7-minimal sudo xz-utils zlib1g-dev
2019-07-17T13:04:13.4825025Z The following packages will be upgraded:
2019-07-17T13:04:13.8833803Z 1 upgraded, 115 newly installed, 0 to remove and 5 not upgraded.
2019-07-17T13:04:13.8834139Z Need to get 121 MB of archives.
2019-07-17T13:04:13.8834368Z After this operation, 592 MB of additional disk space will be used.
2019-07-17T13:04:13.8835332Z Get:1 http://archive.ubuntu.com/ubuntu xenial/main amd64 libgdbm3 amd64 1.8.3-13.1 [16.9 kB]
---
2019-07-17T13:04:16.3306090Z Get:61 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 sudo amd64 1.8.16-0ubuntu1.7 [390 kB]
2019-07-17T13:04:16.4026316Z Get:62 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 openssl amd64 1.0.2g-1ubuntu4.15 [492 kB]
2019-07-17T13:04:16.4102251Z Get:63 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 ca-certificates all 20170717~16.04.2 [167 kB]
2019-07-17T13:04:16.4130961Z Get:64 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 libcurl3-gnutls amd64 7.47.0-1ubuntu2.13 [184 kB]
2019-07-17T13:04:16.4160642Z Get:65 http://archive.ubuntu.com/ubuntu xenial/main amd64 libedit2 amd64 3.1-20150325-1ubuntu2 [76.5 kB]
2019-07-17T13:04:16.4174066Z Get:66 http://archive.ubuntu.com/ubuntu xenial/main amd64 libpipeline1 amd64 1.4.1-2 [24.6 kB]
2019-07-17T13:04:16.4180843Z Get:67 http://archive.ubuntu.com/ubuntu xenial/main amd64 binfmt-support amd64 2.1.6-1 [50.7 kB]
2019-07-17T13:04:16.5467372Z Get:69 http://archive.ubuntu.com/ubuntu xenial/main amd64 libisl15 amd64 0.16.1-1 [524 kB]
2019-07-17T13:04:16.5557808Z Get:70 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 cpp-5 amd64 5.4.0-6ubuntu1~16.04.11 [7660 kB]
2019-07-17T13:04:16.9224062Z Get:71 http://archive.ubuntu.com/ubuntu xenial/main amd64 cpp amd64 4:5.3.1-1ubuntu1 [27.7 kB]
2019-07-17T13:04:16.9233607Z Get:72 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 curl amd64 7.47.0-1ubuntu2.13 [139 kB]
---
2019-07-17T13:04:37.1724247Z Unpacking ca-certificates (20170717~16.04.2) ...
2019-07-17T13:04:37.3299341Z Selecting previously unselected package libcurl3-gnutls:amd64.
2019-07-17T13:04:37.3316124Z Preparing to unpack .../libcurl3-gnutls_7.47.0-1ubuntu2.13_amd64.deb ...
2019-07-17T13:04:37.3444887Z Unpacking libcurl3-gnutls:amd64 (7.47.0-1ubuntu2.13) ...
2019-07-17T13:04:37.4906272Z Selecting previously unselected package libedit2:amd64.
2019-07-17T13:04:37.4921667Z Preparing to unpack .../libedit2_3.1-20150325-1ubuntu2_amd64.deb ...
2019-07-17T13:04:37.5053163Z Unpacking libedit2:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-17T13:04:37.6187485Z Selecting previously unselected package libpipeline1:amd64.
2019-07-17T13:04:37.6202127Z Preparing to unpack .../libpipeline1_1.4.1-2_amd64.deb ...
2019-07-17T13:04:37.6326025Z Unpacking libpipeline1:amd64 (1.4.1-2) ...
2019-07-17T13:04:37.7477267Z Selecting previously unselected package binfmt-support.
2019-07-17T13:04:37.7490627Z Preparing to unpack .../binfmt-support_2.1.6-1_amd64.deb ...
2019-07-17T13:04:37.7612548Z Unpacking binfmt-support (2.1.6-1) ...
2019-07-17T13:04:37.8759950Z Preparing to unpack .../binutils_2.26.1-1ubuntu1~16.04.8_amd64.deb ...
2019-07-17T13:04:37.8885577Z Unpacking binutils (2.26.1-1ubuntu1~16.04.8) ...
2019-07-17T13:04:38.4504023Z Selecting previously unselected package libisl15:amd64.
2019-07-17T13:04:38.4521533Z Preparing to unpack .../libisl15_0.16.1-1_amd64.deb ...
---
2019-07-17T13:04:52.7702477Z Unpacking zlib1g-dev:amd64 (1:1.2.8.dfsg-2ubuntu4.1) ...
2019-07-17T13:04:52.8668694Z Selecting previously unselected package libssl-dev:amd64.
2019-07-17T13:04:52.8685528Z Preparing to unpack .../libssl-dev_1.0.2g-1ubuntu4.15_amd64.deb ...
2019-07-17T13:04:52.8811502Z Unpacking libssl-dev:amd64 (1.0.2g-1ubuntu4.15) ...
2019-07-17T13:04:53.1939751Z Selecting previously unselected package llvm-6.0-runtime.
2019-07-17T13:04:53.1960236Z Preparing to unpack .../llvm-6.0-runtime_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-17T13:04:53.2080683Z Unpacking llvm-6.0-runtime (1:6.0-1ubuntu2~16.04.1) ...
2019-07-17T13:04:53.3345508Z Selecting previously unselected package llvm-6.0.
2019-07-17T13:04:53.3368243Z Preparing to unpack .../llvm-6.0_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-17T13:04:53.3488700Z Unpacking llvm-6.0 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-17T13:04:54.0230291Z Selecting previously unselected package libffi-dev:amd64.
2019-07-17T13:04:54.0251664Z Preparing to unpack .../libffi-dev_3.2.1-4_amd64.deb ...
2019-07-17T13:04:54.0375631Z Unpacking libffi-dev:amd64 (3.2.1-4) ...
2019-07-17T13:04:54.1506480Z Selecting previously unselected package llvm-6.0-dev.
2019-07-17T13:04:54.1522149Z Preparing to unpack .../llvm-6.0-dev_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-17T13:04:54.1709713Z Unpacking llvm-6.0-dev (1:6.0-1ubuntu2~16.04.1) ...
2019-07-17T13:04:58.7010788Z Selecting previously unselected package llvm-6.0-tools.
2019-07-17T13:04:58.7036436Z Preparing to unpack .../llvm-6.0-tools_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-17T13:04:58.7162895Z Unpacking llvm-6.0-tools (1:6.0-1ubuntu2~16.04.1) ...
2019-07-17T13:04:58.8772613Z Selecting previously unselected package pkg-config.
2019-07-17T13:04:58.8794860Z Preparing to unpack .../pkg-config_0.29.1-0ubuntu1_amd64.deb ...
2019-07-17T13:04:58.9477603Z Processing triggers for libc-bin (2.23-0ubuntu11) ...
2019-07-17T13:04:59.0022253Z Processing triggers for systemd (229-4ubuntu21.21) ...
2019-07-17T13:04:59.3507153Z Setting up libgdbm3:amd64 (1.8.3-13.1) ...
2019-07-17T13:04:59.4365346Z Setting up libffi6:amd64 (3.2.1-4) ...
---
2019-07-17T13:05:03.1232108Z debconf: unable to initialize frontend: Dialog
2019-07-17T13:05:03.1232330Z debconf: (TERM is not set, so the dialog frontend is not usable.)
2019-07-17T13:05:03.1238085Z debconf: falling back to frontend: Readline
2019-07-17T13:05:04.0896111Z Setting up libcurl3-gnutls:amd64 (7.47.0-1ubuntu2.13) ...
2019-07-17T13:05:04.0897010Z Setting up libedit2:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-17T13:05:04.1141039Z Setting up libpipeline1:amd64 (1.4.1-2) ...
2019-07-17T13:05:04.1555171Z Setting up binfmt-support (2.1.6-1) ...
2019-07-17T13:05:04.2226113Z mount: permission denied
2019-07-17T13:05:04.2233879Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-17T13:05:04.2242907Z mount: permission denied
2019-07-17T13:05:04.2264073Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-17T13:05:04.3843726Z invoke-rc.d: could not determine current runlevel
2019-07-17T13:05:04.3873006Z invoke-rc.d: policy-rc.d denied execution of start.
2019-07-17T13:05:04.4458175Z Setting up libisl15:amd64 (0.16.1-1) ...
2019-07-17T13:05:04.4904655Z Setting up cpp-5 (5.4.0-6ubuntu1~16.04.11) ...
2019-07-17T13:05:04.5283236Z Setting up cpp (4:5.3.1-1ubuntu1) ...
2019-07-17T13:05:04.5846355Z Setting up curl (7.47.0-1ubuntu2.13) ...
---
2019-07-17T13:05:06.9906899Z Setting up libedit-dev:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-17T13:05:07.0325804Z Setting up libllvm6.0:amd64 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-17T13:05:07.0759596Z Setting up zlib1g-dev:amd64 (1:1.2.8.dfsg-2ubuntu4.1) ...
2019-07-17T13:05:07.1175785Z Setting up libssl-dev:amd64 (1.0.2g-1ubuntu4.15) ...
2019-07-17T13:05:07.1703957Z Setting up llvm-6.0-runtime (1:6.0-1ubuntu2~16.04.1) ...
2019-07-17T13:05:07.2014392Z mount: permission denied
2019-07-17T13:05:07.2021651Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-17T13:05:07.2224849Z Setting up llvm-6.0 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-17T13:05:07.2598875Z Setting up libffi-dev:amd64 (3.2.1-4) ...
2019-07-17T13:05:07.3055359Z Setting up llvm-6.0-dev (1:6.0-1ubuntu2~16.04.1) ...
2019-07-17T13:05:07.3462515Z Setting up llvm-6.0-tools (1:6.0-1ubuntu2~16.04.1) ...
2019-07-17T13:05:07.3844072Z Setting up pkg-config (0.29.1-0ubuntu1) ...
2019-07-17T13:05:07.5321048Z Processing triggers for ca-certificates (20170717~16.04.2) ...
2019-07-17T13:05:07.5483733Z Updating certificates in /etc/ssl/certs...
2019-07-17T13:05:09.1076406Z 148 added, 0 removed; done.
2019-07-17T13:05:09.1077344Z Running hooks in /etc/ca-certificates/update.d...
---
2019-07-17T13:05:42.3928440Z Removing intermediate container c6100db2d9c9
2019-07-17T13:05:42.3929286Z  ---> 6298a573d84d
2019-07-17T13:05:42.3998113Z Successfully built 6298a573d84d
2019-07-17T13:05:43.2372278Z Successfully tagged rust-ci:latest
2019-07-17T13:05:43.2866883Z Built container sha256:6298a573d84dd7b1cf361c9a125510635e19c1787fbca292a11dcb2772a324aa
2019-07-17T13:05:43.2886397Z Uploading finished image to https://.s3.amazonaws.com/docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624
2019-07-17T13:06:44.5725170Z upload failed: - to s3:///docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624 Parameter validation failed:
2019-07-17T13:06:44.5726298Z Invalid bucket name "": Bucket name must match the regex "^[a-zA-Z0-9.\-_]{1,255}$"
2019-07-17T13:06:45.6861214Z [CI_JOB_NAME=x86_64-gnu-llvm-6.0]
2019-07-17T13:06:45.6899220Z Starting sccache server...
2019-07-17T13:06:45.7407514Z configure: processing command line
2019-07-17T13:06:45.7408400Z configure: 
---
2019-07-17T13:10:18.2768390Z    Compiling serde_json v1.0.33
2019-07-17T13:10:22.4983695Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-07-17T13:10:30.9424622Z     Finished release [optimized] target(s) in 1m 29s
2019-07-17T13:10:30.9495159Z tidy check
2019-07-17T13:10:31.0985518Z tidy error: /checkout/src/librustc_typeck/check/mod.rs:1451: trailing whitespace
2019-07-17T13:10:32.6994599Z some tidy checks failed
2019-07-17T13:10:32.6994732Z 
2019-07-17T13:10:32.6994732Z 
2019-07-17T13:10:32.6996305Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-07-17T13:10:32.6996651Z 
2019-07-17T13:10:32.6996675Z 
2019-07-17T13:10:32.6999739Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-07-17T13:10:32.6999837Z Build completed unsuccessfully in 0:01:32
2019-07-17T13:10:32.6999837Z Build completed unsuccessfully in 0:01:32
2019-07-17T13:10:33.9701086Z ##[error]Bash exited with code '1'.
2019-07-17T13:10:33.9734203Z ##[section]Starting: Checkout
2019-07-17T13:10:33.9735997Z ==============================================================================
2019-07-17T13:10:33.9736052Z Task         : Get sources
2019-07-17T13:10:33.9736116Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
