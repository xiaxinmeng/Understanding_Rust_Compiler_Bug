plain
2019-07-13T22:08:27.1049148Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-13T22:08:28.0059218Z ##[command]git config gc.auto 0
2019-07-13T22:08:28.0066100Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-13T22:08:28.0068279Z ##[command]git config --get-all http.proxy
2019-07-13T22:08:28.0071939Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62547/merge:refs/remotes/pull/62547/merge
---
2019-07-13T22:09:00.8197347Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-13T22:09:00.8197383Z 
2019-07-13T22:09:00.8197608Z   git checkout -b <new-branch-name>
2019-07-13T22:09:00.8197639Z 
2019-07-13T22:09:00.8197710Z HEAD is now at 313e1de80 Merge b7157819b7e3f64991eb13d0cde897fbe7b44181 into 69656fa4cbafc378fd63f9186d93b0df3cdd9320
2019-07-13T22:09:00.8365462Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-13T22:09:00.8461372Z ==============================================================================
2019-07-13T22:09:00.8461453Z Task         : Bash
2019-07-13T22:09:00.8461500Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-13T22:10:53.4397589Z Attempting with retry: curl -y 30 -Y 10 --connect-timeout 30 -f -L -C - -o /tmp/rustci_docker_cache https://.s3.amazonaws.com/docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624
2019-07-13T22:10:53.4479716Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-13T22:10:53.4480019Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-13T22:10:53.4480056Z 
2019-07-13T22:10:53.4480639Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-13T22:10:54.4556428Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-13T22:10:54.4556974Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-13T22:10:54.4557146Z 
2019-07-13T22:10:54.4557146Z 
2019-07-13T22:10:54.4600968Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-13T22:10:56.4676931Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-13T22:10:56.4677028Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-13T22:10:56.4677068Z 
2019-07-13T22:10:56.4677068Z 
2019-07-13T22:10:56.4720968Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-13T22:10:59.4804367Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-13T22:10:59.4805743Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-13T22:10:59.4812645Z 
2019-07-13T22:10:59.4812645Z 
2019-07-13T22:10:59.4848059Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-13T22:11:03.4930331Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-13T22:11:03.4930959Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-13T22:11:03.4931271Z 
2019-07-13T22:11:03.4931271Z 
2019-07-13T22:11:03.4976245Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-13T22:11:03.4979686Z The command has failed after 5 attempts.
2019-07-13T22:11:03.6600555Z open /tmp/rustci_docker_cache: no such file or directory
2019-07-13T22:11:03.6636230Z Attempting with retry: docker build --rm -t rust-ci -f /home/vsts/work/1/s/src/ci/docker/x86_64-gnu-llvm-6.0/Dockerfile /home/vsts/work/1/s/src/ci/docker
2019-07-13T22:11:03.8522925Z Sending build context to Docker daemon  521.7kB
2019-07-13T22:11:03.8523136Z 
2019-07-13T22:11:03.8742323Z Step 1/8 : FROM ubuntu:16.04
---
2019-07-13T22:11:20.1149309Z Reading package lists...
2019-07-13T22:11:21.0982580Z Reading package lists...
2019-07-13T22:11:21.2912771Z Building dependency tree...
2019-07-13T22:11:21.2912897Z Reading state information...
2019-07-13T22:11:21.4161861Z The following additional packages will be installed:
2019-07-13T22:11:21.4162722Z   binfmt-support binutils bzip2 cmake-data cpp cpp-5 dpkg-dev g++-5 gcc gcc-5
2019-07-13T22:11:21.4163137Z   git-man libarchive13 libasan2 libasn1-8-heimdal libatomic1
2019-07-13T22:11:21.4163495Z   libbabeltrace-ctf1 libbabeltrace1 libbsd-dev libbsd0 libbz2-1.0 libc-dev-bin
2019-07-13T22:11:21.4164093Z   libedit2 liberror-perl libexpat1 libffi-dev libffi6 libgcc-5-dev libgdbm3
2019-07-13T22:11:21.4164368Z   libglib2.0-0 libgmp10 libgnutls30 libgomp1 libgssapi-krb5-2
2019-07-13T22:11:21.4164627Z   libgssapi3-heimdal libhcrypto4-heimdal libheimbase1-heimdal
2019-07-13T22:11:21.4164968Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-13T22:11:21.4164968Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-13T22:11:21.4169150Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-13T22:11:21.4169590Z   libkrb5-3 libkrb5support0 libldap-2.4-2 libllvm6.0 liblsan0 liblzo2-2
2019-07-13T22:11:21.4169936Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-13T22:11:21.4170561Z   libperl5.22 libpipeline1 libpython-stdlib libpython2.7-minimal
2019-07-13T22:11:21.4170845Z   libpython2.7-stdlib libpython3.5 libpython3.5-minimal libpython3.5-stdlib
2019-07-13T22:11:21.4171160Z   libquadmath0 libroken18-heimdal librtmp1 libsasl2-2 libsasl2-modules-db
2019-07-13T22:11:21.4171769Z   libsqlite3-0 libssl1.0.0 libstdc++-5-dev libtasn1-6 libtinfo-dev libtsan0
2019-07-13T22:11:21.4172145Z   libubsan0 libwind0-heimdal libxml2 linux-libc-dev llvm-6.0 llvm-6.0-dev
2019-07-13T22:11:21.4172465Z   llvm-6.0-runtime mime-support openssl patch perl perl-modules-5.22 python
2019-07-13T22:11:21.4172717Z   python-minimal python2.7-minimal
2019-07-13T22:11:21.4172786Z Suggested packages:
2019-07-13T22:11:21.4173065Z   binutils-doc bzip2-doc codeblocks eclipse ninja-build cpp-doc gcc-5-locales
2019-07-13T22:11:21.4173387Z   debian-keyring g++-multilib g++-5-multilib gcc-5-doc libstdc++6-5-dbg
2019-07-13T22:11:21.4173655Z   gcc-multilib manpages-dev autoconf automake libtool flex bison gcc-doc
2019-07-13T22:11:21.4174271Z   libasan2-dbg liblsan0-dbg libtsan0-dbg libubsan0-dbg libcilkrts5-dbg
2019-07-13T22:11:21.4174538Z   libmpx0-dbg libquadmath0-dbg gdb-doc gettext-base git-daemon-run
2019-07-13T22:11:21.4174538Z   libmpx0-dbg libquadmath0-dbg gdb-doc gettext-base git-daemon-run
2019-07-13T22:11:21.4174812Z   | git-daemon-sysvinit git-doc git-el git-email git-gui gitk gitweb git-arch
2019-07-13T22:11:21.4175158Z   git-cvs git-mediawiki git-svn lrzip glibc-doc gnutls-bin krb5-doc krb5-user
2019-07-13T22:11:21.4175423Z   libstdc++-5-doc llvm-6.0-doc make-doc ed diffutils-doc perl-doc
2019-07-13T22:11:21.4175710Z   libterm-readline-gnu-perl | libterm-readline-perl-perl python-doc python-tk
2019-07-13T22:11:21.4175989Z   python2.7-doc
2019-07-13T22:11:21.4176041Z Recommended packages:
2019-07-13T22:11:21.4176308Z   build-essential fakeroot libalgorithm-merge-perl libc-dbg gdbserver less
2019-07-13T22:11:21.4176629Z   rsync ssh-client manpages manpages-dev libfile-fcntllock-perl
2019-07-13T22:11:21.4176925Z   libglib2.0-data shared-mime-info xdg-user-dirs krb5-locales libsasl2-modules
2019-07-13T22:11:21.4177160Z   libssl-doc xml-core netbase rename
2019-07-13T22:11:21.4177267Z The following NEW packages will be installed:
2019-07-13T22:11:21.4177575Z   binfmt-support binutils bzip2 ca-certificates cmake cmake-data cpp cpp-5
2019-07-13T22:11:21.4178220Z   curl dpkg-dev file g++ g++-5 gcc gcc-5 gdb git git-man libarchive13 libasan2
2019-07-13T22:11:21.4178588Z   libasn1-8-heimdal libatomic1 libbabeltrace-ctf1 libbabeltrace1 libbsd-dev
2019-07-13T22:11:21.4178871Z   libbsd0 libc-dev-bin libc6-dev libcc1-0 libcilkrts5 libcurl3 libcurl3-gnutls
2019-07-13T22:11:21.4179312Z   libdpkg-perl libedit-dev libedit2 liberror-perl libexpat1 libffi-dev libffi6
2019-07-13T22:11:21.4179708Z   libgcc-5-dev libgdbm3 libglib2.0-0 libgmp10 libgnutls30 libgomp1
2019-07-13T22:11:21.4180266Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-13T22:11:21.4180624Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-13T22:11:21.4180899Z   libkrb5-3 libkrb5support0 libldap-2.4-2 libllvm6.0 liblsan0 liblzo2-2
2019-07-13T22:11:21.4181170Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-13T22:11:21.4181170Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-13T22:11:21.4181494Z   libperl5.22 libpipeline1 libpython-stdlib libpython2.7-minimal
2019-07-13T22:11:21.4181771Z   libpython2.7-stdlib libpython3.5 libpython3.5-minimal libpython3.5-stdlib
2019-07-13T22:11:21.4182047Z   libquadmath0 libroken18-heimdal librtmp1 libsasl2-2 libsasl2-modules-db
2019-07-13T22:11:21.4182379Z   libsqlite3-0 libssl-dev libssl1.0.0 libstdc++-5-dev libtasn1-6 libtinfo-dev
2019-07-13T22:11:21.4182653Z   libtsan0 libubsan0 libwind0-heimdal libxml2 linux-libc-dev llvm-6.0
2019-07-13T22:11:21.4182932Z   llvm-6.0-dev llvm-6.0-runtime llvm-6.0-tools make mime-support openssl patch
2019-07-13T22:11:21.4183260Z   perl perl-modules-5.22 pkg-config python python-minimal python2.7
2019-07-13T22:11:21.4183649Z   python2.7-minimal sudo xz-utils zlib1g-dev
2019-07-13T22:11:21.4183710Z The following packages will be upgraded:
2019-07-13T22:11:21.6879059Z 1 upgraded, 115 newly installed, 0 to remove and 4 not upgraded.
2019-07-13T22:11:21.6879478Z Need to get 121 MB of archives.
2019-07-13T22:11:21.6879578Z After this operation, 592 MB of additional disk space will be used.
2019-07-13T22:11:21.6880395Z Get:1 http://archive.ubuntu.com/ubuntu xenial/main amd64 libgdbm3 amd64 1.8.3-13.1 [16.9 kB]
---
2019-07-13T22:11:22.7587835Z Get:61 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 sudo amd64 1.8.16-0ubuntu1.7 [390 kB]
2019-07-13T22:11:22.7693737Z Get:62 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 openssl amd64 1.0.2g-1ubuntu4.15 [492 kB]
2019-07-13T22:11:22.7760914Z Get:63 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 ca-certificates all 20170717~16.04.2 [167 kB]
2019-07-13T22:11:22.7785963Z Get:64 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 libcurl3-gnutls amd64 7.47.0-1ubuntu2.13 [184 kB]
2019-07-13T22:11:22.7815012Z Get:65 http://archive.ubuntu.com/ubuntu xenial/main amd64 libedit2 amd64 3.1-20150325-1ubuntu2 [76.5 kB]
2019-07-13T22:11:22.7996493Z Get:66 http://archive.ubuntu.com/ubuntu xenial/main amd64 libpipeline1 amd64 1.4.1-2 [24.6 kB]
2019-07-13T22:11:22.8005982Z Get:67 http://archive.ubuntu.com/ubuntu xenial/main amd64 binfmt-support amd64 2.1.6-1 [50.7 kB]
2019-07-13T22:11:22.8392458Z Get:69 http://archive.ubuntu.com/ubuntu xenial/main amd64 libisl15 amd64 0.16.1-1 [524 kB]
2019-07-13T22:11:22.8466452Z Get:70 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 cpp-5 amd64 5.4.0-6ubuntu1~16.04.11 [7660 kB]
2019-07-13T22:11:22.9704697Z Get:71 http://archive.ubuntu.com/ubuntu xenial/main amd64 cpp amd64 4:5.3.1-1ubuntu1 [27.7 kB]
2019-07-13T22:11:22.9706186Z Get:72 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 curl amd64 7.47.0-1ubuntu2.13 [139 kB]
---
2019-07-13T22:11:41.0663056Z Unpacking ca-certificates (20170717~16.04.2) ...
2019-07-13T22:11:41.2453606Z Selecting previously unselected package libcurl3-gnutls:amd64.
2019-07-13T22:11:41.2472139Z Preparing to unpack .../libcurl3-gnutls_7.47.0-1ubuntu2.13_amd64.deb ...
2019-07-13T22:11:41.2609437Z Unpacking libcurl3-gnutls:amd64 (7.47.0-1ubuntu2.13) ...
2019-07-13T22:11:41.4269840Z Selecting previously unselected package libedit2:amd64.
2019-07-13T22:11:41.4288795Z Preparing to unpack .../libedit2_3.1-20150325-1ubuntu2_amd64.deb ...
2019-07-13T22:11:41.4468939Z Unpacking libedit2:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-13T22:11:41.5700430Z Selecting previously unselected package libpipeline1:amd64.
2019-07-13T22:11:41.5716197Z Preparing to unpack .../libpipeline1_1.4.1-2_amd64.deb ...
2019-07-13T22:11:41.5903686Z Unpacking libpipeline1:amd64 (1.4.1-2) ...
2019-07-13T22:11:41.7182498Z Selecting previously unselected package binfmt-support.
2019-07-13T22:11:41.7198103Z Preparing to unpack .../binfmt-support_2.1.6-1_amd64.deb ...
2019-07-13T22:11:41.7347970Z Unpacking binfmt-support (2.1.6-1) ...
2019-07-13T22:11:41.8911079Z Preparing to unpack .../binutils_2.26.1-1ubuntu1~16.04.8_amd64.deb ...
2019-07-13T22:11:41.9170386Z Unpacking binutils (2.26.1-1ubuntu1~16.04.8) ...
2019-07-13T22:11:42.3998076Z Selecting previously unselected package libisl15:amd64.
2019-07-13T22:11:42.4012361Z Preparing to unpack .../libisl15_0.16.1-1_amd64.deb ...
---
2019-07-13T22:11:54.1420423Z Selecting previously unselected package libtinfo-dev:amd64.
2019-07-13T22:11:54.1441084Z Preparing to unpack .../libtinfo-dev_6.0+20160213-1ubuntu1_amd64.deb ...
2019-07-13T22:11:54.1571956Z Unpacking libtinfo-dev:amd64 (6.0+20160213-1ubuntu1) ...
2019-07-13T22:11:54.2516281Z Selecting previously unselected package libedit-dev:amd64.
2019-07-13T22:11:54.2536642Z Preparing to unpack .../libedit-dev_3.1-20150325-1ubuntu2_amd64.deb ...
2019-07-13T22:11:54.2654035Z Unpacking libedit-dev:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-13T22:11:54.3750619Z Selecting previously unselected package libllvm6.0:amd64.
2019-07-13T22:11:54.3767043Z Preparing to unpack .../libllvm6.0_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-13T22:11:54.3891005Z Unpacking libllvm6.0:amd64 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-13T22:11:57.3295820Z Selecting previously unselected package zlib1g-dev:amd64.
2019-07-13T22:11:57.3315018Z Preparing to unpack .../zlib1g-dev_1%3a1.2.8.dfsg-2ubuntu4.1_amd64.deb ...
2019-07-13T22:11:57.3439455Z Unpacking zlib1g-dev:amd64 (1:1.2.8.dfsg-2ubuntu4.1) ...
2019-07-13T22:11:57.4437271Z Preparing to unpack .../libssl-dev_1.0.2g-1ubuntu4.15_amd64.deb ...
2019-07-13T22:11:57.4560033Z Unpacking libssl-dev:amd64 (1.0.2g-1ubuntu4.15) ...
2019-07-13T22:11:57.7713213Z Selecting previously unselected package llvm-6.0-runtime.
2019-07-13T22:11:57.7713213Z Selecting previously unselected package llvm-6.0-runtime.
2019-07-13T22:11:57.7732590Z Preparing to unpack .../llvm-6.0-runtime_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-13T22:11:57.7857778Z Unpacking llvm-6.0-runtime (1:6.0-1ubuntu2~16.04.1) ...
2019-07-13T22:11:57.9020518Z Selecting previously unselected package llvm-6.0.
2019-07-13T22:11:57.9045688Z Preparing to unpack .../llvm-6.0_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-13T22:11:57.9172344Z Unpacking llvm-6.0 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-13T22:11:58.5679324Z Selecting previously unselected package libffi-dev:amd64.
2019-07-13T22:11:58.5700035Z Preparing to unpack .../libffi-dev_3.2.1-4_amd64.deb ...
2019-07-13T22:11:58.5829931Z Unpacking libffi-dev:amd64 (3.2.1-4) ...
2019-07-13T22:11:58.7019347Z Selecting previously unselected package llvm-6.0-dev.
2019-07-13T22:11:58.7038241Z Preparing to unpack .../llvm-6.0-dev_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-13T22:11:58.7158991Z Unpacking llvm-6.0-dev (1:6.0-1ubuntu2~16.04.1) ...
2019-07-13T22:12:03.4657645Z Selecting previously unselected package llvm-6.0-tools.
2019-07-13T22:12:03.4682236Z Preparing to unpack .../llvm-6.0-tools_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-13T22:12:03.4803201Z Unpacking llvm-6.0-tools (1:6.0-1ubuntu2~16.04.1) ...
2019-07-13T22:12:03.6391467Z Selecting previously unselected package pkg-config.
2019-07-13T22:12:03.6413866Z Preparing to unpack .../pkg-config_0.29.1-0ubuntu1_amd64.deb ...
2019-07-13T22:12:03.6543183Z Unpacking pkg-config (0.29.1-0ubuntu1) ...
2019-07-13T22:12:03.7697029Z Processing triggers for systemd (229-4ubuntu21.21) ...
2019-07-13T22:12:04.1115931Z Setting up libgdbm3:amd64 (1.8.3-13.1) ...
2019-07-13T22:12:04.1804514Z Setting up libffi6:amd64 (3.2.1-4) ...
2019-07-13T22:12:04.2205013Z Setting up libglib2.0-0:amd64 (2.48.2-0ubuntu4.3) ...
---
2019-07-13T22:12:07.7788833Z debconf: unable to initialize frontend: Dialog
2019-07-13T22:12:07.7789609Z debconf: (TERM is not set, so the dialog frontend is not usable.)
2019-07-13T22:12:07.7789934Z debconf: falling back to frontend: Readline
2019-07-13T22:12:08.2704470Z Setting up libcurl3-gnutls:amd64 (7.47.0-1ubuntu2.13) ...
2019-07-13T22:12:08.3120245Z Setting up libedit2:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-13T22:12:08.3489554Z Setting up libpipeline1:amd64 (1.4.1-2) ...
2019-07-13T22:12:08.3870274Z Setting up binfmt-support (2.1.6-1) ...
2019-07-13T22:12:08.4584991Z mount: permission denied
2019-07-13T22:12:08.4587088Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-13T22:12:08.4612761Z mount: permission denied
2019-07-13T22:12:08.4615521Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-13T22:12:08.6139564Z invoke-rc.d: could not determine current runlevel
2019-07-13T22:12:08.6170306Z invoke-rc.d: policy-rc.d denied execution of start.
2019-07-13T22:12:08.6760867Z Setting up libisl15:amd64 (0.16.1-1) ...
2019-07-13T22:12:08.7176076Z Setting up cpp-5 (5.4.0-6ubuntu1~16.04.11) ...
2019-07-13T22:12:08.7564799Z Setting up cpp (4:5.3.1-1ubuntu1) ...
2019-07-13T22:12:08.8027658Z Setting up curl (7.47.0-1ubuntu2.13) ...
---
2019-07-13T22:12:10.3732182Z Setting up libedit-dev:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-13T22:12:10.4114328Z Setting up libllvm6.0:amd64 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-13T22:12:10.4551423Z Setting up zlib1g-dev:amd64 (1:1.2.8.dfsg-2ubuntu4.1) ...
2019-07-13T22:12:10.4976320Z Setting up libssl-dev:amd64 (1.0.2g-1ubuntu4.15) ...
2019-07-13T22:12:10.5412059Z Setting up llvm-6.0-runtime (1:6.0-1ubuntu2~16.04.1) ...
2019-07-13T22:12:10.5749972Z mount: permission denied
2019-07-13T22:12:10.5754984Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-13T22:12:10.5913423Z Setting up llvm-6.0 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-13T22:12:10.6294124Z Setting up libffi-dev:amd64 (3.2.1-4) ...
2019-07-13T22:12:10.6716566Z Setting up llvm-6.0-dev (1:6.0-1ubuntu2~16.04.1) ...
2019-07-13T22:12:10.7083793Z Setting up llvm-6.0-tools (1:6.0-1ubuntu2~16.04.1) ...
2019-07-13T22:12:10.7444533Z Setting up pkg-config (0.29.1-0ubuntu1) ...
2019-07-13T22:12:10.8713038Z Processing triggers for ca-certificates (20170717~16.04.2) ...
2019-07-13T22:12:10.8878120Z Updating certificates in /etc/ssl/certs...
2019-07-13T22:12:12.4495092Z 148 added, 0 removed; done.
2019-07-13T22:12:12.4496533Z Running hooks in /etc/ca-certificates/update.d...
---
2019-07-13T22:12:45.7879772Z  ---> 4728fd4f5d53
2019-07-13T22:12:45.7919493Z Successfully built 4728fd4f5d53
2019-07-13T22:12:45.9036373Z Successfully tagged rust-ci:latest
2019-07-13T22:12:45.9991258Z Built container sha256:4728fd4f5d537b6f00bf1be59bbab8248b0eb0a43c31f8df5f67136654c9a222
2019-07-13T22:12:46.0013852Z Uploading finished image to https://.s3.amazonaws.com/docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624
2019-07-13T22:13:47.7499558Z upload failed: - to s3:///docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624 Parameter validation failed:
2019-07-13T22:13:47.7500026Z Invalid bucket name "": Bucket name must match the regex "^[a-zA-Z0-9.\-_]{1,255}$"
2019-07-13T22:13:48.6928945Z [CI_JOB_NAME=x86_64-gnu-llvm-6.0]
2019-07-13T22:13:48.6981306Z Starting sccache server...
2019-07-13T22:13:48.7499216Z configure: processing command line
2019-07-13T22:13:48.7500121Z configure: 
---
2019-07-13T22:17:23.7422953Z    Compiling serde_json v1.0.33
2019-07-13T22:17:28.0766744Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-07-13T22:17:37.0072161Z     Finished release [optimized] target(s) in 1m 32s
2019-07-13T22:17:37.0145506Z tidy check
2019-07-13T22:17:37.2876711Z tidy error: /checkout/src/librustc_mir/dataflow/impls/use_def_chain.rs:252: line longer than 100 chars
2019-07-13T22:17:38.9181962Z some tidy checks failed
2019-07-13T22:17:38.9183049Z 
2019-07-13T22:17:38.9183049Z 
2019-07-13T22:17:38.9184176Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-07-13T22:17:38.9188022Z 
2019-07-13T22:17:38.9188142Z 
2019-07-13T22:17:38.9199982Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-07-13T22:17:38.9200239Z Build completed unsuccessfully in 0:01:35
2019-07-13T22:17:38.9200239Z Build completed unsuccessfully in 0:01:35
2019-07-13T22:17:40.1437334Z ##[error]Bash exited with code '1'.
2019-07-13T22:17:40.1470207Z ##[section]Starting: Checkout
2019-07-13T22:17:40.1471835Z ==============================================================================
2019-07-13T22:17:40.1471904Z Task         : Get sources
2019-07-13T22:17:40.1471947Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
