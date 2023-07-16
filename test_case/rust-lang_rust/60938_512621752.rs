plain
2019-07-17T21:04:24.8365099Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-17T21:04:24.8599380Z ##[command]git config gc.auto 0
2019-07-17T21:04:24.8683035Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-17T21:04:24.8738235Z ##[command]git config --get-all http.proxy
2019-07-17T21:04:24.8888063Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/60938/merge:refs/remotes/pull/60938/merge
---
2019-07-17T21:05:01.5837862Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-17T21:05:01.5837890Z 
2019-07-17T21:05:01.5838094Z   git checkout -b <new-branch-name>
2019-07-17T21:05:01.5838297Z 
2019-07-17T21:05:01.5838360Z HEAD is now at 2ed3a4c89 Merge 5273bc412f6b081f8d78f510fdeef547030b3402 into 2eb0bc5e3c52a34b6d62ab0527520c66e4c575bd
2019-07-17T21:05:01.5982773Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-17T21:05:01.5985626Z ==============================================================================
2019-07-17T21:05:01.5985682Z Task         : Bash
2019-07-17T21:05:01.5985741Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-17T21:07:03.0424023Z Attempting with retry: curl -y 30 -Y 10 --connect-timeout 30 -f -L -C - -o /tmp/rustci_docker_cache https://.s3.amazonaws.com/docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624
2019-07-17T21:07:03.0510731Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-17T21:07:03.0511883Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-17T21:07:03.0512116Z 
2019-07-17T21:07:03.0515324Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-17T21:07:04.0584782Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-17T21:07:04.0585063Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-17T21:07:04.0585370Z 
2019-07-17T21:07:04.0585370Z 
2019-07-17T21:07:04.0629817Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-17T21:07:06.0724156Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-17T21:07:06.0724236Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-17T21:07:06.0724271Z 
2019-07-17T21:07:06.0724271Z 
2019-07-17T21:07:06.0752683Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-17T21:07:09.0824530Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-17T21:07:09.0825002Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-17T21:07:09.0825399Z 
2019-07-17T21:07:09.0825399Z 
2019-07-17T21:07:09.0868467Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-17T21:07:13.0946345Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-17T21:07:13.0946433Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-17T21:07:13.0946544Z 
2019-07-17T21:07:13.0946544Z 
2019-07-17T21:07:13.0987258Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-17T21:07:13.0992659Z The command has failed after 5 attempts.
2019-07-17T21:07:13.1661270Z open /tmp/rustci_docker_cache: no such file or directory
2019-07-17T21:07:13.1688821Z Attempting with retry: docker build --rm -t rust-ci -f /home/vsts/work/1/s/src/ci/docker/x86_64-gnu-llvm-6.0/Dockerfile /home/vsts/work/1/s/src/ci/docker
2019-07-17T21:07:13.3667435Z Sending build context to Docker daemon  521.2kB
2019-07-17T21:07:13.3667816Z 
2019-07-17T21:07:13.3791751Z Step 1/8 : FROM ubuntu:16.04
---
2019-07-17T21:07:29.1871008Z Reading package lists...
2019-07-17T21:07:30.1299585Z Reading package lists...
2019-07-17T21:07:30.2947808Z Building dependency tree...
2019-07-17T21:07:30.2947969Z Reading state information...
2019-07-17T21:07:30.4076692Z The following additional packages will be installed:
2019-07-17T21:07:30.4077509Z   binfmt-support binutils bzip2 cmake-data cpp cpp-5 dpkg-dev g++-5 gcc gcc-5
2019-07-17T21:07:30.4077832Z   git-man libarchive13 libasan2 libasn1-8-heimdal libatomic1
2019-07-17T21:07:30.4078075Z   libbabeltrace-ctf1 libbabeltrace1 libbsd-dev libbsd0 libbz2-1.0 libc-dev-bin
2019-07-17T21:07:30.4078587Z   libedit2 liberror-perl libexpat1 libffi-dev libffi6 libgcc-5-dev libgdbm3
2019-07-17T21:07:30.4078814Z   libglib2.0-0 libgmp10 libgnutls30 libgomp1 libgssapi-krb5-2
2019-07-17T21:07:30.4079213Z   libgssapi3-heimdal libhcrypto4-heimdal libheimbase1-heimdal
2019-07-17T21:07:30.4079521Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-17T21:07:30.4079521Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-17T21:07:30.4079771Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-17T21:07:30.4080014Z   libkrb5-3 libkrb5support0 libldap-2.4-2 libllvm6.0 liblsan0 liblzo2-2
2019-07-17T21:07:30.4080297Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-17T21:07:30.4117603Z   libperl5.22 libpipeline1 libpython-stdlib libpython2.7-minimal
2019-07-17T21:07:30.4117853Z   libpython2.7-stdlib libpython3.5 libpython3.5-minimal libpython3.5-stdlib
2019-07-17T21:07:30.4118145Z   libquadmath0 libroken18-heimdal librtmp1 libsasl2-2 libsasl2-modules-db
2019-07-17T21:07:30.4118376Z   libsqlite3-0 libssl1.0.0 libstdc++-5-dev libtasn1-6 libtinfo-dev libtsan0
2019-07-17T21:07:30.4118602Z   libubsan0 libwind0-heimdal libxml2 linux-libc-dev llvm-6.0 llvm-6.0-dev
2019-07-17T21:07:30.4118872Z   llvm-6.0-runtime mime-support openssl patch perl perl-modules-5.22 python
2019-07-17T21:07:30.4119083Z   python-minimal python2.7-minimal
2019-07-17T21:07:30.4119204Z Suggested packages:
2019-07-17T21:07:30.4119489Z   binutils-doc bzip2-doc codeblocks eclipse ninja-build cpp-doc gcc-5-locales
2019-07-17T21:07:30.4119714Z   debian-keyring g++-multilib g++-5-multilib gcc-5-doc libstdc++6-5-dbg
2019-07-17T21:07:30.4119934Z   gcc-multilib manpages-dev autoconf automake libtool flex bison gcc-doc
2019-07-17T21:07:30.4120435Z   libasan2-dbg liblsan0-dbg libtsan0-dbg libubsan0-dbg libcilkrts5-dbg
2019-07-17T21:07:30.4120652Z   libmpx0-dbg libquadmath0-dbg gdb-doc gettext-base git-daemon-run
2019-07-17T21:07:30.4120652Z   libmpx0-dbg libquadmath0-dbg gdb-doc gettext-base git-daemon-run
2019-07-17T21:07:30.4120922Z   | git-daemon-sysvinit git-doc git-el git-email git-gui gitk gitweb git-arch
2019-07-17T21:07:30.4121311Z   git-cvs git-mediawiki git-svn lrzip glibc-doc gnutls-bin krb5-doc krb5-user
2019-07-17T21:07:30.4121533Z   libstdc++-5-doc llvm-6.0-doc make-doc ed diffutils-doc perl-doc
2019-07-17T21:07:30.4121817Z   libterm-readline-gnu-perl | libterm-readline-perl-perl python-doc python-tk
2019-07-17T21:07:30.4122000Z   python2.7-doc
2019-07-17T21:07:30.4122045Z Recommended packages:
2019-07-17T21:07:30.4122319Z   build-essential fakeroot libalgorithm-merge-perl libc-dbg gdbserver less
2019-07-17T21:07:30.4122543Z   rsync ssh-client manpages manpages-dev libfile-fcntllock-perl
2019-07-17T21:07:30.4123198Z   libglib2.0-data shared-mime-info xdg-user-dirs krb5-locales libsasl2-modules
2019-07-17T21:07:30.4123486Z   libssl-doc xml-core netbase rename
2019-07-17T21:07:30.4123540Z The following NEW packages will be installed:
2019-07-17T21:07:30.4123787Z   binfmt-support binutils bzip2 ca-certificates cmake cmake-data cpp cpp-5
2019-07-17T21:07:30.4124087Z   curl dpkg-dev file g++ g++-5 gcc gcc-5 gdb git git-man libarchive13 libasan2
2019-07-17T21:07:30.4124340Z   libasn1-8-heimdal libatomic1 libbabeltrace-ctf1 libbabeltrace1 libbsd-dev
2019-07-17T21:07:30.4124781Z   libbsd0 libc-dev-bin libc6-dev libcc1-0 libcilkrts5 libcurl3 libcurl3-gnutls
2019-07-17T21:07:30.4125142Z   libdpkg-perl libedit-dev libedit2 liberror-perl libexpat1 libffi-dev libffi6
2019-07-17T21:07:30.4125389Z   libgcc-5-dev libgdbm3 libglib2.0-0 libgmp10 libgnutls30 libgomp1
2019-07-17T21:07:30.4125952Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-17T21:07:30.4126209Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-17T21:07:30.4126618Z   libkrb5-3 libkrb5support0 libldap-2.4-2 libllvm6.0 liblsan0 liblzo2-2
2019-07-17T21:07:30.4126903Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-17T21:07:30.4126903Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-17T21:07:30.4127136Z   libperl5.22 libpipeline1 libpython-stdlib libpython2.7-minimal
2019-07-17T21:07:30.4127378Z   libpython2.7-stdlib libpython3.5 libpython3.5-minimal libpython3.5-stdlib
2019-07-17T21:07:30.4127827Z   libquadmath0 libroken18-heimdal librtmp1 libsasl2-2 libsasl2-modules-db
2019-07-17T21:07:30.4128065Z   libsqlite3-0 libssl-dev libssl1.0.0 libstdc++-5-dev libtasn1-6 libtinfo-dev
2019-07-17T21:07:30.4128295Z   libtsan0 libubsan0 libwind0-heimdal libxml2 linux-libc-dev llvm-6.0
2019-07-17T21:07:30.4128576Z   llvm-6.0-dev llvm-6.0-runtime llvm-6.0-tools make mime-support openssl patch
2019-07-17T21:07:30.4128930Z   perl perl-modules-5.22 pkg-config python python-minimal python2.7
2019-07-17T21:07:30.4129139Z   python2.7-minimal sudo xz-utils zlib1g-dev
2019-07-17T21:07:30.4129231Z The following packages will be upgraded:
2019-07-17T21:07:30.8086186Z 1 upgraded, 115 newly installed, 0 to remove and 5 not upgraded.
2019-07-17T21:07:30.8086351Z Need to get 121 MB of archives.
2019-07-17T21:07:30.8086409Z After this operation, 592 MB of additional disk space will be used.
2019-07-17T21:07:30.8087167Z Get:1 http://archive.ubuntu.com/ubuntu xenial/main amd64 libgdbm3 amd64 1.8.3-13.1 [16.9 kB]
---
2019-07-17T21:07:34.6731077Z Get:61 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 sudo amd64 1.8.16-0ubuntu1.7 [390 kB]
2019-07-17T21:07:34.7067964Z Get:62 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 openssl amd64 1.0.2g-1ubuntu4.15 [492 kB]
2019-07-17T21:07:34.8367576Z Get:63 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 ca-certificates all 20170717~16.04.2 [167 kB]
2019-07-17T21:07:34.8398103Z Get:64 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 libcurl3-gnutls amd64 7.47.0-1ubuntu2.13 [184 kB]
2019-07-17T21:07:34.8430867Z Get:65 http://archive.ubuntu.com/ubuntu xenial/main amd64 libedit2 amd64 3.1-20150325-1ubuntu2 [76.5 kB]
2019-07-17T21:07:34.8444300Z Get:66 http://archive.ubuntu.com/ubuntu xenial/main amd64 libpipeline1 amd64 1.4.1-2 [24.6 kB]
2019-07-17T21:07:34.8450975Z Get:67 http://archive.ubuntu.com/ubuntu xenial/main amd64 binfmt-support amd64 2.1.6-1 [50.7 kB]
2019-07-17T21:07:34.9945667Z Get:69 http://archive.ubuntu.com/ubuntu xenial/main amd64 libisl15 amd64 0.16.1-1 [524 kB]
2019-07-17T21:07:35.0483652Z Get:70 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 cpp-5 amd64 5.4.0-6ubuntu1~16.04.11 [7660 kB]
2019-07-17T21:07:35.6941242Z Get:71 http://archive.ubuntu.com/ubuntu xenial/main amd64 cpp amd64 4:5.3.1-1ubuntu1 [27.7 kB]
2019-07-17T21:07:35.6957664Z Get:72 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 curl amd64 7.47.0-1ubuntu2.13 [139 kB]
---
2019-07-17T21:07:58.3675436Z Unpacking ca-certificates (20170717~16.04.2) ...
2019-07-17T21:07:58.5693051Z Selecting previously unselected package libcurl3-gnutls:amd64.
2019-07-17T21:07:58.5708926Z Preparing to unpack .../libcurl3-gnutls_7.47.0-1ubuntu2.13_amd64.deb ...
2019-07-17T21:07:58.5866493Z Unpacking libcurl3-gnutls:amd64 (7.47.0-1ubuntu2.13) ...
2019-07-17T21:07:58.7447928Z Selecting previously unselected package libedit2:amd64.
2019-07-17T21:07:58.7461083Z Preparing to unpack .../libedit2_3.1-20150325-1ubuntu2_amd64.deb ...
2019-07-17T21:07:58.7718809Z Unpacking libedit2:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-17T21:07:59.1817039Z Selecting previously unselected package libpipeline1:amd64.
2019-07-17T21:07:59.1829412Z Preparing to unpack .../libpipeline1_1.4.1-2_amd64.deb ...
2019-07-17T21:07:59.1981890Z Unpacking libpipeline1:amd64 (1.4.1-2) ...
2019-07-17T21:07:59.3147783Z Selecting previously unselected package binfmt-support.
2019-07-17T21:07:59.3159020Z Preparing to unpack .../binfmt-support_2.1.6-1_amd64.deb ...
2019-07-17T21:07:59.3315813Z Unpacking binfmt-support (2.1.6-1) ...
2019-07-17T21:07:59.4626308Z Preparing to unpack .../binutils_2.26.1-1ubuntu1~16.04.8_amd64.deb ...
2019-07-17T21:07:59.4787601Z Unpacking binutils (2.26.1-1ubuntu1~16.04.8) ...
2019-07-17T21:08:00.5150559Z Selecting previously unselected package libisl15:amd64.
2019-07-17T21:08:00.5166345Z Preparing to unpack .../libisl15_0.16.1-1_amd64.deb ...
---
2019-07-17T21:08:15.7991066Z Unpacking zlib1g-dev:amd64 (1:1.2.8.dfsg-2ubuntu4.1) ...
2019-07-17T21:08:15.9278029Z Selecting previously unselected package libssl-dev:amd64.
2019-07-17T21:08:15.9295252Z Preparing to unpack .../libssl-dev_1.0.2g-1ubuntu4.15_amd64.deb ...
2019-07-17T21:08:15.9461280Z Unpacking libssl-dev:amd64 (1.0.2g-1ubuntu4.15) ...
2019-07-17T21:08:16.2782557Z Selecting previously unselected package llvm-6.0-runtime.
2019-07-17T21:08:16.2802287Z Preparing to unpack .../llvm-6.0-runtime_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-17T21:08:16.2970486Z Unpacking llvm-6.0-runtime (1:6.0-1ubuntu2~16.04.1) ...
2019-07-17T21:08:16.4414207Z Selecting previously unselected package llvm-6.0.
2019-07-17T21:08:16.4432494Z Preparing to unpack .../llvm-6.0_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-17T21:08:16.4587486Z Unpacking llvm-6.0 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-17T21:08:17.1257087Z Selecting previously unselected package libffi-dev:amd64.
2019-07-17T21:08:17.1277401Z Preparing to unpack .../libffi-dev_3.2.1-4_amd64.deb ...
2019-07-17T21:08:17.1429628Z Unpacking libffi-dev:amd64 (3.2.1-4) ...
2019-07-17T21:08:17.2906117Z Selecting previously unselected package llvm-6.0-dev.
2019-07-17T21:08:17.2923869Z Preparing to unpack .../llvm-6.0-dev_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-17T21:08:17.3076623Z Unpacking llvm-6.0-dev (1:6.0-1ubuntu2~16.04.1) ...
2019-07-17T21:08:21.8732896Z Selecting previously unselected package llvm-6.0-tools.
2019-07-17T21:08:21.8758632Z Preparing to unpack .../llvm-6.0-tools_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-17T21:08:21.8917145Z Unpacking llvm-6.0-tools (1:6.0-1ubuntu2~16.04.1) ...
2019-07-17T21:08:22.0782761Z Selecting previously unselected package pkg-config.
2019-07-17T21:08:22.0802072Z Preparing to unpack .../pkg-config_0.29.1-0ubuntu1_amd64.deb ...
2019-07-17T21:08:22.1626354Z Processing triggers for libc-bin (2.23-0ubuntu11) ...
2019-07-17T21:08:22.2257936Z Processing triggers for systemd (229-4ubuntu21.21) ...
2019-07-17T21:08:22.6669232Z Setting up libgdbm3:amd64 (1.8.3-13.1) ...
2019-07-17T21:08:22.7601733Z Setting up libffi6:amd64 (3.2.1-4) ...
---
2019-07-17T21:08:27.2124090Z debconf: unable to initialize frontend: Dialog
2019-07-17T21:08:27.2124302Z debconf: (TERM is not set, so the dialog frontend is not usable.)
2019-07-17T21:08:27.2124798Z debconf: falling back to frontend: Readline
2019-07-17T21:08:27.7385729Z Setting up libcurl3-gnutls:amd64 (7.47.0-1ubuntu2.13) ...
2019-07-17T21:08:27.7875535Z Setting up libedit2:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-17T21:08:27.8376125Z Setting up libpipeline1:amd64 (1.4.1-2) ...
2019-07-17T21:08:27.8858027Z Setting up binfmt-support (2.1.6-1) ...
2019-07-17T21:08:27.9714724Z mount: permission denied
2019-07-17T21:08:27.9715705Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-17T21:08:27.9732341Z mount: permission denied
2019-07-17T21:08:27.9733813Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-17T21:08:28.1328540Z invoke-rc.d: could not determine current runlevel
2019-07-17T21:08:28.1360832Z invoke-rc.d: policy-rc.d denied execution of start.
2019-07-17T21:08:28.2169857Z Setting up libisl15:amd64 (0.16.1-1) ...
2019-07-17T21:08:28.2687939Z Setting up cpp-5 (5.4.0-6ubuntu1~16.04.11) ...
2019-07-17T21:08:28.3246682Z Setting up cpp (4:5.3.1-1ubuntu1) ...
2019-07-17T21:08:28.3931814Z Setting up curl (7.47.0-1ubuntu2.13) ...
---
2019-07-17T21:08:30.3718810Z Setting up libedit-dev:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-17T21:08:30.4207630Z Setting up libllvm6.0:amd64 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-17T21:08:30.4719465Z Setting up zlib1g-dev:amd64 (1:1.2.8.dfsg-2ubuntu4.1) ...
2019-07-17T21:08:30.5210595Z Setting up libssl-dev:amd64 (1.0.2g-1ubuntu4.15) ...
2019-07-17T21:08:30.5720350Z Setting up llvm-6.0-runtime (1:6.0-1ubuntu2~16.04.1) ...
2019-07-17T21:08:30.6075200Z mount: permission denied
2019-07-17T21:08:30.6080028Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-17T21:08:30.6253806Z Setting up llvm-6.0 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-17T21:08:30.6736388Z Setting up libffi-dev:amd64 (3.2.1-4) ...
2019-07-17T21:08:30.7399178Z Setting up llvm-6.0-dev (1:6.0-1ubuntu2~16.04.1) ...
2019-07-17T21:08:30.7901955Z Setting up llvm-6.0-tools (1:6.0-1ubuntu2~16.04.1) ...
2019-07-17T21:08:30.8387768Z Setting up pkg-config (0.29.1-0ubuntu1) ...
2019-07-17T21:08:30.9889110Z Processing triggers for ca-certificates (20170717~16.04.2) ...
2019-07-17T21:08:31.0089295Z Updating certificates in /etc/ssl/certs...
2019-07-17T21:08:32.5950123Z 148 added, 0 removed; done.
2019-07-17T21:08:32.5951825Z Running hooks in /etc/ca-certificates/update.d...
---
2019-07-17T21:09:06.6840935Z  ---> 74ce090054fe
2019-07-17T21:09:06.6878600Z Successfully built 74ce090054fe
2019-07-17T21:09:06.9142281Z Successfully tagged rust-ci:latest
2019-07-17T21:09:06.9684791Z Built container sha256:74ce090054fe3776942ef81f2170e3f2a7f49d12be97e9cc4680da8e55f5fbed
2019-07-17T21:09:06.9698650Z Uploading finished image to https://.s3.amazonaws.com/docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624
2019-07-17T21:10:09.0118789Z upload failed: - to s3:///docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624 Parameter validation failed:
2019-07-17T21:10:09.0120161Z Invalid bucket name "": Bucket name must match the regex "^[a-zA-Z0-9.\-_]{1,255}$"
2019-07-17T21:10:10.2447653Z [CI_JOB_NAME=x86_64-gnu-llvm-6.0]
2019-07-17T21:10:10.2496235Z Starting sccache server...
2019-07-17T21:10:10.2977562Z configure: processing command line
2019-07-17T21:10:10.2978233Z configure: 
---
2019-07-17T22:07:22.3064042Z .................................................................................................... 200/5808
2019-07-17T22:07:26.4400272Z .................................................................................................... 300/5808
2019-07-17T22:07:30.0868558Z .................................................................................................... 400/5808
2019-07-17T22:07:34.6011405Z .................................................................................................... 500/5808
2019-07-17T22:07:37.4634331Z ........................................................................i........................... 600/5808
2019-07-17T22:07:46.1898942Z .................................................................................................... 800/5808
2019-07-17T22:07:51.6611680Z .................................................................................................... 900/5808
2019-07-17T22:07:51.6611680Z .................................................................................................... 900/5808
2019-07-17T22:07:57.6035775Z ............................................................................................i....... 1000/5808
2019-07-17T22:08:01.8813718Z ....i............................................................................................... 1100/5808
2019-07-17T22:08:06.6086010Z ......................iiiii......................................................................... 1200/5808
2019-07-17T22:08:11.6704058Z .................................................................................................... 1400/5808
2019-07-17T22:08:14.3811863Z .................................................................................................... 1500/5808
2019-07-17T22:08:18.1147819Z ...........................................................................F........................ 1600/5808
2019-07-17T22:08:20.6003516Z .................................................................................................... 1700/5808
2019-07-17T22:08:20.6003516Z .................................................................................................... 1700/5808
2019-07-17T22:08:24.0766376Z ...........................................................i........................................ 1800/5808
2019-07-17T22:08:32.5256445Z .................................................................................................... 2000/5808
2019-07-17T22:08:37.1148700Z .................................................................................................... 2100/5808
2019-07-17T22:08:40.4625135Z .................................................................................................... 2200/5808
2019-07-17T22:08:40.4625135Z .................................................................................................... 2200/5808
2019-07-17T22:08:44.0320411Z .........................i.......................................................................... 2300/5808
2019-07-17T22:08:54.6186438Z .................................................................................................... 2500/5808
2019-07-17T22:08:58.7811944Z .................................................................................................... 2600/5808
2019-07-17T22:09:03.1830297Z .................................................................................................... 2700/5808
2019-07-17T22:09:07.5377327Z .................................................................................................... 2800/5808
2019-07-17T22:09:07.5377327Z .................................................................................................... 2800/5808
2019-07-17T22:09:11.5365180Z .................................................................................................... 2900/5808
2019-07-17T22:09:17.5980591Z .................................................................................................... 3000/5808
2019-07-17T22:09:22.3907378Z .................................................................................................... 3100/5808
2019-07-17T22:09:26.9708766Z .................................................................................................... 3200/5808
2019-07-17T22:09:30.0617650Z .................................................................................................... 3300/5808
2019-07-17T22:09:35.0081152Z .................................................................................................... 3400/5808
2019-07-17T22:09:39.1262979Z .......................................................................................i............ 3500/5808
2019-07-17T22:09:42.8463078Z .................................................................................................... 3600/5808
2019-07-17T22:09:46.6932462Z .............................................................ii...i..ii............................. 3700/5808
2019-07-17T22:09:55.8108633Z .................................................................................................... 3900/5808
2019-07-17T22:09:55.8108633Z .................................................................................................... 3900/5808
2019-07-17T22:09:59.6947729Z ...........................................................................ii....................... 4000/5808
2019-07-17T22:10:02.4347876Z ................................................................................................i... 4100/5808
2019-07-17T22:10:04.5920067Z .................................................................................................... 4200/5808
2019-07-17T22:10:06.5255428Z ..............................................................i..................................... 4300/5808
2019-07-17T22:10:19.7849919Z .................................................................................................... 4500/5808
2019-07-17T22:10:31.0956536Z .................................................................................................... 4600/5808
2019-07-17T22:10:34.8315274Z .................................................................................................... 4700/5808
2019-07-17T22:10:38.3031459Z .................................................................................................... 4800/5808
---
2019-07-17T22:11:11.7924504Z .................................................................................................... 5400/5808
2019-07-17T22:11:16.6474559Z .................................................................................................... 5500/5808
2019-07-17T22:11:19.6305795Z .................................................................................................... 5600/5808
2019-07-17T22:11:22.7462800Z .................................................................................................... 5700/5808
2019-07-17T22:11:25.6391193Z ................................................i................................................... 5800/5808
2019-07-17T22:11:26.1503857Z failures:
2019-07-17T22:11:26.1541739Z 
2019-07-17T22:11:26.1542266Z ---- [ui] ui/extern/external-doc-error.rs stdout ----
2019-07-17T22:11:26.1542318Z diff of stderr:
2019-07-17T22:11:26.1542318Z diff of stderr:
2019-07-17T22:11:26.1542344Z 
2019-07-17T22:11:26.1542380Z 3    |
2019-07-17T22:11:26.1543040Z 4 LL | #[doc(include = "not-a-file.md")]
2019-07-17T22:11:26.1543286Z 5    |                 ^^^^^^^^^^^^^^^ couldn't read file
2019-07-17T22:11:26.1543471Z -    |
2019-07-17T22:11:26.1543729Z -    = help: external doc paths are relative to the crate root
2019-07-17T22:11:26.1543781Z 8 
2019-07-17T22:11:26.1544013Z 9 error: $DIR/auxiliary/invalid-utf8.txt wasn't a utf-8 file
2019-07-17T22:11:26.1544252Z 10   --> $DIR/external-doc-error.rs:9:17
2019-07-17T22:11:26.1544333Z 
2019-07-17T22:11:26.1544381Z The actual stderr differed from the expected stderr.
2019-07-17T22:11:26.1544709Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/external-doc-error/external-doc-error.stderr
2019-07-17T22:11:26.1544709Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/external-doc-error/external-doc-error.stderr
2019-07-17T22:11:26.1544963Z To update references, rerun the tests and pass the `--bless` flag
2019-07-17T22:11:26.1545478Z To only update this specific test, also pass `--test-args extern/external-doc-error.rs`
2019-07-17T22:11:26.1545582Z error: 1 errors occurred comparing output.
2019-07-17T22:11:26.1545629Z status: exit code: 1
2019-07-17T22:11:26.1545629Z status: exit code: 1
2019-07-17T22:11:26.1546556Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/extern/external-doc-error.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/external-doc-error" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/external-doc-error/auxiliary" "-A" "unused"
2019-07-17T22:11:26.1547234Z ------------------------------------------
2019-07-17T22:11:26.1547271Z 
2019-07-17T22:11:26.1547463Z ------------------------------------------
2019-07-17T22:11:26.1547503Z stderr:
2019-07-17T22:11:26.1547503Z stderr:
2019-07-17T22:11:26.1547706Z ------------------------------------------
2019-07-17T22:11:26.1547947Z error: couldn't read /checkout/src/test/ui/extern/not-a-file.md: No such file or directory (os error 2)
2019-07-17T22:11:26.1548231Z    |
2019-07-17T22:11:26.1548231Z    |
2019-07-17T22:11:26.1548418Z LL | #[doc(include = "not-a-file.md")]
2019-07-17T22:11:26.1548616Z    |                 ^^^^^^^^^^^^^^^ couldn't read file
2019-07-17T22:11:26.1548664Z 
2019-07-17T22:11:26.1548892Z error: /checkout/src/test/ui/extern/auxiliary/invalid-utf8.txt wasn't a utf-8 file
2019-07-17T22:11:26.1549160Z    |
2019-07-17T22:11:26.1549160Z    |
2019-07-17T22:11:26.1549356Z LL | #[doc(include = "auxiliary/invalid-utf8.txt")]
2019-07-17T22:11:26.1549572Z    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ contains invalid utf-8
2019-07-17T22:11:26.1549667Z error: expected path to external documentation
2019-07-17T22:11:26.1549875Z   --> /checkout/src/test/ui/extern/external-doc-error.rs:12:7
2019-07-17T22:11:26.1549916Z    |
2019-07-17T22:11:26.1549916Z    |
2019-07-17T22:11:26.1549973Z LL | #[doc(include)]
2019-07-17T22:11:26.1550018Z    |       ^^^^^^^ help: provide a file path with `=`: `include = "<path>"`
2019-07-17T22:11:26.1550087Z error: expected path to external documentation
2019-07-17T22:11:26.1550495Z   --> /checkout/src/test/ui/extern/external-doc-error.rs:17:7
2019-07-17T22:11:26.1550539Z    |
2019-07-17T22:11:26.1550539Z    |
2019-07-17T22:11:26.1551032Z LL | #[doc(include("../README.md"))]
2019-07-17T22:11:26.1551112Z    |       ^^^^^^^^^^^^^^^^^^^^^^^ help: provide a file path with `=`: `include = "../README.md"`
2019-07-17T22:11:26.1551183Z error: expected path to external documentation
2019-07-17T22:11:26.1551425Z   --> /checkout/src/test/ui/extern/external-doc-error.rs:22:7
2019-07-17T22:11:26.1551497Z    |
2019-07-17T22:11:26.1551497Z    |
2019-07-17T22:11:26.1551535Z LL | #[doc(include = 123)]
2019-07-17T22:11:26.1551581Z    |       ^^^^^^^^^^^^^ help: provide a file path with `=`: `include = "<path>"`
2019-07-17T22:11:26.1551669Z error: expected path to external documentation
2019-07-17T22:11:26.1551883Z   --> /checkout/src/test/ui/extern/external-doc-error.rs:27:7
2019-07-17T22:11:26.1551925Z    |
2019-07-17T22:11:26.1551925Z    |
2019-07-17T22:11:26.1551983Z LL | #[doc(include(123))]
2019-07-17T22:11:26.1552207Z    |       ^^^^^^^^^^^^ help: provide a file path with `=`: `include = "<path>"`
2019-07-17T22:11:26.1552476Z error: aborting due to 6 previous errors
2019-07-17T22:11:26.1552503Z 
2019-07-17T22:11:26.1552687Z 
2019-07-17T22:11:26.1553068Z ------------------------------------------
---
2019-07-17T22:11:26.1557827Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:535:22
2019-07-17T22:11:26.1557899Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-07-17T22:11:26.1573569Z 
2019-07-17T22:11:26.1573707Z 
2019-07-17T22:11:26.1575657Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-07-17T22:11:26.1576107Z 
2019-07-17T22:11:26.1576135Z 
2019-07-17T22:11:26.1580061Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-07-17T22:11:26.1580133Z Build completed unsuccessfully in 0:57:28
2019-07-17T22:11:26.1580133Z Build completed unsuccessfully in 0:57:28
2019-07-17T22:11:27.5711039Z ##[error]Bash exited with code '1'.
2019-07-17T22:11:27.5750253Z ##[section]Starting: Checkout
2019-07-17T22:11:27.5751838Z ==============================================================================
2019-07-17T22:11:27.5751888Z Task         : Get sources
2019-07-17T22:11:27.5751953Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
