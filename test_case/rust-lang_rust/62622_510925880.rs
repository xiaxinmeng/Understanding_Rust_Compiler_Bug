plain
2019-07-12T13:23:04.5504936Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-12T13:23:04.5747754Z ##[command]git config gc.auto 0
2019-07-12T13:23:04.5822399Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-12T13:23:04.5877988Z ##[command]git config --get-all http.proxy
2019-07-12T13:23:04.6000832Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62622/merge:refs/remotes/pull/62622/merge
---
2019-07-12T13:23:39.0039134Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-12T13:23:39.0039521Z 
2019-07-12T13:23:39.0039902Z   git checkout -b <new-branch-name>
2019-07-12T13:23:39.0040075Z 
2019-07-12T13:23:39.0040224Z HEAD is now at d31a4604d Merge 60ddabbdcb102987b090d46358f2a56820a3b562 into cd1381e91ff4889616eb0c87bf3c321ea2697d42
2019-07-12T13:23:39.0163437Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-12T13:23:39.0165776Z ==============================================================================
2019-07-12T13:23:39.0165837Z Task         : Bash
2019-07-12T13:23:39.0165872Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-12T13:23:39.5452314Z 320 ./src/librustc/mir
2019-07-12T13:23:39.5452529Z sort: write failed: 'standard output': Broken pipe
2019-07-12T13:23:39.5452606Z sort: write error
2019-07-12T13:23:39.5527611Z ##[section]Finishing: Show disk usage
2019-07-12T13:23:39.5574351Z ##[section]Starting: Disable git automatic line ending conversion (on C:/)
2019-07-12T13:23:39.5576299Z Task         : Bash
2019-07-12T13:23:39.5576334Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-12T13:23:39.5576368Z Version      : 3.151.2
2019-07-12T13:23:39.5576408Z Author       : Microsoft Corporation
2019-07-12T13:23:39.5576408Z Author       : Microsoft Corporation
2019-07-12T13:23:39.5576445Z Help         : https://docs.microsoft.com/azure/devops/pipelines/tasks/utility/bash
2019-07-12T13:23:39.5576484Z ==============================================================================
2019-07-12T13:23:39.6824903Z Generating script.
2019-07-12T13:23:39.6837121Z Script contents:
2019-07-12T13:23:39.6837465Z git config --replace-all --global core.autocrlf false
2019-07-12T13:23:39.6854699Z ========================== Starting Command Output ===========================
2019-07-12T13:23:39.6874797Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/e870fcd2-691c-4e34-8e93-bbd7d5282b0e.sh
2019-07-12T13:23:39.7000563Z ##[section]Finishing: Disable git automatic line ending conversion (on C:/)
2019-07-12T13:23:39.7037067Z ==============================================================================
2019-07-12T13:23:39.7037153Z Task         : Bash
2019-07-12T13:23:39.7037190Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-12T13:23:39.7037225Z Version      : 3.151.2
---
2019-07-12T13:25:35.4735380Z Attempting with retry: curl -y 30 -Y 10 --connect-timeout 30 -f -L -C - -o /tmp/rustci_docker_cache https://.s3.amazonaws.com/docker/a4940e6914a5e1f6360ebc241d0d850aff083be5c43d268fd716ce48f63cb24e84238becb662f27f8c5d7740413ca17da24bb9cdc003ef32d7b03d2c9052b94d
2019-07-12T13:25:35.4784838Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-12T13:25:35.4785309Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-12T13:25:35.4785381Z 
2019-07-12T13:25:35.4827586Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-12T13:25:36.4893913Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-12T13:25:36.4894333Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-12T13:25:36.4894580Z 
2019-07-12T13:25:36.4894580Z 
2019-07-12T13:25:36.4937158Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-12T13:25:38.4998537Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-12T13:25:38.4998746Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-12T13:25:38.4998777Z 
2019-07-12T13:25:38.4998777Z 
2019-07-12T13:25:38.5042760Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-12T13:25:41.5107201Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-12T13:25:41.5107483Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-12T13:25:41.5107795Z 
2019-07-12T13:25:41.5107795Z 
2019-07-12T13:25:41.5150712Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-12T13:25:45.5219390Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-12T13:25:45.5219747Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-12T13:25:45.5219963Z 
2019-07-12T13:25:45.5219963Z 
2019-07-12T13:25:45.5263766Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-12T13:25:45.5270251Z The command has failed after 5 attempts.
2019-07-12T13:25:45.6508523Z open /tmp/rustci_docker_cache: no such file or directory
2019-07-12T13:25:45.6531219Z Attempting with retry: docker build --rm -t rust-ci -f /home/vsts/work/1/s/src/ci/docker/mingw-check/Dockerfile /home/vsts/work/1/s/src/ci/docker
2019-07-12T13:25:45.8572807Z Sending build context to Docker daemon  521.7kB
2019-07-12T13:25:45.8573538Z 
2019-07-12T13:25:45.8760034Z Step 1/6 : FROM ubuntu:16.04
---
2019-07-12T13:26:00.9260522Z Reading package lists...
2019-07-12T13:26:01.7723728Z Reading package lists...
2019-07-12T13:26:01.9127270Z Building dependency tree...
2019-07-12T13:26:01.9127465Z Reading state information...
2019-07-12T13:26:02.0142588Z The following additional packages will be installed:
2019-07-12T13:26:02.0154962Z   binutils binutils-mingw-w64-i686 binutils-mingw-w64-x86-64 bzip2 cmake-data
2019-07-12T13:26:02.0156333Z   cpp cpp-5 dpkg-dev g++-5 g++-mingw-w64 g++-mingw-w64-i686
2019-07-12T13:26:02.0158009Z   g++-mingw-w64-x86-64 gcc gcc-5 gcc-mingw-w64 gcc-mingw-w64-base
2019-07-12T13:26:02.0159682Z   gcc-mingw-w64-i686 gcc-mingw-w64-x86-64 git-man libarchive13 libasan2
2019-07-12T13:26:02.0160761Z   libasn1-8-heimdal libatomic1 libbabeltrace-ctf1 libbabeltrace1 libbz2-1.0
2019-07-12T13:26:02.0163279Z   libdpkg-perl liberror-perl libexpat1 libffi6 libgcc-5-dev libgdbm3
2019-07-12T13:26:02.0164553Z   libglib2.0-0 libgmp10 libgnutls30 libgomp1 libgssapi-krb5-2
2019-07-12T13:26:02.0165138Z   libgssapi3-heimdal libhcrypto4-heimdal libheimbase1-heimdal
2019-07-12T13:26:02.0172201Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-12T13:26:02.0172201Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-12T13:26:02.0172571Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-12T13:26:02.0176961Z   libkrb5-3 libkrb5support0 libldap-2.4-2 liblsan0 liblzo2-2 libmagic1 libmpc3
2019-07-12T13:26:02.0177381Z   libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0 libperl5.22
2019-07-12T13:26:02.0177789Z   libpython2.7-minimal libpython2.7-stdlib libpython3.5 libpython3.5-minimal
2019-07-12T13:26:02.0180151Z   libpython3.5-stdlib libquadmath0 libroken18-heimdal librtmp1 libsasl2-2
2019-07-12T13:26:02.0180679Z   libsasl2-modules-db libsqlite3-0 libssl1.0.0 libstdc++-5-dev libtasn1-6
2019-07-12T13:26:02.0180894Z   libtsan0 libubsan0 libwind0-heimdal libxml2 linux-libc-dev mime-support
2019-07-12T13:26:02.0181178Z   mingw-w64-common mingw-w64-i686-dev mingw-w64-x86-64-dev openssl patch perl
2019-07-12T13:26:02.0181388Z   perl-modules-5.22 python2.7-minimal zlib1g-dev
2019-07-12T13:26:02.0181432Z Suggested packages:
2019-07-12T13:26:02.0181853Z   binutils-doc bzip2-doc codeblocks eclipse ninja-build cpp-doc gcc-5-locales
2019-07-12T13:26:02.0182076Z   debian-keyring g++-multilib g++-5-multilib gcc-5-doc libstdc++6-5-dbg
2019-07-12T13:26:02.0182287Z   gcc-multilib manpages-dev autoconf automake libtool flex bison gcc-doc
2019-07-12T13:26:02.0182768Z   libasan2-dbg liblsan0-dbg libtsan0-dbg libubsan0-dbg libcilkrts5-dbg
2019-07-12T13:26:02.0183134Z   libmpx0-dbg libquadmath0-dbg gdb-doc gettext-base git-daemon-run
2019-07-12T13:26:02.0183134Z   libmpx0-dbg libquadmath0-dbg gdb-doc gettext-base git-daemon-run
2019-07-12T13:26:02.0185284Z   | git-daemon-sysvinit git-doc git-el git-email git-gui gitk gitweb git-arch
2019-07-12T13:26:02.0185590Z   git-cvs git-mediawiki git-svn lrzip glibc-doc gnutls-bin krb5-doc krb5-user
2019-07-12T13:26:02.0185850Z   libstdc++-5-doc make-doc wine wine64 ed diffutils-doc perl-doc
2019-07-12T13:26:02.0186144Z   libterm-readline-gnu-perl | libterm-readline-perl-perl python2.7-doc
2019-07-12T13:26:02.0186347Z   binfmt-support
2019-07-12T13:26:02.0186395Z Recommended packages:
2019-07-12T13:26:02.0186633Z   build-essential fakeroot libalgorithm-merge-perl gfortran-mingw-w64
2019-07-12T13:26:02.0186928Z   gnat-mingw-w64 libc-dbg gdbserver less rsync ssh-client manpages
2019-07-12T13:26:02.0187171Z   manpages-dev libfile-fcntllock-perl libglib2.0-data shared-mime-info
2019-07-12T13:26:02.0187413Z   xdg-user-dirs krb5-locales libsasl2-modules libssl-doc xml-core netbase
2019-07-12T13:26:02.0187518Z   rename
2019-07-12T13:26:02.0187884Z The following NEW packages will be installed:
2019-07-12T13:26:02.0188119Z   binutils binutils-mingw-w64-i686 binutils-mingw-w64-x86-64 bzip2
2019-07-12T13:26:02.0188409Z   ca-certificates cmake cmake-data cpp cpp-5 curl dpkg-dev file g++ g++-5
2019-07-12T13:26:02.0190535Z   g++-mingw-w64 g++-mingw-w64-i686 g++-mingw-w64-x86-64 gcc gcc-5
2019-07-12T13:26:02.0191178Z   gcc-mingw-w64 gcc-mingw-w64-base gcc-mingw-w64-i686 gcc-mingw-w64-x86-64 gdb
2019-07-12T13:26:02.0191577Z   git git-man libarchive13 libasan2 libasn1-8-heimdal libatomic1
2019-07-12T13:26:02.0191774Z   libbabeltrace-ctf1 libbabeltrace1 libc-dev-bin libc6-dev libcc1-0
2019-07-12T13:26:02.0192222Z   libffi6 libgcc-5-dev libgdbm3 libglib2.0-0 libgmp10 libgnutls30 libgomp1
2019-07-12T13:26:02.0192419Z   libgssapi-krb5-2 libgssapi3-heimdal libhcrypto4-heimdal libheimbase1-heimdal
2019-07-12T13:26:02.0192738Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-12T13:26:02.0192989Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-12T13:26:02.0192989Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-12T13:26:02.0193192Z   libkrb5-3 libkrb5support0 libldap-2.4-2 liblsan0 liblzo2-2 libmagic1 libmpc3
2019-07-12T13:26:02.0193390Z   libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0 libperl5.22
2019-07-12T13:26:02.0193641Z   libpython2.7-minimal libpython2.7-stdlib libpython3.5 libpython3.5-minimal
2019-07-12T13:26:02.0193838Z   libpython3.5-stdlib libquadmath0 libroken18-heimdal librtmp1 libsasl2-2
2019-07-12T13:26:02.0194032Z   libsasl2-modules-db libsqlite3-0 libssl-dev libssl1.0.0 libstdc++-5-dev
2019-07-12T13:26:02.0194682Z   libtasn1-6 libtsan0 libubsan0 libwind0-heimdal libxml2 linux-libc-dev make
2019-07-12T13:26:02.0194929Z   mime-support mingw-w64 mingw-w64-common mingw-w64-i686-dev
2019-07-12T13:26:02.0195170Z   mingw-w64-x86-64-dev openssl patch perl perl-modules-5.22 pkg-config
2019-07-12T13:26:02.0195477Z   python2.7 python2.7-minimal sudo xz-utils zlib1g-dev
2019-07-12T13:26:02.0195531Z The following packages will be upgraded:
2019-07-12T13:26:02.3238280Z 1 upgraded, 112 newly installed, 0 to remove and 4 not upgraded.
2019-07-12T13:26:02.3238959Z Need to get 187 MB of archives.
2019-07-12T13:26:02.3239070Z After this operation, 968 MB of additional disk space will be used.
2019-07-12T13:26:02.3239811Z Get:1 http://archive.ubuntu.com/ubuntu xenial/main amd64 libgdbm3 amd64 1.8.3-13.1 [16.9 kB]
---
2019-07-12T13:26:49.3899859Z Get:97 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 zlib1g-dev amd64 1:1.2.8.dfsg-2ubuntu4.1 [168 kB]
2019-07-12T13:26:49.5024230Z Get:98 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 libssl-dev amd64 1.0.2g-1ubuntu4.15 [1344 kB]
2019-07-12T13:26:50.4765077Z Get:99 http://archive.ubuntu.com/ubuntu xenial/main amd64 pkg-config amd64 0.29.1-0ubuntu1 [45.0 kB]
2019-07-12T13:26:50.5104970Z Get:100 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 python2.7 amd64 2.7.12-1ubuntu0~16.04.4 [224 kB]
2019-07-12T13:26:50.6740314Z Get:101 http://archive.ubuntu.com/ubuntu xenial/universe amd64 binutils-mingw-w64-i686 amd64 2.26-3ubuntu1+6.6 [1782 kB]
2019-07-12T13:26:52.0164860Z Get:102 http://archive.ubuntu.com/ubuntu xenial/universe amd64 binutils-mingw-w64-x86-64 amd64 2.26-3ubuntu1+6.6 [2029 kB]
2019-07-12T13:26:52.4659482Z Get:103 http://archive.ubuntu.com/ubuntu xenial/universe amd64 mingw-w64-common all 4.0.4-2 [4787 kB]
2019-07-12T13:26:52.8127621Z Get:104 http://archive.ubuntu.com/ubuntu xenial/universe amd64 mingw-w64-i686-dev all 4.0.4-2 [2059 kB]
2019-07-12T13:26:52.9883783Z Get:105 http://archive.ubuntu.com/ubuntu xenial/universe amd64 gcc-mingw-w64-base amd64 5.3.1-8ubuntu3+17 [11.2 kB]
2019-07-12T13:26:52.9889635Z Get:106 http://archive.ubuntu.com/ubuntu xenial/universe amd64 gcc-mingw-w64-i686 amd64 5.3.1-8ubuntu3+17 [27.3 MB]
2019-07-12T13:27:09.1968749Z Get:107 http://archive.ubuntu.com/ubuntu xenial/universe amd64 g++-mingw-w64-i686 amd64 5.3.1-8ubuntu3+17 [19.8 MB]
2019-07-12T13:27:22.8656556Z Get:108 http://archive.ubuntu.com/ubuntu xenial/universe amd64 mingw-w64-x86-64-dev all 4.0.4-2 [3238 kB]
2019-07-12T13:27:24.9775567Z Get:109 http://archive.ubuntu.com/ubuntu xenial/universe amd64 gcc-mingw-w64-x86-64 amd64 5.3.1-8ubuntu3+17 [27.4 MB]
2019-07-12T13:27:43.4764527Z Get:110 http://archive.ubuntu.com/ubuntu xenial/universe amd64 g++-mingw-w64-x86-64 amd64 5.3.1-8ubuntu3+17 [20.4 MB]
2019-07-12T13:27:57.8562438Z Get:111 http://archive.ubuntu.com/ubuntu xenial/universe amd64 g++-mingw-w64 all 5.3.1-8ubuntu3+17 [10.7 kB]
2019-07-12T13:27:57.8566703Z Get:112 http://archive.ubuntu.com/ubuntu xenial/universe amd64 gcc-mingw-w64 all 5.3.1-8ubuntu3+17 [10.7 kB]
2019-07-12T13:27:57.8572932Z Get:113 http://archive.ubuntu.com/ubuntu xenial/universe amd64 mingw-w64 all 4.0.4-2 [9274 B]
2019-07-12T13:27:58.4254798Z debconf: delaying package configuration, since apt-utils is not installed
2019-07-12T13:27:58.4490208Z Fetched 187 MB in 1min 55s (1619 kB/s)
2019-07-12T13:27:58.5226915Z (Reading database ... 
2019-07-12T13:27:58.5227277Z (Reading database ... 5%
2019-07-12T13:27:58.5227578Z (Reading database ... 10%
2019-07-12T13:27:58.5227796Z (Reading database ... 15%
---
2019-07-12T13:28:38.0562204Z Unpacking git (1:2.7.4-0ubuntu1.6) ...
2019-07-12T13:28:40.1969464Z Selecting previously unselected package libpython2.7-stdlib:amd64.
2019-07-12T13:28:40.1987244Z Preparing to unpack .../libpython2.7-stdlib_2.7.12-1ubuntu0~16.04.4_amd64.deb ...
2019-07-12T13:28:40.3479596Z Unpacking libpython2.7-stdlib:amd64 (2.7.12-1ubuntu0~16.04.4) ...
2019-07-12T13:28:41.8008765Z Selecting previously unselected package zlib1g-dev:amd64.
2019-07-12T13:28:41.8027473Z Preparing to unpack .../zlib1g-dev_1%3a1.2.8.dfsg-2ubuntu4.1_amd64.deb ...
2019-07-12T13:28:41.9495019Z Unpacking zlib1g-dev:amd64 (1:1.2.8.dfsg-2ubuntu4.1) ...
2019-07-12T13:28:42.6527521Z Preparing to unpack .../libssl-dev_1.0.2g-1ubuntu4.15_amd64.deb ...
2019-07-12T13:28:42.8007573Z Unpacking libssl-dev:amd64 (1.0.2g-1ubuntu4.15) ...
2019-07-12T13:28:44.2471926Z Selecting previously unselected package pkg-config.
2019-07-12T13:28:44.2491600Z Preparing to unpack .../pkg-config_0.29.1-0ubuntu1_amd64.deb ...
2019-07-12T13:28:44.2491600Z Preparing to unpack .../pkg-config_0.29.1-0ubuntu1_amd64.deb ...
2019-07-12T13:28:44.4986711Z Unpacking pkg-config (0.29.1-0ubuntu1) ...
2019-07-12T13:28:46.1479032Z Selecting previously unselected package python2.7.
2019-07-12T13:28:46.1496160Z Preparing to unpack .../python2.7_2.7.12-1ubuntu0~16.04.4_amd64.deb ...
2019-07-12T13:28:46.3524593Z Unpacking python2.7 (2.7.12-1ubuntu0~16.04.4) ...
2019-07-12T13:28:47.2005343Z Selecting previously unselected package binutils-mingw-w64-i686.
2019-07-12T13:28:47.2023518Z Preparing to unpack .../binutils-mingw-w64-i686_2.26-3ubuntu1+6.6_amd64.deb ...
2019-07-12T13:28:47.3476872Z Unpacking binutils-mingw-w64-i686 (2.26-3ubuntu1+6.6) ...
2019-07-12T13:28:48.7476173Z Selecting previously unselected package binutils-mingw-w64-x86-64.
2019-07-12T13:28:48.7492684Z Preparing to unpack .../binutils-mingw-w64-x86-64_2.26-3ubuntu1+6.6_amd64.deb ...
2019-07-12T13:28:48.7883777Z Unpacking binutils-mingw-w64-x86-64 (2.26-3ubuntu1+6.6) ...
2019-07-12T13:28:49.3835766Z Selecting previously unselected package mingw-w64-common.
2019-07-12T13:28:49.3854596Z Preparing to unpack .../mingw-w64-common_4.0.4-2_all.deb ...
2019-07-12T13:28:49.4013253Z Unpacking mingw-w64-common (4.0.4-2) ...
2019-07-12T13:28:50.8436754Z Selecting previously unselected package mingw-w64-i686-dev.
2019-07-12T13:28:50.8460298Z Preparing to unpack .../mingw-w64-i686-dev_4.0.4-2_all.deb ...
2019-07-12T13:28:50.8613764Z Unpacking mingw-w64-i686-dev (4.0.4-2) ...
2019-07-12T13:28:52.2658279Z Selecting previously unselected package gcc-mingw-w64-base.
2019-07-12T13:28:52.2679934Z Preparing to unpack .../gcc-mingw-w64-base_5.3.1-8ubuntu3+17_amd64.deb ...
2019-07-12T13:28:52.2838539Z Unpacking gcc-mingw-w64-base (5.3.1-8ubuntu3+17) ...
2019-07-12T13:28:52.3936268Z Selecting previously unselected package gcc-mingw-w64-i686.
2019-07-12T13:28:52.3957907Z Preparing to unpack .../gcc-mingw-w64-i686_5.3.1-8ubuntu3+17_amd64.deb ...
2019-07-12T13:28:52.4112486Z Unpacking gcc-mingw-w64-i686 (5.3.1-8ubuntu3+17) ...
2019-07-12T13:28:56.3769089Z Selecting previously unselected package g++-mingw-w64-i686.
2019-07-12T13:28:56.3788912Z Preparing to unpack .../g++-mingw-w64-i686_5.3.1-8ubuntu3+17_amd64.deb ...
2019-07-12T13:28:56.3952645Z Unpacking g++-mingw-w64-i686 (5.3.1-8ubuntu3+17) ...
2019-07-12T13:28:59.9346837Z Selecting previously unselected package mingw-w64-x86-64-dev.
2019-07-12T13:28:59.9374178Z Preparing to unpack .../mingw-w64-x86-64-dev_4.0.4-2_all.deb ...
2019-07-12T13:28:59.9527076Z Unpacking mingw-w64-x86-64-dev (4.0.4-2) ...
2019-07-12T13:29:02.1160756Z Selecting previously unselected package gcc-mingw-w64-x86-64.
2019-07-12T13:29:02.1192382Z Preparing to unpack .../gcc-mingw-w64-x86-64_5.3.1-8ubuntu3+17_amd64.deb ...
2019-07-12T13:29:02.1380176Z Unpacking gcc-mingw-w64-x86-64 (5.3.1-8ubuntu3+17) ...
2019-07-12T13:29:06.0774567Z Selecting previously unselected package g++-mingw-w64-x86-64.
2019-07-12T13:29:06.0803053Z Preparing to unpack .../g++-mingw-w64-x86-64_5.3.1-8ubuntu3+17_amd64.deb ...
2019-07-12T13:29:06.0965336Z Unpacking g++-mingw-w64-x86-64 (5.3.1-8ubuntu3+17) ...
2019-07-12T13:29:09.7659357Z Selecting previously unselected package g++-mingw-w64.
2019-07-12T13:29:09.7691384Z Preparing to unpack .../g++-mingw-w64_5.3.1-8ubuntu3+17_all.deb ...
2019-07-12T13:29:09.7894321Z Unpacking g++-mingw-w64 (5.3.1-8ubuntu3+17) ...
2019-07-12T13:29:09.9187167Z Selecting previously unselected package gcc-mingw-w64.
2019-07-12T13:29:09.9212858Z Preparing to unpack .../gcc-mingw-w64_5.3.1-8ubuntu3+17_all.deb ...
2019-07-12T13:29:09.9390192Z Unpacking gcc-mingw-w64 (5.3.1-8ubuntu3+17) ...
2019-07-12T13:29:10.0302711Z Selecting previously unselected package mingw-w64.
2019-07-12T13:29:10.0329635Z Preparing to unpack .../mingw-w64_4.0.4-2_all.deb ...
2019-07-12T13:29:10.0483261Z Unpacking mingw-w64 (4.0.4-2) ...
2019-07-12T13:29:10.3599525Z Setting up libgdbm3:amd64 (1.8.3-13.1) ...
2019-07-12T13:29:10.4476221Z Setting up libffi6:amd64 (3.2.1-4) ...
2019-07-12T13:29:10.4943186Z Setting up libglib2.0-0:amd64 (2.48.2-0ubuntu4.3) ...
2019-07-12T13:29:10.5275648Z No schema files found: doing nothing.
---
2019-07-12T13:29:16.5050872Z Setting up zlib1g-dev:amd64 (1:1.2.8.dfsg-2ubuntu4.1) ...
2019-07-12T13:29:16.5537007Z Setting up libssl-dev:amd64 (1.0.2g-1ubuntu4.15) ...
2019-07-12T13:29:16.6005498Z Setting up pkg-config (0.29.1-0ubuntu1) ...
2019-07-12T13:29:16.6875186Z Setting up python2.7 (2.7.12-1ubuntu0~16.04.4) ...
2019-07-12T13:29:17.4456685Z Setting up binutils-mingw-w64-i686 (2.26-3ubuntu1+6.6) ...
2019-07-12T13:29:17.4913342Z Setting up binutils-mingw-w64-x86-64 (2.26-3ubuntu1+6.6) ...
2019-07-12T13:29:17.5373964Z Setting up mingw-w64-common (4.0.4-2) ...
2019-07-12T13:29:17.5845024Z Setting up mingw-w64-i686-dev (4.0.4-2) ...
2019-07-12T13:29:17.6343818Z Setting up gcc-mingw-w64-base (5.3.1-8ubuntu3+17) ...
2019-07-12T13:29:17.6817039Z Setting up gcc-mingw-w64-i686 (5.3.1-8ubuntu3+17) ...
2019-07-12T13:29:17.7144680Z update-alternatives: using /usr/bin/i686-w64-mingw32-gcc-posix to provide /usr/bin/i686-w64-mingw32-gcc (i686-w64-mingw32-gcc) in auto mode
2019-07-12T13:29:17.7149206Z update-alternatives: warning: skip creation of /usr/bin/i686-w64-mingw32-gcc-5 because associated file /usr/bin/i686-w64-mingw32-gcc-5-posix (of link group i686-w64-mingw32-gcc) doesn't exist
2019-07-12T13:29:17.7252353Z update-alternatives: using /usr/bin/i686-w64-mingw32-gcc-win32 to provide /usr/bin/i686-w64-mingw32-gcc (i686-w64-mingw32-gcc) in auto mode
2019-07-12T13:29:17.7253171Z update-alternatives: warning: skip creation of /usr/bin/i686-w64-mingw32-gcc-5 because associated file /usr/bin/i686-w64-mingw32-gcc-5-win32 (of link group i686-w64-mingw32-gcc) doesn't exist
2019-07-12T13:29:17.7536350Z Setting up g++-mingw-w64-i686 (5.3.1-8ubuntu3+17) ...
2019-07-12T13:29:17.7861383Z update-alternatives: using /usr/bin/i686-w64-mingw32-g++-posix to provide /usr/bin/i686-w64-mingw32-g++ (i686-w64-mingw32-g++) in auto mode
2019-07-12T13:29:17.7964649Z update-alternatives: using /usr/bin/i686-w64-mingw32-g++-win32 to provide /usr/bin/i686-w64-mingw32-g++ (i686-w64-mingw32-g++) in auto mode
2019-07-12T13:29:17.8265455Z Setting up mingw-w64-x86-64-dev (4.0.4-2) ...
2019-07-12T13:29:17.8764295Z Setting up gcc-mingw-w64-x86-64 (5.3.1-8ubuntu3+17) ...
2019-07-12T13:29:17.9089308Z update-alternatives: using /usr/bin/x86_64-w64-mingw32-gcc-posix to provide /usr/bin/x86_64-w64-mingw32-gcc (x86_64-w64-mingw32-gcc) in auto mode
2019-07-12T13:29:17.9092984Z update-alternatives: warning: skip creation of /usr/bin/x86_64-w64-mingw32-gcc-5 because associated file /usr/bin/x86_64-w64-mingw32-gcc-5-posix (of link group x86_64-w64-mingw32-gcc) doesn't exist
2019-07-12T13:29:17.9194956Z update-alternatives: using /usr/bin/x86_64-w64-mingw32-gcc-win32 to provide /usr/bin/x86_64-w64-mingw32-gcc (x86_64-w64-mingw32-gcc) in auto mode
2019-07-12T13:29:17.9197199Z update-alternatives: warning: skip creation of /usr/bin/x86_64-w64-mingw32-gcc-5 because associated file /usr/bin/x86_64-w64-mingw32-gcc-5-win32 (of link group x86_64-w64-mingw32-gcc) doesn't exist
2019-07-12T13:29:17.9484396Z Setting up g++-mingw-w64-x86-64 (5.3.1-8ubuntu3+17) ...
2019-07-12T13:29:17.9840626Z update-alternatives: using /usr/bin/x86_64-w64-mingw32-g++-posix to provide /usr/bin/x86_64-w64-mingw32-g++ (x86_64-w64-mingw32-g++) in auto mode
2019-07-12T13:29:17.9954909Z update-alternatives: using /usr/bin/x86_64-w64-mingw32-g++-win32 to provide /usr/bin/x86_64-w64-mingw32-g++ (x86_64-w64-mingw32-g++) in auto mode
2019-07-12T13:29:18.0227097Z Setting up g++-mingw-w64 (5.3.1-8ubuntu3+17) ...
2019-07-12T13:29:18.0839864Z Setting up gcc-mingw-w64 (5.3.1-8ubuntu3+17) ...
2019-07-12T13:29:18.1374358Z Setting up mingw-w64 (4.0.4-2) ...
2019-07-12T13:29:18.2358458Z Processing triggers for ca-certificates (20170717~16.04.2) ...
2019-07-12T13:29:18.2542932Z Updating certificates in /etc/ssl/certs...
2019-07-12T13:29:19.6961393Z 148 added, 0 removed; done.
2019-07-12T13:29:19.6962478Z Running hooks in /etc/ca-certificates/update.d...
---
2019-07-12T13:29:59.4691238Z Removing intermediate container b11993f0c053
2019-07-12T13:29:59.4692132Z  ---> cd21c236655c
2019-07-12T13:29:59.4730948Z Successfully built cd21c236655c
2019-07-12T13:29:59.7195908Z Successfully tagged rust-ci:latest
2019-07-12T13:29:59.8056387Z Built container sha256:cd21c236655cab52cfb896b677c2b042bcaf12ca90420dfd925bd45cd032b093
2019-07-12T13:29:59.8069278Z Uploading finished image to https://.s3.amazonaws.com/docker/a4940e6914a5e1f6360ebc241d0d850aff083be5c43d268fd716ce48f63cb24e84238becb662f27f8c5d7740413ca17da24bb9cdc003ef32d7b03d2c9052b94d
2019-07-12T13:31:20.6039207Z upload failed: - to s3:///docker/a4940e6914a5e1f6360ebc241d0d850aff083be5c43d268fd716ce48f63cb24e84238becb662f27f8c5d7740413ca17da24bb9cdc003ef32d7b03d2c9052b94d Parameter validation failed:
2019-07-12T13:31:20.6039641Z Invalid bucket name "": Bucket name must match the regex "^[a-zA-Z0-9.\-_]{1,255}$"
2019-07-12T13:31:21.6822745Z [CI_JOB_NAME=mingw-check]
2019-07-12T13:31:21.6867865Z Starting sccache server...
2019-07-12T13:31:21.7348140Z configure: processing command line
2019-07-12T13:31:21.7349262Z configure: 
---
2019-07-12T13:37:23.2356154Z configure: build.locked-deps    := True
2019-07-12T13:37:23.2356195Z configure: llvm.ccache          := sccache
2019-07-12T13:37:23.2356915Z configure: build.cargo-native-static := True
2019-07-12T13:37:23.2358580Z configure: dist.missing-tools   := True
2019-07-12T13:37:23.2358809Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2019-07-12T13:37:23.2358909Z configure: writing `config.toml` in current directory
2019-07-12T13:37:23.2358945Z configure: 
2019-07-12T13:37:23.2359126Z configure: run `python /checkout/x.py --help`
2019-07-12T13:37:23.2359194Z configure: 
---
2019-07-12T13:39:12.5533664Z     Checking hashbrown v0.4.0
2019-07-12T13:39:12.6007679Z error[E0308]: mismatched types
2019-07-12T13:39:12.6011460Z    --> src/libpanic_unwind/gcc.rs:280:67
2019-07-12T13:39:12.6012596Z     |
2019-07-12T13:39:12.6014213Z 280 | unsafe extern "C" fn rust_eh_unwind_resume(panic_ctx: *mut u8) -> ! {
2019-07-12T13:39:12.6016205Z     |                      ---------------------                        ^ expected !, found ()
2019-07-12T13:39:12.6018700Z     |                      |
2019-07-12T13:39:12.6020935Z     |                      this function's body doesn't return
2019-07-12T13:39:12.6024313Z     = note: expected type `!`
2019-07-12T13:39:12.6026612Z                found type `()`
2019-07-12T13:39:12.6027250Z 
2019-07-12T13:39:12.6033205Z error: aborting due to previous error
2019-07-12T13:39:12.6033205Z error: aborting due to previous error
2019-07-12T13:39:12.6033487Z 
2019-07-12T13:39:12.6034706Z For more information about this error, try `rustc --explain E0308`.
2019-07-12T13:39:12.6082500Z error: Could not compile `panic_unwind`.
2019-07-12T13:39:13.2465959Z error: build failed
2019-07-12T13:39:13.2465959Z error: build failed
2019-07-12T13:39:13.2490715Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
2019-07-12T13:39:13.2493870Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2019-07-12T13:39:13.2493942Z Build completed unsuccessfully in 0:00:26
2019-07-12T13:39:13.2493942Z Build completed unsuccessfully in 0:00:26
2019-07-12T13:39:21.4748953Z ##[error]Bash exited with code '1'.
2019-07-12T13:39:21.4785265Z ##[section]Starting: Checkout
2019-07-12T13:39:21.4786860Z ==============================================================================
2019-07-12T13:39:21.4786925Z Task         : Get sources
2019-07-12T13:39:21.4786969Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
