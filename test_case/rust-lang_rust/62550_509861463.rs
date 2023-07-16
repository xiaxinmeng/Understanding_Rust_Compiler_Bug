plain
2019-07-10T00:02:45.6211064Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-10T00:02:45.6384918Z ##[command]git config gc.auto 0
2019-07-10T00:02:45.6444342Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-10T00:02:45.6513019Z ##[command]git config --get-all http.proxy
2019-07-10T00:02:45.6618418Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62550/merge:refs/remotes/pull/62550/merge
---
2019-07-10T00:03:20.9411812Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-10T00:03:20.9411841Z 
2019-07-10T00:03:20.9412056Z   git checkout -b <new-branch-name>
2019-07-10T00:03:20.9412085Z 
2019-07-10T00:03:20.9412139Z HEAD is now at d2c81d8f5 Merge 393f88909d7194f60ca1380594880903c9040998 into 0b680cfce544ff9a59d720020e397c4bf3346650
2019-07-10T00:03:20.9542820Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-10T00:03:20.9546152Z ==============================================================================
2019-07-10T00:03:20.9546213Z Task         : Bash
2019-07-10T00:03:20.9546363Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-10T00:04:56.7935840Z Attempting with retry: curl -y 30 -Y 10 --connect-timeout 30 -f -L -C - -o /tmp/rustci_docker_cache https://.s3.amazonaws.com/docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624
2019-07-10T00:04:56.7984865Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-10T00:04:56.7985303Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-10T00:04:56.7985348Z 
2019-07-10T00:04:56.8028102Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-10T00:04:57.8092674Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-10T00:04:57.8092831Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-10T00:04:57.8092865Z 
2019-07-10T00:04:57.8092865Z 
2019-07-10T00:04:57.8136054Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-10T00:04:59.8211072Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-10T00:04:59.8211288Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-10T00:04:59.8211493Z 
2019-07-10T00:04:59.8211493Z 
2019-07-10T00:04:59.8244204Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-10T00:05:02.8307213Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-10T00:05:02.8314824Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-10T00:05:02.8315572Z 
2019-07-10T00:05:02.8315572Z 
2019-07-10T00:05:02.8354415Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-10T00:05:06.8438414Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-10T00:05:06.8439044Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-10T00:05:06.8439274Z 
2019-07-10T00:05:06.8439274Z 
2019-07-10T00:05:06.8440184Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-10T00:05:06.8443527Z The command has failed after 5 attempts.
2019-07-10T00:05:06.8960167Z open /tmp/rustci_docker_cache: no such file or directory
2019-07-10T00:05:06.8983769Z Attempting with retry: docker build --rm -t rust-ci -f /home/vsts/work/1/s/src/ci/docker/x86_64-gnu-llvm-6.0/Dockerfile /home/vsts/work/1/s/src/ci/docker
2019-07-10T00:05:07.0762627Z Sending build context to Docker daemon  521.7kB
2019-07-10T00:05:07.0762724Z 
2019-07-10T00:05:07.0947565Z Step 1/8 : FROM ubuntu:16.04
---
2019-07-10T00:05:23.2304300Z Reading package lists...
2019-07-10T00:05:24.1068713Z Reading package lists...
2019-07-10T00:05:24.2606360Z Building dependency tree...
2019-07-10T00:05:24.2606463Z Reading state information...
2019-07-10T00:05:24.3685281Z The following additional packages will be installed:
2019-07-10T00:05:24.3686004Z   binfmt-support binutils bzip2 cmake-data cpp cpp-5 dpkg-dev g++-5 gcc gcc-5
2019-07-10T00:05:24.3686241Z   git-man libarchive13 libasan2 libasn1-8-heimdal libatomic1
2019-07-10T00:05:24.3686835Z   libbabeltrace-ctf1 libbabeltrace1 libbsd-dev libbsd0 libbz2-1.0 libc-dev-bin
2019-07-10T00:05:24.3687990Z   libedit2 liberror-perl libexpat1 libffi-dev libffi6 libgcc-5-dev libgdbm3
2019-07-10T00:05:24.3688294Z   libglib2.0-0 libgmp10 libgnutls30 libgomp1 libgssapi-krb5-2
2019-07-10T00:05:24.3688549Z   libgssapi3-heimdal libhcrypto4-heimdal libheimbase1-heimdal
2019-07-10T00:05:24.3688828Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-10T00:05:24.3688828Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-10T00:05:24.3689140Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-10T00:05:24.3689424Z   libkrb5-3 libkrb5support0 libldap-2.4-2 libllvm6.0 liblsan0 liblzo2-2
2019-07-10T00:05:24.3689678Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-10T00:05:24.3689970Z   libperl5.22 libpipeline1 libpython-stdlib libpython2.7-minimal
2019-07-10T00:05:24.3690254Z   libpython2.7-stdlib libpython3.5 libpython3.5-minimal libpython3.5-stdlib
2019-07-10T00:05:24.3690511Z   libquadmath0 libroken18-heimdal librtmp1 libsasl2-2 libsasl2-modules-db
2019-07-10T00:05:24.3691016Z   libsqlite3-0 libssl1.0.0 libstdc++-5-dev libtasn1-6 libtinfo-dev libtsan0
2019-07-10T00:05:24.3691247Z   libubsan0 libwind0-heimdal libxml2 linux-libc-dev llvm-6.0 llvm-6.0-dev
2019-07-10T00:05:24.3691469Z   llvm-6.0-runtime mime-support openssl patch perl perl-modules-5.22 python
2019-07-10T00:05:24.3691713Z   python-minimal python2.7-minimal
2019-07-10T00:05:24.3691760Z Suggested packages:
2019-07-10T00:05:24.3691991Z   binutils-doc bzip2-doc codeblocks eclipse ninja-build cpp-doc gcc-5-locales
2019-07-10T00:05:24.3692211Z   debian-keyring g++-multilib g++-5-multilib gcc-5-doc libstdc++6-5-dbg
2019-07-10T00:05:24.3692477Z   gcc-multilib manpages-dev autoconf automake libtool flex bison gcc-doc
2019-07-10T00:05:24.3692928Z   libasan2-dbg liblsan0-dbg libtsan0-dbg libubsan0-dbg libcilkrts5-dbg
2019-07-10T00:05:24.3693190Z   libmpx0-dbg libquadmath0-dbg gdb-doc gettext-base git-daemon-run
2019-07-10T00:05:24.3693190Z   libmpx0-dbg libquadmath0-dbg gdb-doc gettext-base git-daemon-run
2019-07-10T00:05:24.3693413Z   | git-daemon-sysvinit git-doc git-el git-email git-gui gitk gitweb git-arch
2019-07-10T00:05:24.3693637Z   git-cvs git-mediawiki git-svn lrzip glibc-doc gnutls-bin krb5-doc krb5-user
2019-07-10T00:05:24.3693899Z   libstdc++-5-doc llvm-6.0-doc make-doc ed diffutils-doc perl-doc
2019-07-10T00:05:24.3694130Z   libterm-readline-gnu-perl | libterm-readline-perl-perl python-doc python-tk
2019-07-10T00:05:24.3694308Z   python2.7-doc
2019-07-10T00:05:24.3694389Z Recommended packages:
2019-07-10T00:05:24.3694611Z   build-essential fakeroot libalgorithm-merge-perl libc-dbg gdbserver less
2019-07-10T00:05:24.3694824Z   rsync ssh-client manpages manpages-dev libfile-fcntllock-perl
2019-07-10T00:05:24.3695095Z   libglib2.0-data shared-mime-info xdg-user-dirs krb5-locales libsasl2-modules
2019-07-10T00:05:24.3695300Z   libssl-doc xml-core netbase rename
2019-07-10T00:05:24.3695346Z The following NEW packages will be installed:
2019-07-10T00:05:24.3695607Z   binfmt-support binutils bzip2 ca-certificates cmake cmake-data cpp cpp-5
2019-07-10T00:05:24.3695839Z   curl dpkg-dev file g++ g++-5 gcc gcc-5 gdb git git-man libarchive13 libasan2
2019-07-10T00:05:24.3696064Z   libasn1-8-heimdal libatomic1 libbabeltrace-ctf1 libbabeltrace1 libbsd-dev
2019-07-10T00:05:24.3696457Z   libbsd0 libc-dev-bin libc6-dev libcc1-0 libcilkrts5 libcurl3 libcurl3-gnutls
2019-07-10T00:05:24.3696742Z   libdpkg-perl libedit-dev libedit2 liberror-perl libexpat1 libffi-dev libffi6
2019-07-10T00:05:24.3697650Z   libgcc-5-dev libgdbm3 libglib2.0-0 libgmp10 libgnutls30 libgomp1
2019-07-10T00:05:24.3698273Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-10T00:05:24.3698532Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-10T00:05:24.3699334Z   libkrb5-3 libkrb5support0 libldap-2.4-2 libllvm6.0 liblsan0 liblzo2-2
2019-07-10T00:05:24.3699596Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-10T00:05:24.3699596Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-10T00:05:24.3699846Z   libperl5.22 libpipeline1 libpython-stdlib libpython2.7-minimal
2019-07-10T00:05:24.3700154Z   libpython2.7-stdlib libpython3.5 libpython3.5-minimal libpython3.5-stdlib
2019-07-10T00:05:24.3700433Z   libquadmath0 libroken18-heimdal librtmp1 libsasl2-2 libsasl2-modules-db
2019-07-10T00:05:24.3701034Z   libsqlite3-0 libssl-dev libssl1.0.0 libstdc++-5-dev libtasn1-6 libtinfo-dev
2019-07-10T00:05:24.3701302Z   libtsan0 libubsan0 libwind0-heimdal libxml2 linux-libc-dev llvm-6.0
2019-07-10T00:05:24.3701536Z   llvm-6.0-dev llvm-6.0-runtime llvm-6.0-tools make mime-support openssl patch
2019-07-10T00:05:24.3701754Z   perl perl-modules-5.22 pkg-config python python-minimal python2.7
2019-07-10T00:05:24.3701997Z   python2.7-minimal sudo xz-utils zlib1g-dev
2019-07-10T00:05:24.3702054Z The following packages will be upgraded:
2019-07-10T00:05:24.6302871Z 1 upgraded, 115 newly installed, 0 to remove and 4 not upgraded.
2019-07-10T00:05:24.6303036Z Need to get 121 MB of archives.
2019-07-10T00:05:24.6304119Z After this operation, 592 MB of additional disk space will be used.
2019-07-10T00:05:24.6305141Z Get:1 http://archive.ubuntu.com/ubuntu xenial/main amd64 libgdbm3 amd64 1.8.3-13.1 [16.9 kB]
---
2019-07-10T00:05:25.7577991Z Get:61 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 sudo amd64 1.8.16-0ubuntu1.7 [390 kB]
2019-07-10T00:05:25.7720341Z Get:62 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 openssl amd64 1.0.2g-1ubuntu4.15 [492 kB]
2019-07-10T00:05:25.7789436Z Get:63 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 ca-certificates all 20170717~16.04.2 [167 kB]
2019-07-10T00:05:25.7814082Z Get:64 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 libcurl3-gnutls amd64 7.47.0-1ubuntu2.13 [184 kB]
2019-07-10T00:05:25.7839378Z Get:65 http://archive.ubuntu.com/ubuntu xenial/main amd64 libedit2 amd64 3.1-20150325-1ubuntu2 [76.5 kB]
2019-07-10T00:05:25.7851301Z Get:66 http://archive.ubuntu.com/ubuntu xenial/main amd64 libpipeline1 amd64 1.4.1-2 [24.6 kB]
2019-07-10T00:05:25.7859160Z Get:67 http://archive.ubuntu.com/ubuntu xenial/main amd64 binfmt-support amd64 2.1.6-1 [50.7 kB]
2019-07-10T00:05:25.8475901Z Get:69 http://archive.ubuntu.com/ubuntu xenial/main amd64 libisl15 amd64 0.16.1-1 [524 kB]
2019-07-10T00:05:25.8540592Z Get:70 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 cpp-5 amd64 5.4.0-6ubuntu1~16.04.11 [7660 kB]
2019-07-10T00:05:26.0110745Z Get:71 http://archive.ubuntu.com/ubuntu xenial/main amd64 cpp amd64 4:5.3.1-1ubuntu1 [27.7 kB]
2019-07-10T00:05:26.0119693Z Get:72 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 curl amd64 7.47.0-1ubuntu2.13 [139 kB]
---
2019-07-10T00:05:43.5883369Z Unpacking ca-certificates (20170717~16.04.2) ...
2019-07-10T00:05:43.7557494Z Selecting previously unselected package libcurl3-gnutls:amd64.
2019-07-10T00:05:43.7574915Z Preparing to unpack .../libcurl3-gnutls_7.47.0-1ubuntu2.13_amd64.deb ...
2019-07-10T00:05:43.7704993Z Unpacking libcurl3-gnutls:amd64 (7.47.0-1ubuntu2.13) ...
2019-07-10T00:05:43.9024060Z Selecting previously unselected package libedit2:amd64.
2019-07-10T00:05:43.9041067Z Preparing to unpack .../libedit2_3.1-20150325-1ubuntu2_amd64.deb ...
2019-07-10T00:05:43.9180348Z Unpacking libedit2:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-10T00:05:44.0221141Z Selecting previously unselected package libpipeline1:amd64.
2019-07-10T00:05:44.0236985Z Preparing to unpack .../libpipeline1_1.4.1-2_amd64.deb ...
2019-07-10T00:05:44.0370821Z Unpacking libpipeline1:amd64 (1.4.1-2) ...
2019-07-10T00:05:44.1450718Z Selecting previously unselected package binfmt-support.
2019-07-10T00:05:44.1468395Z Preparing to unpack .../binfmt-support_2.1.6-1_amd64.deb ...
2019-07-10T00:05:44.1634091Z Unpacking binfmt-support (2.1.6-1) ...
2019-07-10T00:05:44.2767533Z Preparing to unpack .../binutils_2.26.1-1ubuntu1~16.04.8_amd64.deb ...
2019-07-10T00:05:44.2897380Z Unpacking binutils (2.26.1-1ubuntu1~16.04.8) ...
2019-07-10T00:05:44.7454702Z Selecting previously unselected package libisl15:amd64.
2019-07-10T00:05:44.7471555Z Preparing to unpack .../libisl15_0.16.1-1_amd64.deb ...
---
2019-07-10T00:05:55.9738198Z Selecting previously unselected package libtinfo-dev:amd64.
2019-07-10T00:05:55.9754452Z Preparing to unpack .../libtinfo-dev_6.0+20160213-1ubuntu1_amd64.deb ...
2019-07-10T00:05:55.9891545Z Unpacking libtinfo-dev:amd64 (6.0+20160213-1ubuntu1) ...
2019-07-10T00:05:56.0879696Z Selecting previously unselected package libedit-dev:amd64.
2019-07-10T00:05:56.0897120Z Preparing to unpack .../libedit-dev_3.1-20150325-1ubuntu2_amd64.deb ...
2019-07-10T00:05:56.1032878Z Unpacking libedit-dev:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-10T00:05:56.2167615Z Selecting previously unselected package libllvm6.0:amd64.
2019-07-10T00:05:56.2187700Z Preparing to unpack .../libllvm6.0_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-10T00:05:56.2326878Z Unpacking libllvm6.0:amd64 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-10T00:05:59.0759930Z Selecting previously unselected package zlib1g-dev:amd64.
2019-07-10T00:05:59.0775508Z Preparing to unpack .../zlib1g-dev_1%3a1.2.8.dfsg-2ubuntu4.1_amd64.deb ...
2019-07-10T00:05:59.0908057Z Unpacking zlib1g-dev:amd64 (1:1.2.8.dfsg-2ubuntu4.1) ...
2019-07-10T00:05:59.1976470Z Preparing to unpack .../libssl-dev_1.0.2g-1ubuntu4.15_amd64.deb ...
2019-07-10T00:05:59.2180635Z Unpacking libssl-dev:amd64 (1.0.2g-1ubuntu4.15) ...
2019-07-10T00:05:59.5328035Z Selecting previously unselected package llvm-6.0-runtime.
2019-07-10T00:05:59.5328035Z Selecting previously unselected package llvm-6.0-runtime.
2019-07-10T00:05:59.5349113Z Preparing to unpack .../llvm-6.0-runtime_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-10T00:05:59.5494730Z Unpacking llvm-6.0-runtime (1:6.0-1ubuntu2~16.04.1) ...
2019-07-10T00:05:59.6688791Z Selecting previously unselected package llvm-6.0.
2019-07-10T00:05:59.6709526Z Preparing to unpack .../llvm-6.0_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-10T00:05:59.6870224Z Unpacking llvm-6.0 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-10T00:06:00.3214756Z Selecting previously unselected package libffi-dev:amd64.
2019-07-10T00:06:00.3232844Z Preparing to unpack .../libffi-dev_3.2.1-4_amd64.deb ...
2019-07-10T00:06:00.3366325Z Unpacking libffi-dev:amd64 (3.2.1-4) ...
2019-07-10T00:06:00.4548654Z Selecting previously unselected package llvm-6.0-dev.
2019-07-10T00:06:00.4567698Z Preparing to unpack .../llvm-6.0-dev_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-10T00:06:00.4692520Z Unpacking llvm-6.0-dev (1:6.0-1ubuntu2~16.04.1) ...
2019-07-10T00:06:05.0327888Z Selecting previously unselected package llvm-6.0-tools.
2019-07-10T00:06:05.0349335Z Preparing to unpack .../llvm-6.0-tools_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-10T00:06:05.0490770Z Unpacking llvm-6.0-tools (1:6.0-1ubuntu2~16.04.1) ...
2019-07-10T00:06:05.2236222Z Selecting previously unselected package pkg-config.
2019-07-10T00:06:05.2256617Z Preparing to unpack .../pkg-config_0.29.1-0ubuntu1_amd64.deb ...
2019-07-10T00:06:05.2385769Z Unpacking pkg-config (0.29.1-0ubuntu1) ...
2019-07-10T00:06:05.3522629Z Processing triggers for systemd (229-4ubuntu21.21) ...
2019-07-10T00:06:05.7215950Z Setting up libgdbm3:amd64 (1.8.3-13.1) ...
2019-07-10T00:06:05.7936462Z Setting up libffi6:amd64 (3.2.1-4) ...
2019-07-10T00:06:05.8326960Z Setting up libglib2.0-0:amd64 (2.48.2-0ubuntu4.3) ...
---
2019-07-10T00:06:09.4785213Z debconf: unable to initialize frontend: Dialog
2019-07-10T00:06:09.4785291Z debconf: (TERM is not set, so the dialog frontend is not usable.)
2019-07-10T00:06:09.4785419Z debconf: falling back to frontend: Readline
2019-07-10T00:06:09.9638259Z Setting up libcurl3-gnutls:amd64 (7.47.0-1ubuntu2.13) ...
2019-07-10T00:06:10.0044728Z Setting up libedit2:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-10T00:06:10.0443638Z Setting up libpipeline1:amd64 (1.4.1-2) ...
2019-07-10T00:06:10.0834692Z Setting up binfmt-support (2.1.6-1) ...
2019-07-10T00:06:10.1625618Z mount: permission denied
2019-07-10T00:06:10.1626305Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-10T00:06:10.1640349Z mount: permission denied
2019-07-10T00:06:10.1641053Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-10T00:06:10.3156660Z invoke-rc.d: could not determine current runlevel
2019-07-10T00:06:10.3191071Z invoke-rc.d: policy-rc.d denied execution of start.
2019-07-10T00:06:10.3800645Z Setting up libisl15:amd64 (0.16.1-1) ...
2019-07-10T00:06:10.4206456Z Setting up cpp-5 (5.4.0-6ubuntu1~16.04.11) ...
2019-07-10T00:06:10.4624902Z Setting up cpp (4:5.3.1-1ubuntu1) ...
2019-07-10T00:06:10.5248281Z Setting up curl (7.47.0-1ubuntu2.13) ...
---
2019-07-10T00:06:12.2249340Z Setting up libedit-dev:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-10T00:06:12.2639575Z Setting up libllvm6.0:amd64 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-10T00:06:12.3035477Z Setting up zlib1g-dev:amd64 (1:1.2.8.dfsg-2ubuntu4.1) ...
2019-07-10T00:06:12.3449610Z Setting up libssl-dev:amd64 (1.0.2g-1ubuntu4.15) ...
2019-07-10T00:06:12.3865824Z Setting up llvm-6.0-runtime (1:6.0-1ubuntu2~16.04.1) ...
2019-07-10T00:06:12.4185913Z mount: permission denied
2019-07-10T00:06:12.4190168Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-10T00:06:12.4337748Z Setting up llvm-6.0 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-10T00:06:12.4756217Z Setting up libffi-dev:amd64 (3.2.1-4) ...
2019-07-10T00:06:12.5175727Z Setting up llvm-6.0-dev (1:6.0-1ubuntu2~16.04.1) ...
2019-07-10T00:06:12.5589242Z Setting up llvm-6.0-tools (1:6.0-1ubuntu2~16.04.1) ...
2019-07-10T00:06:12.5994464Z Setting up pkg-config (0.29.1-0ubuntu1) ...
2019-07-10T00:06:12.7290659Z Processing triggers for ca-certificates (20170717~16.04.2) ...
2019-07-10T00:06:12.7449599Z Updating certificates in /etc/ssl/certs...
2019-07-10T00:06:14.1804598Z 148 added, 0 removed; done.
2019-07-10T00:06:14.1805581Z Running hooks in /etc/ca-certificates/update.d...
---
2019-07-10T00:06:45.2444410Z Removing intermediate container 4883e1394033
2019-07-10T00:06:45.2445287Z  ---> 9233af563490
2019-07-10T00:06:45.2487324Z Successfully built 9233af563490
2019-07-10T00:06:45.4214950Z Successfully tagged rust-ci:latest
2019-07-10T00:06:45.4675075Z Built container sha256:9233af5634905f153b92fee2d09304288456734ce114429ee41f763b81f2b932
2019-07-10T00:06:45.4694436Z Uploading finished image to https://.s3.amazonaws.com/docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624
2019-07-10T00:07:44.4991502Z upload failed: - to s3:///docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624 Parameter validation failed:
2019-07-10T00:07:44.4993998Z Invalid bucket name "": Bucket name must match the regex "^[a-zA-Z0-9.\-_]{1,255}$"
2019-07-10T00:07:45.4704334Z [CI_JOB_NAME=Job1]
2019-07-10T00:07:45.4751864Z Starting sccache server...
2019-07-10T00:07:45.5201605Z configure: processing command line
2019-07-10T00:07:45.5203361Z configure: 
---
2019-07-10T00:36:28.4764448Z    Compiling synstructure v0.10.2
2019-07-10T00:36:52.3749246Z    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
2019-07-10T00:37:02.6963674Z    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
2019-07-10T00:37:07.6821553Z    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2019-07-10T00:37:07.7143742Z error: expected one of `,`, `@`, or `]`, found `..`
2019-07-10T00:37:07.7157327Z     |
2019-07-10T00:37:07.7157327Z     |
2019-07-10T00:37:07.7163284Z 113 |                 ["a", ref a..] => dl.aggregate_align = align(a, "a")?,
2019-07-10T00:37:07.7170145Z     |                            ^^ expected one of `,`, `@`, or `]` here
2019-07-10T00:37:07.7174132Z 
2019-07-10T00:37:07.7181588Z error: expected one of `,`, `@`, or `]`, found `..`
2019-07-10T00:37:07.7263825Z     |
2019-07-10T00:37:07.7263825Z     |
2019-07-10T00:37:07.7264129Z 114 |                 ["f32", ref a..] => dl.f32_align = align(a, "f32")?,
2019-07-10T00:37:07.7266014Z     |                              ^^ expected one of `,`, `@`, or `]` here
2019-07-10T00:37:07.7266073Z 
2019-07-10T00:37:07.7266387Z error: expected one of `,`, `@`, or `]`, found `..`
2019-07-10T00:37:07.7266930Z     |
2019-07-10T00:37:07.7266930Z     |
2019-07-10T00:37:07.7267244Z 115 |                 ["f64", ref a..] => dl.f64_align = align(a, "f64")?,
2019-07-10T00:37:07.7267611Z     |                              ^^ expected one of `,`, `@`, or `]` here
2019-07-10T00:37:07.7267653Z 
2019-07-10T00:37:07.7267923Z error: expected one of `,`, `@`, or `]`, found `..`
2019-07-10T00:37:07.7268430Z     |
2019-07-10T00:37:07.7268430Z     |
2019-07-10T00:37:07.7268740Z 116 |                 [p @ "p", s, ref a..] | [p @ "p0", s, ref a..] => {
2019-07-10T00:37:07.7269234Z     |                                   ^^ expected one of `,`, `@`, or `]` here
2019-07-10T00:37:07.7269273Z 
2019-07-10T00:37:07.7269516Z error: expected one of `,`, `@`, or `]`, found `..`
2019-07-10T00:37:07.7269992Z     |
2019-07-10T00:37:07.7269992Z     |
2019-07-10T00:37:07.7270281Z 116 |                 [p @ "p", s, ref a..] | [p @ "p0", s, ref a..] => {
2019-07-10T00:37:07.7270655Z     |                                                            ^^ expected one of `,`, `@`, or `]` here
2019-07-10T00:37:07.7270700Z 
2019-07-10T00:37:07.7270944Z error: expected one of `,`, `@`, or `]`, found `..`
2019-07-10T00:37:07.7271484Z     |
2019-07-10T00:37:07.7271484Z     |
2019-07-10T00:37:07.7271765Z 120 |                 [s, ref a..] if s.starts_with("i") => {
2019-07-10T00:37:07.7272107Z     |                          ^^ expected one of `,`, `@`, or `]` here
2019-07-10T00:37:07.7272150Z 
2019-07-10T00:37:07.7272402Z error: expected one of `,`, `@`, or `]`, found `..`
2019-07-10T00:37:07.7272879Z     |
2019-07-10T00:37:07.7272879Z     |
2019-07-10T00:37:07.7273172Z 144 |                 [s, ref a..] if s.starts_with("v") => {
2019-07-10T00:37:07.7273621Z     |                          ^^ expected one of `,`, `@`, or `]` here
2019-07-10T00:37:08.2715142Z error[E0308]: mismatched types
2019-07-10T00:37:08.2720892Z    --> src/librustc_target/abi/mod.rs:113:62
2019-07-10T00:37:08.2724243Z     |
2019-07-10T00:37:08.2724243Z     |
2019-07-10T00:37:08.2728807Z 113 |                 ["a", ref a..] => dl.aggregate_align = align(a, "a")?,
2019-07-10T00:37:08.2732233Z     |                                                              ^ expected slice, found &str
2019-07-10T00:37:08.2740995Z     = note: expected type `&[&str]`
2019-07-10T00:37:08.2740995Z     = note: expected type `&[&str]`
2019-07-10T00:37:08.2746779Z                found type `&&str`
2019-07-10T00:37:08.3010912Z error[E0308]: mismatched types
2019-07-10T00:37:08.3014431Z    --> src/librustc_target/abi/mod.rs:114:58
2019-07-10T00:37:08.3018262Z     |
2019-07-10T00:37:08.3018262Z     |
2019-07-10T00:37:08.3022029Z 114 |                 ["f32", ref a..] => dl.f32_align = align(a, "f32")?,
2019-07-10T00:37:08.3061308Z     |                                                          ^ expected slice, found &str
2019-07-10T00:37:08.3061931Z     = note: expected type `&[&str]`
2019-07-10T00:37:08.3061931Z     = note: expected type `&[&str]`
2019-07-10T00:37:08.3062186Z                found type `&&str`
2019-07-10T00:37:08.3320633Z error[E0308]: mismatched types
2019-07-10T00:37:08.3324723Z    --> src/librustc_target/abi/mod.rs:115:58
2019-07-10T00:37:08.3325184Z     |
2019-07-10T00:37:08.3325184Z     |
2019-07-10T00:37:08.3328724Z 115 |                 ["f64", ref a..] => dl.f64_align = align(a, "f64")?,
2019-07-10T00:37:08.3329197Z     |                                                          ^ expected slice, found &str
2019-07-10T00:37:08.3329790Z     = note: expected type `&[&str]`
2019-07-10T00:37:08.3329790Z     = note: expected type `&[&str]`
2019-07-10T00:37:08.3330039Z                found type `&&str`
2019-07-10T00:37:08.3610655Z error[E0308]: mismatched types
2019-07-10T00:37:08.3610944Z    --> src/librustc_target/abi/mod.rs:118:46
2019-07-10T00:37:08.3612015Z     |
2019-07-10T00:37:08.3612015Z     |
2019-07-10T00:37:08.3613609Z 118 |                     dl.pointer_align = align(a, p)?;
2019-07-10T00:37:08.3616680Z     |                                              ^ expected slice, found &str
2019-07-10T00:37:08.3618409Z     = note: expected type `&[&str]`
2019-07-10T00:37:08.3618409Z     = note: expected type `&[&str]`
2019-07-10T00:37:08.3620057Z                found type `&&str`
2019-07-10T00:37:08.3885117Z error[E0308]: mismatched types
2019-07-10T00:37:08.3891192Z    --> src/librustc_target/abi/mod.rs:128:35
2019-07-10T00:37:08.3891618Z     |
2019-07-10T00:37:08.3891618Z     |
2019-07-10T00:37:08.3891902Z 128 |                     let a = align(a, s)?;
2019-07-10T00:37:08.3892249Z     |                                   ^ expected slice, found &str
2019-07-10T00:37:08.3892991Z     = note: expected type `&[&str]`
2019-07-10T00:37:08.3892991Z     = note: expected type `&[&str]`
2019-07-10T00:37:08.3893228Z                found type `&&str`
2019-07-10T00:37:08.4157156Z error[E0308]: mismatched types
2019-07-10T00:37:08.4160635Z    --> src/librustc_target/abi/mod.rs:146:35
2019-07-10T00:37:08.4160922Z     |
2019-07-10T00:37:08.4160922Z     |
2019-07-10T00:37:08.4161210Z 146 |                     let a = align(a, s)?;
2019-07-10T00:37:08.4161557Z     |                                   ^ expected slice, found &str
2019-07-10T00:37:08.4162103Z     = note: expected type `&[&str]`
2019-07-10T00:37:08.4162103Z     = note: expected type `&[&str]`
2019-07-10T00:37:08.4162379Z                found type `&&str`
2019-07-10T00:37:09.5391905Z error: aborting due to 13 previous errors
2019-07-10T00:37:09.5391998Z 
2019-07-10T00:37:09.5392280Z For more information about this error, try `rustc --explain E0308`.
2019-07-10T00:37:09.5638213Z error: Could not compile `rustc_target`.
2019-07-10T00:37:09.5638213Z error: Could not compile `rustc_target`.
2019-07-10T00:37:09.5638661Z warning: build failed, waiting for other jobs to finish...
2019-07-10T00:37:17.7741111Z error: build failed
2019-07-10T00:37:17.7762014Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
2019-07-10T00:37:17.7762227Z expected success, got: exit code: 101
2019-07-10T00:37:17.7772360Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-07-10T00:37:17.7772616Z Build completed unsuccessfully in 0:25:57
2019-07-10T00:37:18.6227993Z ##[error]Bash exited with code '1'.
2019-07-10T00:37:18.6256004Z ##[section]Starting: Checkout
2019-07-10T00:37:18.6258090Z ==============================================================================
2019-07-10T00:37:18.6258166Z Task         : Get sources
2019-07-10T00:37:18.6258214Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
