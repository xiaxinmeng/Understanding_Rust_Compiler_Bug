plain
2019-07-15T00:33:06.4215857Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-15T00:33:06.4425658Z ##[command]git config gc.auto 0
2019-07-15T00:33:07.3335101Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-15T00:33:07.3337317Z ##[command]git config --get-all http.proxy
2019-07-15T00:33:07.3340244Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62682/merge:refs/remotes/pull/62682/merge
---
2019-07-15T00:33:40.6914547Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-15T00:33:40.6914582Z 
2019-07-15T00:33:40.6914816Z   git checkout -b <new-branch-name>
2019-07-15T00:33:40.6914868Z 
2019-07-15T00:33:40.6914922Z HEAD is now at 45d9c5e7d Merge a907b7c51929eccd5af8774c60ac412772f2cacd into 83e4eed16ef7adb54a802e3b684427e0e912c2b7
2019-07-15T00:33:40.7073480Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-15T00:33:40.7076367Z ==============================================================================
2019-07-15T00:33:40.7076434Z Task         : Bash
2019-07-15T00:33:40.7076503Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-15T00:35:33.1356206Z Attempting with retry: curl -y 30 -Y 10 --connect-timeout 30 -f -L -C - -o /tmp/rustci_docker_cache https://.s3.amazonaws.com/docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624
2019-07-15T00:35:33.1428934Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-15T00:35:33.1429461Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-15T00:35:33.1429721Z 
2019-07-15T00:35:33.1438052Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-15T00:35:34.1527152Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-15T00:35:34.1527580Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-15T00:35:34.1556148Z 
2019-07-15T00:35:34.1556148Z 
2019-07-15T00:35:34.1577571Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-15T00:35:36.1655880Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-15T00:35:36.1656459Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-15T00:35:36.1656740Z 
2019-07-15T00:35:36.1656740Z 
2019-07-15T00:35:36.1699490Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-15T00:35:39.1771988Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-15T00:35:39.1772653Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-15T00:35:39.1772759Z 
2019-07-15T00:35:39.1772759Z 
2019-07-15T00:35:39.1816609Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-15T00:35:43.1905065Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-15T00:35:43.1905639Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-15T00:35:43.1905734Z 
2019-07-15T00:35:43.1905734Z 
2019-07-15T00:35:43.1948833Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-15T00:35:43.1954696Z The command has failed after 5 attempts.
2019-07-15T00:35:43.3306052Z open /tmp/rustci_docker_cache: no such file or directory
2019-07-15T00:35:43.3342329Z Attempting with retry: docker build --rm -t rust-ci -f /home/vsts/work/1/s/src/ci/docker/x86_64-gnu-llvm-6.0/Dockerfile /home/vsts/work/1/s/src/ci/docker
2019-07-15T00:35:43.5433548Z Sending build context to Docker daemon  521.7kB
2019-07-15T00:35:43.5434595Z 
2019-07-15T00:35:43.5691705Z Step 1/8 : FROM ubuntu:16.04
---
2019-07-15T00:35:59.6439557Z Reading package lists...
2019-07-15T00:36:00.7242096Z Reading package lists...
2019-07-15T00:36:00.8938257Z Building dependency tree...
2019-07-15T00:36:00.8938379Z Reading state information...
2019-07-15T00:36:01.0206927Z The following additional packages will be installed:
2019-07-15T00:36:01.0207783Z   binfmt-support binutils bzip2 cmake-data cpp cpp-5 dpkg-dev g++-5 gcc gcc-5
2019-07-15T00:36:01.0208086Z   git-man libarchive13 libasan2 libasn1-8-heimdal libatomic1
2019-07-15T00:36:01.0208497Z   libbabeltrace-ctf1 libbabeltrace1 libbsd-dev libbsd0 libbz2-1.0 libc-dev-bin
2019-07-15T00:36:01.0209062Z   libedit2 liberror-perl libexpat1 libffi-dev libffi6 libgcc-5-dev libgdbm3
2019-07-15T00:36:01.0209390Z   libglib2.0-0 libgmp10 libgnutls30 libgomp1 libgssapi-krb5-2
2019-07-15T00:36:01.0209655Z   libgssapi3-heimdal libhcrypto4-heimdal libheimbase1-heimdal
2019-07-15T00:36:01.0209954Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-15T00:36:01.0209954Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-15T00:36:01.0210793Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-15T00:36:01.0211094Z   libkrb5-3 libkrb5support0 libldap-2.4-2 libllvm6.0 liblsan0 liblzo2-2
2019-07-15T00:36:01.0211369Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-15T00:36:01.0211699Z   libperl5.22 libpipeline1 libpython-stdlib libpython2.7-minimal
2019-07-15T00:36:01.0212328Z   libpython2.7-stdlib libpython3.5 libpython3.5-minimal libpython3.5-stdlib
2019-07-15T00:36:01.0212613Z   libquadmath0 libroken18-heimdal librtmp1 libsasl2-2 libsasl2-modules-db
2019-07-15T00:36:01.0212968Z   libsqlite3-0 libssl1.0.0 libstdc++-5-dev libtasn1-6 libtinfo-dev libtsan0
2019-07-15T00:36:01.0213247Z   libubsan0 libwind0-heimdal libxml2 linux-libc-dev llvm-6.0 llvm-6.0-dev
2019-07-15T00:36:01.0213529Z   llvm-6.0-runtime mime-support openssl patch perl perl-modules-5.22 python
2019-07-15T00:36:01.0213830Z   python-minimal python2.7-minimal
2019-07-15T00:36:01.0213889Z Suggested packages:
2019-07-15T00:36:01.0214170Z   binutils-doc bzip2-doc codeblocks eclipse ninja-build cpp-doc gcc-5-locales
2019-07-15T00:36:01.0214491Z   debian-keyring g++-multilib g++-5-multilib gcc-5-doc libstdc++6-5-dbg
2019-07-15T00:36:01.0214765Z   gcc-multilib manpages-dev autoconf automake libtool flex bison gcc-doc
2019-07-15T00:36:01.0215373Z   libasan2-dbg liblsan0-dbg libtsan0-dbg libubsan0-dbg libcilkrts5-dbg
2019-07-15T00:36:01.0215646Z   libmpx0-dbg libquadmath0-dbg gdb-doc gettext-base git-daemon-run
2019-07-15T00:36:01.0215646Z   libmpx0-dbg libquadmath0-dbg gdb-doc gettext-base git-daemon-run
2019-07-15T00:36:01.0216058Z   | git-daemon-sysvinit git-doc git-el git-email git-gui gitk gitweb git-arch
2019-07-15T00:36:01.0216443Z   git-cvs git-mediawiki git-svn lrzip glibc-doc gnutls-bin krb5-doc krb5-user
2019-07-15T00:36:01.0216713Z   libstdc++-5-doc llvm-6.0-doc make-doc ed diffutils-doc perl-doc
2019-07-15T00:36:01.0217001Z   libterm-readline-gnu-perl | libterm-readline-perl-perl python-doc python-tk
2019-07-15T00:36:01.0217283Z   python2.7-doc
2019-07-15T00:36:01.0217339Z Recommended packages:
2019-07-15T00:36:01.0217610Z   build-essential fakeroot libalgorithm-merge-perl libc-dbg gdbserver less
2019-07-15T00:36:01.0217922Z   rsync ssh-client manpages manpages-dev libfile-fcntllock-perl
2019-07-15T00:36:01.0218213Z   libglib2.0-data shared-mime-info xdg-user-dirs krb5-locales libsasl2-modules
2019-07-15T00:36:01.0218467Z   libssl-doc xml-core netbase rename
2019-07-15T00:36:01.0218575Z The following NEW packages will be installed:
2019-07-15T00:36:01.0218898Z   binfmt-support binutils bzip2 ca-certificates cmake cmake-data cpp cpp-5
2019-07-15T00:36:01.0219214Z   curl dpkg-dev file g++ g++-5 gcc gcc-5 gdb git git-man libarchive13 libasan2
2019-07-15T00:36:01.0219569Z   libasn1-8-heimdal libatomic1 libbabeltrace-ctf1 libbabeltrace1 libbsd-dev
2019-07-15T00:36:01.0220014Z   libbsd0 libc-dev-bin libc6-dev libcc1-0 libcilkrts5 libcurl3 libcurl3-gnutls
2019-07-15T00:36:01.0220674Z   libdpkg-perl libedit-dev libedit2 liberror-perl libexpat1 libffi-dev libffi6
2019-07-15T00:36:01.0221028Z   libgcc-5-dev libgdbm3 libglib2.0-0 libgmp10 libgnutls30 libgomp1
2019-07-15T00:36:01.0221612Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-15T00:36:01.0221965Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-15T00:36:01.0222255Z   libkrb5-3 libkrb5support0 libldap-2.4-2 libllvm6.0 liblsan0 liblzo2-2
2019-07-15T00:36:01.0222532Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-15T00:36:01.0222532Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-15T00:36:01.0222830Z   libperl5.22 libpipeline1 libpython-stdlib libpython2.7-minimal
2019-07-15T00:36:01.0223121Z   libpython2.7-stdlib libpython3.5 libpython3.5-minimal libpython3.5-stdlib
2019-07-15T00:36:01.0223412Z   libquadmath0 libroken18-heimdal librtmp1 libsasl2-2 libsasl2-modules-db
2019-07-15T00:36:01.0223780Z   libsqlite3-0 libssl-dev libssl1.0.0 libstdc++-5-dev libtasn1-6 libtinfo-dev
2019-07-15T00:36:01.0224065Z   libtsan0 libubsan0 libwind0-heimdal libxml2 linux-libc-dev llvm-6.0
2019-07-15T00:36:01.0224348Z   llvm-6.0-dev llvm-6.0-runtime llvm-6.0-tools make mime-support openssl patch
2019-07-15T00:36:01.0224651Z   perl perl-modules-5.22 pkg-config python python-minimal python2.7
2019-07-15T00:36:01.0225070Z   python2.7-minimal sudo xz-utils zlib1g-dev
2019-07-15T00:36:01.0225130Z The following packages will be upgraded:
2019-07-15T00:36:01.4439372Z 1 upgraded, 115 newly installed, 0 to remove and 4 not upgraded.
2019-07-15T00:36:01.4439584Z Need to get 121 MB of archives.
2019-07-15T00:36:01.4439759Z After this operation, 592 MB of additional disk space will be used.
2019-07-15T00:36:01.4441021Z Get:1 http://archive.ubuntu.com/ubuntu xenial/main amd64 libgdbm3 amd64 1.8.3-13.1 [16.9 kB]
---
2019-07-15T00:36:03.8880096Z Get:61 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 sudo amd64 1.8.16-0ubuntu1.7 [390 kB]
2019-07-15T00:36:03.9609175Z Get:62 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 openssl amd64 1.0.2g-1ubuntu4.15 [492 kB]
2019-07-15T00:36:03.9705742Z Get:63 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 ca-certificates all 20170717~16.04.2 [167 kB]
2019-07-15T00:36:03.9745047Z Get:64 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 libcurl3-gnutls amd64 7.47.0-1ubuntu2.13 [184 kB]
2019-07-15T00:36:03.9779473Z Get:65 http://archive.ubuntu.com/ubuntu xenial/main amd64 libedit2 amd64 3.1-20150325-1ubuntu2 [76.5 kB]
2019-07-15T00:36:03.9808570Z Get:66 http://archive.ubuntu.com/ubuntu xenial/main amd64 libpipeline1 amd64 1.4.1-2 [24.6 kB]
2019-07-15T00:36:03.9808890Z Get:67 http://archive.ubuntu.com/ubuntu xenial/main amd64 binfmt-support amd64 2.1.6-1 [50.7 kB]
2019-07-15T00:36:04.1067954Z Get:69 http://archive.ubuntu.com/ubuntu xenial/main amd64 libisl15 amd64 0.16.1-1 [524 kB]
2019-07-15T00:36:04.1140314Z Get:70 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 cpp-5 amd64 5.4.0-6ubuntu1~16.04.11 [7660 kB]
2019-07-15T00:36:04.4833419Z Get:71 http://archive.ubuntu.com/ubuntu xenial/main amd64 cpp amd64 4:5.3.1-1ubuntu1 [27.7 kB]
2019-07-15T00:36:04.4841270Z Get:72 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 curl amd64 7.47.0-1ubuntu2.13 [139 kB]
---
2019-07-15T00:36:26.6105775Z Unpacking ca-certificates (20170717~16.04.2) ...
2019-07-15T00:36:26.8076748Z Selecting previously unselected package libcurl3-gnutls:amd64.
2019-07-15T00:36:26.8094758Z Preparing to unpack .../libcurl3-gnutls_7.47.0-1ubuntu2.13_amd64.deb ...
2019-07-15T00:36:26.8253102Z Unpacking libcurl3-gnutls:amd64 (7.47.0-1ubuntu2.13) ...
2019-07-15T00:36:26.9952744Z Selecting previously unselected package libedit2:amd64.
2019-07-15T00:36:26.9969225Z Preparing to unpack .../libedit2_3.1-20150325-1ubuntu2_amd64.deb ...
2019-07-15T00:36:27.0187927Z Unpacking libedit2:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-15T00:36:27.1604903Z Selecting previously unselected package libpipeline1:amd64.
2019-07-15T00:36:27.1622828Z Preparing to unpack .../libpipeline1_1.4.1-2_amd64.deb ...
2019-07-15T00:36:27.1782996Z Unpacking libpipeline1:amd64 (1.4.1-2) ...
2019-07-15T00:36:27.3115090Z Selecting previously unselected package binfmt-support.
2019-07-15T00:36:27.3134743Z Preparing to unpack .../binfmt-support_2.1.6-1_amd64.deb ...
2019-07-15T00:36:27.3304764Z Unpacking binfmt-support (2.1.6-1) ...
2019-07-15T00:36:27.4743472Z Preparing to unpack .../binutils_2.26.1-1ubuntu1~16.04.8_amd64.deb ...
2019-07-15T00:36:27.4904361Z Unpacking binutils (2.26.1-1ubuntu1~16.04.8) ...
2019-07-15T00:36:28.0282023Z Selecting previously unselected package libisl15:amd64.
2019-07-15T00:36:28.0302082Z Preparing to unpack .../libisl15_0.16.1-1_amd64.deb ...
---
2019-07-15T00:36:41.5977259Z Selecting previously unselected package libtinfo-dev:amd64.
2019-07-15T00:36:41.5995445Z Preparing to unpack .../libtinfo-dev_6.0+20160213-1ubuntu1_amd64.deb ...
2019-07-15T00:36:41.6150075Z Unpacking libtinfo-dev:amd64 (6.0+20160213-1ubuntu1) ...
2019-07-15T00:36:41.7328417Z Selecting previously unselected package libedit-dev:amd64.
2019-07-15T00:36:41.7351598Z Preparing to unpack .../libedit-dev_3.1-20150325-1ubuntu2_amd64.deb ...
2019-07-15T00:36:41.7505233Z Unpacking libedit-dev:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-15T00:36:41.8957330Z Selecting previously unselected package libllvm6.0:amd64.
2019-07-15T00:36:41.8977333Z Preparing to unpack .../libllvm6.0_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-15T00:36:41.9125926Z Unpacking libllvm6.0:amd64 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-15T00:36:44.8553054Z Selecting previously unselected package zlib1g-dev:amd64.
2019-07-15T00:36:44.8583206Z Preparing to unpack .../zlib1g-dev_1%3a1.2.8.dfsg-2ubuntu4.1_amd64.deb ...
2019-07-15T00:36:44.8731727Z Unpacking zlib1g-dev:amd64 (1:1.2.8.dfsg-2ubuntu4.1) ...
2019-07-15T00:36:44.9899631Z Preparing to unpack .../libssl-dev_1.0.2g-1ubuntu4.15_amd64.deb ...
2019-07-15T00:36:45.0049854Z Unpacking libssl-dev:amd64 (1.0.2g-1ubuntu4.15) ...
2019-07-15T00:36:45.3539370Z Selecting previously unselected package llvm-6.0-runtime.
2019-07-15T00:36:45.3539370Z Selecting previously unselected package llvm-6.0-runtime.
2019-07-15T00:36:45.3562078Z Preparing to unpack .../llvm-6.0-runtime_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-15T00:36:45.3734440Z Unpacking llvm-6.0-runtime (1:6.0-1ubuntu2~16.04.1) ...
2019-07-15T00:36:45.5203715Z Selecting previously unselected package llvm-6.0.
2019-07-15T00:36:45.5222766Z Preparing to unpack .../llvm-6.0_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-15T00:36:45.5388539Z Unpacking llvm-6.0 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-15T00:36:46.1943073Z Selecting previously unselected package libffi-dev:amd64.
2019-07-15T00:36:46.1960993Z Preparing to unpack .../libffi-dev_3.2.1-4_amd64.deb ...
2019-07-15T00:36:46.2109878Z Unpacking libffi-dev:amd64 (3.2.1-4) ...
2019-07-15T00:36:46.3694269Z Selecting previously unselected package llvm-6.0-dev.
2019-07-15T00:36:46.3710821Z Preparing to unpack .../llvm-6.0-dev_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-15T00:36:46.3863072Z Unpacking llvm-6.0-dev (1:6.0-1ubuntu2~16.04.1) ...
2019-07-15T00:36:51.1018252Z Selecting previously unselected package llvm-6.0-tools.
2019-07-15T00:36:51.1045679Z Preparing to unpack .../llvm-6.0-tools_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-15T00:36:51.1241441Z Unpacking llvm-6.0-tools (1:6.0-1ubuntu2~16.04.1) ...
2019-07-15T00:36:51.3351903Z Selecting previously unselected package pkg-config.
2019-07-15T00:36:51.3371016Z Preparing to unpack .../pkg-config_0.29.1-0ubuntu1_amd64.deb ...
2019-07-15T00:36:51.3546078Z Unpacking pkg-config (0.29.1-0ubuntu1) ...
2019-07-15T00:36:51.4937414Z Processing triggers for systemd (229-4ubuntu21.21) ...
2019-07-15T00:36:51.9440056Z Setting up libgdbm3:amd64 (1.8.3-13.1) ...
2019-07-15T00:36:52.0307816Z Setting up libffi6:amd64 (3.2.1-4) ...
2019-07-15T00:36:52.0800694Z Setting up libglib2.0-0:amd64 (2.48.2-0ubuntu4.3) ...
---
2019-07-15T00:36:56.4141643Z debconf: unable to initialize frontend: Dialog
2019-07-15T00:36:56.4141805Z debconf: (TERM is not set, so the dialog frontend is not usable.)
2019-07-15T00:36:56.4142049Z debconf: falling back to frontend: Readline
2019-07-15T00:36:57.1574732Z Setting up libcurl3-gnutls:amd64 (7.47.0-1ubuntu2.13) ...
2019-07-15T00:36:57.2069026Z Setting up libedit2:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-15T00:36:57.2603859Z Setting up libpipeline1:amd64 (1.4.1-2) ...
2019-07-15T00:36:57.3088209Z Setting up binfmt-support (2.1.6-1) ...
2019-07-15T00:36:57.3903981Z mount: permission denied
2019-07-15T00:36:57.3907409Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-15T00:36:57.3923924Z mount: permission denied
2019-07-15T00:36:57.3932582Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-15T00:36:57.5548861Z invoke-rc.d: could not determine current runlevel
2019-07-15T00:36:57.5588017Z invoke-rc.d: policy-rc.d denied execution of start.
2019-07-15T00:36:57.6357062Z Setting up libisl15:amd64 (0.16.1-1) ...
2019-07-15T00:36:57.6818307Z Setting up cpp-5 (5.4.0-6ubuntu1~16.04.11) ...
2019-07-15T00:36:57.7292408Z Setting up cpp (4:5.3.1-1ubuntu1) ...
2019-07-15T00:36:57.7938658Z Setting up curl (7.47.0-1ubuntu2.13) ...
---
2019-07-15T00:36:59.7761934Z Setting up libedit-dev:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-15T00:36:59.9445842Z Setting up libllvm6.0:amd64 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-15T00:37:00.2085112Z Setting up zlib1g-dev:amd64 (1:1.2.8.dfsg-2ubuntu4.1) ...
2019-07-15T00:37:00.4459771Z Setting up libssl-dev:amd64 (1.0.2g-1ubuntu4.15) ...
2019-07-15T00:37:00.6968012Z Setting up llvm-6.0-runtime (1:6.0-1ubuntu2~16.04.1) ...
2019-07-15T00:37:00.8187451Z mount: permission denied
2019-07-15T00:37:00.8193557Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-15T00:37:00.8957064Z Setting up llvm-6.0 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-15T00:37:01.1666892Z Setting up libffi-dev:amd64 (3.2.1-4) ...
2019-07-15T00:37:01.4012094Z Setting up llvm-6.0-dev (1:6.0-1ubuntu2~16.04.1) ...
2019-07-15T00:37:01.6122454Z Setting up llvm-6.0-tools (1:6.0-1ubuntu2~16.04.1) ...
2019-07-15T00:37:01.6605988Z Setting up pkg-config (0.29.1-0ubuntu1) ...
2019-07-15T00:37:01.8300439Z Processing triggers for ca-certificates (20170717~16.04.2) ...
2019-07-15T00:37:01.8500007Z Updating certificates in /etc/ssl/certs...
2019-07-15T00:37:03.4273298Z 148 added, 0 removed; done.
2019-07-15T00:37:03.4274165Z Running hooks in /etc/ca-certificates/update.d...
---
2019-07-15T00:37:38.5197912Z Removing intermediate container cab539a191ad
2019-07-15T00:37:38.5198805Z  ---> fd0ef4b7f87d
2019-07-15T00:37:38.5237709Z Successfully built fd0ef4b7f87d
2019-07-15T00:37:38.6647576Z Successfully tagged rust-ci:latest
2019-07-15T00:37:38.7294877Z Built container sha256:fd0ef4b7f87dbe1020468c411ec147c120ac33fa9d9bef70cd3a7da5d403e681
2019-07-15T00:37:38.7315174Z Uploading finished image to https://.s3.amazonaws.com/docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624
2019-07-15T00:38:41.6959364Z upload failed: - to s3:///docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624 Parameter validation failed:
2019-07-15T00:38:41.6959971Z Invalid bucket name "": Bucket name must match the regex "^[a-zA-Z0-9.\-_]{1,255}$"
2019-07-15T00:38:43.1200103Z [CI_JOB_NAME=x86_64-gnu-llvm-6.0]
2019-07-15T00:38:43.1258159Z Starting sccache server...
2019-07-15T00:38:43.1768644Z configure: processing command line
2019-07-15T00:38:43.1769254Z configure: 
---
2019-07-15T00:42:14.8907599Z    Compiling serde_json v1.0.33
2019-07-15T00:42:19.1771916Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-07-15T00:42:27.9084007Z     Finished release [optimized] target(s) in 1m 30s
2019-07-15T00:42:27.9157112Z tidy check
2019-07-15T00:42:28.4999826Z tidy error: /checkout/src/test/run-pass/issue-58375-monomorphize-default-impls.rs: missing trailing newline
2019-07-15T00:42:29.7825134Z some tidy checks failed
2019-07-15T00:42:29.7831485Z 
2019-07-15T00:42:29.7831485Z 
2019-07-15T00:42:29.7832841Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-07-15T00:42:29.7833388Z 
2019-07-15T00:42:29.7833412Z 
2019-07-15T00:42:29.7846556Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-07-15T00:42:29.7846660Z Build completed unsuccessfully in 0:01:34
2019-07-15T00:42:29.7846660Z Build completed unsuccessfully in 0:01:34
2019-07-15T00:42:31.1665271Z ##[error]Bash exited with code '1'.
2019-07-15T00:42:31.1709947Z ##[section]Starting: Checkout
2019-07-15T00:42:31.1711799Z ==============================================================================
2019-07-15T00:42:31.1711861Z Task         : Get sources
2019-07-15T00:42:31.1711936Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
