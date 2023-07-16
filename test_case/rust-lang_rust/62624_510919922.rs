plain
2019-07-12T14:51:51.4945693Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-12T14:51:51.5117313Z ##[command]git config gc.auto 0
2019-07-12T14:51:51.5183625Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-12T14:51:51.5224512Z ##[command]git config --get-all http.proxy
2019-07-12T14:51:52.4016122Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62624/merge:refs/remotes/pull/62624/merge
---
2019-07-12T14:52:27.7495767Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-12T14:52:27.7496664Z 
2019-07-12T14:52:27.7497973Z   git checkout -b <new-branch-name>
2019-07-12T14:52:27.7498846Z 
2019-07-12T14:52:27.7500789Z HEAD is now at 08467e538 Merge fbf2fb3e2f4d0b68c25751ad9699a792dc79baef into cd1381e91ff4889616eb0c87bf3c321ea2697d42
2019-07-12T14:52:27.7636199Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-12T14:52:27.7638510Z ==============================================================================
2019-07-12T14:52:27.7638720Z Task         : Bash
2019-07-12T14:52:27.7638775Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-12T14:52:28.2892878Z 320 ./src/librustc/mir
2019-07-12T14:52:28.2893471Z sort: write failed: 'standard output': Broken pipe
2019-07-12T14:52:28.2893649Z sort: write error
2019-07-12T14:52:28.2945595Z ##[section]Finishing: Show disk usage
2019-07-12T14:52:28.2995576Z ##[section]Starting: Disable git automatic line ending conversion (on C:/)
2019-07-12T14:52:28.2998227Z Task         : Bash
2019-07-12T14:52:28.2998264Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-12T14:52:28.2998303Z Version      : 3.151.2
2019-07-12T14:52:28.2998369Z Author       : Microsoft Corporation
2019-07-12T14:52:28.2998369Z Author       : Microsoft Corporation
2019-07-12T14:52:28.2998408Z Help         : https://docs.microsoft.com/azure/devops/pipelines/tasks/utility/bash
2019-07-12T14:52:28.2998448Z ==============================================================================
2019-07-12T14:52:28.4214775Z Generating script.
2019-07-12T14:52:28.4226941Z Script contents:
2019-07-12T14:52:28.4228302Z git config --replace-all --global core.autocrlf false
2019-07-12T14:52:28.4250116Z ========================== Starting Command Output ===========================
2019-07-12T14:52:28.4279293Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/3fdb62f2-f5a9-486d-a335-83ec9a979c04.sh
2019-07-12T14:52:28.4418396Z ##[section]Finishing: Disable git automatic line ending conversion (on C:/)
2019-07-12T14:52:28.4456557Z ==============================================================================
2019-07-12T14:52:28.4456643Z Task         : Bash
2019-07-12T14:52:28.4456847Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-12T14:52:28.4456885Z Version      : 3.151.2
---
2019-07-12T14:54:10.1125854Z Attempting with retry: curl -y 30 -Y 10 --connect-timeout 30 -f -L -C - -o /tmp/rustci_docker_cache https://.s3.amazonaws.com/docker/a4940e6914a5e1f6360ebc241d0d850aff083be5c43d268fd716ce48f63cb24e84238becb662f27f8c5d7740413ca17da24bb9cdc003ef32d7b03d2c9052b94d
2019-07-12T14:54:10.1181124Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-12T14:54:10.1181461Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-12T14:54:10.1181787Z 
2019-07-12T14:54:10.1215745Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-12T14:54:11.1278167Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-12T14:54:11.1278847Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-12T14:54:11.1279437Z 
2019-07-12T14:54:11.1279437Z 
2019-07-12T14:54:11.1320327Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-12T14:54:13.1396466Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-12T14:54:13.1396613Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-12T14:54:13.1396641Z 
2019-07-12T14:54:13.1396641Z 
2019-07-12T14:54:13.1432696Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-12T14:54:16.1517710Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-12T14:54:16.1518122Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-12T14:54:16.1518476Z 
2019-07-12T14:54:16.1518476Z 
2019-07-12T14:54:16.1521844Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-12T14:54:20.1636579Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-12T14:54:20.1641336Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-12T14:54:20.1642235Z 
2019-07-12T14:54:20.1642235Z 
2019-07-12T14:54:20.1643898Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-12T14:54:20.1646136Z The command has failed after 5 attempts.
2019-07-12T14:54:20.2275649Z open /tmp/rustci_docker_cache: no such file or directory
2019-07-12T14:54:20.2322096Z Attempting with retry: docker build --rm -t rust-ci -f /home/vsts/work/1/s/src/ci/docker/mingw-check/Dockerfile /home/vsts/work/1/s/src/ci/docker
2019-07-12T14:54:20.4173340Z Sending build context to Docker daemon  521.7kB
2019-07-12T14:54:20.4174031Z 
2019-07-12T14:54:20.4358157Z Step 1/6 : FROM ubuntu:16.04
---
2019-07-12T14:54:37.0836319Z Reading package lists...
2019-07-12T14:54:38.0262675Z Reading package lists...
2019-07-12T14:54:38.1779587Z Building dependency tree...
2019-07-12T14:54:38.1779683Z Reading state information...
2019-07-12T14:54:38.2848174Z The following additional packages will be installed:
2019-07-12T14:54:38.2848954Z   binutils binutils-mingw-w64-i686 binutils-mingw-w64-x86-64 bzip2 cmake-data
2019-07-12T14:54:38.2849186Z   cpp cpp-5 dpkg-dev g++-5 g++-mingw-w64 g++-mingw-w64-i686
2019-07-12T14:54:38.2849395Z   g++-mingw-w64-x86-64 gcc gcc-5 gcc-mingw-w64 gcc-mingw-w64-base
2019-07-12T14:54:38.2849654Z   gcc-mingw-w64-i686 gcc-mingw-w64-x86-64 git-man libarchive13 libasan2
2019-07-12T14:54:38.2850167Z   libasn1-8-heimdal libatomic1 libbabeltrace-ctf1 libbabeltrace1 libbz2-1.0
2019-07-12T14:54:38.2851282Z   libdpkg-perl liberror-perl libexpat1 libffi6 libgcc-5-dev libgdbm3
2019-07-12T14:54:38.2851552Z   libglib2.0-0 libgmp10 libgnutls30 libgomp1 libgssapi-krb5-2
2019-07-12T14:54:38.2851793Z   libgssapi3-heimdal libhcrypto4-heimdal libheimbase1-heimdal
2019-07-12T14:54:38.2852110Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-12T14:54:38.2852110Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-12T14:54:38.2852363Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-12T14:54:38.2852618Z   libkrb5-3 libkrb5support0 libldap-2.4-2 liblsan0 liblzo2-2 libmagic1 libmpc3
2019-07-12T14:54:38.2852908Z   libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0 libperl5.22
2019-07-12T14:54:38.2853161Z   libpython2.7-minimal libpython2.7-stdlib libpython3.5 libpython3.5-minimal
2019-07-12T14:54:38.2853423Z   libpython3.5-stdlib libquadmath0 libroken18-heimdal librtmp1 libsasl2-2
2019-07-12T14:54:38.2853722Z   libsasl2-modules-db libsqlite3-0 libssl1.0.0 libstdc++-5-dev libtasn1-6
2019-07-12T14:54:38.2853969Z   libtsan0 libubsan0 libwind0-heimdal libxml2 linux-libc-dev mime-support
2019-07-12T14:54:38.2854233Z   mingw-w64-common mingw-w64-i686-dev mingw-w64-x86-64-dev openssl patch perl
2019-07-12T14:54:38.2854966Z   perl-modules-5.22 python2.7-minimal zlib1g-dev
2019-07-12T14:54:38.2855009Z Suggested packages:
2019-07-12T14:54:38.2855210Z   binutils-doc bzip2-doc codeblocks eclipse ninja-build cpp-doc gcc-5-locales
2019-07-12T14:54:38.2855494Z   debian-keyring g++-multilib g++-5-multilib gcc-5-doc libstdc++6-5-dbg
2019-07-12T14:54:38.2855857Z   gcc-multilib manpages-dev autoconf automake libtool flex bison gcc-doc
2019-07-12T14:54:38.2856289Z   libasan2-dbg liblsan0-dbg libtsan0-dbg libubsan0-dbg libcilkrts5-dbg
2019-07-12T14:54:38.2856491Z   libmpx0-dbg libquadmath0-dbg gdb-doc gettext-base git-daemon-run
2019-07-12T14:54:38.2856491Z   libmpx0-dbg libquadmath0-dbg gdb-doc gettext-base git-daemon-run
2019-07-12T14:54:38.2856687Z   | git-daemon-sysvinit git-doc git-el git-email git-gui gitk gitweb git-arch
2019-07-12T14:54:38.2856924Z   git-cvs git-mediawiki git-svn lrzip glibc-doc gnutls-bin krb5-doc krb5-user
2019-07-12T14:54:38.2857121Z   libstdc++-5-doc make-doc wine wine64 ed diffutils-doc perl-doc
2019-07-12T14:54:38.2857315Z   libterm-readline-gnu-perl | libterm-readline-perl-perl python2.7-doc
2019-07-12T14:54:38.2857509Z   binfmt-support
2019-07-12T14:54:38.2857548Z Recommended packages:
2019-07-12T14:54:38.2857737Z   build-essential fakeroot libalgorithm-merge-perl gfortran-mingw-w64
2019-07-12T14:54:38.2857963Z   gnat-mingw-w64 libc-dbg gdbserver less rsync ssh-client manpages
2019-07-12T14:54:38.2858160Z   manpages-dev libfile-fcntllock-perl libglib2.0-data shared-mime-info
2019-07-12T14:54:38.2858354Z   xdg-user-dirs krb5-locales libsasl2-modules libssl-doc xml-core netbase
2019-07-12T14:54:38.2858442Z   rename
2019-07-12T14:54:38.2858482Z The following NEW packages will be installed:
2019-07-12T14:54:38.2858681Z   binutils binutils-mingw-w64-i686 binutils-mingw-w64-x86-64 bzip2
2019-07-12T14:54:38.2858914Z   ca-certificates cmake cmake-data cpp cpp-5 curl dpkg-dev file g++ g++-5
2019-07-12T14:54:38.2859232Z   g++-mingw-w64 g++-mingw-w64-i686 g++-mingw-w64-x86-64 gcc gcc-5
2019-07-12T14:54:38.2859475Z   gcc-mingw-w64 gcc-mingw-w64-base gcc-mingw-w64-i686 gcc-mingw-w64-x86-64 gdb
2019-07-12T14:54:38.2859709Z   git git-man libarchive13 libasan2 libasn1-8-heimdal libatomic1
2019-07-12T14:54:38.2859907Z   libbabeltrace-ctf1 libbabeltrace1 libc-dev-bin libc6-dev libcc1-0
2019-07-12T14:54:38.2860334Z   libffi6 libgcc-5-dev libgdbm3 libglib2.0-0 libgmp10 libgnutls30 libgomp1
2019-07-12T14:54:38.2860725Z   libgssapi-krb5-2 libgssapi3-heimdal libhcrypto4-heimdal libheimbase1-heimdal
2019-07-12T14:54:38.2861472Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-12T14:54:38.2861776Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-12T14:54:38.2861776Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-12T14:54:38.2862039Z   libkrb5-3 libkrb5support0 libldap-2.4-2 liblsan0 liblzo2-2 libmagic1 libmpc3
2019-07-12T14:54:38.2862289Z   libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0 libperl5.22
2019-07-12T14:54:38.2862584Z   libpython2.7-minimal libpython2.7-stdlib libpython3.5 libpython3.5-minimal
2019-07-12T14:54:38.2862840Z   libpython3.5-stdlib libquadmath0 libroken18-heimdal librtmp1 libsasl2-2
2019-07-12T14:54:38.2863087Z   libsasl2-modules-db libsqlite3-0 libssl-dev libssl1.0.0 libstdc++-5-dev
2019-07-12T14:54:38.2863378Z   libtasn1-6 libtsan0 libubsan0 libwind0-heimdal libxml2 linux-libc-dev make
2019-07-12T14:54:38.2863623Z   mime-support mingw-w64 mingw-w64-common mingw-w64-i686-dev
2019-07-12T14:54:38.2863870Z   mingw-w64-x86-64-dev openssl patch perl perl-modules-5.22 pkg-config
2019-07-12T14:54:38.2864154Z   python2.7 python2.7-minimal sudo xz-utils zlib1g-dev
2019-07-12T14:54:38.2864210Z The following packages will be upgraded:
2019-07-12T14:54:38.7272223Z 1 upgraded, 112 newly installed, 0 to remove and 4 not upgraded.
2019-07-12T14:54:38.7272314Z Need to get 187 MB of archives.
2019-07-12T14:54:38.7272390Z After this operation, 968 MB of additional disk space will be used.
2019-07-12T14:54:38.7273131Z Get:1 http://archive.ubuntu.com/ubuntu xenial/main amd64 libgdbm3 amd64 1.8.3-13.1 [16.9 kB]
---
2019-07-12T14:54:43.0057024Z Get:97 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 zlib1g-dev amd64 1:1.2.8.dfsg-2ubuntu4.1 [168 kB]
2019-07-12T14:54:43.0083166Z Get:98 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 libssl-dev amd64 1.0.2g-1ubuntu4.15 [1344 kB]
2019-07-12T14:54:43.0933824Z Get:99 http://archive.ubuntu.com/ubuntu xenial/main amd64 pkg-config amd64 0.29.1-0ubuntu1 [45.0 kB]
2019-07-12T14:54:43.0946931Z Get:100 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 python2.7 amd64 2.7.12-1ubuntu0~16.04.4 [224 kB]
2019-07-12T14:54:43.1028611Z Get:101 http://archive.ubuntu.com/ubuntu xenial/universe amd64 binutils-mingw-w64-i686 amd64 2.26-3ubuntu1+6.6 [1782 kB]
2019-07-12T14:54:43.3489202Z Get:102 http://archive.ubuntu.com/ubuntu xenial/universe amd64 binutils-mingw-w64-x86-64 amd64 2.26-3ubuntu1+6.6 [2029 kB]
2019-07-12T14:54:44.2716000Z Get:103 http://archive.ubuntu.com/ubuntu xenial/universe amd64 mingw-w64-common all 4.0.4-2 [4787 kB]
2019-07-12T14:54:44.4794901Z Get:104 http://archive.ubuntu.com/ubuntu xenial/universe amd64 mingw-w64-i686-dev all 4.0.4-2 [2059 kB]
2019-07-12T14:54:44.5577782Z Get:105 http://archive.ubuntu.com/ubuntu xenial/universe amd64 gcc-mingw-w64-base amd64 5.3.1-8ubuntu3+17 [11.2 kB]
2019-07-12T14:54:44.5580263Z Get:106 http://archive.ubuntu.com/ubuntu xenial/universe amd64 gcc-mingw-w64-i686 amd64 5.3.1-8ubuntu3+17 [27.3 MB]
2019-07-12T14:54:45.6097607Z Get:107 http://archive.ubuntu.com/ubuntu xenial/universe amd64 g++-mingw-w64-i686 amd64 5.3.1-8ubuntu3+17 [19.8 MB]
2019-07-12T14:54:46.3691732Z Get:108 http://archive.ubuntu.com/ubuntu xenial/universe amd64 mingw-w64-x86-64-dev all 4.0.4-2 [3238 kB]
2019-07-12T14:54:46.5083885Z Get:109 http://archive.ubuntu.com/ubuntu xenial/universe amd64 gcc-mingw-w64-x86-64 amd64 5.3.1-8ubuntu3+17 [27.4 MB]
2019-07-12T14:54:47.5519144Z Get:110 http://archive.ubuntu.com/ubuntu xenial/universe amd64 g++-mingw-w64-x86-64 amd64 5.3.1-8ubuntu3+17 [20.4 MB]
2019-07-12T14:54:48.3409900Z Get:111 http://archive.ubuntu.com/ubuntu xenial/universe amd64 g++-mingw-w64 all 5.3.1-8ubuntu3+17 [10.7 kB]
2019-07-12T14:54:48.3418207Z Get:112 http://archive.ubuntu.com/ubuntu xenial/universe amd64 gcc-mingw-w64 all 5.3.1-8ubuntu3+17 [10.7 kB]
2019-07-12T14:54:48.3426311Z Get:113 http://archive.ubuntu.com/ubuntu xenial/universe amd64 mingw-w64 all 4.0.4-2 [9274 B]
2019-07-12T14:54:51.0932884Z debconf: delaying package configuration, since apt-utils is not installed
2019-07-12T14:54:51.1143097Z Fetched 187 MB in 9s (19.0 MB/s)
2019-07-12T14:54:51.1906047Z (Reading database ... 
2019-07-12T14:54:51.1906424Z (Reading database ... 5%
2019-07-12T14:54:51.1906513Z (Reading database ... 10%
2019-07-12T14:54:51.1906551Z (Reading database ... 15%
---
2019-07-12T14:55:16.4075351Z Unpacking git (1:2.7.4-0ubuntu1.6) ...
2019-07-12T14:55:17.1642345Z Selecting previously unselected package libpython2.7-stdlib:amd64.
2019-07-12T14:55:17.1664231Z Preparing to unpack .../libpython2.7-stdlib_2.7.12-1ubuntu0~16.04.4_amd64.deb ...
2019-07-12T14:55:17.1802689Z Unpacking libpython2.7-stdlib:amd64 (2.7.12-1ubuntu0~16.04.4) ...
2019-07-12T14:55:17.5142828Z Selecting previously unselected package zlib1g-dev:amd64.
2019-07-12T14:55:17.5165083Z Preparing to unpack .../zlib1g-dev_1%3a1.2.8.dfsg-2ubuntu4.1_amd64.deb ...
2019-07-12T14:55:17.5299101Z Unpacking zlib1g-dev:amd64 (1:1.2.8.dfsg-2ubuntu4.1) ...
2019-07-12T14:55:17.6389402Z Preparing to unpack .../libssl-dev_1.0.2g-1ubuntu4.15_amd64.deb ...
2019-07-12T14:55:17.6519630Z Unpacking libssl-dev:amd64 (1.0.2g-1ubuntu4.15) ...
2019-07-12T14:55:18.0801245Z Selecting previously unselected package pkg-config.
2019-07-12T14:55:18.0820122Z Preparing to unpack .../pkg-config_0.29.1-0ubuntu1_amd64.deb ...
2019-07-12T14:55:18.0820122Z Preparing to unpack .../pkg-config_0.29.1-0ubuntu1_amd64.deb ...
2019-07-12T14:55:18.0956052Z Unpacking pkg-config (0.29.1-0ubuntu1) ...
2019-07-12T14:55:18.1987609Z Selecting previously unselected package python2.7.
2019-07-12T14:55:18.2008432Z Preparing to unpack .../python2.7_2.7.12-1ubuntu0~16.04.4_amd64.deb ...
2019-07-12T14:55:18.2136333Z Unpacking python2.7 (2.7.12-1ubuntu0~16.04.4) ...
2019-07-12T14:55:18.3086235Z Selecting previously unselected package binutils-mingw-w64-i686.
2019-07-12T14:55:18.3104148Z Preparing to unpack .../binutils-mingw-w64-i686_2.26-3ubuntu1+6.6_amd64.deb ...
2019-07-12T14:55:18.3235594Z Unpacking binutils-mingw-w64-i686 (2.26-3ubuntu1+6.6) ...
2019-07-12T14:55:18.9642650Z Selecting previously unselected package binutils-mingw-w64-x86-64.
2019-07-12T14:55:18.9665355Z Preparing to unpack .../binutils-mingw-w64-x86-64_2.26-3ubuntu1+6.6_amd64.deb ...
2019-07-12T14:55:18.9795855Z Unpacking binutils-mingw-w64-x86-64 (2.26-3ubuntu1+6.6) ...
2019-07-12T14:55:19.5542541Z Selecting previously unselected package mingw-w64-common.
2019-07-12T14:55:19.5561832Z Preparing to unpack .../mingw-w64-common_4.0.4-2_all.deb ...
2019-07-12T14:55:19.5701678Z Unpacking mingw-w64-common (4.0.4-2) ...
2019-07-12T14:55:21.1177540Z Selecting previously unselected package mingw-w64-i686-dev.
2019-07-12T14:55:21.1202035Z Preparing to unpack .../mingw-w64-i686-dev_4.0.4-2_all.deb ...
2019-07-12T14:55:21.1343502Z Unpacking mingw-w64-i686-dev (4.0.4-2) ...
2019-07-12T14:55:22.4130071Z Selecting previously unselected package gcc-mingw-w64-base.
2019-07-12T14:55:22.4154592Z Preparing to unpack .../gcc-mingw-w64-base_5.3.1-8ubuntu3+17_amd64.deb ...
2019-07-12T14:55:22.4289220Z Unpacking gcc-mingw-w64-base (5.3.1-8ubuntu3+17) ...
2019-07-12T14:55:22.5245380Z Selecting previously unselected package gcc-mingw-w64-i686.
2019-07-12T14:55:22.5266520Z Preparing to unpack .../gcc-mingw-w64-i686_5.3.1-8ubuntu3+17_amd64.deb ...
2019-07-12T14:55:22.5399975Z Unpacking gcc-mingw-w64-i686 (5.3.1-8ubuntu3+17) ...
2019-07-12T14:55:26.5625045Z Selecting previously unselected package g++-mingw-w64-i686.
2019-07-12T14:55:26.5646335Z Preparing to unpack .../g++-mingw-w64-i686_5.3.1-8ubuntu3+17_amd64.deb ...
2019-07-12T14:55:26.5795085Z Unpacking g++-mingw-w64-i686 (5.3.1-8ubuntu3+17) ...
2019-07-12T14:55:30.3427136Z Selecting previously unselected package mingw-w64-x86-64-dev.
2019-07-12T14:55:30.3455652Z Preparing to unpack .../mingw-w64-x86-64-dev_4.0.4-2_all.deb ...
2019-07-12T14:55:30.3594737Z Unpacking mingw-w64-x86-64-dev (4.0.4-2) ...
2019-07-12T14:55:32.5325517Z Selecting previously unselected package gcc-mingw-w64-x86-64.
2019-07-12T14:55:32.5358118Z Preparing to unpack .../gcc-mingw-w64-x86-64_5.3.1-8ubuntu3+17_amd64.deb ...
2019-07-12T14:55:32.5489187Z Unpacking gcc-mingw-w64-x86-64 (5.3.1-8ubuntu3+17) ...
2019-07-12T14:55:36.8795268Z Selecting previously unselected package g++-mingw-w64-x86-64.
2019-07-12T14:55:36.8819492Z Preparing to unpack .../g++-mingw-w64-x86-64_5.3.1-8ubuntu3+17_amd64.deb ...
2019-07-12T14:55:36.8991469Z Unpacking g++-mingw-w64-x86-64 (5.3.1-8ubuntu3+17) ...
2019-07-12T14:55:40.7661655Z Selecting previously unselected package g++-mingw-w64.
2019-07-12T14:55:40.7694872Z Preparing to unpack .../g++-mingw-w64_5.3.1-8ubuntu3+17_all.deb ...
2019-07-12T14:55:40.7852755Z Unpacking g++-mingw-w64 (5.3.1-8ubuntu3+17) ...
2019-07-12T14:55:40.9674372Z Selecting previously unselected package gcc-mingw-w64.
2019-07-12T14:55:40.9699636Z Preparing to unpack .../gcc-mingw-w64_5.3.1-8ubuntu3+17_all.deb ...
2019-07-12T14:55:40.9903885Z Unpacking gcc-mingw-w64 (5.3.1-8ubuntu3+17) ...
2019-07-12T14:55:41.0907486Z Selecting previously unselected package mingw-w64.
2019-07-12T14:55:41.0918128Z Preparing to unpack .../mingw-w64_4.0.4-2_all.deb ...
2019-07-12T14:55:41.1050456Z Unpacking mingw-w64 (4.0.4-2) ...
2019-07-12T14:55:41.3897276Z Setting up libgdbm3:amd64 (1.8.3-13.1) ...
2019-07-12T14:55:41.4705833Z Setting up libffi6:amd64 (3.2.1-4) ...
2019-07-12T14:55:41.5161175Z Setting up libglib2.0-0:amd64 (2.48.2-0ubuntu4.3) ...
2019-07-12T14:55:41.5807449Z No schema files found: doing nothing.
---
2019-07-12T14:55:47.7351026Z Setting up zlib1g-dev:amd64 (1:1.2.8.dfsg-2ubuntu4.1) ...
2019-07-12T14:55:47.7857236Z Setting up libssl-dev:amd64 (1.0.2g-1ubuntu4.15) ...
2019-07-12T14:55:47.8310964Z Setting up pkg-config (0.29.1-0ubuntu1) ...
2019-07-12T14:55:47.9164582Z Setting up python2.7 (2.7.12-1ubuntu0~16.04.4) ...
2019-07-12T14:55:48.6856447Z Setting up binutils-mingw-w64-i686 (2.26-3ubuntu1+6.6) ...
2019-07-12T14:55:48.7264483Z Setting up binutils-mingw-w64-x86-64 (2.26-3ubuntu1+6.6) ...
2019-07-12T14:55:48.7672784Z Setting up mingw-w64-common (4.0.4-2) ...
2019-07-12T14:55:48.8602380Z Setting up mingw-w64-i686-dev (4.0.4-2) ...
2019-07-12T14:55:48.9233402Z Setting up gcc-mingw-w64-base (5.3.1-8ubuntu3+17) ...
2019-07-12T14:55:48.9642761Z Setting up gcc-mingw-w64-i686 (5.3.1-8ubuntu3+17) ...
2019-07-12T14:55:48.9946524Z update-alternatives: using /usr/bin/i686-w64-mingw32-gcc-posix to provide /usr/bin/i686-w64-mingw32-gcc (i686-w64-mingw32-gcc) in auto mode
2019-07-12T14:55:48.9953230Z update-alternatives: warning: skip creation of /usr/bin/i686-w64-mingw32-gcc-5 because associated file /usr/bin/i686-w64-mingw32-gcc-5-posix (of link group i686-w64-mingw32-gcc) doesn't exist
2019-07-12T14:55:49.0041713Z update-alternatives: using /usr/bin/i686-w64-mingw32-gcc-win32 to provide /usr/bin/i686-w64-mingw32-gcc (i686-w64-mingw32-gcc) in auto mode
2019-07-12T14:55:49.0046299Z update-alternatives: warning: skip creation of /usr/bin/i686-w64-mingw32-gcc-5 because associated file /usr/bin/i686-w64-mingw32-gcc-5-win32 (of link group i686-w64-mingw32-gcc) doesn't exist
2019-07-12T14:55:49.0333188Z Setting up g++-mingw-w64-i686 (5.3.1-8ubuntu3+17) ...
2019-07-12T14:55:49.0661958Z update-alternatives: using /usr/bin/i686-w64-mingw32-g++-posix to provide /usr/bin/i686-w64-mingw32-g++ (i686-w64-mingw32-g++) in auto mode
2019-07-12T14:55:49.0757466Z update-alternatives: using /usr/bin/i686-w64-mingw32-g++-win32 to provide /usr/bin/i686-w64-mingw32-g++ (i686-w64-mingw32-g++) in auto mode
2019-07-12T14:55:49.1002150Z Setting up mingw-w64-x86-64-dev (4.0.4-2) ...
2019-07-12T14:55:49.1412908Z Setting up gcc-mingw-w64-x86-64 (5.3.1-8ubuntu3+17) ...
2019-07-12T14:55:49.1701717Z update-alternatives: using /usr/bin/x86_64-w64-mingw32-gcc-posix to provide /usr/bin/x86_64-w64-mingw32-gcc (x86_64-w64-mingw32-gcc) in auto mode
2019-07-12T14:55:49.1707663Z update-alternatives: warning: skip creation of /usr/bin/x86_64-w64-mingw32-gcc-5 because associated file /usr/bin/x86_64-w64-mingw32-gcc-5-posix (of link group x86_64-w64-mingw32-gcc) doesn't exist
2019-07-12T14:55:49.1800721Z update-alternatives: using /usr/bin/x86_64-w64-mingw32-gcc-win32 to provide /usr/bin/x86_64-w64-mingw32-gcc (x86_64-w64-mingw32-gcc) in auto mode
2019-07-12T14:55:49.1803863Z update-alternatives: warning: skip creation of /usr/bin/x86_64-w64-mingw32-gcc-5 because associated file /usr/bin/x86_64-w64-mingw32-gcc-5-win32 (of link group x86_64-w64-mingw32-gcc) doesn't exist
2019-07-12T14:55:49.2079863Z Setting up g++-mingw-w64-x86-64 (5.3.1-8ubuntu3+17) ...
2019-07-12T14:55:49.2369341Z update-alternatives: using /usr/bin/x86_64-w64-mingw32-g++-posix to provide /usr/bin/x86_64-w64-mingw32-g++ (x86_64-w64-mingw32-g++) in auto mode
2019-07-12T14:55:49.2464202Z update-alternatives: using /usr/bin/x86_64-w64-mingw32-g++-win32 to provide /usr/bin/x86_64-w64-mingw32-g++ (x86_64-w64-mingw32-g++) in auto mode
2019-07-12T14:55:49.2726470Z Setting up g++-mingw-w64 (5.3.1-8ubuntu3+17) ...
2019-07-12T14:55:49.3199908Z Setting up gcc-mingw-w64 (5.3.1-8ubuntu3+17) ...
2019-07-12T14:55:49.3624656Z Setting up mingw-w64 (4.0.4-2) ...
2019-07-12T14:55:49.4523542Z Processing triggers for ca-certificates (20170717~16.04.2) ...
2019-07-12T14:55:49.4687778Z Updating certificates in /etc/ssl/certs...
2019-07-12T14:55:50.9445122Z 148 added, 0 removed; done.
2019-07-12T14:55:50.9446409Z Running hooks in /etc/ca-certificates/update.d...
---
2019-07-12T14:56:33.4881152Z  ---> 625824b41dc6
2019-07-12T14:56:33.4920648Z Successfully built 625824b41dc6
2019-07-12T14:56:33.6936388Z Successfully tagged rust-ci:latest
2019-07-12T14:56:33.7825354Z Built container sha256:625824b41dc683eaccd0ac33bf1729ec6d6026731dedec2e668cd0c472f7ba7a
2019-07-12T14:56:33.7839727Z Uploading finished image to https://.s3.amazonaws.com/docker/a4940e6914a5e1f6360ebc241d0d850aff083be5c43d268fd716ce48f63cb24e84238becb662f27f8c5d7740413ca17da24bb9cdc003ef32d7b03d2c9052b94d
2019-07-12T14:57:58.6745194Z upload failed: - to s3:///docker/a4940e6914a5e1f6360ebc241d0d850aff083be5c43d268fd716ce48f63cb24e84238becb662f27f8c5d7740413ca17da24bb9cdc003ef32d7b03d2c9052b94d Parameter validation failed:
2019-07-12T14:57:58.6747254Z Invalid bucket name "": Bucket name must match the regex "^[a-zA-Z0-9.\-_]{1,255}$"
2019-07-12T14:58:00.3990249Z [CI_JOB_NAME=mingw-check]
2019-07-12T14:58:00.4040240Z Starting sccache server...
2019-07-12T14:58:00.4517605Z configure: processing command line
2019-07-12T14:58:00.4518315Z configure: 
---
2019-07-12T15:00:01.8169076Z     Checking core v0.0.0 (/checkout/src/libcore)
2019-07-12T15:00:05.2773277Z error[E0547]: missing 'issue'
2019-07-12T15:00:05.2773610Z    --> src/libcore/option.rs:924:5
2019-07-12T15:00:05.2773812Z     |
2019-07-12T15:00:05.2774096Z 924 |     #[unstable(feature = "option_err_or")]
2019-07-12T15:00:05.2774403Z 
2019-07-12T15:00:05.2774614Z error[E0547]: missing 'issue'
2019-07-12T15:00:05.2774816Z    --> src/libcore/option.rs:951:5
2019-07-12T15:00:05.2774987Z     |
2019-07-12T15:00:05.2774987Z     |
2019-07-12T15:00:05.2775235Z 951 |     #[unstable(feature = "option_err_or")]
2019-07-12T15:00:05.2775497Z 
2019-07-12T15:00:10.1001843Z    Compiling libc v0.2.54
2019-07-12T15:00:10.8656941Z    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
2019-07-12T15:00:12.4049524Z    Compiling autocfg v0.1.4
2019-07-12T15:00:12.4049524Z    Compiling autocfg v0.1.4
2019-07-12T15:00:14.0164202Z    Compiling unwind v0.0.0 (/checkout/src/libunwind)
2019-07-12T15:00:14.5116294Z    Compiling compiler_builtins v0.1.16
2019-07-12T15:00:15.5277813Z error: aborting due to 2 previous errors
2019-07-12T15:00:15.5282337Z 
2019-07-12T15:00:15.6314000Z error: Could not compile `core`.
2019-07-12T15:00:15.6314318Z warning: build failed, waiting for other jobs to finish...
2019-07-12T15:00:16.3837612Z error: build failed
2019-07-12T15:00:16.3861138Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
2019-07-12T15:00:16.3875372Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-07-12T15:00:16.3875434Z Build completed unsuccessfully in 0:02:15
2019-07-12T15:00:16.3875434Z Build completed unsuccessfully in 0:02:15
2019-07-12T15:00:25.4133056Z ##[error]Bash exited with code '1'.
2019-07-12T15:00:25.4166155Z ##[section]Starting: Checkout
2019-07-12T15:00:25.4167748Z ==============================================================================
2019-07-12T15:00:25.4167805Z Task         : Get sources
2019-07-12T15:00:25.4167854Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
