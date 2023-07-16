plain
2019-07-16T20:56:01.5970484Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-16T20:56:01.6154886Z ##[command]git config gc.auto 0
2019-07-16T20:56:01.6221438Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-16T20:56:01.6271898Z ##[command]git config --get-all http.proxy
2019-07-16T20:56:01.6407338Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62732/merge:refs/remotes/pull/62732/merge
---
2019-07-16T20:56:36.9787777Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-16T20:56:36.9788002Z 
2019-07-16T20:56:36.9788261Z   git checkout -b <new-branch-name>
2019-07-16T20:56:36.9788291Z 
2019-07-16T20:56:36.9788339Z HEAD is now at 1e66228b8 Merge 658876ae76aef4295cb5226636eb3929ec1fe9ee into 96234d5363286700794973c36178c3df1d9d49d6
2019-07-16T20:56:36.9925356Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-16T20:56:36.9928430Z ==============================================================================
2019-07-16T20:56:36.9928509Z Task         : Bash
2019-07-16T20:56:36.9928559Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-16T20:58:13.2915135Z Attempting with retry: curl -y 30 -Y 10 --connect-timeout 30 -f -L -C - -o /tmp/rustci_docker_cache https://.s3.amazonaws.com/docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624
2019-07-16T20:58:13.2983517Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-16T20:58:13.2983964Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-16T20:58:13.2984197Z 
2019-07-16T20:58:13.2984915Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-16T20:58:14.3049805Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-16T20:58:14.3061537Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-16T20:58:14.3061605Z 
2019-07-16T20:58:14.3061605Z 
2019-07-16T20:58:14.3094313Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-16T20:58:16.3162790Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-16T20:58:16.3174469Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-16T20:58:16.3174577Z 
2019-07-16T20:58:16.3174577Z 
2019-07-16T20:58:16.3207636Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-16T20:58:19.3301688Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-16T20:58:19.3301893Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-16T20:58:19.3301950Z 
2019-07-16T20:58:19.3301950Z 
2019-07-16T20:58:19.3325024Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-16T20:58:23.3395988Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-16T20:58:23.3396975Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-16T20:58:23.3397129Z 
2019-07-16T20:58:23.3397129Z 
2019-07-16T20:58:23.3441332Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-16T20:58:23.3449264Z The command has failed after 5 attempts.
2019-07-16T20:58:23.3697764Z open /tmp/rustci_docker_cache: no such file or directory
2019-07-16T20:58:23.3724158Z Attempting with retry: docker build --rm -t rust-ci -f /home/vsts/work/1/s/src/ci/docker/x86_64-gnu-llvm-6.0/Dockerfile /home/vsts/work/1/s/src/ci/docker
2019-07-16T20:58:23.5648307Z Sending build context to Docker daemon  521.2kB
2019-07-16T20:58:23.5648490Z 
2019-07-16T20:58:23.5849969Z Step 1/8 : FROM ubuntu:16.04
---
2019-07-16T20:58:39.4141798Z Reading package lists...
2019-07-16T20:58:40.4092923Z Reading package lists...
2019-07-16T20:58:40.5937446Z Building dependency tree...
2019-07-16T20:58:40.5937888Z Reading state information...
2019-07-16T20:58:40.7086714Z The following additional packages will be installed:
2019-07-16T20:58:40.7087423Z   binfmt-support binutils bzip2 cmake-data cpp cpp-5 dpkg-dev g++-5 gcc gcc-5
2019-07-16T20:58:40.7087723Z   git-man libarchive13 libasan2 libasn1-8-heimdal libatomic1
2019-07-16T20:58:40.7087955Z   libbabeltrace-ctf1 libbabeltrace1 libbsd-dev libbsd0 libbz2-1.0 libc-dev-bin
2019-07-16T20:58:40.7093126Z   libedit2 liberror-perl libexpat1 libffi-dev libffi6 libgcc-5-dev libgdbm3
2019-07-16T20:58:40.7093647Z   libglib2.0-0 libgmp10 libgnutls30 libgomp1 libgssapi-krb5-2
2019-07-16T20:58:40.7093832Z   libgssapi3-heimdal libhcrypto4-heimdal libheimbase1-heimdal
2019-07-16T20:58:40.7094118Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-16T20:58:40.7094118Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-16T20:58:40.7094318Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-16T20:58:40.7094527Z   libkrb5-3 libkrb5support0 libldap-2.4-2 libllvm6.0 liblsan0 liblzo2-2
2019-07-16T20:58:40.7094762Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-16T20:58:40.7094947Z   libperl5.22 libpipeline1 libpython-stdlib libpython2.7-minimal
2019-07-16T20:58:40.7095141Z   libpython2.7-stdlib libpython3.5 libpython3.5-minimal libpython3.5-stdlib
2019-07-16T20:58:40.7095378Z   libquadmath0 libroken18-heimdal librtmp1 libsasl2-2 libsasl2-modules-db
2019-07-16T20:58:40.7095570Z   libsqlite3-0 libssl1.0.0 libstdc++-5-dev libtasn1-6 libtinfo-dev libtsan0
2019-07-16T20:58:40.7095779Z   libubsan0 libwind0-heimdal libxml2 linux-libc-dev llvm-6.0 llvm-6.0-dev
2019-07-16T20:58:40.7096015Z   llvm-6.0-runtime mime-support openssl patch perl perl-modules-5.22 python
2019-07-16T20:58:40.7096185Z   python-minimal python2.7-minimal
2019-07-16T20:58:40.7096345Z Suggested packages:
2019-07-16T20:58:40.7096566Z   binutils-doc bzip2-doc codeblocks eclipse ninja-build cpp-doc gcc-5-locales
2019-07-16T20:58:40.7097063Z   debian-keyring g++-multilib g++-5-multilib gcc-5-doc libstdc++6-5-dbg
2019-07-16T20:58:40.7097253Z   gcc-multilib manpages-dev autoconf automake libtool flex bison gcc-doc
2019-07-16T20:58:40.7097866Z   libasan2-dbg liblsan0-dbg libtsan0-dbg libubsan0-dbg libcilkrts5-dbg
2019-07-16T20:58:40.7098057Z   libmpx0-dbg libquadmath0-dbg gdb-doc gettext-base git-daemon-run
2019-07-16T20:58:40.7098057Z   libmpx0-dbg libquadmath0-dbg gdb-doc gettext-base git-daemon-run
2019-07-16T20:58:40.7098302Z   | git-daemon-sysvinit git-doc git-el git-email git-gui gitk gitweb git-arch
2019-07-16T20:58:40.7098501Z   git-cvs git-mediawiki git-svn lrzip glibc-doc gnutls-bin krb5-doc krb5-user
2019-07-16T20:58:40.7098693Z   libstdc++-5-doc llvm-6.0-doc make-doc ed diffutils-doc perl-doc
2019-07-16T20:58:40.7098945Z   libterm-readline-gnu-perl | libterm-readline-perl-perl python-doc python-tk
2019-07-16T20:58:40.7099099Z   python2.7-doc
2019-07-16T20:58:40.7099145Z Recommended packages:
2019-07-16T20:58:40.7099563Z   build-essential fakeroot libalgorithm-merge-perl libc-dbg gdbserver less
2019-07-16T20:58:40.7099867Z   rsync ssh-client manpages manpages-dev libfile-fcntllock-perl
2019-07-16T20:58:40.7100304Z   libglib2.0-data shared-mime-info xdg-user-dirs krb5-locales libsasl2-modules
2019-07-16T20:58:40.7100572Z   libssl-doc xml-core netbase rename
2019-07-16T20:58:40.7114697Z The following NEW packages will be installed:
2019-07-16T20:58:40.7115109Z   binfmt-support binutils bzip2 ca-certificates cmake cmake-data cpp cpp-5
2019-07-16T20:58:40.7115385Z   curl dpkg-dev file g++ g++-5 gcc gcc-5 gdb git git-man libarchive13 libasan2
2019-07-16T20:58:40.7115603Z   libasn1-8-heimdal libatomic1 libbabeltrace-ctf1 libbabeltrace1 libbsd-dev
2019-07-16T20:58:40.7115963Z   libbsd0 libc-dev-bin libc6-dev libcc1-0 libcilkrts5 libcurl3 libcurl3-gnutls
2019-07-16T20:58:40.7116258Z   libdpkg-perl libedit-dev libedit2 liberror-perl libexpat1 libffi-dev libffi6
2019-07-16T20:58:40.7116477Z   libgcc-5-dev libgdbm3 libglib2.0-0 libgmp10 libgnutls30 libgomp1
2019-07-16T20:58:40.7116916Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-16T20:58:40.7117127Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-16T20:58:40.7117326Z   libkrb5-3 libkrb5support0 libldap-2.4-2 libllvm6.0 liblsan0 liblzo2-2
2019-07-16T20:58:40.7117745Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-16T20:58:40.7117745Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-16T20:58:40.7125930Z   libperl5.22 libpipeline1 libpython-stdlib libpython2.7-minimal
2019-07-16T20:58:40.7126301Z   libpython2.7-stdlib libpython3.5 libpython3.5-minimal libpython3.5-stdlib
2019-07-16T20:58:40.7126575Z   libquadmath0 libroken18-heimdal librtmp1 libsasl2-2 libsasl2-modules-db
2019-07-16T20:58:40.7126794Z   libsqlite3-0 libssl-dev libssl1.0.0 libstdc++-5-dev libtasn1-6 libtinfo-dev
2019-07-16T20:58:40.7127000Z   libtsan0 libubsan0 libwind0-heimdal libxml2 linux-libc-dev llvm-6.0
2019-07-16T20:58:40.7127237Z   llvm-6.0-dev llvm-6.0-runtime llvm-6.0-tools make mime-support openssl patch
2019-07-16T20:58:40.7127490Z   perl perl-modules-5.22 pkg-config python python-minimal python2.7
2019-07-16T20:58:40.7127670Z   python2.7-minimal sudo xz-utils zlib1g-dev
2019-07-16T20:58:40.7127750Z The following packages will be upgraded:
2019-07-16T20:58:41.1217724Z 1 upgraded, 115 newly installed, 0 to remove and 5 not upgraded.
2019-07-16T20:58:41.1217935Z Need to get 121 MB of archives.
2019-07-16T20:58:41.1217981Z After this operation, 592 MB of additional disk space will be used.
2019-07-16T20:58:41.1218658Z Get:1 http://archive.ubuntu.com/ubuntu xenial/main amd64 libgdbm3 amd64 1.8.3-13.1 [16.9 kB]
---
2019-07-16T20:58:43.6576523Z Get:61 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 sudo amd64 1.8.16-0ubuntu1.7 [390 kB]
2019-07-16T20:58:43.7273207Z Get:62 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 openssl amd64 1.0.2g-1ubuntu4.15 [492 kB]
2019-07-16T20:58:43.7351091Z Get:63 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 ca-certificates all 20170717~16.04.2 [167 kB]
2019-07-16T20:58:43.7377304Z Get:64 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 libcurl3-gnutls amd64 7.47.0-1ubuntu2.13 [184 kB]
2019-07-16T20:58:43.7406259Z Get:65 http://archive.ubuntu.com/ubuntu xenial/main amd64 libedit2 amd64 3.1-20150325-1ubuntu2 [76.5 kB]
2019-07-16T20:58:43.7421439Z Get:66 http://archive.ubuntu.com/ubuntu xenial/main amd64 libpipeline1 amd64 1.4.1-2 [24.6 kB]
2019-07-16T20:58:43.7428672Z Get:67 http://archive.ubuntu.com/ubuntu xenial/main amd64 binfmt-support amd64 2.1.6-1 [50.7 kB]
2019-07-16T20:58:43.8714783Z Get:69 http://archive.ubuntu.com/ubuntu xenial/main amd64 libisl15 amd64 0.16.1-1 [524 kB]
2019-07-16T20:58:43.8807700Z Get:70 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 cpp-5 amd64 5.4.0-6ubuntu1~16.04.11 [7660 kB]
2019-07-16T20:58:44.2038486Z Get:71 http://archive.ubuntu.com/ubuntu xenial/main amd64 cpp amd64 4:5.3.1-1ubuntu1 [27.7 kB]
2019-07-16T20:58:44.2041267Z Get:72 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 curl amd64 7.47.0-1ubuntu2.13 [139 kB]
---
2019-07-16T20:59:05.3075701Z Unpacking ca-certificates (20170717~16.04.2) ...
2019-07-16T20:59:05.4581883Z Selecting previously unselected package libcurl3-gnutls:amd64.
2019-07-16T20:59:05.4597800Z Preparing to unpack .../libcurl3-gnutls_7.47.0-1ubuntu2.13_amd64.deb ...
2019-07-16T20:59:05.4716016Z Unpacking libcurl3-gnutls:amd64 (7.47.0-1ubuntu2.13) ...
2019-07-16T20:59:05.6020475Z Selecting previously unselected package libedit2:amd64.
2019-07-16T20:59:05.6040957Z Preparing to unpack .../libedit2_3.1-20150325-1ubuntu2_amd64.deb ...
2019-07-16T20:59:05.6172798Z Unpacking libedit2:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-16T20:59:05.7305883Z Selecting previously unselected package libpipeline1:amd64.
2019-07-16T20:59:05.7320926Z Preparing to unpack .../libpipeline1_1.4.1-2_amd64.deb ...
2019-07-16T20:59:05.7438673Z Unpacking libpipeline1:amd64 (1.4.1-2) ...
2019-07-16T20:59:05.8536427Z Selecting previously unselected package binfmt-support.
2019-07-16T20:59:05.8552699Z Preparing to unpack .../binfmt-support_2.1.6-1_amd64.deb ...
2019-07-16T20:59:05.8684898Z Unpacking binfmt-support (2.1.6-1) ...
2019-07-16T20:59:05.9833140Z Preparing to unpack .../binutils_2.26.1-1ubuntu1~16.04.8_amd64.deb ...
2019-07-16T20:59:05.9954412Z Unpacking binutils (2.26.1-1ubuntu1~16.04.8) ...
2019-07-16T20:59:06.4749214Z Selecting previously unselected package libisl15:amd64.
2019-07-16T20:59:06.4767473Z Preparing to unpack .../libisl15_0.16.1-1_amd64.deb ...
---
2019-07-16T20:59:20.8281935Z Unpacking zlib1g-dev:amd64 (1:1.2.8.dfsg-2ubuntu4.1) ...
2019-07-16T20:59:20.9357165Z Selecting previously unselected package libssl-dev:amd64.
2019-07-16T20:59:20.9375525Z Preparing to unpack .../libssl-dev_1.0.2g-1ubuntu4.15_amd64.deb ...
2019-07-16T20:59:20.9499025Z Unpacking libssl-dev:amd64 (1.0.2g-1ubuntu4.15) ...
2019-07-16T20:59:21.2572562Z Selecting previously unselected package llvm-6.0-runtime.
2019-07-16T20:59:21.2594198Z Preparing to unpack .../llvm-6.0-runtime_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-16T20:59:21.2726861Z Unpacking llvm-6.0-runtime (1:6.0-1ubuntu2~16.04.1) ...
2019-07-16T20:59:21.3927306Z Selecting previously unselected package llvm-6.0.
2019-07-16T20:59:21.3945144Z Preparing to unpack .../llvm-6.0_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-16T20:59:21.4076680Z Unpacking llvm-6.0 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-16T20:59:22.0632817Z Selecting previously unselected package libffi-dev:amd64.
2019-07-16T20:59:22.0650770Z Preparing to unpack .../libffi-dev_3.2.1-4_amd64.deb ...
2019-07-16T20:59:22.0771513Z Unpacking libffi-dev:amd64 (3.2.1-4) ...
2019-07-16T20:59:22.1950688Z Selecting previously unselected package llvm-6.0-dev.
2019-07-16T20:59:22.1965423Z Preparing to unpack .../llvm-6.0-dev_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-16T20:59:22.2113791Z Unpacking llvm-6.0-dev (1:6.0-1ubuntu2~16.04.1) ...
2019-07-16T20:59:26.7597042Z Selecting previously unselected package llvm-6.0-tools.
2019-07-16T20:59:26.7622101Z Preparing to unpack .../llvm-6.0-tools_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-16T20:59:26.7753539Z Unpacking llvm-6.0-tools (1:6.0-1ubuntu2~16.04.1) ...
2019-07-16T20:59:26.9165100Z Selecting previously unselected package pkg-config.
2019-07-16T20:59:26.9183286Z Preparing to unpack .../pkg-config_0.29.1-0ubuntu1_amd64.deb ...
2019-07-16T20:59:26.9867401Z Processing triggers for libc-bin (2.23-0ubuntu11) ...
2019-07-16T20:59:27.0428154Z Processing triggers for systemd (229-4ubuntu21.21) ...
2019-07-16T20:59:27.3747958Z Setting up libgdbm3:amd64 (1.8.3-13.1) ...
2019-07-16T20:59:27.4427654Z Setting up libffi6:amd64 (3.2.1-4) ...
---
2019-07-16T20:59:31.2895042Z debconf: unable to initialize frontend: Dialog
2019-07-16T20:59:31.2895229Z debconf: (TERM is not set, so the dialog frontend is not usable.)
2019-07-16T20:59:31.2895320Z debconf: falling back to frontend: Readline
2019-07-16T20:59:31.7959867Z Setting up libcurl3-gnutls:amd64 (7.47.0-1ubuntu2.13) ...
2019-07-16T20:59:31.8319499Z Setting up libedit2:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-16T20:59:31.8761844Z Setting up libpipeline1:amd64 (1.4.1-2) ...
2019-07-16T20:59:31.9176630Z Setting up binfmt-support (2.1.6-1) ...
2019-07-16T20:59:31.9853083Z mount: permission denied
2019-07-16T20:59:31.9857781Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-16T20:59:31.9872277Z mount: permission denied
2019-07-16T20:59:31.9876877Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-16T20:59:32.1474296Z invoke-rc.d: could not determine current runlevel
2019-07-16T20:59:32.1511555Z invoke-rc.d: policy-rc.d denied execution of start.
2019-07-16T20:59:32.2054779Z Setting up libisl15:amd64 (0.16.1-1) ...
2019-07-16T20:59:32.2475559Z Setting up cpp-5 (5.4.0-6ubuntu1~16.04.11) ...
2019-07-16T20:59:32.2848574Z Setting up cpp (4:5.3.1-1ubuntu1) ...
2019-07-16T20:59:32.3380531Z Setting up curl (7.47.0-1ubuntu2.13) ...
---
2019-07-16T20:59:33.9870506Z Setting up libedit-dev:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-16T20:59:34.0275853Z Setting up libllvm6.0:amd64 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-16T20:59:34.0710060Z Setting up zlib1g-dev:amd64 (1:1.2.8.dfsg-2ubuntu4.1) ...
2019-07-16T20:59:34.1108609Z Setting up libssl-dev:amd64 (1.0.2g-1ubuntu4.15) ...
2019-07-16T20:59:34.1592997Z Setting up llvm-6.0-runtime (1:6.0-1ubuntu2~16.04.1) ...
2019-07-16T20:59:34.1887391Z mount: permission denied
2019-07-16T20:59:34.1888660Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-16T20:59:34.2042590Z Setting up llvm-6.0 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-16T20:59:34.2386466Z Setting up libffi-dev:amd64 (3.2.1-4) ...
2019-07-16T20:59:34.2755323Z Setting up llvm-6.0-dev (1:6.0-1ubuntu2~16.04.1) ...
2019-07-16T20:59:34.3127229Z Setting up llvm-6.0-tools (1:6.0-1ubuntu2~16.04.1) ...
2019-07-16T20:59:34.3481606Z Setting up pkg-config (0.29.1-0ubuntu1) ...
2019-07-16T20:59:34.4708910Z Processing triggers for ca-certificates (20170717~16.04.2) ...
2019-07-16T20:59:34.4916911Z Updating certificates in /etc/ssl/certs...
2019-07-16T20:59:36.0372955Z 148 added, 0 removed; done.
2019-07-16T20:59:36.0373760Z Running hooks in /etc/ca-certificates/update.d...
---
2019-07-16T21:00:09.4711659Z Removing intermediate container a01cdfc13a29
2019-07-16T21:00:09.4712520Z  ---> 3ba176a4d82a
2019-07-16T21:00:09.4752428Z Successfully built 3ba176a4d82a
2019-07-16T21:00:09.6682443Z Successfully tagged rust-ci:latest
2019-07-16T21:00:09.7244025Z Built container sha256:3ba176a4d82a8a3cb41ad2a0d5ab76125927c614edb5269cdd759a3bef13b9de
2019-07-16T21:00:09.7257462Z Uploading finished image to https://.s3.amazonaws.com/docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624
2019-07-16T21:01:11.4904468Z upload failed: - to s3:///docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624 Parameter validation failed:
2019-07-16T21:01:11.4908999Z Invalid bucket name "": Bucket name must match the regex "^[a-zA-Z0-9.\-_]{1,255}$"
2019-07-16T21:01:12.5856860Z [CI_JOB_NAME=x86_64-gnu-llvm-6.0]
2019-07-16T21:01:12.5906435Z Starting sccache server...
2019-07-16T21:01:13.5258318Z configure: processing command line
2019-07-16T21:01:13.5260848Z configure: 
---
2019-07-16T21:04:33.9076866Z    Compiling serde_json v1.0.33
2019-07-16T21:04:38.2110339Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-07-16T21:04:46.8134381Z     Finished release [optimized] target(s) in 1m 28s
2019-07-16T21:04:46.8208329Z tidy check
2019-07-16T21:04:47.6960091Z tidy error: /checkout/src/libstd/io/util.rs:47: line longer than 100 chars
2019-07-16T21:04:47.6960221Z tidy error: /checkout/src/libstd/io/util.rs:48: line longer than 100 chars
2019-07-16T21:04:48.6837868Z some tidy checks failed
2019-07-16T21:04:48.6841118Z 
2019-07-16T21:04:48.6841118Z 
2019-07-16T21:04:48.6842109Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-07-16T21:04:48.6842509Z 
2019-07-16T21:04:48.6842626Z 
2019-07-16T21:04:48.6849763Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-07-16T21:04:48.6850067Z Build completed unsuccessfully in 0:01:32
2019-07-16T21:04:48.6850067Z Build completed unsuccessfully in 0:01:32
2019-07-16T21:04:49.9754700Z ##[error]Bash exited with code '1'.
2019-07-16T21:04:49.9783746Z ##[section]Starting: Checkout
2019-07-16T21:04:49.9785115Z ==============================================================================
2019-07-16T21:04:49.9785160Z Task         : Get sources
2019-07-16T21:04:49.9785216Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
