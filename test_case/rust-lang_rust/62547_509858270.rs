plain
2019-07-10T00:02:24.3411251Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-10T00:02:24.3646561Z ##[command]git config gc.auto 0
2019-07-10T00:02:24.3722949Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-10T00:02:24.3783766Z ##[command]git config --get-all http.proxy
2019-07-10T00:02:24.3926592Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62547/merge:refs/remotes/pull/62547/merge
---
2019-07-10T00:02:59.5207287Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-10T00:02:59.5207331Z 
2019-07-10T00:02:59.5209603Z   git checkout -b <new-branch-name>
2019-07-10T00:02:59.5209837Z 
2019-07-10T00:02:59.5209890Z HEAD is now at eaf4f47e2 Merge 88921e6e5b85987de66361cae8781f567c26ee9b into 0b680cfce544ff9a59d720020e397c4bf3346650
2019-07-10T00:02:59.5355295Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-10T00:02:59.5358038Z ==============================================================================
2019-07-10T00:02:59.5358115Z Task         : Bash
2019-07-10T00:02:59.5358164Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-10T00:04:36.1902764Z Attempting with retry: curl -y 30 -Y 10 --connect-timeout 30 -f -L -C - -o /tmp/rustci_docker_cache https://.s3.amazonaws.com/docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624
2019-07-10T00:04:36.1969865Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-10T00:04:36.1970935Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-10T00:04:36.1970984Z 
2019-07-10T00:04:36.2002901Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-10T00:04:37.2070447Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-10T00:04:37.2070872Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-10T00:04:37.2080079Z 
2019-07-10T00:04:37.2080079Z 
2019-07-10T00:04:37.2117637Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-10T00:04:39.2186415Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-10T00:04:39.2220396Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-10T00:04:39.2220707Z 
2019-07-10T00:04:39.2220707Z 
2019-07-10T00:04:39.2221931Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-10T00:04:42.2322395Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-10T00:04:42.2323134Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-10T00:04:42.2323357Z 
2019-07-10T00:04:42.2323357Z 
2019-07-10T00:04:42.2324468Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-10T00:04:46.2397793Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-10T00:04:46.2398224Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-10T00:04:46.2398500Z 
2019-07-10T00:04:46.2398500Z 
2019-07-10T00:04:46.2441236Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-10T00:04:46.2445640Z The command has failed after 5 attempts.
2019-07-10T00:04:46.3820380Z open /tmp/rustci_docker_cache: no such file or directory
2019-07-10T00:04:46.3865725Z Attempting with retry: docker build --rm -t rust-ci -f /home/vsts/work/1/s/src/ci/docker/x86_64-gnu-llvm-6.0/Dockerfile /home/vsts/work/1/s/src/ci/docker
2019-07-10T00:04:46.5184903Z Sending build context to Docker daemon  521.7kB
2019-07-10T00:04:46.5185609Z 
2019-07-10T00:04:46.5424522Z Step 1/8 : FROM ubuntu:16.04
---
2019-07-10T00:05:01.8544122Z Reading package lists...
2019-07-10T00:05:02.9861549Z Reading package lists...
2019-07-10T00:05:03.2286875Z Building dependency tree...
2019-07-10T00:05:03.2287717Z Reading state information...
2019-07-10T00:05:03.3570859Z The following additional packages will be installed:
2019-07-10T00:05:03.3572626Z   binfmt-support binutils bzip2 cmake-data cpp cpp-5 dpkg-dev g++-5 gcc gcc-5
2019-07-10T00:05:03.3573387Z   git-man libarchive13 libasan2 libasn1-8-heimdal libatomic1
2019-07-10T00:05:03.3573989Z   libbabeltrace-ctf1 libbabeltrace1 libbsd-dev libbsd0 libbz2-1.0 libc-dev-bin
2019-07-10T00:05:03.3596817Z   libedit2 liberror-perl libexpat1 libffi-dev libffi6 libgcc-5-dev libgdbm3
2019-07-10T00:05:03.3597559Z   libglib2.0-0 libgmp10 libgnutls30 libgomp1 libgssapi-krb5-2
2019-07-10T00:05:03.3598155Z   libgssapi3-heimdal libhcrypto4-heimdal libheimbase1-heimdal
2019-07-10T00:05:03.3598743Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-10T00:05:03.3598743Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-10T00:05:03.3600685Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-10T00:05:03.3601432Z   libkrb5-3 libkrb5support0 libldap-2.4-2 libllvm6.0 liblsan0 liblzo2-2
2019-07-10T00:05:03.3602787Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-10T00:05:03.3603397Z   libperl5.22 libpipeline1 libpython-stdlib libpython2.7-minimal
2019-07-10T00:05:03.3604341Z   libpython2.7-stdlib libpython3.5 libpython3.5-minimal libpython3.5-stdlib
2019-07-10T00:05:03.3605889Z   libquadmath0 libroken18-heimdal librtmp1 libsasl2-2 libsasl2-modules-db
2019-07-10T00:05:03.3607248Z   libsqlite3-0 libssl1.0.0 libstdc++-5-dev libtasn1-6 libtinfo-dev libtsan0
2019-07-10T00:05:03.3608634Z   libubsan0 libwind0-heimdal libxml2 linux-libc-dev llvm-6.0 llvm-6.0-dev
2019-07-10T00:05:03.3609891Z   llvm-6.0-runtime mime-support openssl patch perl perl-modules-5.22 python
2019-07-10T00:05:03.3612020Z   python-minimal python2.7-minimal
2019-07-10T00:05:03.3612330Z Suggested packages:
2019-07-10T00:05:03.3613352Z   binutils-doc bzip2-doc codeblocks eclipse ninja-build cpp-doc gcc-5-locales
2019-07-10T00:05:03.3614015Z   debian-keyring g++-multilib g++-5-multilib gcc-5-doc libstdc++6-5-dbg
2019-07-10T00:05:03.3614443Z   gcc-multilib manpages-dev autoconf automake libtool flex bison gcc-doc
2019-07-10T00:05:03.3615824Z   libasan2-dbg liblsan0-dbg libtsan0-dbg libubsan0-dbg libcilkrts5-dbg
2019-07-10T00:05:03.3616375Z   libmpx0-dbg libquadmath0-dbg gdb-doc gettext-base git-daemon-run
2019-07-10T00:05:03.3616375Z   libmpx0-dbg libquadmath0-dbg gdb-doc gettext-base git-daemon-run
2019-07-10T00:05:03.3617230Z   | git-daemon-sysvinit git-doc git-el git-email git-gui gitk gitweb git-arch
2019-07-10T00:05:03.3617968Z   git-cvs git-mediawiki git-svn lrzip glibc-doc gnutls-bin krb5-doc krb5-user
2019-07-10T00:05:03.3619269Z   libstdc++-5-doc llvm-6.0-doc make-doc ed diffutils-doc perl-doc
2019-07-10T00:05:03.3619784Z   libterm-readline-gnu-perl | libterm-readline-perl-perl python-doc python-tk
2019-07-10T00:05:03.3621780Z   python2.7-doc
2019-07-10T00:05:03.3622101Z Recommended packages:
2019-07-10T00:05:03.3622501Z   build-essential fakeroot libalgorithm-merge-perl libc-dbg gdbserver less
2019-07-10T00:05:03.3622903Z   rsync ssh-client manpages manpages-dev libfile-fcntllock-perl
2019-07-10T00:05:03.3623379Z   libglib2.0-data shared-mime-info xdg-user-dirs krb5-locales libsasl2-modules
2019-07-10T00:05:03.3623746Z   libssl-doc xml-core netbase rename
2019-07-10T00:05:03.3624110Z The following NEW packages will be installed:
2019-07-10T00:05:03.3624705Z   binfmt-support binutils bzip2 ca-certificates cmake cmake-data cpp cpp-5
2019-07-10T00:05:03.3625118Z   curl dpkg-dev file g++ g++-5 gcc gcc-5 gdb git git-man libarchive13 libasan2
2019-07-10T00:05:03.3625610Z   libasn1-8-heimdal libatomic1 libbabeltrace-ctf1 libbabeltrace1 libbsd-dev
2019-07-10T00:05:03.3626178Z   libbsd0 libc-dev-bin libc6-dev libcc1-0 libcilkrts5 libcurl3 libcurl3-gnutls
2019-07-10T00:05:03.3627215Z   libdpkg-perl libedit-dev libedit2 liberror-perl libexpat1 libffi-dev libffi6
2019-07-10T00:05:03.3628267Z   libgcc-5-dev libgdbm3 libglib2.0-0 libgmp10 libgnutls30 libgomp1
2019-07-10T00:05:03.3629157Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-10T00:05:03.3629730Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-10T00:05:03.3630120Z   libkrb5-3 libkrb5support0 libldap-2.4-2 libllvm6.0 liblsan0 liblzo2-2
2019-07-10T00:05:03.3631019Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-10T00:05:03.3631019Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-10T00:05:03.3631491Z   libperl5.22 libpipeline1 libpython-stdlib libpython2.7-minimal
2019-07-10T00:05:03.3631956Z   libpython2.7-stdlib libpython3.5 libpython3.5-minimal libpython3.5-stdlib
2019-07-10T00:05:03.3632394Z   libquadmath0 libroken18-heimdal librtmp1 libsasl2-2 libsasl2-modules-db
2019-07-10T00:05:03.3632816Z   libsqlite3-0 libssl-dev libssl1.0.0 libstdc++-5-dev libtasn1-6 libtinfo-dev
2019-07-10T00:05:03.3633282Z   libtsan0 libubsan0 libwind0-heimdal libxml2 linux-libc-dev llvm-6.0
2019-07-10T00:05:03.3633702Z   llvm-6.0-dev llvm-6.0-runtime llvm-6.0-tools make mime-support openssl patch
2019-07-10T00:05:03.3634259Z   perl perl-modules-5.22 pkg-config python python-minimal python2.7
2019-07-10T00:05:03.3634679Z   python2.7-minimal sudo xz-utils zlib1g-dev
2019-07-10T00:05:03.3634863Z The following packages will be upgraded:
2019-07-10T00:05:04.0035525Z 1 upgraded, 115 newly installed, 0 to remove and 4 not upgraded.
2019-07-10T00:05:04.0035724Z Need to get 121 MB of archives.
2019-07-10T00:05:04.0036255Z After this operation, 592 MB of additional disk space will be used.
2019-07-10T00:05:04.0037148Z Get:1 http://archive.ubuntu.com/ubuntu xenial/main amd64 libgdbm3 amd64 1.8.3-13.1 [16.9 kB]
---
2019-07-10T00:05:06.4584873Z Get:61 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 sudo amd64 1.8.16-0ubuntu1.7 [390 kB]
2019-07-10T00:05:06.5228859Z Get:62 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 openssl amd64 1.0.2g-1ubuntu4.15 [492 kB]
2019-07-10T00:05:06.5306912Z Get:63 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 ca-certificates all 20170717~16.04.2 [167 kB]
2019-07-10T00:05:06.5336074Z Get:64 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 libcurl3-gnutls amd64 7.47.0-1ubuntu2.13 [184 kB]
2019-07-10T00:05:06.5365260Z Get:65 http://archive.ubuntu.com/ubuntu xenial/main amd64 libedit2 amd64 3.1-20150325-1ubuntu2 [76.5 kB]
2019-07-10T00:05:06.5381228Z Get:66 http://archive.ubuntu.com/ubuntu xenial/main amd64 libpipeline1 amd64 1.4.1-2 [24.6 kB]
2019-07-10T00:05:06.5399075Z Get:67 http://archive.ubuntu.com/ubuntu xenial/main amd64 binfmt-support amd64 2.1.6-1 [50.7 kB]
2019-07-10T00:05:06.6647415Z Get:69 http://archive.ubuntu.com/ubuntu xenial/main amd64 libisl15 amd64 0.16.1-1 [524 kB]
2019-07-10T00:05:06.6719909Z Get:70 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 cpp-5 amd64 5.4.0-6ubuntu1~16.04.11 [7660 kB]
2019-07-10T00:05:06.9951501Z Get:71 http://archive.ubuntu.com/ubuntu xenial/main amd64 cpp amd64 4:5.3.1-1ubuntu1 [27.7 kB]
2019-07-10T00:05:06.9958047Z Get:72 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 curl amd64 7.47.0-1ubuntu2.13 [139 kB]
---
2019-07-10T00:05:27.0195116Z Unpacking ca-certificates (20170717~16.04.2) ...
2019-07-10T00:05:27.1877957Z Selecting previously unselected package libcurl3-gnutls:amd64.
2019-07-10T00:05:27.1894384Z Preparing to unpack .../libcurl3-gnutls_7.47.0-1ubuntu2.13_amd64.deb ...
2019-07-10T00:05:27.2006428Z Unpacking libcurl3-gnutls:amd64 (7.47.0-1ubuntu2.13) ...
2019-07-10T00:05:27.3259880Z Selecting previously unselected package libedit2:amd64.
2019-07-10T00:05:27.3278024Z Preparing to unpack .../libedit2_3.1-20150325-1ubuntu2_amd64.deb ...
2019-07-10T00:05:27.3390629Z Unpacking libedit2:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-10T00:05:27.4523005Z Selecting previously unselected package libpipeline1:amd64.
2019-07-10T00:05:27.4539347Z Preparing to unpack .../libpipeline1_1.4.1-2_amd64.deb ...
2019-07-10T00:05:27.4672620Z Unpacking libpipeline1:amd64 (1.4.1-2) ...
2019-07-10T00:05:27.5953875Z Selecting previously unselected package binfmt-support.
2019-07-10T00:05:27.5970193Z Preparing to unpack .../binfmt-support_2.1.6-1_amd64.deb ...
2019-07-10T00:05:27.6086006Z Unpacking binfmt-support (2.1.6-1) ...
2019-07-10T00:05:27.7136213Z Preparing to unpack .../binutils_2.26.1-1ubuntu1~16.04.8_amd64.deb ...
2019-07-10T00:05:27.7255063Z Unpacking binutils (2.26.1-1ubuntu1~16.04.8) ...
2019-07-10T00:05:28.2130503Z Selecting previously unselected package libisl15:amd64.
2019-07-10T00:05:28.2143257Z Preparing to unpack .../libisl15_0.16.1-1_amd64.deb ...
---
2019-07-10T00:05:39.9383670Z Selecting previously unselected package libtinfo-dev:amd64.
2019-07-10T00:05:39.9403716Z Preparing to unpack .../libtinfo-dev_6.0+20160213-1ubuntu1_amd64.deb ...
2019-07-10T00:05:39.9545931Z Unpacking libtinfo-dev:amd64 (6.0+20160213-1ubuntu1) ...
2019-07-10T00:05:40.0544636Z Selecting previously unselected package libedit-dev:amd64.
2019-07-10T00:05:40.0563661Z Preparing to unpack .../libedit-dev_3.1-20150325-1ubuntu2_amd64.deb ...
2019-07-10T00:05:40.0707258Z Unpacking libedit-dev:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-10T00:05:40.2064059Z Selecting previously unselected package libllvm6.0:amd64.
2019-07-10T00:05:40.2082454Z Preparing to unpack .../libllvm6.0_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-10T00:05:40.2187758Z Unpacking libllvm6.0:amd64 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-10T00:05:43.2965536Z Selecting previously unselected package zlib1g-dev:amd64.
2019-07-10T00:05:43.3002686Z Preparing to unpack .../zlib1g-dev_1%3a1.2.8.dfsg-2ubuntu4.1_amd64.deb ...
2019-07-10T00:05:43.3113179Z Unpacking zlib1g-dev:amd64 (1:1.2.8.dfsg-2ubuntu4.1) ...
2019-07-10T00:05:43.4149540Z Preparing to unpack .../libssl-dev_1.0.2g-1ubuntu4.15_amd64.deb ...
2019-07-10T00:05:43.4307242Z Unpacking libssl-dev:amd64 (1.0.2g-1ubuntu4.15) ...
2019-07-10T00:05:43.7529658Z Selecting previously unselected package llvm-6.0-runtime.
2019-07-10T00:05:43.7529658Z Selecting previously unselected package llvm-6.0-runtime.
2019-07-10T00:05:43.7530097Z Preparing to unpack .../llvm-6.0-runtime_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-10T00:05:43.7633957Z Unpacking llvm-6.0-runtime (1:6.0-1ubuntu2~16.04.1) ...
2019-07-10T00:05:43.8737934Z Selecting previously unselected package llvm-6.0.
2019-07-10T00:05:43.8753865Z Preparing to unpack .../llvm-6.0_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-10T00:05:43.8923378Z Unpacking llvm-6.0 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-10T00:05:44.5227184Z Selecting previously unselected package libffi-dev:amd64.
2019-07-10T00:05:44.5247360Z Preparing to unpack .../libffi-dev_3.2.1-4_amd64.deb ...
2019-07-10T00:05:44.5360188Z Unpacking libffi-dev:amd64 (3.2.1-4) ...
2019-07-10T00:05:44.6462721Z Selecting previously unselected package llvm-6.0-dev.
2019-07-10T00:05:44.6482304Z Preparing to unpack .../llvm-6.0-dev_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-10T00:05:44.6586697Z Unpacking llvm-6.0-dev (1:6.0-1ubuntu2~16.04.1) ...
2019-07-10T00:05:49.4517316Z Selecting previously unselected package llvm-6.0-tools.
2019-07-10T00:05:49.4541791Z Preparing to unpack .../llvm-6.0-tools_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-10T00:05:49.4656100Z Unpacking llvm-6.0-tools (1:6.0-1ubuntu2~16.04.1) ...
2019-07-10T00:05:49.6091027Z Selecting previously unselected package pkg-config.
2019-07-10T00:05:49.6117703Z Preparing to unpack .../pkg-config_0.29.1-0ubuntu1_amd64.deb ...
2019-07-10T00:05:49.6250742Z Unpacking pkg-config (0.29.1-0ubuntu1) ...
2019-07-10T00:05:49.7358338Z Processing triggers for systemd (229-4ubuntu21.21) ...
2019-07-10T00:05:50.0414254Z Setting up libgdbm3:amd64 (1.8.3-13.1) ...
2019-07-10T00:05:50.1199092Z Setting up libffi6:amd64 (3.2.1-4) ...
2019-07-10T00:05:50.1540910Z Setting up libglib2.0-0:amd64 (2.48.2-0ubuntu4.3) ...
---
2019-07-10T00:05:53.7251494Z debconf: unable to initialize frontend: Dialog
2019-07-10T00:05:53.7251683Z debconf: (TERM is not set, so the dialog frontend is not usable.)
2019-07-10T00:05:53.7251778Z debconf: falling back to frontend: Readline
2019-07-10T00:05:54.4405279Z Setting up libcurl3-gnutls:amd64 (7.47.0-1ubuntu2.13) ...
2019-07-10T00:05:54.4738874Z Setting up libedit2:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-10T00:05:54.5075266Z Setting up libpipeline1:amd64 (1.4.1-2) ...
2019-07-10T00:05:54.5397830Z Setting up binfmt-support (2.1.6-1) ...
2019-07-10T00:05:54.6043573Z mount: permission denied
2019-07-10T00:05:54.6044522Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-10T00:05:54.6057131Z mount: permission denied
2019-07-10T00:05:54.6062357Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-10T00:05:54.7698921Z invoke-rc.d: could not determine current runlevel
2019-07-10T00:05:54.7730333Z invoke-rc.d: policy-rc.d denied execution of start.
2019-07-10T00:05:54.8275273Z Setting up libisl15:amd64 (0.16.1-1) ...
2019-07-10T00:05:54.8677380Z Setting up cpp-5 (5.4.0-6ubuntu1~16.04.11) ...
2019-07-10T00:05:54.9026401Z Setting up cpp (4:5.3.1-1ubuntu1) ...
2019-07-10T00:05:54.9446418Z Setting up curl (7.47.0-1ubuntu2.13) ...
---
2019-07-10T00:05:56.3654052Z Setting up libedit-dev:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-10T00:05:56.3985056Z Setting up libllvm6.0:amd64 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-10T00:05:56.4402423Z Setting up zlib1g-dev:amd64 (1:1.2.8.dfsg-2ubuntu4.1) ...
2019-07-10T00:05:56.4865643Z Setting up libssl-dev:amd64 (1.0.2g-1ubuntu4.15) ...
2019-07-10T00:05:56.5208151Z Setting up llvm-6.0-runtime (1:6.0-1ubuntu2~16.04.1) ...
2019-07-10T00:05:56.5449921Z mount: permission denied
2019-07-10T00:05:56.5454097Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-10T00:05:56.5566115Z Setting up llvm-6.0 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-10T00:05:56.5891497Z Setting up libffi-dev:amd64 (3.2.1-4) ...
2019-07-10T00:05:56.6245543Z Setting up llvm-6.0-dev (1:6.0-1ubuntu2~16.04.1) ...
2019-07-10T00:05:56.6603067Z Setting up llvm-6.0-tools (1:6.0-1ubuntu2~16.04.1) ...
2019-07-10T00:05:56.6933693Z Setting up pkg-config (0.29.1-0ubuntu1) ...
2019-07-10T00:05:56.8438612Z Processing triggers for ca-certificates (20170717~16.04.2) ...
2019-07-10T00:05:56.8591525Z Updating certificates in /etc/ssl/certs...
2019-07-10T00:05:58.5505682Z 148 added, 0 removed; done.
2019-07-10T00:05:58.5507807Z Running hooks in /etc/ca-certificates/update.d...
---
2019-07-10T00:06:32.4673012Z Removing intermediate container d49966615bdd
2019-07-10T00:06:32.4674544Z  ---> c8c03bce2cb5
2019-07-10T00:06:32.4715769Z Successfully built c8c03bce2cb5
2019-07-10T00:06:32.6452011Z Successfully tagged rust-ci:latest
2019-07-10T00:06:32.7062671Z Built container sha256:c8c03bce2cb5a9551f4df3d23fc9aea3c7462412a2464cad1caca8ca9c7fdc8e
2019-07-10T00:06:32.7083670Z Uploading finished image to https://.s3.amazonaws.com/docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624
2019-07-10T00:07:37.8431214Z upload failed: - to s3:///docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624 Parameter validation failed:
2019-07-10T00:07:37.8431734Z Invalid bucket name "": Bucket name must match the regex "^[a-zA-Z0-9.\-_]{1,255}$"
2019-07-10T00:07:38.8359880Z [CI_JOB_NAME=Job1]
2019-07-10T00:07:38.8423772Z Starting sccache server...
2019-07-10T00:07:38.9049898Z configure: processing command line
2019-07-10T00:07:38.9050014Z configure: 
---
2019-07-10T00:11:28.0610598Z    Compiling serde_json v1.0.33
2019-07-10T00:11:32.6384538Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-07-10T00:11:42.0467791Z     Finished release [optimized] target(s) in 1m 39s
2019-07-10T00:11:42.0532906Z tidy check
2019-07-10T00:11:42.3447265Z tidy error: /checkout/src/librustc_mir/dataflow/impls/reaching_defs.rs:172: TODO is deprecated; use FIXME
2019-07-10T00:11:43.9649274Z some tidy checks failed
2019-07-10T00:11:43.9650322Z 
2019-07-10T00:11:43.9650322Z 
2019-07-10T00:11:43.9651665Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-07-10T00:11:43.9652240Z 
2019-07-10T00:11:43.9652358Z 
2019-07-10T00:11:43.9652529Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-07-10T00:11:43.9653202Z Build completed unsuccessfully in 0:01:42
2019-07-10T00:11:43.9653202Z Build completed unsuccessfully in 0:01:42
2019-07-10T00:11:45.3192603Z ##[error]Bash exited with code '1'.
2019-07-10T00:11:45.3239915Z ##[section]Starting: Checkout
2019-07-10T00:11:45.3241548Z ==============================================================================
2019-07-10T00:11:45.3241602Z Task         : Get sources
2019-07-10T00:11:45.3242049Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
