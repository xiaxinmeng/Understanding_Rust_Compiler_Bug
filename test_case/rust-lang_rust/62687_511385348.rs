plain
2019-07-15T12:06:21.9394340Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-15T12:06:21.9583193Z ##[command]git config gc.auto 0
2019-07-15T12:06:22.8914771Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-15T12:06:22.8921426Z ##[command]git config --get-all http.proxy
2019-07-15T12:06:22.8927701Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62687/merge:refs/remotes/pull/62687/merge
---
2019-07-15T12:06:56.7157988Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-15T12:06:56.7158019Z 
2019-07-15T12:06:56.7158227Z   git checkout -b <new-branch-name>
2019-07-15T12:06:56.7158256Z 
2019-07-15T12:06:56.7158324Z HEAD is now at 03b329273 Merge 677cf3de3ac37c5fb2ca947a34f7ffda0748e7e2 into 9bb855cda0c5ae97faf5dbf1cd4935dd37fad066
2019-07-15T12:06:56.7307555Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-15T12:06:56.7310326Z ==============================================================================
2019-07-15T12:06:56.7310387Z Task         : Bash
2019-07-15T12:06:56.7310471Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-15T12:08:33.4847293Z Attempting with retry: curl -y 30 -Y 10 --connect-timeout 30 -f -L -C - -o /tmp/rustci_docker_cache https://.s3.amazonaws.com/docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624
2019-07-15T12:08:33.4922068Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-15T12:08:33.4922373Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-15T12:08:33.4922549Z 
2019-07-15T12:08:33.4923458Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-15T12:08:34.5010928Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-15T12:08:34.5013374Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-15T12:08:34.5013699Z 
2019-07-15T12:08:34.5013699Z 
2019-07-15T12:08:34.5017097Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-15T12:08:36.5104158Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-15T12:08:36.5104853Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-15T12:08:36.5105892Z 
2019-07-15T12:08:36.5105892Z 
2019-07-15T12:08:36.5147690Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-15T12:08:39.5223146Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-15T12:08:39.5223626Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-15T12:08:39.5223786Z 
2019-07-15T12:08:39.5223786Z 
2019-07-15T12:08:39.5267662Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-15T12:08:43.5349890Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-15T12:08:43.5364222Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-15T12:08:43.5364602Z 
2019-07-15T12:08:43.5364602Z 
2019-07-15T12:08:43.5392653Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-15T12:08:43.5398684Z The command has failed after 5 attempts.
2019-07-15T12:08:43.5829338Z open /tmp/rustci_docker_cache: no such file or directory
2019-07-15T12:08:43.5856902Z Attempting with retry: docker build --rm -t rust-ci -f /home/vsts/work/1/s/src/ci/docker/x86_64-gnu-llvm-6.0/Dockerfile /home/vsts/work/1/s/src/ci/docker
2019-07-15T12:08:44.2094883Z Sending build context to Docker daemon  521.7kB
2019-07-15T12:08:44.2095021Z 
2019-07-15T12:08:44.2266392Z Step 1/8 : FROM ubuntu:16.04
---
2019-07-15T12:09:00.3938107Z Reading package lists...
2019-07-15T12:09:01.3864863Z Reading package lists...
2019-07-15T12:09:01.5818795Z Building dependency tree...
2019-07-15T12:09:01.5819527Z Reading state information...
2019-07-15T12:09:01.7012249Z The following additional packages will be installed:
2019-07-15T12:09:01.7014034Z   binfmt-support binutils bzip2 cmake-data cpp cpp-5 dpkg-dev g++-5 gcc gcc-5
2019-07-15T12:09:01.7015060Z   git-man libarchive13 libasan2 libasn1-8-heimdal libatomic1
2019-07-15T12:09:01.7037455Z   libbabeltrace-ctf1 libbabeltrace1 libbsd-dev libbsd0 libbz2-1.0 libc-dev-bin
2019-07-15T12:09:01.7039082Z   libedit2 liberror-perl libexpat1 libffi-dev libffi6 libgcc-5-dev libgdbm3
2019-07-15T12:09:01.7039499Z   libglib2.0-0 libgmp10 libgnutls30 libgomp1 libgssapi-krb5-2
2019-07-15T12:09:01.7039929Z   libgssapi3-heimdal libhcrypto4-heimdal libheimbase1-heimdal
2019-07-15T12:09:01.7040243Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-15T12:09:01.7040243Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-15T12:09:01.7040539Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-15T12:09:01.7040771Z   libkrb5-3 libkrb5support0 libldap-2.4-2 libllvm6.0 liblsan0 liblzo2-2
2019-07-15T12:09:01.7041360Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-15T12:09:01.7041640Z   libperl5.22 libpipeline1 libpython-stdlib libpython2.7-minimal
2019-07-15T12:09:01.7041870Z   libpython2.7-stdlib libpython3.5 libpython3.5-minimal libpython3.5-stdlib
2019-07-15T12:09:01.7042104Z   libquadmath0 libroken18-heimdal librtmp1 libsasl2-2 libsasl2-modules-db
2019-07-15T12:09:01.7042389Z   libsqlite3-0 libssl1.0.0 libstdc++-5-dev libtasn1-6 libtinfo-dev libtsan0
2019-07-15T12:09:01.7042618Z   libubsan0 libwind0-heimdal libxml2 linux-libc-dev llvm-6.0 llvm-6.0-dev
2019-07-15T12:09:01.7042846Z   llvm-6.0-runtime mime-support openssl patch perl perl-modules-5.22 python
2019-07-15T12:09:01.7043106Z   python-minimal python2.7-minimal
2019-07-15T12:09:01.7043200Z Suggested packages:
2019-07-15T12:09:01.7043440Z   binutils-doc bzip2-doc codeblocks eclipse ninja-build cpp-doc gcc-5-locales
2019-07-15T12:09:01.7043732Z   debian-keyring g++-multilib g++-5-multilib gcc-5-doc libstdc++6-5-dbg
2019-07-15T12:09:01.7043973Z   gcc-multilib manpages-dev autoconf automake libtool flex bison gcc-doc
2019-07-15T12:09:01.7044471Z   libasan2-dbg liblsan0-dbg libtsan0-dbg libubsan0-dbg libcilkrts5-dbg
2019-07-15T12:09:01.7044691Z   libmpx0-dbg libquadmath0-dbg gdb-doc gettext-base git-daemon-run
2019-07-15T12:09:01.7044691Z   libmpx0-dbg libquadmath0-dbg gdb-doc gettext-base git-daemon-run
2019-07-15T12:09:01.7044959Z   | git-daemon-sysvinit git-doc git-el git-email git-gui gitk gitweb git-arch
2019-07-15T12:09:01.7045193Z   git-cvs git-mediawiki git-svn lrzip glibc-doc gnutls-bin krb5-doc krb5-user
2019-07-15T12:09:01.7045412Z   libstdc++-5-doc llvm-6.0-doc make-doc ed diffutils-doc perl-doc
2019-07-15T12:09:01.7045688Z   libterm-readline-gnu-perl | libterm-readline-perl-perl python-doc python-tk
2019-07-15T12:09:01.7045873Z   python2.7-doc
2019-07-15T12:09:01.7045916Z Recommended packages:
2019-07-15T12:09:01.7046136Z   build-essential fakeroot libalgorithm-merge-perl libc-dbg gdbserver less
2019-07-15T12:09:01.7046413Z   rsync ssh-client manpages manpages-dev libfile-fcntllock-perl
2019-07-15T12:09:01.7046644Z   libglib2.0-data shared-mime-info xdg-user-dirs krb5-locales libsasl2-modules
2019-07-15T12:09:01.7047282Z   libssl-doc xml-core netbase rename
2019-07-15T12:09:01.7047401Z The following NEW packages will be installed:
2019-07-15T12:09:01.7047713Z   binfmt-support binutils bzip2 ca-certificates cmake cmake-data cpp cpp-5
2019-07-15T12:09:01.7048001Z   curl dpkg-dev file g++ g++-5 gcc gcc-5 gdb git git-man libarchive13 libasan2
2019-07-15T12:09:01.7048310Z   libasn1-8-heimdal libatomic1 libbabeltrace-ctf1 libbabeltrace1 libbsd-dev
2019-07-15T12:09:01.7048723Z   libbsd0 libc-dev-bin libc6-dev libcc1-0 libcilkrts5 libcurl3 libcurl3-gnutls
2019-07-15T12:09:01.7049027Z   libdpkg-perl libedit-dev libedit2 liberror-perl libexpat1 libffi-dev libffi6
2019-07-15T12:09:01.7049334Z   libgcc-5-dev libgdbm3 libglib2.0-0 libgmp10 libgnutls30 libgomp1
2019-07-15T12:09:01.7049855Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-15T12:09:01.7050229Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-15T12:09:01.7050642Z   libkrb5-3 libkrb5support0 libldap-2.4-2 libllvm6.0 liblsan0 liblzo2-2
2019-07-15T12:09:01.7050874Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-15T12:09:01.7050874Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-15T12:09:01.7051328Z   libperl5.22 libpipeline1 libpython-stdlib libpython2.7-minimal
2019-07-15T12:09:01.7051774Z   libpython2.7-stdlib libpython3.5 libpython3.5-minimal libpython3.5-stdlib
2019-07-15T12:09:01.7052232Z   libquadmath0 libroken18-heimdal librtmp1 libsasl2-2 libsasl2-modules-db
2019-07-15T12:09:01.7052590Z   libsqlite3-0 libssl-dev libssl1.0.0 libstdc++-5-dev libtasn1-6 libtinfo-dev
2019-07-15T12:09:01.7052813Z   libtsan0 libubsan0 libwind0-heimdal libxml2 linux-libc-dev llvm-6.0
2019-07-15T12:09:01.7053042Z   llvm-6.0-dev llvm-6.0-runtime llvm-6.0-tools make mime-support openssl patch
2019-07-15T12:09:01.7053427Z   perl perl-modules-5.22 pkg-config python python-minimal python2.7
2019-07-15T12:09:01.7053632Z   python2.7-minimal sudo xz-utils zlib1g-dev
2019-07-15T12:09:01.7053678Z The following packages will be upgraded:
2019-07-15T12:09:02.4476194Z 1 upgraded, 115 newly installed, 0 to remove and 4 not upgraded.
2019-07-15T12:09:02.4477807Z Need to get 121 MB of archives.
2019-07-15T12:09:02.4477944Z After this operation, 592 MB of additional disk space will be used.
2019-07-15T12:09:02.4478851Z Get:1 http://archive.ubuntu.com/ubuntu xenial/main amd64 libgdbm3 amd64 1.8.3-13.1 [16.9 kB]
---
2019-07-15T12:09:03.1822785Z Get:61 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 sudo amd64 1.8.16-0ubuntu1.7 [390 kB]
2019-07-15T12:09:03.1892936Z Get:62 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 openssl amd64 1.0.2g-1ubuntu4.15 [492 kB]
2019-07-15T12:09:03.2067905Z Get:63 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 ca-certificates all 20170717~16.04.2 [167 kB]
2019-07-15T12:09:03.2068398Z Get:64 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 libcurl3-gnutls amd64 7.47.0-1ubuntu2.13 [184 kB]
2019-07-15T12:09:03.2068702Z Get:65 http://archive.ubuntu.com/ubuntu xenial/main amd64 libedit2 amd64 3.1-20150325-1ubuntu2 [76.5 kB]
2019-07-15T12:09:03.2069069Z Get:66 http://archive.ubuntu.com/ubuntu xenial/main amd64 libpipeline1 amd64 1.4.1-2 [24.6 kB]
2019-07-15T12:09:03.2120983Z Get:67 http://archive.ubuntu.com/ubuntu xenial/main amd64 binfmt-support amd64 2.1.6-1 [50.7 kB]
2019-07-15T12:09:03.2729806Z Get:69 http://archive.ubuntu.com/ubuntu xenial/main amd64 libisl15 amd64 0.16.1-1 [524 kB]
2019-07-15T12:09:03.2809975Z Get:70 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 cpp-5 amd64 5.4.0-6ubuntu1~16.04.11 [7660 kB]
2019-07-15T12:09:03.4106147Z Get:71 http://archive.ubuntu.com/ubuntu xenial/main amd64 cpp amd64 4:5.3.1-1ubuntu1 [27.7 kB]
2019-07-15T12:09:03.4115900Z Get:72 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 curl amd64 7.47.0-1ubuntu2.13 [139 kB]
---
2019-07-15T12:09:22.5834398Z Unpacking ca-certificates (20170717~16.04.2) ...
2019-07-15T12:09:22.7728533Z Selecting previously unselected package libcurl3-gnutls:amd64.
2019-07-15T12:09:22.7747676Z Preparing to unpack .../libcurl3-gnutls_7.47.0-1ubuntu2.13_amd64.deb ...
2019-07-15T12:09:22.7937984Z Unpacking libcurl3-gnutls:amd64 (7.47.0-1ubuntu2.13) ...
2019-07-15T12:09:22.9375937Z Selecting previously unselected package libedit2:amd64.
2019-07-15T12:09:22.9394634Z Preparing to unpack .../libedit2_3.1-20150325-1ubuntu2_amd64.deb ...
2019-07-15T12:09:22.9557656Z Unpacking libedit2:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-15T12:09:23.1062249Z Selecting previously unselected package libpipeline1:amd64.
2019-07-15T12:09:23.1081563Z Preparing to unpack .../libpipeline1_1.4.1-2_amd64.deb ...
2019-07-15T12:09:23.1223794Z Unpacking libpipeline1:amd64 (1.4.1-2) ...
2019-07-15T12:09:23.2746064Z Selecting previously unselected package binfmt-support.
2019-07-15T12:09:23.2764763Z Preparing to unpack .../binfmt-support_2.1.6-1_amd64.deb ...
2019-07-15T12:09:23.2903470Z Unpacking binfmt-support (2.1.6-1) ...
2019-07-15T12:09:23.4658373Z Preparing to unpack .../binutils_2.26.1-1ubuntu1~16.04.8_amd64.deb ...
2019-07-15T12:09:23.4812715Z Unpacking binutils (2.26.1-1ubuntu1~16.04.8) ...
2019-07-15T12:09:23.9866712Z Selecting previously unselected package libisl15:amd64.
2019-07-15T12:09:23.9883680Z Preparing to unpack .../libisl15_0.16.1-1_amd64.deb ...
---
2019-07-15T12:09:36.5154648Z Selecting previously unselected package libtinfo-dev:amd64.
2019-07-15T12:09:36.5177620Z Preparing to unpack .../libtinfo-dev_6.0+20160213-1ubuntu1_amd64.deb ...
2019-07-15T12:09:36.5334085Z Unpacking libtinfo-dev:amd64 (6.0+20160213-1ubuntu1) ...
2019-07-15T12:09:36.6429822Z Selecting previously unselected package libedit-dev:amd64.
2019-07-15T12:09:36.6455531Z Preparing to unpack .../libedit-dev_3.1-20150325-1ubuntu2_amd64.deb ...
2019-07-15T12:09:36.6611920Z Unpacking libedit-dev:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-15T12:09:36.7928967Z Selecting previously unselected package libllvm6.0:amd64.
2019-07-15T12:09:36.7946647Z Preparing to unpack .../libllvm6.0_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-15T12:09:36.8103856Z Unpacking libllvm6.0:amd64 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-15T12:09:39.8205005Z Selecting previously unselected package zlib1g-dev:amd64.
2019-07-15T12:09:39.8227242Z Preparing to unpack .../zlib1g-dev_1%3a1.2.8.dfsg-2ubuntu4.1_amd64.deb ...
2019-07-15T12:09:39.8368314Z Unpacking zlib1g-dev:amd64 (1:1.2.8.dfsg-2ubuntu4.1) ...
2019-07-15T12:09:39.9498631Z Preparing to unpack .../libssl-dev_1.0.2g-1ubuntu4.15_amd64.deb ...
2019-07-15T12:09:39.9661616Z Unpacking libssl-dev:amd64 (1.0.2g-1ubuntu4.15) ...
2019-07-15T12:09:40.2845201Z Selecting previously unselected package llvm-6.0-runtime.
2019-07-15T12:09:40.2845201Z Selecting previously unselected package llvm-6.0-runtime.
2019-07-15T12:09:40.2868209Z Preparing to unpack .../llvm-6.0-runtime_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-15T12:09:40.3013978Z Unpacking llvm-6.0-runtime (1:6.0-1ubuntu2~16.04.1) ...
2019-07-15T12:09:40.4393487Z Selecting previously unselected package llvm-6.0.
2019-07-15T12:09:40.4416459Z Preparing to unpack .../llvm-6.0_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-15T12:09:40.4574089Z Unpacking llvm-6.0 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-15T12:09:41.1245943Z Selecting previously unselected package libffi-dev:amd64.
2019-07-15T12:09:41.1269521Z Preparing to unpack .../libffi-dev_3.2.1-4_amd64.deb ...
2019-07-15T12:09:41.1423947Z Unpacking libffi-dev:amd64 (3.2.1-4) ...
2019-07-15T12:09:41.2829514Z Selecting previously unselected package llvm-6.0-dev.
2019-07-15T12:09:41.2845563Z Preparing to unpack .../llvm-6.0-dev_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-15T12:09:41.3032838Z Unpacking llvm-6.0-dev (1:6.0-1ubuntu2~16.04.1) ...
2019-07-15T12:09:46.0745785Z Selecting previously unselected package llvm-6.0-tools.
2019-07-15T12:09:46.0773358Z Preparing to unpack .../llvm-6.0-tools_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-15T12:09:46.0919572Z Unpacking llvm-6.0-tools (1:6.0-1ubuntu2~16.04.1) ...
2019-07-15T12:09:46.2699680Z Selecting previously unselected package pkg-config.
2019-07-15T12:09:46.2717625Z Preparing to unpack .../pkg-config_0.29.1-0ubuntu1_amd64.deb ...
2019-07-15T12:09:46.2872605Z Unpacking pkg-config (0.29.1-0ubuntu1) ...
2019-07-15T12:09:46.4208702Z Processing triggers for systemd (229-4ubuntu21.21) ...
2019-07-15T12:09:46.8175062Z Setting up libgdbm3:amd64 (1.8.3-13.1) ...
2019-07-15T12:09:46.8945924Z Setting up libffi6:amd64 (3.2.1-4) ...
2019-07-15T12:09:46.9383692Z Setting up libglib2.0-0:amd64 (2.48.2-0ubuntu4.3) ...
---
2019-07-15T12:09:50.9896868Z debconf: unable to initialize frontend: Dialog
2019-07-15T12:09:50.9896982Z debconf: (TERM is not set, so the dialog frontend is not usable.)
2019-07-15T12:09:50.9897082Z debconf: falling back to frontend: Readline
2019-07-15T12:09:51.5406062Z Setting up libcurl3-gnutls:amd64 (7.47.0-1ubuntu2.13) ...
2019-07-15T12:09:51.5849680Z Setting up libedit2:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-15T12:09:51.6336872Z Setting up libpipeline1:amd64 (1.4.1-2) ...
2019-07-15T12:09:51.7513788Z Setting up binfmt-support (2.1.6-1) ...
2019-07-15T12:09:51.8347744Z mount: permission denied
2019-07-15T12:09:51.8354304Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-15T12:09:51.8370216Z mount: permission denied
2019-07-15T12:09:51.8375321Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-15T12:09:52.0043394Z invoke-rc.d: could not determine current runlevel
2019-07-15T12:09:52.0076044Z invoke-rc.d: policy-rc.d denied execution of start.
2019-07-15T12:09:52.0704464Z Setting up libisl15:amd64 (0.16.1-1) ...
2019-07-15T12:09:52.1194794Z Setting up cpp-5 (5.4.0-6ubuntu1~16.04.11) ...
2019-07-15T12:09:52.1651792Z Setting up cpp (4:5.3.1-1ubuntu1) ...
2019-07-15T12:09:52.2264348Z Setting up curl (7.47.0-1ubuntu2.13) ...
---
2019-07-15T12:09:54.1316623Z Setting up libedit-dev:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-15T12:09:54.1804181Z Setting up libllvm6.0:amd64 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-15T12:09:54.2326588Z Setting up zlib1g-dev:amd64 (1:1.2.8.dfsg-2ubuntu4.1) ...
2019-07-15T12:09:54.2803465Z Setting up libssl-dev:amd64 (1.0.2g-1ubuntu4.15) ...
2019-07-15T12:09:54.3369125Z Setting up llvm-6.0-runtime (1:6.0-1ubuntu2~16.04.1) ...
2019-07-15T12:09:54.3703645Z mount: permission denied
2019-07-15T12:09:54.3704615Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-15T12:09:54.3914493Z Setting up llvm-6.0 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-15T12:09:54.4416883Z Setting up libffi-dev:amd64 (3.2.1-4) ...
2019-07-15T12:09:54.5018443Z Setting up llvm-6.0-dev (1:6.0-1ubuntu2~16.04.1) ...
2019-07-15T12:09:54.5467264Z Setting up llvm-6.0-tools (1:6.0-1ubuntu2~16.04.1) ...
2019-07-15T12:09:54.6044005Z Setting up pkg-config (0.29.1-0ubuntu1) ...
2019-07-15T12:09:54.7554546Z Processing triggers for ca-certificates (20170717~16.04.2) ...
2019-07-15T12:09:54.7758990Z Updating certificates in /etc/ssl/certs...
2019-07-15T12:09:56.4355557Z 148 added, 0 removed; done.
2019-07-15T12:09:56.4356346Z Running hooks in /etc/ca-certificates/update.d...
---
2019-07-15T12:10:30.5046535Z  ---> faf986337748
2019-07-15T12:10:30.5084826Z Successfully built faf986337748
2019-07-15T12:10:30.6050224Z Successfully tagged rust-ci:latest
2019-07-15T12:10:30.7278568Z Built container sha256:faf9863377480950eab9d671c6c453cec0c8bb06a4e499828f287f71f86e7a36
2019-07-15T12:10:30.7295003Z Uploading finished image to https://.s3.amazonaws.com/docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624
2019-07-15T12:11:33.5970093Z upload failed: - to s3:///docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624 Parameter validation failed:
2019-07-15T12:11:33.5972766Z Invalid bucket name "": Bucket name must match the regex "^[a-zA-Z0-9.\-_]{1,255}$"
2019-07-15T12:11:34.6417419Z [CI_JOB_NAME=x86_64-gnu-llvm-6.0]
2019-07-15T12:11:34.6463602Z Starting sccache server...
2019-07-15T12:11:34.6943848Z configure: processing command line
2019-07-15T12:11:34.6944671Z configure: 
---
2019-07-15T12:39:28.2818063Z    |
2019-07-15T12:39:28.2818362Z 77 | #![cfg_attr(not(bootstrap), feature(const_generics))]
2019-07-15T12:39:28.2818651Z    |                                     ^^^^^^^^^^^^^^
2019-07-15T12:39:28.2818729Z 
2019-07-15T12:39:28.3429848Z error: couldn't read src/libcore/core_arch_docs.md: No such file or directory (os error 2)
2019-07-15T12:39:28.3430214Z   --> src/libcore/../stdarch/crates/core_arch/src/mod.rs:15:42
2019-07-15T12:39:28.3430473Z    |
2019-07-15T12:39:28.3430756Z 15 | #[cfg_attr(not(bootstrap), doc(include = "core_arch_docs.md"))]
2019-07-15T12:39:28.3431103Z    |                                          ^^^^^^^^^^^^^^^^^^^ couldn't read file
2019-07-15T12:39:28.3431352Z    |
2019-07-15T12:39:28.3438547Z    = help: external doc paths are relative to the crate root
2019-07-15T12:39:36.7004872Z    Compiling libc v0.2.54
2019-07-15T12:39:37.5408498Z    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
2019-07-15T12:39:39.2033831Z    Compiling autocfg v0.1.4
2019-07-15T12:39:40.8388256Z error: aborting due to previous error
2019-07-15T12:39:40.8388256Z error: aborting due to previous error
2019-07-15T12:39:40.8391085Z 
2019-07-15T12:39:40.9302589Z    Compiling unwind v0.0.0 (/checkout/src/libunwind)
2019-07-15T12:39:41.0069465Z error: Could not compile `core`.
2019-07-15T12:39:41.0069825Z warning: build failed, waiting for other jobs to finish...
2019-07-15T12:39:41.3613715Z error: build failed
2019-07-15T12:39:41.3638690Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
2019-07-15T12:39:41.3649006Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-07-15T12:39:41.3649181Z Build completed unsuccessfully in 0:24:17
2019-07-15T12:39:41.3649181Z Build completed unsuccessfully in 0:24:17
2019-07-15T12:39:42.2184210Z ##[error]Bash exited with code '1'.
2019-07-15T12:39:42.2215390Z ##[section]Starting: Checkout
2019-07-15T12:39:42.2217637Z ==============================================================================
2019-07-15T12:39:42.2217713Z Task         : Get sources
2019-07-15T12:39:42.2217763Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
