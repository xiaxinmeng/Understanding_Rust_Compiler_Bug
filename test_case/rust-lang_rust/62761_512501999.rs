plain
2019-07-17T17:33:47.3439818Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-17T17:33:47.3671502Z ##[command]git config gc.auto 0
2019-07-17T17:33:47.3716600Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-17T17:33:47.3767840Z ##[command]git config --get-all http.proxy
2019-07-17T17:33:47.3886785Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62761/merge:refs/remotes/pull/62761/merge
---
2019-07-17T17:34:22.8562289Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-17T17:34:22.8562505Z 
2019-07-17T17:34:22.8562836Z   git checkout -b <new-branch-name>
2019-07-17T17:34:22.8563022Z 
2019-07-17T17:34:22.8563188Z HEAD is now at ecd576909 Merge 637244bbf933c5f2a2f59bf6fae45db07d8d8b7f into e34d4ae8692f93155cbf1135ebea4c3fd37abf4e
2019-07-17T17:34:22.8693320Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-17T17:34:22.8695573Z ==============================================================================
2019-07-17T17:34:22.8695621Z Task         : Bash
2019-07-17T17:34:22.8695657Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-17T17:36:26.7106813Z Attempting with retry: curl -y 30 -Y 10 --connect-timeout 30 -f -L -C - -o /tmp/rustci_docker_cache https://.s3.amazonaws.com/docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624
2019-07-17T17:36:26.7163697Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-17T17:36:26.7164142Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-17T17:36:26.7164282Z 
2019-07-17T17:36:26.7165361Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-17T17:36:27.7250730Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-17T17:36:27.7251283Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-17T17:36:27.7251597Z 
2019-07-17T17:36:27.7251597Z 
2019-07-17T17:36:27.7280275Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-17T17:36:29.7339686Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-17T17:36:29.7350671Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-17T17:36:29.7350862Z 
2019-07-17T17:36:29.7350862Z 
2019-07-17T17:36:29.7383229Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-17T17:36:32.7438824Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-17T17:36:32.7448256Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-17T17:36:32.7448884Z 
2019-07-17T17:36:32.7448884Z 
2019-07-17T17:36:32.7482865Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-17T17:36:36.7555088Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-17T17:36:36.7555158Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-17T17:36:36.7555264Z 
2019-07-17T17:36:36.7555264Z 
2019-07-17T17:36:36.7589175Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-17T17:36:36.7593033Z The command has failed after 5 attempts.
2019-07-17T17:36:37.3483668Z open /tmp/rustci_docker_cache: no such file or directory
2019-07-17T17:36:37.3487019Z Attempting with retry: docker build --rm -t rust-ci -f /home/vsts/work/1/s/src/ci/docker/x86_64-gnu-llvm-6.0/Dockerfile /home/vsts/work/1/s/src/ci/docker
2019-07-17T17:36:37.3487388Z Sending build context to Docker daemon  521.2kB
2019-07-17T17:36:37.3487562Z 
2019-07-17T17:36:37.3487735Z Step 1/8 : FROM ubuntu:16.04
---
2019-07-17T17:36:53.6548618Z Reading package lists...
2019-07-17T17:36:54.5047923Z Reading package lists...
2019-07-17T17:36:54.6509014Z Building dependency tree...
2019-07-17T17:36:54.6509119Z Reading state information...
2019-07-17T17:36:54.7599801Z The following additional packages will be installed:
2019-07-17T17:36:54.7602941Z   binfmt-support binutils bzip2 cmake-data cpp cpp-5 dpkg-dev g++-5 gcc gcc-5
2019-07-17T17:36:54.7603297Z   git-man libarchive13 libasan2 libasn1-8-heimdal libatomic1
2019-07-17T17:36:54.7603700Z   libbabeltrace-ctf1 libbabeltrace1 libbsd-dev libbsd0 libbz2-1.0 libc-dev-bin
2019-07-17T17:36:54.7606964Z   libedit2 liberror-perl libexpat1 libffi-dev libffi6 libgcc-5-dev libgdbm3
2019-07-17T17:36:54.7607255Z   libglib2.0-0 libgmp10 libgnutls30 libgomp1 libgssapi-krb5-2
2019-07-17T17:36:54.7607463Z   libgssapi3-heimdal libhcrypto4-heimdal libheimbase1-heimdal
2019-07-17T17:36:54.7607660Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-17T17:36:54.7607660Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-17T17:36:54.7607901Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-17T17:36:54.7608087Z   libkrb5-3 libkrb5support0 libldap-2.4-2 libllvm6.0 liblsan0 liblzo2-2
2019-07-17T17:36:54.7608269Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-17T17:36:54.7608490Z   libperl5.22 libpipeline1 libpython-stdlib libpython2.7-minimal
2019-07-17T17:36:54.7608675Z   libpython2.7-stdlib libpython3.5 libpython3.5-minimal libpython3.5-stdlib
2019-07-17T17:36:54.7608859Z   libquadmath0 libroken18-heimdal librtmp1 libsasl2-2 libsasl2-modules-db
2019-07-17T17:36:54.7609096Z   libsqlite3-0 libssl1.0.0 libstdc++-5-dev libtasn1-6 libtinfo-dev libtsan0
2019-07-17T17:36:54.7609280Z   libubsan0 libwind0-heimdal libxml2 linux-libc-dev llvm-6.0 llvm-6.0-dev
2019-07-17T17:36:54.7609745Z   llvm-6.0-runtime mime-support openssl patch perl perl-modules-5.22 python
2019-07-17T17:36:54.7609970Z   python-minimal python2.7-minimal
2019-07-17T17:36:54.7610012Z Suggested packages:
2019-07-17T17:36:54.7610197Z   binutils-doc bzip2-doc codeblocks eclipse ninja-build cpp-doc gcc-5-locales
2019-07-17T17:36:54.7610426Z   debian-keyring g++-multilib g++-5-multilib gcc-5-doc libstdc++6-5-dbg
2019-07-17T17:36:54.7610608Z   gcc-multilib manpages-dev autoconf automake libtool flex bison gcc-doc
2019-07-17T17:36:54.7611016Z   libasan2-dbg liblsan0-dbg libtsan0-dbg libubsan0-dbg libcilkrts5-dbg
2019-07-17T17:36:54.7611195Z   libmpx0-dbg libquadmath0-dbg gdb-doc gettext-base git-daemon-run
2019-07-17T17:36:54.7611195Z   libmpx0-dbg libquadmath0-dbg gdb-doc gettext-base git-daemon-run
2019-07-17T17:36:54.7611376Z   | git-daemon-sysvinit git-doc git-el git-email git-gui gitk gitweb git-arch
2019-07-17T17:36:54.7611604Z   git-cvs git-mediawiki git-svn lrzip glibc-doc gnutls-bin krb5-doc krb5-user
2019-07-17T17:36:54.7611792Z   libstdc++-5-doc llvm-6.0-doc make-doc ed diffutils-doc perl-doc
2019-07-17T17:36:54.7611980Z   libterm-readline-gnu-perl | libterm-readline-perl-perl python-doc python-tk
2019-07-17T17:36:54.7612177Z   python2.7-doc
2019-07-17T17:36:54.7612215Z Recommended packages:
2019-07-17T17:36:54.7612393Z   build-essential fakeroot libalgorithm-merge-perl libc-dbg gdbserver less
2019-07-17T17:36:54.7613216Z   rsync ssh-client manpages manpages-dev libfile-fcntllock-perl
2019-07-17T17:36:54.7613482Z   libglib2.0-data shared-mime-info xdg-user-dirs krb5-locales libsasl2-modules
2019-07-17T17:36:54.7613704Z   libssl-doc xml-core netbase rename
2019-07-17T17:36:54.7613805Z The following NEW packages will be installed:
2019-07-17T17:36:54.7614097Z   binfmt-support binutils bzip2 ca-certificates cmake cmake-data cpp cpp-5
2019-07-17T17:36:54.7614380Z   curl dpkg-dev file g++ g++-5 gcc gcc-5 gdb git git-man libarchive13 libasan2
2019-07-17T17:36:54.7614702Z   libasn1-8-heimdal libatomic1 libbabeltrace-ctf1 libbabeltrace1 libbsd-dev
2019-07-17T17:36:54.7615121Z   libbsd0 libc-dev-bin libc6-dev libcc1-0 libcilkrts5 libcurl3 libcurl3-gnutls
2019-07-17T17:36:54.7615414Z   libdpkg-perl libedit-dev libedit2 liberror-perl libexpat1 libffi-dev libffi6
2019-07-17T17:36:54.7615711Z   libgcc-5-dev libgdbm3 libglib2.0-0 libgmp10 libgnutls30 libgomp1
2019-07-17T17:36:54.7616332Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-17T17:36:54.7616552Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-17T17:36:54.7616744Z   libkrb5-3 libkrb5support0 libldap-2.4-2 libllvm6.0 liblsan0 liblzo2-2
2019-07-17T17:36:54.7616924Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-17T17:36:54.7616924Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-17T17:36:54.7617139Z   libperl5.22 libpipeline1 libpython-stdlib libpython2.7-minimal
2019-07-17T17:36:54.7617334Z   libpython2.7-stdlib libpython3.5 libpython3.5-minimal libpython3.5-stdlib
2019-07-17T17:36:54.7617534Z   libquadmath0 libroken18-heimdal librtmp1 libsasl2-2 libsasl2-modules-db
2019-07-17T17:36:54.7617760Z   libsqlite3-0 libssl-dev libssl1.0.0 libstdc++-5-dev libtasn1-6 libtinfo-dev
2019-07-17T17:36:54.7617948Z   libtsan0 libubsan0 libwind0-heimdal libxml2 linux-libc-dev llvm-6.0
2019-07-17T17:36:54.7618133Z   llvm-6.0-dev llvm-6.0-runtime llvm-6.0-tools make mime-support openssl patch
2019-07-17T17:36:54.7618346Z   perl perl-modules-5.22 pkg-config python python-minimal python2.7
2019-07-17T17:36:54.7618527Z   python2.7-minimal sudo xz-utils zlib1g-dev
2019-07-17T17:36:54.7618611Z The following packages will be upgraded:
2019-07-17T17:36:55.1282707Z 1 upgraded, 115 newly installed, 0 to remove and 5 not upgraded.
2019-07-17T17:36:55.1283377Z Need to get 121 MB of archives.
2019-07-17T17:36:55.1283498Z After this operation, 592 MB of additional disk space will be used.
2019-07-17T17:36:55.1284624Z Get:1 http://archive.ubuntu.com/ubuntu xenial/main amd64 libgdbm3 amd64 1.8.3-13.1 [16.9 kB]
---
2019-07-17T17:36:57.2488009Z Get:61 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 sudo amd64 1.8.16-0ubuntu1.7 [390 kB]
2019-07-17T17:36:57.3055578Z Get:62 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 openssl amd64 1.0.2g-1ubuntu4.15 [492 kB]
2019-07-17T17:36:57.3119542Z Get:63 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 ca-certificates all 20170717~16.04.2 [167 kB]
2019-07-17T17:36:57.3147135Z Get:64 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 libcurl3-gnutls amd64 7.47.0-1ubuntu2.13 [184 kB]
2019-07-17T17:36:57.3173662Z Get:65 http://archive.ubuntu.com/ubuntu xenial/main amd64 libedit2 amd64 3.1-20150325-1ubuntu2 [76.5 kB]
2019-07-17T17:36:57.3191421Z Get:66 http://archive.ubuntu.com/ubuntu xenial/main amd64 libpipeline1 amd64 1.4.1-2 [24.6 kB]
2019-07-17T17:36:57.3191719Z Get:67 http://archive.ubuntu.com/ubuntu xenial/main amd64 binfmt-support amd64 2.1.6-1 [50.7 kB]
2019-07-17T17:36:57.4303669Z Get:69 http://archive.ubuntu.com/ubuntu xenial/main amd64 libisl15 amd64 0.16.1-1 [524 kB]
2019-07-17T17:36:57.4361422Z Get:70 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 cpp-5 amd64 5.4.0-6ubuntu1~16.04.11 [7660 kB]
2019-07-17T17:36:57.7025907Z Get:71 http://archive.ubuntu.com/ubuntu xenial/main amd64 cpp amd64 4:5.3.1-1ubuntu1 [27.7 kB]
2019-07-17T17:36:57.7026570Z Get:72 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 curl amd64 7.47.0-1ubuntu2.13 [139 kB]
---
2019-07-17T17:37:17.5450512Z Unpacking ca-certificates (20170717~16.04.2) ...
2019-07-17T17:37:17.7343039Z Selecting previously unselected package libcurl3-gnutls:amd64.
2019-07-17T17:37:17.7354641Z Preparing to unpack .../libcurl3-gnutls_7.47.0-1ubuntu2.13_amd64.deb ...
2019-07-17T17:37:17.7508578Z Unpacking libcurl3-gnutls:amd64 (7.47.0-1ubuntu2.13) ...
2019-07-17T17:37:17.8961213Z Selecting previously unselected package libedit2:amd64.
2019-07-17T17:37:17.8974072Z Preparing to unpack .../libedit2_3.1-20150325-1ubuntu2_amd64.deb ...
2019-07-17T17:37:17.9151128Z Unpacking libedit2:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-17T17:37:18.0401993Z Selecting previously unselected package libpipeline1:amd64.
2019-07-17T17:37:18.0414212Z Preparing to unpack .../libpipeline1_1.4.1-2_amd64.deb ...
2019-07-17T17:37:18.0562968Z Unpacking libpipeline1:amd64 (1.4.1-2) ...
2019-07-17T17:37:18.1816098Z Selecting previously unselected package binfmt-support.
2019-07-17T17:37:18.1829294Z Preparing to unpack .../binfmt-support_2.1.6-1_amd64.deb ...
2019-07-17T17:37:18.1975543Z Unpacking binfmt-support (2.1.6-1) ...
2019-07-17T17:37:18.3375667Z Preparing to unpack .../binutils_2.26.1-1ubuntu1~16.04.8_amd64.deb ...
2019-07-17T17:37:18.3528627Z Unpacking binutils (2.26.1-1ubuntu1~16.04.8) ...
2019-07-17T17:37:18.8313490Z Selecting previously unselected package libisl15:amd64.
2019-07-17T17:37:18.8329680Z Preparing to unpack .../libisl15_0.16.1-1_amd64.deb ...
---
2019-07-17T17:37:33.7071795Z Unpacking zlib1g-dev:amd64 (1:1.2.8.dfsg-2ubuntu4.1) ...
2019-07-17T17:37:33.8185742Z Selecting previously unselected package libssl-dev:amd64.
2019-07-17T17:37:33.8198880Z Preparing to unpack .../libssl-dev_1.0.2g-1ubuntu4.15_amd64.deb ...
2019-07-17T17:37:33.8348308Z Unpacking libssl-dev:amd64 (1.0.2g-1ubuntu4.15) ...
2019-07-17T17:37:34.1577870Z Selecting previously unselected package llvm-6.0-runtime.
2019-07-17T17:37:34.1594558Z Preparing to unpack .../llvm-6.0-runtime_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-17T17:37:34.1738000Z Unpacking llvm-6.0-runtime (1:6.0-1ubuntu2~16.04.1) ...
2019-07-17T17:37:34.3081807Z Selecting previously unselected package llvm-6.0.
2019-07-17T17:37:34.3098876Z Preparing to unpack .../llvm-6.0_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-17T17:37:34.3242023Z Unpacking llvm-6.0 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-17T17:37:34.9409765Z Selecting previously unselected package libffi-dev:amd64.
2019-07-17T17:37:34.9426828Z Preparing to unpack .../libffi-dev_3.2.1-4_amd64.deb ...
2019-07-17T17:37:34.9591335Z Unpacking libffi-dev:amd64 (3.2.1-4) ...
2019-07-17T17:37:35.0938597Z Selecting previously unselected package llvm-6.0-dev.
2019-07-17T17:37:35.0954749Z Preparing to unpack .../llvm-6.0-dev_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-17T17:37:35.1104722Z Unpacking llvm-6.0-dev (1:6.0-1ubuntu2~16.04.1) ...
2019-07-17T17:37:39.5159443Z Selecting previously unselected package llvm-6.0-tools.
2019-07-17T17:37:39.5180825Z Preparing to unpack .../llvm-6.0-tools_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-17T17:37:39.5324187Z Unpacking llvm-6.0-tools (1:6.0-1ubuntu2~16.04.1) ...
2019-07-17T17:37:39.7134001Z Selecting previously unselected package pkg-config.
2019-07-17T17:37:39.7149953Z Preparing to unpack .../pkg-config_0.29.1-0ubuntu1_amd64.deb ...
2019-07-17T17:37:39.7928683Z Processing triggers for libc-bin (2.23-0ubuntu11) ...
2019-07-17T17:37:39.8533674Z Processing triggers for systemd (229-4ubuntu21.21) ...
2019-07-17T17:37:40.3022334Z Setting up libgdbm3:amd64 (1.8.3-13.1) ...
2019-07-17T17:37:40.3835899Z Setting up libffi6:amd64 (3.2.1-4) ...
---
2019-07-17T17:37:44.3251779Z debconf: unable to initialize frontend: Dialog
2019-07-17T17:37:44.3251940Z debconf: (TERM is not set, so the dialog frontend is not usable.)
2019-07-17T17:37:44.3252797Z debconf: falling back to frontend: Readline
2019-07-17T17:37:44.8233296Z Setting up libcurl3-gnutls:amd64 (7.47.0-1ubuntu2.13) ...
2019-07-17T17:37:44.8677233Z Setting up libedit2:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-17T17:37:44.9105773Z Setting up libpipeline1:amd64 (1.4.1-2) ...
2019-07-17T17:37:44.9565001Z Setting up binfmt-support (2.1.6-1) ...
2019-07-17T17:37:45.0802196Z mount: permission denied
2019-07-17T17:37:45.0804791Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-17T17:37:45.0816620Z mount: permission denied
2019-07-17T17:37:45.0820580Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-17T17:37:45.2236682Z invoke-rc.d: could not determine current runlevel
2019-07-17T17:37:45.2263135Z invoke-rc.d: policy-rc.d denied execution of start.
2019-07-17T17:37:45.3003143Z Setting up libisl15:amd64 (0.16.1-1) ...
2019-07-17T17:37:45.3450621Z Setting up cpp-5 (5.4.0-6ubuntu1~16.04.11) ...
2019-07-17T17:37:45.3917868Z Setting up cpp (4:5.3.1-1ubuntu1) ...
2019-07-17T17:37:45.4494242Z Setting up curl (7.47.0-1ubuntu2.13) ...
---
2019-07-17T17:37:47.3664145Z Setting up libedit-dev:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-17T17:37:47.4119448Z Setting up libllvm6.0:amd64 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-17T17:37:47.4558231Z Setting up zlib1g-dev:amd64 (1:1.2.8.dfsg-2ubuntu4.1) ...
2019-07-17T17:37:47.5156955Z Setting up libssl-dev:amd64 (1.0.2g-1ubuntu4.15) ...
2019-07-17T17:37:47.5611755Z Setting up llvm-6.0-runtime (1:6.0-1ubuntu2~16.04.1) ...
2019-07-17T17:37:47.5956004Z mount: permission denied
2019-07-17T17:37:47.5961446Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-17T17:37:47.6211472Z Setting up llvm-6.0 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-17T17:37:47.6685139Z Setting up libffi-dev:amd64 (3.2.1-4) ...
2019-07-17T17:37:47.7136026Z Setting up llvm-6.0-dev (1:6.0-1ubuntu2~16.04.1) ...
2019-07-17T17:37:47.7715901Z Setting up llvm-6.0-tools (1:6.0-1ubuntu2~16.04.1) ...
2019-07-17T17:37:47.8175327Z Setting up pkg-config (0.29.1-0ubuntu1) ...
2019-07-17T17:37:48.8731582Z Processing triggers for ca-certificates (20170717~16.04.2) ...
2019-07-17T17:37:48.8731708Z Updating certificates in /etc/ssl/certs...
2019-07-17T17:37:49.4406842Z 148 added, 0 removed; done.
2019-07-17T17:37:49.4407572Z Running hooks in /etc/ca-certificates/update.d...
---
2019-07-17T17:38:22.1004422Z Removing intermediate container aab866b94e54
2019-07-17T17:38:22.1006103Z  ---> 8ec24c32d55b
2019-07-17T17:38:22.1076823Z Successfully built 8ec24c32d55b
2019-07-17T17:38:22.3149659Z Successfully tagged rust-ci:latest
2019-07-17T17:38:22.4413739Z Built container sha256:8ec24c32d55b4d05f85c9b1487b45ce00a780ccf3dbd56e9de2b5d96628fe611
2019-07-17T17:38:22.4430257Z Uploading finished image to https://.s3.amazonaws.com/docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624
2019-07-17T17:39:20.5841275Z upload failed: - to s3:///docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624 Parameter validation failed:
2019-07-17T17:39:20.5842111Z Invalid bucket name "": Bucket name must match the regex "^[a-zA-Z0-9.\-_]{1,255}$"
2019-07-17T17:39:21.7372973Z [CI_JOB_NAME=x86_64-gnu-llvm-6.0]
2019-07-17T17:39:21.7418542Z Starting sccache server...
2019-07-17T17:39:21.7878097Z configure: processing command line
2019-07-17T17:39:21.7879080Z configure: 
---
2019-07-17T17:42:31.0493476Z    Compiling serde_json v1.0.33
2019-07-17T17:42:34.7754091Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-07-17T17:42:42.3474393Z     Finished release [optimized] target(s) in 1m 20s
2019-07-17T17:42:42.3533006Z tidy check
2019-07-17T17:42:42.7775461Z tidy error: /checkout/src/test/ui/generator/issue-62506-two_awaits.rs:2: line longer than 100 chars
2019-07-17T17:42:43.9020723Z some tidy checks failed
2019-07-17T17:42:43.9020852Z 
2019-07-17T17:42:43.9020852Z 
2019-07-17T17:42:43.9022179Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-07-17T17:42:43.9022339Z 
2019-07-17T17:42:43.9022367Z 
2019-07-17T17:42:43.9032225Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-07-17T17:42:43.9032339Z Build completed unsuccessfully in 0:01:22
2019-07-17T17:42:43.9032339Z Build completed unsuccessfully in 0:01:22
2019-07-17T17:42:45.1934580Z ##[error]Bash exited with code '1'.
2019-07-17T17:42:45.1972006Z ##[section]Starting: Checkout
2019-07-17T17:42:45.1973684Z ==============================================================================
2019-07-17T17:42:45.1973744Z Task         : Get sources
2019-07-17T17:42:45.1973793Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
