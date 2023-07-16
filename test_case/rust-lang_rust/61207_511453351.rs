plain
2019-07-15T14:29:37.6769154Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-15T14:29:37.6963640Z ##[command]git config gc.auto 0
2019-07-15T14:29:37.7065509Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-15T14:29:37.7135424Z ##[command]git config --get-all http.proxy
2019-07-15T14:29:37.7274839Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/61207/merge:refs/remotes/pull/61207/merge
---
2019-07-15T14:30:12.6182593Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-15T14:30:12.6184395Z 
2019-07-15T14:30:12.6186194Z   git checkout -b <new-branch-name>
2019-07-15T14:30:12.6187599Z 
2019-07-15T14:30:12.6188711Z HEAD is now at 49345a78f Merge b96e048eb3ec15a3125c0a5ee88dd3640c824165 into 5480b47d7f9e708300d3ba319869f21cd1ffd487
2019-07-15T14:30:12.6316963Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-15T14:30:12.6319741Z ==============================================================================
2019-07-15T14:30:12.6319821Z Task         : Bash
2019-07-15T14:30:12.6319867Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-15T14:32:05.8524002Z Attempting with retry: curl -y 30 -Y 10 --connect-timeout 30 -f -L -C - -o /tmp/rustci_docker_cache https://.s3.amazonaws.com/docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624
2019-07-15T14:32:05.8618117Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-15T14:32:05.8618358Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-15T14:32:05.8618691Z 
2019-07-15T14:32:05.8661752Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-15T14:32:06.8782335Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-15T14:32:06.8782861Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-15T14:32:06.8783198Z 
2019-07-15T14:32:06.8783198Z 
2019-07-15T14:32:06.8784189Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-15T14:32:08.8859712Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-15T14:32:08.8859915Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-15T14:32:08.8859991Z 
2019-07-15T14:32:08.8859991Z 
2019-07-15T14:32:08.8903261Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-15T14:32:11.8977240Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-15T14:32:11.8977387Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-15T14:32:11.8977424Z 
2019-07-15T14:32:11.8977424Z 
2019-07-15T14:32:11.9020617Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-15T14:32:15.9096177Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-15T14:32:15.9096625Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-15T14:32:15.9096949Z 
2019-07-15T14:32:15.9096949Z 
2019-07-15T14:32:15.9152517Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-15T14:32:15.9152988Z The command has failed after 5 attempts.
2019-07-15T14:32:16.0606115Z open /tmp/rustci_docker_cache: no such file or directory
2019-07-15T14:32:16.0643671Z Attempting with retry: docker build --rm -t rust-ci -f /home/vsts/work/1/s/src/ci/docker/x86_64-gnu-llvm-6.0/Dockerfile /home/vsts/work/1/s/src/ci/docker
2019-07-15T14:32:16.2525806Z Sending build context to Docker daemon  521.7kB
2019-07-15T14:32:16.2526557Z 
2019-07-15T14:32:16.2787478Z Step 1/8 : FROM ubuntu:16.04
---
2019-07-15T14:32:32.8206928Z Reading package lists...
2019-07-15T14:32:33.8403227Z Reading package lists...
2019-07-15T14:32:34.0209241Z Building dependency tree...
2019-07-15T14:32:34.0210039Z Reading state information...
2019-07-15T14:32:34.1454227Z The following additional packages will be installed:
2019-07-15T14:32:34.1455931Z   binfmt-support binutils bzip2 cmake-data cpp cpp-5 dpkg-dev g++-5 gcc gcc-5
2019-07-15T14:32:34.1456437Z   git-man libarchive13 libasan2 libasn1-8-heimdal libatomic1
2019-07-15T14:32:34.1457041Z   libbabeltrace-ctf1 libbabeltrace1 libbsd-dev libbsd0 libbz2-1.0 libc-dev-bin
2019-07-15T14:32:34.1458063Z   libedit2 liberror-perl libexpat1 libffi-dev libffi6 libgcc-5-dev libgdbm3
2019-07-15T14:32:34.1458510Z   libglib2.0-0 libgmp10 libgnutls30 libgomp1 libgssapi-krb5-2
2019-07-15T14:32:34.1458950Z   libgssapi3-heimdal libhcrypto4-heimdal libheimbase1-heimdal
2019-07-15T14:32:34.1459902Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-15T14:32:34.1459902Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-15T14:32:34.1460279Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-15T14:32:34.1460548Z   libkrb5-3 libkrb5support0 libldap-2.4-2 libllvm6.0 liblsan0 liblzo2-2
2019-07-15T14:32:34.1460833Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-15T14:32:34.1461089Z   libperl5.22 libpipeline1 libpython-stdlib libpython2.7-minimal
2019-07-15T14:32:34.1461716Z   libpython2.7-stdlib libpython3.5 libpython3.5-minimal libpython3.5-stdlib
2019-07-15T14:32:34.1462002Z   libquadmath0 libroken18-heimdal librtmp1 libsasl2-2 libsasl2-modules-db
2019-07-15T14:32:34.1462541Z   libsqlite3-0 libssl1.0.0 libstdc++-5-dev libtasn1-6 libtinfo-dev libtsan0
2019-07-15T14:32:34.1472499Z   libubsan0 libwind0-heimdal libxml2 linux-libc-dev llvm-6.0 llvm-6.0-dev
2019-07-15T14:32:34.1473521Z   llvm-6.0-runtime mime-support openssl patch perl perl-modules-5.22 python
2019-07-15T14:32:34.1474063Z   python-minimal python2.7-minimal
2019-07-15T14:32:34.1474377Z Suggested packages:
2019-07-15T14:32:34.1474843Z   binutils-doc bzip2-doc codeblocks eclipse ninja-build cpp-doc gcc-5-locales
2019-07-15T14:32:34.1475286Z   debian-keyring g++-multilib g++-5-multilib gcc-5-doc libstdc++6-5-dbg
2019-07-15T14:32:34.1475727Z   gcc-multilib manpages-dev autoconf automake libtool flex bison gcc-doc
2019-07-15T14:32:34.1476602Z   libasan2-dbg liblsan0-dbg libtsan0-dbg libubsan0-dbg libcilkrts5-dbg
2019-07-15T14:32:34.1477043Z   libmpx0-dbg libquadmath0-dbg gdb-doc gettext-base git-daemon-run
2019-07-15T14:32:34.1477043Z   libmpx0-dbg libquadmath0-dbg gdb-doc gettext-base git-daemon-run
2019-07-15T14:32:34.1478266Z   | git-daemon-sysvinit git-doc git-el git-email git-gui gitk gitweb git-arch
2019-07-15T14:32:34.1478795Z   git-cvs git-mediawiki git-svn lrzip glibc-doc gnutls-bin krb5-doc krb5-user
2019-07-15T14:32:34.1479260Z   libstdc++-5-doc llvm-6.0-doc make-doc ed diffutils-doc perl-doc
2019-07-15T14:32:34.1479703Z   libterm-readline-gnu-perl | libterm-readline-perl-perl python-doc python-tk
2019-07-15T14:32:34.1480105Z   python2.7-doc
2019-07-15T14:32:34.1480291Z Recommended packages:
2019-07-15T14:32:34.1480684Z   build-essential fakeroot libalgorithm-merge-perl libc-dbg gdbserver less
2019-07-15T14:32:34.1481120Z   rsync ssh-client manpages manpages-dev libfile-fcntllock-perl
2019-07-15T14:32:34.1481556Z   libglib2.0-data shared-mime-info xdg-user-dirs krb5-locales libsasl2-modules
2019-07-15T14:32:34.1481964Z   libssl-doc xml-core netbase rename
2019-07-15T14:32:34.1482177Z The following NEW packages will be installed:
2019-07-15T14:32:34.1482625Z   binfmt-support binutils bzip2 ca-certificates cmake cmake-data cpp cpp-5
2019-07-15T14:32:34.1483092Z   curl dpkg-dev file g++ g++-5 gcc gcc-5 gdb git git-man libarchive13 libasan2
2019-07-15T14:32:34.1484139Z   libasn1-8-heimdal libatomic1 libbabeltrace-ctf1 libbabeltrace1 libbsd-dev
2019-07-15T14:32:34.1484806Z   libbsd0 libc-dev-bin libc6-dev libcc1-0 libcilkrts5 libcurl3 libcurl3-gnutls
2019-07-15T14:32:34.1485341Z   libdpkg-perl libedit-dev libedit2 liberror-perl libexpat1 libffi-dev libffi6
2019-07-15T14:32:34.1485787Z   libgcc-5-dev libgdbm3 libglib2.0-0 libgmp10 libgnutls30 libgomp1
2019-07-15T14:32:34.1486679Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-15T14:32:34.1487139Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-15T14:32:34.1487593Z   libkrb5-3 libkrb5support0 libldap-2.4-2 libllvm6.0 liblsan0 liblzo2-2
2019-07-15T14:32:34.1488026Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-15T14:32:34.1488026Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-15T14:32:34.1488455Z   libperl5.22 libpipeline1 libpython-stdlib libpython2.7-minimal
2019-07-15T14:32:34.1488944Z   libpython2.7-stdlib libpython3.5 libpython3.5-minimal libpython3.5-stdlib
2019-07-15T14:32:34.1489386Z   libquadmath0 libroken18-heimdal librtmp1 libsasl2-2 libsasl2-modules-db
2019-07-15T14:32:34.1489829Z   libsqlite3-0 libssl-dev libssl1.0.0 libstdc++-5-dev libtasn1-6 libtinfo-dev
2019-07-15T14:32:34.1490280Z   libtsan0 libubsan0 libwind0-heimdal libxml2 linux-libc-dev llvm-6.0
2019-07-15T14:32:34.1490713Z   llvm-6.0-dev llvm-6.0-runtime llvm-6.0-tools make mime-support openssl patch
2019-07-15T14:32:34.1491161Z   perl perl-modules-5.22 pkg-config python python-minimal python2.7
2019-07-15T14:32:34.1491742Z   python2.7-minimal sudo xz-utils zlib1g-dev
2019-07-15T14:32:34.1491939Z The following packages will be upgraded:
2019-07-15T14:32:34.4152323Z 1 upgraded, 115 newly installed, 0 to remove and 4 not upgraded.
2019-07-15T14:32:34.4152526Z Need to get 121 MB of archives.
2019-07-15T14:32:34.4152822Z After this operation, 592 MB of additional disk space will be used.
2019-07-15T14:32:34.4153946Z Get:1 http://archive.ubuntu.com/ubuntu xenial/main amd64 libgdbm3 amd64 1.8.3-13.1 [16.9 kB]
---
2019-07-15T14:32:35.5061984Z Get:61 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 sudo amd64 1.8.16-0ubuntu1.7 [390 kB]
2019-07-15T14:32:35.5140039Z Get:62 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 openssl amd64 1.0.2g-1ubuntu4.15 [492 kB]
2019-07-15T14:32:35.5205489Z Get:63 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 ca-certificates all 20170717~16.04.2 [167 kB]
2019-07-15T14:32:35.5231402Z Get:64 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 libcurl3-gnutls amd64 7.47.0-1ubuntu2.13 [184 kB]
2019-07-15T14:32:35.5261339Z Get:65 http://archive.ubuntu.com/ubuntu xenial/main amd64 libedit2 amd64 3.1-20150325-1ubuntu2 [76.5 kB]
2019-07-15T14:32:35.5283050Z Get:66 http://archive.ubuntu.com/ubuntu xenial/main amd64 libpipeline1 amd64 1.4.1-2 [24.6 kB]
2019-07-15T14:32:35.5284150Z Get:67 http://archive.ubuntu.com/ubuntu xenial/main amd64 binfmt-support amd64 2.1.6-1 [50.7 kB]
2019-07-15T14:32:35.5821623Z Get:69 http://archive.ubuntu.com/ubuntu xenial/main amd64 libisl15 amd64 0.16.1-1 [524 kB]
2019-07-15T14:32:35.5920144Z Get:70 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 cpp-5 amd64 5.4.0-6ubuntu1~16.04.11 [7660 kB]
2019-07-15T14:32:35.7205753Z Get:71 http://archive.ubuntu.com/ubuntu xenial/main amd64 cpp amd64 4:5.3.1-1ubuntu1 [27.7 kB]
2019-07-15T14:32:35.7206213Z Get:72 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 curl amd64 7.47.0-1ubuntu2.13 [139 kB]
---
2019-07-15T14:32:52.7904865Z Unpacking ca-certificates (20170717~16.04.2) ...
2019-07-15T14:32:52.9375074Z Selecting previously unselected package libcurl3-gnutls:amd64.
2019-07-15T14:32:52.9391131Z Preparing to unpack .../libcurl3-gnutls_7.47.0-1ubuntu2.13_amd64.deb ...
2019-07-15T14:32:52.9521860Z Unpacking libcurl3-gnutls:amd64 (7.47.0-1ubuntu2.13) ...
2019-07-15T14:32:53.0704836Z Selecting previously unselected package libedit2:amd64.
2019-07-15T14:32:53.0720991Z Preparing to unpack .../libedit2_3.1-20150325-1ubuntu2_amd64.deb ...
2019-07-15T14:32:53.0871462Z Unpacking libedit2:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-15T14:32:53.1984818Z Selecting previously unselected package libpipeline1:amd64.
2019-07-15T14:32:53.2003347Z Preparing to unpack .../libpipeline1_1.4.1-2_amd64.deb ...
2019-07-15T14:32:53.2135010Z Unpacking libpipeline1:amd64 (1.4.1-2) ...
2019-07-15T14:32:53.3148186Z Selecting previously unselected package binfmt-support.
2019-07-15T14:32:53.3160564Z Preparing to unpack .../binfmt-support_2.1.6-1_amd64.deb ...
2019-07-15T14:32:53.3282521Z Unpacking binfmt-support (2.1.6-1) ...
2019-07-15T14:32:53.4459740Z Preparing to unpack .../binutils_2.26.1-1ubuntu1~16.04.8_amd64.deb ...
2019-07-15T14:32:53.4579858Z Unpacking binutils (2.26.1-1ubuntu1~16.04.8) ...
2019-07-15T14:32:53.9477840Z Selecting previously unselected package libisl15:amd64.
2019-07-15T14:32:53.9498032Z Preparing to unpack .../libisl15_0.16.1-1_amd64.deb ...
---
2019-07-15T14:33:05.3628607Z Selecting previously unselected package libtinfo-dev:amd64.
2019-07-15T14:33:05.3644706Z Preparing to unpack .../libtinfo-dev_6.0+20160213-1ubuntu1_amd64.deb ...
2019-07-15T14:33:05.3754867Z Unpacking libtinfo-dev:amd64 (6.0+20160213-1ubuntu1) ...
2019-07-15T14:33:05.4700742Z Selecting previously unselected package libedit-dev:amd64.
2019-07-15T14:33:05.4723340Z Preparing to unpack .../libedit-dev_3.1-20150325-1ubuntu2_amd64.deb ...
2019-07-15T14:33:05.4868445Z Unpacking libedit-dev:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-15T14:33:05.5977388Z Selecting previously unselected package libllvm6.0:amd64.
2019-07-15T14:33:05.5999820Z Preparing to unpack .../libllvm6.0_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-15T14:33:05.6120850Z Unpacking libllvm6.0:amd64 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-15T14:33:08.5875892Z Selecting previously unselected package zlib1g-dev:amd64.
2019-07-15T14:33:08.5894015Z Preparing to unpack .../zlib1g-dev_1%3a1.2.8.dfsg-2ubuntu4.1_amd64.deb ...
2019-07-15T14:33:08.6011729Z Unpacking zlib1g-dev:amd64 (1:1.2.8.dfsg-2ubuntu4.1) ...
2019-07-15T14:33:08.6908099Z Preparing to unpack .../libssl-dev_1.0.2g-1ubuntu4.15_amd64.deb ...
2019-07-15T14:33:08.7059690Z Unpacking libssl-dev:amd64 (1.0.2g-1ubuntu4.15) ...
2019-07-15T14:33:08.9938053Z Selecting previously unselected package llvm-6.0-runtime.
2019-07-15T14:33:08.9938053Z Selecting previously unselected package llvm-6.0-runtime.
2019-07-15T14:33:08.9957571Z Preparing to unpack .../llvm-6.0-runtime_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-15T14:33:09.0078998Z Unpacking llvm-6.0-runtime (1:6.0-1ubuntu2~16.04.1) ...
2019-07-15T14:33:09.1112891Z Selecting previously unselected package llvm-6.0.
2019-07-15T14:33:09.1131204Z Preparing to unpack .../llvm-6.0_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-15T14:33:09.1243971Z Unpacking llvm-6.0 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-15T14:33:09.7692042Z Selecting previously unselected package libffi-dev:amd64.
2019-07-15T14:33:09.7714089Z Preparing to unpack .../libffi-dev_3.2.1-4_amd64.deb ...
2019-07-15T14:33:09.7828968Z Unpacking libffi-dev:amd64 (3.2.1-4) ...
2019-07-15T14:33:09.8907257Z Selecting previously unselected package llvm-6.0-dev.
2019-07-15T14:33:09.8928645Z Preparing to unpack .../llvm-6.0-dev_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-15T14:33:09.9043748Z Unpacking llvm-6.0-dev (1:6.0-1ubuntu2~16.04.1) ...
2019-07-15T14:33:15.1958015Z Selecting previously unselected package llvm-6.0-tools.
2019-07-15T14:33:15.1958453Z Preparing to unpack .../llvm-6.0-tools_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-15T14:33:15.1959147Z Unpacking llvm-6.0-tools (1:6.0-1ubuntu2~16.04.1) ...
2019-07-15T14:33:15.1959481Z Selecting previously unselected package pkg-config.
2019-07-15T14:33:15.1959738Z Preparing to unpack .../pkg-config_0.29.1-0ubuntu1_amd64.deb ...
2019-07-15T14:33:15.1959965Z Unpacking pkg-config (0.29.1-0ubuntu1) ...
2019-07-15T14:33:15.1960461Z Processing triggers for systemd (229-4ubuntu21.21) ...
2019-07-15T14:33:15.3192716Z Setting up libgdbm3:amd64 (1.8.3-13.1) ...
2019-07-15T14:33:15.3900740Z Setting up libffi6:amd64 (3.2.1-4) ...
2019-07-15T14:33:15.4245689Z Setting up libglib2.0-0:amd64 (2.48.2-0ubuntu4.3) ...
---
2019-07-15T14:33:18.8652764Z debconf: unable to initialize frontend: Dialog
2019-07-15T14:33:18.8652951Z debconf: (TERM is not set, so the dialog frontend is not usable.)
2019-07-15T14:33:18.8653047Z debconf: falling back to frontend: Readline
2019-07-15T14:33:19.4224235Z Setting up libcurl3-gnutls:amd64 (7.47.0-1ubuntu2.13) ...
2019-07-15T14:33:19.4565730Z Setting up libedit2:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-15T14:33:19.4953446Z Setting up libpipeline1:amd64 (1.4.1-2) ...
2019-07-15T14:33:19.5307576Z Setting up binfmt-support (2.1.6-1) ...
2019-07-15T14:33:19.5961263Z mount: permission denied
2019-07-15T14:33:19.5962168Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-15T14:33:19.5971037Z mount: permission denied
2019-07-15T14:33:19.5976797Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-15T14:33:19.7681355Z invoke-rc.d: could not determine current runlevel
2019-07-15T14:33:19.7712667Z invoke-rc.d: policy-rc.d denied execution of start.
2019-07-15T14:33:19.8223462Z Setting up libisl15:amd64 (0.16.1-1) ...
2019-07-15T14:33:19.8626528Z Setting up cpp-5 (5.4.0-6ubuntu1~16.04.11) ...
2019-07-15T14:33:19.8996158Z Setting up cpp (4:5.3.1-1ubuntu1) ...
2019-07-15T14:33:19.9455008Z Setting up curl (7.47.0-1ubuntu2.13) ...
---
2019-07-15T14:33:21.4463336Z Setting up libedit-dev:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-15T14:33:21.4852199Z Setting up libllvm6.0:amd64 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-15T14:33:21.5264791Z Setting up zlib1g-dev:amd64 (1:1.2.8.dfsg-2ubuntu4.1) ...
2019-07-15T14:33:21.5627071Z Setting up libssl-dev:amd64 (1.0.2g-1ubuntu4.15) ...
2019-07-15T14:33:21.6163619Z Setting up llvm-6.0-runtime (1:6.0-1ubuntu2~16.04.1) ...
2019-07-15T14:33:21.6438376Z mount: permission denied
2019-07-15T14:33:21.6439548Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-15T14:33:21.6566557Z Setting up llvm-6.0 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-15T14:33:21.6989732Z Setting up libffi-dev:amd64 (3.2.1-4) ...
2019-07-15T14:33:21.7342062Z Setting up llvm-6.0-dev (1:6.0-1ubuntu2~16.04.1) ...
2019-07-15T14:33:21.7704875Z Setting up llvm-6.0-tools (1:6.0-1ubuntu2~16.04.1) ...
2019-07-15T14:33:21.8064136Z Setting up pkg-config (0.29.1-0ubuntu1) ...
2019-07-15T14:33:21.9300781Z Processing triggers for ca-certificates (20170717~16.04.2) ...
2019-07-15T14:33:21.9468630Z Updating certificates in /etc/ssl/certs...
2019-07-15T14:33:23.6322408Z 148 added, 0 removed; done.
2019-07-15T14:33:23.6323362Z Running hooks in /etc/ca-certificates/update.d...
---
2019-07-15T14:33:55.8094164Z  ---> 349b4fab66ea
2019-07-15T14:33:55.8139940Z Successfully built 349b4fab66ea
2019-07-15T14:33:55.8950387Z Successfully tagged rust-ci:latest
2019-07-15T14:33:55.9647675Z Built container sha256:349b4fab66eac73714850a389ccc919a9f8ebe6553c85016eafcaf765e62702b
2019-07-15T14:33:55.9662750Z Uploading finished image to https://.s3.amazonaws.com/docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624
2019-07-15T14:34:58.1120593Z upload failed: - to s3:///docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624 Parameter validation failed:
2019-07-15T14:34:58.1121648Z Invalid bucket name "": Bucket name must match the regex "^[a-zA-Z0-9.\-_]{1,255}$"
2019-07-15T14:34:59.0000319Z [CI_JOB_NAME=x86_64-gnu-llvm-6.0]
2019-07-15T14:34:59.0050324Z Starting sccache server...
2019-07-15T14:34:59.0559359Z configure: processing command line
2019-07-15T14:34:59.0560263Z configure: 
---
2019-07-15T15:31:57.7020122Z .................................................................................................... 200/5823
2019-07-15T15:32:02.1585349Z .................................................................................................... 300/5823
2019-07-15T15:32:06.0816547Z .................................................................................................... 400/5823
2019-07-15T15:32:10.0807161Z .................................................................................................... 500/5823
2019-07-15T15:32:14.1047817Z ........................................................................i........................... 600/5823
2019-07-15T15:32:23.5036101Z .................................................................................................... 800/5823
2019-07-15T15:32:29.4680018Z .................................................................................................... 900/5823
2019-07-15T15:32:29.4680018Z .................................................................................................... 900/5823
2019-07-15T15:32:35.6147162Z ............................................................................................i....... 1000/5823
2019-07-15T15:32:40.2325579Z ....i............................................................................................... 1100/5823
2019-07-15T15:32:44.6733002Z ......................iiiii......................................................................... 1200/5823
2019-07-15T15:32:50.6156378Z .................................................................................................... 1400/5823
2019-07-15T15:32:53.5238663Z .................................................................................................... 1500/5823
2019-07-15T15:32:57.4389960Z .................................................................................................... 1600/5823
2019-07-15T15:33:00.0519919Z .................................................................................................... 1700/5823
2019-07-15T15:33:00.0519919Z .................................................................................................... 1700/5823
2019-07-15T15:33:03.7246628Z ...........................................................i........................................ 1800/5823
2019-07-15T15:33:12.6421714Z .................................................................................................... 2000/5823
2019-07-15T15:33:16.8033618Z .................................................................................................... 2100/5823
2019-07-15T15:33:21.0688841Z .................................................................................................... 2200/5823
2019-07-15T15:33:21.0688841Z .................................................................................................... 2200/5823
2019-07-15T15:33:24.8001754Z .........................i.......................................................................... 2300/5823
2019-07-15T15:33:35.1677766Z .................................................................................................... 2500/5823
2019-07-15T15:33:40.1077795Z .................................................................................................... 2600/5823
2019-07-15T15:33:44.6932480Z .................................................................................................... 2700/5823
2019-07-15T15:33:49.2687905Z .................................................................................................... 2800/5823
2019-07-15T15:33:49.2687905Z .................................................................................................... 2800/5823
2019-07-15T15:33:53.4430015Z .................................................................................................... 2900/5823
2019-07-15T15:33:59.1752342Z .................................................................................................... 3000/5823
2019-07-15T15:34:04.0756464Z .................................................................................................... 3100/5823
2019-07-15T15:34:08.8334907Z .................................................................................................... 3200/5823
2019-07-15T15:34:12.0428683Z .................................................................................................... 3300/5823
2019-07-15T15:34:17.2069899Z .................................................................................................... 3400/5823
2019-07-15T15:34:21.5361537Z .......................................................................................i............ 3500/5823
2019-07-15T15:34:25.4971397Z .................................................................................................... 3600/5823
2019-07-15T15:34:29.6258820Z .............................................................ii...i..ii............................. 3700/5823
2019-07-15T15:34:39.1692849Z .................................................................................................... 3900/5823
2019-07-15T15:34:39.1692849Z .................................................................................................... 3900/5823
2019-07-15T15:34:43.3110626Z ...........................................................................ii....................... 4000/5823
2019-07-15T15:34:46.2416535Z ................................................................................................i... 4100/5823
2019-07-15T15:34:48.5514167Z .................................................................................................... 4200/5823
2019-07-15T15:34:50.7118018Z .............................................................i...................................... 4300/5823
2019-07-15T15:35:04.8503839Z .................................................................................................... 4500/5823
2019-07-15T15:35:16.6007105Z .................................................................................................... 4600/5823
2019-07-15T15:35:20.5100402Z .................................................................................................... 4700/5823
2019-07-15T15:35:24.1686249Z .................................................................................................... 4800/5823
2019-07-15T15:35:24.1686249Z .................................................................................................... 4800/5823
2019-07-15T15:35:31.1066163Z .................................................................................................... 4900/5823
2019-07-15T15:35:38.4528683Z .................................................................................................... 5000/5823
2019-07-15T15:35:45.4835154Z .....................FFFF........................................................................... 5100/5823
2019-07-15T15:35:54.0584907Z .................................................................................................... 5300/5823
2019-07-15T15:35:59.2325232Z .................................................................................................... 5400/5823
2019-07-15T15:36:03.4339259Z .................................................................................................... 5500/5823
2019-07-15T15:36:07.4528839Z .................................................................................................... 5600/5823
2019-07-15T15:36:07.4528839Z .................................................................................................... 5600/5823
2019-07-15T15:36:10.8427658Z .................................................................................................... 5700/5823
2019-07-15T15:36:13.9731833Z ...............................................................i.................................... 5800/5823
2019-07-15T15:36:14.9725621Z failures:
2019-07-15T15:36:14.9776608Z 
2019-07-15T15:36:14.9776608Z 
2019-07-15T15:36:14.9777406Z ---- [ui] ui/self/elision/ref-mut-self.rs stdout ----
2019-07-15T15:36:14.9778138Z 
2019-07-15T15:36:14.9778138Z 
2019-07-15T15:36:14.9778350Z 1 error[E0623]: lifetime mismatch
2019-07-15T15:36:14.9779012Z -   --> $DIR/ref-mut-self.rs:14:9
2019-07-15T15:36:14.9779565Z +   --> $DIR/ref-mut-self.rs:12:9
2019-07-15T15:36:14.9780000Z 3    |
2019-07-15T15:36:14.9780478Z 4 LL |     fn ref_self(&mut self, f: &u32) -> &u32 {
2019-07-15T15:36:14.9780988Z 5    |                               ----     ----
2019-07-15T15:36:14.9781243Z 
2019-07-15T15:36:14.9781480Z 9    |         ^ ...but data from `f` is returned here
2019-07-15T15:36:14.9781688Z 10 
2019-07-15T15:36:14.9781895Z 11 error[E0623]: lifetime mismatch
2019-07-15T15:36:14.9782349Z -   --> $DIR/ref-mut-self.rs:20:9
2019-07-15T15:36:14.9782860Z +   --> $DIR/ref-mut-self.rs:18:9
2019-07-15T15:36:14.9783138Z 13    |
2019-07-15T15:36:14.9783941Z 14 LL |     fn ref_Self(self: &mut Self, f: &u32) -> &u32 {
2019-07-15T15:36:14.9784762Z 15    |                                     ----     ----
2019-07-15T15:36:14.9785091Z 
2019-07-15T15:36:14.9785344Z 19    |         ^ ...but data from `f` is returned here
2019-07-15T15:36:14.9785561Z 20 
2019-07-15T15:36:14.9785774Z 21 error[E0623]: lifetime mismatch
2019-07-15T15:36:14.9786272Z -   --> $DIR/ref-mut-self.rs:24:9
2019-07-15T15:36:14.9786814Z +   --> $DIR/ref-mut-self.rs:22:9
2019-07-15T15:36:14.9787100Z 23    |
2019-07-15T15:36:14.9787608Z 24 LL |     fn box_ref_Self(self: Box<&mut Self>, f: &u32) -> &u32 {
2019-07-15T15:36:14.9788143Z 25    |                                              ----     ----
2019-07-15T15:36:14.9788403Z 
2019-07-15T15:36:14.9788649Z 29    |         ^ ...but data from `f` is returned here
2019-07-15T15:36:14.9788858Z 30 
2019-07-15T15:36:14.9789263Z 31 error[E0623]: lifetime mismatch
2019-07-15T15:36:14.9789986Z -   --> $DIR/ref-mut-self.rs:28:9
2019-07-15T15:36:14.9790483Z +   --> $DIR/ref-mut-self.rs:26:9
2019-07-15T15:36:14.9790756Z 33    |
2019-07-15T15:36:14.9791247Z 34 LL |     fn pin_ref_Self(self: Pin<&mut Self>, f: &u32) -> &u32 {
2019-07-15T15:36:14.9791764Z 35    |                                              ----     ----
2019-07-15T15:36:14.9792014Z 
2019-07-15T15:36:14.9792252Z 39    |         ^ ...but data from `f` is returned here
2019-07-15T15:36:14.9792470Z 40 
2019-07-15T15:36:14.9792674Z 41 error[E0623]: lifetime mismatch
2019-07-15T15:36:14.9793137Z -   --> $DIR/ref-mut-self.rs:32:9
2019-07-15T15:36:14.9793626Z +   --> $DIR/ref-mut-self.rs:30:9
2019-07-15T15:36:14.9793925Z 43    |
2019-07-15T15:36:14.9794840Z 44 LL |     fn box_box_ref_Self(self: Box<Box<&mut Self>>, f: &u32) -> &u32 {
2019-07-15T15:36:14.9795468Z 45    |                                                       ----     ----
2019-07-15T15:36:14.9795757Z 
2019-07-15T15:36:14.9795997Z 49    |         ^ ...but data from `f` is returned here
2019-07-15T15:36:14.9796213Z 50 
2019-07-15T15:36:14.9796443Z 51 error[E0623]: lifetime mismatch
2019-07-15T15:36:14.9796898Z -   --> $DIR/ref-mut-self.rs:36:9
2019-07-15T15:36:14.9797399Z +   --> $DIR/ref-mut-self.rs:34:9
2019-07-15T15:36:14.9797699Z 53    |
2019-07-15T15:36:14.9798194Z 54 LL |     fn box_pin_ref_Self(self: Box<Pin<&mut Self>>, f: &u32) -> &u32 {
2019-07-15T15:36:14.9798735Z 55    |                                                       ----     ----
2019-07-15T15:36:14.9799552Z 
2019-07-15T15:36:14.9799760Z The actual stderr differed from the expected stderr.
2019-07-15T15:36:14.9799760Z The actual stderr differed from the expected stderr.
2019-07-15T15:36:14.9800488Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-mut-self/ref-mut-self.stderr
2019-07-15T15:36:14.9801828Z To update references, rerun the tests and pass the `--bless` flag
2019-07-15T15:36:14.9802429Z To only update this specific test, also pass `--test-args self/elision/ref-mut-self.rs`
2019-07-15T15:36:14.9802801Z error: 1 errors occurred comparing output.
2019-07-15T15:36:14.9802936Z status: exit code: 1
2019-07-15T15:36:14.9802936Z status: exit code: 1
2019-07-15T15:36:14.9803842Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/elision/ref-mut-self.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-mut-self" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-mut-self/auxiliary" "-A" "unused"
2019-07-15T15:36:14.9804663Z ------------------------------------------
2019-07-15T15:36:14.9804829Z 
2019-07-15T15:36:14.9805210Z ------------------------------------------
2019-07-15T15:36:14.9805392Z stderr:
2019-07-15T15:36:14.9805392Z stderr:
2019-07-15T15:36:14.9805755Z ------------------------------------------
2019-07-15T15:36:14.9805959Z error[E0623]: lifetime mismatch
2019-07-15T15:36:14.9806890Z   --> /checkout/src/test/ui/self/elision/ref-mut-self.rs:12:9
2019-07-15T15:36:14.9807141Z    |
2019-07-15T15:36:14.9807574Z LL |     fn ref_self(&mut self, f: &u32) -> &u32 {
2019-07-15T15:36:14.9807973Z    |                               ----     ----
2019-07-15T15:36:14.9808182Z    |                               |
2019-07-15T15:36:14.9808354Z    |                               this parameter and the return type are declared with different lifetimes...
2019-07-15T15:36:14.9808507Z LL |         f //~ ERROR lifetime mismatch
2019-07-15T15:36:14.9808680Z    |         ^ ...but data from `f` is returned here
2019-07-15T15:36:14.9808943Z error[E0623]: lifetime mismatch
2019-07-15T15:36:14.9809349Z   --> /checkout/src/test/ui/self/elision/ref-mut-self.rs:18:9
2019-07-15T15:36:14.9809531Z    |
2019-07-15T15:36:14.9809531Z    |
2019-07-15T15:36:14.9810028Z LL |     fn ref_Self(self: &mut Self, f: &u32) -> &u32 {
2019-07-15T15:36:14.9810523Z    |                                     ----     ----
2019-07-15T15:36:14.9810866Z    |                                     |
2019-07-15T15:36:14.9811036Z    |                                     this parameter and the return type are declared with different lifetimes...
2019-07-15T15:36:14.9811184Z LL |         f //~ ERROR lifetime mismatch
2019-07-15T15:36:14.9811323Z    |         ^ ...but data from `f` is returned here
2019-07-15T15:36:14.9811608Z error[E0623]: lifetime mismatch
2019-07-15T15:36:14.9811977Z   --> /checkout/src/test/ui/self/elision/ref-mut-self.rs:22:9
2019-07-15T15:36:14.9812170Z    |
2019-07-15T15:36:14.9812170Z    |
2019-07-15T15:36:14.9812716Z LL |     fn box_ref_Self(self: Box<&mut Self>, f: &u32) -> &u32 {
2019-07-15T15:36:14.9813382Z    |                                              ----     ----
2019-07-15T15:36:14.9813797Z    |                                              |
2019-07-15T15:36:14.9814270Z    |                                              this parameter and the return type are declared with different lifetimes...
2019-07-15T15:36:14.9814426Z LL |         f //~ ERROR lifetime mismatch
2019-07-15T15:36:14.9815020Z    |         ^ ...but data from `f` is returned here
2019-07-15T15:36:14.9815298Z error[E0623]: lifetime mismatch
2019-07-15T15:36:14.9815778Z   --> /checkout/src/test/ui/self/elision/ref-mut-self.rs:26:9
2019-07-15T15:36:14.9817032Z    |
2019-07-15T15:36:14.9817032Z    |
2019-07-15T15:36:14.9819138Z LL |     fn pin_ref_Self(self: Pin<&mut Self>, f: &u32) -> &u32 {
2019-07-15T15:36:14.9821191Z    |                                              ----     ----
2019-07-15T15:36:14.9821918Z    |                                              |
2019-07-15T15:36:14.9822397Z    |                                              this parameter and the return type are declared with different lifetimes...
2019-07-15T15:36:14.9822590Z LL |         f //~ ERROR lifetime mismatch
2019-07-15T15:36:14.9822755Z    |         ^ ...but data from `f` is returned here
2019-07-15T15:36:14.9823040Z error[E0623]: lifetime mismatch
2019-07-15T15:36:14.9823497Z   --> /checkout/src/test/ui/self/elision/ref-mut-self.rs:30:9
2019-07-15T15:36:14.9823716Z    |
2019-07-15T15:36:14.9823716Z    |
2019-07-15T15:36:14.9824108Z LL |     fn box_box_ref_Self(self: Box<Box<&mut Self>>, f: &u32) -> &u32 {
2019-07-15T15:36:14.9825077Z    |                                                       ----     ----
2019-07-15T15:36:14.9825366Z    |                                                       |
2019-07-15T15:36:14.9825531Z    |                                                       this parameter and the return type are declared with different lifetimes...
2019-07-15T15:36:14.9825703Z LL |         f //~ ERROR lifetime mismatch
2019-07-15T15:36:14.9825849Z    |         ^ ...but data from `f` is returned here
2019-07-15T15:36:14.9826155Z error[E0623]: lifetime mismatch
2019-07-15T15:36:14.9826570Z   --> /checkout/src/test/ui/self/elision/ref-mut-self.rs:34:9
2019-07-15T15:36:14.9826758Z    |
2019-07-15T15:36:14.9826758Z    |
2019-07-15T15:36:14.9827166Z LL |     fn box_pin_ref_Self(self: Box<Pin<&mut Self>>, f: &u32) -> &u32 {
2019-07-15T15:36:14.9827591Z    |                                                       ----     ----
2019-07-15T15:36:14.9827776Z    |                                                       |
2019-07-15T15:36:14.9827957Z    |                                                       this parameter and the return type are declared with different lifetimes...
2019-07-15T15:36:14.9828121Z LL |         f //~ ERROR lifetime mismatch
2019-07-15T15:36:14.9828269Z    |         ^ ...but data from `f` is returned here
2019-07-15T15:36:14.9828554Z error: aborting due to 6 previous errors
2019-07-15T15:36:14.9828677Z 
2019-07-15T15:36:14.9828813Z 
2019-07-15T15:36:14.9829182Z ------------------------------------------
2019-07-15T15:36:14.9829182Z ------------------------------------------
2019-07-15T15:36:14.9829340Z 
2019-07-15T15:36:14.9829478Z 
2019-07-15T15:36:14.9829995Z ---- [ui] ui/self/elision/ref-mut-struct.rs stdout ----
2019-07-15T15:36:14.9830390Z 
2019-07-15T15:36:14.9830390Z 
2019-07-15T15:36:14.9830530Z 1 error[E0623]: lifetime mismatch
2019-07-15T15:36:14.9831088Z -   --> $DIR/ref-mut-struct.rs:14:9
2019-07-15T15:36:14.9831508Z +   --> $DIR/ref-mut-struct.rs:12:9
2019-07-15T15:36:14.9831684Z 3    |
2019-07-15T15:36:14.9832061Z 4 LL |     fn ref_Struct(self: &mut Struct, f: &u32) -> &u32 {
2019-07-15T15:36:14.9832485Z 5    |                                         ----     ----
2019-07-15T15:36:14.9832665Z 
2019-07-15T15:36:14.9832808Z 9    |         ^ ...but data from `f` is returned here
2019-07-15T15:36:14.9832970Z 10 
2019-07-15T15:36:14.9833105Z 11 error[E0623]: lifetime mismatch
2019-07-15T15:36:14.9833455Z -   --> $DIR/ref-mut-struct.rs:18:9
2019-07-15T15:36:14.9833856Z +   --> $DIR/ref-mut-struct.rs:16:9
2019-07-15T15:36:14.9834035Z 13    |
2019-07-15T15:36:14.9834842Z 14 LL |     fn box_ref_Struct(self: Box<&mut Struct>, f: &u32) -> &u32 {
2019-07-15T15:36:14.9835366Z 15    |                                                  ----     ----
2019-07-15T15:36:14.9835568Z 
2019-07-15T15:36:14.9835716Z 19    |         ^ ...but data from `f` is returned here
2019-07-15T15:36:14.9835861Z 20 
2019-07-15T15:36:14.9836027Z 21 error[E0623]: lifetime mismatch
2019-07-15T15:36:14.9836413Z -   --> $DIR/ref-mut-struct.rs:22:9
2019-07-15T15:36:14.9836816Z +   --> $DIR/ref-mut-struct.rs:20:9
2019-07-15T15:36:14.9836998Z 23    |
2019-07-15T15:36:14.9837575Z 24 LL |     fn pin_ref_Struct(self: Pin<&mut Struct>, f: &u32) -> &u32 {
2019-07-15T15:36:14.9838017Z 25    |                                                  ----     ----
2019-07-15T15:36:14.9838208Z 
2019-07-15T15:36:14.9838375Z 29    |         ^ ...but data from `f` is returned here
2019-07-15T15:36:14.9838518Z 30 
2019-07-15T15:36:14.9838657Z 31 error[E0623]: lifetime mismatch
2019-07-15T15:36:14.9839013Z -   --> $DIR/ref-mut-struct.rs:26:9
2019-07-15T15:36:14.9839429Z +   --> $DIR/ref-mut-struct.rs:24:9
2019-07-15T15:36:14.9839613Z 33    |
2019-07-15T15:36:14.9840046Z 34 LL |     fn box_box_ref_Struct(self: Box<Box<&mut Struct>>, f: &u32) -> &u32 {
2019-07-15T15:36:14.9840495Z 35    |                                                           ----     ----
2019-07-15T15:36:14.9840665Z 
2019-07-15T15:36:14.9840812Z 39    |         ^ ...but data from `f` is returned here
2019-07-15T15:36:14.9840973Z 40 
2019-07-15T15:36:14.9841113Z 41 error[E0623]: lifetime mismatch
2019-07-15T15:36:14.9841487Z -   --> $DIR/ref-mut-struct.rs:30:9
2019-07-15T15:36:14.9841900Z +   --> $DIR/ref-mut-struct.rs:28:9
2019-07-15T15:36:14.9842083Z 43    |
2019-07-15T15:36:14.9842477Z 44 LL |     fn box_pin_ref_Struct(self: Box<Pin<&mut Struct>>, f: &u32) -> &u32 {
2019-07-15T15:36:14.9842937Z 45    |                                                           ----     ----
2019-07-15T15:36:14.9843253Z 
2019-07-15T15:36:14.9843401Z The actual stderr differed from the expected stderr.
2019-07-15T15:36:14.9844010Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-mut-struct/ref-mut-struct.stderr
2019-07-15T15:36:14.9844010Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-mut-struct/ref-mut-struct.stderr
2019-07-15T15:36:14.9844851Z To update references, rerun the tests and pass the `--bless` flag
2019-07-15T15:36:14.9845383Z To only update this specific test, also pass `--test-args self/elision/ref-mut-struct.rs`
2019-07-15T15:36:14.9845725Z error: 1 errors occurred comparing output.
2019-07-15T15:36:14.9845871Z status: exit code: 1
2019-07-15T15:36:14.9845871Z status: exit code: 1
2019-07-15T15:36:14.9847027Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/elision/ref-mut-struct.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-mut-struct" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-mut-struct/auxiliary" "-A" "unused"
2019-07-15T15:36:14.9847746Z ------------------------------------------
2019-07-15T15:36:14.9847912Z 
2019-07-15T15:36:14.9848268Z ------------------------------------------
2019-07-15T15:36:14.9848465Z stderr:
2019-07-15T15:36:14.9848465Z stderr:
2019-07-15T15:36:14.9848831Z ------------------------------------------
2019-07-15T15:36:14.9849018Z error[E0623]: lifetime mismatch
2019-07-15T15:36:14.9849551Z   --> /checkout/src/test/ui/self/elision/ref-mut-struct.rs:12:9
2019-07-15T15:36:14.9849763Z    |
2019-07-15T15:36:14.9850169Z LL |     fn ref_Struct(self: &mut Struct, f: &u32) -> &u32 {
2019-07-15T15:36:14.9850576Z    |                                         ----     ----
2019-07-15T15:36:14.9850758Z    |                                         |
2019-07-15T15:36:14.9850936Z    |                                         this parameter and the return type are declared with different lifetimes...
2019-07-15T15:36:14.9851088Z LL |         f //~ ERROR lifetime mismatch
2019-07-15T15:36:14.9851238Z    |         ^ ...but data from `f` is returned here
2019-07-15T15:36:14.9851517Z error[E0623]: lifetime mismatch
2019-07-15T15:36:14.9851913Z   --> /checkout/src/test/ui/self/elision/ref-mut-struct.rs:16:9
2019-07-15T15:36:14.9852093Z    |
2019-07-15T15:36:14.9852093Z    |
2019-07-15T15:36:14.9852472Z LL |     fn box_ref_Struct(self: Box<&mut Struct>, f: &u32) -> &u32 {
2019-07-15T15:36:14.9852881Z    |                                                  ----     ----
2019-07-15T15:36:14.9853216Z    |                                                  |
2019-07-15T15:36:14.9853374Z    |                                                  this parameter and the return type are declared with different lifetimes...
2019-07-15T15:36:14.9853679Z LL |         f //~ ERROR lifetime mismatch
2019-07-15T15:36:14.9853855Z    |         ^ ...but data from `f` is returned here
2019-07-15T15:36:14.9854138Z error[E0623]: lifetime mismatch
2019-07-15T15:36:14.9854947Z   --> /checkout/src/test/ui/self/elision/ref-mut-struct.rs:20:9
2019-07-15T15:36:14.9855193Z    |
2019-07-15T15:36:14.9855193Z    |
2019-07-15T15:36:14.9855625Z LL |     fn pin_ref_Struct(self: Pin<&mut Struct>, f: &u32) -> &u32 {
2019-07-15T15:36:14.9856077Z    |                                                  ----     ----
2019-07-15T15:36:14.9856273Z    |                                                  |
2019-07-15T15:36:14.9856456Z    |                                                  this parameter and the return type are declared with different lifetimes...
2019-07-15T15:36:14.9856624Z LL |         f //~ ERROR lifetime mismatch
2019-07-15T15:36:14.9856792Z    |         ^ ...but data from `f` is returned here
2019-07-15T15:36:14.9857082Z error[E0623]: lifetime mismatch
2019-07-15T15:36:14.9857496Z   --> /checkout/src/test/ui/self/elision/ref-mut-struct.rs:24:9
2019-07-15T15:36:14.9857680Z    |
2019-07-15T15:36:14.9857680Z    |
2019-07-15T15:36:14.9858238Z LL |     fn box_box_ref_Struct(self: Box<Box<&mut Struct>>, f: &u32) -> &u32 {
2019-07-15T15:36:14.9858673Z    |                                                           ----     ----
2019-07-15T15:36:14.9858880Z    |                                                           |
2019-07-15T15:36:14.9859040Z    |                                                           this parameter and the return type are declared with different lifetimes...
2019-07-15T15:36:14.9859212Z LL |         f //~ ERROR lifetime mismatch
2019-07-15T15:36:14.9859352Z    |         ^ ...but data from `f` is returned here
2019-07-15T15:36:14.9859618Z error[E0623]: lifetime mismatch
2019-07-15T15:36:14.9860014Z   --> /checkout/src/test/ui/self/elision/ref-mut-struct.rs:28:9
2019-07-15T15:36:14.9860210Z    |
2019-07-15T15:36:14.9860210Z    |
2019-07-15T15:36:14.9860600Z LL |     fn box_pin_ref_Struct(self: Box<Pin<&mut Struct>>, f: &u32) -> &u32 {
2019-07-15T15:36:14.9861020Z    |                                                           ----     ----
2019-07-15T15:36:14.9861320Z    |                                                           |
2019-07-15T15:36:14.9861546Z    |                                                           this parameter and the return type are declared with different lifetimes...
2019-07-15T15:36:14.9861718Z LL |         f //~ ERROR lifetime mismatch
2019-07-15T15:36:14.9861858Z    |         ^ ...but data from `f` is returned here
2019-07-15T15:36:14.9862114Z error: aborting due to 5 previous errors
2019-07-15T15:36:14.9862232Z 
2019-07-15T15:36:14.9862358Z 
2019-07-15T15:36:14.9862775Z ------------------------------------------
2019-07-15T15:36:14.9862775Z ------------------------------------------
2019-07-15T15:36:14.9862940Z 
2019-07-15T15:36:14.9863061Z 
2019-07-15T15:36:14.9863416Z ---- [ui] ui/self/elision/ref-self.rs stdout ----
2019-07-15T15:36:14.9863787Z 
2019-07-15T15:36:14.9863787Z 
2019-07-15T15:36:14.9863921Z 1 error[E0623]: lifetime mismatch
2019-07-15T15:36:14.9864281Z -   --> $DIR/ref-self.rs:14:9
2019-07-15T15:36:14.9865085Z +   --> $DIR/ref-self.rs:21:9
2019-07-15T15:36:14.9865318Z 3    |
2019-07-15T15:36:14.9865717Z 4 LL |     fn ref_self(&self, f: &u32) -> &u32 {
2019-07-15T15:36:14.9866167Z 5    |                           ----     ----
2019-07-15T15:36:14.9866332Z 
2019-07-15T15:36:14.9866478Z 9    |         ^ ...but data from `f` is returned here
2019-07-15T15:36:14.9870357Z 10 
2019-07-15T15:36:14.9872111Z 11 error[E0623]: lifetime mismatch
2019-07-15T15:36:14.9872651Z -   --> $DIR/ref-self.rs:20:9
2019-07-15T15:36:14.9873034Z +   --> $DIR/ref-self.rs:27:9
2019-07-15T15:36:14.9873746Z 13    |
2019-07-15T15:36:14.9874524Z 14 LL |     fn ref_Self(self: &Self, f: &u32) -> &u32 {
2019-07-15T15:36:14.9875245Z 15    |                                 ----     ----
2019-07-15T15:36:14.9875318Z 
2019-07-15T15:36:14.9875370Z 19    |         ^ ...but data from `f` is returned here
2019-07-15T15:36:14.9875416Z 20 
2019-07-15T15:36:14.9875484Z 21 error[E0623]: lifetime mismatch
2019-07-15T15:36:14.9875905Z -   --> $DIR/ref-self.rs:24:9
2019-07-15T15:36:14.9876120Z +   --> $DIR/ref-self.rs:31:9
2019-07-15T15:36:14.9876184Z 23    |
2019-07-15T15:36:14.9876453Z 24 LL |     fn box_ref_Self(self: Box<&Self>, f: &u32) -> &u32 {
2019-07-15T15:36:14.9876710Z 25    |                                          ----     ----
2019-07-15T15:36:14.9876748Z 
2019-07-15T15:36:14.9876814Z 29    |         ^ ...but data from `f` is returned here
2019-07-15T15:36:14.9876858Z 30 
2019-07-15T15:36:14.9876903Z 31 error[E0623]: lifetime mismatch
2019-07-15T15:36:14.9877132Z -   --> $DIR/ref-self.rs:28:9
2019-07-15T15:36:14.9877357Z +   --> $DIR/ref-self.rs:35:9
2019-07-15T15:36:14.9877403Z 33    |
2019-07-15T15:36:14.9877681Z 34 LL |     fn pin_ref_Self(self: Pin<&Self>, f: &u32) -> &u32 {
2019-07-15T15:36:14.9877933Z 35    |                                          ----     ----
2019-07-15T15:36:14.9877966Z 
2019-07-15T15:36:14.9878012Z 39    |         ^ ...but data from `f` is returned here
2019-07-15T15:36:14.9878074Z 40 
2019-07-15T15:36:14.9878118Z 41 error[E0623]: lifetime mismatch
2019-07-15T15:36:14.9878327Z -   --> $DIR/ref-self.rs:32:9
2019-07-15T15:36:14.9878561Z +   --> $DIR/ref-self.rs:39:9
2019-07-15T15:36:14.9878607Z 43    |
2019-07-15T15:36:14.9878865Z 44 LL |     fn box_box_ref_Self(self: Box<Box<&Self>>, f: &u32) -> &u32 {
2019-07-15T15:36:14.9879118Z 45    |                                                   ----     ----
2019-07-15T15:36:14.9879170Z 
2019-07-15T15:36:14.9879215Z 49    |         ^ ...but data from `f` is returned here
2019-07-15T15:36:14.9879258Z 50 
2019-07-15T15:36:14.9879329Z 51 error[E0623]: lifetime mismatch
2019-07-15T15:36:14.9879542Z -   --> $DIR/ref-self.rs:36:9
2019-07-15T15:36:14.9880070Z +   --> $DIR/ref-self.rs:43:9
2019-07-15T15:36:14.9880112Z 53    |
2019-07-15T15:36:14.9880551Z 54 LL |     fn box_pin_ref_Self(self: Box<Pin<&Self>>, f: &u32) -> &u32 {
2019-07-15T15:36:14.9880798Z 55    |                                                   ----     ----
2019-07-15T15:36:14.9880889Z 58 LL |         f
2019-07-15T15:36:14.9880889Z 58 LL |         f
2019-07-15T15:36:14.9880934Z 59    |         ^ ...but data from `f` is returned here
2019-07-15T15:36:14.9881424Z - error: aborting due to 6 previous errors
2019-07-15T15:36:14.9881424Z - error: aborting due to 6 previous errors
2019-07-15T15:36:14.9881491Z + error[E0623]: lifetime mismatch
2019-07-15T15:36:14.9881694Z +   --> $DIR/ref-self.rs:53:9
2019-07-15T15:36:14.9881737Z +    |
2019-07-15T15:36:14.9882005Z + LL |     fn wrap_ref_Self_Self(self: Wrap<&Self, Self>, f: &u8) -> &u8 {
2019-07-15T15:36:14.9882248Z +    |                                                       ---     ---
2019-07-15T15:36:14.9882312Z +    |                                                       |
2019-07-15T15:36:14.9882392Z +    |                                                       this parameter and the return type are declared with different lifetimes...
2019-07-15T15:36:14.9882445Z + LL |         f
2019-07-15T15:36:14.9882494Z +    |         ^ ...but data from `f` is returned here
2019-07-15T15:36:14.9882598Z + error: aborting due to 7 previous errors
2019-07-15T15:36:14.9882638Z 62 
2019-07-15T15:36:14.9882691Z 63 
2019-07-15T15:36:14.9882724Z 
2019-07-15T15:36:14.9882724Z 
2019-07-15T15:36:14.9882750Z 
2019-07-15T15:36:14.9882794Z The actual stderr differed from the expected stderr.
2019-07-15T15:36:14.9883111Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-self/ref-self.stderr
2019-07-15T15:36:14.9883355Z To update references, rerun the tests and pass the `--bless` flag
2019-07-15T15:36:14.9883609Z To only update this specific test, also pass `--test-args self/elision/ref-self.rs`
2019-07-15T15:36:14.9883812Z error: 1 errors occurred comparing output.
2019-07-15T15:36:14.9883855Z status: exit code: 1
2019-07-15T15:36:14.9883855Z status: exit code: 1
2019-07-15T15:36:14.9885207Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/elision/ref-self.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-self" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/ref-self/auxiliary" "-A" "unused"
2019-07-15T15:36:14.9885578Z ------------------------------------------
2019-07-15T15:36:14.9885633Z 
2019-07-15T15:36:14.9885856Z ------------------------------------------
2019-07-15T15:36:14.9885901Z stderr:
2019-07-15T15:36:14.9885901Z stderr:
2019-07-15T15:36:14.9886150Z ------------------------------------------
2019-07-15T15:36:14.9886199Z error[E0623]: lifetime mismatch
2019-07-15T15:36:14.9886438Z   --> /checkout/src/test/ui/self/elision/ref-self.rs:21:9
2019-07-15T15:36:14.9886506Z    |
2019-07-15T15:36:14.9886735Z LL |     fn ref_self(&self, f: &u32) -> &u32 {
2019-07-15T15:36:14.9886958Z    |                           ----     ----
2019-07-15T15:36:14.9887006Z    |                           |
2019-07-15T15:36:14.9887088Z    |                           this parameter and the return type are declared with different lifetimes...
2019-07-15T15:36:14.9887144Z LL |         f //~ ERROR lifetime mismatch
2019-07-15T15:36:14.9887210Z    |         ^ ...but data from `f` is returned here
2019-07-15T15:36:14.9887281Z error[E0623]: lifetime mismatch
2019-07-15T15:36:14.9887530Z   --> /checkout/src/test/ui/self/elision/ref-self.rs:27:9
2019-07-15T15:36:14.9887596Z    |
2019-07-15T15:36:14.9887596Z    |
2019-07-15T15:36:14.9887827Z LL |     fn ref_Self(self: &Self, f: &u32) -> &u32 {
2019-07-15T15:36:14.9888220Z    |                                 ----     ----
2019-07-15T15:36:14.9888285Z    |                                 |
2019-07-15T15:36:14.9888339Z    |                                 this parameter and the return type are declared with different lifetimes...
2019-07-15T15:36:14.9888390Z LL |         f //~ ERROR lifetime mismatch
2019-07-15T15:36:14.9888452Z    |         ^ ...but data from `f` is returned here
2019-07-15T15:36:14.9888521Z error[E0623]: lifetime mismatch
2019-07-15T15:36:14.9888862Z   --> /checkout/src/test/ui/self/elision/ref-self.rs:31:9
2019-07-15T15:36:14.9888936Z    |
2019-07-15T15:36:14.9888936Z    |
2019-07-15T15:36:14.9889204Z LL |     fn box_ref_Self(self: Box<&Self>, f: &u32) -> &u32 {
2019-07-15T15:36:14.9889437Z    |                                          ----     ----
2019-07-15T15:36:14.9889504Z    |                                          |
2019-07-15T15:36:14.9889560Z    |                                          this parameter and the return type are declared with different lifetimes...
2019-07-15T15:36:14.9889621Z LL |         f //~ ERROR lifetime mismatch
2019-07-15T15:36:14.9889686Z    |         ^ ...but data from `f` is returned here
2019-07-15T15:36:14.9889754Z error[E0623]: lifetime mismatch
2019-07-15T15:36:14.9889990Z   --> /checkout/src/test/ui/self/elision/ref-self.rs:35:9
2019-07-15T15:36:14.9890053Z    |
2019-07-15T15:36:14.9890053Z    |
2019-07-15T15:36:14.9890282Z LL |     fn pin_ref_Self(self: Pin<&Self>, f: &u32) -> &u32 {
2019-07-15T15:36:14.9890520Z    |                                          ----     ----
2019-07-15T15:36:14.9890588Z    |                                          |
2019-07-15T15:36:14.9890643Z    |                                          this parameter and the return type are declared with different lifetimes...
2019-07-15T15:36:14.9890694Z LL |         f //~ ERROR lifetime mismatch
2019-07-15T15:36:14.9890759Z    |         ^ ...but data from `f` is returned here
2019-07-15T15:36:14.9891070Z error[E0623]: lifetime mismatch
2019-07-15T15:36:14.9891372Z   --> /checkout/src/test/ui/self/elision/ref-self.rs:39:9
2019-07-15T15:36:14.9891437Z    |
2019-07-15T15:36:14.9891437Z    |
2019-07-15T15:36:14.9891672Z LL |     fn box_box_ref_Self(self: Box<Box<&Self>>, f: &u32) -> &u32 {
2019-07-15T15:36:14.9891905Z    |                                                   ----     ----
2019-07-15T15:36:14.9891974Z    |                                                   |
2019-07-15T15:36:14.9892039Z    |                                                   this parameter and the return type are declared with different lifetimes...
2019-07-15T15:36:14.9892090Z LL |         f //~ ERROR lifetime mismatch
2019-07-15T15:36:14.9892151Z    |         ^ ...but data from `f` is returned here
2019-07-15T15:36:14.9892217Z error[E0623]: lifetime mismatch
2019-07-15T15:36:14.9892467Z   --> /checkout/src/test/ui/self/elision/ref-self.rs:43:9
2019-07-15T15:36:14.9892511Z    |
2019-07-15T15:36:14.9892511Z    |
2019-07-15T15:36:14.9892743Z LL |     fn box_pin_ref_Self(self: Box<Pin<&Self>>, f: &u32) -> &u32 {
2019-07-15T15:36:14.9892983Z    |                                                   ----     ----
2019-07-15T15:36:14.9893051Z    |                                                   |
2019-07-15T15:36:14.9893106Z    |                                                   this parameter and the return type are declared with different lifetimes...
2019-07-15T15:36:14.9893156Z LL |         f //~ ERROR lifetime mismatch
2019-07-15T15:36:14.9893220Z    |         ^ ...but data from `f` is returned here
2019-07-15T15:36:14.9893294Z error[E0623]: lifetime mismatch
2019-07-15T15:36:14.9893680Z   --> /checkout/src/test/ui/self/elision/ref-self.rs:53:9
2019-07-15T15:36:14.9893728Z    |
2019-07-15T15:36:14.9893728Z    |
2019-07-15T15:36:14.9893982Z LL |     fn wrap_ref_Self_Self(self: Wrap<&Self, Self>, f: &u8) -> &u8 {
2019-07-15T15:36:14.9894233Z    |                                                       ---     ---
2019-07-15T15:36:14.9894284Z    |                                                       |
2019-07-15T15:36:14.9894349Z    |                                                       this parameter and the return type are declared with different lifetimes...
2019-07-15T15:36:14.9894419Z LL |         f //~ ERROR lifetime mismatch
2019-07-15T15:36:14.9894467Z    |         ^ ...but data from `f` is returned here
2019-07-15T15:36:14.9894893Z error: aborting due to 7 previous errors
2019-07-15T15:36:14.9894945Z 
2019-07-15T15:36:14.9894971Z 
2019-07-15T15:36:14.9895374Z ------------------------------------------
2019-07-15T15:36:14.9895374Z ------------------------------------------
2019-07-15T15:36:14.9895418Z 
2019-07-15T15:36:14.9895444Z 
2019-07-15T15:36:14.9896203Z ---- [ui] ui/self/elision/ref-struct.rs stdout ----
2019-07-15T15:36:14.9896293Z 
2019-07-15T15:36:14.9896293Z 
2019-07-15T15:36:14.9896357Z 1 error[E0623]: lifetime mismatch
2019-07-15T15:36:14.9896587Z -   --> $DIR/ref-struct.rs:14:9
2019-07-15T15:36:14.9896800Z +   --> $DIR/ref-struct.rs:12:9
2019-07-15T15:36:14.9896844Z 3    |
2019-07-15T15:36:14.9897118Z 4 LL |     fn ref_Struct(self: &Struct, f: &u32) -> &u32 {
2019-07-15T15:36:14.9897356Z 5    |                                     ----     ----
2019-07-15T15:36:14.9897388Z 
2019-07-15T15:36:14.9897451Z 9    |         ^ ...but data from `f` is returned here
2019-07-15T15:36:14.9897495Z 10 
2019-07-15T15:36:14.9897539Z 11 error[E0623]: lifetime mismatch
2019-07-15T15:36:14.9897750Z -   --> $DIR/ref-struct.rs:18:9
2019-07-15T15:36:14.9897979Z +   --> $DIR/ref-struct.rs:16:9
2019-07-15T15:36:14.9898024Z 13    |
2019-07-15T15:36:14.9898278Z 14 LL |     fn box_ref_Struct(self: Box<&Struct>, f: &u32) -> &u32 {
2019-07-15T15:36:14.9898864Z 15    |                                              ----     ----
2019-07-15T15:36:14.9898895Z 
2019-07-15T15:36:14.9898937Z 19    |         ^ ...but data from `f` is returned here
2019-07-15T15:36:14.9898976Z 20 
2019-07-15T15:36:14.9899035Z 21 error[E0623]: lifetime mismatch
2019-07-15T15:36:14.9899235Z -   --> $DIR/ref-struct.rs:22:9
---
2019-07-15T15:36:14.9913982Z test result: FAILED. 5798 passed; 4 failed; 21 ignored; 0 measured; 0 filtered out
2019-07-15T15:36:14.9914017Z 
2019-07-15T15:36:14.9914042Z 
2019-07-15T15:36:14.9914067Z 
2019-07-15T15:36:14.9916055Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-07-15T15:36:14.9916425Z 
2019-07-15T15:36:14.9916455Z 
2019-07-15T15:36:14.9916520Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-07-15T15:36:14.9916570Z Build completed unsuccessfully in 0:57:24
2019-07-15T15:36:14.9916570Z Build completed unsuccessfully in 0:57:24
2019-07-15T15:36:14.9916885Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:535:22
2019-07-15T15:36:14.9916967Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-07-15T15:36:15.9584443Z ##[error]Bash exited with code '1'.
2019-07-15T15:36:15.9616919Z ##[section]Starting: Checkout
2019-07-15T15:36:15.9618551Z ==============================================================================
2019-07-15T15:36:15.9618626Z Task         : Get sources
2019-07-15T15:36:15.9618887Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
