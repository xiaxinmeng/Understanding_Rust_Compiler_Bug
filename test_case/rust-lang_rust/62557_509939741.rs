plain
2019-07-10T06:52:08.4604240Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-10T06:52:08.4815144Z ##[command]git config gc.auto 0
2019-07-10T06:52:08.4879803Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-10T06:52:08.4937529Z ##[command]git config --get-all http.proxy
2019-07-10T06:52:08.5101382Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62557/merge:refs/remotes/pull/62557/merge
---
2019-07-10T06:52:44.2041844Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-10T06:52:44.2041886Z 
2019-07-10T06:52:44.2042160Z   git checkout -b <new-branch-name>
2019-07-10T06:52:44.2042221Z 
2019-07-10T06:52:44.2042281Z HEAD is now at 5c1e38d8b Merge 2da1971a039f90fc9c6d4088d696815f447fa1e8 into 3f435f622e0c05a199eb89b71a11181133fdb74c
2019-07-10T06:52:44.2196104Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-10T06:52:44.2199264Z ==============================================================================
2019-07-10T06:52:44.2199333Z Task         : Bash
2019-07-10T06:52:44.2199410Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-10T06:54:33.2621131Z Attempting with retry: curl -y 30 -Y 10 --connect-timeout 30 -f -L -C - -o /tmp/rustci_docker_cache https://.s3.amazonaws.com/docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624
2019-07-10T06:54:33.2682840Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-10T06:54:33.2683352Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-10T06:54:33.2683594Z 
2019-07-10T06:54:33.2685084Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-10T06:54:34.2785410Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-10T06:54:34.2785687Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-10T06:54:34.2785763Z 
2019-07-10T06:54:34.2785763Z 
2019-07-10T06:54:34.2786888Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-10T06:54:36.2911666Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-10T06:54:36.2912049Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-10T06:54:36.2912251Z 
2019-07-10T06:54:36.2912251Z 
2019-07-10T06:54:36.2953234Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-10T06:54:39.3025494Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-10T06:54:39.3025609Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-10T06:54:39.3025653Z 
2019-07-10T06:54:39.3025653Z 
2019-07-10T06:54:39.3076770Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-10T06:54:43.3140118Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-10T06:54:43.3140685Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-10T06:54:43.3140794Z 
2019-07-10T06:54:43.3140794Z 
2019-07-10T06:54:43.3183325Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-10T06:54:43.3188363Z The command has failed after 5 attempts.
2019-07-10T06:54:43.3862393Z open /tmp/rustci_docker_cache: no such file or directory
2019-07-10T06:54:43.3894048Z Attempting with retry: docker build --rm -t rust-ci -f /home/vsts/work/1/s/src/ci/docker/x86_64-gnu-llvm-6.0/Dockerfile /home/vsts/work/1/s/src/ci/docker
2019-07-10T06:54:43.6298363Z Sending build context to Docker daemon  521.7kB
2019-07-10T06:54:43.6299156Z 
2019-07-10T06:54:43.6583313Z Step 1/8 : FROM ubuntu:16.04
---
2019-07-10T06:55:00.1320694Z Reading package lists...
2019-07-10T06:55:01.1562036Z Reading package lists...
2019-07-10T06:55:01.3442641Z Building dependency tree...
2019-07-10T06:55:01.3442821Z Reading state information...
2019-07-10T06:55:01.4688941Z The following additional packages will be installed:
2019-07-10T06:55:01.4690660Z   binfmt-support binutils bzip2 cmake-data cpp cpp-5 dpkg-dev g++-5 gcc gcc-5
2019-07-10T06:55:01.4710473Z   git-man libarchive13 libasan2 libasn1-8-heimdal libatomic1
2019-07-10T06:55:01.4713788Z   libbabeltrace-ctf1 libbabeltrace1 libbsd-dev libbsd0 libbz2-1.0 libc-dev-bin
2019-07-10T06:55:01.4719370Z   libedit2 liberror-perl libexpat1 libffi-dev libffi6 libgcc-5-dev libgdbm3
2019-07-10T06:55:01.4720100Z   libglib2.0-0 libgmp10 libgnutls30 libgomp1 libgssapi-krb5-2
2019-07-10T06:55:01.4720696Z   libgssapi3-heimdal libhcrypto4-heimdal libheimbase1-heimdal
2019-07-10T06:55:01.4721275Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-10T06:55:01.4721275Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-10T06:55:01.4721857Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-10T06:55:01.4722407Z   libkrb5-3 libkrb5support0 libldap-2.4-2 libllvm6.0 liblsan0 liblzo2-2
2019-07-10T06:55:01.4722949Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-10T06:55:01.4723513Z   libperl5.22 libpipeline1 libpython-stdlib libpython2.7-minimal
2019-07-10T06:55:01.4724059Z   libpython2.7-stdlib libpython3.5 libpython3.5-minimal libpython3.5-stdlib
2019-07-10T06:55:01.4724600Z   libquadmath0 libroken18-heimdal librtmp1 libsasl2-2 libsasl2-modules-db
2019-07-10T06:55:01.4725179Z   libsqlite3-0 libssl1.0.0 libstdc++-5-dev libtasn1-6 libtinfo-dev libtsan0
2019-07-10T06:55:01.4725793Z   libubsan0 libwind0-heimdal libxml2 linux-libc-dev llvm-6.0 llvm-6.0-dev
2019-07-10T06:55:01.4726359Z   llvm-6.0-runtime mime-support openssl patch perl perl-modules-5.22 python
2019-07-10T06:55:01.4727287Z   python-minimal python2.7-minimal
2019-07-10T06:55:01.4741580Z Suggested packages:
2019-07-10T06:55:01.4742088Z   binutils-doc bzip2-doc codeblocks eclipse ninja-build cpp-doc gcc-5-locales
2019-07-10T06:55:01.4742420Z   debian-keyring g++-multilib g++-5-multilib gcc-5-doc libstdc++6-5-dbg
2019-07-10T06:55:01.4742796Z   gcc-multilib manpages-dev autoconf automake libtool flex bison gcc-doc
2019-07-10T06:55:01.4743517Z   libasan2-dbg liblsan0-dbg libtsan0-dbg libubsan0-dbg libcilkrts5-dbg
2019-07-10T06:55:01.4743830Z   libmpx0-dbg libquadmath0-dbg gdb-doc gettext-base git-daemon-run
2019-07-10T06:55:01.4743830Z   libmpx0-dbg libquadmath0-dbg gdb-doc gettext-base git-daemon-run
2019-07-10T06:55:01.4744175Z   | git-daemon-sysvinit git-doc git-el git-email git-gui gitk gitweb git-arch
2019-07-10T06:55:01.4744521Z   git-cvs git-mediawiki git-svn lrzip glibc-doc gnutls-bin krb5-doc krb5-user
2019-07-10T06:55:01.4744834Z   libstdc++-5-doc llvm-6.0-doc make-doc ed diffutils-doc perl-doc
2019-07-10T06:55:01.4745191Z   libterm-readline-gnu-perl | libterm-readline-perl-perl python-doc python-tk
2019-07-10T06:55:01.4745454Z   python2.7-doc
2019-07-10T06:55:01.4745515Z Recommended packages:
2019-07-10T06:55:01.4745847Z   build-essential fakeroot libalgorithm-merge-perl libc-dbg gdbserver less
2019-07-10T06:55:01.4746155Z   rsync ssh-client manpages manpages-dev libfile-fcntllock-perl
2019-07-10T06:55:01.4746478Z   libglib2.0-data shared-mime-info xdg-user-dirs krb5-locales libsasl2-modules
2019-07-10T06:55:01.4747096Z   libssl-doc xml-core netbase rename
2019-07-10T06:55:01.4747393Z The following NEW packages will be installed:
2019-07-10T06:55:01.4747847Z   binfmt-support binutils bzip2 ca-certificates cmake cmake-data cpp cpp-5
2019-07-10T06:55:01.4748231Z   curl dpkg-dev file g++ g++-5 gcc gcc-5 gdb git git-man libarchive13 libasan2
2019-07-10T06:55:01.4748932Z   libasn1-8-heimdal libatomic1 libbabeltrace-ctf1 libbabeltrace1 libbsd-dev
2019-07-10T06:55:01.4749423Z   libbsd0 libc-dev-bin libc6-dev libcc1-0 libcilkrts5 libcurl3 libcurl3-gnutls
2019-07-10T06:55:01.4749853Z   libdpkg-perl libedit-dev libedit2 liberror-perl libexpat1 libffi-dev libffi6
2019-07-10T06:55:01.4750206Z   libgcc-5-dev libgdbm3 libglib2.0-0 libgmp10 libgnutls30 libgomp1
2019-07-10T06:55:01.4750922Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-10T06:55:01.4751279Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-10T06:55:01.4751624Z   libkrb5-3 libkrb5support0 libldap-2.4-2 libllvm6.0 liblsan0 liblzo2-2
2019-07-10T06:55:01.4752074Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-10T06:55:01.4752074Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-10T06:55:01.4752415Z   libperl5.22 libpipeline1 libpython-stdlib libpython2.7-minimal
2019-07-10T06:55:01.4752780Z   libpython2.7-stdlib libpython3.5 libpython3.5-minimal libpython3.5-stdlib
2019-07-10T06:55:01.4753158Z   libquadmath0 libroken18-heimdal librtmp1 libsasl2-2 libsasl2-modules-db
2019-07-10T06:55:01.4753518Z   libsqlite3-0 libssl-dev libssl1.0.0 libstdc++-5-dev libtasn1-6 libtinfo-dev
2019-07-10T06:55:01.4753854Z   libtsan0 libubsan0 libwind0-heimdal libxml2 linux-libc-dev llvm-6.0
2019-07-10T06:55:01.4754216Z   llvm-6.0-dev llvm-6.0-runtime llvm-6.0-tools make mime-support openssl patch
2019-07-10T06:55:01.4754559Z   perl perl-modules-5.22 pkg-config python python-minimal python2.7
2019-07-10T06:55:01.4754870Z   python2.7-minimal sudo xz-utils zlib1g-dev
2019-07-10T06:55:01.4754957Z The following packages will be upgraded:
2019-07-10T06:55:01.9006142Z 1 upgraded, 115 newly installed, 0 to remove and 4 not upgraded.
2019-07-10T06:55:01.9007243Z Need to get 121 MB of archives.
2019-07-10T06:55:01.9007424Z After this operation, 592 MB of additional disk space will be used.
2019-07-10T06:55:01.9008320Z Get:1 http://archive.ubuntu.com/ubuntu xenial/main amd64 libgdbm3 amd64 1.8.3-13.1 [16.9 kB]
---
2019-07-10T06:55:05.0994480Z Get:61 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 sudo amd64 1.8.16-0ubuntu1.7 [390 kB]
2019-07-10T06:55:05.1300362Z Get:62 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 openssl amd64 1.0.2g-1ubuntu4.15 [492 kB]
2019-07-10T06:55:05.1440516Z Get:63 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 ca-certificates all 20170717~16.04.2 [167 kB]
2019-07-10T06:55:05.1495029Z Get:64 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 libcurl3-gnutls amd64 7.47.0-1ubuntu2.13 [184 kB]
2019-07-10T06:55:05.2327463Z Get:65 http://archive.ubuntu.com/ubuntu xenial/main amd64 libedit2 amd64 3.1-20150325-1ubuntu2 [76.5 kB]
2019-07-10T06:55:05.2356208Z Get:66 http://archive.ubuntu.com/ubuntu xenial/main amd64 libpipeline1 amd64 1.4.1-2 [24.6 kB]
2019-07-10T06:55:05.2367223Z Get:67 http://archive.ubuntu.com/ubuntu xenial/main amd64 binfmt-support amd64 2.1.6-1 [50.7 kB]
2019-07-10T06:55:05.3013204Z Get:69 http://archive.ubuntu.com/ubuntu xenial/main amd64 libisl15 amd64 0.16.1-1 [524 kB]
2019-07-10T06:55:05.3153196Z Get:70 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 cpp-5 amd64 5.4.0-6ubuntu1~16.04.11 [7660 kB]
2019-07-10T06:55:05.6382960Z Get:71 http://archive.ubuntu.com/ubuntu xenial/main amd64 cpp amd64 4:5.3.1-1ubuntu1 [27.7 kB]
2019-07-10T06:55:05.6415781Z Get:72 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 curl amd64 7.47.0-1ubuntu2.13 [139 kB]
---
2019-07-10T06:55:27.1356655Z Unpacking ca-certificates (20170717~16.04.2) ...
2019-07-10T06:55:27.3118310Z Selecting previously unselected package libcurl3-gnutls:amd64.
2019-07-10T06:55:27.3130872Z Preparing to unpack .../libcurl3-gnutls_7.47.0-1ubuntu2.13_amd64.deb ...
2019-07-10T06:55:27.3257236Z Unpacking libcurl3-gnutls:amd64 (7.47.0-1ubuntu2.13) ...
2019-07-10T06:55:27.4840225Z Selecting previously unselected package libedit2:amd64.
2019-07-10T06:55:27.4863644Z Preparing to unpack .../libedit2_3.1-20150325-1ubuntu2_amd64.deb ...
2019-07-10T06:55:27.4992735Z Unpacking libedit2:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-10T06:55:27.6418680Z Selecting previously unselected package libpipeline1:amd64.
2019-07-10T06:55:27.6435218Z Preparing to unpack .../libpipeline1_1.4.1-2_amd64.deb ...
2019-07-10T06:55:27.6574121Z Unpacking libpipeline1:amd64 (1.4.1-2) ...
2019-07-10T06:55:27.7635805Z Selecting previously unselected package binfmt-support.
2019-07-10T06:55:27.7652342Z Preparing to unpack .../binfmt-support_2.1.6-1_amd64.deb ...
2019-07-10T06:55:27.7790888Z Unpacking binfmt-support (2.1.6-1) ...
2019-07-10T06:55:27.9826742Z Preparing to unpack .../binutils_2.26.1-1ubuntu1~16.04.8_amd64.deb ...
2019-07-10T06:55:27.9958062Z Unpacking binutils (2.26.1-1ubuntu1~16.04.8) ...
2019-07-10T06:55:28.4952025Z Selecting previously unselected package libisl15:amd64.
2019-07-10T06:55:28.4973315Z Preparing to unpack .../libisl15_0.16.1-1_amd64.deb ...
---
2019-07-10T06:55:40.0804629Z Selecting previously unselected package libtinfo-dev:amd64.
2019-07-10T06:55:40.0820825Z Preparing to unpack .../libtinfo-dev_6.0+20160213-1ubuntu1_amd64.deb ...
2019-07-10T06:55:40.0972266Z Unpacking libtinfo-dev:amd64 (6.0+20160213-1ubuntu1) ...
2019-07-10T06:55:40.1948849Z Selecting previously unselected package libedit-dev:amd64.
2019-07-10T06:55:40.1969630Z Preparing to unpack .../libedit-dev_3.1-20150325-1ubuntu2_amd64.deb ...
2019-07-10T06:55:40.2098187Z Unpacking libedit-dev:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-10T06:55:40.3239975Z Selecting previously unselected package libllvm6.0:amd64.
2019-07-10T06:55:40.3262024Z Preparing to unpack .../libllvm6.0_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-10T06:55:40.3387823Z Unpacking libllvm6.0:amd64 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-10T06:55:43.3253366Z Selecting previously unselected package zlib1g-dev:amd64.
2019-07-10T06:55:43.3276429Z Preparing to unpack .../zlib1g-dev_1%3a1.2.8.dfsg-2ubuntu4.1_amd64.deb ...
2019-07-10T06:55:43.3402661Z Unpacking zlib1g-dev:amd64 (1:1.2.8.dfsg-2ubuntu4.1) ...
2019-07-10T06:55:43.4449342Z Preparing to unpack .../libssl-dev_1.0.2g-1ubuntu4.15_amd64.deb ...
2019-07-10T06:55:43.4583266Z Unpacking libssl-dev:amd64 (1.0.2g-1ubuntu4.15) ...
2019-07-10T06:55:43.7846287Z Selecting previously unselected package llvm-6.0-runtime.
2019-07-10T06:55:43.7846287Z Selecting previously unselected package llvm-6.0-runtime.
2019-07-10T06:55:43.7877643Z Preparing to unpack .../llvm-6.0-runtime_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-10T06:55:43.8012391Z Unpacking llvm-6.0-runtime (1:6.0-1ubuntu2~16.04.1) ...
2019-07-10T06:55:43.9294500Z Selecting previously unselected package llvm-6.0.
2019-07-10T06:55:43.9317346Z Preparing to unpack .../llvm-6.0_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-10T06:55:43.9453360Z Unpacking llvm-6.0 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-10T06:55:44.6263979Z Selecting previously unselected package libffi-dev:amd64.
2019-07-10T06:55:44.6284382Z Preparing to unpack .../libffi-dev_3.2.1-4_amd64.deb ...
2019-07-10T06:55:44.6426342Z Unpacking libffi-dev:amd64 (3.2.1-4) ...
2019-07-10T06:55:44.7663386Z Selecting previously unselected package llvm-6.0-dev.
2019-07-10T06:55:44.7680510Z Preparing to unpack .../llvm-6.0-dev_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-10T06:55:44.7835953Z Unpacking llvm-6.0-dev (1:6.0-1ubuntu2~16.04.1) ...
2019-07-10T06:55:49.5252490Z Selecting previously unselected package llvm-6.0-tools.
2019-07-10T06:55:49.5277273Z Preparing to unpack .../llvm-6.0-tools_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-10T06:55:49.5411157Z Unpacking llvm-6.0-tools (1:6.0-1ubuntu2~16.04.1) ...
2019-07-10T06:55:49.6979254Z Selecting previously unselected package pkg-config.
2019-07-10T06:55:49.7002528Z Preparing to unpack .../pkg-config_0.29.1-0ubuntu1_amd64.deb ...
2019-07-10T06:55:49.7124009Z Unpacking pkg-config (0.29.1-0ubuntu1) ...
2019-07-10T06:55:49.8285128Z Processing triggers for systemd (229-4ubuntu21.21) ...
2019-07-10T06:55:50.1945809Z Setting up libgdbm3:amd64 (1.8.3-13.1) ...
2019-07-10T06:55:50.2645814Z Setting up libffi6:amd64 (3.2.1-4) ...
2019-07-10T06:55:50.3065244Z Setting up libglib2.0-0:amd64 (2.48.2-0ubuntu4.3) ...
---
2019-07-10T06:55:54.0057529Z debconf: unable to initialize frontend: Dialog
2019-07-10T06:55:54.0057869Z debconf: (TERM is not set, so the dialog frontend is not usable.)
2019-07-10T06:55:54.0058041Z debconf: falling back to frontend: Readline
2019-07-10T06:55:54.5410372Z Setting up libcurl3-gnutls:amd64 (7.47.0-1ubuntu2.13) ...
2019-07-10T06:55:54.5822918Z Setting up libedit2:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-10T06:55:54.6241270Z Setting up libpipeline1:amd64 (1.4.1-2) ...
2019-07-10T06:55:54.6639138Z Setting up binfmt-support (2.1.6-1) ...
2019-07-10T06:55:54.7367348Z mount: permission denied
2019-07-10T06:55:54.7368114Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-10T06:55:54.7384973Z mount: permission denied
2019-07-10T06:55:54.7387146Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-10T06:55:54.9139892Z invoke-rc.d: could not determine current runlevel
2019-07-10T06:55:54.9173690Z invoke-rc.d: policy-rc.d denied execution of start.
2019-07-10T06:55:54.9793778Z Setting up libisl15:amd64 (0.16.1-1) ...
2019-07-10T06:55:55.0181556Z Setting up cpp-5 (5.4.0-6ubuntu1~16.04.11) ...
2019-07-10T06:55:55.0580057Z Setting up cpp (4:5.3.1-1ubuntu1) ...
2019-07-10T06:55:55.1102371Z Setting up curl (7.47.0-1ubuntu2.13) ...
---
2019-07-10T06:55:57.1532249Z Setting up libedit-dev:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-10T06:55:57.1931013Z Setting up libllvm6.0:amd64 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-10T06:55:57.2336211Z Setting up zlib1g-dev:amd64 (1:1.2.8.dfsg-2ubuntu4.1) ...
2019-07-10T06:55:57.2738164Z Setting up libssl-dev:amd64 (1.0.2g-1ubuntu4.15) ...
2019-07-10T06:55:57.3139335Z Setting up llvm-6.0-runtime (1:6.0-1ubuntu2~16.04.1) ...
2019-07-10T06:55:57.3434742Z mount: permission denied
2019-07-10T06:55:57.3437611Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-10T06:55:57.3577652Z Setting up llvm-6.0 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-10T06:55:57.3967197Z Setting up libffi-dev:amd64 (3.2.1-4) ...
2019-07-10T06:55:57.4397642Z Setting up llvm-6.0-dev (1:6.0-1ubuntu2~16.04.1) ...
2019-07-10T06:55:57.4800785Z Setting up llvm-6.0-tools (1:6.0-1ubuntu2~16.04.1) ...
2019-07-10T06:55:57.5187253Z Setting up pkg-config (0.29.1-0ubuntu1) ...
2019-07-10T06:55:57.6502252Z Processing triggers for ca-certificates (20170717~16.04.2) ...
2019-07-10T06:55:57.6669386Z Updating certificates in /etc/ssl/certs...
2019-07-10T06:55:59.3075303Z 148 added, 0 removed; done.
2019-07-10T06:55:59.3076104Z Running hooks in /etc/ca-certificates/update.d...
---
2019-07-10T06:56:34.7527256Z  ---> 2bb34ce4f1f3
2019-07-10T06:56:34.7562874Z Successfully built 2bb34ce4f1f3
2019-07-10T06:56:34.9287043Z Successfully tagged rust-ci:latest
2019-07-10T06:56:35.0144960Z Built container sha256:2bb34ce4f1f3b7a50518c9a80638a72a890d038a7eb0c2adc59188df7fc181ac
2019-07-10T06:56:35.0158735Z Uploading finished image to https://.s3.amazonaws.com/docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624
2019-07-10T06:57:36.9319631Z upload failed: - to s3:///docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624 Parameter validation failed:
2019-07-10T06:57:36.9322064Z Invalid bucket name "": Bucket name must match the regex "^[a-zA-Z0-9.\-_]{1,255}$"
2019-07-10T06:57:37.9582028Z [CI_JOB_NAME=x86_64-gnu-llvm-6.0]
2019-07-10T06:57:37.9582296Z Starting sccache server...
2019-07-10T06:57:37.9582460Z configure: processing command line
2019-07-10T06:57:37.9582635Z configure: 
---
2019-07-10T07:01:07.8226820Z    Compiling serde_json v1.0.33
2019-07-10T07:01:12.0928172Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-07-10T07:01:20.9426986Z     Finished release [optimized] target(s) in 1m 31s
2019-07-10T07:01:20.9498796Z tidy check
2019-07-10T07:01:21.1861794Z tidy error: /checkout/src/libcore/intrinsics.rs:708: line longer than 100 chars
2019-07-10T07:01:22.8262878Z some tidy checks failed
2019-07-10T07:01:22.8268537Z 
2019-07-10T07:01:22.8268537Z 
2019-07-10T07:01:22.8270621Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-07-10T07:01:22.8270744Z 
2019-07-10T07:01:22.8270795Z 
2019-07-10T07:01:22.8277365Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-07-10T07:01:22.8277692Z Build completed unsuccessfully in 0:01:34
2019-07-10T07:01:22.8277692Z Build completed unsuccessfully in 0:01:34
2019-07-10T07:01:24.0992135Z ##[error]Bash exited with code '1'.
2019-07-10T07:01:24.1025783Z ##[section]Starting: Checkout
2019-07-10T07:01:24.1027617Z ==============================================================================
2019-07-10T07:01:24.1027702Z Task         : Get sources
2019-07-10T07:01:24.1027757Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
