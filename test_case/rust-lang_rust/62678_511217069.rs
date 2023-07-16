plain
2019-07-14T15:59:31.1720243Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-14T15:59:31.1940307Z ##[command]git config gc.auto 0
2019-07-14T15:59:31.2014701Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-14T15:59:31.2087021Z ##[command]git config --get-all http.proxy
2019-07-14T15:59:31.2217398Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62678/merge:refs/remotes/pull/62678/merge
---
2019-07-14T16:00:04.9224250Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-14T16:00:04.9224304Z 
2019-07-14T16:00:04.9224541Z   git checkout -b <new-branch-name>
2019-07-14T16:00:04.9224574Z 
2019-07-14T16:00:04.9224627Z HEAD is now at 0d69a8b56 Merge e256f53d2db5e212deaf4c906a01c103b9dd9555 into 7d41ebf768faca490addc7c616b3a9274621f0e9
2019-07-14T16:00:04.9370548Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-14T16:00:04.9373288Z ==============================================================================
2019-07-14T16:00:04.9373346Z Task         : Bash
2019-07-14T16:00:04.9373395Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-14T16:01:57.0192976Z Attempting with retry: curl -y 30 -Y 10 --connect-timeout 30 -f -L -C - -o /tmp/rustci_docker_cache https://.s3.amazonaws.com/docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624
2019-07-14T16:01:57.0263978Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-14T16:01:57.0264450Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-14T16:01:57.0264790Z 
2019-07-14T16:01:57.0265818Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-14T16:01:58.0359413Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-14T16:01:58.0359933Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-14T16:01:58.0360122Z 
2019-07-14T16:01:58.0360122Z 
2019-07-14T16:01:58.0390220Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-14T16:02:00.0462424Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-14T16:02:00.0462683Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-14T16:02:00.0462724Z 
2019-07-14T16:02:00.0462724Z 
2019-07-14T16:02:00.0505858Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-14T16:02:03.0578019Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-14T16:02:03.0580079Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-14T16:02:03.0580486Z 
2019-07-14T16:02:03.0580486Z 
2019-07-14T16:02:03.0632367Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-14T16:02:07.0696085Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-14T16:02:07.0696455Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-14T16:02:07.0697109Z 
2019-07-14T16:02:07.0697109Z 
2019-07-14T16:02:07.0739527Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-14T16:02:07.0746648Z The command has failed after 5 attempts.
2019-07-14T16:02:07.1315620Z open /tmp/rustci_docker_cache: no such file or directory
2019-07-14T16:02:07.1344324Z Attempting with retry: docker build --rm -t rust-ci -f /home/vsts/work/1/s/src/ci/docker/x86_64-gnu-llvm-6.0/Dockerfile /home/vsts/work/1/s/src/ci/docker
2019-07-14T16:02:07.3636253Z Sending build context to Docker daemon  521.7kB
2019-07-14T16:02:07.3636496Z 
2019-07-14T16:02:07.3934363Z Step 1/8 : FROM ubuntu:16.04
---
2019-07-14T16:02:24.0124507Z Reading package lists...
2019-07-14T16:02:25.0063283Z Reading package lists...
2019-07-14T16:02:25.1912776Z Building dependency tree...
2019-07-14T16:02:25.1913006Z Reading state information...
2019-07-14T16:02:25.3252926Z The following additional packages will be installed:
2019-07-14T16:02:25.3253788Z   binfmt-support binutils bzip2 cmake-data cpp cpp-5 dpkg-dev g++-5 gcc gcc-5
2019-07-14T16:02:25.3280249Z   git-man libarchive13 libasan2 libasn1-8-heimdal libatomic1
2019-07-14T16:02:25.3280625Z   libbabeltrace-ctf1 libbabeltrace1 libbsd-dev libbsd0 libbz2-1.0 libc-dev-bin
2019-07-14T16:02:25.3281300Z   libedit2 liberror-perl libexpat1 libffi-dev libffi6 libgcc-5-dev libgdbm3
2019-07-14T16:02:25.3281547Z   libglib2.0-0 libgmp10 libgnutls30 libgomp1 libgssapi-krb5-2
2019-07-14T16:02:25.3281785Z   libgssapi3-heimdal libhcrypto4-heimdal libheimbase1-heimdal
2019-07-14T16:02:25.3282113Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-14T16:02:25.3282113Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-14T16:02:25.3282374Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-14T16:02:25.3282628Z   libkrb5-3 libkrb5support0 libldap-2.4-2 libllvm6.0 liblsan0 liblzo2-2
2019-07-14T16:02:25.3282934Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-14T16:02:25.3283177Z   libperl5.22 libpipeline1 libpython-stdlib libpython2.7-minimal
2019-07-14T16:02:25.3283431Z   libpython2.7-stdlib libpython3.5 libpython3.5-minimal libpython3.5-stdlib
2019-07-14T16:02:25.3284082Z   libquadmath0 libroken18-heimdal librtmp1 libsasl2-2 libsasl2-modules-db
2019-07-14T16:02:25.3284341Z   libsqlite3-0 libssl1.0.0 libstdc++-5-dev libtasn1-6 libtinfo-dev libtsan0
2019-07-14T16:02:25.3284591Z   libubsan0 libwind0-heimdal libxml2 linux-libc-dev llvm-6.0 llvm-6.0-dev
2019-07-14T16:02:25.3284896Z   llvm-6.0-runtime mime-support openssl patch perl perl-modules-5.22 python
2019-07-14T16:02:25.3285114Z   python-minimal python2.7-minimal
2019-07-14T16:02:25.3285504Z Suggested packages:
2019-07-14T16:02:25.3285888Z   binutils-doc bzip2-doc codeblocks eclipse ninja-build cpp-doc gcc-5-locales
2019-07-14T16:02:25.3286161Z   debian-keyring g++-multilib g++-5-multilib gcc-5-doc libstdc++6-5-dbg
2019-07-14T16:02:25.3286407Z   gcc-multilib manpages-dev autoconf automake libtool flex bison gcc-doc
2019-07-14T16:02:25.3286956Z   libasan2-dbg liblsan0-dbg libtsan0-dbg libubsan0-dbg libcilkrts5-dbg
2019-07-14T16:02:25.3287210Z   libmpx0-dbg libquadmath0-dbg gdb-doc gettext-base git-daemon-run
2019-07-14T16:02:25.3287210Z   libmpx0-dbg libquadmath0-dbg gdb-doc gettext-base git-daemon-run
2019-07-14T16:02:25.3287505Z   | git-daemon-sysvinit git-doc git-el git-email git-gui gitk gitweb git-arch
2019-07-14T16:02:25.3287757Z   git-cvs git-mediawiki git-svn lrzip glibc-doc gnutls-bin krb5-doc krb5-user
2019-07-14T16:02:25.3287998Z   libstdc++-5-doc llvm-6.0-doc make-doc ed diffutils-doc perl-doc
2019-07-14T16:02:25.3288312Z   libterm-readline-gnu-perl | libterm-readline-perl-perl python-doc python-tk
2019-07-14T16:02:25.3288513Z   python2.7-doc
2019-07-14T16:02:25.3288559Z Recommended packages:
2019-07-14T16:02:25.3288849Z   build-essential fakeroot libalgorithm-merge-perl libc-dbg gdbserver less
2019-07-14T16:02:25.3289088Z   rsync ssh-client manpages manpages-dev libfile-fcntllock-perl
2019-07-14T16:02:25.3289343Z   libglib2.0-data shared-mime-info xdg-user-dirs krb5-locales libsasl2-modules
2019-07-14T16:02:25.3289581Z   libssl-doc xml-core netbase rename
2019-07-14T16:02:25.3289641Z The following NEW packages will be installed:
2019-07-14T16:02:25.3289891Z   binfmt-support binutils bzip2 ca-certificates cmake cmake-data cpp cpp-5
2019-07-14T16:02:25.3290187Z   curl dpkg-dev file g++ g++-5 gcc gcc-5 gdb git git-man libarchive13 libasan2
2019-07-14T16:02:25.3290450Z   libasn1-8-heimdal libatomic1 libbabeltrace-ctf1 libbabeltrace1 libbsd-dev
2019-07-14T16:02:25.3290706Z   libbsd0 libc-dev-bin libc6-dev libcc1-0 libcilkrts5 libcurl3 libcurl3-gnutls
2019-07-14T16:02:25.3291134Z   libdpkg-perl libedit-dev libedit2 liberror-perl libexpat1 libffi-dev libffi6
2019-07-14T16:02:25.3291441Z   libgcc-5-dev libgdbm3 libglib2.0-0 libgmp10 libgnutls30 libgomp1
2019-07-14T16:02:25.3292067Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-14T16:02:25.3292337Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-14T16:02:25.3292603Z   libkrb5-3 libkrb5support0 libldap-2.4-2 libllvm6.0 liblsan0 liblzo2-2
2019-07-14T16:02:25.3292895Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-14T16:02:25.3292895Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-14T16:02:25.3293145Z   libperl5.22 libpipeline1 libpython-stdlib libpython2.7-minimal
2019-07-14T16:02:25.3293398Z   libpython2.7-stdlib libpython3.5 libpython3.5-minimal libpython3.5-stdlib
2019-07-14T16:02:25.3293695Z   libquadmath0 libroken18-heimdal librtmp1 libsasl2-2 libsasl2-modules-db
2019-07-14T16:02:25.3293967Z   libsqlite3-0 libssl-dev libssl1.0.0 libstdc++-5-dev libtasn1-6 libtinfo-dev
2019-07-14T16:02:25.3294218Z   libtsan0 libubsan0 libwind0-heimdal libxml2 linux-libc-dev llvm-6.0
2019-07-14T16:02:25.3294519Z   llvm-6.0-dev llvm-6.0-runtime llvm-6.0-tools make mime-support openssl patch
2019-07-14T16:02:25.3294770Z   perl perl-modules-5.22 pkg-config python python-minimal python2.7
2019-07-14T16:02:25.3294995Z   python2.7-minimal sudo xz-utils zlib1g-dev
2019-07-14T16:02:25.3295088Z The following packages will be upgraded:
2019-07-14T16:02:25.6062486Z 1 upgraded, 115 newly installed, 0 to remove and 4 not upgraded.
2019-07-14T16:02:25.6062842Z Need to get 121 MB of archives.
2019-07-14T16:02:25.6063040Z After this operation, 592 MB of additional disk space will be used.
2019-07-14T16:02:25.6063989Z Get:1 http://archive.ubuntu.com/ubuntu xenial/main amd64 libgdbm3 amd64 1.8.3-13.1 [16.9 kB]
---
2019-07-14T16:02:26.7826907Z Get:61 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 sudo amd64 1.8.16-0ubuntu1.7 [390 kB]
2019-07-14T16:02:26.7922572Z Get:62 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 openssl amd64 1.0.2g-1ubuntu4.15 [492 kB]
2019-07-14T16:02:26.7994913Z Get:63 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 ca-certificates all 20170717~16.04.2 [167 kB]
2019-07-14T16:02:26.8021425Z Get:64 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 libcurl3-gnutls amd64 7.47.0-1ubuntu2.13 [184 kB]
2019-07-14T16:02:26.8050048Z Get:65 http://archive.ubuntu.com/ubuntu xenial/main amd64 libedit2 amd64 3.1-20150325-1ubuntu2 [76.5 kB]
2019-07-14T16:02:26.8067632Z Get:66 http://archive.ubuntu.com/ubuntu xenial/main amd64 libpipeline1 amd64 1.4.1-2 [24.6 kB]
2019-07-14T16:02:26.8076967Z Get:67 http://archive.ubuntu.com/ubuntu xenial/main amd64 binfmt-support amd64 2.1.6-1 [50.7 kB]
2019-07-14T16:02:26.8708717Z Get:69 http://archive.ubuntu.com/ubuntu xenial/main amd64 libisl15 amd64 0.16.1-1 [524 kB]
2019-07-14T16:02:26.8788837Z Get:70 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 cpp-5 amd64 5.4.0-6ubuntu1~16.04.11 [7660 kB]
2019-07-14T16:02:27.0138968Z Get:71 http://archive.ubuntu.com/ubuntu xenial/main amd64 cpp amd64 4:5.3.1-1ubuntu1 [27.7 kB]
2019-07-14T16:02:27.0147108Z Get:72 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 curl amd64 7.47.0-1ubuntu2.13 [139 kB]
---
2019-07-14T16:02:44.7555752Z Unpacking ca-certificates (20170717~16.04.2) ...
2019-07-14T16:02:44.9173304Z Selecting previously unselected package libcurl3-gnutls:amd64.
2019-07-14T16:02:44.9189487Z Preparing to unpack .../libcurl3-gnutls_7.47.0-1ubuntu2.13_amd64.deb ...
2019-07-14T16:02:44.9320459Z Unpacking libcurl3-gnutls:amd64 (7.47.0-1ubuntu2.13) ...
2019-07-14T16:02:45.0593044Z Selecting previously unselected package libedit2:amd64.
2019-07-14T16:02:45.0608733Z Preparing to unpack .../libedit2_3.1-20150325-1ubuntu2_amd64.deb ...
2019-07-14T16:02:45.0735365Z Unpacking libedit2:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-14T16:02:45.1873102Z Selecting previously unselected package libpipeline1:amd64.
2019-07-14T16:02:45.1892666Z Preparing to unpack .../libpipeline1_1.4.1-2_amd64.deb ...
2019-07-14T16:02:45.2028908Z Unpacking libpipeline1:amd64 (1.4.1-2) ...
2019-07-14T16:02:45.3084887Z Selecting previously unselected package binfmt-support.
2019-07-14T16:02:45.3102317Z Preparing to unpack .../binfmt-support_2.1.6-1_amd64.deb ...
2019-07-14T16:02:45.3227159Z Unpacking binfmt-support (2.1.6-1) ...
2019-07-14T16:02:45.4366497Z Preparing to unpack .../binutils_2.26.1-1ubuntu1~16.04.8_amd64.deb ...
2019-07-14T16:02:45.4502713Z Unpacking binutils (2.26.1-1ubuntu1~16.04.8) ...
2019-07-14T16:02:46.0384090Z Selecting previously unselected package libisl15:amd64.
2019-07-14T16:02:46.0403205Z Preparing to unpack .../libisl15_0.16.1-1_amd64.deb ...
---
2019-07-14T16:02:57.5645924Z Selecting previously unselected package libtinfo-dev:amd64.
2019-07-14T16:02:57.5665383Z Preparing to unpack .../libtinfo-dev_6.0+20160213-1ubuntu1_amd64.deb ...
2019-07-14T16:02:57.5790896Z Unpacking libtinfo-dev:amd64 (6.0+20160213-1ubuntu1) ...
2019-07-14T16:02:57.6725918Z Selecting previously unselected package libedit-dev:amd64.
2019-07-14T16:02:57.6744313Z Preparing to unpack .../libedit-dev_3.1-20150325-1ubuntu2_amd64.deb ...
2019-07-14T16:02:57.6873252Z Unpacking libedit-dev:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-14T16:02:57.8056926Z Selecting previously unselected package libllvm6.0:amd64.
2019-07-14T16:02:57.8080393Z Preparing to unpack .../libllvm6.0_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-14T16:02:57.8220799Z Unpacking libllvm6.0:amd64 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-14T16:03:00.7901480Z Selecting previously unselected package zlib1g-dev:amd64.
2019-07-14T16:03:00.7921323Z Preparing to unpack .../zlib1g-dev_1%3a1.2.8.dfsg-2ubuntu4.1_amd64.deb ...
2019-07-14T16:03:00.8042314Z Unpacking zlib1g-dev:amd64 (1:1.2.8.dfsg-2ubuntu4.1) ...
2019-07-14T16:03:00.9171775Z Preparing to unpack .../libssl-dev_1.0.2g-1ubuntu4.15_amd64.deb ...
2019-07-14T16:03:00.9301037Z Unpacking libssl-dev:amd64 (1.0.2g-1ubuntu4.15) ...
2019-07-14T16:03:01.2478704Z Selecting previously unselected package llvm-6.0-runtime.
2019-07-14T16:03:01.2478704Z Selecting previously unselected package llvm-6.0-runtime.
2019-07-14T16:03:01.2499389Z Preparing to unpack .../llvm-6.0-runtime_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-14T16:03:01.2626523Z Unpacking llvm-6.0-runtime (1:6.0-1ubuntu2~16.04.1) ...
2019-07-14T16:03:01.3778757Z Selecting previously unselected package llvm-6.0.
2019-07-14T16:03:01.3792342Z Preparing to unpack .../llvm-6.0_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-14T16:03:01.3938154Z Unpacking llvm-6.0 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-14T16:03:02.0542550Z Selecting previously unselected package libffi-dev:amd64.
2019-07-14T16:03:02.0563670Z Preparing to unpack .../libffi-dev_3.2.1-4_amd64.deb ...
2019-07-14T16:03:02.0683781Z Unpacking libffi-dev:amd64 (3.2.1-4) ...
2019-07-14T16:03:02.1907504Z Selecting previously unselected package llvm-6.0-dev.
2019-07-14T16:03:02.1931342Z Preparing to unpack .../llvm-6.0-dev_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-14T16:03:02.2092846Z Unpacking llvm-6.0-dev (1:6.0-1ubuntu2~16.04.1) ...
2019-07-14T16:03:07.0389276Z Selecting previously unselected package llvm-6.0-tools.
2019-07-14T16:03:07.0402913Z Preparing to unpack .../llvm-6.0-tools_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-14T16:03:07.0528758Z Unpacking llvm-6.0-tools (1:6.0-1ubuntu2~16.04.1) ...
2019-07-14T16:03:07.2128424Z Selecting previously unselected package pkg-config.
2019-07-14T16:03:07.2153445Z Preparing to unpack .../pkg-config_0.29.1-0ubuntu1_amd64.deb ...
2019-07-14T16:03:07.2276405Z Unpacking pkg-config (0.29.1-0ubuntu1) ...
2019-07-14T16:03:07.8166847Z Processing triggers for systemd (229-4ubuntu21.21) ...
2019-07-14T16:03:07.8167224Z Setting up libgdbm3:amd64 (1.8.3-13.1) ...
2019-07-14T16:03:07.8167573Z Setting up libffi6:amd64 (3.2.1-4) ...
2019-07-14T16:03:07.8167871Z Setting up libglib2.0-0:amd64 (2.48.2-0ubuntu4.3) ...
---
2019-07-14T16:03:11.4825241Z debconf: unable to initialize frontend: Dialog
2019-07-14T16:03:11.4825849Z debconf: (TERM is not set, so the dialog frontend is not usable.)
2019-07-14T16:03:11.4825922Z debconf: falling back to frontend: Readline
2019-07-14T16:03:11.9982615Z Setting up libcurl3-gnutls:amd64 (7.47.0-1ubuntu2.13) ...
2019-07-14T16:03:12.0407215Z Setting up libedit2:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-14T16:03:12.0806695Z Setting up libpipeline1:amd64 (1.4.1-2) ...
2019-07-14T16:03:12.1181937Z Setting up binfmt-support (2.1.6-1) ...
2019-07-14T16:03:12.1843518Z mount: permission denied
2019-07-14T16:03:12.1849538Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-14T16:03:12.1863690Z mount: permission denied
2019-07-14T16:03:12.1868001Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-14T16:03:12.3552313Z invoke-rc.d: could not determine current runlevel
2019-07-14T16:03:12.3581717Z invoke-rc.d: policy-rc.d denied execution of start.
2019-07-14T16:03:12.4144715Z Setting up libisl15:amd64 (0.16.1-1) ...
2019-07-14T16:03:12.4512077Z Setting up cpp-5 (5.4.0-6ubuntu1~16.04.11) ...
2019-07-14T16:03:12.4904930Z Setting up cpp (4:5.3.1-1ubuntu1) ...
2019-07-14T16:03:12.5383899Z Setting up curl (7.47.0-1ubuntu2.13) ...
---
2019-07-14T16:03:14.0960931Z Setting up libedit-dev:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-14T16:03:14.1345463Z Setting up libllvm6.0:amd64 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-14T16:03:14.1730720Z Setting up zlib1g-dev:amd64 (1:1.2.8.dfsg-2ubuntu4.1) ...
2019-07-14T16:03:14.2106476Z Setting up libssl-dev:amd64 (1.0.2g-1ubuntu4.15) ...
2019-07-14T16:03:14.2551708Z Setting up llvm-6.0-runtime (1:6.0-1ubuntu2~16.04.1) ...
2019-07-14T16:03:14.2886384Z mount: permission denied
2019-07-14T16:03:14.2889558Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-14T16:03:14.3021271Z Setting up llvm-6.0 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-14T16:03:14.3434696Z Setting up libffi-dev:amd64 (3.2.1-4) ...
2019-07-14T16:03:14.3810295Z Setting up llvm-6.0-dev (1:6.0-1ubuntu2~16.04.1) ...
2019-07-14T16:03:14.4207547Z Setting up llvm-6.0-tools (1:6.0-1ubuntu2~16.04.1) ...
2019-07-14T16:03:14.4625502Z Setting up pkg-config (0.29.1-0ubuntu1) ...
2019-07-14T16:03:14.5916980Z Processing triggers for ca-certificates (20170717~16.04.2) ...
2019-07-14T16:03:14.6077700Z Updating certificates in /etc/ssl/certs...
2019-07-14T16:03:16.2061698Z 148 added, 0 removed; done.
2019-07-14T16:03:16.2062475Z Running hooks in /etc/ca-certificates/update.d...
---
2019-07-14T16:03:49.5630717Z  ---> 29e4d4e109c6
2019-07-14T16:03:49.5670052Z Successfully built 29e4d4e109c6
2019-07-14T16:03:49.6999300Z Successfully tagged rust-ci:latest
2019-07-14T16:03:49.7782303Z Built container sha256:29e4d4e109c6ec928c9bca5751710e9d5c4431b1cad85085508db398ddcac983
2019-07-14T16:03:49.7799222Z Uploading finished image to https://.s3.amazonaws.com/docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624
2019-07-14T16:04:50.8385537Z upload failed: - to s3:///docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624 Parameter validation failed:
2019-07-14T16:04:50.8385975Z Invalid bucket name "": Bucket name must match the regex "^[a-zA-Z0-9.\-_]{1,255}$"
2019-07-14T16:04:51.8177599Z [CI_JOB_NAME=x86_64-gnu-llvm-6.0]
2019-07-14T16:04:51.8225353Z Starting sccache server...
2019-07-14T16:04:51.8691483Z configure: processing command line
2019-07-14T16:04:51.8692243Z configure: 
---
2019-07-14T16:34:31.4262338Z    Compiling rustc_data_structures v0.0.0 (/checkout/src/librustc_data_structures)
2019-07-14T16:34:34.7746733Z    Compiling arena v0.0.0 (/checkout/src/libarena)
2019-07-14T16:34:52.7910089Z    Compiling synstructure v0.10.2
2019-07-14T16:35:17.8293913Z    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
2019-07-14T16:35:18.3921192Z error: usage of `Vec::new()`
2019-07-14T16:35:18.3921642Z    --> src/librustc_macros/src/query.rs:182:24
2019-07-14T16:35:18.3921918Z     |
2019-07-14T16:35:18.3922203Z 182 |         let mut list = Vec::new();
2019-07-14T16:35:18.3922797Z     |                        ^^^^^^^^^^ help: use: `vec![]`
2019-07-14T16:35:18.3923062Z     |
2019-07-14T16:35:18.3923356Z     = note: `-D rustc::vec-new` implied by `-D rustc::internal`
2019-07-14T16:35:18.3923409Z 
2019-07-14T16:35:18.3923671Z error: usage of `Vec::new()`
2019-07-14T16:35:18.3923938Z    --> src/librustc_macros/src/query.rs:416:32
2019-07-14T16:35:18.3924157Z     |
2019-07-14T16:35:18.3924463Z 416 |     let mut no_force_queries = Vec::new();
2019-07-14T16:35:18.3924794Z     |                                ^^^^^^^^^^ help: use: `vec![]`
2019-07-14T16:35:18.3924844Z 
2019-07-14T16:35:18.3925095Z error: usage of `Vec::new()`
2019-07-14T16:35:18.3925584Z    --> src/librustc_macros/src/query.rs:453:34
2019-07-14T16:35:18.3925812Z     |
2019-07-14T16:35:18.3926126Z 453 |             let mut attributes = Vec::new();
2019-07-14T16:35:18.3926467Z     |                                  ^^^^^^^^^^ help: use: `vec![]`
2019-07-14T16:35:18.3926507Z 
2019-07-14T16:35:18.3926756Z error: usage of `Vec::new()`
2019-07-14T16:35:18.3927025Z   --> src/librustc_macros/src/symbols.rs:62:24
2019-07-14T16:35:18.3927542Z    |
2019-07-14T16:35:18.3927889Z 62 |         let mut list = Vec::new();
2019-07-14T16:35:18.3928221Z    |                        ^^^^^^^^^^ help: use: `vec![]`
2019-07-14T16:35:18.3994224Z error: aborting due to 4 previous errors
2019-07-14T16:35:18.3994349Z 
2019-07-14T16:35:18.4214394Z error: Could not compile `rustc_macros`.
2019-07-14T16:35:18.4214515Z 
2019-07-14T16:35:18.4214515Z 
2019-07-14T16:35:18.4214871Z To learn more, run the command again with --verbose.
2019-07-14T16:35:18.4276517Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
2019-07-14T16:35:18.4276617Z expected success, got: exit code: 101
2019-07-14T16:35:18.4281650Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-07-14T16:35:18.4281730Z Build completed unsuccessfully in 0:26:42
2019-07-14T16:35:19.3792275Z ##[error]Bash exited with code '1'.
2019-07-14T16:35:19.3825198Z ##[section]Starting: Checkout
2019-07-14T16:35:19.3826965Z ==============================================================================
2019-07-14T16:35:19.3827046Z Task         : Get sources
2019-07-14T16:35:19.3827100Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
