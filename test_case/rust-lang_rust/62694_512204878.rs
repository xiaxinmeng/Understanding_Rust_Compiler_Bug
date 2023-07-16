plain
2019-07-17T10:01:14.5054473Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-17T10:01:14.5239646Z ##[command]git config gc.auto 0
2019-07-17T10:01:14.5317908Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-17T10:01:14.5371901Z ##[command]git config --get-all http.proxy
2019-07-17T10:01:14.5512430Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62694/merge:refs/remotes/pull/62694/merge
---
2019-07-17T10:01:51.9529762Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-17T10:01:51.9530537Z 
2019-07-17T10:01:51.9531231Z   git checkout -b <new-branch-name>
2019-07-17T10:01:51.9531270Z 
2019-07-17T10:01:51.9531538Z HEAD is now at 9f0041ee6 Merge be9898e7c1a99319685bb7292f313af75518a8d2 into bf16480f9cf124630f4a4ffc6d8a57b72fbd5ce9
2019-07-17T10:01:51.9668854Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-17T10:01:51.9672287Z ==============================================================================
2019-07-17T10:01:51.9672354Z Task         : Bash
2019-07-17T10:01:51.9672407Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-17T10:03:46.3413988Z Attempting with retry: curl -y 30 -Y 10 --connect-timeout 30 -f -L -C - -o /tmp/rustci_docker_cache https://.s3.amazonaws.com/docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624
2019-07-17T10:03:46.3466786Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-17T10:03:46.3467333Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-17T10:03:46.3477471Z 
2019-07-17T10:03:46.3509353Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-17T10:03:47.3600238Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-17T10:03:47.3600465Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-17T10:03:47.3600509Z 
2019-07-17T10:03:47.3600509Z 
2019-07-17T10:03:47.3632740Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-17T10:03:49.3734164Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-17T10:03:49.3734408Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-17T10:03:49.3734523Z 
2019-07-17T10:03:49.3734523Z 
2019-07-17T10:03:49.3760561Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-17T10:03:52.3842438Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-17T10:03:52.3855053Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-17T10:03:52.3855190Z 
2019-07-17T10:03:52.3855190Z 
2019-07-17T10:03:52.3884380Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-17T10:03:56.3967664Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-17T10:03:56.3968518Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-17T10:03:56.3968922Z 
2019-07-17T10:03:56.3968922Z 
2019-07-17T10:03:56.3972356Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-17T10:03:56.3976732Z The command has failed after 5 attempts.
2019-07-17T10:03:56.4743682Z open /tmp/rustci_docker_cache: no such file or directory
2019-07-17T10:03:56.4772032Z Attempting with retry: docker build --rm -t rust-ci -f /home/vsts/work/1/s/src/ci/docker/x86_64-gnu-llvm-6.0/Dockerfile /home/vsts/work/1/s/src/ci/docker
2019-07-17T10:03:56.6231873Z Sending build context to Docker daemon  521.2kB
2019-07-17T10:03:56.6232009Z 
2019-07-17T10:03:56.6566468Z Step 1/8 : FROM ubuntu:16.04
---
2019-07-17T10:04:13.5045509Z Reading package lists...
2019-07-17T10:04:14.5126065Z Reading package lists...
2019-07-17T10:04:14.7136254Z Building dependency tree...
2019-07-17T10:04:14.7136374Z Reading state information...
2019-07-17T10:04:14.8335595Z The following additional packages will be installed:
2019-07-17T10:04:14.8336487Z   binfmt-support binutils bzip2 cmake-data cpp cpp-5 dpkg-dev g++-5 gcc gcc-5
2019-07-17T10:04:14.8336810Z   git-man libarchive13 libasan2 libasn1-8-heimdal libatomic1
2019-07-17T10:04:14.8337118Z   libbabeltrace-ctf1 libbabeltrace1 libbsd-dev libbsd0 libbz2-1.0 libc-dev-bin
2019-07-17T10:04:14.8337882Z   libedit2 liberror-perl libexpat1 libffi-dev libffi6 libgcc-5-dev libgdbm3
2019-07-17T10:04:14.8338171Z   libglib2.0-0 libgmp10 libgnutls30 libgomp1 libgssapi-krb5-2
2019-07-17T10:04:14.8338868Z   libgssapi3-heimdal libhcrypto4-heimdal libheimbase1-heimdal
2019-07-17T10:04:14.8339194Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-17T10:04:14.8339194Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-17T10:04:14.8339500Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-17T10:04:14.8339846Z   libkrb5-3 libkrb5support0 libldap-2.4-2 libllvm6.0 liblsan0 liblzo2-2
2019-07-17T10:04:14.8340148Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-17T10:04:14.8340436Z   libperl5.22 libpipeline1 libpython-stdlib libpython2.7-minimal
2019-07-17T10:04:14.8340831Z   libpython2.7-stdlib libpython3.5 libpython3.5-minimal libpython3.5-stdlib
2019-07-17T10:04:14.8341134Z   libquadmath0 libroken18-heimdal librtmp1 libsasl2-2 libsasl2-modules-db
2019-07-17T10:04:14.8341510Z   libsqlite3-0 libssl1.0.0 libstdc++-5-dev libtasn1-6 libtinfo-dev libtsan0
2019-07-17T10:04:14.8341853Z   libubsan0 libwind0-heimdal libxml2 linux-libc-dev llvm-6.0 llvm-6.0-dev
2019-07-17T10:04:14.8342171Z   llvm-6.0-runtime mime-support openssl patch perl perl-modules-5.22 python
2019-07-17T10:04:14.8342453Z   python-minimal python2.7-minimal
2019-07-17T10:04:14.8353448Z Suggested packages:
2019-07-17T10:04:14.8354231Z   binutils-doc bzip2-doc codeblocks eclipse ninja-build cpp-doc gcc-5-locales
2019-07-17T10:04:14.8354835Z   debian-keyring g++-multilib g++-5-multilib gcc-5-doc libstdc++6-5-dbg
2019-07-17T10:04:14.8355396Z   gcc-multilib manpages-dev autoconf automake libtool flex bison gcc-doc
2019-07-17T10:04:14.8356404Z   libasan2-dbg liblsan0-dbg libtsan0-dbg libubsan0-dbg libcilkrts5-dbg
2019-07-17T10:04:14.8356892Z   libmpx0-dbg libquadmath0-dbg gdb-doc gettext-base git-daemon-run
2019-07-17T10:04:14.8356892Z   libmpx0-dbg libquadmath0-dbg gdb-doc gettext-base git-daemon-run
2019-07-17T10:04:14.8357524Z   | git-daemon-sysvinit git-doc git-el git-email git-gui gitk gitweb git-arch
2019-07-17T10:04:14.8358092Z   git-cvs git-mediawiki git-svn lrzip glibc-doc gnutls-bin krb5-doc krb5-user
2019-07-17T10:04:14.8358606Z   libstdc++-5-doc llvm-6.0-doc make-doc ed diffutils-doc perl-doc
2019-07-17T10:04:14.8359107Z   libterm-readline-gnu-perl | libterm-readline-perl-perl python-doc python-tk
2019-07-17T10:04:14.8359585Z   python2.7-doc
2019-07-17T10:04:14.8359782Z Recommended packages:
2019-07-17T10:04:14.8362669Z   build-essential fakeroot libalgorithm-merge-perl libc-dbg gdbserver less
2019-07-17T10:04:14.8365718Z   rsync ssh-client manpages manpages-dev libfile-fcntllock-perl
2019-07-17T10:04:14.8366447Z   libglib2.0-data shared-mime-info xdg-user-dirs krb5-locales libsasl2-modules
2019-07-17T10:04:14.8366981Z   libssl-doc xml-core netbase rename
2019-07-17T10:04:14.8367256Z The following NEW packages will be installed:
2019-07-17T10:04:14.8368108Z   binfmt-support binutils bzip2 ca-certificates cmake cmake-data cpp cpp-5
2019-07-17T10:04:14.8368677Z   curl dpkg-dev file g++ g++-5 gcc gcc-5 gdb git git-man libarchive13 libasan2
2019-07-17T10:04:14.8369158Z   libasn1-8-heimdal libatomic1 libbabeltrace-ctf1 libbabeltrace1 libbsd-dev
2019-07-17T10:04:14.8369851Z   libbsd0 libc-dev-bin libc6-dev libcc1-0 libcilkrts5 libcurl3 libcurl3-gnutls
2019-07-17T10:04:14.8370489Z   libdpkg-perl libedit-dev libedit2 liberror-perl libexpat1 libffi-dev libffi6
2019-07-17T10:04:14.8371082Z   libgcc-5-dev libgdbm3 libglib2.0-0 libgmp10 libgnutls30 libgomp1
2019-07-17T10:04:14.8372193Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-17T10:04:14.8372677Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-17T10:04:14.8373237Z   libkrb5-3 libkrb5support0 libldap-2.4-2 libllvm6.0 liblsan0 liblzo2-2
2019-07-17T10:04:14.8373722Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-17T10:04:14.8373722Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-17T10:04:14.8374244Z   libperl5.22 libpipeline1 libpython-stdlib libpython2.7-minimal
2019-07-17T10:04:14.8374744Z   libpython2.7-stdlib libpython3.5 libpython3.5-minimal libpython3.5-stdlib
2019-07-17T10:04:14.8375512Z   libquadmath0 libroken18-heimdal librtmp1 libsasl2-2 libsasl2-modules-db
2019-07-17T10:04:14.8376084Z   libsqlite3-0 libssl-dev libssl1.0.0 libstdc++-5-dev libtasn1-6 libtinfo-dev
2019-07-17T10:04:14.8376567Z   libtsan0 libubsan0 libwind0-heimdal libxml2 linux-libc-dev llvm-6.0
2019-07-17T10:04:14.8377055Z   llvm-6.0-dev llvm-6.0-runtime llvm-6.0-tools make mime-support openssl patch
2019-07-17T10:04:14.8377595Z   perl perl-modules-5.22 pkg-config python python-minimal python2.7
2019-07-17T10:04:14.8378191Z   python2.7-minimal sudo xz-utils zlib1g-dev
2019-07-17T10:04:14.8378537Z The following packages will be upgraded:
2019-07-17T10:04:15.2370441Z 1 upgraded, 115 newly installed, 0 to remove and 5 not upgraded.
2019-07-17T10:04:15.2370608Z Need to get 121 MB of archives.
2019-07-17T10:04:15.2370802Z After this operation, 592 MB of additional disk space will be used.
2019-07-17T10:04:15.2371905Z Get:1 http://archive.ubuntu.com/ubuntu xenial/main amd64 libgdbm3 amd64 1.8.3-13.1 [16.9 kB]
---
2019-07-17T10:04:18.1594465Z Get:61 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 sudo amd64 1.8.16-0ubuntu1.7 [390 kB]
2019-07-17T10:04:18.1650196Z Get:62 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 openssl amd64 1.0.2g-1ubuntu4.15 [492 kB]
2019-07-17T10:04:18.2341240Z Get:63 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 ca-certificates all 20170717~16.04.2 [167 kB]
2019-07-17T10:04:18.2372090Z Get:64 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 libcurl3-gnutls amd64 7.47.0-1ubuntu2.13 [184 kB]
2019-07-17T10:04:18.2410234Z Get:65 http://archive.ubuntu.com/ubuntu xenial/main amd64 libedit2 amd64 3.1-20150325-1ubuntu2 [76.5 kB]
2019-07-17T10:04:18.2431952Z Get:66 http://archive.ubuntu.com/ubuntu xenial/main amd64 libpipeline1 amd64 1.4.1-2 [24.6 kB]
2019-07-17T10:04:18.2444117Z Get:67 http://archive.ubuntu.com/ubuntu xenial/main amd64 binfmt-support amd64 2.1.6-1 [50.7 kB]
2019-07-17T10:04:18.3646374Z Get:69 http://archive.ubuntu.com/ubuntu xenial/main amd64 libisl15 amd64 0.16.1-1 [524 kB]
2019-07-17T10:04:18.3830322Z Get:70 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 cpp-5 amd64 5.4.0-6ubuntu1~16.04.11 [7660 kB]
2019-07-17T10:04:18.7036821Z Get:71 http://archive.ubuntu.com/ubuntu xenial/main amd64 cpp amd64 4:5.3.1-1ubuntu1 [27.7 kB]
2019-07-17T10:04:18.7059449Z Get:72 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 curl amd64 7.47.0-1ubuntu2.13 [139 kB]
---
2019-07-17T10:04:39.6906750Z Unpacking ca-certificates (20170717~16.04.2) ...
2019-07-17T10:04:39.8791067Z Selecting previously unselected package libcurl3-gnutls:amd64.
2019-07-17T10:04:39.8807754Z Preparing to unpack .../libcurl3-gnutls_7.47.0-1ubuntu2.13_amd64.deb ...
2019-07-17T10:04:39.8957011Z Unpacking libcurl3-gnutls:amd64 (7.47.0-1ubuntu2.13) ...
2019-07-17T10:04:40.0495905Z Selecting previously unselected package libedit2:amd64.
2019-07-17T10:04:40.0510005Z Preparing to unpack .../libedit2_3.1-20150325-1ubuntu2_amd64.deb ...
2019-07-17T10:04:40.0678791Z Unpacking libedit2:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-17T10:04:40.1899479Z Selecting previously unselected package libpipeline1:amd64.
2019-07-17T10:04:40.1910981Z Preparing to unpack .../libpipeline1_1.4.1-2_amd64.deb ...
2019-07-17T10:04:40.2070223Z Unpacking libpipeline1:amd64 (1.4.1-2) ...
2019-07-17T10:04:40.3262824Z Selecting previously unselected package binfmt-support.
2019-07-17T10:04:40.3275487Z Preparing to unpack .../binfmt-support_2.1.6-1_amd64.deb ...
2019-07-17T10:04:40.3418050Z Unpacking binfmt-support (2.1.6-1) ...
2019-07-17T10:04:40.4770575Z Preparing to unpack .../binutils_2.26.1-1ubuntu1~16.04.8_amd64.deb ...
2019-07-17T10:04:40.4938844Z Unpacking binutils (2.26.1-1ubuntu1~16.04.8) ...
2019-07-17T10:04:41.0040395Z Selecting previously unselected package libisl15:amd64.
2019-07-17T10:04:41.0055731Z Preparing to unpack .../libisl15_0.16.1-1_amd64.deb ...
---
2019-07-17T10:04:55.6944568Z Unpacking zlib1g-dev:amd64 (1:1.2.8.dfsg-2ubuntu4.1) ...
2019-07-17T10:04:55.8023238Z Selecting previously unselected package libssl-dev:amd64.
2019-07-17T10:04:55.8046421Z Preparing to unpack .../libssl-dev_1.0.2g-1ubuntu4.15_amd64.deb ...
2019-07-17T10:04:55.8253242Z Unpacking libssl-dev:amd64 (1.0.2g-1ubuntu4.15) ...
2019-07-17T10:04:56.1244873Z Selecting previously unselected package llvm-6.0-runtime.
2019-07-17T10:04:56.1263838Z Preparing to unpack .../llvm-6.0-runtime_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-17T10:04:56.1398477Z Unpacking llvm-6.0-runtime (1:6.0-1ubuntu2~16.04.1) ...
2019-07-17T10:04:56.2614898Z Selecting previously unselected package llvm-6.0.
2019-07-17T10:04:56.2634154Z Preparing to unpack .../llvm-6.0_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-17T10:04:56.2772875Z Unpacking llvm-6.0 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-17T10:04:56.9742689Z Selecting previously unselected package libffi-dev:amd64.
2019-07-17T10:04:56.9763539Z Preparing to unpack .../libffi-dev_3.2.1-4_amd64.deb ...
2019-07-17T10:04:56.9948686Z Unpacking libffi-dev:amd64 (3.2.1-4) ...
2019-07-17T10:04:57.1212285Z Selecting previously unselected package llvm-6.0-dev.
2019-07-17T10:04:57.1228934Z Preparing to unpack .../llvm-6.0-dev_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-17T10:04:57.1362213Z Unpacking llvm-6.0-dev (1:6.0-1ubuntu2~16.04.1) ...
2019-07-17T10:05:01.7299526Z Selecting previously unselected package llvm-6.0-tools.
2019-07-17T10:05:01.7334482Z Preparing to unpack .../llvm-6.0-tools_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-17T10:05:01.7474048Z Unpacking llvm-6.0-tools (1:6.0-1ubuntu2~16.04.1) ...
2019-07-17T10:05:01.9196895Z Selecting previously unselected package pkg-config.
2019-07-17T10:05:01.9220100Z Preparing to unpack .../pkg-config_0.29.1-0ubuntu1_amd64.deb ...
2019-07-17T10:05:01.9945340Z Processing triggers for libc-bin (2.23-0ubuntu11) ...
2019-07-17T10:05:02.0504509Z Processing triggers for systemd (229-4ubuntu21.21) ...
2019-07-17T10:05:02.3860835Z Setting up libgdbm3:amd64 (1.8.3-13.1) ...
2019-07-17T10:05:02.4553327Z Setting up libffi6:amd64 (3.2.1-4) ...
---
2019-07-17T10:05:06.1515183Z debconf: unable to initialize frontend: Dialog
2019-07-17T10:05:06.1518224Z debconf: (TERM is not set, so the dialog frontend is not usable.)
2019-07-17T10:05:06.1518328Z debconf: falling back to frontend: Readline
2019-07-17T10:05:06.6891544Z Setting up libcurl3-gnutls:amd64 (7.47.0-1ubuntu2.13) ...
2019-07-17T10:05:06.7281238Z Setting up libedit2:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-17T10:05:06.7695105Z Setting up libpipeline1:amd64 (1.4.1-2) ...
2019-07-17T10:05:06.8086268Z Setting up binfmt-support (2.1.6-1) ...
2019-07-17T10:05:06.8808056Z mount: permission denied
2019-07-17T10:05:06.8815116Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-17T10:05:06.8827964Z mount: permission denied
2019-07-17T10:05:06.8831810Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-17T10:05:07.0430200Z invoke-rc.d: could not determine current runlevel
2019-07-17T10:05:07.0458699Z invoke-rc.d: policy-rc.d denied execution of start.
2019-07-17T10:05:07.1088567Z Setting up libisl15:amd64 (0.16.1-1) ...
2019-07-17T10:05:07.1500467Z Setting up cpp-5 (5.4.0-6ubuntu1~16.04.11) ...
2019-07-17T10:05:07.1909919Z Setting up cpp (4:5.3.1-1ubuntu1) ...
2019-07-17T10:05:07.2448498Z Setting up curl (7.47.0-1ubuntu2.13) ...
---
2019-07-17T10:05:08.9449119Z Setting up libedit-dev:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-17T10:05:08.9852279Z Setting up libllvm6.0:amd64 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-17T10:05:09.0308618Z Setting up zlib1g-dev:amd64 (1:1.2.8.dfsg-2ubuntu4.1) ...
2019-07-17T10:05:09.0714017Z Setting up libssl-dev:amd64 (1.0.2g-1ubuntu4.15) ...
2019-07-17T10:05:09.1188439Z Setting up llvm-6.0-runtime (1:6.0-1ubuntu2~16.04.1) ...
2019-07-17T10:05:09.1492499Z mount: permission denied
2019-07-17T10:05:09.1495087Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-17T10:05:09.1646197Z Setting up llvm-6.0 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-17T10:05:09.2040908Z Setting up libffi-dev:amd64 (3.2.1-4) ...
2019-07-17T10:05:09.2472200Z Setting up llvm-6.0-dev (1:6.0-1ubuntu2~16.04.1) ...
2019-07-17T10:05:09.2927992Z Setting up llvm-6.0-tools (1:6.0-1ubuntu2~16.04.1) ...
2019-07-17T10:05:09.3405805Z Setting up pkg-config (0.29.1-0ubuntu1) ...
2019-07-17T10:05:09.4797874Z Processing triggers for ca-certificates (20170717~16.04.2) ...
2019-07-17T10:05:09.4975491Z Updating certificates in /etc/ssl/certs...
2019-07-17T10:05:11.1088131Z 148 added, 0 removed; done.
2019-07-17T10:05:11.1089056Z Running hooks in /etc/ca-certificates/update.d...
---
2019-07-17T10:05:44.7975143Z Removing intermediate container 89c855699953
2019-07-17T10:05:44.7975977Z  ---> 7832593be80e
2019-07-17T10:05:44.8010652Z Successfully built 7832593be80e
2019-07-17T10:05:44.9797148Z Successfully tagged rust-ci:latest
2019-07-17T10:05:45.0968905Z Built container sha256:7832593be80e7483aece1912f1c2c5bd02e5f7c01975c331f1acff2f6dd1b801
2019-07-17T10:05:45.0985665Z Uploading finished image to https://.s3.amazonaws.com/docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624
2019-07-17T10:06:46.5645740Z upload failed: - to s3:///docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624 Parameter validation failed:
2019-07-17T10:06:46.5646207Z Invalid bucket name "": Bucket name must match the regex "^[a-zA-Z0-9.\-_]{1,255}$"
2019-07-17T10:06:47.6197340Z [CI_JOB_NAME=x86_64-gnu-llvm-6.0]
2019-07-17T10:06:47.6240668Z Starting sccache server...
2019-07-17T10:06:47.6677710Z configure: processing command line
2019-07-17T10:06:47.6677993Z configure: 
---
2019-07-17T10:40:04.2706298Z    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
2019-07-17T10:47:59.2941454Z    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
2019-07-17T10:49:43.9447289Z    Compiling rustc_allocator v0.0.0 (/checkout/src/librustc_allocator)
2019-07-17T10:50:02.8072228Z    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
2019-07-17T10:50:11.0412670Z error: passing `TyCtxt<'tcx>` by reference
2019-07-17T10:50:11.0413661Z    --> src/librustc_typeck/collect.rs:163:30
2019-07-17T10:50:11.0414157Z     |
2019-07-17T10:50:11.0414782Z 163 | fn bad_placeholder_type(tcx: &TyCtxt<'tcx>, span: Span) -> errors::DiagnosticBuilder<'tcx> {
2019-07-17T10:50:11.0415357Z     |                              ^^^^^^^^^^^^^ help: try passing by value: `TyCtxt<'tcx>`
2019-07-17T10:50:11.0415814Z     |
2019-07-17T10:50:11.0416300Z     = note: `-D rustc::ty-pass-by-reference` implied by `-D rustc::internal`
2019-07-17T10:50:11.1691315Z error: aborting due to previous error
2019-07-17T10:50:11.1695122Z 
2019-07-17T10:50:11.2556862Z error: Could not compile `rustc_typeck`.
2019-07-17T10:50:11.2573657Z warning: build failed, waiting for other jobs to finish...
2019-07-17T10:50:11.2573657Z warning: build failed, waiting for other jobs to finish...
2019-07-17T10:53:03.2468333Z error: build failed
2019-07-17T10:53:03.2489798Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
2019-07-17T10:53:03.2490218Z expected success, got: exit code: 101
2019-07-17T10:53:03.2498985Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-07-17T10:53:03.2499205Z Build completed unsuccessfully in 0:42:34
2019-07-17T10:53:04.9240161Z ##[error]Bash exited with code '1'.
2019-07-17T10:53:04.9284055Z ##[section]Starting: Checkout
2019-07-17T10:53:04.9285769Z ==============================================================================
2019-07-17T10:53:04.9285832Z Task         : Get sources
2019-07-17T10:53:04.9285884Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
