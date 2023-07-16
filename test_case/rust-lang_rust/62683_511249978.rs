plain
2019-07-15T00:36:08.9080265Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-15T00:36:08.9293207Z ##[command]git config gc.auto 0
2019-07-15T00:36:08.9380793Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-15T00:36:08.9444287Z ##[command]git config --get-all http.proxy
2019-07-15T00:36:08.9576755Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62683/merge:refs/remotes/pull/62683/merge
---
2019-07-15T00:36:42.2574008Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-15T00:36:42.2574037Z 
2019-07-15T00:36:42.2574255Z   git checkout -b <new-branch-name>
2019-07-15T00:36:42.2574281Z 
2019-07-15T00:36:42.2574323Z HEAD is now at b863287e3 Merge 27015b1b4191cd2a975abb30d36d44f72a78bbd8 into 83e4eed16ef7adb54a802e3b684427e0e912c2b7
2019-07-15T00:36:42.2715711Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-15T00:36:42.2719071Z ==============================================================================
2019-07-15T00:36:42.2719132Z Task         : Bash
2019-07-15T00:36:42.2719181Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-15T00:38:16.0943852Z Attempting with retry: curl -y 30 -Y 10 --connect-timeout 30 -f -L -C - -o /tmp/rustci_docker_cache https://.s3.amazonaws.com/docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624
2019-07-15T00:38:16.0993315Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-15T00:38:16.0993581Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-15T00:38:16.0993736Z 
2019-07-15T00:38:16.1036367Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-15T00:38:17.1111996Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-15T00:38:17.1112458Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-15T00:38:17.1112643Z 
2019-07-15T00:38:17.1112643Z 
2019-07-15T00:38:17.1159642Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-15T00:38:19.1225517Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-15T00:38:19.1226128Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-15T00:38:19.1267485Z 
2019-07-15T00:38:19.1267485Z 
2019-07-15T00:38:19.1268912Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-15T00:38:22.1344237Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-15T00:38:22.1344315Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-15T00:38:22.1344350Z 
2019-07-15T00:38:22.1344350Z 
2019-07-15T00:38:22.1386320Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-15T00:38:26.1483506Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-15T00:38:26.1483942Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-15T00:38:26.1484148Z 
2019-07-15T00:38:26.1484148Z 
2019-07-15T00:38:26.1485209Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-15T00:38:26.1486743Z The command has failed after 5 attempts.
2019-07-15T00:38:26.2063764Z open /tmp/rustci_docker_cache: no such file or directory
2019-07-15T00:38:26.2090834Z Attempting with retry: docker build --rm -t rust-ci -f /home/vsts/work/1/s/src/ci/docker/x86_64-gnu-llvm-6.0/Dockerfile /home/vsts/work/1/s/src/ci/docker
2019-07-15T00:38:26.4234935Z Sending build context to Docker daemon  521.7kB
2019-07-15T00:38:26.4235428Z 
2019-07-15T00:38:26.4511104Z Step 1/8 : FROM ubuntu:16.04
---
2019-07-15T00:38:41.6885468Z Reading package lists...
2019-07-15T00:38:43.2131282Z Reading package lists...
2019-07-15T00:38:43.2132226Z Building dependency tree...
2019-07-15T00:38:43.2132489Z Reading state information...
2019-07-15T00:38:43.2132656Z The following additional packages will be installed:
2019-07-15T00:38:43.2133992Z   binfmt-support binutils bzip2 cmake-data cpp cpp-5 dpkg-dev g++-5 gcc gcc-5
2019-07-15T00:38:43.2134457Z   git-man libarchive13 libasan2 libasn1-8-heimdal libatomic1
2019-07-15T00:38:43.2134887Z   libbabeltrace-ctf1 libbabeltrace1 libbsd-dev libbsd0 libbz2-1.0 libc-dev-bin
2019-07-15T00:38:43.2135833Z   libedit2 liberror-perl libexpat1 libffi-dev libffi6 libgcc-5-dev libgdbm3
2019-07-15T00:38:43.2136184Z   libglib2.0-0 libgmp10 libgnutls30 libgomp1 libgssapi-krb5-2
2019-07-15T00:38:43.2136743Z   libgssapi3-heimdal libhcrypto4-heimdal libheimbase1-heimdal
2019-07-15T00:38:43.2137492Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-15T00:38:43.2137492Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-15T00:38:43.2137829Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-15T00:38:43.2138116Z   libkrb5-3 libkrb5support0 libldap-2.4-2 libllvm6.0 liblsan0 liblzo2-2
2019-07-15T00:38:43.2138374Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-15T00:38:43.2138592Z   libperl5.22 libpipeline1 libpython-stdlib libpython2.7-minimal
2019-07-15T00:38:43.2140183Z   libpython2.7-stdlib libpython3.5 libpython3.5-minimal libpython3.5-stdlib
2019-07-15T00:38:43.2140515Z   libquadmath0 libroken18-heimdal librtmp1 libsasl2-2 libsasl2-modules-db
2019-07-15T00:38:43.2140792Z   libsqlite3-0 libssl1.0.0 libstdc++-5-dev libtasn1-6 libtinfo-dev libtsan0
2019-07-15T00:38:43.2141044Z   libubsan0 libwind0-heimdal libxml2 linux-libc-dev llvm-6.0 llvm-6.0-dev
2019-07-15T00:38:43.2141339Z   llvm-6.0-runtime mime-support openssl patch perl perl-modules-5.22 python
2019-07-15T00:38:43.2141583Z   python-minimal python2.7-minimal
2019-07-15T00:38:43.2141635Z Suggested packages:
2019-07-15T00:38:43.2141889Z   binutils-doc bzip2-doc codeblocks eclipse ninja-build cpp-doc gcc-5-locales
2019-07-15T00:38:43.2142194Z   debian-keyring g++-multilib g++-5-multilib gcc-5-doc libstdc++6-5-dbg
2019-07-15T00:38:43.2142441Z   gcc-multilib manpages-dev autoconf automake libtool flex bison gcc-doc
2019-07-15T00:38:43.2143383Z   libasan2-dbg liblsan0-dbg libtsan0-dbg libubsan0-dbg libcilkrts5-dbg
2019-07-15T00:38:43.2143609Z   libmpx0-dbg libquadmath0-dbg gdb-doc gettext-base git-daemon-run
2019-07-15T00:38:43.2143609Z   libmpx0-dbg libquadmath0-dbg gdb-doc gettext-base git-daemon-run
2019-07-15T00:38:43.2143842Z   | git-daemon-sysvinit git-doc git-el git-email git-gui gitk gitweb git-arch
2019-07-15T00:38:43.2144130Z   git-cvs git-mediawiki git-svn lrzip glibc-doc gnutls-bin krb5-doc krb5-user
2019-07-15T00:38:43.2144355Z   libstdc++-5-doc llvm-6.0-doc make-doc ed diffutils-doc perl-doc
2019-07-15T00:38:43.2144769Z   libterm-readline-gnu-perl | libterm-readline-perl-perl python-doc python-tk
2019-07-15T00:38:43.2145015Z   python2.7-doc
2019-07-15T00:38:43.2145061Z Recommended packages:
2019-07-15T00:38:43.2145446Z   build-essential fakeroot libalgorithm-merge-perl libc-dbg gdbserver less
2019-07-15T00:38:43.2146193Z   rsync ssh-client manpages manpages-dev libfile-fcntllock-perl
2019-07-15T00:38:43.2146563Z   libglib2.0-data shared-mime-info xdg-user-dirs krb5-locales libsasl2-modules
2019-07-15T00:38:43.2146749Z   libssl-doc xml-core netbase rename
2019-07-15T00:38:43.2146833Z The following NEW packages will be installed:
2019-07-15T00:38:43.2147043Z   binfmt-support binutils bzip2 ca-certificates cmake cmake-data cpp cpp-5
2019-07-15T00:38:43.2147268Z   curl dpkg-dev file g++ g++-5 gcc gcc-5 gdb git git-man libarchive13 libasan2
2019-07-15T00:38:43.2147515Z   libasn1-8-heimdal libatomic1 libbabeltrace-ctf1 libbabeltrace1 libbsd-dev
2019-07-15T00:38:43.2147866Z   libbsd0 libc-dev-bin libc6-dev libcc1-0 libcilkrts5 libcurl3 libcurl3-gnutls
2019-07-15T00:38:43.2148119Z   libdpkg-perl libedit-dev libedit2 liberror-perl libexpat1 libffi-dev libffi6
2019-07-15T00:38:43.2148369Z   libgcc-5-dev libgdbm3 libglib2.0-0 libgmp10 libgnutls30 libgomp1
2019-07-15T00:38:43.2148973Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-15T00:38:43.2150131Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-15T00:38:43.2150417Z   libkrb5-3 libkrb5support0 libldap-2.4-2 libllvm6.0 liblsan0 liblzo2-2
2019-07-15T00:38:43.2150667Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-15T00:38:43.2150667Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-15T00:38:43.2150957Z   libperl5.22 libpipeline1 libpython-stdlib libpython2.7-minimal
2019-07-15T00:38:43.2151223Z   libpython2.7-stdlib libpython3.5 libpython3.5-minimal libpython3.5-stdlib
2019-07-15T00:38:43.2151482Z   libquadmath0 libroken18-heimdal librtmp1 libsasl2-2 libsasl2-modules-db
2019-07-15T00:38:43.2151785Z   libsqlite3-0 libssl-dev libssl1.0.0 libstdc++-5-dev libtasn1-6 libtinfo-dev
2019-07-15T00:38:43.2152044Z   libtsan0 libubsan0 libwind0-heimdal libxml2 linux-libc-dev llvm-6.0
2019-07-15T00:38:43.2152298Z   llvm-6.0-dev llvm-6.0-runtime llvm-6.0-tools make mime-support openssl patch
2019-07-15T00:38:43.2152588Z   perl perl-modules-5.22 pkg-config python python-minimal python2.7
2019-07-15T00:38:43.2153099Z   python2.7-minimal sudo xz-utils zlib1g-dev
2019-07-15T00:38:43.2153250Z The following packages will be upgraded:
2019-07-15T00:38:43.2713995Z 1 upgraded, 115 newly installed, 0 to remove and 4 not upgraded.
2019-07-15T00:38:43.2714393Z Need to get 121 MB of archives.
2019-07-15T00:38:43.2715264Z After this operation, 592 MB of additional disk space will be used.
2019-07-15T00:38:43.2716245Z Get:1 http://archive.ubuntu.com/ubuntu xenial/main amd64 libgdbm3 amd64 1.8.3-13.1 [16.9 kB]
---
2019-07-15T00:38:44.4475498Z Get:61 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 sudo amd64 1.8.16-0ubuntu1.7 [390 kB]
2019-07-15T00:38:44.4533537Z Get:62 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 openssl amd64 1.0.2g-1ubuntu4.15 [492 kB]
2019-07-15T00:38:44.4640517Z Get:63 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 ca-certificates all 20170717~16.04.2 [167 kB]
2019-07-15T00:38:44.4660030Z Get:64 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 libcurl3-gnutls amd64 7.47.0-1ubuntu2.13 [184 kB]
2019-07-15T00:38:44.4690694Z Get:65 http://archive.ubuntu.com/ubuntu xenial/main amd64 libedit2 amd64 3.1-20150325-1ubuntu2 [76.5 kB]
2019-07-15T00:38:44.4705809Z Get:66 http://archive.ubuntu.com/ubuntu xenial/main amd64 libpipeline1 amd64 1.4.1-2 [24.6 kB]
2019-07-15T00:38:44.4724552Z Get:67 http://archive.ubuntu.com/ubuntu xenial/main amd64 binfmt-support amd64 2.1.6-1 [50.7 kB]
2019-07-15T00:38:44.5373610Z Get:69 http://archive.ubuntu.com/ubuntu xenial/main amd64 libisl15 amd64 0.16.1-1 [524 kB]
2019-07-15T00:38:44.5461224Z Get:70 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 cpp-5 amd64 5.4.0-6ubuntu1~16.04.11 [7660 kB]
2019-07-15T00:38:44.7269141Z Get:71 http://archive.ubuntu.com/ubuntu xenial/main amd64 cpp amd64 4:5.3.1-1ubuntu1 [27.7 kB]
2019-07-15T00:38:44.7269545Z Get:72 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 curl amd64 7.47.0-1ubuntu2.13 [139 kB]
---
2019-07-15T00:39:02.5805226Z Unpacking ca-certificates (20170717~16.04.2) ...
2019-07-15T00:39:02.7390846Z Selecting previously unselected package libcurl3-gnutls:amd64.
2019-07-15T00:39:02.7408272Z Preparing to unpack .../libcurl3-gnutls_7.47.0-1ubuntu2.13_amd64.deb ...
2019-07-15T00:39:02.7537509Z Unpacking libcurl3-gnutls:amd64 (7.47.0-1ubuntu2.13) ...
2019-07-15T00:39:02.8929000Z Selecting previously unselected package libedit2:amd64.
2019-07-15T00:39:02.8950488Z Preparing to unpack .../libedit2_3.1-20150325-1ubuntu2_amd64.deb ...
2019-07-15T00:39:02.9080794Z Unpacking libedit2:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-15T00:39:03.0347545Z Selecting previously unselected package libpipeline1:amd64.
2019-07-15T00:39:03.0368686Z Preparing to unpack .../libpipeline1_1.4.1-2_amd64.deb ...
2019-07-15T00:39:03.0497958Z Unpacking libpipeline1:amd64 (1.4.1-2) ...
2019-07-15T00:39:03.1708649Z Selecting previously unselected package binfmt-support.
2019-07-15T00:39:03.1726549Z Preparing to unpack .../binfmt-support_2.1.6-1_amd64.deb ...
2019-07-15T00:39:03.1865842Z Unpacking binfmt-support (2.1.6-1) ...
2019-07-15T00:39:03.4421076Z Preparing to unpack .../binutils_2.26.1-1ubuntu1~16.04.8_amd64.deb ...
2019-07-15T00:39:03.4566004Z Unpacking binutils (2.26.1-1ubuntu1~16.04.8) ...
2019-07-15T00:39:03.9478018Z Selecting previously unselected package libisl15:amd64.
2019-07-15T00:39:03.9494416Z Preparing to unpack .../libisl15_0.16.1-1_amd64.deb ...
---
2019-07-15T00:39:15.1941020Z Selecting previously unselected package libtinfo-dev:amd64.
2019-07-15T00:39:15.1959004Z Preparing to unpack .../libtinfo-dev_6.0+20160213-1ubuntu1_amd64.deb ...
2019-07-15T00:39:15.2077596Z Unpacking libtinfo-dev:amd64 (6.0+20160213-1ubuntu1) ...
2019-07-15T00:39:15.2956258Z Selecting previously unselected package libedit-dev:amd64.
2019-07-15T00:39:15.2974872Z Preparing to unpack .../libedit-dev_3.1-20150325-1ubuntu2_amd64.deb ...
2019-07-15T00:39:15.3096807Z Unpacking libedit-dev:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-15T00:39:15.4248426Z Selecting previously unselected package libllvm6.0:amd64.
2019-07-15T00:39:15.4270374Z Preparing to unpack .../libllvm6.0_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-15T00:39:15.4395554Z Unpacking libllvm6.0:amd64 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-15T00:39:18.3795450Z Selecting previously unselected package zlib1g-dev:amd64.
2019-07-15T00:39:18.3812583Z Preparing to unpack .../zlib1g-dev_1%3a1.2.8.dfsg-2ubuntu4.1_amd64.deb ...
2019-07-15T00:39:18.3945252Z Unpacking zlib1g-dev:amd64 (1:1.2.8.dfsg-2ubuntu4.1) ...
2019-07-15T00:39:18.4958381Z Preparing to unpack .../libssl-dev_1.0.2g-1ubuntu4.15_amd64.deb ...
2019-07-15T00:39:18.5111375Z Unpacking libssl-dev:amd64 (1.0.2g-1ubuntu4.15) ...
2019-07-15T00:39:18.8378635Z Selecting previously unselected package llvm-6.0-runtime.
2019-07-15T00:39:18.8378635Z Selecting previously unselected package llvm-6.0-runtime.
2019-07-15T00:39:18.8398431Z Preparing to unpack .../llvm-6.0-runtime_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-15T00:39:18.8541880Z Unpacking llvm-6.0-runtime (1:6.0-1ubuntu2~16.04.1) ...
2019-07-15T00:39:18.9729025Z Selecting previously unselected package llvm-6.0.
2019-07-15T00:39:18.9749591Z Preparing to unpack .../llvm-6.0_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-15T00:39:18.9896214Z Unpacking llvm-6.0 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-15T00:39:20.2223205Z Selecting previously unselected package libffi-dev:amd64.
2019-07-15T00:39:20.2224946Z Preparing to unpack .../libffi-dev_3.2.1-4_amd64.deb ...
2019-07-15T00:39:20.2226007Z Unpacking libffi-dev:amd64 (3.2.1-4) ...
2019-07-15T00:39:20.2227305Z Selecting previously unselected package llvm-6.0-dev.
2019-07-15T00:39:20.2229000Z Preparing to unpack .../llvm-6.0-dev_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-15T00:39:20.2230976Z Unpacking llvm-6.0-dev (1:6.0-1ubuntu2~16.04.1) ...
2019-07-15T00:39:24.7272511Z Selecting previously unselected package llvm-6.0-tools.
2019-07-15T00:39:24.7294501Z Preparing to unpack .../llvm-6.0-tools_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-15T00:39:24.7441602Z Unpacking llvm-6.0-tools (1:6.0-1ubuntu2~16.04.1) ...
2019-07-15T00:39:24.9271896Z Selecting previously unselected package pkg-config.
2019-07-15T00:39:24.9293923Z Preparing to unpack .../pkg-config_0.29.1-0ubuntu1_amd64.deb ...
2019-07-15T00:39:24.9455662Z Unpacking pkg-config (0.29.1-0ubuntu1) ...
2019-07-15T00:39:25.0727842Z Processing triggers for systemd (229-4ubuntu21.21) ...
2019-07-15T00:39:25.4163218Z Setting up libgdbm3:amd64 (1.8.3-13.1) ...
2019-07-15T00:39:25.4846363Z Setting up libffi6:amd64 (3.2.1-4) ...
2019-07-15T00:39:25.5306897Z Setting up libglib2.0-0:amd64 (2.48.2-0ubuntu4.3) ...
---
2019-07-15T00:39:29.8457519Z debconf: unable to initialize frontend: Dialog
2019-07-15T00:39:29.8457755Z debconf: (TERM is not set, so the dialog frontend is not usable.)
2019-07-15T00:39:29.8457960Z debconf: falling back to frontend: Readline
2019-07-15T00:39:30.3717717Z Setting up libcurl3-gnutls:amd64 (7.47.0-1ubuntu2.13) ...
2019-07-15T00:39:30.4131318Z Setting up libedit2:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-15T00:39:30.4562367Z Setting up libpipeline1:amd64 (1.4.1-2) ...
2019-07-15T00:39:30.4948386Z Setting up binfmt-support (2.1.6-1) ...
2019-07-15T00:39:30.5692630Z mount: permission denied
2019-07-15T00:39:30.5696875Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-15T00:39:30.5709760Z mount: permission denied
2019-07-15T00:39:30.5714242Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-15T00:39:30.7264134Z invoke-rc.d: could not determine current runlevel
2019-07-15T00:39:30.7294413Z invoke-rc.d: policy-rc.d denied execution of start.
2019-07-15T00:39:30.7951389Z Setting up libisl15:amd64 (0.16.1-1) ...
2019-07-15T00:39:30.8346601Z Setting up cpp-5 (5.4.0-6ubuntu1~16.04.11) ...
2019-07-15T00:39:30.8728097Z Setting up cpp (4:5.3.1-1ubuntu1) ...
2019-07-15T00:39:30.9179808Z Setting up curl (7.47.0-1ubuntu2.13) ...
---
2019-07-15T00:39:32.5819451Z Setting up libedit-dev:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-15T00:39:32.6269625Z Setting up libllvm6.0:amd64 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-15T00:39:32.6649391Z Setting up zlib1g-dev:amd64 (1:1.2.8.dfsg-2ubuntu4.1) ...
2019-07-15T00:39:32.7023327Z Setting up libssl-dev:amd64 (1.0.2g-1ubuntu4.15) ...
2019-07-15T00:39:32.7377706Z Setting up llvm-6.0-runtime (1:6.0-1ubuntu2~16.04.1) ...
2019-07-15T00:39:32.7656427Z mount: permission denied
2019-07-15T00:39:32.7658037Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-15T00:39:32.7817783Z Setting up llvm-6.0 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-15T00:39:32.8229462Z Setting up libffi-dev:amd64 (3.2.1-4) ...
2019-07-15T00:39:32.8627237Z Setting up llvm-6.0-dev (1:6.0-1ubuntu2~16.04.1) ...
2019-07-15T00:39:32.9137687Z Setting up llvm-6.0-tools (1:6.0-1ubuntu2~16.04.1) ...
2019-07-15T00:39:32.9527901Z Setting up pkg-config (0.29.1-0ubuntu1) ...
2019-07-15T00:39:33.0820277Z Processing triggers for ca-certificates (20170717~16.04.2) ...
2019-07-15T00:39:33.0976894Z Updating certificates in /etc/ssl/certs...
2019-07-15T00:39:34.6894374Z 148 added, 0 removed; done.
2019-07-15T00:39:34.6895587Z Running hooks in /etc/ca-certificates/update.d...
---
2019-07-15T00:40:07.7013072Z Removing intermediate container 7bd76d68d7ae
2019-07-15T00:40:07.7013839Z  ---> 1e70cc262893
2019-07-15T00:40:07.7060060Z Successfully built 1e70cc262893
2019-07-15T00:40:07.7961685Z Successfully tagged rust-ci:latest
2019-07-15T00:40:07.9005674Z Built container sha256:1e70cc262893563e787f70a5a975e9dd567baa4362d82917421d0f4761ab7e8d
2019-07-15T00:40:07.9026407Z Uploading finished image to https://.s3.amazonaws.com/docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624
2019-07-15T00:41:09.8336583Z upload failed: - to s3:///docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624 Parameter validation failed:
2019-07-15T00:41:09.8337181Z Invalid bucket name "": Bucket name must match the regex "^[a-zA-Z0-9.\-_]{1,255}$"
2019-07-15T00:41:10.7402037Z [CI_JOB_NAME=x86_64-gnu-llvm-6.0]
2019-07-15T00:41:10.7457079Z Starting sccache server...
2019-07-15T00:41:10.7898975Z configure: processing command line
2019-07-15T00:41:10.7899543Z configure: 
---
2019-07-15T00:44:55.8157917Z tidy check
2019-07-15T00:44:56.9792277Z * 577 error codes
2019-07-15T00:44:56.9792507Z * highest error code: E0732
2019-07-15T00:44:57.2855035Z * 265 features
2019-07-15T00:44:57.5396520Z tidy error: The Unstable Book has a 'language feature' section 'param_attrs' which doesn't correspond to an unstable language feature
2019-07-15T00:44:57.8774798Z some tidy checks failed
2019-07-15T00:44:57.8777738Z 
2019-07-15T00:44:57.8777738Z 
2019-07-15T00:44:57.8778650Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-07-15T00:44:57.8778811Z 
2019-07-15T00:44:57.8778837Z 
2019-07-15T00:44:57.8787970Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-07-15T00:44:57.8788390Z Build completed unsuccessfully in 0:01:35
2019-07-15T00:44:57.8788390Z Build completed unsuccessfully in 0:01:35
2019-07-15T00:44:59.1615943Z ##[error]Bash exited with code '1'.
2019-07-15T00:44:59.1648610Z ##[section]Starting: Checkout
2019-07-15T00:44:59.1650611Z ==============================================================================
2019-07-15T00:44:59.1650662Z Task         : Get sources
2019-07-15T00:44:59.1650707Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
