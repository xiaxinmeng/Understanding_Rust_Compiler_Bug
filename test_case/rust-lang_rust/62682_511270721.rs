plain
2019-07-15T04:04:07.2736543Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-15T04:04:08.0484787Z ##[command]git config gc.auto 0
2019-07-15T04:04:08.0487435Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-15T04:04:08.0489379Z ##[command]git config --get-all http.proxy
2019-07-15T04:04:08.0492945Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62682/merge:refs/remotes/pull/62682/merge
---
2019-07-15T04:04:41.9177574Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-15T04:04:41.9177606Z 
2019-07-15T04:04:41.9177814Z   git checkout -b <new-branch-name>
2019-07-15T04:04:41.9177844Z 
2019-07-15T04:04:41.9177914Z HEAD is now at 5cf6be3a8 Merge 2b038f2a2c6b72f5c95885e10e7db76143a7110a into d82fd9ecd3e65a313b0e0bdd24de127d4b566156
2019-07-15T04:04:41.9314724Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-15T04:04:41.9317665Z ==============================================================================
2019-07-15T04:04:41.9317727Z Task         : Bash
2019-07-15T04:04:41.9317778Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-15T04:06:34.5197445Z Attempting with retry: curl -y 30 -Y 10 --connect-timeout 30 -f -L -C - -o /tmp/rustci_docker_cache https://.s3.amazonaws.com/docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624
2019-07-15T04:06:34.5247955Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-15T04:06:34.5248441Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-15T04:06:34.5248517Z 
2019-07-15T04:06:34.5290839Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-15T04:06:35.5359500Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-15T04:06:35.5359905Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-15T04:06:35.5360118Z 
2019-07-15T04:06:35.5360118Z 
2019-07-15T04:06:35.5403109Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-15T04:06:37.5471803Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-15T04:06:37.5472390Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-15T04:06:37.5472680Z 
2019-07-15T04:06:37.5472680Z 
2019-07-15T04:06:37.5514590Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-15T04:06:40.5586102Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-15T04:06:40.5595314Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-15T04:06:40.5595422Z 
2019-07-15T04:06:40.5595422Z 
2019-07-15T04:06:40.5628833Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-15T04:06:44.5696387Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-15T04:06:44.5696947Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-15T04:06:44.5697030Z 
2019-07-15T04:06:44.5697030Z 
2019-07-15T04:06:44.5739793Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-15T04:06:44.5744540Z The command has failed after 5 attempts.
2019-07-15T04:06:44.6102939Z open /tmp/rustci_docker_cache: no such file or directory
2019-07-15T04:06:44.6138759Z Attempting with retry: docker build --rm -t rust-ci -f /home/vsts/work/1/s/src/ci/docker/x86_64-gnu-llvm-6.0/Dockerfile /home/vsts/work/1/s/src/ci/docker
2019-07-15T04:06:44.7856138Z Sending build context to Docker daemon  521.7kB
2019-07-15T04:06:44.7856357Z 
2019-07-15T04:06:44.7999808Z Step 1/8 : FROM ubuntu:16.04
---
2019-07-15T04:07:02.0404830Z Reading package lists...
2019-07-15T04:07:03.0646344Z Reading package lists...
2019-07-15T04:07:03.2698070Z Building dependency tree...
2019-07-15T04:07:03.2698180Z Reading state information...
2019-07-15T04:07:03.3940933Z The following additional packages will be installed:
2019-07-15T04:07:03.3941850Z   binfmt-support binutils bzip2 cmake-data cpp cpp-5 dpkg-dev g++-5 gcc gcc-5
2019-07-15T04:07:03.3942166Z   git-man libarchive13 libasan2 libasn1-8-heimdal libatomic1
2019-07-15T04:07:03.3942426Z   libbabeltrace-ctf1 libbabeltrace1 libbsd-dev libbsd0 libbz2-1.0 libc-dev-bin
2019-07-15T04:07:03.3942976Z   libedit2 liberror-perl libexpat1 libffi-dev libffi6 libgcc-5-dev libgdbm3
2019-07-15T04:07:03.3943216Z   libglib2.0-0 libgmp10 libgnutls30 libgomp1 libgssapi-krb5-2
2019-07-15T04:07:03.3943493Z   libgssapi3-heimdal libhcrypto4-heimdal libheimbase1-heimdal
2019-07-15T04:07:03.3943764Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-15T04:07:03.3943764Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-15T04:07:03.3944018Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-15T04:07:03.3944306Z   libkrb5-3 libkrb5support0 libldap-2.4-2 libllvm6.0 liblsan0 liblzo2-2
2019-07-15T04:07:03.3944560Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-15T04:07:03.3944799Z   libperl5.22 libpipeline1 libpython-stdlib libpython2.7-minimal
2019-07-15T04:07:03.3945483Z   libpython2.7-stdlib libpython3.5 libpython3.5-minimal libpython3.5-stdlib
2019-07-15T04:07:03.3945742Z   libquadmath0 libroken18-heimdal librtmp1 libsasl2-2 libsasl2-modules-db
2019-07-15T04:07:03.3945992Z   libsqlite3-0 libssl1.0.0 libstdc++-5-dev libtasn1-6 libtinfo-dev libtsan0
2019-07-15T04:07:03.3946336Z   libubsan0 libwind0-heimdal libxml2 linux-libc-dev llvm-6.0 llvm-6.0-dev
2019-07-15T04:07:03.3946594Z   llvm-6.0-runtime mime-support openssl patch perl perl-modules-5.22 python
2019-07-15T04:07:03.3946812Z   python-minimal python2.7-minimal
2019-07-15T04:07:03.3967422Z Suggested packages:
2019-07-15T04:07:03.3967992Z   binutils-doc bzip2-doc codeblocks eclipse ninja-build cpp-doc gcc-5-locales
2019-07-15T04:07:03.3968260Z   debian-keyring g++-multilib g++-5-multilib gcc-5-doc libstdc++6-5-dbg
2019-07-15T04:07:03.3968575Z   gcc-multilib manpages-dev autoconf automake libtool flex bison gcc-doc
2019-07-15T04:07:03.3969081Z   libasan2-dbg liblsan0-dbg libtsan0-dbg libubsan0-dbg libcilkrts5-dbg
2019-07-15T04:07:03.3969375Z   libmpx0-dbg libquadmath0-dbg gdb-doc gettext-base git-daemon-run
2019-07-15T04:07:03.3969375Z   libmpx0-dbg libquadmath0-dbg gdb-doc gettext-base git-daemon-run
2019-07-15T04:07:03.3969622Z   | git-daemon-sysvinit git-doc git-el git-email git-gui gitk gitweb git-arch
2019-07-15T04:07:03.3969869Z   git-cvs git-mediawiki git-svn lrzip glibc-doc gnutls-bin krb5-doc krb5-user
2019-07-15T04:07:03.3970157Z   libstdc++-5-doc llvm-6.0-doc make-doc ed diffutils-doc perl-doc
2019-07-15T04:07:03.3970414Z   libterm-readline-gnu-perl | libterm-readline-perl-perl python-doc python-tk
2019-07-15T04:07:03.3970611Z   python2.7-doc
2019-07-15T04:07:03.3970701Z Recommended packages:
2019-07-15T04:07:03.3970950Z   build-essential fakeroot libalgorithm-merge-perl libc-dbg gdbserver less
2019-07-15T04:07:03.3971188Z   rsync ssh-client manpages manpages-dev libfile-fcntllock-perl
2019-07-15T04:07:03.3971485Z   libglib2.0-data shared-mime-info xdg-user-dirs krb5-locales libsasl2-modules
2019-07-15T04:07:03.3971723Z   libssl-doc xml-core netbase rename
2019-07-15T04:07:03.3971776Z The following NEW packages will be installed:
2019-07-15T04:07:03.3972104Z   binfmt-support binutils bzip2 ca-certificates cmake cmake-data cpp cpp-5
2019-07-15T04:07:03.3972363Z   curl dpkg-dev file g++ g++-5 gcc gcc-5 gdb git git-man libarchive13 libasan2
2019-07-15T04:07:03.3972614Z   libasn1-8-heimdal libatomic1 libbabeltrace-ctf1 libbabeltrace1 libbsd-dev
2019-07-15T04:07:03.3972908Z   libbsd0 libc-dev-bin libc6-dev libcc1-0 libcilkrts5 libcurl3 libcurl3-gnutls
2019-07-15T04:07:03.3973380Z   libdpkg-perl libedit-dev libedit2 liberror-perl libexpat1 libffi-dev libffi6
2019-07-15T04:07:03.3973682Z   libgcc-5-dev libgdbm3 libglib2.0-0 libgmp10 libgnutls30 libgomp1
2019-07-15T04:07:03.3974251Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-15T04:07:03.3974517Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-15T04:07:03.3974807Z   libkrb5-3 libkrb5support0 libldap-2.4-2 libllvm6.0 liblsan0 liblzo2-2
2019-07-15T04:07:03.3975059Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-15T04:07:03.3975059Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-15T04:07:03.3975299Z   libperl5.22 libpipeline1 libpython-stdlib libpython2.7-minimal
2019-07-15T04:07:03.3975590Z   libpython2.7-stdlib libpython3.5 libpython3.5-minimal libpython3.5-stdlib
2019-07-15T04:07:03.3975848Z   libquadmath0 libroken18-heimdal librtmp1 libsasl2-2 libsasl2-modules-db
2019-07-15T04:07:03.3976429Z   libsqlite3-0 libssl-dev libssl1.0.0 libstdc++-5-dev libtasn1-6 libtinfo-dev
2019-07-15T04:07:03.3976785Z   libtsan0 libubsan0 libwind0-heimdal libxml2 linux-libc-dev llvm-6.0
2019-07-15T04:07:03.3977052Z   llvm-6.0-dev llvm-6.0-runtime llvm-6.0-tools make mime-support openssl patch
2019-07-15T04:07:03.3977292Z   perl perl-modules-5.22 pkg-config python python-minimal python2.7
2019-07-15T04:07:03.3977712Z   python2.7-minimal sudo xz-utils zlib1g-dev
2019-07-15T04:07:03.3977769Z The following packages will be upgraded:
2019-07-15T04:07:04.0983067Z 1 upgraded, 115 newly installed, 0 to remove and 4 not upgraded.
2019-07-15T04:07:04.0984208Z Need to get 121 MB of archives.
2019-07-15T04:07:04.0984300Z After this operation, 592 MB of additional disk space will be used.
2019-07-15T04:07:04.0985206Z Get:1 http://archive.ubuntu.com/ubuntu xenial/main amd64 libgdbm3 amd64 1.8.3-13.1 [16.9 kB]
---
2019-07-15T04:07:06.3420351Z Get:61 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 sudo amd64 1.8.16-0ubuntu1.7 [390 kB]
2019-07-15T04:07:06.4063759Z Get:62 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 openssl amd64 1.0.2g-1ubuntu4.15 [492 kB]
2019-07-15T04:07:06.4139674Z Get:63 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 ca-certificates all 20170717~16.04.2 [167 kB]
2019-07-15T04:07:06.4163863Z Get:64 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 libcurl3-gnutls amd64 7.47.0-1ubuntu2.13 [184 kB]
2019-07-15T04:07:06.4192885Z Get:65 http://archive.ubuntu.com/ubuntu xenial/main amd64 libedit2 amd64 3.1-20150325-1ubuntu2 [76.5 kB]
2019-07-15T04:07:06.4214571Z Get:66 http://archive.ubuntu.com/ubuntu xenial/main amd64 libpipeline1 amd64 1.4.1-2 [24.6 kB]
2019-07-15T04:07:06.4228423Z Get:67 http://archive.ubuntu.com/ubuntu xenial/main amd64 binfmt-support amd64 2.1.6-1 [50.7 kB]
2019-07-15T04:07:06.5477421Z Get:69 http://archive.ubuntu.com/ubuntu xenial/main amd64 libisl15 amd64 0.16.1-1 [524 kB]
2019-07-15T04:07:06.5553592Z Get:70 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 cpp-5 amd64 5.4.0-6ubuntu1~16.04.11 [7660 kB]
2019-07-15T04:07:06.8797820Z Get:71 http://archive.ubuntu.com/ubuntu xenial/main amd64 cpp amd64 4:5.3.1-1ubuntu1 [27.7 kB]
2019-07-15T04:07:06.8802541Z Get:72 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 curl amd64 7.47.0-1ubuntu2.13 [139 kB]
---
2019-07-15T04:07:26.7031769Z Unpacking ca-certificates (20170717~16.04.2) ...
2019-07-15T04:07:26.8686425Z Selecting previously unselected package libcurl3-gnutls:amd64.
2019-07-15T04:07:26.8703936Z Preparing to unpack .../libcurl3-gnutls_7.47.0-1ubuntu2.13_amd64.deb ...
2019-07-15T04:07:26.8833183Z Unpacking libcurl3-gnutls:amd64 (7.47.0-1ubuntu2.13) ...
2019-07-15T04:07:27.0258310Z Selecting previously unselected package libedit2:amd64.
2019-07-15T04:07:27.0274038Z Preparing to unpack .../libedit2_3.1-20150325-1ubuntu2_amd64.deb ...
2019-07-15T04:07:27.0400036Z Unpacking libedit2:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-15T04:07:27.1557719Z Selecting previously unselected package libpipeline1:amd64.
2019-07-15T04:07:27.1576156Z Preparing to unpack .../libpipeline1_1.4.1-2_amd64.deb ...
2019-07-15T04:07:27.1714746Z Unpacking libpipeline1:amd64 (1.4.1-2) ...
2019-07-15T04:07:27.2861534Z Selecting previously unselected package binfmt-support.
2019-07-15T04:07:27.2877757Z Preparing to unpack .../binfmt-support_2.1.6-1_amd64.deb ...
2019-07-15T04:07:27.3022115Z Unpacking binfmt-support (2.1.6-1) ...
2019-07-15T04:07:27.4132956Z Preparing to unpack .../binutils_2.26.1-1ubuntu1~16.04.8_amd64.deb ...
2019-07-15T04:07:27.4262059Z Unpacking binutils (2.26.1-1ubuntu1~16.04.8) ...
2019-07-15T04:07:28.0102142Z Selecting previously unselected package libisl15:amd64.
2019-07-15T04:07:28.0121326Z Preparing to unpack .../libisl15_0.16.1-1_amd64.deb ...
---
2019-07-15T04:07:39.1718841Z Selecting previously unselected package libtinfo-dev:amd64.
2019-07-15T04:07:39.1739486Z Preparing to unpack .../libtinfo-dev_6.0+20160213-1ubuntu1_amd64.deb ...
2019-07-15T04:07:39.1899215Z Unpacking libtinfo-dev:amd64 (6.0+20160213-1ubuntu1) ...
2019-07-15T04:07:39.2901225Z Selecting previously unselected package libedit-dev:amd64.
2019-07-15T04:07:39.2916405Z Preparing to unpack .../libedit-dev_3.1-20150325-1ubuntu2_amd64.deb ...
2019-07-15T04:07:39.3056645Z Unpacking libedit-dev:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-15T04:07:39.4150461Z Selecting previously unselected package libllvm6.0:amd64.
2019-07-15T04:07:39.4170962Z Preparing to unpack .../libllvm6.0_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-15T04:07:39.4307716Z Unpacking libllvm6.0:amd64 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-15T04:07:42.3517943Z Selecting previously unselected package zlib1g-dev:amd64.
2019-07-15T04:07:42.3538782Z Preparing to unpack .../zlib1g-dev_1%3a1.2.8.dfsg-2ubuntu4.1_amd64.deb ...
2019-07-15T04:07:42.3670143Z Unpacking zlib1g-dev:amd64 (1:1.2.8.dfsg-2ubuntu4.1) ...
2019-07-15T04:07:42.4785531Z Preparing to unpack .../libssl-dev_1.0.2g-1ubuntu4.15_amd64.deb ...
2019-07-15T04:07:42.4940276Z Unpacking libssl-dev:amd64 (1.0.2g-1ubuntu4.15) ...
2019-07-15T04:07:42.8073900Z Selecting previously unselected package llvm-6.0-runtime.
2019-07-15T04:07:42.8073900Z Selecting previously unselected package llvm-6.0-runtime.
2019-07-15T04:07:42.8092348Z Preparing to unpack .../llvm-6.0-runtime_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-15T04:07:42.8225182Z Unpacking llvm-6.0-runtime (1:6.0-1ubuntu2~16.04.1) ...
2019-07-15T04:07:42.9757211Z Selecting previously unselected package llvm-6.0.
2019-07-15T04:07:42.9770996Z Preparing to unpack .../llvm-6.0_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-15T04:07:42.9931269Z Unpacking llvm-6.0 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-15T04:07:43.6427030Z Selecting previously unselected package libffi-dev:amd64.
2019-07-15T04:07:43.6444308Z Preparing to unpack .../libffi-dev_3.2.1-4_amd64.deb ...
2019-07-15T04:07:43.6573828Z Unpacking libffi-dev:amd64 (3.2.1-4) ...
2019-07-15T04:07:43.7829384Z Selecting previously unselected package llvm-6.0-dev.
2019-07-15T04:07:43.7847493Z Preparing to unpack .../llvm-6.0-dev_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-15T04:07:43.8016832Z Unpacking llvm-6.0-dev (1:6.0-1ubuntu2~16.04.1) ...
2019-07-15T04:07:48.4486785Z Selecting previously unselected package llvm-6.0-tools.
2019-07-15T04:07:48.4507873Z Preparing to unpack .../llvm-6.0-tools_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-15T04:07:48.4636932Z Unpacking llvm-6.0-tools (1:6.0-1ubuntu2~16.04.1) ...
2019-07-15T04:07:48.6164753Z Selecting previously unselected package pkg-config.
2019-07-15T04:07:48.6181487Z Preparing to unpack .../pkg-config_0.29.1-0ubuntu1_amd64.deb ...
2019-07-15T04:07:48.6329548Z Unpacking pkg-config (0.29.1-0ubuntu1) ...
2019-07-15T04:07:48.7408301Z Processing triggers for systemd (229-4ubuntu21.21) ...
2019-07-15T04:07:49.0962198Z Setting up libgdbm3:amd64 (1.8.3-13.1) ...
2019-07-15T04:07:49.1651765Z Setting up libffi6:amd64 (3.2.1-4) ...
2019-07-15T04:07:49.2063280Z Setting up libglib2.0-0:amd64 (2.48.2-0ubuntu4.3) ...
---
2019-07-15T04:07:52.8352347Z debconf: unable to initialize frontend: Dialog
2019-07-15T04:07:52.8352746Z debconf: (TERM is not set, so the dialog frontend is not usable.)
2019-07-15T04:07:52.8352808Z debconf: falling back to frontend: Readline
2019-07-15T04:07:53.3628329Z Setting up libcurl3-gnutls:amd64 (7.47.0-1ubuntu2.13) ...
2019-07-15T04:07:53.4059407Z Setting up libedit2:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-15T04:07:53.4439243Z Setting up libpipeline1:amd64 (1.4.1-2) ...
2019-07-15T04:07:53.4807024Z Setting up binfmt-support (2.1.6-1) ...
2019-07-15T04:07:53.5490826Z mount: permission denied
2019-07-15T04:07:53.5496686Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-15T04:07:53.5513084Z mount: permission denied
2019-07-15T04:07:53.5519354Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-15T04:07:53.7101714Z invoke-rc.d: could not determine current runlevel
2019-07-15T04:07:53.7129169Z invoke-rc.d: policy-rc.d denied execution of start.
2019-07-15T04:07:53.7704773Z Setting up libisl15:amd64 (0.16.1-1) ...
2019-07-15T04:07:53.8074138Z Setting up cpp-5 (5.4.0-6ubuntu1~16.04.11) ...
2019-07-15T04:07:53.8483480Z Setting up cpp (4:5.3.1-1ubuntu1) ...
2019-07-15T04:07:53.8999884Z Setting up curl (7.47.0-1ubuntu2.13) ...
---
2019-07-15T04:07:55.5299598Z Setting up libedit-dev:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-15T04:07:55.5699434Z Setting up libllvm6.0:amd64 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-15T04:07:55.6139615Z Setting up zlib1g-dev:amd64 (1:1.2.8.dfsg-2ubuntu4.1) ...
2019-07-15T04:07:55.6555631Z Setting up libssl-dev:amd64 (1.0.2g-1ubuntu4.15) ...
2019-07-15T04:07:55.7024746Z Setting up llvm-6.0-runtime (1:6.0-1ubuntu2~16.04.1) ...
2019-07-15T04:07:55.7358001Z mount: permission denied
2019-07-15T04:07:55.7358911Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-15T04:07:55.7498996Z Setting up llvm-6.0 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-15T04:07:55.7888353Z Setting up libffi-dev:amd64 (3.2.1-4) ...
2019-07-15T04:07:55.8320202Z Setting up llvm-6.0-dev (1:6.0-1ubuntu2~16.04.1) ...
2019-07-15T04:07:55.8708220Z Setting up llvm-6.0-tools (1:6.0-1ubuntu2~16.04.1) ...
2019-07-15T04:07:55.9083896Z Setting up pkg-config (0.29.1-0ubuntu1) ...
2019-07-15T04:07:56.0475642Z Processing triggers for ca-certificates (20170717~16.04.2) ...
2019-07-15T04:07:56.0675656Z Updating certificates in /etc/ssl/certs...
2019-07-15T04:07:57.6508591Z 148 added, 0 removed; done.
2019-07-15T04:07:57.6509761Z Running hooks in /etc/ca-certificates/update.d...
---
2019-07-15T04:08:28.1102058Z Removing intermediate container 6435f2511c50
2019-07-15T04:08:28.1102866Z  ---> 3ca44aa41ca2
2019-07-15T04:08:28.1140770Z Successfully built 3ca44aa41ca2
2019-07-15T04:08:28.2476780Z Successfully tagged rust-ci:latest
2019-07-15T04:08:28.3328395Z Built container sha256:3ca44aa41ca21f87541e2791077e8d67db5d63531c933afb78e948a6e9672cd2
2019-07-15T04:08:28.3343304Z Uploading finished image to https://.s3.amazonaws.com/docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624
2019-07-15T04:09:29.9795460Z upload failed: - to s3:///docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624 Parameter validation failed:
2019-07-15T04:09:29.9795950Z Invalid bucket name "": Bucket name must match the regex "^[a-zA-Z0-9.\-_]{1,255}$"
2019-07-15T04:09:30.9248064Z [CI_JOB_NAME=x86_64-gnu-llvm-6.0]
2019-07-15T04:09:30.9294470Z Starting sccache server...
2019-07-15T04:09:30.9775526Z configure: processing command line
2019-07-15T04:09:30.9776257Z configure: 
---
2019-07-15T04:13:05.9361583Z    Compiling serde_json v1.0.33
2019-07-15T04:13:10.1832774Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-07-15T04:13:18.8517338Z     Finished release [optimized] target(s) in 1m 30s
2019-07-15T04:13:18.8590504Z tidy check
2019-07-15T04:13:19.4080572Z tidy error: /checkout/src/test/run-pass/issue-58375-monomorphize-default-impls.rs: too many trailing newlines (2)
2019-07-15T04:13:20.6835452Z some tidy checks failed
2019-07-15T04:13:20.6840084Z 
2019-07-15T04:13:20.6840084Z 
2019-07-15T04:13:20.6841009Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-07-15T04:13:20.6841525Z 
2019-07-15T04:13:20.6841583Z 
2019-07-15T04:13:20.6855247Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-07-15T04:13:20.6855333Z Build completed unsuccessfully in 0:01:33
2019-07-15T04:13:20.6855333Z Build completed unsuccessfully in 0:01:33
2019-07-15T04:13:21.9462876Z ##[error]Bash exited with code '1'.
2019-07-15T04:13:21.9497108Z ##[section]Starting: Checkout
2019-07-15T04:13:21.9498779Z ==============================================================================
2019-07-15T04:13:21.9498835Z Task         : Get sources
2019-07-15T04:13:21.9498884Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
