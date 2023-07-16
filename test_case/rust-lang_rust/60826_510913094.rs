plain
2019-07-12T13:27:58.6737016Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-12T13:27:58.6918401Z ##[command]git config gc.auto 0
2019-07-12T13:27:58.6991152Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-12T13:27:58.7053568Z ##[command]git config --get-all http.proxy
2019-07-12T13:27:58.7193321Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/60826/merge:refs/remotes/pull/60826/merge
---
2019-07-12T13:28:33.6483650Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-12T13:28:33.6483702Z 
2019-07-12T13:28:33.6483915Z   git checkout -b <new-branch-name>
2019-07-12T13:28:33.6484103Z 
2019-07-12T13:28:33.6484152Z HEAD is now at aef0a6019 Merge 9b8e08d4d9b9fff8d939890ad844d445ea9f7c44 into cd1381e91ff4889616eb0c87bf3c321ea2697d42
2019-07-12T13:28:33.6630347Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-12T13:28:33.6633226Z ==============================================================================
2019-07-12T13:28:33.6633302Z Task         : Bash
2019-07-12T13:28:33.6633347Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-12T13:28:34.2348778Z 320 ./src/librustc/mir
2019-07-12T13:28:34.2349574Z sort: write failed: 'standard output': Broken pipe
2019-07-12T13:28:34.2349902Z sort: write error
2019-07-12T13:28:34.2412462Z ##[section]Finishing: Show disk usage
2019-07-12T13:28:34.2466444Z ##[section]Starting: Disable git automatic line ending conversion (on C:/)
2019-07-12T13:28:34.2469529Z Task         : Bash
2019-07-12T13:28:34.2469579Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-12T13:28:34.2469630Z Version      : 3.151.2
2019-07-12T13:28:34.2469678Z Author       : Microsoft Corporation
2019-07-12T13:28:34.2469678Z Author       : Microsoft Corporation
2019-07-12T13:28:34.2469746Z Help         : https://docs.microsoft.com/azure/devops/pipelines/tasks/utility/bash
2019-07-12T13:28:34.2469802Z ==============================================================================
2019-07-12T13:28:34.3806529Z Generating script.
2019-07-12T13:28:34.3820560Z Script contents:
2019-07-12T13:28:34.3820870Z git config --replace-all --global core.autocrlf false
2019-07-12T13:28:34.3841394Z ========================== Starting Command Output ===========================
2019-07-12T13:28:34.3858622Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/07638c26-debd-4703-a28b-aac6a4236fe7.sh
2019-07-12T13:28:34.4000518Z ##[section]Finishing: Disable git automatic line ending conversion (on C:/)
2019-07-12T13:28:34.4042766Z ==============================================================================
2019-07-12T13:28:34.4042826Z Task         : Bash
2019-07-12T13:28:34.4042875Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-12T13:28:34.4043122Z Version      : 3.151.2
---
2019-07-12T13:30:17.2341016Z Attempting with retry: curl -y 30 -Y 10 --connect-timeout 30 -f -L -C - -o /tmp/rustci_docker_cache https://.s3.amazonaws.com/docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624
2019-07-12T13:30:17.2394093Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-12T13:30:17.2394387Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-12T13:30:17.2394545Z 
2019-07-12T13:30:17.2438835Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-12T13:30:18.2510035Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-12T13:30:18.2510421Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-12T13:30:18.2510581Z 
2019-07-12T13:30:18.2510581Z 
2019-07-12T13:30:18.2553178Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-12T13:30:20.2664618Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-12T13:30:20.2667186Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-12T13:30:20.2667475Z 
2019-07-12T13:30:20.2667475Z 
2019-07-12T13:30:20.2668529Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-12T13:30:23.2755851Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-12T13:30:23.2756041Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-12T13:30:23.2756080Z 
2019-07-12T13:30:23.2756080Z 
2019-07-12T13:30:23.2787914Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-12T13:30:27.2888791Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-12T13:30:27.2891643Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-12T13:30:27.2891914Z 
2019-07-12T13:30:27.2891914Z 
2019-07-12T13:30:27.2892862Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-12T13:30:27.2893232Z The command has failed after 5 attempts.
2019-07-12T13:30:27.3906389Z open /tmp/rustci_docker_cache: no such file or directory
2019-07-12T13:30:27.3933082Z Attempting with retry: docker build --rm -t rust-ci -f /home/vsts/work/1/s/src/ci/docker/x86_64-gnu-llvm-6.0/Dockerfile /home/vsts/work/1/s/src/ci/docker
2019-07-12T13:30:27.6506784Z Sending build context to Docker daemon  521.7kB
2019-07-12T13:30:27.6507734Z 
2019-07-12T13:30:27.6706244Z Step 1/8 : FROM ubuntu:16.04
---
2019-07-12T13:30:44.9458447Z Reading package lists...
2019-07-12T13:30:45.8684211Z Reading package lists...
2019-07-12T13:30:46.0233850Z Building dependency tree...
2019-07-12T13:30:46.0234767Z Reading state information...
2019-07-12T13:30:46.1440289Z The following additional packages will be installed:
2019-07-12T13:30:46.1441040Z   binfmt-support binutils bzip2 cmake-data cpp cpp-5 dpkg-dev g++-5 gcc gcc-5
2019-07-12T13:30:46.1441403Z   git-man libarchive13 libasan2 libasn1-8-heimdal libatomic1
2019-07-12T13:30:46.1441869Z   libbabeltrace-ctf1 libbabeltrace1 libbsd-dev libbsd0 libbz2-1.0 libc-dev-bin
2019-07-12T13:30:46.1446993Z   libedit2 liberror-perl libexpat1 libffi-dev libffi6 libgcc-5-dev libgdbm3
2019-07-12T13:30:46.1447372Z   libglib2.0-0 libgmp10 libgnutls30 libgomp1 libgssapi-krb5-2
2019-07-12T13:30:46.1448369Z   libgssapi3-heimdal libhcrypto4-heimdal libheimbase1-heimdal
2019-07-12T13:30:46.1448715Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-12T13:30:46.1448715Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-12T13:30:46.1448995Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-12T13:30:46.1449256Z   libkrb5-3 libkrb5support0 libldap-2.4-2 libllvm6.0 liblsan0 liblzo2-2
2019-07-12T13:30:46.1449556Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-12T13:30:46.1450075Z   libperl5.22 libpipeline1 libpython-stdlib libpython2.7-minimal
2019-07-12T13:30:46.1450337Z   libpython2.7-stdlib libpython3.5 libpython3.5-minimal libpython3.5-stdlib
2019-07-12T13:30:46.1450650Z   libquadmath0 libroken18-heimdal librtmp1 libsasl2-2 libsasl2-modules-db
2019-07-12T13:30:46.1450920Z   libsqlite3-0 libssl1.0.0 libstdc++-5-dev libtasn1-6 libtinfo-dev libtsan0
2019-07-12T13:30:46.1451177Z   libubsan0 libwind0-heimdal libxml2 linux-libc-dev llvm-6.0 llvm-6.0-dev
2019-07-12T13:30:46.1451657Z   llvm-6.0-runtime mime-support openssl patch perl perl-modules-5.22 python
2019-07-12T13:30:46.1451879Z   python-minimal python2.7-minimal
2019-07-12T13:30:46.1452001Z Suggested packages:
2019-07-12T13:30:46.1452317Z   binutils-doc bzip2-doc codeblocks eclipse ninja-build cpp-doc gcc-5-locales
2019-07-12T13:30:46.1452572Z   debian-keyring g++-multilib g++-5-multilib gcc-5-doc libstdc++6-5-dbg
2019-07-12T13:30:46.1452817Z   gcc-multilib manpages-dev autoconf automake libtool flex bison gcc-doc
2019-07-12T13:30:46.1453376Z   libasan2-dbg liblsan0-dbg libtsan0-dbg libubsan0-dbg libcilkrts5-dbg
2019-07-12T13:30:46.1453614Z   libmpx0-dbg libquadmath0-dbg gdb-doc gettext-base git-daemon-run
2019-07-12T13:30:46.1453614Z   libmpx0-dbg libquadmath0-dbg gdb-doc gettext-base git-daemon-run
2019-07-12T13:30:46.1453905Z   | git-daemon-sysvinit git-doc git-el git-email git-gui gitk gitweb git-arch
2019-07-12T13:30:46.1454160Z   git-cvs git-mediawiki git-svn lrzip glibc-doc gnutls-bin krb5-doc krb5-user
2019-07-12T13:30:46.1454677Z   libstdc++-5-doc llvm-6.0-doc make-doc ed diffutils-doc perl-doc
2019-07-12T13:30:46.1455021Z   libterm-readline-gnu-perl | libterm-readline-perl-perl python-doc python-tk
2019-07-12T13:30:46.1455234Z   python2.7-doc
2019-07-12T13:30:46.1455284Z Recommended packages:
2019-07-12T13:30:46.1455570Z   build-essential fakeroot libalgorithm-merge-perl libc-dbg gdbserver less
2019-07-12T13:30:46.1455814Z   rsync ssh-client manpages manpages-dev libfile-fcntllock-perl
2019-07-12T13:30:46.1456067Z   libglib2.0-data shared-mime-info xdg-user-dirs krb5-locales libsasl2-modules
2019-07-12T13:30:46.1456337Z   libssl-doc xml-core netbase rename
2019-07-12T13:30:46.1475808Z The following NEW packages will be installed:
2019-07-12T13:30:46.1476276Z   binfmt-support binutils bzip2 ca-certificates cmake cmake-data cpp cpp-5
2019-07-12T13:30:46.1476610Z   curl dpkg-dev file g++ g++-5 gcc gcc-5 gdb git git-man libarchive13 libasan2
2019-07-12T13:30:46.1476878Z   libasn1-8-heimdal libatomic1 libbabeltrace-ctf1 libbabeltrace1 libbsd-dev
2019-07-12T13:30:46.1477439Z   libbsd0 libc-dev-bin libc6-dev libcc1-0 libcilkrts5 libcurl3 libcurl3-gnutls
2019-07-12T13:30:46.1481491Z   libdpkg-perl libedit-dev libedit2 liberror-perl libexpat1 libffi-dev libffi6
2019-07-12T13:30:46.1481894Z   libgcc-5-dev libgdbm3 libglib2.0-0 libgmp10 libgnutls30 libgomp1
2019-07-12T13:30:46.1482475Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-12T13:30:46.1482765Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-12T13:30:46.1483023Z   libkrb5-3 libkrb5support0 libldap-2.4-2 libllvm6.0 liblsan0 liblzo2-2
2019-07-12T13:30:46.1483325Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-12T13:30:46.1483325Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-12T13:30:46.1483581Z   libperl5.22 libpipeline1 libpython-stdlib libpython2.7-minimal
2019-07-12T13:30:46.1483842Z   libpython2.7-stdlib libpython3.5 libpython3.5-minimal libpython3.5-stdlib
2019-07-12T13:30:46.1484145Z   libquadmath0 libroken18-heimdal librtmp1 libsasl2-2 libsasl2-modules-db
2019-07-12T13:30:46.1484422Z   libsqlite3-0 libssl-dev libssl1.0.0 libstdc++-5-dev libtasn1-6 libtinfo-dev
2019-07-12T13:30:46.1484677Z   libtsan0 libubsan0 libwind0-heimdal libxml2 linux-libc-dev llvm-6.0
2019-07-12T13:30:46.1484982Z   llvm-6.0-dev llvm-6.0-runtime llvm-6.0-tools make mime-support openssl patch
2019-07-12T13:30:46.1485241Z   perl perl-modules-5.22 pkg-config python python-minimal python2.7
2019-07-12T13:30:46.1485609Z   python2.7-minimal sudo xz-utils zlib1g-dev
2019-07-12T13:30:46.1485714Z The following packages will be upgraded:
2019-07-12T13:30:46.4258518Z 1 upgraded, 115 newly installed, 0 to remove and 4 not upgraded.
2019-07-12T13:30:46.4258746Z Need to get 121 MB of archives.
2019-07-12T13:30:46.4258990Z After this operation, 592 MB of additional disk space will be used.
2019-07-12T13:30:46.4259778Z Get:1 http://archive.ubuntu.com/ubuntu xenial/main amd64 libgdbm3 amd64 1.8.3-13.1 [16.9 kB]
---
2019-07-12T13:30:47.7866326Z Get:61 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 sudo amd64 1.8.16-0ubuntu1.7 [390 kB]
2019-07-12T13:30:47.7866584Z Get:62 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 openssl amd64 1.0.2g-1ubuntu4.15 [492 kB]
2019-07-12T13:30:47.7866912Z Get:63 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 ca-certificates all 20170717~16.04.2 [167 kB]
2019-07-12T13:30:47.7867178Z Get:64 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 libcurl3-gnutls amd64 7.47.0-1ubuntu2.13 [184 kB]
2019-07-12T13:30:47.7867477Z Get:65 http://archive.ubuntu.com/ubuntu xenial/main amd64 libedit2 amd64 3.1-20150325-1ubuntu2 [76.5 kB]
2019-07-12T13:30:47.7867733Z Get:66 http://archive.ubuntu.com/ubuntu xenial/main amd64 libpipeline1 amd64 1.4.1-2 [24.6 kB]
2019-07-12T13:30:47.7868547Z Get:67 http://archive.ubuntu.com/ubuntu xenial/main amd64 binfmt-support amd64 2.1.6-1 [50.7 kB]
2019-07-12T13:30:47.7869219Z Get:69 http://archive.ubuntu.com/ubuntu xenial/main amd64 libisl15 amd64 0.16.1-1 [524 kB]
2019-07-12T13:30:47.7869536Z Get:70 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 cpp-5 amd64 5.4.0-6ubuntu1~16.04.11 [7660 kB]
2019-07-12T13:30:47.8853476Z Get:71 http://archive.ubuntu.com/ubuntu xenial/main amd64 cpp amd64 4:5.3.1-1ubuntu1 [27.7 kB]
2019-07-12T13:30:47.8865813Z Get:72 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 curl amd64 7.47.0-1ubuntu2.13 [139 kB]
---
2019-07-12T13:31:09.6362417Z Unpacking ca-certificates (20170717~16.04.2) ...
2019-07-12T13:31:09.8143963Z Selecting previously unselected package libcurl3-gnutls:amd64.
2019-07-12T13:31:09.8161535Z Preparing to unpack .../libcurl3-gnutls_7.47.0-1ubuntu2.13_amd64.deb ...
2019-07-12T13:31:09.8301334Z Unpacking libcurl3-gnutls:amd64 (7.47.0-1ubuntu2.13) ...
2019-07-12T13:31:09.9634390Z Selecting previously unselected package libedit2:amd64.
2019-07-12T13:31:09.9652249Z Preparing to unpack .../libedit2_3.1-20150325-1ubuntu2_amd64.deb ...
2019-07-12T13:31:09.9792710Z Unpacking libedit2:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-12T13:31:10.1010742Z Selecting previously unselected package libpipeline1:amd64.
2019-07-12T13:31:10.1011408Z Preparing to unpack .../libpipeline1_1.4.1-2_amd64.deb ...
2019-07-12T13:31:10.1140544Z Unpacking libpipeline1:amd64 (1.4.1-2) ...
2019-07-12T13:31:10.2372056Z Selecting previously unselected package binfmt-support.
2019-07-12T13:31:10.2390927Z Preparing to unpack .../binfmt-support_2.1.6-1_amd64.deb ...
2019-07-12T13:31:10.2532339Z Unpacking binfmt-support (2.1.6-1) ...
2019-07-12T13:31:10.3884730Z Preparing to unpack .../binutils_2.26.1-1ubuntu1~16.04.8_amd64.deb ...
2019-07-12T13:31:10.4022048Z Unpacking binutils (2.26.1-1ubuntu1~16.04.8) ...
2019-07-12T13:31:11.0092072Z Selecting previously unselected package libisl15:amd64.
2019-07-12T13:31:11.0113074Z Preparing to unpack .../libisl15_0.16.1-1_amd64.deb ...
---
2019-07-12T13:31:22.5262639Z Selecting previously unselected package libtinfo-dev:amd64.
2019-07-12T13:31:22.5281188Z Preparing to unpack .../libtinfo-dev_6.0+20160213-1ubuntu1_amd64.deb ...
2019-07-12T13:31:22.5430010Z Unpacking libtinfo-dev:amd64 (6.0+20160213-1ubuntu1) ...
2019-07-12T13:31:22.6482556Z Selecting previously unselected package libedit-dev:amd64.
2019-07-12T13:31:22.6500987Z Preparing to unpack .../libedit-dev_3.1-20150325-1ubuntu2_amd64.deb ...
2019-07-12T13:31:22.6651181Z Unpacking libedit-dev:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-12T13:31:22.7888935Z Selecting previously unselected package libllvm6.0:amd64.
2019-07-12T13:31:22.7907502Z Preparing to unpack .../libllvm6.0_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-12T13:31:22.8078943Z Unpacking libllvm6.0:amd64 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-12T13:31:25.7576802Z Selecting previously unselected package zlib1g-dev:amd64.
2019-07-12T13:31:25.7595281Z Preparing to unpack .../zlib1g-dev_1%3a1.2.8.dfsg-2ubuntu4.1_amd64.deb ...
2019-07-12T13:31:25.7734977Z Unpacking zlib1g-dev:amd64 (1:1.2.8.dfsg-2ubuntu4.1) ...
2019-07-12T13:31:25.8836024Z Preparing to unpack .../libssl-dev_1.0.2g-1ubuntu4.15_amd64.deb ...
2019-07-12T13:31:25.8990079Z Unpacking libssl-dev:amd64 (1.0.2g-1ubuntu4.15) ...
2019-07-12T13:31:26.2056241Z Selecting previously unselected package llvm-6.0-runtime.
2019-07-12T13:31:26.2056241Z Selecting previously unselected package llvm-6.0-runtime.
2019-07-12T13:31:26.2074988Z Preparing to unpack .../llvm-6.0-runtime_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-12T13:31:26.2212027Z Unpacking llvm-6.0-runtime (1:6.0-1ubuntu2~16.04.1) ...
2019-07-12T13:31:26.3421805Z Selecting previously unselected package llvm-6.0.
2019-07-12T13:31:26.3439564Z Preparing to unpack .../llvm-6.0_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-12T13:31:26.3584689Z Unpacking llvm-6.0 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-12T13:31:27.0000576Z Selecting previously unselected package libffi-dev:amd64.
2019-07-12T13:31:27.0022490Z Preparing to unpack .../libffi-dev_3.2.1-4_amd64.deb ...
2019-07-12T13:31:27.0173425Z Unpacking libffi-dev:amd64 (3.2.1-4) ...
2019-07-12T13:31:27.1487571Z Selecting previously unselected package llvm-6.0-dev.
2019-07-12T13:31:27.1504244Z Preparing to unpack .../llvm-6.0-dev_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-12T13:31:27.1655163Z Unpacking llvm-6.0-dev (1:6.0-1ubuntu2~16.04.1) ...
2019-07-12T13:31:31.7533965Z Selecting previously unselected package llvm-6.0-tools.
2019-07-12T13:31:31.7552982Z Preparing to unpack .../llvm-6.0-tools_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-12T13:31:31.7700604Z Unpacking llvm-6.0-tools (1:6.0-1ubuntu2~16.04.1) ...
2019-07-12T13:31:31.9362218Z Selecting previously unselected package pkg-config.
2019-07-12T13:31:31.9387732Z Preparing to unpack .../pkg-config_0.29.1-0ubuntu1_amd64.deb ...
2019-07-12T13:31:31.9536061Z Unpacking pkg-config (0.29.1-0ubuntu1) ...
2019-07-12T13:31:32.0731577Z Processing triggers for systemd (229-4ubuntu21.21) ...
2019-07-12T13:31:32.4654173Z Setting up libgdbm3:amd64 (1.8.3-13.1) ...
2019-07-12T13:31:32.5452017Z Setting up libffi6:amd64 (3.2.1-4) ...
2019-07-12T13:31:32.5865252Z Setting up libglib2.0-0:amd64 (2.48.2-0ubuntu4.3) ...
---
2019-07-12T13:31:36.4829331Z debconf: unable to initialize frontend: Dialog
2019-07-12T13:31:36.4829414Z debconf: (TERM is not set, so the dialog frontend is not usable.)
2019-07-12T13:31:36.4829469Z debconf: falling back to frontend: Readline
2019-07-12T13:31:37.0060594Z Setting up libcurl3-gnutls:amd64 (7.47.0-1ubuntu2.13) ...
2019-07-12T13:31:37.0519702Z Setting up libedit2:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-12T13:31:37.0960757Z Setting up libpipeline1:amd64 (1.4.1-2) ...
2019-07-12T13:31:37.1394198Z Setting up binfmt-support (2.1.6-1) ...
2019-07-12T13:31:37.2197818Z mount: permission denied
2019-07-12T13:31:37.2202464Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-12T13:31:37.2217902Z mount: permission denied
2019-07-12T13:31:37.2218464Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-12T13:31:37.3843854Z invoke-rc.d: could not determine current runlevel
2019-07-12T13:31:37.3874673Z invoke-rc.d: policy-rc.d denied execution of start.
2019-07-12T13:31:37.4501490Z Setting up libisl15:amd64 (0.16.1-1) ...
2019-07-12T13:31:37.4951004Z Setting up cpp-5 (5.4.0-6ubuntu1~16.04.11) ...
2019-07-12T13:31:37.5409716Z Setting up cpp (4:5.3.1-1ubuntu1) ...
2019-07-12T13:31:37.5927104Z Setting up curl (7.47.0-1ubuntu2.13) ...
---
2019-07-12T13:31:39.4252984Z Setting up libedit-dev:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-12T13:31:39.4680805Z Setting up libllvm6.0:amd64 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-12T13:31:39.5211878Z Setting up zlib1g-dev:amd64 (1:1.2.8.dfsg-2ubuntu4.1) ...
2019-07-12T13:31:39.5722801Z Setting up libssl-dev:amd64 (1.0.2g-1ubuntu4.15) ...
2019-07-12T13:31:39.6142554Z Setting up llvm-6.0-runtime (1:6.0-1ubuntu2~16.04.1) ...
2019-07-12T13:31:39.6581450Z mount: permission denied
2019-07-12T13:31:39.6582039Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-12T13:31:39.6738992Z Setting up llvm-6.0 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-12T13:31:39.7167717Z Setting up libffi-dev:amd64 (3.2.1-4) ...
2019-07-12T13:31:39.7603312Z Setting up llvm-6.0-dev (1:6.0-1ubuntu2~16.04.1) ...
2019-07-12T13:31:39.8040160Z Setting up llvm-6.0-tools (1:6.0-1ubuntu2~16.04.1) ...
2019-07-12T13:31:39.8474101Z Setting up pkg-config (0.29.1-0ubuntu1) ...
2019-07-12T13:31:39.9955193Z Processing triggers for ca-certificates (20170717~16.04.2) ...
2019-07-12T13:31:40.0142437Z Updating certificates in /etc/ssl/certs...
2019-07-12T13:31:41.6447084Z 148 added, 0 removed; done.
2019-07-12T13:31:41.6452869Z Running hooks in /etc/ca-certificates/update.d...
---
2019-07-12T13:32:13.5656662Z Removing intermediate container bdd7e1603546
2019-07-12T13:32:13.5657539Z  ---> 9c6c960c38c4
2019-07-12T13:32:13.5698752Z Successfully built 9c6c960c38c4
2019-07-12T13:32:13.7630842Z Successfully tagged rust-ci:latest
2019-07-12T13:32:13.8740520Z Built container sha256:9c6c960c38c4b9080e29faf52058122c1216eb9032edd4b345fa02961627d2ba
2019-07-12T13:32:13.8755954Z Uploading finished image to https://.s3.amazonaws.com/docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624
2019-07-12T13:33:16.0768251Z upload failed: - to s3:///docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624 Parameter validation failed:
2019-07-12T13:33:16.0769206Z Invalid bucket name "": Bucket name must match the regex "^[a-zA-Z0-9.\-_]{1,255}$"
2019-07-12T13:33:17.1048390Z [CI_JOB_NAME=x86_64-gnu-llvm-6.0]
2019-07-12T13:33:17.1100269Z Starting sccache server...
2019-07-12T13:33:17.1605819Z configure: processing command line
2019-07-12T13:33:17.1607157Z configure: 
---
2019-07-12T14:29:30.5859505Z .................................................................................................... 200/5799
2019-07-12T14:29:34.8244305Z .................................................................................................... 300/5799
2019-07-12T14:29:38.6467373Z .................................................................................................... 400/5799
2019-07-12T14:29:42.5143296Z .................................................................................................... 500/5799
2019-07-12T14:29:46.3621607Z .......................................................................i............................ 600/5799
2019-07-12T14:29:55.4696428Z .................................................................................................... 800/5799
2019-07-12T14:30:01.0239601Z .................................................................................................... 900/5799
2019-07-12T14:30:01.0239601Z .................................................................................................... 900/5799
2019-07-12T14:30:07.0062842Z ..........................................................................................i......... 1000/5799
2019-07-12T14:30:11.4951291Z ..i................................................................................................. 1100/5799
2019-07-12T14:30:15.8263763Z ....................iiiii........................................................................... 1200/5799
2019-07-12T14:30:21.6037140Z .................................................................................................... 1400/5799
2019-07-12T14:30:24.4404551Z .................................................................................................... 1500/5799
2019-07-12T14:30:28.2528049Z .................................................................................................... 1600/5799
2019-07-12T14:30:30.8119540Z .................................................................................................... 1700/5799
2019-07-12T14:30:30.8119540Z .................................................................................................... 1700/5799
2019-07-12T14:30:34.3544656Z ..........................................................i......................................... 1800/5799
2019-07-12T14:30:43.0682279Z .................................................................................................... 2000/5799
2019-07-12T14:30:47.1207300Z .................................................................................................... 2100/5799
2019-07-12T14:30:51.2077435Z .................................................................................................... 2200/5799
2019-07-12T14:30:51.2077435Z .................................................................................................... 2200/5799
2019-07-12T14:30:54.9304507Z ........................i........................................................................... 2300/5799
2019-07-12T14:31:04.9691984Z .................................................................................................... 2500/5799
2019-07-12T14:31:09.8179471Z .................................................................................................... 2600/5799
2019-07-12T14:31:14.1825859Z .................................................................................................... 2700/5799
2019-07-12T14:31:18.6074698Z .................................................................................................... 2800/5799
2019-07-12T14:31:18.6074698Z .................................................................................................... 2800/5799
2019-07-12T14:31:22.6707039Z .................................................................................................... 2900/5799
2019-07-12T14:31:28.3016991Z .................................................................................................... 3000/5799
2019-07-12T14:31:33.2273161Z .................................................................................................... 3100/5799
2019-07-12T14:31:37.8263854Z .................................................................................................... 3200/5799
2019-07-12T14:31:40.9080759Z .................................................................................................... 3300/5799
2019-07-12T14:31:46.2199353Z .................................................................................................... 3400/5799
2019-07-12T14:31:50.2060065Z ..................................................................................i................. 3500/5799
2019-07-12T14:31:54.2080847Z .................................................................................................... 3600/5799
2019-07-12T14:31:58.1982130Z ........................................................ii...i..ii.................................. 3700/5799
2019-07-12T14:32:07.5643901Z .................................................................................................... 3900/5799
2019-07-12T14:32:07.5643901Z .................................................................................................... 3900/5799
2019-07-12T14:32:11.5741414Z ......................................................................ii............................ 4000/5799
2019-07-12T14:32:14.4454030Z ...........................................................................................i........ 4100/5799
2019-07-12T14:32:16.7260837Z .................................................................................................... 4200/5799
2019-07-12T14:32:18.7994665Z .......................................................i............................................ 4300/5799
2019-07-12T14:32:34.1305973Z .................................................................................................... 4500/5799
2019-07-12T14:32:44.7160317Z .................................................................................................... 4600/5799
2019-07-12T14:32:48.3474391Z .................................................................................................... 4700/5799
2019-07-12T14:32:52.3752434Z .................................................................................................... 4800/5799
---
2019-07-12T14:33:26.4896284Z .................................................................................................... 5400/5799
2019-07-12T14:33:31.3706005Z .................................................................................................... 5500/5799
2019-07-12T14:33:34.5798420Z .................................................................................................... 5600/5799
2019-07-12T14:33:37.7855170Z .................................................................................................... 5700/5799
2019-07-12T14:33:41.0429913Z .......................................i...........................................................
2019-07-12T14:33:41.0433449Z 
2019-07-12T14:33:41.0518372Z  finished in 266.552
2019-07-12T14:33:41.0695789Z Check compiletest suite=run-pass mode=run-pass (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-07-12T14:33:41.3109321Z 
2019-07-12T14:33:41.3109321Z 
2019-07-12T14:33:41.3109446Z running 2917 tests
2019-07-12T14:33:56.1598948Z .................................................................................................... 100/2917
2019-07-12T14:34:11.6401009Z ............................................................................i....................... 200/2917
2019-07-12T14:34:36.6088953Z .................................................................................................... 400/2917
2019-07-12T14:34:48.4028685Z .................................................................................................... 500/2917
2019-07-12T14:35:03.0653517Z .................................................................................................... 600/2917
2019-07-12T14:35:24.7929383Z .................................................................................................... 700/2917
2019-07-12T14:35:24.7929383Z .................................................................................................... 700/2917
2019-07-12T14:35:39.2056622Z .................................................................................................... 800/2917
2019-07-12T14:35:52.0426634Z .................................................................................................... 900/2917
2019-07-12T14:36:08.3568222Z .................................................................................................... 1000/2917
2019-07-12T14:36:22.6245598Z .................................................................................................... 1100/2917
2019-07-12T14:36:34.1220519Z .................................................................................................... 1200/2917
2019-07-12T14:36:46.8673353Z .................................................................................................... 1300/2917
2019-07-12T14:37:03.4562409Z .....................ii............................................................................. 1400/2917
2019-07-12T14:37:16.9214723Z .................................................................................................... 1500/2917
2019-07-12T14:37:28.8722097Z .........................................................................i.......i.................. 1600/2917
2019-07-12T14:38:02.8177293Z .................................................................................................... 1800/2917
2019-07-12T14:38:17.0690760Z .................................................................................................... 1900/2917
2019-07-12T14:38:17.0690760Z .................................................................................................... 1900/2917
2019-07-12T14:38:34.5664708Z ...i.......................................................................i........................ 2000/2917
2019-07-12T14:39:22.6717468Z .................................................................................................... 2200/2917
2019-07-12T14:39:34.8018326Z .................................................................................................... 2300/2917
2019-07-12T14:39:34.8018326Z .................................................................................................... 2300/2917
2019-07-12T14:39:56.7512221Z ..........ii........................................................................................ 2400/2917
2019-07-12T14:40:52.7654170Z .................................................................................................... 2600/2917
2019-07-12T14:41:05.4569107Z .................................................................................................... 2700/2917
2019-07-12T14:41:18.0889005Z .................................................................................................... 2800/2917
2019-07-12T14:41:33.5688227Z .................................................................................................... 2900/2917
---
2019-07-12T14:42:34.3607719Z  finished in 35.705
2019-07-12T14:42:34.3805719Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-07-12T14:42:34.5461787Z 
2019-07-12T14:42:34.5462610Z running 146 tests
2019-07-12T14:42:37.8426689Z i....iii......ii.i..iiii...i............................i..i................i....i.........ii.i.i..i 100/146
2019-07-12T14:42:39.7437778Z .ii..............i.........iii.i......ii......
2019-07-12T14:42:39.7438838Z 
2019-07-12T14:42:39.7446164Z  finished in 5.363
2019-07-12T14:42:39.7624019Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-07-12T14:42:39.9258538Z 
2019-07-12T14:42:39.9258538Z 
2019-07-12T14:42:39.9258742Z running 39 tests
2019-07-12T14:42:42.0151033Z i.........i......................i.....
2019-07-12T14:42:42.0151648Z 
2019-07-12T14:42:42.0154843Z  finished in 2.253
2019-07-12T14:42:42.0343656Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-07-12T14:42:42.2003753Z 
2019-07-12T14:42:42.2003753Z 
2019-07-12T14:42:42.2004098Z running 9 tests
2019-07-12T14:42:42.2004889Z iiiiiiiii
2019-07-12T14:42:42.2005255Z 
2019-07-12T14:42:42.2005305Z  finished in 0.165
2019-07-12T14:42:42.2185362Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-07-12T14:42:42.3851555Z 
---
2019-07-12T14:43:00.8967781Z  finished in 18.678
2019-07-12T14:43:00.9159441Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-07-12T14:43:01.0832279Z 
2019-07-12T14:43:01.0833707Z running 123 tests
2019-07-12T14:43:26.1942598Z .iiiii...i.....i..i...i..i.i.i..i.iiF.i.i.....i..i....i..........iiii..........i..FiiFF..i.......ii. 100/123
2019-07-12T14:43:30.9956182Z i.i.i......iii.i.....ii
2019-07-12T14:43:30.9956401Z 
2019-07-12T14:43:30.9956871Z ---- [debuginfo-gdb+lldb] debuginfo/gdb-pretty-struct-and-enums.rs stdout ----
2019-07-12T14:43:30.9956871Z ---- [debuginfo-gdb+lldb] debuginfo/gdb-pretty-struct-and-enums.rs stdout ----
2019-07-12T14:43:30.9956935Z NOTE: compiletest thinks it is using GDB without native rust support
2019-07-12T14:43:30.9957017Z NOTE: compiletest thinks it is using GDB version 7011001
2019-07-12T14:43:30.9957068Z 
2019-07-12T14:43:30.9957126Z error: line not found in debugger output: $1 = RegularStruct = {the_first_field = 101, the_second_field = 102.5, the_third_field = false}
2019-07-12T14:43:30.9957179Z status: exit code: 0
2019-07-12T14:43:30.9957782Z command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/gdb-pretty-struct-and-enums/gdb-pretty-struct-and-enums.debugger.script"
2019-07-12T14:43:30.9958077Z ------------------------------------------
2019-07-12T14:43:30.9958077Z ------------------------------------------
2019-07-12T14:43:30.9958319Z GNU gdb (Ubuntu 7.11.1-0ubuntu1~16.5) 7.11.1
2019-07-12T14:43:30.9958438Z Copyright (C) 2016 Free Software Foundation, Inc.
2019-07-12T14:43:30.9958491Z License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
2019-07-12T14:43:30.9958562Z This is free software: you are free to change and redistribute it.
2019-07-12T14:43:30.9959057Z There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
2019-07-12T14:43:30.9959125Z and "show warranty" for details.
2019-07-12T14:43:30.9959402Z This GDB was configured as "x86_64-linux-gnu".
2019-07-12T14:43:30.9959474Z Type "show configuration" for configuration details.
2019-07-12T14:43:30.9959525Z For bug reporting instructions, please see:
2019-07-12T14:43:30.9959585Z <http://www.gnu.org/software/gdb/bugs/>.
2019-07-12T14:43:30.9959655Z Find the GDB manual and other documentation resources online at:
2019-07-12T14:43:30.9959708Z <http://www.gnu.org/software/gdb/documentation/>.
2019-07-12T14:43:30.9959755Z For help, type "help".
2019-07-12T14:43:30.9959821Z Type "apropos word" to search for commands related to "word".
2019-07-12T14:43:30.9960111Z Breakpoint 1 at 0xaaf: file /checkout/src/test/debuginfo/gdb-pretty-struct-and-enums.rs, line 60.
2019-07-12T14:43:30.9960170Z [Thread debugging using libthread_db enabled]
2019-07-12T14:43:30.9960453Z Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
2019-07-12T14:43:30.9960492Z 
2019-07-12T14:43:30.9960812Z Breakpoint 1, gdb_pretty_struct_and_enums::main::hb77f1d849dda9a93 () at /checkout/src/test/debuginfo/gdb-pretty-struct-and-enums.rs:60
2019-07-12T14:43:30.9960887Z 60     zzz(); // #break
2019-07-12T14:43:30.9960940Z $1 = {the_first_field = 101, the_second_field = 102.5, the_third_field = false}
2019-07-12T14:43:30.9961116Z $2 = {<No data fields>}
2019-07-12T14:43:30.9961159Z $3 = CStyleEnumVar1
2019-07-12T14:43:30.9961220Z $4 = CStyleEnumVar2
2019-07-12T14:43:30.9961264Z $5 = CStyleEnumVar3
2019-07-12T14:43:30.9961309Z A debugging session is active.
2019-07-12T14:43:30.9961356Z 
2019-07-12T14:43:30.9961402Z  Inferior 1 [process 114941] will be killed.
2019-07-12T14:43:30.9961432Z 
2019-07-12T14:43:30.9961478Z Quit anyway? (y or n) [answered Y; input not from terminal]
2019-07-12T14:43:30.9961783Z ------------------------------------------
2019-07-12T14:43:30.9961834Z stderr:
2019-07-12T14:43:30.9962062Z ------------------------------------------
2019-07-12T14:43:30.9962113Z 
2019-07-12T14:43:30.9962113Z 
2019-07-12T14:43:30.9962335Z ------------------------------------------
2019-07-12T14:43:30.9962369Z 
2019-07-12T14:43:30.9962395Z 
2019-07-12T14:43:30.9962651Z ---- [debuginfo-gdb+lldb] debuginfo/pretty-huge-vec.rs stdout ----
2019-07-12T14:43:30.9962709Z NOTE: compiletest thinks it is using GDB without native rust support
2019-07-12T14:43:30.9962773Z NOTE: compiletest thinks it is using GDB version 7011001
2019-07-12T14:43:30.9962805Z 
2019-07-12T14:43:30.9962876Z error: line not found in debugger output: $1 = Vec<u8>(len: 1000000000, cap: 1000000000) = {[...]...}
2019-07-12T14:43:30.9962929Z status: exit code: 0
2019-07-12T14:43:30.9963604Z command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/pretty-huge-vec/pretty-huge-vec.debugger.script"
2019-07-12T14:43:30.9963936Z ------------------------------------------
2019-07-12T14:43:30.9963936Z ------------------------------------------
2019-07-12T14:43:30.9964180Z GNU gdb (Ubuntu 7.11.1-0ubuntu1~16.5) 7.11.1
2019-07-12T14:43:30.9964251Z Copyright (C) 2016 Free Software Foundation, Inc.
2019-07-12T14:43:30.9964305Z License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
2019-07-12T14:43:30.9964358Z This is free software: you are free to change and redistribute it.
2019-07-12T14:43:30.9964430Z There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
2019-07-12T14:43:30.9964489Z and "show warranty" for details.
2019-07-12T14:43:30.9964725Z This GDB was configured as "x86_64-linux-gnu".
2019-07-12T14:43:30.9964796Z Type "show configuration" for configuration details.
2019-07-12T14:43:30.9964847Z For bug reporting instructions, please see:
2019-07-12T14:43:30.9964895Z <http://www.gnu.org/software/gdb/bugs/>.
2019-07-12T14:43:30.9964963Z Find the GDB manual and other documentation resources online at:
2019-07-12T14:43:30.9965015Z <http://www.gnu.org/software/gdb/documentation/>.
2019-07-12T14:43:30.9965064Z For help, type "help".
2019-07-12T14:43:30.9965215Z Type "apropos word" to search for commands related to "word".
2019-07-12T14:43:30.9965542Z Breakpoint 1 at 0x18ec: file /checkout/src/test/debuginfo/pretty-huge-vec.rs, line 28.
2019-07-12T14:43:30.9965599Z [Thread debugging using libthread_db enabled]
2019-07-12T14:43:30.9965855Z Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
2019-07-12T14:43:30.9965918Z 
2019-07-12T14:43:30.9966212Z Breakpoint 1, pretty_huge_vec::main::h724d3ae1bb5250fc () at /checkout/src/test/debuginfo/pretty-huge-vec.rs:28
2019-07-12T14:43:30.9966269Z 28     zzz(); // #break
2019-07-12T14:43:30.9966343Z $1 = {buf = {ptr = {pointer = 0x7fffbb526010 "", _marker = {<No data fields>}}, cap = 1000000000, a = {<No data fields>}}, len = 1000000000}
2019-07-12T14:43:30.9966401Z $2 = {data_ptr = 0x7fffbb526010 "", length = 1000000000}
2019-07-12T14:43:30.9966448Z A debugging session is active.
2019-07-12T14:43:30.9966478Z 
2019-07-12T14:43:30.9966544Z  Inferior 1 [process 115647] will be killed.
2019-07-12T14:43:30.9966583Z 
2019-07-12T14:43:30.9966630Z Quit anyway? (y or n) [answered Y; input not from terminal]
2019-07-12T14:43:30.9966907Z ------------------------------------------
2019-07-12T14:43:30.9966955Z stderr:
2019-07-12T14:43:30.9967171Z ------------------------------------------
2019-07-12T14:43:30.9967220Z 
2019-07-12T14:43:30.9967220Z 
2019-07-12T14:43:30.9967547Z ------------------------------------------
2019-07-12T14:43:30.9967580Z 
2019-07-12T14:43:30.9967606Z 
2019-07-12T14:43:30.9967856Z ---- [debuginfo-gdb+lldb] debuginfo/pretty-uninitialized-vec.rs stdout ----
2019-07-12T14:43:30.9967931Z NOTE: compiletest thinks it is using GDB without native rust support
2019-07-12T14:43:30.9967984Z NOTE: compiletest thinks it is using GDB version 7011001
2019-07-12T14:43:30.9968015Z 
2019-07-12T14:43:30.9968082Z error: line not found in debugger output: $1 = Vec<i32>(len: [...], cap: [...])[...]
2019-07-12T14:43:30.9968133Z status: exit code: 0
2019-07-12T14:43:30.9968504Z command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/pretty-uninitialized-vec/pretty-uninitialized-vec.debugger.script"
2019-07-12T14:43:30.9968804Z ------------------------------------------
2019-07-12T14:43:30.9968804Z ------------------------------------------
2019-07-12T14:43:30.9969034Z GNU gdb (Ubuntu 7.11.1-0ubuntu1~16.5) 7.11.1
2019-07-12T14:43:30.9969114Z Copyright (C) 2016 Free Software Foundation, Inc.
2019-07-12T14:43:30.9969167Z License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
2019-07-12T14:43:30.9969219Z This is free software: you are free to change and redistribute it.
2019-07-12T14:43:30.9969292Z There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
2019-07-12T14:43:30.9969342Z and "show warranty" for details.
2019-07-12T14:43:30.9969574Z This GDB was configured as "x86_64-linux-gnu".
2019-07-12T14:43:30.9969643Z Type "show configuration" for configuration details.
2019-07-12T14:43:30.9969694Z For bug reporting instructions, please see:
2019-07-12T14:43:30.9969749Z <http://www.gnu.org/software/gdb/bugs/>.
2019-07-12T14:43:30.9969800Z Find the GDB manual and other documentation resources online at:
2019-07-12T14:43:30.9969871Z <http://www.gnu.org/software/gdb/documentation/>.
2019-07-12T14:43:30.9969918Z For help, type "help".
2019-07-12T14:43:30.9969966Z Type "apropos word" to search for commands related to "word".
2019-07-12T14:43:30.9970274Z Breakpoint 1 at 0x1010: file /checkout/src/test/debuginfo/pretty-uninitialized-vec.rs, line 21.
2019-07-12T14:43:30.9970331Z [Thread debugging using libthread_db enabled]
2019-07-12T14:43:30.9970589Z Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
2019-07-12T14:43:30.9970640Z 
2019-07-12T14:43:30.9970950Z Breakpoint 1, pretty_uninitialized_vec::main::h8ad4cf0b9701f4b8 () at /checkout/src/test/debuginfo/pretty-uninitialized-vec.rs:21
2019-07-12T14:43:30.9971008Z 21     zzz(); // #break
2019-07-12T14:43:30.9971156Z $1 = {buf = {ptr = {pointer = 0x7fffff7ff000, _marker = {<No data fields>}}, cap = 93824994345120, a = {<No data fields>}}, len = 140737488348608}
2019-07-12T14:43:30.9971220Z A debugging session is active.
2019-07-12T14:43:30.9971249Z 
2019-07-12T14:43:30.9971294Z  Inferior 1 [process 115676] will be killed.
2019-07-12T14:43:30.9971339Z 
2019-07-12T14:43:30.9971386Z Quit anyway? (y or n) [answered Y; input not from terminal]
2019-07-12T14:43:30.9971673Z ------------------------------------------
2019-07-12T14:43:30.9971738Z stderr:
2019-07-12T14:43:30.9971956Z ------------------------------------------
2019-07-12T14:43:30.9971988Z 
2019-07-12T14:43:30.9971988Z 
2019-07-12T14:43:30.9972206Z ------------------------------------------
2019-07-12T14:43:30.9972255Z 
2019-07-12T14:43:30.9972281Z 
2019-07-12T14:43:30.9972519Z ---- [debuginfo-gdb+lldb] debuginfo/rc_arc.rs stdout ----
2019-07-12T14:43:30.9972576Z NOTE: compiletest thinks it is using GDB without native rust support
2019-07-12T14:43:30.9972646Z NOTE: compiletest thinks it is using GDB version 7011001
2019-07-12T14:43:30.9972686Z 
2019-07-12T14:43:30.9972738Z error: line not found in debugger output: [...]$1 = strong=2, weak=1 = {value = 42, strong = 2, weak = 1}
2019-07-12T14:43:30.9972804Z status: exit code: 0
2019-07-12T14:43:30.9973136Z command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/rc_arc/rc_arc.debugger.script"
2019-07-12T14:43:30.9973898Z ------------------------------------------
2019-07-12T14:43:30.9973898Z ------------------------------------------
2019-07-12T14:43:30.9974133Z GNU gdb (Ubuntu 7.11.1-0ubuntu1~16.5) 7.11.1
2019-07-12T14:43:30.9974186Z Copyright (C) 2016 Free Software Foundation, Inc.
2019-07-12T14:43:30.9974253Z License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
2019-07-12T14:43:30.9974553Z This is free software: you are free to change and redistribute it.
2019-07-12T14:43:30.9974617Z There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
2019-07-12T14:43:30.9974684Z and "show warranty" for details.
2019-07-12T14:43:30.9974966Z This GDB was configured as "x86_64-linux-gnu".
2019-07-12T14:43:30.9975020Z Type "show configuration" for configuration details.
2019-07-12T14:43:30.9975068Z For bug reporting instructions, please see:
2019-07-12T14:43:30.9975133Z <http://www.gnu.org/software/gdb/bugs/>.
2019-07-12T14:43:30.9975184Z Find the GDB manual and other documentation resources online at:
2019-07-12T14:43:30.9975244Z <http://www.gnu.org/software/gdb/documentation/>.
2019-07-12T14:43:30.9975308Z For help, type "help".
2019-07-12T14:43:30.9975357Z Type "apropos word" to search for commands related to "word".
2019-07-12T14:43:30.9975411Z Breakpoint 1 at 0x39ce: file /checkout/src/test/debuginfo/rc_arc.rs, line 34.
2019-07-12T14:43:30.9975484Z [Thread debugging using libthread_db enabled]
2019-07-12T14:43:30.9975748Z Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
2019-07-12T14:43:30.9975785Z 
2019-07-12T14:43:30.9975860Z Breakpoint 1, rc_arc::main::h89750330289bbf81 () at /checkout/src/test/debuginfo/rc_arc.rs:34
2019-07-12T14:43:30.9975915Z 34     print!(""); // #break
2019-07-12T14:43:30.9975964Z $1 = {ptr = {pointer = 0x55555575b130}, phantom = {<No data fields>}}
2019-07-12T14:43:30.9976038Z $2 = {ptr = {pointer = 0x55555575b150}, phantom = {<No data fields>}}
2019-07-12T14:43:30.9976086Z A debugging session is active.
2019-07-12T14:43:30.9976122Z 
2019-07-12T14:43:30.9976166Z  Inferior 1 [process 115705] will be killed.
2019-07-12T14:43:30.9976212Z 
2019-07-12T14:43:30.9976259Z Quit anyway? (y or n) [answered Y; input not from terminal]
2019-07-12T14:43:30.9976516Z ------------------------------------------
2019-07-12T14:43:30.9976580Z stderr:
2019-07-12T14:43:30.9976798Z ------------------------------------------
2019-07-12T14:43:30.9976831Z 
2019-07-12T14:43:30.9976831Z 
2019-07-12T14:43:30.9977047Z ------------------------------------------
2019-07-12T14:43:30.9977095Z 
2019-07-12T14:43:30.9977122Z 
2019-07-12T14:43:30.9977148Z 
2019-07-12T14:43:30.9977192Z failures:
2019-07-12T14:43:30.9977561Z     [debuginfo-gdb+lldb] debuginfo/gdb-pretty-struct-and-enums.rs
2019-07-12T14:43:30.9977838Z     [debuginfo-gdb+lldb] debuginfo/pretty-huge-vec.rs
2019-07-12T14:43:30.9978082Z     [debuginfo-gdb+lldb] debuginfo/pretty-uninitialized-vec.rs
2019-07-12T14:43:30.9978311Z     [debuginfo-gdb+lldb] debuginfo/rc_arc.rs
2019-07-12T14:43:30.9978649Z test result: FAILED. 80 passed; 4 failed; 39 ignored; 0 measured; 0 filtered out
2019-07-12T14:43:30.9978687Z 
2019-07-12T14:43:30.9980602Z 
2019-07-12T14:43:30.9980666Z 
2019-07-12T14:43:30.9980666Z 
2019-07-12T14:43:30.9983493Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/debuginfo" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "debuginfo-gdb+lldb" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-07-12T14:43:30.9984013Z 
2019-07-12T14:43:30.9984045Z 
2019-07-12T14:43:30.9985310Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:535:22
2019-07-12T14:43:30.9985419Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-07-12T14:43:30.9985419Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-07-12T14:43:30.9991340Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-07-12T14:43:30.9991433Z Build completed unsuccessfully in 1:06:29
2019-07-12T14:43:32.3331378Z ##[error]Bash exited with code '1'.
2019-07-12T14:43:32.3374051Z ##[section]Starting: Checkout
2019-07-12T14:43:32.3376943Z ==============================================================================
2019-07-12T14:43:32.3377017Z Task         : Get sources
2019-07-12T14:43:32.3377065Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
