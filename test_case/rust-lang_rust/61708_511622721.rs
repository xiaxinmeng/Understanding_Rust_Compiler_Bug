plain
2019-07-16T00:35:38.9045843Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-16T00:35:38.9221756Z ##[command]git config gc.auto 0
2019-07-16T00:35:38.9296874Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-16T00:35:38.9353876Z ##[command]git config --get-all http.proxy
2019-07-16T00:35:38.9491635Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/61708/merge:refs/remotes/pull/61708/merge
---
2019-07-16T00:36:11.8966577Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-16T00:36:11.8966841Z 
2019-07-16T00:36:11.8967106Z   git checkout -b <new-branch-name>
2019-07-16T00:36:11.8967138Z 
2019-07-16T00:36:11.8967187Z HEAD is now at e8977408a Merge 15709601a609b2dba376ecbae59ea8b4a5249eee into 4b65a86ebace8600c8e269e8bfe3365cdc460e68
2019-07-16T00:36:11.9102762Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-16T00:36:11.9105990Z ==============================================================================
2019-07-16T00:36:11.9106051Z Task         : Bash
2019-07-16T00:36:11.9106116Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-16T00:38:01.3578925Z Attempting with retry: curl -y 30 -Y 10 --connect-timeout 30 -f -L -C - -o /tmp/rustci_docker_cache https://.s3.amazonaws.com/docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624
2019-07-16T00:38:01.3642482Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-16T00:38:01.3643493Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-16T00:38:01.3643542Z 
2019-07-16T00:38:01.3685483Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-16T00:38:02.3757019Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-16T00:38:02.3757728Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-16T00:38:02.3757871Z 
2019-07-16T00:38:02.3757871Z 
2019-07-16T00:38:02.3804569Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-16T00:38:04.3868507Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-16T00:38:04.3868891Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-16T00:38:04.3869135Z 
2019-07-16T00:38:04.3869135Z 
2019-07-16T00:38:04.3911265Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-16T00:38:07.3982795Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-16T00:38:07.3983672Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-16T00:38:07.3983745Z 
2019-07-16T00:38:07.3983745Z 
2019-07-16T00:38:07.4027401Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-16T00:38:11.4099685Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-16T00:38:11.4100190Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-16T00:38:11.4100381Z 
2019-07-16T00:38:11.4100381Z 
2019-07-16T00:38:11.4143708Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-16T00:38:11.4147340Z The command has failed after 5 attempts.
2019-07-16T00:38:11.4701073Z open /tmp/rustci_docker_cache: no such file or directory
2019-07-16T00:38:12.4052382Z Attempting with retry: docker build --rm -t rust-ci -f /home/vsts/work/1/s/src/ci/docker/x86_64-gnu-llvm-6.0/Dockerfile /home/vsts/work/1/s/src/ci/docker
2019-07-16T00:38:12.4052868Z Sending build context to Docker daemon  521.7kB
2019-07-16T00:38:12.4052910Z 
2019-07-16T00:38:12.4052957Z Step 1/8 : FROM ubuntu:16.04
---
2019-07-16T00:38:28.1010131Z Reading package lists...
2019-07-16T00:38:29.0763891Z Reading package lists...
2019-07-16T00:38:29.2704723Z Building dependency tree...
2019-07-16T00:38:29.2705387Z Reading state information...
2019-07-16T00:38:29.3930285Z The following additional packages will be installed:
2019-07-16T00:38:29.3932466Z   binfmt-support binutils bzip2 cmake-data cpp cpp-5 dpkg-dev g++-5 gcc gcc-5
2019-07-16T00:38:29.3933359Z   git-man libarchive13 libasan2 libasn1-8-heimdal libatomic1
2019-07-16T00:38:29.3933848Z   libbabeltrace-ctf1 libbabeltrace1 libbsd-dev libbsd0 libbz2-1.0 libc-dev-bin
2019-07-16T00:38:29.3934754Z   libedit2 liberror-perl libexpat1 libffi-dev libffi6 libgcc-5-dev libgdbm3
2019-07-16T00:38:29.3935206Z   libglib2.0-0 libgmp10 libgnutls30 libgomp1 libgssapi-krb5-2
2019-07-16T00:38:29.3951093Z   libgssapi3-heimdal libhcrypto4-heimdal libheimbase1-heimdal
2019-07-16T00:38:29.3952151Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-16T00:38:29.3952151Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-16T00:38:29.3954269Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-16T00:38:29.3955199Z   libkrb5-3 libkrb5support0 libldap-2.4-2 libllvm6.0 liblsan0 liblzo2-2
2019-07-16T00:38:29.3955827Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-16T00:38:29.3956203Z   libperl5.22 libpipeline1 libpython-stdlib libpython2.7-minimal
2019-07-16T00:38:29.3956660Z   libpython2.7-stdlib libpython3.5 libpython3.5-minimal libpython3.5-stdlib
2019-07-16T00:38:29.3957037Z   libquadmath0 libroken18-heimdal librtmp1 libsasl2-2 libsasl2-modules-db
2019-07-16T00:38:29.3957459Z   libsqlite3-0 libssl1.0.0 libstdc++-5-dev libtasn1-6 libtinfo-dev libtsan0
2019-07-16T00:38:29.3958268Z   libubsan0 libwind0-heimdal libxml2 linux-libc-dev llvm-6.0 llvm-6.0-dev
2019-07-16T00:38:29.3959931Z   llvm-6.0-runtime mime-support openssl patch perl perl-modules-5.22 python
2019-07-16T00:38:29.3960400Z   python-minimal python2.7-minimal
2019-07-16T00:38:29.3960584Z Suggested packages:
2019-07-16T00:38:29.3960941Z   binutils-doc bzip2-doc codeblocks eclipse ninja-build cpp-doc gcc-5-locales
2019-07-16T00:38:29.3961373Z   debian-keyring g++-multilib g++-5-multilib gcc-5-doc libstdc++6-5-dbg
2019-07-16T00:38:29.3961745Z   gcc-multilib manpages-dev autoconf automake libtool flex bison gcc-doc
2019-07-16T00:38:29.3962842Z   libasan2-dbg liblsan0-dbg libtsan0-dbg libubsan0-dbg libcilkrts5-dbg
2019-07-16T00:38:29.3963253Z   libmpx0-dbg libquadmath0-dbg gdb-doc gettext-base git-daemon-run
2019-07-16T00:38:29.3963253Z   libmpx0-dbg libquadmath0-dbg gdb-doc gettext-base git-daemon-run
2019-07-16T00:38:29.3963729Z   | git-daemon-sysvinit git-doc git-el git-email git-gui gitk gitweb git-arch
2019-07-16T00:38:29.3964165Z   git-cvs git-mediawiki git-svn lrzip glibc-doc gnutls-bin krb5-doc krb5-user
2019-07-16T00:38:29.3964571Z   libstdc++-5-doc llvm-6.0-doc make-doc ed diffutils-doc perl-doc
2019-07-16T00:38:29.3965055Z   libterm-readline-gnu-perl | libterm-readline-perl-perl python-doc python-tk
2019-07-16T00:38:29.3965417Z   python2.7-doc
2019-07-16T00:38:29.3965798Z Recommended packages:
2019-07-16T00:38:29.3966155Z   build-essential fakeroot libalgorithm-merge-perl libc-dbg gdbserver less
2019-07-16T00:38:29.3966519Z   rsync ssh-client manpages manpages-dev libfile-fcntllock-perl
2019-07-16T00:38:29.3966956Z   libglib2.0-data shared-mime-info xdg-user-dirs krb5-locales libsasl2-modules
2019-07-16T00:38:29.3967300Z   libssl-doc xml-core netbase rename
2019-07-16T00:38:29.3967461Z The following NEW packages will be installed:
2019-07-16T00:38:29.3967913Z   binfmt-support binutils bzip2 ca-certificates cmake cmake-data cpp cpp-5
2019-07-16T00:38:29.3968291Z   curl dpkg-dev file g++ g++-5 gcc gcc-5 gdb git git-man libarchive13 libasan2
2019-07-16T00:38:29.3968724Z   libasn1-8-heimdal libatomic1 libbabeltrace-ctf1 libbabeltrace1 libbsd-dev
2019-07-16T00:38:29.3969117Z   libbsd0 libc-dev-bin libc6-dev libcc1-0 libcilkrts5 libcurl3 libcurl3-gnutls
2019-07-16T00:38:29.3969639Z   libdpkg-perl libedit-dev libedit2 liberror-perl libexpat1 libffi-dev libffi6
2019-07-16T00:38:29.3970138Z   libgcc-5-dev libgdbm3 libglib2.0-0 libgmp10 libgnutls30 libgomp1
2019-07-16T00:38:29.3970904Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-16T00:38:29.3971338Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-16T00:38:29.3971710Z   libkrb5-3 libkrb5support0 libldap-2.4-2 libllvm6.0 liblsan0 liblzo2-2
2019-07-16T00:38:29.3972123Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-16T00:38:29.3972123Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-16T00:38:29.3972805Z   libperl5.22 libpipeline1 libpython-stdlib libpython2.7-minimal
2019-07-16T00:38:29.3973863Z   libpython2.7-stdlib libpython3.5 libpython3.5-minimal libpython3.5-stdlib
2019-07-16T00:38:29.3974191Z   libquadmath0 libroken18-heimdal librtmp1 libsasl2-2 libsasl2-modules-db
2019-07-16T00:38:29.3974461Z   libsqlite3-0 libssl-dev libssl1.0.0 libstdc++-5-dev libtasn1-6 libtinfo-dev
2019-07-16T00:38:29.3974711Z   libtsan0 libubsan0 libwind0-heimdal libxml2 linux-libc-dev llvm-6.0
2019-07-16T00:38:29.3975021Z   llvm-6.0-dev llvm-6.0-runtime llvm-6.0-tools make mime-support openssl patch
2019-07-16T00:38:29.3975266Z   perl perl-modules-5.22 pkg-config python python-minimal python2.7
2019-07-16T00:38:29.3975494Z   python2.7-minimal sudo xz-utils zlib1g-dev
2019-07-16T00:38:29.3975591Z The following packages will be upgraded:
2019-07-16T00:38:29.8161036Z 1 upgraded, 115 newly installed, 0 to remove and 5 not upgraded.
2019-07-16T00:38:29.8162907Z Need to get 121 MB of archives.
2019-07-16T00:38:29.8163232Z After this operation, 592 MB of additional disk space will be used.
2019-07-16T00:38:29.8164048Z Get:1 http://archive.ubuntu.com/ubuntu xenial/main amd64 libgdbm3 amd64 1.8.3-13.1 [16.9 kB]
---
2019-07-16T00:38:32.2706464Z Get:61 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 sudo amd64 1.8.16-0ubuntu1.7 [390 kB]
2019-07-16T00:38:32.3451465Z Get:62 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 openssl amd64 1.0.2g-1ubuntu4.15 [492 kB]
2019-07-16T00:38:32.3520050Z Get:63 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 ca-certificates all 20170717~16.04.2 [167 kB]
2019-07-16T00:38:32.3548227Z Get:64 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 libcurl3-gnutls amd64 7.47.0-1ubuntu2.13 [184 kB]
2019-07-16T00:38:32.3577752Z Get:65 http://archive.ubuntu.com/ubuntu xenial/main amd64 libedit2 amd64 3.1-20150325-1ubuntu2 [76.5 kB]
2019-07-16T00:38:32.3591171Z Get:66 http://archive.ubuntu.com/ubuntu xenial/main amd64 libpipeline1 amd64 1.4.1-2 [24.6 kB]
2019-07-16T00:38:32.3598077Z Get:67 http://archive.ubuntu.com/ubuntu xenial/main amd64 binfmt-support amd64 2.1.6-1 [50.7 kB]
2019-07-16T00:38:32.4878039Z Get:69 http://archive.ubuntu.com/ubuntu xenial/main amd64 libisl15 amd64 0.16.1-1 [524 kB]
2019-07-16T00:38:32.4934873Z Get:70 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 cpp-5 amd64 5.4.0-6ubuntu1~16.04.11 [7660 kB]
2019-07-16T00:38:32.8370719Z Get:71 http://archive.ubuntu.com/ubuntu xenial/main amd64 cpp amd64 4:5.3.1-1ubuntu1 [27.7 kB]
2019-07-16T00:38:32.8371080Z Get:72 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 curl amd64 7.47.0-1ubuntu2.13 [139 kB]
---
2019-07-16T00:38:51.3913932Z Unpacking ca-certificates (20170717~16.04.2) ...
2019-07-16T00:38:51.5391967Z Selecting previously unselected package libcurl3-gnutls:amd64.
2019-07-16T00:38:51.5397873Z Preparing to unpack .../libcurl3-gnutls_7.47.0-1ubuntu2.13_amd64.deb ...
2019-07-16T00:38:51.5537159Z Unpacking libcurl3-gnutls:amd64 (7.47.0-1ubuntu2.13) ...
2019-07-16T00:38:51.6665971Z Selecting previously unselected package libedit2:amd64.
2019-07-16T00:38:51.6684042Z Preparing to unpack .../libedit2_3.1-20150325-1ubuntu2_amd64.deb ...
2019-07-16T00:38:51.6789946Z Unpacking libedit2:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-16T00:38:51.7727888Z Selecting previously unselected package libpipeline1:amd64.
2019-07-16T00:38:51.7742937Z Preparing to unpack .../libpipeline1_1.4.1-2_amd64.deb ...
2019-07-16T00:38:51.7856666Z Unpacking libpipeline1:amd64 (1.4.1-2) ...
2019-07-16T00:38:51.8822718Z Selecting previously unselected package binfmt-support.
2019-07-16T00:38:51.8836500Z Preparing to unpack .../binfmt-support_2.1.6-1_amd64.deb ...
2019-07-16T00:38:51.9000964Z Unpacking binfmt-support (2.1.6-1) ...
2019-07-16T00:38:51.9966585Z Preparing to unpack .../binutils_2.26.1-1ubuntu1~16.04.8_amd64.deb ...
2019-07-16T00:38:52.0070515Z Unpacking binutils (2.26.1-1ubuntu1~16.04.8) ...
2019-07-16T00:38:52.5023245Z Selecting previously unselected package libisl15:amd64.
2019-07-16T00:38:52.5041174Z Preparing to unpack .../libisl15_0.16.1-1_amd64.deb ...
---
2019-07-16T00:39:03.9738723Z Selecting previously unselected package libtinfo-dev:amd64.
2019-07-16T00:39:03.9758727Z Preparing to unpack .../libtinfo-dev_6.0+20160213-1ubuntu1_amd64.deb ...
2019-07-16T00:39:03.9912314Z Unpacking libtinfo-dev:amd64 (6.0+20160213-1ubuntu1) ...
2019-07-16T00:39:04.0958914Z Selecting previously unselected package libedit-dev:amd64.
2019-07-16T00:39:04.0980112Z Preparing to unpack .../libedit-dev_3.1-20150325-1ubuntu2_amd64.deb ...
2019-07-16T00:39:04.1139266Z Unpacking libedit-dev:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-16T00:39:04.2395485Z Selecting previously unselected package libllvm6.0:amd64.
2019-07-16T00:39:04.2416911Z Preparing to unpack .../libllvm6.0_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-16T00:39:04.2584606Z Unpacking libllvm6.0:amd64 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-16T00:39:07.2335829Z Selecting previously unselected package zlib1g-dev:amd64.
2019-07-16T00:39:07.2356033Z Preparing to unpack .../zlib1g-dev_1%3a1.2.8.dfsg-2ubuntu4.1_amd64.deb ...
2019-07-16T00:39:07.2572728Z Unpacking zlib1g-dev:amd64 (1:1.2.8.dfsg-2ubuntu4.1) ...
2019-07-16T00:39:07.4097683Z Preparing to unpack .../libssl-dev_1.0.2g-1ubuntu4.15_amd64.deb ...
2019-07-16T00:39:07.4311597Z Unpacking libssl-dev:amd64 (1.0.2g-1ubuntu4.15) ...
2019-07-16T00:39:07.7583303Z Selecting previously unselected package llvm-6.0-runtime.
2019-07-16T00:39:07.7583303Z Selecting previously unselected package llvm-6.0-runtime.
2019-07-16T00:39:07.7603735Z Preparing to unpack .../llvm-6.0-runtime_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-16T00:39:07.7795108Z Unpacking llvm-6.0-runtime (1:6.0-1ubuntu2~16.04.1) ...
2019-07-16T00:39:07.9258412Z Selecting previously unselected package llvm-6.0.
2019-07-16T00:39:07.9276870Z Preparing to unpack .../llvm-6.0_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-16T00:39:07.9545192Z Unpacking llvm-6.0 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-16T00:39:08.5772794Z Selecting previously unselected package libffi-dev:amd64.
2019-07-16T00:39:08.5792946Z Preparing to unpack .../libffi-dev_3.2.1-4_amd64.deb ...
2019-07-16T00:39:08.5904246Z Unpacking libffi-dev:amd64 (3.2.1-4) ...
2019-07-16T00:39:08.6962216Z Selecting previously unselected package llvm-6.0-dev.
2019-07-16T00:39:08.6980800Z Preparing to unpack .../llvm-6.0-dev_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-16T00:39:08.7098620Z Unpacking llvm-6.0-dev (1:6.0-1ubuntu2~16.04.1) ...
2019-07-16T00:39:13.5105241Z Selecting previously unselected package llvm-6.0-tools.
2019-07-16T00:39:13.5105873Z Preparing to unpack .../llvm-6.0-tools_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-16T00:39:13.5106120Z Unpacking llvm-6.0-tools (1:6.0-1ubuntu2~16.04.1) ...
2019-07-16T00:39:13.5106593Z Selecting previously unselected package pkg-config.
2019-07-16T00:39:13.5106887Z Preparing to unpack .../pkg-config_0.29.1-0ubuntu1_amd64.deb ...
2019-07-16T00:39:13.5107384Z Unpacking pkg-config (0.29.1-0ubuntu1) ...
2019-07-16T00:39:13.5983490Z Processing triggers for systemd (229-4ubuntu21.21) ...
2019-07-16T00:39:13.8870214Z Setting up libgdbm3:amd64 (1.8.3-13.1) ...
2019-07-16T00:39:13.9465668Z Setting up libffi6:amd64 (3.2.1-4) ...
2019-07-16T00:39:13.9806672Z Setting up libglib2.0-0:amd64 (2.48.2-0ubuntu4.3) ...
---
2019-07-16T00:39:17.1940973Z debconf: unable to initialize frontend: Dialog
2019-07-16T00:39:17.1941190Z debconf: (TERM is not set, so the dialog frontend is not usable.)
2019-07-16T00:39:17.1941242Z debconf: falling back to frontend: Readline
2019-07-16T00:39:17.6914816Z Setting up libcurl3-gnutls:amd64 (7.47.0-1ubuntu2.13) ...
2019-07-16T00:39:17.7268347Z Setting up libedit2:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-16T00:39:17.7610785Z Setting up libpipeline1:amd64 (1.4.1-2) ...
2019-07-16T00:39:17.8022456Z Setting up binfmt-support (2.1.6-1) ...
2019-07-16T00:39:17.8592092Z mount: permission denied
2019-07-16T00:39:17.8597191Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-16T00:39:17.8612763Z mount: permission denied
2019-07-16T00:39:17.8617561Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-16T00:39:18.0229253Z invoke-rc.d: could not determine current runlevel
2019-07-16T00:39:18.0263037Z invoke-rc.d: policy-rc.d denied execution of start.
2019-07-16T00:39:18.0751900Z Setting up libisl15:amd64 (0.16.1-1) ...
2019-07-16T00:39:18.1160153Z Setting up cpp-5 (5.4.0-6ubuntu1~16.04.11) ...
2019-07-16T00:39:18.1538468Z Setting up cpp (4:5.3.1-1ubuntu1) ...
2019-07-16T00:39:18.1975350Z Setting up curl (7.47.0-1ubuntu2.13) ...
---
2019-07-16T00:39:19.7762657Z Setting up libedit-dev:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-16T00:39:19.8136394Z Setting up libllvm6.0:amd64 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-16T00:39:19.8530301Z Setting up zlib1g-dev:amd64 (1:1.2.8.dfsg-2ubuntu4.1) ...
2019-07-16T00:39:19.8903858Z Setting up libssl-dev:amd64 (1.0.2g-1ubuntu4.15) ...
2019-07-16T00:39:19.9286443Z Setting up llvm-6.0-runtime (1:6.0-1ubuntu2~16.04.1) ...
2019-07-16T00:39:19.9537053Z mount: permission denied
2019-07-16T00:39:19.9540491Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-16T00:39:19.9738591Z Setting up llvm-6.0 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-16T00:39:20.0120165Z Setting up libffi-dev:amd64 (3.2.1-4) ...
2019-07-16T00:39:20.0442433Z Setting up llvm-6.0-dev (1:6.0-1ubuntu2~16.04.1) ...
2019-07-16T00:39:20.0787486Z Setting up llvm-6.0-tools (1:6.0-1ubuntu2~16.04.1) ...
2019-07-16T00:39:20.1144260Z Setting up pkg-config (0.29.1-0ubuntu1) ...
2019-07-16T00:39:20.2372644Z Processing triggers for ca-certificates (20170717~16.04.2) ...
2019-07-16T00:39:20.2520784Z Updating certificates in /etc/ssl/certs...
2019-07-16T00:39:21.8199470Z 148 added, 0 removed; done.
2019-07-16T00:39:21.8204245Z Running hooks in /etc/ca-certificates/update.d...
---
2019-07-16T00:39:54.4129823Z  ---> 1d0d1b6229dd
2019-07-16T00:39:54.4130297Z Successfully built 1d0d1b6229dd
2019-07-16T00:39:54.4377068Z Successfully tagged rust-ci:latest
2019-07-16T00:39:54.5822289Z Built container sha256:1d0d1b6229dd38228fd97d35add3f16abea3d6e2b80a7f5d4396bfd77dfaabba
2019-07-16T00:39:54.5836895Z Uploading finished image to https://.s3.amazonaws.com/docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624
2019-07-16T00:40:56.2484894Z upload failed: - to s3:///docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624 Parameter validation failed:
2019-07-16T00:40:56.2489777Z Invalid bucket name "": Bucket name must match the regex "^[a-zA-Z0-9.\-_]{1,255}$"
2019-07-16T00:40:57.4134364Z [CI_JOB_NAME=x86_64-gnu-llvm-6.0]
2019-07-16T00:40:57.4183045Z Starting sccache server...
2019-07-16T00:40:57.4661254Z configure: processing command line
2019-07-16T00:40:57.4662018Z configure: 
---
2019-07-16T00:44:27.1711913Z    Compiling serde_json v1.0.33
2019-07-16T00:44:31.2956715Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-07-16T00:44:39.8473059Z     Finished release [optimized] target(s) in 1m 30s
2019-07-16T00:44:39.8534815Z tidy check
2019-07-16T00:44:40.0248010Z tidy error: /checkout/src/libsyntax/print/pprust.rs: too many lines (3021) (add `// ignore-tidy-filelength` to the file to suppress this error)
2019-07-16T00:44:41.6192606Z some tidy checks failed
2019-07-16T00:44:41.6193573Z 
2019-07-16T00:44:41.6193573Z 
2019-07-16T00:44:41.6195463Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-07-16T00:44:41.6195803Z 
2019-07-16T00:44:41.6195917Z 
2019-07-16T00:44:41.6205093Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-07-16T00:44:41.6205423Z Build completed unsuccessfully in 0:01:33
2019-07-16T00:44:41.6205423Z Build completed unsuccessfully in 0:01:33
2019-07-16T00:44:42.9263405Z ##[error]Bash exited with code '1'.
2019-07-16T00:44:42.9295184Z ##[section]Starting: Checkout
2019-07-16T00:44:42.9296612Z ==============================================================================
2019-07-16T00:44:42.9296676Z Task         : Get sources
2019-07-16T00:44:42.9296716Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
