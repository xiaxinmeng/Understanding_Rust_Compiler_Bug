plain
2019-07-17T07:55:07.4650692Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-17T07:55:07.4855977Z ##[command]git config gc.auto 0
2019-07-17T07:55:07.4943281Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-17T07:55:07.5004661Z ##[command]git config --get-all http.proxy
2019-07-17T07:55:07.5144457Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62746/merge:refs/remotes/pull/62746/merge
---
2019-07-17T07:55:44.5403075Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-17T07:55:44.5403263Z 
2019-07-17T07:55:44.5403618Z   git checkout -b <new-branch-name>
2019-07-17T07:55:44.5404038Z 
2019-07-17T07:55:44.5404194Z HEAD is now at 7e8aacc10 Merge 530651136c617ff952a93519f049e93ca6870464 into bf16480f9cf124630f4a4ffc6d8a57b72fbd5ce9
2019-07-17T07:55:44.5542158Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-17T07:55:44.5545025Z ==============================================================================
2019-07-17T07:55:44.5545105Z Task         : Bash
2019-07-17T07:55:44.5545153Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-17T07:57:36.6546735Z Attempting with retry: curl -y 30 -Y 10 --connect-timeout 30 -f -L -C - -o /tmp/rustci_docker_cache https://.s3.amazonaws.com/docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624
2019-07-17T07:57:36.6601748Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-17T07:57:36.6601895Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-17T07:57:36.6601952Z 
2019-07-17T07:57:36.6646261Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-17T07:57:37.6719629Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-17T07:57:37.6720058Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-17T07:57:37.6720441Z 
2019-07-17T07:57:37.6720441Z 
2019-07-17T07:57:37.6762905Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-17T07:57:39.6847990Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-17T07:57:39.6848071Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-17T07:57:39.6848105Z 
2019-07-17T07:57:39.6848105Z 
2019-07-17T07:57:39.6880132Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-17T07:57:42.6949836Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-17T07:57:42.6949956Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-17T07:57:42.6950023Z 
2019-07-17T07:57:42.6950023Z 
2019-07-17T07:57:42.6992543Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-17T07:57:46.7080493Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-17T07:57:46.7080579Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-17T07:57:46.7080615Z 
2019-07-17T07:57:46.7080615Z 
2019-07-17T07:57:46.7108804Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-17T07:57:46.7113192Z The command has failed after 5 attempts.
2019-07-17T07:57:46.7684594Z open /tmp/rustci_docker_cache: no such file or directory
2019-07-17T07:57:46.7712484Z Attempting with retry: docker build --rm -t rust-ci -f /home/vsts/work/1/s/src/ci/docker/x86_64-gnu-llvm-6.0/Dockerfile /home/vsts/work/1/s/src/ci/docker
2019-07-17T07:57:46.9316922Z Sending build context to Docker daemon  521.2kB
2019-07-17T07:57:46.9317131Z 
2019-07-17T07:57:46.9383472Z Step 1/8 : FROM ubuntu:16.04
---
2019-07-17T07:58:03.8527734Z Reading package lists...
2019-07-17T07:58:04.8570276Z Reading package lists...
2019-07-17T07:58:05.0510023Z Building dependency tree...
2019-07-17T07:58:05.0510144Z Reading state information...
2019-07-17T07:58:05.1786888Z The following additional packages will be installed:
2019-07-17T07:58:05.1788015Z   binfmt-support binutils bzip2 cmake-data cpp cpp-5 dpkg-dev g++-5 gcc gcc-5
2019-07-17T07:58:05.1788277Z   git-man libarchive13 libasan2 libasn1-8-heimdal libatomic1
2019-07-17T07:58:05.1788532Z   libbabeltrace-ctf1 libbabeltrace1 libbsd-dev libbsd0 libbz2-1.0 libc-dev-bin
2019-07-17T07:58:05.1789044Z   libedit2 liberror-perl libexpat1 libffi-dev libffi6 libgcc-5-dev libgdbm3
2019-07-17T07:58:05.1789461Z   libglib2.0-0 libgmp10 libgnutls30 libgomp1 libgssapi-krb5-2
2019-07-17T07:58:05.1789722Z   libgssapi3-heimdal libhcrypto4-heimdal libheimbase1-heimdal
2019-07-17T07:58:05.1790014Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-17T07:58:05.1790014Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-17T07:58:05.1790281Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-17T07:58:05.1790554Z   libkrb5-3 libkrb5support0 libldap-2.4-2 libllvm6.0 liblsan0 liblzo2-2
2019-07-17T07:58:05.1790824Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-17T07:58:05.1791072Z   libperl5.22 libpipeline1 libpython-stdlib libpython2.7-minimal
2019-07-17T07:58:05.1791348Z   libpython2.7-stdlib libpython3.5 libpython3.5-minimal libpython3.5-stdlib
2019-07-17T07:58:05.1795222Z   libquadmath0 libroken18-heimdal librtmp1 libsasl2-2 libsasl2-modules-db
2019-07-17T07:58:05.1795781Z   libsqlite3-0 libssl1.0.0 libstdc++-5-dev libtasn1-6 libtinfo-dev libtsan0
2019-07-17T07:58:05.1796087Z   libubsan0 libwind0-heimdal libxml2 linux-libc-dev llvm-6.0 llvm-6.0-dev
2019-07-17T07:58:05.1796348Z   llvm-6.0-runtime mime-support openssl patch perl perl-modules-5.22 python
2019-07-17T07:58:05.1796585Z   python-minimal python2.7-minimal
2019-07-17T07:58:05.1818218Z Suggested packages:
2019-07-17T07:58:05.1818677Z   binutils-doc bzip2-doc codeblocks eclipse ninja-build cpp-doc gcc-5-locales
2019-07-17T07:58:05.1818931Z   debian-keyring g++-multilib g++-5-multilib gcc-5-doc libstdc++6-5-dbg
2019-07-17T07:58:05.1822823Z   gcc-multilib manpages-dev autoconf automake libtool flex bison gcc-doc
2019-07-17T07:58:05.1824000Z   libasan2-dbg liblsan0-dbg libtsan0-dbg libubsan0-dbg libcilkrts5-dbg
2019-07-17T07:58:05.1824279Z   libmpx0-dbg libquadmath0-dbg gdb-doc gettext-base git-daemon-run
2019-07-17T07:58:05.1824279Z   libmpx0-dbg libquadmath0-dbg gdb-doc gettext-base git-daemon-run
2019-07-17T07:58:05.1824535Z   | git-daemon-sysvinit git-doc git-el git-email git-gui gitk gitweb git-arch
2019-07-17T07:58:05.1824790Z   git-cvs git-mediawiki git-svn lrzip glibc-doc gnutls-bin krb5-doc krb5-user
2019-07-17T07:58:05.1828018Z   libstdc++-5-doc llvm-6.0-doc make-doc ed diffutils-doc perl-doc
2019-07-17T07:58:05.1828312Z   libterm-readline-gnu-perl | libterm-readline-perl-perl python-doc python-tk
2019-07-17T07:58:05.1828502Z   python2.7-doc
2019-07-17T07:58:05.1828571Z Recommended packages:
2019-07-17T07:58:05.1828799Z   build-essential fakeroot libalgorithm-merge-perl libc-dbg gdbserver less
2019-07-17T07:58:05.1829033Z   rsync ssh-client manpages manpages-dev libfile-fcntllock-perl
2019-07-17T07:58:05.1829284Z   libglib2.0-data shared-mime-info xdg-user-dirs krb5-locales libsasl2-modules
2019-07-17T07:58:05.1829664Z   libssl-doc xml-core netbase rename
2019-07-17T07:58:05.1829714Z The following NEW packages will be installed:
2019-07-17T07:58:05.1830202Z   binfmt-support binutils bzip2 ca-certificates cmake cmake-data cpp cpp-5
2019-07-17T07:58:05.1830456Z   curl dpkg-dev file g++ g++-5 gcc gcc-5 gdb git git-man libarchive13 libasan2
2019-07-17T07:58:05.1830704Z   libasn1-8-heimdal libatomic1 libbabeltrace-ctf1 libbabeltrace1 libbsd-dev
2019-07-17T07:58:05.1831145Z   libbsd0 libc-dev-bin libc6-dev libcc1-0 libcilkrts5 libcurl3 libcurl3-gnutls
2019-07-17T07:58:05.1831454Z   libdpkg-perl libedit-dev libedit2 liberror-perl libexpat1 libffi-dev libffi6
2019-07-17T07:58:05.1831701Z   libgcc-5-dev libgdbm3 libglib2.0-0 libgmp10 libgnutls30 libgomp1
2019-07-17T07:58:05.1832234Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-17T07:58:05.1832485Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-17T07:58:05.1832930Z   libkrb5-3 libkrb5support0 libldap-2.4-2 libllvm6.0 liblsan0 liblzo2-2
2019-07-17T07:58:05.1833544Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-17T07:58:05.1833544Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-17T07:58:05.1833799Z   libperl5.22 libpipeline1 libpython-stdlib libpython2.7-minimal
2019-07-17T07:58:05.1834076Z   libpython2.7-stdlib libpython3.5 libpython3.5-minimal libpython3.5-stdlib
2019-07-17T07:58:05.1834347Z   libquadmath0 libroken18-heimdal librtmp1 libsasl2-2 libsasl2-modules-db
2019-07-17T07:58:05.1834613Z   libsqlite3-0 libssl-dev libssl1.0.0 libstdc++-5-dev libtasn1-6 libtinfo-dev
2019-07-17T07:58:05.1834885Z   libtsan0 libubsan0 libwind0-heimdal libxml2 linux-libc-dev llvm-6.0
2019-07-17T07:58:05.1835160Z   llvm-6.0-dev llvm-6.0-runtime llvm-6.0-tools make mime-support openssl patch
2019-07-17T07:58:05.1835410Z   perl perl-modules-5.22 pkg-config python python-minimal python2.7
2019-07-17T07:58:05.1835659Z   python2.7-minimal sudo xz-utils zlib1g-dev
2019-07-17T07:58:05.1835764Z The following packages will be upgraded:
2019-07-17T07:58:05.5906067Z 1 upgraded, 115 newly installed, 0 to remove and 5 not upgraded.
2019-07-17T07:58:05.5906284Z Need to get 121 MB of archives.
2019-07-17T07:58:05.5906519Z After this operation, 592 MB of additional disk space will be used.
2019-07-17T07:58:05.5907387Z Get:1 http://archive.ubuntu.com/ubuntu xenial/main amd64 libgdbm3 amd64 1.8.3-13.1 [16.9 kB]
---
2019-07-17T07:58:08.0319489Z Get:61 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 sudo amd64 1.8.16-0ubuntu1.7 [390 kB]
2019-07-17T07:58:08.1036432Z Get:62 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 openssl amd64 1.0.2g-1ubuntu4.15 [492 kB]
2019-07-17T07:58:08.1108441Z Get:63 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 ca-certificates all 20170717~16.04.2 [167 kB]
2019-07-17T07:58:08.1181325Z Get:64 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 libcurl3-gnutls amd64 7.47.0-1ubuntu2.13 [184 kB]
2019-07-17T07:58:08.1209688Z Get:65 http://archive.ubuntu.com/ubuntu xenial/main amd64 libedit2 amd64 3.1-20150325-1ubuntu2 [76.5 kB]
2019-07-17T07:58:08.1223920Z Get:66 http://archive.ubuntu.com/ubuntu xenial/main amd64 libpipeline1 amd64 1.4.1-2 [24.6 kB]
2019-07-17T07:58:08.1230550Z Get:67 http://archive.ubuntu.com/ubuntu xenial/main amd64 binfmt-support amd64 2.1.6-1 [50.7 kB]
2019-07-17T07:58:08.2464611Z Get:69 http://archive.ubuntu.com/ubuntu xenial/main amd64 libisl15 amd64 0.16.1-1 [524 kB]
2019-07-17T07:58:08.2535386Z Get:70 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 cpp-5 amd64 5.4.0-6ubuntu1~16.04.11 [7660 kB]
2019-07-17T07:58:08.5777158Z Get:71 http://archive.ubuntu.com/ubuntu xenial/main amd64 cpp amd64 4:5.3.1-1ubuntu1 [27.7 kB]
2019-07-17T07:58:08.5782603Z Get:72 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 curl amd64 7.47.0-1ubuntu2.13 [139 kB]
---
2019-07-17T07:58:28.1573418Z Unpacking ca-certificates (20170717~16.04.2) ...
2019-07-17T07:58:28.3068372Z Selecting previously unselected package libcurl3-gnutls:amd64.
2019-07-17T07:58:28.3084203Z Preparing to unpack .../libcurl3-gnutls_7.47.0-1ubuntu2.13_amd64.deb ...
2019-07-17T07:58:28.3213791Z Unpacking libcurl3-gnutls:amd64 (7.47.0-1ubuntu2.13) ...
2019-07-17T07:58:28.4412640Z Selecting previously unselected package libedit2:amd64.
2019-07-17T07:58:28.4430659Z Preparing to unpack .../libedit2_3.1-20150325-1ubuntu2_amd64.deb ...
2019-07-17T07:58:28.4539649Z Unpacking libedit2:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-17T07:58:28.5575558Z Selecting previously unselected package libpipeline1:amd64.
2019-07-17T07:58:28.5594140Z Preparing to unpack .../libpipeline1_1.4.1-2_amd64.deb ...
2019-07-17T07:58:28.5709949Z Unpacking libpipeline1:amd64 (1.4.1-2) ...
2019-07-17T07:58:28.6755000Z Selecting previously unselected package binfmt-support.
2019-07-17T07:58:28.6772352Z Preparing to unpack .../binfmt-support_2.1.6-1_amd64.deb ...
2019-07-17T07:58:28.6905237Z Unpacking binfmt-support (2.1.6-1) ...
2019-07-17T07:58:28.8331558Z Preparing to unpack .../binutils_2.26.1-1ubuntu1~16.04.8_amd64.deb ...
2019-07-17T07:58:28.8469730Z Unpacking binutils (2.26.1-1ubuntu1~16.04.8) ...
2019-07-17T07:58:29.3731678Z Selecting previously unselected package libisl15:amd64.
2019-07-17T07:58:29.3749232Z Preparing to unpack .../libisl15_0.16.1-1_amd64.deb ...
---
2019-07-17T07:58:44.0959329Z Unpacking zlib1g-dev:amd64 (1:1.2.8.dfsg-2ubuntu4.1) ...
2019-07-17T07:58:44.1943887Z Selecting previously unselected package libssl-dev:amd64.
2019-07-17T07:58:44.1964303Z Preparing to unpack .../libssl-dev_1.0.2g-1ubuntu4.15_amd64.deb ...
2019-07-17T07:58:44.2410773Z Unpacking libssl-dev:amd64 (1.0.2g-1ubuntu4.15) ...
2019-07-17T07:58:44.5922400Z Selecting previously unselected package llvm-6.0-runtime.
2019-07-17T07:58:44.5944736Z Preparing to unpack .../llvm-6.0-runtime_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-17T07:58:44.6070509Z Unpacking llvm-6.0-runtime (1:6.0-1ubuntu2~16.04.1) ...
2019-07-17T07:58:44.7453896Z Selecting previously unselected package llvm-6.0.
2019-07-17T07:58:44.7473800Z Preparing to unpack .../llvm-6.0_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-17T07:58:44.7586283Z Unpacking llvm-6.0 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-17T07:58:45.4338713Z Selecting previously unselected package libffi-dev:amd64.
2019-07-17T07:58:45.4359354Z Preparing to unpack .../libffi-dev_3.2.1-4_amd64.deb ...
2019-07-17T07:58:45.4477479Z Unpacking libffi-dev:amd64 (3.2.1-4) ...
2019-07-17T07:58:45.5827541Z Selecting previously unselected package llvm-6.0-dev.
2019-07-17T07:58:45.5848821Z Preparing to unpack .../llvm-6.0-dev_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-17T07:58:45.6028420Z Unpacking llvm-6.0-dev (1:6.0-1ubuntu2~16.04.1) ...
2019-07-17T07:58:50.2112547Z Selecting previously unselected package llvm-6.0-tools.
2019-07-17T07:58:50.2136590Z Preparing to unpack .../llvm-6.0-tools_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-17T07:58:50.2250955Z Unpacking llvm-6.0-tools (1:6.0-1ubuntu2~16.04.1) ...
2019-07-17T07:58:50.3691034Z Selecting previously unselected package pkg-config.
2019-07-17T07:58:50.3711124Z Preparing to unpack .../pkg-config_0.29.1-0ubuntu1_amd64.deb ...
2019-07-17T07:58:50.4372496Z Processing triggers for libc-bin (2.23-0ubuntu11) ...
2019-07-17T07:58:50.4835494Z Processing triggers for systemd (229-4ubuntu21.21) ...
2019-07-17T07:58:50.7847295Z Setting up libgdbm3:amd64 (1.8.3-13.1) ...
2019-07-17T07:58:50.8486809Z Setting up libffi6:amd64 (3.2.1-4) ...
---
2019-07-17T07:58:54.4144633Z debconf: unable to initialize frontend: Dialog
2019-07-17T07:58:54.4144822Z debconf: (TERM is not set, so the dialog frontend is not usable.)
2019-07-17T07:58:54.4144880Z debconf: falling back to frontend: Readline
2019-07-17T07:58:54.9211176Z Setting up libcurl3-gnutls:amd64 (7.47.0-1ubuntu2.13) ...
2019-07-17T07:58:54.9596643Z Setting up libedit2:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-17T07:58:55.0007263Z Setting up libpipeline1:amd64 (1.4.1-2) ...
2019-07-17T07:58:55.0503141Z Setting up binfmt-support (2.1.6-1) ...
2019-07-17T07:58:55.1315269Z mount: permission denied
2019-07-17T07:58:55.1316159Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-17T07:58:55.1331553Z mount: permission denied
2019-07-17T07:58:55.1334213Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-17T07:58:55.2946118Z invoke-rc.d: could not determine current runlevel
2019-07-17T07:58:55.2977694Z invoke-rc.d: policy-rc.d denied execution of start.
2019-07-17T07:58:55.3579668Z Setting up libisl15:amd64 (0.16.1-1) ...
2019-07-17T07:58:55.3963239Z Setting up cpp-5 (5.4.0-6ubuntu1~16.04.11) ...
2019-07-17T07:58:55.4415537Z Setting up cpp (4:5.3.1-1ubuntu1) ...
2019-07-17T07:58:55.4934842Z Setting up curl (7.47.0-1ubuntu2.13) ...
---
2019-07-17T07:58:57.3652327Z Setting up libedit-dev:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-17T07:58:57.4249564Z Setting up libllvm6.0:amd64 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-17T07:58:57.4837931Z Setting up zlib1g-dev:amd64 (1:1.2.8.dfsg-2ubuntu4.1) ...
2019-07-17T07:58:57.5263725Z Setting up libssl-dev:amd64 (1.0.2g-1ubuntu4.15) ...
2019-07-17T07:58:57.5749326Z Setting up llvm-6.0-runtime (1:6.0-1ubuntu2~16.04.1) ...
2019-07-17T07:58:57.6085735Z mount: permission denied
2019-07-17T07:58:57.6087104Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-17T07:58:57.6209947Z Setting up llvm-6.0 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-17T07:58:57.6642497Z Setting up libffi-dev:amd64 (3.2.1-4) ...
2019-07-17T07:58:57.7055753Z Setting up llvm-6.0-dev (1:6.0-1ubuntu2~16.04.1) ...
2019-07-17T07:58:57.7524624Z Setting up llvm-6.0-tools (1:6.0-1ubuntu2~16.04.1) ...
2019-07-17T07:58:57.8112566Z Setting up pkg-config (0.29.1-0ubuntu1) ...
2019-07-17T07:58:57.9893897Z Processing triggers for ca-certificates (20170717~16.04.2) ...
2019-07-17T07:58:58.0050948Z Updating certificates in /etc/ssl/certs...
2019-07-17T07:58:59.6498315Z 148 added, 0 removed; done.
2019-07-17T07:58:59.6499089Z Running hooks in /etc/ca-certificates/update.d...
---
2019-07-17T07:59:33.2915519Z  ---> fd68cfa319bc
2019-07-17T07:59:33.2960937Z Successfully built fd68cfa319bc
2019-07-17T07:59:33.5186280Z Successfully tagged rust-ci:latest
2019-07-17T07:59:33.6475999Z Built container sha256:fd68cfa319bced69c4dfc05c2f76350155ded0ccf7613b1b85296f9f5fd31370
2019-07-17T07:59:33.6490878Z Uploading finished image to https://.s3.amazonaws.com/docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624
2019-07-17T08:00:35.6530607Z upload failed: - to s3:///docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624 Parameter validation failed:
2019-07-17T08:00:35.6531095Z Invalid bucket name "": Bucket name must match the regex "^[a-zA-Z0-9.\-_]{1,255}$"
2019-07-17T08:00:36.6572757Z [CI_JOB_NAME=x86_64-gnu-llvm-6.0]
2019-07-17T08:00:36.6618592Z Starting sccache server...
2019-07-17T08:00:36.7119516Z configure: processing command line
2019-07-17T08:00:36.7120259Z configure: 
---
2019-07-17T08:27:53.0734595Z    Compiling hashbrown v0.4.0
2019-07-17T08:27:55.0887195Z error: use of deprecated item 'sync::once::ONCE_INIT': the `new` function is now preferred
2019-07-17T08:27:55.0894473Z    --> src/libstd/sync/mod.rs:167:39
2019-07-17T08:27:55.0902558Z     |
2019-07-17T08:27:55.0903161Z 167 | pub use self::once::{Once, OnceState, ONCE_INIT};
2019-07-17T08:27:55.0903713Z     |
2019-07-17T08:27:55.0903713Z     |
2019-07-17T08:27:55.0904135Z     = note: `-D deprecated` implied by `-D warnings`
2019-07-17T08:27:57.1171266Z error: aborting due to previous error
2019-07-17T08:27:57.1171366Z 
2019-07-17T08:27:57.1687092Z error: Could not compile `std`.
2019-07-17T08:27:57.1687255Z 
2019-07-17T08:27:57.1687255Z 
2019-07-17T08:27:57.1687541Z To learn more, run the command again with --verbose.
2019-07-17T08:27:57.1747478Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
2019-07-17T08:27:57.1755208Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-07-17T08:27:57.1755314Z Build completed unsuccessfully in 0:23:33
2019-07-17T08:27:57.1755314Z Build completed unsuccessfully in 0:23:33
2019-07-17T08:28:00.0585530Z ##[error]Bash exited with code '1'.
2019-07-17T08:28:00.0618043Z ##[section]Starting: Checkout
2019-07-17T08:28:00.0619878Z ==============================================================================
2019-07-17T08:28:00.0619936Z Task         : Get sources
2019-07-17T08:28:00.0619974Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
