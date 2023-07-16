plain
2019-07-10T03:18:46.8173038Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-10T03:18:46.8364477Z ##[command]git config gc.auto 0
2019-07-10T03:18:46.8404005Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-10T03:18:46.8463725Z ##[command]git config --get-all http.proxy
2019-07-10T03:18:46.8598599Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62550/merge:refs/remotes/pull/62550/merge
---
2019-07-10T03:19:20.8259407Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-10T03:19:20.8259615Z 
2019-07-10T03:19:20.8260020Z   git checkout -b <new-branch-name>
2019-07-10T03:19:20.8260221Z 
2019-07-10T03:19:20.8260262Z HEAD is now at 32c369d60 Merge 637ed8f445bd3869dfac036ea33df2b22998666d into e7efdf1c33c699def0e594f03337efc78120bd9c
2019-07-10T03:19:20.8414934Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-10T03:19:20.8417195Z ==============================================================================
2019-07-10T03:19:20.8417259Z Task         : Bash
2019-07-10T03:19:20.8417297Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-10T03:20:55.5387959Z Attempting with retry: curl -y 30 -Y 10 --connect-timeout 30 -f -L -C - -o /tmp/rustci_docker_cache https://.s3.amazonaws.com/docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624
2019-07-10T03:20:55.5444478Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-10T03:20:55.5444981Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-10T03:20:55.5445136Z 
2019-07-10T03:20:55.5488345Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-10T03:20:56.5560647Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-10T03:20:56.5560736Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-10T03:20:56.5560869Z 
2019-07-10T03:20:56.5560869Z 
2019-07-10T03:20:56.5604064Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-10T03:20:58.5669114Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-10T03:20:58.5669483Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-10T03:20:58.5669681Z 
2019-07-10T03:20:58.5669681Z 
2019-07-10T03:20:58.5712208Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-10T03:21:01.5779777Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-10T03:21:01.5780441Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-10T03:21:01.5780596Z 
2019-07-10T03:21:01.5780596Z 
2019-07-10T03:21:01.5822815Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-10T03:21:05.5892789Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-10T03:21:05.5892894Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-10T03:21:05.5893014Z 
2019-07-10T03:21:05.5893014Z 
2019-07-10T03:21:05.5935237Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-10T03:21:05.5939205Z The command has failed after 5 attempts.
2019-07-10T03:21:05.7938786Z open /tmp/rustci_docker_cache: no such file or directory
2019-07-10T03:21:05.7962350Z Attempting with retry: docker build --rm -t rust-ci -f /home/vsts/work/1/s/src/ci/docker/x86_64-gnu-llvm-6.0/Dockerfile /home/vsts/work/1/s/src/ci/docker
2019-07-10T03:21:05.9628343Z Sending build context to Docker daemon  521.7kB
2019-07-10T03:21:05.9628917Z 
2019-07-10T03:21:05.9899782Z Step 1/8 : FROM ubuntu:16.04
---
2019-07-10T03:21:22.0458431Z Reading package lists...
2019-07-10T03:21:22.9520152Z Reading package lists...
2019-07-10T03:21:23.1170627Z Building dependency tree...
2019-07-10T03:21:23.1171253Z Reading state information...
2019-07-10T03:21:23.2350730Z The following additional packages will be installed:
2019-07-10T03:21:23.2352665Z   binfmt-support binutils bzip2 cmake-data cpp cpp-5 dpkg-dev g++-5 gcc gcc-5
2019-07-10T03:21:23.2353260Z   git-man libarchive13 libasan2 libasn1-8-heimdal libatomic1
2019-07-10T03:21:23.2353528Z   libbabeltrace-ctf1 libbabeltrace1 libbsd-dev libbsd0 libbz2-1.0 libc-dev-bin
2019-07-10T03:21:23.2354102Z   libedit2 liberror-perl libexpat1 libffi-dev libffi6 libgcc-5-dev libgdbm3
2019-07-10T03:21:23.2354346Z   libglib2.0-0 libgmp10 libgnutls30 libgomp1 libgssapi-krb5-2
2019-07-10T03:21:23.2362243Z   libgssapi3-heimdal libhcrypto4-heimdal libheimbase1-heimdal
2019-07-10T03:21:23.2362975Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-10T03:21:23.2362975Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-10T03:21:23.2363252Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-10T03:21:23.2363572Z   libkrb5-3 libkrb5support0 libldap-2.4-2 libllvm6.0 liblsan0 liblzo2-2
2019-07-10T03:21:23.2363824Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-10T03:21:23.2364069Z   libperl5.22 libpipeline1 libpython-stdlib libpython2.7-minimal
2019-07-10T03:21:23.2364376Z   libpython2.7-stdlib libpython3.5 libpython3.5-minimal libpython3.5-stdlib
2019-07-10T03:21:23.2364631Z   libquadmath0 libroken18-heimdal librtmp1 libsasl2-2 libsasl2-modules-db
2019-07-10T03:21:23.2364887Z   libsqlite3-0 libssl1.0.0 libstdc++-5-dev libtasn1-6 libtinfo-dev libtsan0
2019-07-10T03:21:23.2365179Z   libubsan0 libwind0-heimdal libxml2 linux-libc-dev llvm-6.0 llvm-6.0-dev
2019-07-10T03:21:23.2365433Z   llvm-6.0-runtime mime-support openssl patch perl perl-modules-5.22 python
2019-07-10T03:21:23.2365676Z   python-minimal python2.7-minimal
2019-07-10T03:21:23.2365761Z Suggested packages:
2019-07-10T03:21:23.2366024Z   binutils-doc bzip2-doc codeblocks eclipse ninja-build cpp-doc gcc-5-locales
2019-07-10T03:21:23.2366393Z   debian-keyring g++-multilib g++-5-multilib gcc-5-doc libstdc++6-5-dbg
2019-07-10T03:21:23.2366631Z   gcc-multilib manpages-dev autoconf automake libtool flex bison gcc-doc
2019-07-10T03:21:23.2367135Z   libasan2-dbg liblsan0-dbg libtsan0-dbg libubsan0-dbg libcilkrts5-dbg
2019-07-10T03:21:23.2367339Z   libmpx0-dbg libquadmath0-dbg gdb-doc gettext-base git-daemon-run
2019-07-10T03:21:23.2367339Z   libmpx0-dbg libquadmath0-dbg gdb-doc gettext-base git-daemon-run
2019-07-10T03:21:23.2367541Z   | git-daemon-sysvinit git-doc git-el git-email git-gui gitk gitweb git-arch
2019-07-10T03:21:23.2367780Z   git-cvs git-mediawiki git-svn lrzip glibc-doc gnutls-bin krb5-doc krb5-user
2019-07-10T03:21:23.2367979Z   libstdc++-5-doc llvm-6.0-doc make-doc ed diffutils-doc perl-doc
2019-07-10T03:21:23.2368204Z   libterm-readline-gnu-perl | libterm-readline-perl-perl python-doc python-tk
2019-07-10T03:21:23.2368402Z   python2.7-doc
2019-07-10T03:21:23.2368441Z Recommended packages:
2019-07-10T03:21:23.2368642Z   build-essential fakeroot libalgorithm-merge-perl libc-dbg gdbserver less
2019-07-10T03:21:23.2368870Z   rsync ssh-client manpages manpages-dev libfile-fcntllock-perl
2019-07-10T03:21:23.2369330Z   libglib2.0-data shared-mime-info xdg-user-dirs krb5-locales libsasl2-modules
2019-07-10T03:21:23.2369515Z   libssl-doc xml-core netbase rename
2019-07-10T03:21:23.2369557Z The following NEW packages will be installed:
2019-07-10T03:21:23.2369838Z   binfmt-support binutils bzip2 ca-certificates cmake cmake-data cpp cpp-5
2019-07-10T03:21:23.2370067Z   curl dpkg-dev file g++ g++-5 gcc gcc-5 gdb git git-man libarchive13 libasan2
2019-07-10T03:21:23.2370287Z   libasn1-8-heimdal libatomic1 libbabeltrace-ctf1 libbabeltrace1 libbsd-dev
2019-07-10T03:21:23.2370791Z   libbsd0 libc-dev-bin libc6-dev libcc1-0 libcilkrts5 libcurl3 libcurl3-gnutls
2019-07-10T03:21:23.2371103Z   libdpkg-perl libedit-dev libedit2 liberror-perl libexpat1 libffi-dev libffi6
2019-07-10T03:21:23.2371302Z   libgcc-5-dev libgdbm3 libglib2.0-0 libgmp10 libgnutls30 libgomp1
2019-07-10T03:21:23.2371760Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-10T03:21:23.2371963Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-10T03:21:23.2372209Z   libkrb5-3 libkrb5support0 libldap-2.4-2 libllvm6.0 liblsan0 liblzo2-2
2019-07-10T03:21:23.2372408Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-10T03:21:23.2372408Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-10T03:21:23.2372602Z   libperl5.22 libpipeline1 libpython-stdlib libpython2.7-minimal
2019-07-10T03:21:23.2372903Z   libpython2.7-stdlib libpython3.5 libpython3.5-minimal libpython3.5-stdlib
2019-07-10T03:21:23.2373390Z   libquadmath0 libroken18-heimdal librtmp1 libsasl2-2 libsasl2-modules-db
2019-07-10T03:21:23.2373661Z   libsqlite3-0 libssl-dev libssl1.0.0 libstdc++-5-dev libtasn1-6 libtinfo-dev
2019-07-10T03:21:23.2373951Z   libtsan0 libubsan0 libwind0-heimdal libxml2 linux-libc-dev llvm-6.0
2019-07-10T03:21:23.2374208Z   llvm-6.0-dev llvm-6.0-runtime llvm-6.0-tools make mime-support openssl patch
2019-07-10T03:21:23.2374455Z   perl perl-modules-5.22 pkg-config python python-minimal python2.7
2019-07-10T03:21:23.2374733Z   python2.7-minimal sudo xz-utils zlib1g-dev
2019-07-10T03:21:23.2374786Z The following packages will be upgraded:
2019-07-10T03:21:23.6259325Z 1 upgraded, 115 newly installed, 0 to remove and 4 not upgraded.
2019-07-10T03:21:23.6259530Z Need to get 121 MB of archives.
2019-07-10T03:21:23.6259816Z After this operation, 592 MB of additional disk space will be used.
2019-07-10T03:21:23.6260681Z Get:1 http://archive.ubuntu.com/ubuntu xenial/main amd64 libgdbm3 amd64 1.8.3-13.1 [16.9 kB]
---
2019-07-10T03:21:26.0617742Z Get:61 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 sudo amd64 1.8.16-0ubuntu1.7 [390 kB]
2019-07-10T03:21:26.1342268Z Get:62 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 openssl amd64 1.0.2g-1ubuntu4.15 [492 kB]
2019-07-10T03:21:26.1409600Z Get:63 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 ca-certificates all 20170717~16.04.2 [167 kB]
2019-07-10T03:21:26.1436563Z Get:64 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 libcurl3-gnutls amd64 7.47.0-1ubuntu2.13 [184 kB]
2019-07-10T03:21:26.1462251Z Get:65 http://archive.ubuntu.com/ubuntu xenial/main amd64 libedit2 amd64 3.1-20150325-1ubuntu2 [76.5 kB]
2019-07-10T03:21:26.1476022Z Get:66 http://archive.ubuntu.com/ubuntu xenial/main amd64 libpipeline1 amd64 1.4.1-2 [24.6 kB]
2019-07-10T03:21:26.1483818Z Get:67 http://archive.ubuntu.com/ubuntu xenial/main amd64 binfmt-support amd64 2.1.6-1 [50.7 kB]
2019-07-10T03:21:26.2727836Z Get:69 http://archive.ubuntu.com/ubuntu xenial/main amd64 libisl15 amd64 0.16.1-1 [524 kB]
2019-07-10T03:21:26.2819363Z Get:70 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 cpp-5 amd64 5.4.0-6ubuntu1~16.04.11 [7660 kB]
2019-07-10T03:21:26.6469450Z Get:71 http://archive.ubuntu.com/ubuntu xenial/main amd64 cpp amd64 4:5.3.1-1ubuntu1 [27.7 kB]
2019-07-10T03:21:26.6469878Z Get:72 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 curl amd64 7.47.0-1ubuntu2.13 [139 kB]
---
2019-07-10T03:21:46.2597079Z Unpacking ca-certificates (20170717~16.04.2) ...
2019-07-10T03:21:46.4198640Z Selecting previously unselected package libcurl3-gnutls:amd64.
2019-07-10T03:21:46.4208411Z Preparing to unpack .../libcurl3-gnutls_7.47.0-1ubuntu2.13_amd64.deb ...
2019-07-10T03:21:46.4341824Z Unpacking libcurl3-gnutls:amd64 (7.47.0-1ubuntu2.13) ...
2019-07-10T03:21:46.5660594Z Selecting previously unselected package libedit2:amd64.
2019-07-10T03:21:46.5672002Z Preparing to unpack .../libedit2_3.1-20150325-1ubuntu2_amd64.deb ...
2019-07-10T03:21:46.5800426Z Unpacking libedit2:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-10T03:21:46.6930518Z Selecting previously unselected package libpipeline1:amd64.
2019-07-10T03:21:46.6943991Z Preparing to unpack .../libpipeline1_1.4.1-2_amd64.deb ...
2019-07-10T03:21:46.7074496Z Unpacking libpipeline1:amd64 (1.4.1-2) ...
2019-07-10T03:21:46.8158248Z Selecting previously unselected package binfmt-support.
2019-07-10T03:21:46.8168902Z Preparing to unpack .../binfmt-support_2.1.6-1_amd64.deb ...
2019-07-10T03:21:46.8299854Z Unpacking binfmt-support (2.1.6-1) ...
2019-07-10T03:21:46.9496141Z Preparing to unpack .../binutils_2.26.1-1ubuntu1~16.04.8_amd64.deb ...
2019-07-10T03:21:46.9630131Z Unpacking binutils (2.26.1-1ubuntu1~16.04.8) ...
2019-07-10T03:21:47.4183403Z Selecting previously unselected package libisl15:amd64.
2019-07-10T03:21:47.4196309Z Preparing to unpack .../libisl15_0.16.1-1_amd64.deb ...
---
2019-07-10T03:21:58.4689954Z Selecting previously unselected package libtinfo-dev:amd64.
2019-07-10T03:21:58.4705312Z Preparing to unpack .../libtinfo-dev_6.0+20160213-1ubuntu1_amd64.deb ...
2019-07-10T03:21:58.4845802Z Unpacking libtinfo-dev:amd64 (6.0+20160213-1ubuntu1) ...
2019-07-10T03:21:58.5773457Z Selecting previously unselected package libedit-dev:amd64.
2019-07-10T03:21:58.5791743Z Preparing to unpack .../libedit-dev_3.1-20150325-1ubuntu2_amd64.deb ...
2019-07-10T03:21:58.5915592Z Unpacking libedit-dev:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-10T03:21:58.7014578Z Selecting previously unselected package libllvm6.0:amd64.
2019-07-10T03:21:58.7031172Z Preparing to unpack .../libllvm6.0_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-10T03:21:58.7162159Z Unpacking libllvm6.0:amd64 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-10T03:22:01.4510162Z Selecting previously unselected package zlib1g-dev:amd64.
2019-07-10T03:22:01.4526083Z Preparing to unpack .../zlib1g-dev_1%3a1.2.8.dfsg-2ubuntu4.1_amd64.deb ...
2019-07-10T03:22:01.4664482Z Unpacking zlib1g-dev:amd64 (1:1.2.8.dfsg-2ubuntu4.1) ...
2019-07-10T03:22:01.5698382Z Preparing to unpack .../libssl-dev_1.0.2g-1ubuntu4.15_amd64.deb ...
2019-07-10T03:22:01.5820697Z Unpacking libssl-dev:amd64 (1.0.2g-1ubuntu4.15) ...
2019-07-10T03:22:01.8995288Z Selecting previously unselected package llvm-6.0-runtime.
2019-07-10T03:22:01.8995288Z Selecting previously unselected package llvm-6.0-runtime.
2019-07-10T03:22:01.9013313Z Preparing to unpack .../llvm-6.0-runtime_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-10T03:22:01.9169307Z Unpacking llvm-6.0-runtime (1:6.0-1ubuntu2~16.04.1) ...
2019-07-10T03:22:02.0381044Z Selecting previously unselected package llvm-6.0.
2019-07-10T03:22:02.0395554Z Preparing to unpack .../llvm-6.0_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-10T03:22:02.0522267Z Unpacking llvm-6.0 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-10T03:22:02.6482573Z Selecting previously unselected package libffi-dev:amd64.
2019-07-10T03:22:02.6502540Z Preparing to unpack .../libffi-dev_3.2.1-4_amd64.deb ...
2019-07-10T03:22:02.6653822Z Unpacking libffi-dev:amd64 (3.2.1-4) ...
2019-07-10T03:22:02.7775118Z Selecting previously unselected package llvm-6.0-dev.
2019-07-10T03:22:02.7790207Z Preparing to unpack .../llvm-6.0-dev_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-10T03:22:02.7919705Z Unpacking llvm-6.0-dev (1:6.0-1ubuntu2~16.04.1) ...
2019-07-10T03:22:07.2476924Z Selecting previously unselected package llvm-6.0-tools.
2019-07-10T03:22:07.2505389Z Preparing to unpack .../llvm-6.0-tools_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-10T03:22:07.2631720Z Unpacking llvm-6.0-tools (1:6.0-1ubuntu2~16.04.1) ...
2019-07-10T03:22:07.4137773Z Selecting previously unselected package pkg-config.
2019-07-10T03:22:07.4157962Z Preparing to unpack .../pkg-config_0.29.1-0ubuntu1_amd64.deb ...
2019-07-10T03:22:07.4291665Z Unpacking pkg-config (0.29.1-0ubuntu1) ...
2019-07-10T03:22:07.5383478Z Processing triggers for systemd (229-4ubuntu21.21) ...
2019-07-10T03:22:07.8989903Z Setting up libgdbm3:amd64 (1.8.3-13.1) ...
2019-07-10T03:22:07.9698142Z Setting up libffi6:amd64 (3.2.1-4) ...
2019-07-10T03:22:08.0079426Z Setting up libglib2.0-0:amd64 (2.48.2-0ubuntu4.3) ...
---
2019-07-10T03:22:11.4982117Z debconf: unable to initialize frontend: Dialog
2019-07-10T03:22:11.4982210Z debconf: (TERM is not set, so the dialog frontend is not usable.)
2019-07-10T03:22:11.4982577Z debconf: falling back to frontend: Readline
2019-07-10T03:22:11.9549806Z Setting up libcurl3-gnutls:amd64 (7.47.0-1ubuntu2.13) ...
2019-07-10T03:22:11.9958509Z Setting up libedit2:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-10T03:22:12.0387592Z Setting up libpipeline1:amd64 (1.4.1-2) ...
2019-07-10T03:22:12.0791259Z Setting up binfmt-support (2.1.6-1) ...
2019-07-10T03:22:12.1521740Z mount: permission denied
2019-07-10T03:22:12.1527576Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-10T03:22:12.1540962Z mount: permission denied
2019-07-10T03:22:12.1548432Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-10T03:22:12.2996302Z invoke-rc.d: could not determine current runlevel
2019-07-10T03:22:12.3025119Z invoke-rc.d: policy-rc.d denied execution of start.
2019-07-10T03:22:12.3654817Z Setting up libisl15:amd64 (0.16.1-1) ...
2019-07-10T03:22:12.4095686Z Setting up cpp-5 (5.4.0-6ubuntu1~16.04.11) ...
2019-07-10T03:22:12.4479289Z Setting up cpp (4:5.3.1-1ubuntu1) ...
2019-07-10T03:22:12.4967556Z Setting up curl (7.47.0-1ubuntu2.13) ...
---
2019-07-10T03:22:14.1645283Z Setting up libedit-dev:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-10T03:22:14.2094280Z Setting up libllvm6.0:amd64 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-10T03:22:14.2484715Z Setting up zlib1g-dev:amd64 (1:1.2.8.dfsg-2ubuntu4.1) ...
2019-07-10T03:22:14.2871545Z Setting up libssl-dev:amd64 (1.0.2g-1ubuntu4.15) ...
2019-07-10T03:22:14.3326271Z Setting up llvm-6.0-runtime (1:6.0-1ubuntu2~16.04.1) ...
2019-07-10T03:22:14.3612643Z mount: permission denied
2019-07-10T03:22:14.3618223Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-10T03:22:14.3769320Z Setting up llvm-6.0 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-10T03:22:14.4317926Z Setting up libffi-dev:amd64 (3.2.1-4) ...
2019-07-10T03:22:14.4750054Z Setting up llvm-6.0-dev (1:6.0-1ubuntu2~16.04.1) ...
2019-07-10T03:22:14.5171116Z Setting up llvm-6.0-tools (1:6.0-1ubuntu2~16.04.1) ...
2019-07-10T03:22:14.5639856Z Setting up pkg-config (0.29.1-0ubuntu1) ...
2019-07-10T03:22:14.6875049Z Processing triggers for ca-certificates (20170717~16.04.2) ...
2019-07-10T03:22:14.7037222Z Updating certificates in /etc/ssl/certs...
2019-07-10T03:22:16.1005409Z 148 added, 0 removed; done.
2019-07-10T03:22:16.1006529Z Running hooks in /etc/ca-certificates/update.d...
---
2019-07-10T03:22:49.0837327Z Removing intermediate container 2689fd576cc0
2019-07-10T03:22:49.0838553Z  ---> abff63f28197
2019-07-10T03:22:49.0876989Z Successfully built abff63f28197
2019-07-10T03:22:49.2674671Z Successfully tagged rust-ci:latest
2019-07-10T03:22:49.3395549Z Built container sha256:abff63f28197436a769c2695a590b980f71fe25cdd07224738a47a4d76c67c30
2019-07-10T03:22:49.3407575Z Uploading finished image to https://.s3.amazonaws.com/docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624
2019-07-10T03:23:46.5429301Z upload failed: - to s3:///docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624 Parameter validation failed:
2019-07-10T03:23:46.5429847Z Invalid bucket name "": Bucket name must match the regex "^[a-zA-Z0-9.\-_]{1,255}$"
2019-07-10T03:23:47.7826997Z [CI_JOB_NAME=x86_64-gnu-llvm-6.0]
2019-07-10T03:23:47.7870095Z Starting sccache server...
2019-07-10T03:23:47.8366257Z configure: processing command line
2019-07-10T03:23:47.8366946Z configure: 
---
2019-07-10T04:15:32.1663817Z .................................................................................................... 200/5770
2019-07-10T04:15:36.2171619Z .................................................................................................... 300/5770
2019-07-10T04:15:39.9411310Z .................................................................................................... 400/5770
2019-07-10T04:15:43.7059583Z .................................................................................................... 500/5770
2019-07-10T04:15:47.5950899Z ..................................................................i................................. 600/5770
2019-07-10T04:15:56.3315055Z .................................................................................................... 800/5770
2019-07-10T04:16:01.8019190Z .................................................................................................... 900/5770
2019-07-10T04:16:01.8019190Z .................................................................................................... 900/5770
2019-07-10T04:16:07.7068808Z .....................................................................................i...........i.. 1000/5770
2019-07-10T04:16:11.8770673Z .................................................................................................... 1100/5770
2019-07-10T04:16:15.9576960Z ..............iiiii................................................................................. 1200/5770
2019-07-10T04:16:21.7091234Z .................................................................................................... 1400/5770
2019-07-10T04:16:24.7309727Z .................................................................................................... 1500/5770
2019-07-10T04:16:28.2843024Z .................................................................................................... 1600/5770
2019-07-10T04:16:30.9687087Z .................................................................................................... 1700/5770
2019-07-10T04:16:30.9687087Z .................................................................................................... 1700/5770
2019-07-10T04:16:34.4708432Z ....................................................i............................................... 1800/5770
2019-07-10T04:16:43.0179994Z .................................................................................................... 2000/5770
2019-07-10T04:16:46.9892319Z .................................................................................................... 2100/5770
2019-07-10T04:16:51.0009787Z .................................................................................................... 2200/5770
2019-07-10T04:16:51.0009787Z .................................................................................................... 2200/5770
2019-07-10T04:16:54.6462541Z .............i...................................................................................... 2300/5770
2019-07-10T04:17:04.4371687Z .................................................................................................... 2500/5770
2019-07-10T04:17:09.7627189Z .................................................................................................... 2600/5770
2019-07-10T04:17:13.3320115Z .................................................................................................... 2700/5770
2019-07-10T04:17:17.7395870Z .................................................................................................... 2800/5770
2019-07-10T04:17:17.7395870Z .................................................................................................... 2800/5770
2019-07-10T04:17:22.5302982Z .................................................................................................... 2900/5770
2019-07-10T04:17:27.5395473Z .................................................................................................... 3000/5770
2019-07-10T04:17:32.2551301Z .................................................................................................... 3100/5770
2019-07-10T04:17:36.8658049Z .................................................................................................... 3200/5770
2019-07-10T04:17:40.0417608Z .................................................................................................... 3300/5770
2019-07-10T04:17:44.9141098Z .................................................................................................... 3400/5770
2019-07-10T04:17:48.9562354Z ...................................................................i................................ 3500/5770
2019-07-10T04:17:52.7066460Z .................................................................................................... 3600/5770
2019-07-10T04:17:56.8338187Z .........................................ii...i..ii................................................. 3700/5770
2019-07-10T04:18:05.6153250Z .................................................................................................... 3900/5770
2019-07-10T04:18:05.6153250Z .................................................................................................... 3900/5770
2019-07-10T04:18:09.6324038Z .......................................................ii........................................... 4000/5770
2019-07-10T04:18:12.1354320Z ............................................................................i....................... 4100/5770
2019-07-10T04:18:14.4683733Z .................................................................................................... 4200/5770
2019-07-10T04:18:16.6876074Z .......................................Fi......................F.........FF......................... 4300/5770
2019-07-10T04:18:34.8328070Z .................................................................................................... 4500/5770
2019-07-10T04:18:41.7869977Z .................................................................................................... 4600/5770
2019-07-10T04:18:45.1934468Z .................................................................................................... 4700/5770
2019-07-10T04:18:49.7027227Z .................................................................................................... 4800/5770
---
2019-07-10T04:19:21.9547621Z .................................................................................................... 5400/5770
2019-07-10T04:19:26.4472759Z .................................................................................................... 5500/5770
2019-07-10T04:19:29.5850736Z .................................................................................................... 5600/5770
2019-07-10T04:19:32.5412903Z .................................................................................................... 5700/5770
2019-07-10T04:19:34.9368482Z ..........i...........................................................
2019-07-10T04:19:34.9417464Z 
2019-07-10T04:19:34.9418421Z ---- [ui] ui/parser/match-vec-invalid.rs stdout ----
2019-07-10T04:19:34.9419107Z diff of stderr:
2019-07-10T04:19:34.9419396Z 
2019-07-10T04:19:34.9419396Z 
2019-07-10T04:19:34.9419630Z 11    |             ^^^^^^^^^
2019-07-10T04:19:34.9419875Z 12    |
2019-07-10T04:19:34.9420549Z 13    = note: for more information, see ***/issues/62254
2019-07-10T04:19:34.9421152Z -    = help: add #![feature(slice_patterns)] to the crate attributes to enable
2019-07-10T04:19:34.9421457Z +    = help: add `#![feature(slice_patterns)]` to the crate attributes to enable
2019-07-10T04:19:34.9421681Z 15 
2019-07-10T04:19:34.9421927Z 16 error[E0658]: subslice patterns are unstable
2019-07-10T04:19:34.9422796Z 17   --> $DIR/match-vec-invalid.rs:4:24
2019-07-10T04:19:34.9423661Z 20    |                        ^^^^^^^^^
2019-07-10T04:19:34.9423881Z 21    |
2019-07-10T04:19:34.9423881Z 21    |
2019-07-10T04:19:34.9424483Z 22    = note: for more information, see ***/issues/62254
2019-07-10T04:19:34.9425051Z -    = help: add #![feature(slice_patterns)] to the crate attributes to enable
2019-07-10T04:19:34.9425901Z +    = help: add `#![feature(slice_patterns)]` to the crate attributes to enable
2019-07-10T04:19:34.9426157Z 24 
2019-07-10T04:19:34.9426350Z 25 error: `..` can only be used once per slice pattern
2019-07-10T04:19:34.9426816Z 26   --> $DIR/match-vec-invalid.rs:4:31
2019-07-10T04:19:34.9427136Z 
2019-07-10T04:19:34.9428586Z The actual stderr differed from the expected stderr.
2019-07-10T04:19:34.9428586Z The actual stderr differed from the expected stderr.
2019-07-10T04:19:34.9429697Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/match-vec-invalid/match-vec-invalid.stderr
2019-07-10T04:19:34.9430235Z To update references, rerun the tests and pass the `--bless` flag
2019-07-10T04:19:34.9430707Z To only update this specific test, also pass `--test-args parser/match-vec-invalid.rs`
2019-07-10T04:19:34.9431070Z error: 1 errors occurred comparing output.
2019-07-10T04:19:34.9431229Z status: exit code: 1
2019-07-10T04:19:34.9431229Z status: exit code: 1
2019-07-10T04:19:34.9432138Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/match-vec-invalid.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/match-vec-invalid" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/match-vec-invalid/auxiliary" "-A" "unused"
2019-07-10T04:19:34.9432761Z ------------------------------------------
2019-07-10T04:19:34.9433326Z 
2019-07-10T04:19:34.9433761Z ------------------------------------------
2019-07-10T04:19:34.9433976Z stderr:
2019-07-10T04:19:34.9433976Z stderr:
2019-07-10T04:19:34.9434334Z ------------------------------------------
2019-07-10T04:19:34.9434584Z error[E0416]: identifier `tail` is bound more than once in the same pattern
2019-07-10T04:19:34.9435213Z    |
2019-07-10T04:19:34.9435213Z    |
2019-07-10T04:19:34.9435373Z LL |         [1, tail @ .., tail @ ..] => {},
2019-07-10T04:19:34.9435567Z    |                        ^^^^ used in a pattern more than once
2019-07-10T04:19:34.9435890Z error[E0658]: subslice patterns are unstable
2019-07-10T04:19:34.9436293Z   --> /checkout/src/test/ui/parser/match-vec-invalid.rs:4:13
2019-07-10T04:19:34.9436483Z    |
2019-07-10T04:19:34.9436483Z    |
2019-07-10T04:19:34.9436645Z LL |         [1, tail @ .., tail @ ..] => {},
2019-07-10T04:19:34.9436980Z    |
2019-07-10T04:19:34.9436980Z    |
2019-07-10T04:19:34.9437475Z    = note: for more information, see ***/issues/62254
2019-07-10T04:19:34.9437692Z    = help: add `#![feature(slice_patterns)]` to the crate attributes to enable
2019-07-10T04:19:34.9438012Z error[E0658]: subslice patterns are unstable
2019-07-10T04:19:34.9438405Z   --> /checkout/src/test/ui/parser/match-vec-invalid.rs:4:24
2019-07-10T04:19:34.9438595Z    |
2019-07-10T04:19:34.9438595Z    |
2019-07-10T04:19:34.9439044Z LL |         [1, tail @ .., tail @ ..] => {},
2019-07-10T04:19:34.9439378Z    |
2019-07-10T04:19:34.9439378Z    |
2019-07-10T04:19:34.9439867Z    = note: for more information, see ***/issues/62254
2019-07-10T04:19:34.9440078Z    = help: add `#![feature(slice_patterns)]` to the crate attributes to enable
2019-07-10T04:19:34.9447543Z 
2019-07-10T04:19:34.9447699Z error: `..` can only be used once per slice pattern
2019-07-10T04:19:34.9448362Z    |
2019-07-10T04:19:34.9448362Z    |
2019-07-10T04:19:34.9448674Z LL |         [1, tail @ .., tail @ ..] => {},
2019-07-10T04:19:34.9449174Z    |                    --         ^^ can only be used once per slice pattern
2019-07-10T04:19:34.9449349Z    |                    previously used here
2019-07-10T04:19:34.9449383Z 
2019-07-10T04:19:34.9449383Z 
2019-07-10T04:19:34.9449453Z error[E0529]: expected an array or slice, found `std::vec::Vec<_>`
2019-07-10T04:19:34.9449787Z    |
2019-07-10T04:19:34.9449787Z    |
2019-07-10T04:19:34.9449833Z LL |         [1, tail @ .., tail @ ..] => {},
2019-07-10T04:19:34.9449908Z    |         ^^^^^^^^^^^^^^^^^^^^^^^^^ pattern cannot match with input type `std::vec::Vec<_>`
2019-07-10T04:19:34.9449988Z error: aborting due to 5 previous errors
2019-07-10T04:19:34.9450037Z 
2019-07-10T04:19:34.9450085Z Some errors have detailed explanations: E0416, E0529, E0658.
2019-07-10T04:19:34.9450356Z For more information about an error, try `rustc --explain E0416`.
---
2019-07-10T04:19:34.9451004Z diff of stderr:
2019-07-10T04:19:34.9451033Z 
2019-07-10T04:19:34.9451077Z 23    |                    ^^
2019-07-10T04:19:34.9451121Z 24    |
2019-07-10T04:19:34.9451473Z 25    = note: for more information, see ***/issues/62254
2019-07-10T04:19:34.9451753Z -    = help: add #![feature(slice_patterns)] to the crate attributes to enable
2019-07-10T04:19:34.9451812Z +    = help: add `#![feature(slice_patterns)]` to the crate attributes to enable
2019-07-10T04:19:34.9451926Z 28 error: aborting due to 4 previous errors
2019-07-10T04:19:34.9451968Z 29 
2019-07-10T04:19:34.9452015Z 
2019-07-10T04:19:34.9452041Z 
2019-07-10T04:19:34.9452041Z 
2019-07-10T04:19:34.9452096Z The actual stderr differed from the expected stderr.
2019-07-10T04:19:34.9452415Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/pat-lt-bracket-6/pat-lt-bracket-6.stderr
2019-07-10T04:19:34.9452693Z To update references, rerun the tests and pass the `--bless` flag
2019-07-10T04:19:34.9453222Z To only update this specific test, also pass `--test-args parser/pat-lt-bracket-6.rs`
2019-07-10T04:19:34.9453337Z error: 1 errors occurred comparing output.
2019-07-10T04:19:34.9453383Z status: exit code: 1
2019-07-10T04:19:34.9453383Z status: exit code: 1
2019-07-10T04:19:34.9454158Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/pat-lt-bracket-6.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/pat-lt-bracket-6" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/pat-lt-bracket-6/auxiliary" "-A" "unused"
2019-07-10T04:19:34.9454509Z ------------------------------------------
2019-07-10T04:19:34.9454564Z 
2019-07-10T04:19:34.9454787Z ------------------------------------------
2019-07-10T04:19:34.9454832Z stderr:
2019-07-10T04:19:34.9454832Z stderr:
2019-07-10T04:19:34.9455045Z ------------------------------------------
2019-07-10T04:19:34.9455117Z error: expected one of `)`, `,`, or `@`, found `[`
2019-07-10T04:19:34.9455362Z   --> /checkout/src/test/ui/parser/pat-lt-bracket-6.rs:2:19
2019-07-10T04:19:34.9455411Z    |
2019-07-10T04:19:34.9455487Z LL |     let Test(&desc[..]) = x; //~ ERROR: expected one of `)`, `,`, or `@`, found `[`
2019-07-10T04:19:34.9455543Z    |                   ^ expected one of `)`, `,`, or `@` here
2019-07-10T04:19:34.9455639Z error[E0425]: cannot find value `x` in this scope
2019-07-10T04:19:34.9455884Z   --> /checkout/src/test/ui/parser/pat-lt-bracket-6.rs:2:27
2019-07-10T04:19:34.9456139Z    |
2019-07-10T04:19:34.9456139Z    |
2019-07-10T04:19:34.9456215Z LL |     let Test(&desc[..]) = x; //~ ERROR: expected one of `)`, `,`, or `@`, found `[`
2019-07-10T04:19:34.9456269Z    |                           ^ not found in this scope
2019-07-10T04:19:34.9456300Z 
2019-07-10T04:19:34.9456348Z error[E0531]: cannot find tuple struct/variant `Test` in this scope
2019-07-10T04:19:34.9456642Z   --> /checkout/src/test/ui/parser/pat-lt-bracket-6.rs:2:9
2019-07-10T04:19:34.9456691Z    |
2019-07-10T04:19:34.9456740Z LL |     let Test(&desc[..]) = x; //~ ERROR: expected one of `)`, `,`, or `@`, found `[`
2019-07-10T04:19:34.9456840Z 
2019-07-10T04:19:34.9456884Z error[E0658]: subslice patterns are unstable
2019-07-10T04:19:34.9457149Z   --> /checkout/src/test/ui/parser/pat-lt-bracket-6.rs:2:20
2019-07-10T04:19:34.9457197Z    |
2019-07-10T04:19:34.9457197Z    |
2019-07-10T04:19:34.9457246Z LL |     let Test(&desc[..]) = x; //~ ERROR: expected one of `)`, `,`, or `@`, found `[`
2019-07-10T04:19:34.9457373Z    |
2019-07-10T04:19:34.9457373Z    |
2019-07-10T04:19:34.9457680Z    = note: for more information, see ***/issues/62254
2019-07-10T04:19:34.9457760Z    = help: add `#![feature(slice_patterns)]` to the crate attributes to enable
2019-07-10T04:19:34.9457839Z error: aborting due to 4 previous errors
2019-07-10T04:19:34.9457869Z 
2019-07-10T04:19:34.9457932Z Some errors have detailed explanations: E0425, E0658.
2019-07-10T04:19:34.9458190Z For more information about an error, try `rustc --explain E0425`.
---
2019-07-10T04:19:34.9458796Z diff of stderr:
2019-07-10T04:19:34.9458823Z 
2019-07-10T04:19:34.9458884Z 23    |          ^^^^^^
2019-07-10T04:19:34.9458927Z 24    |
2019-07-10T04:19:34.9459257Z 25    = note: for more information, see ***/issues/37854
2019-07-10T04:19:34.9459542Z -    = help: add #![feature(exclusive_range_pattern)] to the crate attributes to enable
2019-07-10T04:19:34.9459603Z +    = help: add `#![feature(exclusive_range_pattern)]` to the crate attributes to enable
2019-07-10T04:19:34.9459667Z 27 
2019-07-10T04:19:34.9459718Z 28 error[E0029]: only char and numeric types are allowed in range patterns
2019-07-10T04:19:34.9459945Z 29   --> $DIR/pat-tuple-4.rs:3:10
2019-07-10T04:19:34.9460022Z 
2019-07-10T04:19:34.9460069Z The actual stderr differed from the expected stderr.
2019-07-10T04:19:34.9460069Z The actual stderr differed from the expected stderr.
2019-07-10T04:19:34.9460366Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/pat-tuple-4/pat-tuple-4.stderr
2019-07-10T04:19:34.9460637Z To update references, rerun the tests and pass the `--bless` flag
2019-07-10T04:19:34.9460949Z To only update this specific test, also pass `--test-args parser/pat-tuple-4.rs`
2019-07-10T04:19:34.9461057Z error: 1 errors occurred comparing output.
2019-07-10T04:19:34.9461128Z status: exit code: 1
2019-07-10T04:19:34.9461128Z status: exit code: 1
2019-07-10T04:19:34.9461875Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/pat-tuple-4.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/pat-tuple-4" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/pat-tuple-4/auxiliary" "-A" "unused"
2019-07-10T04:19:34.9462306Z ------------------------------------------
2019-07-10T04:19:34.9462342Z 
2019-07-10T04:19:34.9462596Z ------------------------------------------
2019-07-10T04:19:34.9462646Z stderr:
2019-07-10T04:19:34.9462646Z stderr:
2019-07-10T04:19:34.9462880Z ------------------------------------------
2019-07-10T04:19:34.9463724Z error: `..X` range patterns are not supported
2019-07-10T04:19:34.9464042Z   --> /checkout/src/test/ui/parser/pat-tuple-4.rs:3:10
2019-07-10T04:19:34.9464093Z    |
2019-07-10T04:19:34.9464136Z LL |         (.. pat) => {}
2019-07-10T04:19:34.9464208Z    |          ^^^^^^ help: try using the minimum value for the type: `MIN..pat`
2019-07-10T04:19:34.9464245Z 
2019-07-10T04:19:34.9464488Z error: arbitrary expressions aren't allowed in patterns
2019-07-10T04:19:34.9465126Z   --> /checkout/src/test/ui/parser/pat-tuple-4.rs:3:10
2019-07-10T04:19:34.9465181Z    |
2019-07-10T04:19:34.9465428Z LL |         (.. pat) => {}
2019-07-10T04:19:34.9516534Z 
2019-07-10T04:19:34.9516534Z 
2019-07-10T04:19:34.9516589Z error[E0425]: cannot find value `pat` in this scope
2019-07-10T04:19:34.9517122Z   --> /checkout/src/test/ui/parser/pat-tuple-4.rs:3:13
2019-07-10T04:19:34.9517178Z    |
2019-07-10T04:19:34.9517433Z LL |         (.. pat) => {}
2019-07-10T04:19:34.9517603Z 
2019-07-10T04:19:34.9517603Z 
2019-07-10T04:19:34.9517770Z error[E0658]: exclusive range pattern syntax is experimental
2019-07-10T04:19:34.9518129Z   --> /checkout/src/test/ui/parser/pat-tuple-4.rs:3:10
2019-07-10T04:19:34.9518181Z    |
2019-07-10T04:19:34.9518365Z LL |         (.. pat) => {}
2019-07-10T04:19:34.9518502Z    |
2019-07-10T04:19:34.9518502Z    |
2019-07-10T04:19:34.9518896Z    = note: for more information, see ***/issues/37854
2019-07-10T04:19:34.9519121Z    = help: add `#![feature(exclusive_range_pattern)]` to the crate attributes to enable
2019-07-10T04:19:34.9519224Z 
2019-07-10T04:19:34.9519276Z error[E0029]: only char and numeric types are allowed in range patterns
2019-07-10T04:19:34.9519623Z   --> /checkout/src/test/ui/parser/pat-tuple-4.rs:3:10
2019-07-10T04:19:34.9519673Z    |
2019-07-10T04:19:34.9519863Z LL |         (.. pat) => {}
2019-07-10T04:19:34.9519988Z    |          ^^^^^^ ranges require char or numeric types
2019-07-10T04:19:34.9520046Z    |
2019-07-10T04:19:34.9520099Z    = note: start type: [type error]
2019-07-10T04:19:34.9520164Z    = note: end type: [type error]
2019-07-10T04:19:34.9520239Z error: aborting due to 5 previous errors
2019-07-10T04:19:34.9520268Z 
2019-07-10T04:19:34.9520333Z Some errors have detailed explanations: E0029, E0425, E0658.
2019-07-10T04:19:34.9520632Z For more information about an error, try `rustc --explain E0029`.
---
2019-07-10T04:19:34.9521237Z diff of stderr:
2019-07-10T04:19:34.9521286Z 
2019-07-10T04:19:34.9521329Z 23    |          ^^^^^^
2019-07-10T04:19:34.9521372Z 24    |
2019-07-10T04:19:34.9521692Z 25    = note: for more information, see ***/issues/37854
2019-07-10T04:19:34.9521983Z -    = help: add #![feature(exclusive_range_pattern)] to the crate attributes to enable
2019-07-10T04:19:34.9522052Z +    = help: add `#![feature(exclusive_range_pattern)]` to the crate attributes to enable
2019-07-10T04:19:34.9522119Z 27 
2019-07-10T04:19:34.9522388Z 28 error[E0029]: only char and numeric types are allowed in range patterns
2019-07-10T04:19:34.9522686Z 29   --> $DIR/pat-tuple-5.rs:3:10
2019-07-10T04:19:34.9522897Z 
2019-07-10T04:19:34.9523469Z The actual stderr differed from the expected stderr.
2019-07-10T04:19:34.9523469Z The actual stderr differed from the expected stderr.
2019-07-10T04:19:34.9524012Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/pat-tuple-5/pat-tuple-5.stderr
2019-07-10T04:19:34.9524473Z To update references, rerun the tests and pass the `--bless` flag
2019-07-10T04:19:34.9524911Z To only update this specific test, also pass `--test-args parser/pat-tuple-5.rs`
2019-07-10T04:19:34.9525211Z error: 1 errors occurred comparing output.
2019-07-10T04:19:34.9525278Z status: exit code: 1
2019-07-10T04:19:34.9525278Z status: exit code: 1
2019-07-10T04:19:34.9526274Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/pat-tuple-5.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/pat-tuple-5" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/pat-tuple-5/auxiliary" "-A" "unused"
2019-07-10T04:19:34.9526731Z ------------------------------------------
2019-07-10T04:19:34.9526768Z 
2019-07-10T04:19:34.9527223Z ------------------------------------------
2019-07-10T04:19:34.9527427Z stderr:
2019-07-10T04:19:34.9527427Z stderr:
2019-07-10T04:19:34.9529265Z ------------------------------------------
2019-07-10T04:19:34.9529396Z error: `X..` range patterns are not supported
2019-07-10T04:19:34.9529864Z   --> /checkout/src/test/ui/parser/pat-tuple-5.rs:3:10
2019-07-10T04:19:34.9529944Z    |
2019-07-10T04:19:34.9530172Z LL |         (pat ..) => {}
2019-07-10T04:19:34.9530281Z    |          ^^^^^^ help: try using the maximum value for the type: `pat..MAX`
2019-07-10T04:19:34.9530339Z 
2019-07-10T04:19:34.9530628Z error: arbitrary expressions aren't allowed in patterns
2019-07-10T04:19:34.9531067Z   --> /checkout/src/test/ui/parser/pat-tuple-5.rs:3:10
2019-07-10T04:19:34.9531244Z    |
2019-07-10T04:19:34.9531336Z LL |         (pat ..) => {}
2019-07-10T04:19:34.9531439Z 
2019-07-10T04:19:34.9531439Z 
2019-07-10T04:19:34.9531565Z error[E0425]: cannot find value `pat` in this scope
2019-07-10T04:19:34.9531867Z   --> /checkout/src/test/ui/parser/pat-tuple-5.rs:3:10
2019-07-10T04:19:34.9531917Z    |
2019-07-10T04:19:34.9532099Z LL |         (pat ..) => {}
2019-07-10T04:19:34.9532227Z 
2019-07-10T04:19:34.9532227Z 
2019-07-10T04:19:34.9532301Z error[E0658]: exclusive range pattern syntax is experimental
2019-07-10T04:19:34.9532621Z   --> /checkout/src/test/ui/parser/pat-tuple-5.rs:3:10
2019-07-10T04:19:34.9532801Z    |
2019-07-10T04:19:34.9532880Z LL |         (pat ..) => {}
2019-07-10T04:19:34.9532991Z    |
2019-07-10T04:19:34.9532991Z    |
2019-07-10T04:19:34.9533666Z    = note: for more information, see ***/issues/37854
2019-07-10T04:19:34.9533932Z    = help: add `#![feature(exclusive_range_pattern)]` to the crate attributes to enable
2019-07-10T04:19:34.9533997Z 
2019-07-10T04:19:34.9534047Z error[E0029]: only char and numeric types are allowed in range patterns
2019-07-10T04:19:34.9534392Z   --> /checkout/src/test/ui/parser/pat-tuple-5.rs:3:10
2019-07-10T04:19:34.9534531Z    |
2019-07-10T04:19:34.9534597Z LL |         (pat ..) => {}
2019-07-10T04:19:34.9534721Z    |          ^^^^^^ ranges require char or numeric types
2019-07-10T04:19:34.9534786Z    |
2019-07-10T04:19:34.9534852Z    = note: start type: [type error]
2019-07-10T04:19:34.9534919Z    = note: end type: [type error]
2019-07-10T04:19:34.9535189Z error: aborting due to 5 previous errors
2019-07-10T04:19:34.9535239Z 
2019-07-10T04:19:34.9535314Z Some errors have detailed explanations: E0029, E0425, E0658.
2019-07-10T04:19:34.9535620Z For more information about an error, try `rustc --explain E0029`.
---
2019-07-10T04:19:34.9539066Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:535:22
2019-07-10T04:19:34.9539396Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-07-10T04:19:34.9539682Z 
2019-07-10T04:19:34.9539711Z 
2019-07-10T04:19:34.9541333Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-07-10T04:19:34.9541820Z 
2019-07-10T04:19:34.9541873Z 
2019-07-10T04:19:34.9541949Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-07-10T04:19:34.9542020Z Build completed unsuccessfully in 0:52:18
2019-07-10T04:19:34.9542020Z Build completed unsuccessfully in 0:52:18
2019-07-10T04:19:36.0219137Z ##[error]Bash exited with code '1'.
2019-07-10T04:19:36.0251485Z ##[section]Starting: Checkout
2019-07-10T04:19:36.0253813Z ==============================================================================
2019-07-10T04:19:36.0253872Z Task         : Get sources
2019-07-10T04:19:36.0253921Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
