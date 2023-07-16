plain
2019-07-12T17:31:57.0585020Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-12T17:31:57.0791533Z ##[command]git config gc.auto 0
2019-07-12T17:31:57.0858273Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-12T17:31:57.0924231Z ##[command]git config --get-all http.proxy
2019-07-12T17:31:57.1072244Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/60826/merge:refs/remotes/pull/60826/merge
---
2019-07-12T17:32:32.1622313Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-12T17:32:32.1622346Z 
2019-07-12T17:32:32.1622594Z   git checkout -b <new-branch-name>
2019-07-12T17:32:32.1622623Z 
2019-07-12T17:32:32.1622667Z HEAD is now at b2a27ed58 Merge 5da0c3a2fa772b1b14afef2beb7cceb6d9cff8a2 into 71f9384e3bec467158a628e2d11e77ffada16a90
2019-07-12T17:32:32.1796818Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-12T17:32:32.1799575Z ==============================================================================
2019-07-12T17:32:32.1799636Z Task         : Bash
2019-07-12T17:32:32.1799683Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-12T17:34:25.5148725Z Attempting with retry: curl -y 30 -Y 10 --connect-timeout 30 -f -L -C - -o /tmp/rustci_docker_cache https://.s3.amazonaws.com/docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624
2019-07-12T17:34:25.5204348Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-12T17:34:25.5204821Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-12T17:34:25.5205003Z 
2019-07-12T17:34:25.5247021Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-12T17:34:26.9302074Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-12T17:34:26.9302203Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-12T17:34:26.9302274Z 
2019-07-12T17:34:26.9302274Z 
2019-07-12T17:34:26.9303622Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-12T17:34:28.5437751Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-12T17:34:28.5438161Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-12T17:34:28.5438230Z 
2019-07-12T17:34:28.5438230Z 
2019-07-12T17:34:28.5481801Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-12T17:34:31.5554157Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-12T17:34:31.5554718Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-12T17:34:31.5554780Z 
2019-07-12T17:34:31.5554780Z 
2019-07-12T17:34:31.5596774Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-12T17:34:35.5668699Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-12T17:34:35.5669345Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-12T17:34:35.5669672Z 
2019-07-12T17:34:35.5669672Z 
2019-07-12T17:34:35.5712765Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-12T17:34:35.5718395Z The command has failed after 5 attempts.
2019-07-12T17:34:35.6363830Z open /tmp/rustci_docker_cache: no such file or directory
2019-07-12T17:34:35.6402382Z Attempting with retry: docker build --rm -t rust-ci -f /home/vsts/work/1/s/src/ci/docker/x86_64-gnu-llvm-6.0/Dockerfile /home/vsts/work/1/s/src/ci/docker
2019-07-12T17:34:35.7788771Z Sending build context to Docker daemon  521.7kB
2019-07-12T17:34:35.7790020Z 
2019-07-12T17:34:35.7984626Z Step 1/8 : FROM ubuntu:16.04
---
2019-07-12T17:34:52.4117630Z Reading package lists...
2019-07-12T17:34:53.4345814Z Reading package lists...
2019-07-12T17:34:53.6225489Z Building dependency tree...
2019-07-12T17:34:53.6226241Z Reading state information...
2019-07-12T17:34:53.7452598Z The following additional packages will be installed:
2019-07-12T17:34:53.7459428Z   binfmt-support binutils bzip2 cmake-data cpp cpp-5 dpkg-dev g++-5 gcc gcc-5
2019-07-12T17:34:53.7459996Z   git-man libarchive13 libasan2 libasn1-8-heimdal libatomic1
2019-07-12T17:34:53.7460346Z   libbabeltrace-ctf1 libbabeltrace1 libbsd-dev libbsd0 libbz2-1.0 libc-dev-bin
2019-07-12T17:34:53.7476393Z   libedit2 liberror-perl libexpat1 libffi-dev libffi6 libgcc-5-dev libgdbm3
2019-07-12T17:34:53.7476789Z   libglib2.0-0 libgmp10 libgnutls30 libgomp1 libgssapi-krb5-2
2019-07-12T17:34:53.7477083Z   libgssapi3-heimdal libhcrypto4-heimdal libheimbase1-heimdal
2019-07-12T17:34:53.7477369Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-12T17:34:53.7477369Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-12T17:34:53.7478580Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-12T17:34:53.7479244Z   libkrb5-3 libkrb5support0 libldap-2.4-2 libllvm6.0 liblsan0 liblzo2-2
2019-07-12T17:34:53.7479934Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-12T17:34:53.7480847Z   libperl5.22 libpipeline1 libpython-stdlib libpython2.7-minimal
2019-07-12T17:34:53.7481181Z   libpython2.7-stdlib libpython3.5 libpython3.5-minimal libpython3.5-stdlib
2019-07-12T17:34:53.7481477Z   libquadmath0 libroken18-heimdal librtmp1 libsasl2-2 libsasl2-modules-db
2019-07-12T17:34:53.7483114Z   libsqlite3-0 libssl1.0.0 libstdc++-5-dev libtasn1-6 libtinfo-dev libtsan0
2019-07-12T17:34:53.7483556Z   libubsan0 libwind0-heimdal libxml2 linux-libc-dev llvm-6.0 llvm-6.0-dev
2019-07-12T17:34:53.7484478Z   llvm-6.0-runtime mime-support openssl patch perl perl-modules-5.22 python
2019-07-12T17:34:53.7485053Z   python-minimal python2.7-minimal
2019-07-12T17:34:53.7488853Z Suggested packages:
2019-07-12T17:34:53.7489281Z   binutils-doc bzip2-doc codeblocks eclipse ninja-build cpp-doc gcc-5-locales
2019-07-12T17:34:53.7489562Z   debian-keyring g++-multilib g++-5-multilib gcc-5-doc libstdc++6-5-dbg
2019-07-12T17:34:53.7490417Z   gcc-multilib manpages-dev autoconf automake libtool flex bison gcc-doc
2019-07-12T17:34:53.7491073Z   libasan2-dbg liblsan0-dbg libtsan0-dbg libubsan0-dbg libcilkrts5-dbg
2019-07-12T17:34:53.7491387Z   libmpx0-dbg libquadmath0-dbg gdb-doc gettext-base git-daemon-run
2019-07-12T17:34:53.7491387Z   libmpx0-dbg libquadmath0-dbg gdb-doc gettext-base git-daemon-run
2019-07-12T17:34:53.7491684Z   | git-daemon-sysvinit git-doc git-el git-email git-gui gitk gitweb git-arch
2019-07-12T17:34:53.7491983Z   git-cvs git-mediawiki git-svn lrzip glibc-doc gnutls-bin krb5-doc krb5-user
2019-07-12T17:34:53.7492293Z   libstdc++-5-doc llvm-6.0-doc make-doc ed diffutils-doc perl-doc
2019-07-12T17:34:53.7492599Z   libterm-readline-gnu-perl | libterm-readline-perl-perl python-doc python-tk
2019-07-12T17:34:53.7493896Z   python2.7-doc
2019-07-12T17:34:53.7494018Z Recommended packages:
2019-07-12T17:34:53.7494372Z   build-essential fakeroot libalgorithm-merge-perl libc-dbg gdbserver less
2019-07-12T17:34:53.7494634Z   rsync ssh-client manpages manpages-dev libfile-fcntllock-perl
2019-07-12T17:34:53.7494948Z   libglib2.0-data shared-mime-info xdg-user-dirs krb5-locales libsasl2-modules
2019-07-12T17:34:53.7495190Z   libssl-doc xml-core netbase rename
2019-07-12T17:34:53.7495239Z The following NEW packages will be installed:
2019-07-12T17:34:53.7495552Z   binfmt-support binutils bzip2 ca-certificates cmake cmake-data cpp cpp-5
2019-07-12T17:34:53.7495852Z   curl dpkg-dev file g++ g++-5 gcc gcc-5 gdb git git-man libarchive13 libasan2
2019-07-12T17:34:53.7496142Z   libasn1-8-heimdal libatomic1 libbabeltrace-ctf1 libbabeltrace1 libbsd-dev
2019-07-12T17:34:53.7496453Z   libbsd0 libc-dev-bin libc6-dev libcc1-0 libcilkrts5 libcurl3 libcurl3-gnutls
2019-07-12T17:34:53.7496925Z   libdpkg-perl libedit-dev libedit2 liberror-perl libexpat1 libffi-dev libffi6
2019-07-12T17:34:53.7497269Z   libgcc-5-dev libgdbm3 libglib2.0-0 libgmp10 libgnutls30 libgomp1
2019-07-12T17:34:53.7497868Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-12T17:34:53.7498171Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-12T17:34:53.7498473Z   libkrb5-3 libkrb5support0 libldap-2.4-2 libllvm6.0 liblsan0 liblzo2-2
2019-07-12T17:34:53.7498758Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-12T17:34:53.7498758Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-12T17:34:53.7499036Z   libperl5.22 libpipeline1 libpython-stdlib libpython2.7-minimal
2019-07-12T17:34:53.7499341Z   libpython2.7-stdlib libpython3.5 libpython3.5-minimal libpython3.5-stdlib
2019-07-12T17:34:53.7499631Z   libquadmath0 libroken18-heimdal librtmp1 libsasl2-2 libsasl2-modules-db
2019-07-12T17:34:53.7500514Z   libsqlite3-0 libssl-dev libssl1.0.0 libstdc++-5-dev libtasn1-6 libtinfo-dev
2019-07-12T17:34:53.7500848Z   libtsan0 libubsan0 libwind0-heimdal libxml2 linux-libc-dev llvm-6.0
2019-07-12T17:34:53.7501156Z   llvm-6.0-dev llvm-6.0-runtime llvm-6.0-tools make mime-support openssl patch
2019-07-12T17:34:53.7501449Z   perl perl-modules-5.22 pkg-config python python-minimal python2.7
2019-07-12T17:34:53.7501908Z   python2.7-minimal sudo xz-utils zlib1g-dev
2019-07-12T17:34:53.7501967Z The following packages will be upgraded:
2019-07-12T17:34:54.0241476Z 1 upgraded, 115 newly installed, 0 to remove and 4 not upgraded.
2019-07-12T17:34:54.0241659Z Need to get 121 MB of archives.
2019-07-12T17:34:54.0241713Z After this operation, 592 MB of additional disk space will be used.
2019-07-12T17:34:54.0242545Z Get:1 http://archive.ubuntu.com/ubuntu xenial/main amd64 libgdbm3 amd64 1.8.3-13.1 [16.9 kB]
---
2019-07-12T17:34:55.0929611Z Get:61 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 sudo amd64 1.8.16-0ubuntu1.7 [390 kB]
2019-07-12T17:34:55.0998524Z Get:62 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 openssl amd64 1.0.2g-1ubuntu4.15 [492 kB]
2019-07-12T17:34:55.1082493Z Get:63 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 ca-certificates all 20170717~16.04.2 [167 kB]
2019-07-12T17:34:55.1111406Z Get:64 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 libcurl3-gnutls amd64 7.47.0-1ubuntu2.13 [184 kB]
2019-07-12T17:34:55.1141340Z Get:65 http://archive.ubuntu.com/ubuntu xenial/main amd64 libedit2 amd64 3.1-20150325-1ubuntu2 [76.5 kB]
2019-07-12T17:34:55.1156601Z Get:66 http://archive.ubuntu.com/ubuntu xenial/main amd64 libpipeline1 amd64 1.4.1-2 [24.6 kB]
2019-07-12T17:34:55.1163900Z Get:67 http://archive.ubuntu.com/ubuntu xenial/main amd64 binfmt-support amd64 2.1.6-1 [50.7 kB]
2019-07-12T17:34:55.1738926Z Get:69 http://archive.ubuntu.com/ubuntu xenial/main amd64 libisl15 amd64 0.16.1-1 [524 kB]
2019-07-12T17:34:55.1853069Z Get:70 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 cpp-5 amd64 5.4.0-6ubuntu1~16.04.11 [7660 kB]
2019-07-12T17:34:55.3060563Z Get:71 http://archive.ubuntu.com/ubuntu xenial/main amd64 cpp amd64 4:5.3.1-1ubuntu1 [27.7 kB]
2019-07-12T17:34:55.3069567Z Get:72 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 curl amd64 7.47.0-1ubuntu2.13 [139 kB]
---
2019-07-12T17:35:13.5403706Z Unpacking ca-certificates (20170717~16.04.2) ...
2019-07-12T17:35:13.7159342Z Selecting previously unselected package libcurl3-gnutls:amd64.
2019-07-12T17:35:13.7170055Z Preparing to unpack .../libcurl3-gnutls_7.47.0-1ubuntu2.13_amd64.deb ...
2019-07-12T17:35:13.7333344Z Unpacking libcurl3-gnutls:amd64 (7.47.0-1ubuntu2.13) ...
2019-07-12T17:35:13.8798453Z Selecting previously unselected package libedit2:amd64.
2019-07-12T17:35:13.8815459Z Preparing to unpack .../libedit2_3.1-20150325-1ubuntu2_amd64.deb ...
2019-07-12T17:35:13.8965026Z Unpacking libedit2:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-12T17:35:14.0221774Z Selecting previously unselected package libpipeline1:amd64.
2019-07-12T17:35:14.0242249Z Preparing to unpack .../libpipeline1_1.4.1-2_amd64.deb ...
2019-07-12T17:35:14.0388251Z Unpacking libpipeline1:amd64 (1.4.1-2) ...
2019-07-12T17:35:14.1557692Z Selecting previously unselected package binfmt-support.
2019-07-12T17:35:14.1570337Z Preparing to unpack .../binfmt-support_2.1.6-1_amd64.deb ...
2019-07-12T17:35:14.1705258Z Unpacking binfmt-support (2.1.6-1) ...
2019-07-12T17:35:14.2954094Z Preparing to unpack .../binutils_2.26.1-1ubuntu1~16.04.8_amd64.deb ...
2019-07-12T17:35:14.3115270Z Unpacking binutils (2.26.1-1ubuntu1~16.04.8) ...
2019-07-12T17:35:15.4429790Z Selecting previously unselected package libisl15:amd64.
2019-07-12T17:35:15.4436324Z Preparing to unpack .../libisl15_0.16.1-1_amd64.deb ...
---
2019-07-12T17:35:26.7660280Z Selecting previously unselected package libtinfo-dev:amd64.
2019-07-12T17:35:26.7683380Z Preparing to unpack .../libtinfo-dev_6.0+20160213-1ubuntu1_amd64.deb ...
2019-07-12T17:35:26.7857578Z Unpacking libtinfo-dev:amd64 (6.0+20160213-1ubuntu1) ...
2019-07-12T17:35:26.9030510Z Selecting previously unselected package libedit-dev:amd64.
2019-07-12T17:35:26.9051887Z Preparing to unpack .../libedit-dev_3.1-20150325-1ubuntu2_amd64.deb ...
2019-07-12T17:35:26.9262557Z Unpacking libedit-dev:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-12T17:35:27.0731475Z Selecting previously unselected package libllvm6.0:amd64.
2019-07-12T17:35:27.0751516Z Preparing to unpack .../libllvm6.0_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-12T17:35:27.1071380Z Unpacking libllvm6.0:amd64 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-12T17:35:30.0435872Z Selecting previously unselected package zlib1g-dev:amd64.
2019-07-12T17:35:30.0457026Z Preparing to unpack .../zlib1g-dev_1%3a1.2.8.dfsg-2ubuntu4.1_amd64.deb ...
2019-07-12T17:35:30.0599054Z Unpacking zlib1g-dev:amd64 (1:1.2.8.dfsg-2ubuntu4.1) ...
2019-07-12T17:35:30.1695604Z Preparing to unpack .../libssl-dev_1.0.2g-1ubuntu4.15_amd64.deb ...
2019-07-12T17:35:30.1825063Z Unpacking libssl-dev:amd64 (1.0.2g-1ubuntu4.15) ...
2019-07-12T17:35:30.4924478Z Selecting previously unselected package llvm-6.0-runtime.
2019-07-12T17:35:30.4924478Z Selecting previously unselected package llvm-6.0-runtime.
2019-07-12T17:35:30.4945950Z Preparing to unpack .../llvm-6.0-runtime_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-12T17:35:30.5078659Z Unpacking llvm-6.0-runtime (1:6.0-1ubuntu2~16.04.1) ...
2019-07-12T17:35:30.6292062Z Selecting previously unselected package llvm-6.0.
2019-07-12T17:35:30.6311241Z Preparing to unpack .../llvm-6.0_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-12T17:35:30.6445897Z Unpacking llvm-6.0 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-12T17:35:31.2887531Z Selecting previously unselected package libffi-dev:amd64.
2019-07-12T17:35:31.2907242Z Preparing to unpack .../libffi-dev_3.2.1-4_amd64.deb ...
2019-07-12T17:35:31.3047782Z Unpacking libffi-dev:amd64 (3.2.1-4) ...
2019-07-12T17:35:31.4279546Z Selecting previously unselected package llvm-6.0-dev.
2019-07-12T17:35:31.4299375Z Preparing to unpack .../llvm-6.0-dev_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-12T17:35:31.6421449Z Unpacking llvm-6.0-dev (1:6.0-1ubuntu2~16.04.1) ...
2019-07-12T17:35:36.4483277Z Selecting previously unselected package llvm-6.0-tools.
2019-07-12T17:35:36.4483810Z Preparing to unpack .../llvm-6.0-tools_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-12T17:35:36.4484362Z Unpacking llvm-6.0-tools (1:6.0-1ubuntu2~16.04.1) ...
2019-07-12T17:35:36.4703905Z Selecting previously unselected package pkg-config.
2019-07-12T17:35:36.4736232Z Preparing to unpack .../pkg-config_0.29.1-0ubuntu1_amd64.deb ...
2019-07-12T17:35:36.4867611Z Unpacking pkg-config (0.29.1-0ubuntu1) ...
2019-07-12T17:35:36.6046420Z Processing triggers for systemd (229-4ubuntu21.21) ...
2019-07-12T17:35:37.0113482Z Setting up libgdbm3:amd64 (1.8.3-13.1) ...
2019-07-12T17:35:37.0830063Z Setting up libffi6:amd64 (3.2.1-4) ...
2019-07-12T17:35:37.1236329Z Setting up libglib2.0-0:amd64 (2.48.2-0ubuntu4.3) ...
---
2019-07-12T17:35:40.9544098Z debconf: unable to initialize frontend: Dialog
2019-07-12T17:35:40.9544314Z debconf: (TERM is not set, so the dialog frontend is not usable.)
2019-07-12T17:35:40.9544531Z debconf: falling back to frontend: Readline
2019-07-12T17:35:41.4753540Z Setting up libcurl3-gnutls:amd64 (7.47.0-1ubuntu2.13) ...
2019-07-12T17:35:41.5200412Z Setting up libedit2:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-12T17:35:41.5602760Z Setting up libpipeline1:amd64 (1.4.1-2) ...
2019-07-12T17:35:41.6009596Z Setting up binfmt-support (2.1.6-1) ...
2019-07-12T17:35:41.6695483Z mount: permission denied
2019-07-12T17:35:41.6696353Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-12T17:35:41.6705783Z mount: permission denied
2019-07-12T17:35:41.6709237Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-12T17:35:41.8397430Z invoke-rc.d: could not determine current runlevel
2019-07-12T17:35:41.8423824Z invoke-rc.d: policy-rc.d denied execution of start.
2019-07-12T17:35:41.9052988Z Setting up libisl15:amd64 (0.16.1-1) ...
2019-07-12T17:35:41.9463094Z Setting up cpp-5 (5.4.0-6ubuntu1~16.04.11) ...
2019-07-12T17:35:41.9853449Z Setting up cpp (4:5.3.1-1ubuntu1) ...
2019-07-12T17:35:42.0334074Z Setting up curl (7.47.0-1ubuntu2.13) ...
---
2019-07-12T17:35:43.7725862Z Setting up libedit-dev:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-12T17:35:43.8193292Z Setting up libllvm6.0:amd64 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-12T17:35:43.8638353Z Setting up zlib1g-dev:amd64 (1:1.2.8.dfsg-2ubuntu4.1) ...
2019-07-12T17:35:43.9048648Z Setting up libssl-dev:amd64 (1.0.2g-1ubuntu4.15) ...
2019-07-12T17:35:43.9469065Z Setting up llvm-6.0-runtime (1:6.0-1ubuntu2~16.04.1) ...
2019-07-12T17:35:43.9778059Z mount: permission denied
2019-07-12T17:35:43.9778697Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-12T17:35:43.9928004Z Setting up llvm-6.0 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-12T17:35:44.0406693Z Setting up libffi-dev:amd64 (3.2.1-4) ...
2019-07-12T17:35:44.0828082Z Setting up llvm-6.0-dev (1:6.0-1ubuntu2~16.04.1) ...
2019-07-12T17:35:44.1255752Z Setting up llvm-6.0-tools (1:6.0-1ubuntu2~16.04.1) ...
2019-07-12T17:35:44.1675907Z Setting up pkg-config (0.29.1-0ubuntu1) ...
2019-07-12T17:35:44.3147243Z Processing triggers for ca-certificates (20170717~16.04.2) ...
2019-07-12T17:35:44.3338121Z Updating certificates in /etc/ssl/certs...
2019-07-12T17:35:45.9175093Z 148 added, 0 removed; done.
2019-07-12T17:35:45.9176121Z Running hooks in /etc/ca-certificates/update.d...
---
2019-07-12T17:36:19.1467048Z  ---> 66e818ef8a29
2019-07-12T17:36:19.1507470Z Successfully built 66e818ef8a29
2019-07-12T17:36:19.3270047Z Successfully tagged rust-ci:latest
2019-07-12T17:36:19.4122187Z Built container sha256:66e818ef8a290dbe10ababd5767d687a5a89c58e291b2249d19bace591a7f66d
2019-07-12T17:36:19.4142538Z Uploading finished image to https://.s3.amazonaws.com/docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624
2019-07-12T17:37:21.3237109Z upload failed: - to s3:///docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624 Parameter validation failed:
2019-07-12T17:37:21.3238116Z Invalid bucket name "": Bucket name must match the regex "^[a-zA-Z0-9.\-_]{1,255}$"
2019-07-12T17:37:22.4043948Z [CI_JOB_NAME=x86_64-gnu-llvm-6.0]
2019-07-12T17:37:22.4091782Z Starting sccache server...
2019-07-12T17:37:22.4634642Z configure: processing command line
2019-07-12T17:37:22.4635352Z configure: 
---
2019-07-12T18:34:50.1990777Z .................................................................................................... 200/5799
2019-07-12T18:34:54.4704333Z .................................................................................................... 300/5799
2019-07-12T18:34:58.2223581Z .................................................................................................... 400/5799
2019-07-12T18:35:02.0687189Z .................................................................................................... 500/5799
2019-07-12T18:35:05.9458685Z .......................................................................i............................ 600/5799
2019-07-12T18:35:15.1503718Z .................................................................................................... 800/5799
2019-07-12T18:35:20.7854547Z .................................................................................................... 900/5799
2019-07-12T18:35:20.7854547Z .................................................................................................... 900/5799
2019-07-12T18:35:26.9032176Z ..........................................................................................i......... 1000/5799
2019-07-12T18:35:31.4003174Z ..i................................................................................................. 1100/5799
2019-07-12T18:35:35.7773041Z ....................iiiii........................................................................... 1200/5799
2019-07-12T18:35:41.6491604Z .................................................................................................... 1400/5799
2019-07-12T18:35:44.5262527Z .................................................................................................... 1500/5799
2019-07-12T18:35:48.4390214Z .................................................................................................... 1600/5799
2019-07-12T18:35:51.0166338Z .................................................................................................... 1700/5799
2019-07-12T18:35:51.0166338Z .................................................................................................... 1700/5799
2019-07-12T18:35:54.6343530Z ..........................................................i......................................... 1800/5799
2019-07-12T18:36:03.4406249Z .................................................................................................... 2000/5799
2019-07-12T18:36:07.5297453Z .................................................................................................... 2100/5799
2019-07-12T18:36:11.7207969Z .................................................................................................... 2200/5799
2019-07-12T18:36:11.7207969Z .................................................................................................... 2200/5799
2019-07-12T18:36:15.5105940Z ........................i........................................................................... 2300/5799
2019-07-12T18:36:25.6052916Z .................................................................................................... 2500/5799
2019-07-12T18:36:30.5612974Z .................................................................................................... 2600/5799
2019-07-12T18:36:35.0119104Z .................................................................................................... 2700/5799
2019-07-12T18:36:39.5300192Z .................................................................................................... 2800/5799
2019-07-12T18:36:39.5300192Z .................................................................................................... 2800/5799
2019-07-12T18:36:43.8889824Z .................................................................................................... 2900/5799
2019-07-12T18:36:49.4226898Z .................................................................................................... 3000/5799
2019-07-12T18:36:54.3540041Z .................................................................................................... 3100/5799
2019-07-12T18:36:59.1376333Z .................................................................................................... 3200/5799
2019-07-12T18:37:02.3038734Z .................................................................................................... 3300/5799
2019-07-12T18:37:07.7197656Z .................................................................................................... 3400/5799
2019-07-12T18:37:11.7398313Z ..................................................................................i................. 3500/5799
2019-07-12T18:37:15.6886839Z .................................................................................................... 3600/5799
2019-07-12T18:37:19.7407106Z ........................................................ii...i..ii.................................. 3700/5799
2019-07-12T18:37:29.1946304Z .................................................................................................... 3900/5799
2019-07-12T18:37:29.1946304Z .................................................................................................... 3900/5799
2019-07-12T18:37:33.1634502Z ......................................................................ii............................ 4000/5799
2019-07-12T18:37:36.1529040Z ...........................................................................................i........ 4100/5799
2019-07-12T18:37:38.4885860Z .................................................................................................... 4200/5799
2019-07-12T18:37:40.5739670Z .......................................................i............................................ 4300/5799
2019-07-12T18:37:55.7790474Z .................................................................................................... 4500/5799
2019-07-12T18:38:06.2554661Z .................................................................................................... 4600/5799
2019-07-12T18:38:09.7983130Z .................................................................................................... 4700/5799
2019-07-12T18:38:13.8359715Z .................................................................................................... 4800/5799
---
2019-07-12T18:38:47.5456360Z .................................................................................................... 5400/5799
2019-07-12T18:38:52.3628706Z .................................................................................................... 5500/5799
2019-07-12T18:38:55.5550062Z .................................................................................................... 5600/5799
2019-07-12T18:38:58.7794474Z .................................................................................................... 5700/5799
2019-07-12T18:39:02.0671366Z .......................................i...........................................................
2019-07-12T18:39:02.0673335Z 
2019-07-12T18:39:02.0742777Z  finished in 268.117
2019-07-12T18:39:02.0949482Z Check compiletest suite=run-pass mode=run-pass (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-07-12T18:39:02.3250250Z 
2019-07-12T18:39:02.3250250Z 
2019-07-12T18:39:02.3251177Z running 2917 tests
2019-07-12T18:39:17.4044067Z .................................................................................................... 100/2917
2019-07-12T18:39:33.2301086Z ............................................................................i....................... 200/2917
2019-07-12T18:39:58.5552187Z .................................................................................................... 400/2917
2019-07-12T18:40:10.4466180Z .................................................................................................... 500/2917
2019-07-12T18:40:25.2829633Z .................................................................................................... 600/2917
2019-07-12T18:40:47.2823240Z .................................................................................................... 700/2917
2019-07-12T18:40:47.2823240Z .................................................................................................... 700/2917
2019-07-12T18:41:01.7514585Z .................................................................................................... 800/2917
2019-07-12T18:41:14.1566252Z .................................................................................................... 900/2917
2019-07-12T18:41:31.1001977Z .................................................................................................... 1000/2917
2019-07-12T18:41:45.6088032Z .................................................................................................... 1100/2917
2019-07-12T18:41:57.3518198Z .................................................................................................... 1200/2917
2019-07-12T18:42:10.3538138Z .................................................................................................... 1300/2917
2019-07-12T18:42:27.2446172Z .....................ii............................................................................. 1400/2917
2019-07-12T18:42:40.9487371Z .................................................................................................... 1500/2917
2019-07-12T18:42:53.1934107Z .........................................................................i.......i.................. 1600/2917
2019-07-12T18:43:27.7518431Z .................................................................................................... 1800/2917
2019-07-12T18:43:42.2751458Z .................................................................................................... 1900/2917
2019-07-12T18:43:42.2751458Z .................................................................................................... 1900/2917
2019-07-12T18:44:00.0226900Z ...i.......................................................................i........................ 2000/2917
2019-07-12T18:44:49.1425402Z .................................................................................................... 2200/2917
2019-07-12T18:45:01.3611472Z .................................................................................................... 2300/2917
2019-07-12T18:45:01.3611472Z .................................................................................................... 2300/2917
2019-07-12T18:45:23.0992875Z ..........ii........................................................................................ 2400/2917
2019-07-12T18:46:19.8454336Z .................................................................................................... 2600/2917
2019-07-12T18:46:33.5302439Z .................................................................................................... 2700/2917
2019-07-12T18:46:45.4697959Z .................................................................................................... 2800/2917
2019-07-12T18:47:01.0554923Z .................................................................................................... 2900/2917
---
2019-07-12T18:47:58.2273795Z  finished in 31.994
2019-07-12T18:47:58.2442259Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-07-12T18:47:58.4093935Z 
2019-07-12T18:47:58.4094694Z running 146 tests
2019-07-12T18:48:01.7048751Z i....iii......iii..iiii....i............................i..i................i....i.........ii.i.i..i 100/146
2019-07-12T18:48:03.6196741Z .ii..............i.........iii.i......ii......
2019-07-12T18:48:03.6197707Z 
2019-07-12T18:48:03.6200310Z  finished in 5.375
2019-07-12T18:48:03.6372493Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-07-12T18:48:03.8004823Z 
2019-07-12T18:48:03.8004823Z 
2019-07-12T18:48:04.5561012Z running 39 tests
2019-07-12T18:48:05.8801744Z i.........i.......................i....
2019-07-12T18:48:05.8803593Z 
2019-07-12T18:48:05.8804690Z  finished in 2.243
2019-07-12T18:48:05.8981492Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-07-12T18:48:06.0570863Z 
2019-07-12T18:48:06.0570863Z 
2019-07-12T18:48:06.0571432Z running 9 tests
2019-07-12T18:48:06.0574103Z iiiiiiiii
2019-07-12T18:48:06.0574979Z 
2019-07-12T18:48:06.0580294Z  finished in 0.159
2019-07-12T18:48:06.0771344Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-07-12T18:48:06.2445621Z 
---
2019-07-12T18:48:24.7015546Z  finished in 18.624
2019-07-12T18:48:24.7201189Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-07-12T18:48:24.8817943Z 
2019-07-12T18:48:24.8818685Z running 123 tests
2019-07-12T18:48:49.2518199Z .iiiii...i.....i..i...i..i.i.i..i.iiF.i.i.....i..i....i..........iiii..........i..FiiF...i.......ii. 100/123
2019-07-12T18:48:53.9776476Z i.i.i......iii.i.....ii
2019-07-12T18:48:53.9777278Z 
2019-07-12T18:48:53.9777800Z ---- [debuginfo-gdb+lldb] debuginfo/gdb-pretty-struct-and-enums.rs stdout ----
2019-07-12T18:48:53.9777800Z ---- [debuginfo-gdb+lldb] debuginfo/gdb-pretty-struct-and-enums.rs stdout ----
2019-07-12T18:48:53.9778012Z NOTE: compiletest thinks it is using GDB without native rust support
2019-07-12T18:48:53.9778222Z NOTE: compiletest thinks it is using GDB version 7011001
2019-07-12T18:48:53.9778354Z 
2019-07-12T18:48:53.9778516Z error: line not found in debugger output: $1 = RegularStruct = {the_first_field = 101, the_second_field = 102.5, the_third_field = false}
2019-07-12T18:48:53.9778685Z status: exit code: 0
2019-07-12T18:48:53.9779250Z command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/gdb-pretty-struct-and-enums/gdb-pretty-struct-and-enums.debugger.script"
2019-07-12T18:48:53.9780265Z ------------------------------------------
2019-07-12T18:48:53.9780265Z ------------------------------------------
2019-07-12T18:48:53.9780767Z GNU gdb (Ubuntu 7.11.1-0ubuntu1~16.5) 7.11.1
2019-07-12T18:48:53.9780965Z Copyright (C) 2016 Free Software Foundation, Inc.
2019-07-12T18:48:53.9781166Z License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
2019-07-12T18:48:53.9782688Z This is free software: you are free to change and redistribute it.
2019-07-12T18:48:53.9783485Z There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
2019-07-12T18:48:53.9783762Z and "show warranty" for details.
2019-07-12T18:48:53.9784235Z This GDB was configured as "x86_64-linux-gnu".
2019-07-12T18:48:53.9784460Z Type "show configuration" for configuration details.
2019-07-12T18:48:53.9784602Z For bug reporting instructions, please see:
2019-07-12T18:48:53.9784756Z <http://www.gnu.org/software/gdb/bugs/>.
2019-07-12T18:48:53.9784918Z Find the GDB manual and other documentation resources online at:
2019-07-12T18:48:53.9785058Z <http://www.gnu.org/software/gdb/documentation/>.
2019-07-12T18:48:53.9785195Z For help, type "help".
2019-07-12T18:48:53.9785352Z Type "apropos word" to search for commands related to "word".
2019-07-12T18:48:53.9785791Z Breakpoint 1 at 0xaaf: file /checkout/src/test/debuginfo/gdb-pretty-struct-and-enums.rs, line 60.
2019-07-12T18:48:53.9786001Z [Thread debugging using libthread_db enabled]
2019-07-12T18:48:53.9786417Z Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
2019-07-12T18:48:53.9786583Z 
2019-07-12T18:48:53.9787061Z Breakpoint 1, gdb_pretty_struct_and_enums::main::hb77f1d849dda9a93 () at /checkout/src/test/debuginfo/gdb-pretty-struct-and-enums.rs:60
2019-07-12T18:48:53.9787252Z 60     zzz(); // #break
2019-07-12T18:48:53.9787393Z $1 = {the_first_field = 101, the_second_field = 102.5, the_third_field = false}
2019-07-12T18:48:53.9787917Z $2 = {<No data fields>}
2019-07-12T18:48:53.9788057Z $3 = CStyleEnumVar1
2019-07-12T18:48:53.9788189Z $4 = CStyleEnumVar2
2019-07-12T18:48:53.9788341Z $5 = CStyleEnumVar3
2019-07-12T18:48:53.9788476Z A debugging session is active.
2019-07-12T18:48:53.9788593Z 
2019-07-12T18:48:53.9788745Z  Inferior 1 [process 114942] will be killed.
2019-07-12T18:48:53.9788863Z 
2019-07-12T18:48:53.9789000Z Quit anyway? (y or n) [answered Y; input not from terminal]
2019-07-12T18:48:53.9790129Z ------------------------------------------
2019-07-12T18:48:53.9790360Z stderr:
2019-07-12T18:48:53.9790789Z ------------------------------------------
2019-07-12T18:48:53.9790961Z 
2019-07-12T18:48:53.9790961Z 
2019-07-12T18:48:53.9791349Z ------------------------------------------
2019-07-12T18:48:53.9791512Z 
2019-07-12T18:48:53.9791657Z 
2019-07-12T18:48:53.9792071Z ---- [debuginfo-gdb+lldb] debuginfo/pretty-huge-vec.rs stdout ----
2019-07-12T18:48:53.9792280Z NOTE: compiletest thinks it is using GDB without native rust support
2019-07-12T18:48:53.9792465Z NOTE: compiletest thinks it is using GDB version 7011001
2019-07-12T18:48:53.9792591Z 
2019-07-12T18:48:53.9792739Z error: line not found in debugger output: $1 = Vec<u8>(len: 1000000000, cap: 1000000000) = {[...]...}
2019-07-12T18:48:53.9792907Z status: exit code: 0
2019-07-12T18:48:53.9793646Z command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/pretty-huge-vec/pretty-huge-vec.debugger.script"
2019-07-12T18:48:53.9794232Z ------------------------------------------
2019-07-12T18:48:53.9794232Z ------------------------------------------
2019-07-12T18:48:53.9794715Z GNU gdb (Ubuntu 7.11.1-0ubuntu1~16.5) 7.11.1
2019-07-12T18:48:53.9794911Z Copyright (C) 2016 Free Software Foundation, Inc.
2019-07-12T18:48:53.9795057Z License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
2019-07-12T18:48:53.9795197Z This is free software: you are free to change and redistribute it.
2019-07-12T18:48:53.9795369Z There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
2019-07-12T18:48:53.9795507Z and "show warranty" for details.
2019-07-12T18:48:53.9795881Z This GDB was configured as "x86_64-linux-gnu".
2019-07-12T18:48:53.9796079Z Type "show configuration" for configuration details.
2019-07-12T18:48:53.9796218Z For bug reporting instructions, please see:
2019-07-12T18:48:53.9796353Z <http://www.gnu.org/software/gdb/bugs/>.
2019-07-12T18:48:53.9796514Z Find the GDB manual and other documentation resources online at:
2019-07-12T18:48:53.9796652Z <http://www.gnu.org/software/gdb/documentation/>.
2019-07-12T18:48:53.9796801Z For help, type "help".
2019-07-12T18:48:53.9797044Z Type "apropos word" to search for commands related to "word".
2019-07-12T18:48:53.9797528Z Breakpoint 1 at 0x18ec: file /checkout/src/test/debuginfo/pretty-huge-vec.rs, line 28.
2019-07-12T18:48:53.9797743Z [Thread debugging using libthread_db enabled]
2019-07-12T18:48:53.9798141Z Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
2019-07-12T18:48:53.9798314Z 
2019-07-12T18:48:53.9798765Z Breakpoint 1, pretty_huge_vec::main::h724d3ae1bb5250fc () at /checkout/src/test/debuginfo/pretty-huge-vec.rs:28
2019-07-12T18:48:53.9798948Z 28     zzz(); // #break
2019-07-12T18:48:53.9802043Z $1 = size=1000000000 = {0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000', 0 '\000'...}
2019-07-12T18:48:53.9803544Z $2 = {data_ptr = 0x7fffbb526010 "", length = 1000000000}
2019-07-12T18:48:53.9803598Z A debugging session is active.
2019-07-12T18:48:53.9803650Z 
2019-07-12T18:48:53.9803692Z  Inferior 1 [process 115648] will be killed.
2019-07-12T18:48:53.9803721Z 
2019-07-12T18:48:53.9803765Z Quit anyway? (y or n) [answered Y; input not from terminal]
2019-07-12T18:48:53.9804166Z ------------------------------------------
2019-07-12T18:48:53.9804217Z stderr:
2019-07-12T18:48:53.9804488Z ------------------------------------------
2019-07-12T18:48:53.9804521Z 
2019-07-12T18:48:53.9804521Z 
2019-07-12T18:48:53.9804768Z ------------------------------------------
2019-07-12T18:48:53.9804800Z 
2019-07-12T18:48:53.9804834Z 
2019-07-12T18:48:53.9805132Z ---- [debuginfo-gdb+lldb] debuginfo/pretty-uninitialized-vec.rs stdout ----
2019-07-12T18:48:53.9805190Z NOTE: compiletest thinks it is using GDB without native rust support
2019-07-12T18:48:53.9805240Z NOTE: compiletest thinks it is using GDB version 7011001
2019-07-12T18:48:53.9805286Z 
2019-07-12T18:48:53.9805333Z error: line not found in debugger output: $1 = Vec<i32>(len: [...], cap: [...])[...]
2019-07-12T18:48:53.9805380Z status: exit code: 0
2019-07-12T18:48:53.9805898Z command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/pretty-uninitialized-vec/pretty-uninitialized-vec.debugger.script"
2019-07-12T18:48:53.9806261Z ------------------------------------------
2019-07-12T18:48:53.9806261Z ------------------------------------------
2019-07-12T18:48:53.9806544Z GNU gdb (Ubuntu 7.11.1-0ubuntu1~16.5) 7.11.1
2019-07-12T18:48:53.9806596Z Copyright (C) 2016 Free Software Foundation, Inc.
2019-07-12T18:48:53.9806648Z License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
2019-07-12T18:48:53.9806726Z This is free software: you are free to change and redistribute it.
2019-07-12T18:48:53.9806776Z There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
2019-07-12T18:48:53.9806822Z and "show warranty" for details.
2019-07-12T18:48:53.9807103Z This GDB was configured as "x86_64-linux-gnu".
2019-07-12T18:48:53.9807157Z Type "show configuration" for configuration details.
2019-07-12T18:48:53.9807204Z For bug reporting instructions, please see:
2019-07-12T18:48:53.9807267Z <http://www.gnu.org/software/gdb/bugs/>.
2019-07-12T18:48:53.9807324Z Find the GDB manual and other documentation resources online at:
2019-07-12T18:48:53.9807373Z <http://www.gnu.org/software/gdb/documentation/>.
2019-07-12T18:48:53.9807417Z For help, type "help".
2019-07-12T18:48:53.9807481Z Type "apropos word" to search for commands related to "word".
2019-07-12T18:48:53.9807794Z Breakpoint 1 at 0x1010: file /checkout/src/test/debuginfo/pretty-uninitialized-vec.rs, line 21.
2019-07-12T18:48:53.9807939Z [Thread debugging using libthread_db enabled]
2019-07-12T18:48:53.9808270Z Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
2019-07-12T18:48:53.9808307Z 
2019-07-12T18:48:53.9808638Z Breakpoint 1, pretty_uninitialized_vec::main::h8ad4cf0b9701f4b8 () at /checkout/src/test/debuginfo/pretty-uninitialized-vec.rs:21
2019-07-12T18:48:53.9808710Z 21     zzz(); // #break
2019-07-12T18:48:53.9808825Z $1 = size=140737488348608 = {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0...}
2019-07-12T18:48:53.9808938Z A debugging session is active.
2019-07-12T18:48:53.9808969Z 
2019-07-12T18:48:53.9809013Z  Inferior 1 [process 115677] will be killed.
2019-07-12T18:48:53.9809060Z 
2019-07-12T18:48:53.9809106Z Quit anyway? (y or n) [answered Y; input not from terminal]
2019-07-12T18:48:53.9809394Z ------------------------------------------
2019-07-12T18:48:53.9809457Z stderr:
2019-07-12T18:48:53.9809702Z ------------------------------------------
2019-07-12T18:48:53.9809734Z 
---
2019-07-12T18:48:53.9811314Z test result: FAILED. 81 passed; 3 failed; 39 ignored; 0 measured; 0 filtered out
2019-07-12T18:48:53.9811352Z 
2019-07-12T18:48:53.9811380Z 
2019-07-12T18:48:53.9811404Z 
2019-07-12T18:48:53.9813194Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/debuginfo" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "debuginfo-gdb+lldb" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-07-12T18:48:53.9813657Z 
2019-07-12T18:48:53.9813701Z 
2019-07-12T18:48:53.9814208Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:535:22
2019-07-12T18:48:53.9814270Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-07-12T18:48:53.9814270Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-07-12T18:48:53.9814342Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-07-12T18:48:53.9814489Z Build completed unsuccessfully in 1:07:40
2019-07-12T18:48:54.8328032Z ##[error]Bash exited with code '1'.
2019-07-12T18:48:54.8360659Z ##[section]Starting: Checkout
2019-07-12T18:48:54.8362264Z ==============================================================================
2019-07-12T18:48:54.8362320Z Task         : Get sources
2019-07-12T18:48:54.8362369Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
