plain
2019-07-14T12:14:16.7933996Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-14T12:14:16.8098204Z ##[command]git config gc.auto 0
2019-07-14T12:14:17.3668283Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-14T12:14:17.3676837Z ##[command]git config --get-all http.proxy
2019-07-14T12:14:17.3683492Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/61708/merge:refs/remotes/pull/61708/merge
---
2019-07-14T12:14:49.3723670Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-14T12:14:49.3723693Z 
2019-07-14T12:14:49.3723854Z   git checkout -b <new-branch-name>
2019-07-14T12:14:49.3723878Z 
2019-07-14T12:14:49.3723932Z HEAD is now at 41af7098d Merge 94d8979407c715069381a3a53ba91b2bbd8de887 into 03a68d6973c4165563b97b86c7159a157ae1b109
2019-07-14T12:14:49.3862143Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-14T12:14:49.3864318Z ==============================================================================
2019-07-14T12:14:49.3864362Z Task         : Bash
2019-07-14T12:14:49.3864397Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-14T12:16:35.5266256Z Attempting with retry: curl -y 30 -Y 10 --connect-timeout 30 -f -L -C - -o /tmp/rustci_docker_cache https://.s3.amazonaws.com/docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624
2019-07-14T12:16:35.5309401Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-14T12:16:35.5309722Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-14T12:16:35.5309779Z 
2019-07-14T12:16:35.5355032Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-14T12:16:36.5416357Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-14T12:16:36.5416614Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-14T12:16:36.5416813Z 
2019-07-14T12:16:36.5416813Z 
2019-07-14T12:16:36.5458876Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-14T12:16:38.5522447Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-14T12:16:38.5522710Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-14T12:16:38.5523096Z 
2019-07-14T12:16:38.5523096Z 
2019-07-14T12:16:38.5565986Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-14T12:16:41.5629239Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-14T12:16:41.5629389Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-14T12:16:41.5629418Z 
2019-07-14T12:16:41.5629418Z 
2019-07-14T12:16:41.5672739Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-14T12:16:45.5731824Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-14T12:16:45.5731922Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-14T12:16:45.5732382Z 
2019-07-14T12:16:45.5732382Z 
2019-07-14T12:16:45.5775616Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-14T12:16:45.5778502Z The command has failed after 5 attempts.
2019-07-14T12:16:46.4097089Z open /tmp/rustci_docker_cache: no such file or directory
2019-07-14T12:16:46.4099066Z Attempting with retry: docker build --rm -t rust-ci -f /home/vsts/work/1/s/src/ci/docker/x86_64-gnu-llvm-6.0/Dockerfile /home/vsts/work/1/s/src/ci/docker
2019-07-14T12:16:46.4100515Z Sending build context to Docker daemon  521.7kB
2019-07-14T12:16:46.4100555Z 
2019-07-14T12:16:46.4100592Z Step 1/8 : FROM ubuntu:16.04
---
2019-07-14T12:17:01.5199827Z Reading package lists...
2019-07-14T12:17:02.3070416Z Reading package lists...
2019-07-14T12:17:02.4311715Z Building dependency tree...
2019-07-14T12:17:02.4312283Z Reading state information...
2019-07-14T12:17:02.5352151Z The following additional packages will be installed:
2019-07-14T12:17:02.5353714Z   binfmt-support binutils bzip2 cmake-data cpp cpp-5 dpkg-dev g++-5 gcc gcc-5
2019-07-14T12:17:02.5370314Z   git-man libarchive13 libasan2 libasn1-8-heimdal libatomic1
2019-07-14T12:17:02.5375081Z   libbabeltrace-ctf1 libbabeltrace1 libbsd-dev libbsd0 libbz2-1.0 libc-dev-bin
2019-07-14T12:17:02.5375655Z   libedit2 liberror-perl libexpat1 libffi-dev libffi6 libgcc-5-dev libgdbm3
2019-07-14T12:17:02.5375952Z   libglib2.0-0 libgmp10 libgnutls30 libgomp1 libgssapi-krb5-2
2019-07-14T12:17:02.5376192Z   libgssapi3-heimdal libhcrypto4-heimdal libheimbase1-heimdal
2019-07-14T12:17:02.5376459Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-14T12:17:02.5376459Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-14T12:17:02.5376782Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-14T12:17:02.5377347Z   libkrb5-3 libkrb5support0 libldap-2.4-2 libllvm6.0 liblsan0 liblzo2-2
2019-07-14T12:17:02.5377527Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-14T12:17:02.5377739Z   libperl5.22 libpipeline1 libpython-stdlib libpython2.7-minimal
2019-07-14T12:17:02.5377933Z   libpython2.7-stdlib libpython3.5 libpython3.5-minimal libpython3.5-stdlib
2019-07-14T12:17:02.5378126Z   libquadmath0 libroken18-heimdal librtmp1 libsasl2-2 libsasl2-modules-db
2019-07-14T12:17:02.5378348Z   libsqlite3-0 libssl1.0.0 libstdc++-5-dev libtasn1-6 libtinfo-dev libtsan0
2019-07-14T12:17:02.5378537Z   libubsan0 libwind0-heimdal libxml2 linux-libc-dev llvm-6.0 llvm-6.0-dev
2019-07-14T12:17:02.5378719Z   llvm-6.0-runtime mime-support openssl patch perl perl-modules-5.22 python
2019-07-14T12:17:02.5378918Z   python-minimal python2.7-minimal
2019-07-14T12:17:02.5378959Z Suggested packages:
2019-07-14T12:17:02.5379150Z   binutils-doc bzip2-doc codeblocks eclipse ninja-build cpp-doc gcc-5-locales
2019-07-14T12:17:02.5379367Z   debian-keyring g++-multilib g++-5-multilib gcc-5-doc libstdc++6-5-dbg
2019-07-14T12:17:02.5379550Z   gcc-multilib manpages-dev autoconf automake libtool flex bison gcc-doc
2019-07-14T12:17:02.5379940Z   libasan2-dbg liblsan0-dbg libtsan0-dbg libubsan0-dbg libcilkrts5-dbg
2019-07-14T12:17:02.5380399Z   libmpx0-dbg libquadmath0-dbg gdb-doc gettext-base git-daemon-run
2019-07-14T12:17:02.5380399Z   libmpx0-dbg libquadmath0-dbg gdb-doc gettext-base git-daemon-run
2019-07-14T12:17:02.5381295Z   | git-daemon-sysvinit git-doc git-el git-email git-gui gitk gitweb git-arch
2019-07-14T12:17:02.5381631Z   git-cvs git-mediawiki git-svn lrzip glibc-doc gnutls-bin krb5-doc krb5-user
2019-07-14T12:17:02.5381842Z   libstdc++-5-doc llvm-6.0-doc make-doc ed diffutils-doc perl-doc
2019-07-14T12:17:02.5382051Z   libterm-readline-gnu-perl | libterm-readline-perl-perl python-doc python-tk
2019-07-14T12:17:02.5382245Z   python2.7-doc
2019-07-14T12:17:02.5382287Z Recommended packages:
2019-07-14T12:17:02.5382481Z   build-essential fakeroot libalgorithm-merge-perl libc-dbg gdbserver less
2019-07-14T12:17:02.5382666Z   rsync ssh-client manpages manpages-dev libfile-fcntllock-perl
2019-07-14T12:17:02.5382910Z   libglib2.0-data shared-mime-info xdg-user-dirs krb5-locales libsasl2-modules
2019-07-14T12:17:02.5383077Z   libssl-doc xml-core netbase rename
2019-07-14T12:17:02.5383129Z The following NEW packages will be installed:
2019-07-14T12:17:02.5383889Z   binfmt-support binutils bzip2 ca-certificates cmake cmake-data cpp cpp-5
2019-07-14T12:17:02.5384159Z   curl dpkg-dev file g++ g++-5 gcc gcc-5 gdb git git-man libarchive13 libasan2
2019-07-14T12:17:02.5384413Z   libasn1-8-heimdal libatomic1 libbabeltrace-ctf1 libbabeltrace1 libbsd-dev
2019-07-14T12:17:02.5384922Z   libbsd0 libc-dev-bin libc6-dev libcc1-0 libcilkrts5 libcurl3 libcurl3-gnutls
2019-07-14T12:17:02.5385230Z   libdpkg-perl libedit-dev libedit2 liberror-perl libexpat1 libffi-dev libffi6
2019-07-14T12:17:02.5385477Z   libgcc-5-dev libgdbm3 libglib2.0-0 libgmp10 libgnutls30 libgomp1
2019-07-14T12:17:02.5386042Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-14T12:17:02.5386298Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-14T12:17:02.5386619Z   libkrb5-3 libkrb5support0 libldap-2.4-2 libllvm6.0 liblsan0 liblzo2-2
2019-07-14T12:17:02.5387196Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-14T12:17:02.5387196Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-14T12:17:02.5387539Z   libperl5.22 libpipeline1 libpython-stdlib libpython2.7-minimal
2019-07-14T12:17:02.5387770Z   libpython2.7-stdlib libpython3.5 libpython3.5-minimal libpython3.5-stdlib
2019-07-14T12:17:02.5387963Z   libquadmath0 libroken18-heimdal librtmp1 libsasl2-2 libsasl2-modules-db
2019-07-14T12:17:02.5388151Z   libsqlite3-0 libssl-dev libssl1.0.0 libstdc++-5-dev libtasn1-6 libtinfo-dev
2019-07-14T12:17:02.5388376Z   libtsan0 libubsan0 libwind0-heimdal libxml2 linux-libc-dev llvm-6.0
2019-07-14T12:17:02.5388563Z   llvm-6.0-dev llvm-6.0-runtime llvm-6.0-tools make mime-support openssl patch
2019-07-14T12:17:02.5388925Z   perl perl-modules-5.22 pkg-config python python-minimal python2.7
2019-07-14T12:17:02.5389143Z   python2.7-minimal sudo xz-utils zlib1g-dev
2019-07-14T12:17:02.5389184Z The following packages will be upgraded:
2019-07-14T12:17:04.0019800Z 1 upgraded, 115 newly installed, 0 to remove and 4 not upgraded.
2019-07-14T12:17:04.0019975Z Need to get 121 MB of archives.
2019-07-14T12:17:04.0020154Z After this operation, 592 MB of additional disk space will be used.
2019-07-14T12:17:04.0020824Z Get:1 http://archive.ubuntu.com/ubuntu xenial/main amd64 libgdbm3 amd64 1.8.3-13.1 [16.9 kB]
---
2019-07-14T12:17:05.1808299Z Get:61 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 sudo amd64 1.8.16-0ubuntu1.7 [390 kB]
2019-07-14T12:17:05.1920083Z Get:62 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 openssl amd64 1.0.2g-1ubuntu4.15 [492 kB]
2019-07-14T12:17:05.1974086Z Get:63 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 ca-certificates all 20170717~16.04.2 [167 kB]
2019-07-14T12:17:05.1997644Z Get:64 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 libcurl3-gnutls amd64 7.47.0-1ubuntu2.13 [184 kB]
2019-07-14T12:17:05.2024708Z Get:65 http://archive.ubuntu.com/ubuntu xenial/main amd64 libedit2 amd64 3.1-20150325-1ubuntu2 [76.5 kB]
2019-07-14T12:17:05.2040119Z Get:66 http://archive.ubuntu.com/ubuntu xenial/main amd64 libpipeline1 amd64 1.4.1-2 [24.6 kB]
2019-07-14T12:17:05.2045361Z Get:67 http://archive.ubuntu.com/ubuntu xenial/main amd64 binfmt-support amd64 2.1.6-1 [50.7 kB]
2019-07-14T12:17:05.2711297Z Get:69 http://archive.ubuntu.com/ubuntu xenial/main amd64 libisl15 amd64 0.16.1-1 [524 kB]
2019-07-14T12:17:05.2793435Z Get:70 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 cpp-5 amd64 5.4.0-6ubuntu1~16.04.11 [7660 kB]
2019-07-14T12:17:05.4118659Z Get:71 http://archive.ubuntu.com/ubuntu xenial/main amd64 cpp amd64 4:5.3.1-1ubuntu1 [27.7 kB]
2019-07-14T12:17:05.4118999Z Get:72 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 curl amd64 7.47.0-1ubuntu2.13 [139 kB]
---
2019-07-14T12:17:23.2091847Z Unpacking ca-certificates (20170717~16.04.2) ...
2019-07-14T12:17:23.3889444Z Selecting previously unselected package libcurl3-gnutls:amd64.
2019-07-14T12:17:23.3903202Z Preparing to unpack .../libcurl3-gnutls_7.47.0-1ubuntu2.13_amd64.deb ...
2019-07-14T12:17:23.4045320Z Unpacking libcurl3-gnutls:amd64 (7.47.0-1ubuntu2.13) ...
2019-07-14T12:17:23.5368379Z Selecting previously unselected package libedit2:amd64.
2019-07-14T12:17:23.5382014Z Preparing to unpack .../libedit2_3.1-20150325-1ubuntu2_amd64.deb ...
2019-07-14T12:17:23.5600329Z Unpacking libedit2:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-14T12:17:23.6766920Z Selecting previously unselected package libpipeline1:amd64.
2019-07-14T12:17:23.6779091Z Preparing to unpack .../libpipeline1_1.4.1-2_amd64.deb ...
2019-07-14T12:17:23.6940909Z Unpacking libpipeline1:amd64 (1.4.1-2) ...
2019-07-14T12:17:23.8134888Z Selecting previously unselected package binfmt-support.
2019-07-14T12:17:23.8148801Z Preparing to unpack .../binfmt-support_2.1.6-1_amd64.deb ...
2019-07-14T12:17:23.8290591Z Unpacking binfmt-support (2.1.6-1) ...
2019-07-14T12:17:23.9483307Z Preparing to unpack .../binutils_2.26.1-1ubuntu1~16.04.8_amd64.deb ...
2019-07-14T12:17:23.9638617Z Unpacking binutils (2.26.1-1ubuntu1~16.04.8) ...
2019-07-14T12:17:24.4303390Z Selecting previously unselected package libisl15:amd64.
2019-07-14T12:17:24.4323739Z Preparing to unpack .../libisl15_0.16.1-1_amd64.deb ...
---
2019-07-14T12:17:35.5358867Z Selecting previously unselected package libtinfo-dev:amd64.
2019-07-14T12:17:35.5370933Z Preparing to unpack .../libtinfo-dev_6.0+20160213-1ubuntu1_amd64.deb ...
2019-07-14T12:17:35.5511085Z Unpacking libtinfo-dev:amd64 (6.0+20160213-1ubuntu1) ...
2019-07-14T12:17:35.6494737Z Selecting previously unselected package libedit-dev:amd64.
2019-07-14T12:17:35.6507214Z Preparing to unpack .../libedit-dev_3.1-20150325-1ubuntu2_amd64.deb ...
2019-07-14T12:17:35.6648327Z Unpacking libedit-dev:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-14T12:17:35.7761462Z Selecting previously unselected package libllvm6.0:amd64.
2019-07-14T12:17:35.7776332Z Preparing to unpack .../libllvm6.0_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-14T12:17:35.7920019Z Unpacking libllvm6.0:amd64 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-14T12:17:38.4627708Z Selecting previously unselected package zlib1g-dev:amd64.
2019-07-14T12:17:38.4647438Z Preparing to unpack .../zlib1g-dev_1%3a1.2.8.dfsg-2ubuntu4.1_amd64.deb ...
2019-07-14T12:17:38.4781351Z Unpacking zlib1g-dev:amd64 (1:1.2.8.dfsg-2ubuntu4.1) ...
2019-07-14T12:17:38.5873854Z Preparing to unpack .../libssl-dev_1.0.2g-1ubuntu4.15_amd64.deb ...
2019-07-14T12:17:38.6014350Z Unpacking libssl-dev:amd64 (1.0.2g-1ubuntu4.15) ...
2019-07-14T12:17:38.9270455Z Selecting previously unselected package llvm-6.0-runtime.
2019-07-14T12:17:38.9270455Z Selecting previously unselected package llvm-6.0-runtime.
2019-07-14T12:17:38.9289119Z Preparing to unpack .../llvm-6.0-runtime_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-14T12:17:38.9435033Z Unpacking llvm-6.0-runtime (1:6.0-1ubuntu2~16.04.1) ...
2019-07-14T12:17:39.0653084Z Selecting previously unselected package llvm-6.0.
2019-07-14T12:17:39.0665558Z Preparing to unpack .../llvm-6.0_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-14T12:17:39.0823556Z Unpacking llvm-6.0 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-14T12:17:39.6587430Z Selecting previously unselected package libffi-dev:amd64.
2019-07-14T12:17:39.6602672Z Preparing to unpack .../libffi-dev_3.2.1-4_amd64.deb ...
2019-07-14T12:17:39.6752505Z Unpacking libffi-dev:amd64 (3.2.1-4) ...
2019-07-14T12:17:39.8244309Z Selecting previously unselected package llvm-6.0-dev.
2019-07-14T12:17:39.8260113Z Preparing to unpack .../llvm-6.0-dev_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-14T12:17:39.8429382Z Unpacking llvm-6.0-dev (1:6.0-1ubuntu2~16.04.1) ...
2019-07-14T12:17:44.3765342Z Selecting previously unselected package llvm-6.0-tools.
2019-07-14T12:17:44.3786905Z Preparing to unpack .../llvm-6.0-tools_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-14T12:17:44.3932710Z Unpacking llvm-6.0-tools (1:6.0-1ubuntu2~16.04.1) ...
2019-07-14T12:17:44.5511402Z Selecting previously unselected package pkg-config.
2019-07-14T12:17:44.5530029Z Preparing to unpack .../pkg-config_0.29.1-0ubuntu1_amd64.deb ...
2019-07-14T12:17:44.5675299Z Unpacking pkg-config (0.29.1-0ubuntu1) ...
2019-07-14T12:17:44.6819248Z Processing triggers for systemd (229-4ubuntu21.21) ...
2019-07-14T12:17:45.1027379Z Setting up libgdbm3:amd64 (1.8.3-13.1) ...
2019-07-14T12:17:45.1759093Z Setting up libffi6:amd64 (3.2.1-4) ...
2019-07-14T12:17:45.2174135Z Setting up libglib2.0-0:amd64 (2.48.2-0ubuntu4.3) ...
---
2019-07-14T12:17:48.9276356Z debconf: unable to initialize frontend: Dialog
2019-07-14T12:17:48.9277542Z debconf: (TERM is not set, so the dialog frontend is not usable.)
2019-07-14T12:17:48.9277801Z debconf: falling back to frontend: Readline
2019-07-14T12:17:49.3543499Z Setting up libcurl3-gnutls:amd64 (7.47.0-1ubuntu2.13) ...
2019-07-14T12:17:49.3963655Z Setting up libedit2:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-14T12:17:49.4393843Z Setting up libpipeline1:amd64 (1.4.1-2) ...
2019-07-14T12:17:49.4788341Z Setting up binfmt-support (2.1.6-1) ...
2019-07-14T12:17:49.5533798Z mount: permission denied
2019-07-14T12:17:49.5539700Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-14T12:17:49.5552650Z mount: permission denied
2019-07-14T12:17:49.5557672Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-14T12:17:49.6935626Z invoke-rc.d: could not determine current runlevel
2019-07-14T12:17:49.6968255Z invoke-rc.d: policy-rc.d denied execution of start.
2019-07-14T12:17:49.7584091Z Setting up libisl15:amd64 (0.16.1-1) ...
2019-07-14T12:17:49.7998802Z Setting up cpp-5 (5.4.0-6ubuntu1~16.04.11) ...
2019-07-14T12:17:49.8409800Z Setting up cpp (4:5.3.1-1ubuntu1) ...
2019-07-14T12:17:49.8932595Z Setting up curl (7.47.0-1ubuntu2.13) ...
---
2019-07-14T12:17:51.6211825Z Setting up libedit-dev:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-14T12:17:51.6673793Z Setting up libllvm6.0:amd64 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-14T12:17:51.7121783Z Setting up zlib1g-dev:amd64 (1:1.2.8.dfsg-2ubuntu4.1) ...
2019-07-14T12:17:51.7543839Z Setting up libssl-dev:amd64 (1.0.2g-1ubuntu4.15) ...
2019-07-14T12:17:51.7982520Z Setting up llvm-6.0-runtime (1:6.0-1ubuntu2~16.04.1) ...
2019-07-14T12:17:51.8286527Z mount: permission denied
2019-07-14T12:17:51.8290781Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-14T12:17:51.8432293Z Setting up llvm-6.0 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-14T12:17:51.8854408Z Setting up libffi-dev:amd64 (3.2.1-4) ...
2019-07-14T12:17:51.9284206Z Setting up llvm-6.0-dev (1:6.0-1ubuntu2~16.04.1) ...
2019-07-14T12:17:51.9722732Z Setting up llvm-6.0-tools (1:6.0-1ubuntu2~16.04.1) ...
2019-07-14T12:17:52.0164175Z Setting up pkg-config (0.29.1-0ubuntu1) ...
2019-07-14T12:17:52.1441109Z Processing triggers for ca-certificates (20170717~16.04.2) ...
2019-07-14T12:17:52.1629719Z Updating certificates in /etc/ssl/certs...
2019-07-14T12:17:53.4853398Z 148 added, 0 removed; done.
2019-07-14T12:17:53.4854950Z Running hooks in /etc/ca-certificates/update.d...
---
2019-07-14T12:18:25.3636430Z Removing intermediate container 5ecc500c39e9
2019-07-14T12:18:25.3637632Z  ---> dec7370211b3
2019-07-14T12:18:25.3671816Z Successfully built dec7370211b3
2019-07-14T12:18:25.5280312Z Successfully tagged rust-ci:latest
2019-07-14T12:18:25.5895098Z Built container sha256:dec7370211b3ae19107638af48460c3df9d263fbe9c2f817c89beed39e28e9ab
2019-07-14T12:18:25.5911564Z Uploading finished image to https://.s3.amazonaws.com/docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624
2019-07-14T12:19:20.9607509Z upload failed: - to s3:///docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624 Parameter validation failed:
2019-07-14T12:19:20.9608495Z Invalid bucket name "": Bucket name must match the regex "^[a-zA-Z0-9.\-_]{1,255}$"
2019-07-14T12:19:21.9301124Z [CI_JOB_NAME=x86_64-gnu-llvm-6.0]
2019-07-14T12:19:21.9347769Z Starting sccache server...
2019-07-14T12:19:21.9831140Z configure: processing command line
2019-07-14T12:19:21.9831208Z configure: 
---
2019-07-14T13:09:10.7256122Z ....................................................................................F............... 200/5811
2019-07-14T13:09:14.7525766Z .................................................................................................... 300/5811
2019-07-14T13:09:18.2002899Z .................................................................................................... 400/5811
2019-07-14T13:09:22.1406880Z .................................................................................................... 500/5811
2019-07-14T13:09:25.6099187Z ........................................................................i........................... 600/5811
2019-07-14T13:09:33.8014878Z .................................................................................................... 800/5811
2019-07-14T13:09:39.0067823Z .................................................................................................... 900/5811
2019-07-14T13:09:39.0067823Z .................................................................................................... 900/5811
2019-07-14T13:09:44.5393346Z ............................................................................................i....... 1000/5811
2019-07-14T13:09:48.7198376Z ....i............................................................................................... 1100/5811
2019-07-14T13:09:52.6593511Z ......................iiiii......................................................................... 1200/5811
2019-07-14T13:09:57.8334044Z .................................................................................................... 1400/5811
2019-07-14T13:10:00.3771614Z .................................................................................................... 1500/5811
2019-07-14T13:10:04.0712539Z .................................................................................................... 1600/5811
2019-07-14T13:10:06.4731424Z .................................................................................................... 1700/5811
2019-07-14T13:10:06.4731424Z .................................................................................................... 1700/5811
2019-07-14T13:10:09.7211265Z ...........................................................i........................................ 1800/5811
2019-07-14T13:10:17.6077254Z .................................................................................................... 2000/5811
2019-07-14T13:10:21.3342330Z .................................................................................................... 2100/5811
2019-07-14T13:10:25.1652749Z .................................................................................................... 2200/5811
2019-07-14T13:10:25.1652749Z .................................................................................................... 2200/5811
2019-07-14T13:10:28.4612187Z .........................i.......................................................................... 2300/5811
2019-07-14T13:10:37.7529407Z .................................................................................................... 2500/5811
2019-07-14T13:10:42.0889100Z .................................................................................................... 2600/5811
2019-07-14T13:10:46.1557723Z .................................................................................................... 2700/5811
2019-07-14T13:10:50.1979548Z .................................................................................................... 2800/5811
2019-07-14T13:10:50.1979548Z .................................................................................................... 2800/5811
2019-07-14T13:10:53.8490059Z .................................................................................................... 2900/5811
2019-07-14T13:10:58.9051414Z .................................................................................................... 3000/5811
2019-07-14T13:11:03.3263419Z .................................................................................................... 3100/5811
2019-07-14T13:11:07.5873453Z .............................................................F...................................... 3200/5811
2019-07-14T13:11:10.3605467Z .................................................................................................... 3300/5811
2019-07-14T13:11:15.2357476Z .................................................................................................... 3400/5811
2019-07-14T13:11:18.9995430Z ......................................................................................i............. 3500/5811
2019-07-14T13:11:22.6578277Z .................................................................................................... 3600/5811
2019-07-14T13:11:26.7774530Z ............................................................ii...i..ii.............................. 3700/5811
2019-07-14T13:11:35.4976296Z .................................................................................................... 3900/5811
2019-07-14T13:11:35.4976296Z .................................................................................................... 3900/5811
2019-07-14T13:11:39.3005490Z ..........................................................................ii........................ 4000/5811
2019-07-14T13:11:42.5974002Z .................................................................................................... 4100/5811
2019-07-14T13:11:44.6333271Z .i.................................................................................................. 4200/5811
2019-07-14T13:11:46.6366687Z ..................................................................i................................. 4300/5811
2019-07-14T13:11:49.1633745Z .......................................................F............................F............... 4400/5811
2019-07-14T13:11:59.2732340Z .......................................................................................FF.F......... 4500/5811
2019-07-14T13:12:10.8779934Z ............................................................F....................................... 4600/5811
2019-07-14T13:12:17.9472876Z .................................................................................................... 4800/5811
2019-07-14T13:12:24.1715646Z ............................................................................................F....... 4900/5811
2019-07-14T13:12:31.2902537Z .................................................................................................... 5000/5811
2019-07-14T13:12:37.9552378Z .................................................................................................... 5100/5811
2019-07-14T13:12:37.9552378Z .................................................................................................... 5100/5811
2019-07-14T13:12:41.4292877Z .................................................................................................... 5200/5811
2019-07-14T13:12:45.8467595Z .................................................................................................... 5300/5811
2019-07-14T13:12:50.6823851Z .................................................................................................... 5400/5811
2019-07-14T13:12:54.9069075Z .................................................................................................... 5500/5811
2019-07-14T13:12:58.5875235Z .................................................................................................... 5600/5811
2019-07-14T13:13:01.7410183Z .................................................................................................... 5700/5811
2019-07-14T13:13:04.5099700Z ...................................................i................................................ 5800/5811
2019-07-14T13:13:05.1359240Z failures:
2019-07-14T13:13:05.1417009Z 
2019-07-14T13:13:05.1417573Z ---- [ui] ui/async-await/issues/issue-60674.rs stdout ----
2019-07-14T13:13:05.1417675Z diff of stdout:
2019-07-14T13:13:05.1417675Z diff of stdout:
2019-07-14T13:13:05.1417706Z 
2019-07-14T13:13:05.1417927Z - async fn f(mut x: u8) { }
2019-07-14T13:13:05.1418189Z - async fn g((mut x, y, mut z): (u8, u8, u8)) { }
2019-07-14T13:13:05.1418437Z - async fn g(mut x: u8, (a, mut b, c): (u8, u8, u8), y: u8) { }
2019-07-14T13:13:05.1418489Z + async fn f(, mut x: u8) { }
2019-07-14T13:13:05.1418554Z + async fn g(, (, mut x, y, mut z): (, u8, u8, u8)) { }
2019-07-14T13:13:05.1418620Z + async fn g(, mut x: u8, (, a, mut b, c): (, u8, u8, u8), y: u8) { }
2019-07-14T13:13:05.1418713Z 
2019-07-14T13:13:05.1418739Z 
2019-07-14T13:13:05.1418786Z The actual stdout differed from the expected stdout.
2019-07-14T13:13:05.1419096Z Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-60674/issue-60674.stdout
2019-07-14T13:13:05.1419096Z Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-60674/issue-60674.stdout
2019-07-14T13:13:05.1419362Z To update references, rerun the tests and pass the `--bless` flag
2019-07-14T13:13:05.1419633Z To only update this specific test, also pass `--test-args async-await/issues/issue-60674.rs`
2019-07-14T13:13:05.1419741Z error: 1 errors occurred comparing output.
2019-07-14T13:13:05.1419789Z status: exit code: 0
2019-07-14T13:13:05.1419789Z status: exit code: 0
2019-07-14T13:13:05.1420836Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issues/issue-60674.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-60674" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-60674/auxiliary" "-A" "unused"
2019-07-14T13:13:05.1421124Z ------------------------------------------
2019-07-14T13:13:05.1421124Z ------------------------------------------
2019-07-14T13:13:05.1421184Z async fn f(, mut x: u8) { }
2019-07-14T13:13:05.1421389Z async fn g(, (, mut x, y, mut z): (, u8, u8, u8)) { }
2019-07-14T13:13:05.1421434Z async fn g(, mut x: u8, (, a, mut b, c): (, u8, u8, u8), y: u8) { }
2019-07-14T13:13:05.1421695Z ------------------------------------------
2019-07-14T13:13:05.1421734Z stderr:
2019-07-14T13:13:05.1421914Z ------------------------------------------
2019-07-14T13:13:05.1421955Z 
2019-07-14T13:13:05.1421955Z 
2019-07-14T13:13:05.1422144Z ------------------------------------------
2019-07-14T13:13:05.1422170Z 
2019-07-14T13:13:05.1422193Z 
2019-07-14T13:13:05.1422554Z ---- [ui] ui/issues/issue-60662.rs stdout ----
2019-07-14T13:13:05.1422595Z diff of stdout:
2019-07-14T13:13:05.1422618Z 
2019-07-14T13:13:05.1422805Z 1 // build-pass (FIXME(62277): could be check-pass?)
2019-07-14T13:13:05.1422998Z 2 // compile-flags: -Z unpretty=hir
2019-07-14T13:13:05.1423036Z 3 
2019-07-14T13:13:05.1423209Z - #![feature(existential_type)]
2019-07-14T13:13:05.1423265Z + #![feature(, existential_type)]
2019-07-14T13:13:05.1423301Z 5 #[prelude_import]
2019-07-14T13:13:05.1423347Z 6 use ::std::prelude::v1::*;
2019-07-14T13:13:05.1423383Z 7 #[macro_use]
2019-07-14T13:13:05.1423445Z 
2019-07-14T13:13:05.1423647Z The actual stdout differed from the expected stdout.
2019-07-14T13:13:05.1424054Z Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-60662/issue-60662.stdout
2019-07-14T13:13:05.1424054Z Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-60662/issue-60662.stdout
2019-07-14T13:13:05.1424359Z To update references, rerun the tests and pass the `--bless` flag
2019-07-14T13:13:05.1424619Z To only update this specific test, also pass `--test-args issues/issue-60662.rs`
2019-07-14T13:13:05.1424704Z error: 1 errors occurred comparing output.
2019-07-14T13:13:05.1424742Z status: exit code: 0
2019-07-14T13:13:05.1424742Z status: exit code: 0
2019-07-14T13:13:05.1425328Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-60662.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-60662" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unpretty=hir" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-60662/auxiliary" "-A" "unused"
2019-07-14T13:13:05.1425609Z ------------------------------------------
2019-07-14T13:13:05.1425609Z ------------------------------------------
2019-07-14T13:13:05.1425825Z // build-pass (FIXME(62277): could be check-pass?)
2019-07-14T13:13:05.1426006Z // compile-flags: -Z unpretty=hir
2019-07-14T13:13:05.1426033Z 
2019-07-14T13:13:05.1426085Z #![feature(, existential_type)]
2019-07-14T13:13:05.1426121Z #[prelude_import]
2019-07-14T13:13:05.1426157Z use ::std::prelude::v1::*;
2019-07-14T13:13:05.1426193Z #[macro_use]
2019-07-14T13:13:05.1426245Z extern crate std;
2019-07-14T13:13:05.1426267Z 
2019-07-14T13:13:05.1426300Z trait Animal { }
2019-07-14T13:13:05.1426323Z 
2019-07-14T13:13:05.1426373Z fn main() {
2019-07-14T13:13:05.1426418Z               pub existential type ServeFut : Animal;
2019-07-14T13:13:05.1426640Z 
2019-07-14T13:13:05.1427084Z ------------------------------------------
2019-07-14T13:13:05.1427132Z stderr:
2019-07-14T13:13:05.1427346Z ------------------------------------------
2019-07-14T13:13:05.1427346Z ------------------------------------------
2019-07-14T13:13:05.1427394Z 
2019-07-14T13:13:05.1427610Z ------------------------------------------
2019-07-14T13:13:05.1427641Z 
2019-07-14T13:13:05.1427677Z 
2019-07-14T13:13:05.1427914Z ---- [ui] ui/parser/trait-object-polytrait-priority.rs stdout ----
2019-07-14T13:13:05.1427985Z diff of stderr:
2019-07-14T13:13:05.1428015Z 
2019-07-14T13:13:05.1428277Z - error[E0178]: expected a path on the left-hand side of `+`, not `&for<'a> Trait<'a>`
2019-07-14T13:13:05.1428566Z + error[E0178]: expected a path on the left-hand side of `+`, not `&for<, 'a> Trait<, 'a>`
2019-07-14T13:13:05.1428806Z 2   --> $DIR/trait-object-polytrait-priority.rs:6:12
2019-07-14T13:13:05.1428854Z 3    |
2019-07-14T13:13:05.1429213Z 4 LL |     let _: &for<'a> Trait<'a> + 'static;
2019-07-14T13:13:05.1429247Z 
2019-07-14T13:13:05.1429527Z -    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try adding parentheses: `&(for<'a> Trait<'a> + 'static)`
2019-07-14T13:13:05.1429835Z +    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try adding parentheses: `&(for<, 'a> Trait<, 'a> + 'static)`
2019-07-14T13:13:05.1429950Z 7 error: aborting due to previous error
2019-07-14T13:13:05.1430178Z 8 
2019-07-14T13:13:05.1430201Z 
2019-07-14T13:13:05.1430223Z 
2019-07-14T13:13:05.1430223Z 
2019-07-14T13:13:05.1430264Z The actual stderr differed from the expected stderr.
2019-07-14T13:13:05.1430747Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/trait-object-polytrait-priority/trait-object-polytrait-priority.stderr
2019-07-14T13:13:05.1431116Z To update references, rerun the tests and pass the `--bless` flag
2019-07-14T13:13:05.1431336Z To only update this specific test, also pass `--test-args parser/trait-object-polytrait-priority.rs`
2019-07-14T13:13:05.1431425Z error: 1 errors occurred comparing output.
2019-07-14T13:13:05.1431461Z status: exit code: 1
2019-07-14T13:13:05.1431461Z status: exit code: 1
2019-07-14T13:13:05.1432142Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/trait-object-polytrait-priority.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/trait-object-polytrait-priority" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/trait-object-polytrait-priority/auxiliary" "-A" "unused"
2019-07-14T13:13:05.1432608Z ------------------------------------------
2019-07-14T13:13:05.1432636Z 
2019-07-14T13:13:05.1432811Z ------------------------------------------
2019-07-14T13:13:05.1432856Z stderr:
2019-07-14T13:13:05.1432856Z stderr:
2019-07-14T13:13:05.1433046Z ------------------------------------------
2019-07-14T13:13:05.1433260Z error[E0178]: expected a path on the left-hand side of `+`, not `&for<, 'a> Trait<, 'a>`
2019-07-14T13:13:05.1433529Z    |
2019-07-14T13:13:05.1433529Z    |
2019-07-14T13:13:05.1433707Z LL |     let _: &for<'a> Trait<'a> + 'static;
2019-07-14T13:13:05.1433942Z    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try adding parentheses: `&(for<, 'a> Trait<, 'a> + 'static)`
2019-07-14T13:13:05.1434027Z error: aborting due to previous error
2019-07-14T13:13:05.1434051Z 
2019-07-14T13:13:05.1434245Z For more information about this error, try `rustc --explain E0178`.
2019-07-14T13:13:05.1434288Z 
2019-07-14T13:13:05.1434288Z 
2019-07-14T13:13:05.1434461Z ------------------------------------------
2019-07-14T13:13:05.1434487Z 
2019-07-14T13:13:05.1434508Z 
2019-07-14T13:13:05.1434881Z ---- [ui] ui/pattern/const-pat-ice.rs stdout ----
2019-07-14T13:13:05.1434930Z diff of stderr:
2019-07-14T13:13:05.1434954Z 
2019-07-14T13:13:05.1435393Z - thread 'rustc' panicked at 'assertion failed: rows.iter().all(|r| r.len() == v.len())', src/librustc_mir/hair/pattern/_match.rs:LL:CC
2019-07-14T13:13:05.1435681Z + thread 'rustc' panicked at 'assertion failed: rows.iter().all(|, r| r.len() == v.len())', src/librustc_mir/hair/pattern/_match.rs:LL:CC
2019-07-14T13:13:05.1435744Z 2 note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-07-14T13:13:05.1435840Z 4 error: internal compiler error: unexpected panic
2019-07-14T13:13:05.1435867Z 
2019-07-14T13:13:05.1435890Z 
2019-07-14T13:13:05.1435946Z The actual stderr differed from the expected stderr.
2019-07-14T13:13:05.1435946Z The actual stderr differed from the expected stderr.
2019-07-14T13:13:05.1436204Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/const-pat-ice/const-pat-ice.stderr
2019-07-14T13:13:05.1436415Z To update references, rerun the tests and pass the `--bless` flag
2019-07-14T13:13:05.1437344Z To only update this specific test, also pass `--test-args pattern/const-pat-ice.rs`
2019-07-14T13:13:05.1437428Z error: 1 errors occurred comparing output.
2019-07-14T13:13:05.1437473Z status: exit code: 101
2019-07-14T13:13:05.1437473Z status: exit code: 101
2019-07-14T13:13:05.1441056Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/pattern/const-pat-ice.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/const-pat-ice" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/const-pat-ice/auxiliary" "-A" "unused"
2019-07-14T13:13:05.1441430Z ------------------------------------------
2019-07-14T13:13:05.1441462Z 
2019-07-14T13:13:05.1441652Z ------------------------------------------
2019-07-14T13:13:05.1441707Z stderr:
2019-07-14T13:13:05.1441707Z stderr:
2019-07-14T13:13:05.1441881Z ------------------------------------------
2019-07-14T13:13:05.1442134Z thread 'rustc' panicked at 'assertion failed: rows.iter().all(|, r| r.len() == v.len())', src/librustc_mir/hair/pattern/_match.rs:1085:5
2019-07-14T13:13:05.1442349Z 
2019-07-14T13:13:05.1442396Z error: internal compiler error: unexpected panic
2019-07-14T13:13:05.1442421Z 
2019-07-14T13:13:05.1442421Z 
2019-07-14T13:13:05.1442475Z note: the compiler unexpectedly panicked. this is a bug.
2019-07-14T13:13:05.1442500Z 
2019-07-14T13:13:05.1442849Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-07-14T13:13:05.1442897Z 
2019-07-14T13:13:05.1443103Z note: rustc 1.38.0-dev running on x86_64-unknown-linux-gnu
2019-07-14T13:13:05.1443131Z 
2019-07-14T13:13:05.1443357Z note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2019-07-14T13:13:05.1443433Z 
2019-07-14T13:13:05.1443606Z ------------------------------------------
2019-07-14T13:13:05.1443632Z 
2019-07-14T13:13:05.1443653Z 
2019-07-14T13:13:05.1443653Z 
2019-07-14T13:13:05.1444021Z ---- [ui] ui/proc-macro/dollar-crate-issue-57089.rs stdout ----
2019-07-14T13:13:05.1444060Z diff of stdout:
2019-07-14T13:13:05.1444082Z 
2019-07-14T13:13:05.1444124Z 38         span: #2 bytes(LO..HI),
2019-07-14T13:13:05.1444210Z 40 ]
2019-07-14T13:13:05.1444210Z 40 ]
2019-07-14T13:13:05.1444388Z - PRINT-ATTR INPUT (DISPLAY): struct A(crate::S);
2019-07-14T13:13:05.1444583Z + PRINT-ATTR INPUT (DISPLAY): struct A(, crate::S);
2019-07-14T13:13:05.1444775Z 42 PRINT-ATTR RE-COLLECTED (DISPLAY): struct A ( $crate :: S ) ;
2019-07-14T13:13:05.1444948Z 43 PRINT-ATTR INPUT (DEBUG): TokenStream [
2019-07-14T13:13:05.1445022Z 
2019-07-14T13:13:05.1445043Z 
2019-07-14T13:13:05.1445087Z The actual stdout differed from the expected stdout.
2019-07-14T13:13:05.1445362Z Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/dollar-crate-issue-57089/dollar-crate-issue-57089.stdout
2019-07-14T13:13:05.1445362Z Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/dollar-crate-issue-57089/dollar-crate-issue-57089.stdout
2019-07-14T13:13:05.1445560Z To update references, rerun the tests and pass the `--bless` flag
2019-07-14T13:13:05.1445780Z To only update this specific test, also pass `--test-args proc-macro/dollar-crate-issue-57089.rs`
2019-07-14T13:13:05.1445866Z error: 1 errors occurred comparing output.
2019-07-14T13:13:05.1446081Z status: exit code: 0
2019-07-14T13:13:05.1446081Z status: exit code: 0
2019-07-14T13:13:05.1447174Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/dollar-crate-issue-57089.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/dollar-crate-issue-57089" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/dollar-crate-issue-57089/auxiliary" "-A" "unused"
2019-07-14T13:13:05.1447667Z ------------------------------------------
2019-07-14T13:13:05.1447667Z ------------------------------------------
2019-07-14T13:13:05.1447899Z PRINT-BANG INPUT (DISPLAY): struct M ( $crate :: S ) ;
2019-07-14T13:13:05.1448130Z PRINT-BANG INPUT (DEBUG): TokenStream [
2019-07-14T13:13:05.1448194Z     Ident {
2019-07-14T13:13:05.1448240Z         ident: "struct",
2019-07-14T13:13:05.1448286Z         span: #2 bytes(436..442),
2019-07-14T13:13:05.1448392Z     Ident {
2019-07-14T13:13:05.1448392Z     Ident {
2019-07-14T13:13:05.1448436Z         ident: "M",
2019-07-14T13:13:05.1448482Z         span: #2 bytes(443..444),
2019-07-14T13:13:05.1448586Z     Group {
2019-07-14T13:13:05.1448586Z     Group {
2019-07-14T13:13:05.1448630Z         delimiter: Parenthesis,
2019-07-14T13:13:05.1448694Z         stream: TokenStream [
2019-07-14T13:13:05.1448746Z             Ident {
2019-07-14T13:13:05.1448791Z                 ident: "$crate",
2019-07-14T13:13:05.1448839Z                 span: #2 bytes(445..451),
2019-07-14T13:13:05.1448946Z             Punct {
2019-07-14T13:13:05.1448946Z             Punct {
2019-07-14T13:13:05.1449151Z                 ch: ':',
2019-07-14T13:13:05.1449215Z                 spacing: Joint,
2019-07-14T13:13:05.1449334Z                 span: #2 bytes(451..453),
2019-07-14T13:13:05.1449444Z             Punct {
2019-07-14T13:13:05.1449444Z             Punct {
2019-07-14T13:13:05.1449657Z                 ch: ':',
2019-07-14T13:13:05.1449704Z                 spacing: Alone,
2019-07-14T13:13:05.1449751Z                 span: #2 bytes(451..453),
2019-07-14T13:13:05.1450016Z             Ident {
2019-07-14T13:13:05.1450016Z             Ident {
2019-07-14T13:13:05.1450057Z                 ident: "S",
2019-07-14T13:13:05.1450116Z                 span: #2 bytes(453..454),
2019-07-14T13:13:05.1450201Z         ],
2019-07-14T13:13:05.1450201Z         ],
2019-07-14T13:13:05.1450256Z         span: #2 bytes(444..455),
2019-07-14T13:13:05.1450332Z     Punct {
2019-07-14T13:13:05.1450332Z     Punct {
2019-07-14T13:13:05.1451909Z         ch: ';',
2019-07-14T13:13:05.1452009Z         spacing: Alone,
2019-07-14T13:13:05.1452048Z         span: #2 bytes(455..456),
2019-07-14T13:13:05.1452133Z ]
2019-07-14T13:13:05.1452133Z ]
2019-07-14T13:13:05.1452583Z PRINT-ATTR INPUT (DISPLAY): struct A(, crate::S);
2019-07-14T13:13:05.1452950Z PRINT-ATTR RE-COLLECTED (DISPLAY): struct A ( $crate :: S ) ;
2019-07-14T13:13:05.1453186Z PRINT-ATTR INPUT (DEBUG): TokenStream [
2019-07-14T13:13:05.1453230Z     Ident {
2019-07-14T13:13:05.1453266Z         ident: "struct",
2019-07-14T13:13:05.1453302Z         span: #2 bytes(498..504),
2019-07-14T13:13:05.1454046Z     Ident {
2019-07-14T13:13:05.1454046Z     Ident {
2019-07-14T13:13:05.1454079Z         ident: "A",
2019-07-14T13:13:05.1454132Z         span: #2 bytes(505..506),
2019-07-14T13:13:05.1454209Z     Group {
2019-07-14T13:13:05.1454209Z     Group {
2019-07-14T13:13:05.1454260Z         delimiter: Parenthesis,
2019-07-14T13:13:05.1454296Z         stream: TokenStream [
2019-07-14T13:13:05.1454329Z             Ident {
2019-07-14T13:13:05.1454363Z                 ident: "$crate",
2019-07-14T13:13:05.1454607Z                 span: #2 bytes(507..513),
2019-07-14T13:13:05.1454687Z             Punct {
2019-07-14T13:13:05.1454687Z             Punct {
2019-07-14T13:13:05.1454947Z                 ch: ':',
2019-07-14T13:13:05.1454986Z                 spacing: Joint,
2019-07-14T13:13:05.1455022Z                 span: #2 bytes(513..515),
2019-07-14T13:13:05.1455715Z             Punct {
2019-07-14T13:13:05.1455715Z             Punct {
2019-07-14T13:13:05.1455941Z                 ch: ':',
2019-07-14T13:13:05.1456400Z                 spacing: Alone,
2019-07-14T13:13:05.1456967Z                 span: #2 bytes(513..515),
2019-07-14T13:13:05.1457068Z             Ident {
2019-07-14T13:13:05.1457068Z             Ident {
2019-07-14T13:13:05.1457325Z                 ident: "S",
2019-07-14T13:13:05.1457373Z                 span: #2 bytes(515..516),
2019-07-14T13:13:05.1481756Z         ],
2019-07-14T13:13:05.1481756Z         ],
2019-07-14T13:13:05.1481842Z         span: #2 bytes(506..517),
2019-07-14T13:13:05.1481915Z     Punct {
2019-07-14T13:13:05.1481915Z     Punct {
2019-07-14T13:13:05.1482244Z         ch: ';',
2019-07-14T13:13:05.1482286Z         spacing: Alone,
2019-07-14T13:13:05.1482339Z         span: #2 bytes(517..518),
2019-07-14T13:13:05.1482425Z ]
2019-07-14T13:13:05.1482449Z 
2019-07-14T13:13:05.1482637Z ------------------------------------------
2019-07-14T13:13:05.1482692Z stderr:
---
2019-07-14T13:13:05.1483124Z 
2019-07-14T13:13:05.1483320Z ---- [ui] ui/proc-macro/dollar-crate-issue-62325.rs stdout ----
2019-07-14T13:13:05.1483370Z diff of stdout:
2019-07-14T13:13:05.1483409Z 
2019-07-14T13:13:05.1483762Z - PRINT-ATTR INPUT (DISPLAY): struct A(identity!(crate :: S));
2019-07-14T13:13:05.1484119Z + PRINT-ATTR INPUT (DISPLAY): struct A(, identity!(crate :: S));
2019-07-14T13:13:05.1484333Z 2 PRINT-ATTR RE-COLLECTED (DISPLAY): struct A ( identity ! ( $crate :: S ) ) ;
2019-07-14T13:13:05.1484503Z 3 PRINT-ATTR INPUT (DEBUG): TokenStream [
2019-07-14T13:13:05.1484698Z 
2019-07-14T13:13:05.1484698Z 
2019-07-14T13:13:05.1484748Z 54         span: #2 bytes(LO..HI),
2019-07-14T13:13:05.1484818Z 56 ]
2019-07-14T13:13:05.1484818Z 56 ]
2019-07-14T13:13:05.1485084Z - PRINT-ATTR INPUT (DISPLAY): struct B(identity!(::dollar_crate_external :: S));
2019-07-14T13:13:05.1485308Z + PRINT-ATTR INPUT (DISPLAY): struct B(, identity!(::dollar_crate_external :: S));
2019-07-14T13:13:05.1485526Z 58 PRINT-ATTR RE-COLLECTED (DISPLAY): struct B ( identity ! ( $crate :: S ) ) ;
2019-07-14T13:13:05.1485728Z 59 PRINT-ATTR INPUT (DEBUG): TokenStream [
2019-07-14T13:13:05.1485798Z 
2019-07-14T13:13:05.1485819Z 
2019-07-14T13:13:05.1485873Z The actual stdout differed from the expected stdout.
2019-07-14T13:13:05.1486141Z Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/dollar-crate-issue-62325/dollar-crate-issue-62325.stdout
2019-07-14T13:13:05.1486141Z Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/dollar-crate-issue-62325/dollar-crate-issue-62325.stdout
2019-07-14T13:13:05.1486352Z To update references, rerun the tests and pass the `--bless` flag
2019-07-14T13:13:05.1486997Z To only update this specific test, also pass `--test-args proc-macro/dollar-crate-issue-62325.rs`
2019-07-14T13:13:05.1487085Z error: 1 errors occurred comparing output.
2019-07-14T13:13:05.1487133Z status: exit code: 0
2019-07-14T13:13:05.1487133Z status: exit code: 0
2019-07-14T13:13:05.1487956Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/dollar-crate-issue-62325.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/dollar-crate-issue-62325" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/dollar-crate-issue-62325/auxiliary" "-A" "unused"
2019-07-14T13:13:05.1488308Z ------------------------------------------
2019-07-14T13:13:05.1488308Z ------------------------------------------
2019-07-14T13:13:05.1488546Z PRINT-ATTR INPUT (DISPLAY): struct A(, identity!(crate :: S));
2019-07-14T13:13:05.1488820Z PRINT-ATTR RE-COLLECTED (DISPLAY): struct A ( identity ! ( $crate :: S ) ) ;
2019-07-14T13:13:05.1489039Z PRINT-ATTR INPUT (DEBUG): TokenStream [
2019-07-14T13:13:05.1489088Z     Ident {
2019-07-14T13:13:05.1489146Z         ident: "struct",
2019-07-14T13:13:05.1489193Z         span: #2 bytes(490..496),
2019-07-14T13:13:05.1489299Z     Ident {
2019-07-14T13:13:05.1489299Z     Ident {
2019-07-14T13:13:05.1489462Z         ident: "A",
2019-07-14T13:13:05.1489507Z         span: #2 bytes(497..498),
2019-07-14T13:13:05.1489607Z     Group {
2019-07-14T13:13:05.1489607Z     Group {
2019-07-14T13:13:05.1489652Z         delimiter: Parenthesis,
2019-07-14T13:13:05.1489697Z         stream: TokenStream [
2019-07-14T13:13:05.1489758Z             Ident {
2019-07-14T13:13:05.1489803Z                 ident: "identity",
2019-07-14T13:13:05.1489859Z                 span: #2 bytes(499..507),
2019-07-14T13:13:05.1490122Z             Punct {
2019-07-14T13:13:05.1490122Z             Punct {
2019-07-14T13:13:05.1490670Z                 ch: '!',
2019-07-14T13:13:05.1490709Z                 spacing: Alone,
2019-07-14T13:13:05.1490763Z                 span: #2 bytes(507..508),
2019-07-14T13:13:05.1490834Z             Group {
2019-07-14T13:13:05.1490834Z             Group {
2019-07-14T13:13:05.1490888Z                 delimiter: Parenthesis,
2019-07-14T13:13:05.1490926Z                 stream: TokenStream [
2019-07-14T13:13:05.1490971Z                     Ident {
2019-07-14T13:13:05.1491008Z                         ident: "$crate",
2019-07-14T13:13:05.1491065Z                         span: #2 bytes(509..515),
2019-07-14T13:13:05.1491139Z                     Punct {
2019-07-14T13:13:05.1491139Z                     Punct {
2019-07-14T13:13:05.1491331Z                         ch: ':',
2019-07-14T13:13:05.1491371Z                         spacing: Joint,
2019-07-14T13:13:05.1491476Z                         span: #2 bytes(515..517),
2019-07-14T13:13:05.1491571Z                     Punct {
2019-07-14T13:13:05.1491571Z                     Punct {
2019-07-14T13:13:05.1491754Z                         ch: ':',
2019-07-14T13:13:05.1491809Z                         spacing: Alone,
2019-07-14T13:13:05.1491848Z                         span: #2 bytes(515..517),
2019-07-14T13:13:05.1491923Z                     Ident {
2019-07-14T13:13:05.1491923Z                     Ident {
2019-07-14T13:13:05.1491977Z                         ident: "S",
2019-07-14T13:13:05.1492024Z                         span: #2 bytes(517..518),
2019-07-14T13:13:05.1492115Z                 ],
2019-07-14T13:13:05.1492115Z                 ],
2019-07-14T13:13:05.1492152Z                 span: #2 bytes(508..519),
2019-07-14T13:13:05.1492240Z         ],
2019-07-14T13:13:05.1492240Z         ],
2019-07-14T13:13:05.1492276Z         span: #2 bytes(498..520),
2019-07-14T13:13:05.1492344Z     Punct {
2019-07-14T13:13:05.1492344Z     Punct {
2019-07-14T13:13:05.1492528Z         ch: ';',
2019-07-14T13:13:05.1492566Z         spacing: Alone,
2019-07-14T13:13:05.1492604Z         span: #2 bytes(520..521),
2019-07-14T13:13:05.1492685Z ]
2019-07-14T13:13:05.1492685Z ]
2019-07-14T13:13:05.1492890Z PRINT-ATTR INPUT (DISPLAY): struct B(, identity!(::dollar_crate_external :: S));
2019-07-14T13:13:05.1493113Z PRINT-ATTR RE-COLLECTED (DISPLAY): struct B ( identity ! ( $crate :: S ) ) ;
2019-07-14T13:13:05.1493290Z PRINT-ATTR INPUT (DEBUG): TokenStream [
2019-07-14T13:13:05.1493329Z     Ident {
2019-07-14T13:13:05.1493371Z         ident: "struct",
2019-07-14T13:13:05.1493425Z         span: #7 bytes(8297634..8297640),
2019-07-14T13:13:05.1493495Z     Ident {
2019-07-14T13:13:05.1493495Z     Ident {
2019-07-14T13:13:05.1493550Z         ident: "B",
2019-07-14T13:13:05.1493587Z         span: #7 bytes(8297641..8297642),
2019-07-14T13:13:05.1493671Z     Group {
2019-07-14T13:13:05.1493671Z     Group {
2019-07-14T13:13:05.1493707Z         delimiter: Parenthesis,
2019-07-14T13:13:05.1493750Z         stream: TokenStream [
2019-07-14T13:13:05.1493786Z             Ident {
2019-07-14T13:13:05.1493841Z                 ident: "identity",
2019-07-14T13:13:05.1493880Z                 span: #7 bytes(8297645..8297653),
2019-07-14T13:13:05.1493970Z             Punct {
2019-07-14T13:13:05.1493970Z             Punct {
2019-07-14T13:13:05.1494135Z                 ch: '!',
2019-07-14T13:13:05.1494174Z                 spacing: Alone,
2019-07-14T13:13:05.1494229Z                 span: #7 bytes(8297654..8297655),
2019-07-14T13:13:05.1494368Z             Group {
2019-07-14T13:13:05.1494368Z             Group {
2019-07-14T13:13:05.1494405Z                 delimiter: Parenthesis,
2019-07-14T13:13:05.1494458Z                 stream: TokenStream [
2019-07-14T13:13:05.1494495Z                     Ident {
2019-07-14T13:13:05.1494534Z                         ident: "$crate",
2019-07-14T13:13:05.1494591Z                         span: #7 bytes(8297658..8297665),
2019-07-14T13:13:05.1494672Z                     Punct {
2019-07-14T13:13:05.1494672Z                     Punct {
2019-07-14T13:13:05.1494877Z                         ch: ':',
2019-07-14T13:13:05.1494918Z                         spacing: Joint,
2019-07-14T13:13:05.1494958Z                         span: #7 bytes(8297666..8297668),
2019-07-14T13:13:05.1495047Z                     Punct {
2019-07-14T13:13:05.1495047Z                     Punct {
2019-07-14T13:13:05.1495215Z                         ch: ':',
2019-07-14T13:13:05.1495256Z                         spacing: Alone,
2019-07-14T13:13:05.1495311Z                         span: #7 bytes(8297666..8297668),
2019-07-14T13:13:05.1495392Z                     Ident {
2019-07-14T13:13:05.1495392Z                     Ident {
2019-07-14T13:13:05.1495446Z                         ident: "S",
2019-07-14T13:13:05.1495486Z                         span: #7 bytes(8297669..8297670),
2019-07-14T13:13:05.1495574Z                 ],
2019-07-14T13:13:05.1495574Z                 ],
2019-07-14T13:13:05.1495614Z                 span: #7 bytes(8297656..8297672),
2019-07-14T13:13:05.1495752Z         ],
2019-07-14T13:13:05.1495752Z         ],
2019-07-14T13:13:05.1495805Z         span: #7 bytes(8297643..8297674),
2019-07-14T13:13:05.1495873Z     Punct {
2019-07-14T13:13:05.1495873Z     Punct {
2019-07-14T13:13:05.1496061Z         ch: ';',
2019-07-14T13:13:05.1496099Z         spacing: Alone,
2019-07-14T13:13:05.1496137Z         span: #7 bytes(8297675..8297676),
2019-07-14T13:13:05.1496222Z ]
2019-07-14T13:13:05.1496245Z 
2019-07-14T13:13:05.1496420Z ------------------------------------------
2019-07-14T13:13:05.1496480Z stderr:
---
2019-07-14T13:13:05.1497402Z 
2019-07-14T13:13:05.1497623Z ---- [ui] ui/proc-macro/dollar-crate.rs stdout ----
2019-07-14T13:13:05.1497672Z diff of stdout:
2019-07-14T13:13:05.1497701Z 
2019-07-14T13:13:05.1497772Z 38         span: #2 bytes(LO..HI),
2019-07-14T13:13:05.1497862Z 40 ]
2019-07-14T13:13:05.1497862Z 40 ]
2019-07-14T13:13:05.1498101Z - PRINT-ATTR INPUT (DISPLAY): struct A(crate::S);
2019-07-14T13:13:05.1498327Z + PRINT-ATTR INPUT (DISPLAY): struct A(, crate::S);
2019-07-14T13:13:05.1498565Z 42 PRINT-ATTR RE-COLLECTED (DISPLAY): struct A ( $crate :: S ) ;
2019-07-14T13:13:05.1498801Z 43 PRINT-ATTR INPUT (DEBUG): TokenStream [
2019-07-14T13:13:05.1498877Z 
2019-07-14T13:13:05.1498877Z 
2019-07-14T13:13:05.1498920Z 79         span: #2 bytes(LO..HI),
2019-07-14T13:13:05.1499031Z 81 ]
2019-07-14T13:13:05.1499031Z 81 ]
2019-07-14T13:13:05.1499253Z - PRINT-DERIVE INPUT (DISPLAY): struct D(crate::S);
2019-07-14T13:13:05.1499495Z + PRINT-DERIVE INPUT (DISPLAY): struct D(, crate::S);
2019-07-14T13:13:05.1499738Z 83 PRINT-DERIVE RE-COLLECTED (DISPLAY): struct D ( $crate :: S ) ;
2019-07-14T13:13:05.1500120Z 84 PRINT-DERIVE INPUT (DEBUG): TokenStream [
2019-07-14T13:13:05.1500203Z 
2019-07-14T13:13:05.1500203Z 
2019-07-14T13:13:05.1500240Z 160         span: #10 bytes(LO..HI),
2019-07-14T13:13:05.1500326Z 162 ]
2019-07-14T13:13:05.1500326Z 162 ]
2019-07-14T13:13:05.1500520Z - PRINT-ATTR INPUT (DISPLAY): struct A(::dollar_crate_external::S);
2019-07-14T13:13:05.1500717Z + PRINT-ATTR INPUT (DISPLAY): struct A(, ::dollar_crate_external::S);
2019-07-14T13:13:05.1500912Z 164 PRINT-ATTR RE-COLLECTED (DISPLAY): struct A ( $crate :: S ) ;
2019-07-14T13:13:05.1501108Z 165 PRINT-ATTR INPUT (DEBUG): TokenStream [
2019-07-14T13:13:05.1501257Z 
2019-07-14T13:13:05.1501257Z 
2019-07-14T13:13:05.1501309Z 201         span: #10 bytes(LO..HI),
2019-07-14T13:13:05.1501378Z 203 ]
2019-07-14T13:13:05.1501378Z 203 ]
2019-07-14T13:13:05.1501592Z - PRINT-DERIVE INPUT (DISPLAY): struct D(::dollar_crate_external::S);
2019-07-14T13:13:05.1501809Z + PRINT-DERIVE INPUT (DISPLAY): struct D(, ::dollar_crate_external::S);
2019-07-14T13:13:05.1502015Z 205 PRINT-DERIVE RE-COLLECTED (DISPLAY): struct D ( $crate :: S ) ;
2019-07-14T13:13:05.1502199Z 206 PRINT-DERIVE INPUT (DEBUG): TokenStream [
2019-07-14T13:13:05.1502274Z 
2019-07-14T13:13:05.1502296Z 
2019-07-14T13:13:05.1502333Z The actual stdout differed from the expected stdout.
2019-07-14T13:13:05.1502582Z Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/dollar-crate/dollar-crate.stdout
2019-07-14T13:13:05.1502582Z Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/dollar-crate/dollar-crate.stdout
2019-07-14T13:13:05.1502784Z To update references, rerun the tests and pass the `--bless` flag
2019-07-14T13:13:05.1503012Z To only update this specific test, also pass `--test-args proc-macro/dollar-crate.rs`
2019-07-14T13:13:05.1503079Z error: 1 errors occurred comparing output.
2019-07-14T13:13:05.1503117Z status: exit code: 1
2019-07-14T13:13:05.1503117Z status: exit code: 1
2019-07-14T13:13:05.1503816Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/dollar-crate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/dollar-crate" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/dollar-crate/auxiliary" "-A" "unused"
2019-07-14T13:13:05.1504104Z ------------------------------------------
2019-07-14T13:13:05.1504104Z ------------------------------------------
2019-07-14T13:13:05.1504292Z PRINT-BANG INPUT (DISPLAY): struct M ( $crate :: S ) ;
2019-07-14T13:13:05.1504480Z PRINT-BANG INPUT (DEBUG): TokenStream [
2019-07-14T13:13:05.1504530Z     Ident {
2019-07-14T13:13:05.1504566Z         ident: "struct",
2019-07-14T13:13:05.1504603Z         span: #2 bytes(491..497),
2019-07-14T13:13:05.1504679Z     Ident {
2019-07-14T13:13:05.1504679Z     Ident {
2019-07-14T13:13:05.1504714Z         ident: "M",
2019-07-14T13:13:05.1504751Z         span: #2 bytes(498..499),
2019-07-14T13:13:05.1504836Z     Group {
2019-07-14T13:13:05.1504836Z     Group {
2019-07-14T13:13:05.1504873Z         delimiter: Parenthesis,
2019-07-14T13:13:05.1504915Z         stream: TokenStream [
2019-07-14T13:13:05.1504951Z             Ident {
2019-07-14T13:13:05.1504987Z                 ident: "$crate",
2019-07-14T13:13:05.1505038Z                 span: #2 bytes(500..506),
2019-07-14T13:13:05.1505109Z             Punct {
2019-07-14T13:13:05.1505109Z             Punct {
2019-07-14T13:13:05.1505274Z                 ch: ':',
2019-07-14T13:13:05.1505325Z                 spacing: Joint,
2019-07-14T13:13:05.1505371Z                 span: #2 bytes(506..508),
2019-07-14T13:13:05.1505454Z             Punct {
2019-07-14T13:13:05.1505454Z             Punct {
2019-07-14T13:13:05.1505613Z                 ch: ':',
2019-07-14T13:13:05.1505652Z                 spacing: Alone,
2019-07-14T13:13:05.1505698Z                 span: #2 bytes(506..508),
2019-07-14T13:13:05.1505933Z             Ident {
2019-07-14T13:13:05.1505933Z             Ident {
2019-07-14T13:13:05.1506146Z                 ident: "S",
2019-07-14T13:13:05.1506190Z                 span: #2 bytes(508..509),
2019-07-14T13:13:05.1506255Z         ],
2019-07-14T13:13:05.1506255Z         ],
2019-07-14T13:13:05.1506296Z         span: #2 bytes(499..510),
2019-07-14T13:13:05.1506360Z     Punct {
2019-07-14T13:13:05.1506360Z     Punct {
2019-07-14T13:13:05.1506858Z         ch: ';',
2019-07-14T13:13:05.1506921Z         spacing: Alone,
2019-07-14T13:13:05.1506966Z         span: #2 bytes(510..511),
2019-07-14T13:13:05.1507064Z ]
2019-07-14T13:13:05.1507064Z ]
2019-07-14T13:13:05.1507410Z PRINT-ATTR INPUT (DISPLAY): struct A(, crate::S);
2019-07-14T13:13:05.1507647Z PRINT-ATTR RE-COLLECTED (DISPLAY): struct A ( $crate :: S ) ;
2019-07-14T13:13:05.1507882Z PRINT-ATTR INPUT (DEBUG): TokenStream [
2019-07-14T13:13:05.1507932Z     Ident {
2019-07-14T13:13:05.1507974Z         ident: "struct",
2019-07-14T13:13:05.1508035Z         span: #2 bytes(565..571),
2019-07-14T13:13:05.1508128Z     Ident {
2019-07-14T13:13:05.1508128Z     Ident {
2019-07-14T13:13:05.1508170Z         ident: "A",
2019-07-14T13:13:05.1508220Z         span: #2 bytes(572..573),
2019-07-14T13:13:05.1508303Z     Group {
2019-07-14T13:13:05.1508303Z     Group {
2019-07-14T13:13:05.1508359Z         delimiter: Parenthesis,
2019-07-14T13:13:05.1508404Z         stream: TokenStream [
2019-07-14T13:13:05.1508449Z             Ident {
2019-07-14T13:13:05.1508493Z                 ident: "$crate",
2019-07-14T13:13:05.1508555Z                 span: #2 bytes(574..580),
2019-07-14T13:13:05.1508650Z             Punct {
2019-07-14T13:13:05.1508650Z             Punct {
2019-07-14T13:13:05.1508863Z                 ch: ':',
2019-07-14T13:13:05.1508910Z                 spacing: Joint,
2019-07-14T13:13:05.1508957Z                 span: #2 bytes(580..582),
2019-07-14T13:13:05.1509055Z             Punct {
2019-07-14T13:13:05.1509055Z             Punct {
2019-07-14T13:13:05.1509248Z                 ch: ':',
2019-07-14T13:13:05.1509295Z                 spacing: Alone,
2019-07-14T13:13:05.1509419Z                 span: #2 bytes(580..582),
2019-07-14T13:13:05.1509510Z             Ident {
2019-07-14T13:13:05.1509510Z             Ident {
2019-07-14T13:13:05.1509569Z                 ident: "S",
2019-07-14T13:13:05.1509616Z                 span: #2 bytes(582..583),
2019-07-14T13:13:05.1509716Z         ],
2019-07-14T13:13:05.1509716Z         ],
2019-07-14T13:13:05.1509759Z         span: #2 bytes(573..584),
2019-07-14T13:13:05.1509843Z     Punct {
2019-07-14T13:13:05.1509843Z     Punct {
2019-07-14T13:13:05.1510364Z         ch: ';',
2019-07-14T13:13:05.1510401Z         spacing: Alone,
2019-07-14T13:13:05.1510624Z         span: #2 bytes(584..585),
2019-07-14T13:13:05.1510702Z ]
2019-07-14T13:13:05.1510702Z ]
2019-07-14T13:13:05.1510884Z PRINT-DERIVE INPUT (DISPLAY): struct D(, crate::S);
2019-07-14T13:13:05.1511086Z PRINT-DERIVE RE-COLLECTED (DISPLAY): struct D ( $crate :: S ) ;
2019-07-14T13:13:05.1511266Z PRINT-DERIVE INPUT (DEBUG): TokenStream [
2019-07-14T13:13:05.1511304Z     Ident {
2019-07-14T13:13:05.1511346Z         ident: "struct",
2019-07-14T13:13:05.1511389Z         span: #2 bytes(628..634),
2019-07-14T13:13:05.1511456Z     Ident {
2019-07-14T13:13:05.1511456Z     Ident {
2019-07-14T13:13:05.1511497Z         ident: "D",
2019-07-14T13:13:05.1511532Z         span: #2 bytes(635..636),
2019-07-14T13:13:05.1511600Z     Group {
2019-07-14T13:13:05.1511600Z     Group {
2019-07-14T13:13:05.1511646Z         delimiter: Parenthesis,
2019-07-14T13:13:05.1511683Z         stream: TokenStream [
2019-07-14T13:13:05.1511719Z             Ident {
2019-07-14T13:13:05.1511769Z                 ident: "$crate",
2019-07-14T13:13:05.1511814Z                 span: #2 bytes(637..643),
2019-07-14T13:13:05.1511898Z             Punct {
2019-07-14T13:13:05.1511898Z             Punct {
2019-07-14T13:13:05.1512066Z                 ch: ':',
2019-07-14T13:13:05.1512104Z                 spacing: Joint,
2019-07-14T13:13:05.1512142Z                 span: #2 bytes(643..645),
2019-07-14T13:13:05.1512229Z             Punct {
2019-07-14T13:13:05.1512229Z             Punct {
2019-07-14T13:13:05.1512389Z                 ch: ':',
2019-07-14T13:13:05.1512438Z                 spacing: Alone,
2019-07-14T13:13:05.1512477Z                 span: #2 bytes(643..645),
2019-07-14T13:13:05.1512560Z             Ident {
2019-07-14T13:13:05.1512560Z             Ident {
2019-07-14T13:13:05.1512596Z                 ident: "S",
2019-07-14T13:13:05.1512633Z                 span: #2 bytes(645..646),
2019-07-14T13:13:05.1512713Z         ],
2019-07-14T13:13:05.1512713Z         ],
2019-07-14T13:13:05.1512748Z         span: #2 bytes(636..647),
2019-07-14T13:13:05.1512892Z     Punct {
2019-07-14T13:13:05.1512892Z     Punct {
2019-07-14T13:13:05.1513061Z         ch: ';',
2019-07-14T13:13:05.1513099Z         spacing: Alone,
2019-07-14T13:13:05.1513135Z         span: #2 bytes(647..648),
2019-07-14T13:13:05.1513211Z ]
2019-07-14T13:13:05.1513211Z ]
2019-07-14T13:13:05.1513396Z PRINT-BANG INPUT (DISPLAY): struct M ( $crate :: S ) ;
2019-07-14T13:13:05.1513741Z PRINT-BANG INPUT (DEBUG): TokenStream [
2019-07-14T13:13:05.1513956Z     Ident {
2019-07-14T13:13:05.1513992Z         ident: "struct",
2019-07-14T13:13:05.1514040Z         span: #10 bytes(8297947..8297953),
2019-07-14T13:13:05.1514103Z     Ident {
2019-07-14T13:13:05.1514103Z     Ident {
2019-07-14T13:13:05.1514136Z         ident: "M",
2019-07-14T13:13:05.1514186Z         span: #10 bytes(8297954..8297955),
2019-07-14T13:13:05.1514250Z     Group {
2019-07-14T13:13:05.1514250Z     Group {
2019-07-14T13:13:05.1514295Z         delimiter: Parenthesis,
2019-07-14T13:13:05.1514330Z         stream: TokenStream [
2019-07-14T13:13:05.1514368Z             Ident {
2019-07-14T13:13:05.1514411Z                 ident: "$crate",
2019-07-14T13:13:05.1514449Z                 span: #10 bytes(8297958..8297965),
2019-07-14T13:13:05.1514515Z             Punct {
2019-07-14T13:13:05.1514515Z             Punct {
2019-07-14T13:13:05.1514677Z                 ch: ':',
2019-07-14T13:13:05.1514713Z                 spacing: Joint,
2019-07-14T13:13:05.1514811Z                 span: #10 bytes(8297966..8297968),
2019-07-14T13:13:05.1514891Z             Punct {
2019-07-14T13:13:05.1514891Z             Punct {
2019-07-14T13:13:05.1515238Z                 ch: ':',
2019-07-14T13:13:05.1515285Z                 spacing: Alone,
2019-07-14T13:13:05.1515323Z                 span: #10 bytes(8297966..8297968),
2019-07-14T13:13:05.1515392Z             Ident {
2019-07-14T13:13:05.1515392Z             Ident {
2019-07-14T13:13:05.1515440Z                 ident: "S",
2019-07-14T13:13:05.1515478Z                 span: #10 bytes(8297969..8297970),
2019-07-14T13:13:05.1515567Z         ],
2019-07-14T13:13:05.1515567Z         ],
2019-07-14T13:13:05.1515603Z         span: #10 bytes(8297956..8297972),
2019-07-14T13:13:05.1515773Z     Punct {
2019-07-14T13:13:05.1515773Z     Punct {
2019-07-14T13:13:05.1515925Z         ch: ';',
2019-07-14T13:13:05.1515962Z         spacing: Alone,
2019-07-14T13:13:05.1515999Z         span: #10 bytes(8297973..8297974),
2019-07-14T13:13:05.1516076Z ]
2019-07-14T13:13:05.1516076Z ]
2019-07-14T13:13:05.1516270Z PRINT-ATTR INPUT (DISPLAY): struct A(, ::dollar_crate_external::S);
2019-07-14T13:13:05.1517662Z PRINT-ATTR RE-COLLECTED (DISPLAY): struct A ( $crate :: S ) ;
2019-07-14T13:13:05.1517929Z PRINT-ATTR INPUT (DEBUG): TokenStream [
2019-07-14T13:13:05.1517975Z     Ident {
2019-07-14T13:13:05.1518031Z         ident: "struct",
2019-07-14T13:13:05.1518077Z         span: #10 bytes(8297994..8298000),
2019-07-14T13:13:05.1518160Z     Ident {
2019-07-14T13:13:05.1518160Z     Ident {
2019-07-14T13:13:05.1518208Z         ident: "A",
2019-07-14T13:13:05.1518254Z         span: #10 bytes(8298001..8298002),
2019-07-14T13:13:05.1518357Z     Group {
2019-07-14T13:13:05.1518357Z     Group {
2019-07-14T13:13:05.1518401Z         delimiter: Parenthesis,
2019-07-14T13:13:05.1518445Z         stream: TokenStream [
2019-07-14T13:13:05.1518489Z             Ident {
2019-07-14T13:13:05.1518539Z                 ident: "$crate",
2019-07-14T13:13:05.1518587Z                 span: #10 bytes(8298005..8298012),
2019-07-14T13:13:05.1518694Z             Punct {
2019-07-14T13:13:05.1518694Z             Punct {
2019-07-14T13:13:05.1518896Z                 ch: ':',
2019-07-14T13:13:05.1518943Z                 spacing: Joint,
2019-07-14T13:13:05.1519005Z                 span: #10 bytes(8298013..8298015),
2019-07-14T13:13:05.1519093Z             Punct {
2019-07-14T13:13:05.1519093Z             Punct {
2019-07-14T13:13:05.1519287Z                 ch: ':',
2019-07-14T13:13:05.1519346Z                 spacing: Alone,
2019-07-14T13:13:05.1519393Z                 span: #10 bytes(8298013..8298015),
2019-07-14T13:13:05.1519605Z             Ident {
2019-07-14T13:13:05.1519605Z             Ident {
2019-07-14T13:13:05.1519650Z                 ident: "S",
2019-07-14T13:13:05.1519696Z                 span: #10 bytes(8298016..8298017),
2019-07-14T13:13:05.1519790Z         ],
2019-07-14T13:13:05.1519790Z         ],
2019-07-14T13:13:05.1519835Z         span: #10 bytes(8298003..8298019),
2019-07-14T13:13:05.1519926Z     Punct {
2019-07-14T13:13:05.1519926Z     Punct {
2019-07-14T13:13:05.1520295Z         ch: ';',
2019-07-14T13:13:05.1520510Z         spacing: Alone,
2019-07-14T13:13:05.1520563Z         span: #10 bytes(8298020..8298021),
2019-07-14T13:13:05.1520635Z ]
2019-07-14T13:13:05.1520635Z ]
2019-07-14T13:13:05.1520843Z PRINT-DERIVE INPUT (DISPLAY): struct D(, ::dollar_crate_external::S);
2019-07-14T13:13:05.1521067Z PRINT-DERIVE RE-COLLECTED (DISPLAY): struct D ( $crate :: S ) ;
2019-07-14T13:13:05.1521256Z PRINT-DERIVE INPUT (DEBUG): TokenStream [
2019-07-14T13:13:05.1521296Z     Ident {
2019-07-14T13:13:05.1521347Z         ident: "struct",
2019-07-14T13:13:05.1521394Z         span: #10 bytes(8298045..8298051),
2019-07-14T13:13:05.1521476Z     Ident {
2019-07-14T13:13:05.1521476Z     Ident {
2019-07-14T13:13:05.1521514Z         ident: "D",
2019-07-14T13:13:05.1521552Z         span: #10 bytes(8298052..8298053),
2019-07-14T13:13:05.1521634Z     Group {
2019-07-14T13:13:05.1521634Z     Group {
2019-07-14T13:13:05.1521672Z         delimiter: Parenthesis,
2019-07-14T13:13:05.1521988Z         stream: TokenStream [
2019-07-14T13:13:05.1522054Z             Ident {
2019-07-14T13:13:05.1522094Z                 ident: "$crate",
2019-07-14T13:13:05.1522135Z                 span: #10 bytes(8298056..8298063),
2019-07-14T13:13:05.1522221Z             Punct {
2019-07-14T13:13:05.1522221Z             Punct {
2019-07-14T13:13:05.1522455Z                 ch: ':',
2019-07-14T13:13:05.1522830Z                 spacing: Joint,
2019-07-14T13:13:05.1522902Z                 span: #10 bytes(8298064..8298066),
2019-07-14T13:13:05.1522992Z             Punct {
2019-07-14T13:13:05.1522992Z             Punct {
2019-07-14T13:13:05.1523219Z                 ch: ':',
2019-07-14T13:13:05.1524729Z                 spacing: Alone,
2019-07-14T13:13:05.1524783Z                 span: #10 bytes(8298064..8298066),
2019-07-14T13:13:05.1524873Z             Ident {
2019-07-14T13:13:05.1524873Z             Ident {
2019-07-14T13:13:05.1524912Z                 ident: "S",
2019-07-14T13:13:05.1524952Z                 span: #10 bytes(8298067..8298068),
2019-07-14T13:13:05.1525051Z         ],
2019-07-14T13:13:05.1525051Z         ],
2019-07-14T13:13:05.1525089Z         span: #10 bytes(8298054..8298070),
2019-07-14T13:13:05.1525176Z     Punct {
2019-07-14T13:13:05.1525176Z     Punct {
2019-07-14T13:13:05.1525421Z         ch: ';',
2019-07-14T13:13:05.1525475Z         spacing: Alone,
2019-07-14T13:13:05.1525515Z         span: #10 bytes(8298071..8298072),
2019-07-14T13:13:05.1525588Z ]
2019-07-14T13:13:05.1525622Z 
2019-07-14T13:13:05.1525807Z ------------------------------------------
2019-07-14T13:13:05.1525856Z stderr:
2019-07-14T13:13:05.1525856Z stderr:
2019-07-14T13:13:05.1526205Z ------------------------------------------
2019-07-14T13:13:05.1526261Z error[E0428]: the name `D` is defined multiple times
2019-07-14T13:13:05.1526875Z    |
2019-07-14T13:13:05.1526875Z    |
2019-07-14T13:13:05.1526938Z LL |             struct D($crate::S); //~ ERROR the name `D` is defined multiple times
2019-07-14T13:13:05.1527056Z    |             |
2019-07-14T13:13:05.1527056Z    |             |
2019-07-14T13:13:05.1527110Z    |             `D` redefined here
2019-07-14T13:13:05.1527160Z    |             previous definition of the type `D` here
2019-07-14T13:13:05.1527205Z ...
2019-07-14T13:13:05.1527261Z LL |     local!();
2019-07-14T13:13:05.1527559Z    |
2019-07-14T13:13:05.1527559Z    |
2019-07-14T13:13:05.1527623Z    = note: `D` must be defined only once in the type namespace of this module
2019-07-14T13:13:05.1527658Z 
2019-07-14T13:13:05.1527860Z error[E0428]: the name `D` is defined multiple times
2019-07-14T13:13:05.1528185Z    |
2019-07-14T13:13:05.1528185Z    |
2019-07-14T13:13:05.1528236Z LL |     dollar_crate_external::external!(); //~ ERROR the name `D` is defined multiple times
2019-07-14T13:13:05.1528345Z    |     |
2019-07-14T13:13:05.1528345Z    |     |
2019-07-14T13:13:05.1528399Z    |     `D` redefined here
2019-07-14T13:13:05.1528447Z    |     previous definition of the type `D` here
2019-07-14T13:13:05.1528504Z    |
2019-07-14T13:13:05.1528553Z    = note: `D` must be defined only once in the type namespace of this module
2019-07-14T13:13:05.1528876Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-07-14T13:13:05.1528988Z error: aborting due to 2 previous errors
2019-07-14T13:13:05.1529021Z 
2019-07-14T13:13:05.1529297Z For more information about this error, try `rustc --explain E0428`.
2019-07-14T13:13:05.1529342Z 
2019-07-14T13:13:05.1529342Z 
2019-07-14T13:13:05.1529576Z ------------------------------------------
2019-07-14T13:13:05.1529609Z 
2019-07-14T13:13:05.1529637Z 
2019-07-14T13:13:05.1529880Z ---- [ui] ui/qualified/qualified-path-params.rs stdout ----
2019-07-14T13:13:05.1529942Z diff of stderr:
2019-07-14T13:13:05.1529972Z 
2019-07-14T13:13:05.1530633Z - error[E0533]: expected unit struct/variant or constant, found method `<<S as Tr>::A>::f<u8>`
2019-07-14T13:13:05.1530707Z + error[E0533]: expected unit struct/variant or constant, found method `<<S as Tr>::A>::f<, u8>`
2019-07-14T13:13:05.1530926Z 2   --> $DIR/qualified-path-params.rs:20:9
2019-07-14T13:13:05.1530967Z 3    |
2019-07-14T13:13:05.1531022Z 4 LL |         <S as Tr>::A::f::<u8> => {}
2019-07-14T13:13:05.1531073Z 
2019-07-14T13:13:05.1531115Z The actual stderr differed from the expected stderr.
2019-07-14T13:13:05.1531115Z The actual stderr differed from the expected stderr.
2019-07-14T13:13:05.1531411Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/qualified/qualified-path-params/qualified-path-params.stderr
2019-07-14T13:13:05.1531699Z To update references, rerun the tests and pass the `--bless` flag
2019-07-14T13:13:05.1532103Z To only update this specific test, also pass `--test-args qualified/qualified-path-params.rs`
2019-07-14T13:13:05.1535769Z error: 1 errors occurred comparing output.
2019-07-14T13:13:05.1535826Z status: exit code: 1
2019-07-14T13:13:05.1535826Z status: exit code: 1
2019-07-14T13:13:05.1537255Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/qualified/qualified-path-params.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/qualified/qualified-path-params" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/qualified/qualified-path-params/auxiliary" "-A" "unused"
2019-07-14T13:13:05.1537607Z ------------------------------------------
2019-07-14T13:13:05.1537643Z 
2019-07-14T13:13:05.1537857Z ------------------------------------------
2019-07-14T13:13:05.1537904Z stderr:
2019-07-14T13:13:05.1537904Z stderr:
2019-07-14T13:13:05.1538127Z ------------------------------------------
2019-07-14T13:13:05.1538192Z error[E0533]: expected unit struct/variant or constant, found method `<<S as Tr>::A>::f<, u8>`
2019-07-14T13:13:05.1538445Z   --> /checkout/src/test/ui/qualified/qualified-path-params.rs:20:9
2019-07-14T13:13:05.1538513Z    |
2019-07-14T13:13:05.1538558Z LL |         <S as Tr>::A::f::<u8> => {}
2019-07-14T13:13:05.1538651Z 
2019-07-14T13:13:05.1538651Z 
2019-07-14T13:13:05.1538700Z error[E0029]: only char and numeric types are allowed in range patterns
2019-07-14T13:13:05.1538947Z   --> /checkout/src/test/ui/qualified/qualified-path-params.rs:22:15
2019-07-14T13:13:05.1539142Z    |
2019-07-14T13:13:05.1539197Z LL |         0 ..= <S as Tr>::A::f::<u8> => {} //~ ERROR only char and numeric types are allowed in range
2019-07-14T13:13:05.1539255Z    |               ^^^^^^^^^^^^^^^^^^^^^ ranges require char or numeric types
2019-07-14T13:13:05.1539316Z    |
2019-07-14T13:13:05.1539360Z    = note: start type: {integer}
2019-07-14T13:13:05.1539408Z    = note: end type: fn() {S::f::<u8>}
2019-07-14T13:13:05.1539503Z error: aborting due to 2 previous errors
2019-07-14T13:13:05.1539533Z 
2019-07-14T13:13:05.1539801Z For more information about this error, try `rustc --explain E0029`.
2019-07-14T13:13:05.1539835Z 
2019-07-14T13:13:05.1539835Z 
2019-07-14T13:13:05.1540545Z ------------------------------------------
2019-07-14T13:13:05.1540574Z 
2019-07-14T13:13:05.1540596Z 
2019-07-14T13:13:05.1541231Z ---- [ui] ui/rfc-2497-if-let-chains/ast-pretty-check.rs stdout ----
2019-07-14T13:13:05.1541283Z diff of stdout:
2019-07-14T13:13:05.1541309Z 
2019-07-14T13:13:05.1541494Z - #![feature(prelude_import)]
2019-07-14T13:13:05.1541536Z + #![feature(, prelude_import)]
2019-07-14T13:13:05.1541585Z 2 #![no_std]
2019-07-14T13:13:05.1541624Z 3 #[prelude_import]
2019-07-14T13:13:05.1541663Z 4 use ::std::prelude::v1::*;
2019-07-14T13:13:05.1541722Z 
2019-07-14T13:13:05.1541762Z The actual stdout differed from the expected stdout.
2019-07-14T13:13:05.1542111Z Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2497-if-let-chains/ast-pretty-check/ast-pretty-check.stdout
2019-07-14T13:13:05.1542111Z Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2497-if-let-chains/ast-pretty-check/ast-pretty-check.stdout
2019-07-14T13:13:05.1542371Z To update references, rerun the tests and pass the `--bless` flag
2019-07-14T13:13:05.1542617Z To only update this specific test, also pass `--test-args rfc-2497-if-let-chains/ast-pretty-check.rs`
2019-07-14T13:13:05.1542703Z error: 1 errors occurred comparing output.
2019-07-14T13:13:05.1542744Z status: exit code: 0
2019-07-14T13:13:05.1542744Z status: exit code: 0
2019-07-14T13:13:05.1543444Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2497-if-let-chains/ast-pretty-check.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2497-if-let-chains/ast-pretty-check" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unpretty=expanded" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2497-if-let-chains/ast-pretty-check/auxiliary" "-A" "unused"
2019-07-14T13:13:05.1543740Z ------------------------------------------
2019-07-14T13:13:05.1543740Z ------------------------------------------
2019-07-14T13:13:05.1543796Z #![feature(, prelude_import)]
2019-07-14T13:13:05.1543836Z #![no_std]
2019-07-14T13:13:05.1543873Z #[prelude_import]
2019-07-14T13:13:05.1543925Z use ::std::prelude::v1::*;
2019-07-14T13:13:05.1543963Z #[macro_use]
2019-07-14T13:13:05.1544000Z extern crate std;
2019-07-14T13:13:05.1544208Z // build-pass (FIXME(62277): could be check-pass?)
2019-07-14T13:13:05.1544406Z // compile-flags: -Z unpretty=expanded
2019-07-14T13:13:05.1544434Z 
2019-07-14T13:13:05.1544638Z fn main() { if let 0 = 1 { } }
2019-07-14T13:13:05.1544851Z ------------------------------------------
2019-07-14T13:13:05.1544890Z stderr:
2019-07-14T13:13:05.1545069Z ------------------------------------------
2019-07-14T13:13:05.1545095Z 
---
2019-07-14T13:13:05.1552300Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:535:22
2019-07-14T13:13:05.1552541Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-07-14T13:13:05.1552744Z 
2019-07-14T13:13:05.1552793Z 
2019-07-14T13:13:05.1555096Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-07-14T13:13:05.1555340Z 
2019-07-14T13:13:05.1555364Z 
2019-07-14T13:13:05.1555404Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-07-14T13:13:05.1555447Z Build completed unsuccessfully in 0:50:27
2019-07-14T13:13:05.1555447Z Build completed unsuccessfully in 0:50:27
2019-07-14T13:13:06.0416799Z ##[error]Bash exited with code '1'.
2019-07-14T13:13:06.0479710Z ##[section]Starting: Checkout
2019-07-14T13:13:06.0482029Z ==============================================================================
2019-07-14T13:13:06.0482072Z Task         : Get sources
2019-07-14T13:13:06.0482109Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
