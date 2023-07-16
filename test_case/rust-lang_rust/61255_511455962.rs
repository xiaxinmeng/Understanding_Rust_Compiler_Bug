plain
2019-07-15T14:37:22.9517452Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-15T14:37:22.9756154Z ##[command]git config gc.auto 0
2019-07-15T14:37:22.9820073Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-15T14:37:22.9867720Z ##[command]git config --get-all http.proxy
2019-07-15T14:37:23.0005054Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/61255/merge:refs/remotes/pull/61255/merge
---
2019-07-15T14:37:57.9003102Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-15T14:37:57.9004006Z 
2019-07-15T14:37:57.9005130Z   git checkout -b <new-branch-name>
2019-07-15T14:37:57.9006020Z 
2019-07-15T14:37:57.9006883Z HEAD is now at c3963640c Merge d94b2ff83a9cdd7a5768ee9ec0e3da2d220508f7 into 5480b47d7f9e708300d3ba319869f21cd1ffd487
2019-07-15T14:37:57.9130337Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-15T14:37:57.9133331Z ==============================================================================
2019-07-15T14:37:57.9133387Z Task         : Bash
2019-07-15T14:37:57.9133431Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-15T14:39:33.5053102Z Attempting with retry: curl -y 30 -Y 10 --connect-timeout 30 -f -L -C - -o /tmp/rustci_docker_cache https://.s3.amazonaws.com/docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624
2019-07-15T14:39:33.5104500Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-15T14:39:33.5104948Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-15T14:39:33.5105262Z 
2019-07-15T14:39:33.5149322Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-15T14:39:34.5239162Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-15T14:39:34.5239395Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-15T14:39:34.5239430Z 
2019-07-15T14:39:34.5239430Z 
2019-07-15T14:39:34.5240724Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-15T14:39:36.5327927Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-15T14:39:36.5328020Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-15T14:39:36.5328052Z 
2019-07-15T14:39:36.5328052Z 
2019-07-15T14:39:36.5362217Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-15T14:39:39.5428638Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-15T14:39:39.5429188Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-15T14:39:39.5429496Z 
2019-07-15T14:39:39.5429496Z 
2019-07-15T14:39:39.5479324Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-15T14:39:43.5549034Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-15T14:39:43.5549601Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-15T14:39:43.5549675Z 
2019-07-15T14:39:43.5549675Z 
2019-07-15T14:39:43.5592366Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-15T14:39:43.5598223Z The command has failed after 5 attempts.
2019-07-15T14:39:43.6134945Z open /tmp/rustci_docker_cache: no such file or directory
2019-07-15T14:39:43.6160394Z Attempting with retry: docker build --rm -t rust-ci -f /home/vsts/work/1/s/src/ci/docker/x86_64-gnu-llvm-6.0/Dockerfile /home/vsts/work/1/s/src/ci/docker
2019-07-15T14:39:43.7901610Z Sending build context to Docker daemon  521.7kB
2019-07-15T14:39:43.8116160Z 
2019-07-15T14:39:43.8116377Z Step 1/8 : FROM ubuntu:16.04
---
2019-07-15T14:39:59.5201566Z Reading package lists...
2019-07-15T14:40:00.5429843Z Reading package lists...
2019-07-15T14:40:00.7439171Z Building dependency tree...
2019-07-15T14:40:00.7439284Z Reading state information...
2019-07-15T14:40:00.8782675Z The following additional packages will be installed:
2019-07-15T14:40:00.8783934Z   binfmt-support binutils bzip2 cmake-data cpp cpp-5 dpkg-dev g++-5 gcc gcc-5
2019-07-15T14:40:00.8784480Z   git-man libarchive13 libasan2 libasn1-8-heimdal libatomic1
2019-07-15T14:40:00.8784828Z   libbabeltrace-ctf1 libbabeltrace1 libbsd-dev libbsd0 libbz2-1.0 libc-dev-bin
2019-07-15T14:40:00.8785304Z   libedit2 liberror-perl libexpat1 libffi-dev libffi6 libgcc-5-dev libgdbm3
2019-07-15T14:40:00.8785581Z   libglib2.0-0 libgmp10 libgnutls30 libgomp1 libgssapi-krb5-2
2019-07-15T14:40:00.8785804Z   libgssapi3-heimdal libhcrypto4-heimdal libheimbase1-heimdal
2019-07-15T14:40:00.8786087Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-15T14:40:00.8786087Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-15T14:40:00.8786374Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-15T14:40:00.8786609Z   libkrb5-3 libkrb5support0 libldap-2.4-2 libllvm6.0 liblsan0 liblzo2-2
2019-07-15T14:40:00.8786841Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-15T14:40:00.8787113Z   libperl5.22 libpipeline1 libpython-stdlib libpython2.7-minimal
2019-07-15T14:40:00.8787351Z   libpython2.7-stdlib libpython3.5 libpython3.5-minimal libpython3.5-stdlib
2019-07-15T14:40:00.8787587Z   libquadmath0 libroken18-heimdal librtmp1 libsasl2-2 libsasl2-modules-db
2019-07-15T14:40:00.8788034Z   libsqlite3-0 libssl1.0.0 libstdc++-5-dev libtasn1-6 libtinfo-dev libtsan0
2019-07-15T14:40:00.8788426Z   libubsan0 libwind0-heimdal libxml2 linux-libc-dev llvm-6.0 llvm-6.0-dev
2019-07-15T14:40:00.8788646Z   llvm-6.0-runtime mime-support openssl patch perl perl-modules-5.22 python
2019-07-15T14:40:00.8789224Z   python-minimal python2.7-minimal
2019-07-15T14:40:00.8789279Z Suggested packages:
2019-07-15T14:40:00.8789500Z   binutils-doc bzip2-doc codeblocks eclipse ninja-build cpp-doc gcc-5-locales
2019-07-15T14:40:00.8789759Z   debian-keyring g++-multilib g++-5-multilib gcc-5-doc libstdc++6-5-dbg
2019-07-15T14:40:00.8789974Z   gcc-multilib manpages-dev autoconf automake libtool flex bison gcc-doc
2019-07-15T14:40:00.8790944Z   libasan2-dbg liblsan0-dbg libtsan0-dbg libubsan0-dbg libcilkrts5-dbg
2019-07-15T14:40:00.8791189Z   libmpx0-dbg libquadmath0-dbg gdb-doc gettext-base git-daemon-run
2019-07-15T14:40:00.8791189Z   libmpx0-dbg libquadmath0-dbg gdb-doc gettext-base git-daemon-run
2019-07-15T14:40:00.8791441Z   | git-daemon-sysvinit git-doc git-el git-email git-gui gitk gitweb git-arch
2019-07-15T14:40:00.8791736Z   git-cvs git-mediawiki git-svn lrzip glibc-doc gnutls-bin krb5-doc krb5-user
2019-07-15T14:40:00.8791988Z   libstdc++-5-doc llvm-6.0-doc make-doc ed diffutils-doc perl-doc
2019-07-15T14:40:00.8792263Z   libterm-readline-gnu-perl | libterm-readline-perl-perl python-doc python-tk
2019-07-15T14:40:00.8792503Z   python2.7-doc
2019-07-15T14:40:00.8792553Z Recommended packages:
2019-07-15T14:40:00.8792804Z   build-essential fakeroot libalgorithm-merge-perl libc-dbg gdbserver less
2019-07-15T14:40:00.8793087Z   rsync ssh-client manpages manpages-dev libfile-fcntllock-perl
2019-07-15T14:40:00.8793348Z   libglib2.0-data shared-mime-info xdg-user-dirs krb5-locales libsasl2-modules
2019-07-15T14:40:00.8793723Z   libssl-doc xml-core netbase rename
2019-07-15T14:40:00.8793808Z The following NEW packages will be installed:
2019-07-15T14:40:00.8794071Z   binfmt-support binutils bzip2 ca-certificates cmake cmake-data cpp cpp-5
2019-07-15T14:40:00.8795546Z   curl dpkg-dev file g++ g++-5 gcc gcc-5 gdb git git-man libarchive13 libasan2
2019-07-15T14:40:00.8795958Z   libasn1-8-heimdal libatomic1 libbabeltrace-ctf1 libbabeltrace1 libbsd-dev
2019-07-15T14:40:00.8796222Z   libbsd0 libc-dev-bin libc6-dev libcc1-0 libcilkrts5 libcurl3 libcurl3-gnutls
2019-07-15T14:40:00.8796609Z   libdpkg-perl libedit-dev libedit2 liberror-perl libexpat1 libffi-dev libffi6
2019-07-15T14:40:00.8796932Z   libgcc-5-dev libgdbm3 libglib2.0-0 libgmp10 libgnutls30 libgomp1
2019-07-15T14:40:00.8797401Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-15T14:40:00.8797882Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-15T14:40:00.8798125Z   libkrb5-3 libkrb5support0 libldap-2.4-2 libllvm6.0 liblsan0 liblzo2-2
2019-07-15T14:40:00.8798519Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-15T14:40:00.8798519Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-15T14:40:00.8798776Z   libperl5.22 libpipeline1 libpython-stdlib libpython2.7-minimal
2019-07-15T14:40:00.8799012Z   libpython2.7-stdlib libpython3.5 libpython3.5-minimal libpython3.5-stdlib
2019-07-15T14:40:00.8799251Z   libquadmath0 libroken18-heimdal librtmp1 libsasl2-2 libsasl2-modules-db
2019-07-15T14:40:00.8799525Z   libsqlite3-0 libssl-dev libssl1.0.0 libstdc++-5-dev libtasn1-6 libtinfo-dev
2019-07-15T14:40:00.8799946Z   libtsan0 libubsan0 libwind0-heimdal libxml2 linux-libc-dev llvm-6.0
2019-07-15T14:40:00.8800758Z   llvm-6.0-dev llvm-6.0-runtime llvm-6.0-tools make mime-support openssl patch
2019-07-15T14:40:00.8801064Z   perl perl-modules-5.22 pkg-config python python-minimal python2.7
2019-07-15T14:40:00.8801298Z   python2.7-minimal sudo xz-utils zlib1g-dev
2019-07-15T14:40:00.8801353Z The following packages will be upgraded:
2019-07-15T14:40:01.1479833Z 1 upgraded, 115 newly installed, 0 to remove and 4 not upgraded.
2019-07-15T14:40:01.1480054Z Need to get 121 MB of archives.
2019-07-15T14:40:01.1480109Z After this operation, 592 MB of additional disk space will be used.
2019-07-15T14:40:01.1480915Z Get:1 http://archive.ubuntu.com/ubuntu xenial/main amd64 libgdbm3 amd64 1.8.3-13.1 [16.9 kB]
---
2019-07-15T14:40:02.2338740Z Get:61 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 sudo amd64 1.8.16-0ubuntu1.7 [390 kB]
2019-07-15T14:40:02.2401923Z Get:62 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 openssl amd64 1.0.2g-1ubuntu4.15 [492 kB]
2019-07-15T14:40:02.2475864Z Get:63 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 ca-certificates all 20170717~16.04.2 [167 kB]
2019-07-15T14:40:02.2502199Z Get:64 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 libcurl3-gnutls amd64 7.47.0-1ubuntu2.13 [184 kB]
2019-07-15T14:40:02.2528140Z Get:65 http://archive.ubuntu.com/ubuntu xenial/main amd64 libedit2 amd64 3.1-20150325-1ubuntu2 [76.5 kB]
2019-07-15T14:40:02.2546129Z Get:66 http://archive.ubuntu.com/ubuntu xenial/main amd64 libpipeline1 amd64 1.4.1-2 [24.6 kB]
2019-07-15T14:40:02.2546444Z Get:67 http://archive.ubuntu.com/ubuntu xenial/main amd64 binfmt-support amd64 2.1.6-1 [50.7 kB]
2019-07-15T14:40:02.3233545Z Get:69 http://archive.ubuntu.com/ubuntu xenial/main amd64 libisl15 amd64 0.16.1-1 [524 kB]
2019-07-15T14:40:02.3311339Z Get:70 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 cpp-5 amd64 5.4.0-6ubuntu1~16.04.11 [7660 kB]
2019-07-15T14:40:02.4463512Z Get:71 http://archive.ubuntu.com/ubuntu xenial/main amd64 cpp amd64 4:5.3.1-1ubuntu1 [27.7 kB]
2019-07-15T14:40:02.4472072Z Get:72 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 curl amd64 7.47.0-1ubuntu2.13 [139 kB]
---
2019-07-15T14:40:21.1543388Z Unpacking ca-certificates (20170717~16.04.2) ...
2019-07-15T14:40:21.3019436Z Selecting previously unselected package libcurl3-gnutls:amd64.
2019-07-15T14:40:21.3034728Z Preparing to unpack .../libcurl3-gnutls_7.47.0-1ubuntu2.13_amd64.deb ...
2019-07-15T14:40:21.3196197Z Unpacking libcurl3-gnutls:amd64 (7.47.0-1ubuntu2.13) ...
2019-07-15T14:40:21.4407825Z Selecting previously unselected package libedit2:amd64.
2019-07-15T14:40:21.4424864Z Preparing to unpack .../libedit2_3.1-20150325-1ubuntu2_amd64.deb ...
2019-07-15T14:40:21.4577048Z Unpacking libedit2:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-15T14:40:21.5654821Z Selecting previously unselected package libpipeline1:amd64.
2019-07-15T14:40:21.5673756Z Preparing to unpack .../libpipeline1_1.4.1-2_amd64.deb ...
2019-07-15T14:40:21.5809472Z Unpacking libpipeline1:amd64 (1.4.1-2) ...
2019-07-15T14:40:21.6820267Z Selecting previously unselected package binfmt-support.
2019-07-15T14:40:21.6834273Z Preparing to unpack .../binfmt-support_2.1.6-1_amd64.deb ...
2019-07-15T14:40:21.6949563Z Unpacking binfmt-support (2.1.6-1) ...
2019-07-15T14:40:21.7986012Z Preparing to unpack .../binutils_2.26.1-1ubuntu1~16.04.8_amd64.deb ...
2019-07-15T14:40:21.8118027Z Unpacking binutils (2.26.1-1ubuntu1~16.04.8) ...
2019-07-15T14:40:22.2773097Z Selecting previously unselected package libisl15:amd64.
2019-07-15T14:40:22.2789743Z Preparing to unpack .../libisl15_0.16.1-1_amd64.deb ...
---
2019-07-15T14:40:33.6787810Z Selecting previously unselected package libtinfo-dev:amd64.
2019-07-15T14:40:33.6806021Z Preparing to unpack .../libtinfo-dev_6.0+20160213-1ubuntu1_amd64.deb ...
2019-07-15T14:40:33.6919733Z Unpacking libtinfo-dev:amd64 (6.0+20160213-1ubuntu1) ...
2019-07-15T14:40:33.7774285Z Selecting previously unselected package libedit-dev:amd64.
2019-07-15T14:40:33.7795809Z Preparing to unpack .../libedit-dev_3.1-20150325-1ubuntu2_amd64.deb ...
2019-07-15T14:40:33.7911881Z Unpacking libedit-dev:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-15T14:40:33.8964201Z Selecting previously unselected package libllvm6.0:amd64.
2019-07-15T14:40:33.8988286Z Preparing to unpack .../libllvm6.0_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-15T14:40:33.9104774Z Unpacking libllvm6.0:amd64 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-15T14:40:36.8486353Z Selecting previously unselected package zlib1g-dev:amd64.
2019-07-15T14:40:36.8499167Z Preparing to unpack .../zlib1g-dev_1%3a1.2.8.dfsg-2ubuntu4.1_amd64.deb ...
2019-07-15T14:40:36.8611110Z Unpacking zlib1g-dev:amd64 (1:1.2.8.dfsg-2ubuntu4.1) ...
2019-07-15T14:40:36.9548119Z Preparing to unpack .../libssl-dev_1.0.2g-1ubuntu4.15_amd64.deb ...
2019-07-15T14:40:36.9660153Z Unpacking libssl-dev:amd64 (1.0.2g-1ubuntu4.15) ...
2019-07-15T14:40:37.2746712Z Selecting previously unselected package llvm-6.0-runtime.
2019-07-15T14:40:37.2746712Z Selecting previously unselected package llvm-6.0-runtime.
2019-07-15T14:40:37.2770711Z Preparing to unpack .../llvm-6.0-runtime_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-15T14:40:37.2877780Z Unpacking llvm-6.0-runtime (1:6.0-1ubuntu2~16.04.1) ...
2019-07-15T14:40:37.4053217Z Selecting previously unselected package llvm-6.0.
2019-07-15T14:40:37.4073379Z Preparing to unpack .../llvm-6.0_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-15T14:40:37.4237414Z Unpacking llvm-6.0 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-15T14:40:38.0723072Z Selecting previously unselected package libffi-dev:amd64.
2019-07-15T14:40:38.0760596Z Preparing to unpack .../libffi-dev_3.2.1-4_amd64.deb ...
2019-07-15T14:40:38.0875301Z Unpacking libffi-dev:amd64 (3.2.1-4) ...
2019-07-15T14:40:38.1968637Z Selecting previously unselected package llvm-6.0-dev.
2019-07-15T14:40:38.1988502Z Preparing to unpack .../llvm-6.0-dev_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-15T14:40:38.2109211Z Unpacking llvm-6.0-dev (1:6.0-1ubuntu2~16.04.1) ...
2019-07-15T14:40:42.8550121Z Selecting previously unselected package llvm-6.0-tools.
2019-07-15T14:40:42.8577577Z Preparing to unpack .../llvm-6.0-tools_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-15T14:40:42.8700488Z Unpacking llvm-6.0-tools (1:6.0-1ubuntu2~16.04.1) ...
2019-07-15T14:40:43.0089957Z Selecting previously unselected package pkg-config.
2019-07-15T14:40:43.0108493Z Preparing to unpack .../pkg-config_0.29.1-0ubuntu1_amd64.deb ...
2019-07-15T14:40:43.0225700Z Unpacking pkg-config (0.29.1-0ubuntu1) ...
2019-07-15T14:40:43.1236218Z Processing triggers for systemd (229-4ubuntu21.21) ...
2019-07-15T14:40:43.4590121Z Setting up libgdbm3:amd64 (1.8.3-13.1) ...
2019-07-15T14:40:43.5197876Z Setting up libffi6:amd64 (3.2.1-4) ...
2019-07-15T14:40:43.5547733Z Setting up libglib2.0-0:amd64 (2.48.2-0ubuntu4.3) ...
---
2019-07-15T14:40:47.1201426Z debconf: unable to initialize frontend: Dialog
2019-07-15T14:40:47.1201806Z debconf: (TERM is not set, so the dialog frontend is not usable.)
2019-07-15T14:40:47.1207760Z debconf: falling back to frontend: Readline
2019-07-15T14:40:47.8787156Z Setting up libcurl3-gnutls:amd64 (7.47.0-1ubuntu2.13) ...
2019-07-15T14:40:47.9123253Z Setting up libedit2:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-15T14:40:47.9457395Z Setting up libpipeline1:amd64 (1.4.1-2) ...
2019-07-15T14:40:47.9798118Z Setting up binfmt-support (2.1.6-1) ...
2019-07-15T14:40:48.0493011Z mount: permission denied
2019-07-15T14:40:48.0493838Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-15T14:40:48.0507822Z mount: permission denied
2019-07-15T14:40:48.0527415Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-15T14:40:48.2159491Z invoke-rc.d: could not determine current runlevel
2019-07-15T14:40:48.2188051Z invoke-rc.d: policy-rc.d denied execution of start.
2019-07-15T14:40:48.2702095Z Setting up libisl15:amd64 (0.16.1-1) ...
2019-07-15T14:40:48.3056315Z Setting up cpp-5 (5.4.0-6ubuntu1~16.04.11) ...
2019-07-15T14:40:48.3450996Z Setting up cpp (4:5.3.1-1ubuntu1) ...
2019-07-15T14:40:48.3895980Z Setting up curl (7.47.0-1ubuntu2.13) ...
---
2019-07-15T14:40:49.8949999Z Setting up libedit-dev:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-15T14:40:49.9305142Z Setting up libllvm6.0:amd64 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-15T14:40:49.9740380Z Setting up zlib1g-dev:amd64 (1:1.2.8.dfsg-2ubuntu4.1) ...
2019-07-15T14:40:50.0084117Z Setting up libssl-dev:amd64 (1.0.2g-1ubuntu4.15) ...
2019-07-15T14:40:50.0503939Z Setting up llvm-6.0-runtime (1:6.0-1ubuntu2~16.04.1) ...
2019-07-15T14:40:50.0790944Z mount: permission denied
2019-07-15T14:40:50.0795328Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-15T14:40:50.0917398Z Setting up llvm-6.0 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-15T14:40:50.1262307Z Setting up libffi-dev:amd64 (3.2.1-4) ...
2019-07-15T14:40:50.1600272Z Setting up llvm-6.0-dev (1:6.0-1ubuntu2~16.04.1) ...
2019-07-15T14:40:50.1983220Z Setting up llvm-6.0-tools (1:6.0-1ubuntu2~16.04.1) ...
2019-07-15T14:40:50.2319308Z Setting up pkg-config (0.29.1-0ubuntu1) ...
2019-07-15T14:40:50.3523620Z Processing triggers for ca-certificates (20170717~16.04.2) ...
2019-07-15T14:40:50.3675978Z Updating certificates in /etc/ssl/certs...
2019-07-15T14:40:51.9453099Z 148 added, 0 removed; done.
2019-07-15T14:40:51.9455334Z Running hooks in /etc/ca-certificates/update.d...
---
2019-07-15T14:41:25.4850362Z  ---> 2d5f384afa41
2019-07-15T14:41:25.4924626Z Successfully built 2d5f384afa41
2019-07-15T14:41:25.6298467Z Successfully tagged rust-ci:latest
2019-07-15T14:41:25.7138879Z Built container sha256:2d5f384afa4121914421c584421c1d4c4bf3b50e04229019bfff3a9e4640e432
2019-07-15T14:41:25.7152191Z Uploading finished image to https://.s3.amazonaws.com/docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624
2019-07-15T14:42:27.2519970Z upload failed: - to s3:///docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624 Parameter validation failed:
2019-07-15T14:42:27.2521681Z Invalid bucket name "": Bucket name must match the regex "^[a-zA-Z0-9.\-_]{1,255}$"
2019-07-15T14:42:28.2754824Z [CI_JOB_NAME=x86_64-gnu-llvm-6.0]
2019-07-15T14:42:28.2807347Z Starting sccache server...
2019-07-15T14:42:28.3309601Z configure: processing command line
2019-07-15T14:42:28.3310503Z configure: 
---
2019-07-15T15:38:47.6207498Z .................................................................................................... 200/5809
2019-07-15T15:38:51.7303649Z .................................................................................................... 300/5809
2019-07-15T15:38:55.3319734Z .................................................................................................... 400/5809
2019-07-15T15:38:59.0690729Z .................................................................................................... 500/5809
2019-07-15T15:39:02.7951436Z ........................................................................i........................... 600/5809
2019-07-15T15:39:12.2546616Z .................................................................................................... 800/5809
2019-07-15T15:39:17.9711505Z .................................................................................................... 900/5809
2019-07-15T15:39:17.9711505Z .................................................................................................... 900/5809
2019-07-15T15:39:24.0617342Z ............................................................................................i....... 1000/5809
2019-07-15T15:39:28.5192626Z ....i............................................................................................... 1100/5809
2019-07-15T15:39:32.8153735Z ......................iiiii......................................................................... 1200/5809
2019-07-15T15:39:38.6692385Z .................................................................................................... 1400/5809
2019-07-15T15:39:41.4569081Z .................................................................................................... 1500/5809
2019-07-15T15:39:45.3913894Z .................................................................................................... 1600/5809
2019-07-15T15:39:47.9246268Z .................................................................................................... 1700/5809
2019-07-15T15:39:47.9246268Z .................................................................................................... 1700/5809
2019-07-15T15:39:51.5107547Z ...........................................................i........................................ 1800/5809
2019-07-15T15:40:00.1316816Z .................................................................................................... 2000/5809
2019-07-15T15:40:04.2109154Z .................................................................................................... 2100/5809
2019-07-15T15:40:08.3785972Z .................................................................................................... 2200/5809
2019-07-15T15:40:08.3785972Z .................................................................................................... 2200/5809
2019-07-15T15:40:11.9787727Z .........................i.......................................................................... 2300/5809
2019-07-15T15:40:21.8775709Z .................................................................................................... 2500/5809
2019-07-15T15:40:26.5212436Z .................................................................................................... 2600/5809
2019-07-15T15:40:30.8989733Z .................................................................................................... 2700/5809
2019-07-15T15:40:35.2769491Z .................................................................................................... 2800/5809
2019-07-15T15:40:35.2769491Z .................................................................................................... 2800/5809
2019-07-15T15:40:39.2913130Z .................................................................................................... 2900/5809
2019-07-15T15:40:44.7419339Z .................................................................................................... 3000/5809
2019-07-15T15:40:49.5099197Z .................................................................................................... 3100/5809
2019-07-15T15:40:54.0383186Z .................................................................................................... 3200/5809
2019-07-15T15:40:57.1429247Z .................................................................................................... 3300/5809
2019-07-15T15:41:02.5228343Z .................................................................................................... 3400/5809
2019-07-15T15:41:06.2639597Z .......................................................................................i............ 3500/5809
2019-07-15T15:41:10.0000834Z .................................................................................................... 3600/5809
2019-07-15T15:41:13.9101417Z .............................................................ii...i..ii............................. 3700/5809
2019-07-15T15:41:22.8437120Z .................................................................................................... 3900/5809
2019-07-15T15:41:22.8437120Z .................................................................................................... 3900/5809
2019-07-15T15:41:26.6361720Z ...........................................................................ii....................... 4000/5809
2019-07-15T15:41:29.3841787Z ................................................................................................i... 4100/5809
2019-07-15T15:41:31.5108462Z .................................................................................................... 4200/5809
2019-07-15T15:41:33.4340438Z .............................................................i...................................... 4300/5809
2019-07-15T15:41:35.9653335Z F................................................................................................... 4400/5809
2019-07-15T15:41:57.4890294Z .................................................................................................... 4600/5809
2019-07-15T15:42:01.0633326Z .................................................................................................... 4700/5809
2019-07-15T15:42:04.5535627Z .................................................................................................... 4800/5809
2019-07-15T15:42:11.0195440Z .................................................................................................... 4900/5809
---
2019-07-15T15:42:38.0421458Z .................................................................................................... 5400/5809
2019-07-15T15:42:42.6407130Z .................................................................................................... 5500/5809
2019-07-15T15:42:46.1989493Z .................................................................................................... 5600/5809
2019-07-15T15:42:49.3658042Z .................................................................................................... 5700/5809
2019-07-15T15:42:52.3240658Z .................................................i.................................................. 5800/5809
2019-07-15T15:42:52.8423591Z failures:
2019-07-15T15:42:52.8463111Z 
2019-07-15T15:42:52.8463693Z ---- [ui] ui/parser/raw/raw-byte-string-eof.rs stdout ----
2019-07-15T15:42:52.8463755Z diff of stderr:
2019-07-15T15:42:52.8463755Z diff of stderr:
2019-07-15T15:42:52.8463942Z 
2019-07-15T15:42:52.8464051Z 1 error: unterminated raw string
2019-07-15T15:42:52.8464517Z -   --> $DIR/raw-byte-string-eof.rs:2:6
2019-07-15T15:42:52.8464918Z +   --> $DIR/raw-byte-string-eof.rs:2:5
2019-07-15T15:42:52.8465125Z 3    |
2019-07-15T15:42:52.8465239Z 4 LL |     br##"a"#;
2019-07-15T15:42:52.8465540Z -    |      ^ unterminated raw string
2019-07-15T15:42:52.8465592Z +    |     ^^^^ unterminated raw string
2019-07-15T15:42:52.8465775Z + help: you migh have meant to end the raw string here
2019-07-15T15:42:52.8465872Z 6    |
2019-07-15T15:42:52.8466143Z -    = note: this raw string should be terminated with `"##`
2019-07-15T15:42:52.8466343Z + LL |     br##"a'##;
2019-07-15T15:42:52.8466757Z 8 
2019-07-15T15:42:52.8467161Z 9 error: aborting due to previous error
2019-07-15T15:42:52.8467205Z 10 
2019-07-15T15:42:52.8467254Z 
2019-07-15T15:42:52.8467254Z 
2019-07-15T15:42:52.8467584Z 
2019-07-15T15:42:52.8467697Z The actual stderr differed from the expected stderr.
2019-07-15T15:42:52.8468126Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/raw/raw-byte-string-eof/raw-byte-string-eof.stderr
2019-07-15T15:42:52.8468575Z To update references, rerun the tests and pass the `--bless` flag
2019-07-15T15:42:52.8469069Z To only update this specific test, also pass `--test-args parser/raw/raw-byte-string-eof.rs`
2019-07-15T15:42:52.8469393Z error: 1 errors occurred comparing output.
2019-07-15T15:42:52.8469470Z status: exit code: 1
2019-07-15T15:42:52.8469470Z status: exit code: 1
2019-07-15T15:42:52.8470356Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/raw/raw-byte-string-eof.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/raw/raw-byte-string-eof" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/raw/raw-byte-string-eof/auxiliary" "-A" "unused"
2019-07-15T15:42:52.8470983Z ------------------------------------------
2019-07-15T15:42:52.8471322Z 
2019-07-15T15:42:52.8472301Z ------------------------------------------
2019-07-15T15:42:52.8472355Z stderr:
2019-07-15T15:42:52.8472355Z stderr:
2019-07-15T15:42:52.8472566Z ------------------------------------------
2019-07-15T15:42:52.8472788Z error: unterminated raw string
2019-07-15T15:42:52.8473305Z   --> /checkout/src/test/ui/parser/raw/raw-byte-string-eof.rs:2:5
2019-07-15T15:42:52.8473491Z    |
2019-07-15T15:42:52.8473807Z LL |     br##"a"#;  //~ unterminated raw string
2019-07-15T15:42:52.8473887Z    |     ^^^^ unterminated raw string
2019-07-15T15:42:52.8474231Z help: you migh have meant to end the raw string here
2019-07-15T15:42:52.8474330Z    |
2019-07-15T15:42:52.8474674Z LL |     br##"a'##;  //~ unterminated raw string
2019-07-15T15:42:52.8474942Z 
2019-07-15T15:42:52.8475011Z error: aborting due to previous error
2019-07-15T15:42:52.8475061Z 
2019-07-15T15:42:52.8475129Z 
---
2019-07-15T15:42:52.8498583Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:535:22
2019-07-15T15:42:52.8498717Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-07-15T15:42:52.8514845Z 
2019-07-15T15:42:52.8514944Z 
2019-07-15T15:42:52.8520509Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-07-15T15:42:52.8521341Z 
2019-07-15T15:42:52.8521411Z 
2019-07-15T15:42:52.8535273Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-07-15T15:42:52.8535360Z Build completed unsuccessfully in 0:56:46
2019-07-15T15:42:52.8535360Z Build completed unsuccessfully in 0:56:46
2019-07-15T15:42:53.7911866Z ##[error]Bash exited with code '1'.
2019-07-15T15:42:53.7953026Z ##[section]Starting: Checkout
2019-07-15T15:42:53.7972992Z ==============================================================================
2019-07-15T15:42:53.7973102Z Task         : Get sources
2019-07-15T15:42:53.7973153Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
