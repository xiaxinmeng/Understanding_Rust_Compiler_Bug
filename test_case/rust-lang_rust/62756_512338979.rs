plain
2019-07-17T15:38:49.2011872Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-17T15:38:49.2231250Z ##[command]git config gc.auto 0
2019-07-17T15:38:49.2319496Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-17T15:38:49.2374389Z ##[command]git config --get-all http.proxy
2019-07-17T15:38:49.2542060Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62756/merge:refs/remotes/pull/62756/merge
---
2019-07-17T15:39:25.8530909Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-17T15:39:25.8533111Z 
2019-07-17T15:39:25.8534910Z   git checkout -b <new-branch-name>
2019-07-17T15:39:25.8535273Z 
2019-07-17T15:39:25.8535493Z HEAD is now at dc849d73f Merge 0a227f33a80f02ce0182d52ff1224ace87ad2101 into d56128d2919132aceaf74cc3c68a4554f5445fce
2019-07-17T15:39:25.8687114Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-17T15:39:25.8691067Z ==============================================================================
2019-07-17T15:39:25.8691139Z Task         : Bash
2019-07-17T15:39:25.8691193Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-17T15:41:19.2459946Z Attempting with retry: curl -y 30 -Y 10 --connect-timeout 30 -f -L -C - -o /tmp/rustci_docker_cache https://.s3.amazonaws.com/docker/a4940e6914a5e1f6360ebc241d0d850aff083be5c43d268fd716ce48f63cb24e84238becb662f27f8c5d7740413ca17da24bb9cdc003ef32d7b03d2c9052b94d
2019-07-17T15:41:19.2529820Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-17T15:41:19.2530182Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-17T15:41:19.2530332Z 
2019-07-17T15:41:19.2534777Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-17T15:41:20.2612371Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-17T15:41:20.2612731Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-17T15:41:20.2612843Z 
2019-07-17T15:41:20.2612843Z 
2019-07-17T15:41:20.2661181Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-17T15:41:22.2736447Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-17T15:41:22.2737098Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-17T15:41:22.2737189Z 
2019-07-17T15:41:22.2737189Z 
2019-07-17T15:41:22.2776350Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-17T15:41:25.2876782Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-17T15:41:25.2877132Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-17T15:41:25.2877277Z 
2019-07-17T15:41:25.2877277Z 
2019-07-17T15:41:25.2878773Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-17T15:41:29.2955962Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-17T15:41:29.2956343Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-17T15:41:29.2956498Z 
2019-07-17T15:41:29.2956498Z 
2019-07-17T15:41:29.2998632Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-17T15:41:29.3005661Z The command has failed after 5 attempts.
2019-07-17T15:41:29.4232292Z open /tmp/rustci_docker_cache: no such file or directory
2019-07-17T15:41:29.4259325Z Attempting with retry: docker build --rm -t rust-ci -f /home/vsts/work/1/s/src/ci/docker/mingw-check/Dockerfile /home/vsts/work/1/s/src/ci/docker
2019-07-17T15:41:29.5903956Z Sending build context to Docker daemon  521.2kB
2019-07-17T15:41:29.5904615Z 
2019-07-17T15:41:29.6120688Z Step 1/6 : FROM ubuntu:16.04
---
2019-07-17T15:41:46.2888963Z Reading package lists...
2019-07-17T15:41:47.3634123Z Reading package lists...
2019-07-17T15:41:47.5484866Z Building dependency tree...
2019-07-17T15:41:47.5484966Z Reading state information...
2019-07-17T15:41:47.6652758Z The following additional packages will be installed:
2019-07-17T15:41:47.6653932Z   binutils binutils-mingw-w64-i686 binutils-mingw-w64-x86-64 bzip2 cmake-data
2019-07-17T15:41:47.6654225Z   cpp cpp-5 dpkg-dev g++-5 g++-mingw-w64 g++-mingw-w64-i686
2019-07-17T15:41:47.6654503Z   g++-mingw-w64-x86-64 gcc gcc-5 gcc-mingw-w64 gcc-mingw-w64-base
2019-07-17T15:41:47.6654805Z   gcc-mingw-w64-i686 gcc-mingw-w64-x86-64 git-man libarchive13 libasan2
2019-07-17T15:41:47.6655086Z   libasn1-8-heimdal libatomic1 libbabeltrace-ctf1 libbabeltrace1 libbz2-1.0
2019-07-17T15:41:47.6656157Z   libdpkg-perl liberror-perl libexpat1 libffi6 libgcc-5-dev libgdbm3
2019-07-17T15:41:47.6656457Z   libglib2.0-0 libgmp10 libgnutls30 libgomp1 libgssapi-krb5-2
2019-07-17T15:41:47.6656725Z   libgssapi3-heimdal libhcrypto4-heimdal libheimbase1-heimdal
2019-07-17T15:41:47.6657025Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-17T15:41:47.6657025Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-17T15:41:47.6657319Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-17T15:41:47.6657605Z   libkrb5-3 libkrb5support0 libldap-2.4-2 liblsan0 liblzo2-2 libmagic1 libmpc3
2019-07-17T15:41:47.6657892Z   libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0 libperl5.22
2019-07-17T15:41:47.6658174Z   libpython2.7-minimal libpython2.7-stdlib libpython3.5 libpython3.5-minimal
2019-07-17T15:41:47.6658455Z   libpython3.5-stdlib libquadmath0 libroken18-heimdal librtmp1 libsasl2-2
2019-07-17T15:41:47.6658751Z   libsasl2-modules-db libsqlite3-0 libssl1.0.0 libstdc++-5-dev libtasn1-6
2019-07-17T15:41:47.6659029Z   libtsan0 libubsan0 libwind0-heimdal libxml2 linux-libc-dev mime-support
2019-07-17T15:41:47.6659324Z   mingw-w64-common mingw-w64-i686-dev mingw-w64-x86-64-dev openssl patch perl
2019-07-17T15:41:47.6659648Z   perl-modules-5.22 python2.7-minimal zlib1g-dev
2019-07-17T15:41:47.6659756Z Suggested packages:
2019-07-17T15:41:47.6660074Z   binutils-doc bzip2-doc codeblocks eclipse ninja-build cpp-doc gcc-5-locales
2019-07-17T15:41:47.6660376Z   debian-keyring g++-multilib g++-5-multilib gcc-5-doc libstdc++6-5-dbg
2019-07-17T15:41:47.6660650Z   gcc-multilib manpages-dev autoconf automake libtool flex bison gcc-doc
2019-07-17T15:41:47.6661206Z   libasan2-dbg liblsan0-dbg libtsan0-dbg libubsan0-dbg libcilkrts5-dbg
2019-07-17T15:41:47.6661475Z   libmpx0-dbg libquadmath0-dbg gdb-doc gettext-base git-daemon-run
2019-07-17T15:41:47.6661475Z   libmpx0-dbg libquadmath0-dbg gdb-doc gettext-base git-daemon-run
2019-07-17T15:41:47.6661751Z   | git-daemon-sysvinit git-doc git-el git-email git-gui gitk gitweb git-arch
2019-07-17T15:41:47.6662046Z   git-cvs git-mediawiki git-svn lrzip glibc-doc gnutls-bin krb5-doc krb5-user
2019-07-17T15:41:47.6662323Z   libstdc++-5-doc make-doc wine wine64 ed diffutils-doc perl-doc
2019-07-17T15:41:47.6662596Z   libterm-readline-gnu-perl | libterm-readline-perl-perl python2.7-doc
2019-07-17T15:41:47.6662827Z   binfmt-support
2019-07-17T15:41:47.6662879Z Recommended packages:
2019-07-17T15:41:47.6663157Z   build-essential fakeroot libalgorithm-merge-perl gfortran-mingw-w64
2019-07-17T15:41:47.6663440Z   gnat-mingw-w64 libc-dbg gdbserver less rsync ssh-client manpages
2019-07-17T15:41:47.6663712Z   manpages-dev libfile-fcntllock-perl libglib2.0-data shared-mime-info
2019-07-17T15:41:47.6663986Z   xdg-user-dirs krb5-locales libsasl2-modules libssl-doc xml-core netbase
2019-07-17T15:41:47.6664055Z   rename
2019-07-17T15:41:47.6664107Z The following NEW packages will be installed:
2019-07-17T15:41:47.6664381Z   binutils binutils-mingw-w64-i686 binutils-mingw-w64-x86-64 bzip2
2019-07-17T15:41:47.6664654Z   ca-certificates cmake cmake-data cpp cpp-5 curl dpkg-dev file g++ g++-5
2019-07-17T15:41:47.6665106Z   g++-mingw-w64 g++-mingw-w64-i686 g++-mingw-w64-x86-64 gcc gcc-5
2019-07-17T15:41:47.6665449Z   gcc-mingw-w64 gcc-mingw-w64-base gcc-mingw-w64-i686 gcc-mingw-w64-x86-64 gdb
2019-07-17T15:41:47.6666073Z   git git-man libarchive13 libasan2 libasn1-8-heimdal libatomic1
2019-07-17T15:41:47.6666589Z   libbabeltrace-ctf1 libbabeltrace1 libc-dev-bin libc6-dev libcc1-0
2019-07-17T15:41:47.6667214Z   libffi6 libgcc-5-dev libgdbm3 libglib2.0-0 libgmp10 libgnutls30 libgomp1
2019-07-17T15:41:47.6667555Z   libgssapi-krb5-2 libgssapi3-heimdal libhcrypto4-heimdal libheimbase1-heimdal
2019-07-17T15:41:47.6667860Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-17T15:41:47.6668165Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-17T15:41:47.6668165Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-17T15:41:47.6668493Z   libkrb5-3 libkrb5support0 libldap-2.4-2 liblsan0 liblzo2-2 libmagic1 libmpc3
2019-07-17T15:41:47.6668798Z   libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0 libperl5.22
2019-07-17T15:41:47.6669174Z   libpython2.7-minimal libpython2.7-stdlib libpython3.5 libpython3.5-minimal
2019-07-17T15:41:47.6669915Z   libpython3.5-stdlib libquadmath0 libroken18-heimdal librtmp1 libsasl2-2
2019-07-17T15:41:47.6670244Z   libsasl2-modules-db libsqlite3-0 libssl-dev libssl1.0.0 libstdc++-5-dev
2019-07-17T15:41:47.6670546Z   libtasn1-6 libtsan0 libubsan0 libwind0-heimdal libxml2 linux-libc-dev make
2019-07-17T15:41:47.6670862Z   mime-support mingw-w64 mingw-w64-common mingw-w64-i686-dev
2019-07-17T15:41:47.6671156Z   mingw-w64-x86-64-dev openssl patch perl perl-modules-5.22 pkg-config
2019-07-17T15:41:47.6671441Z   python2.7 python2.7-minimal sudo xz-utils zlib1g-dev
2019-07-17T15:41:47.6671522Z The following packages will be upgraded:
2019-07-17T15:41:48.1272931Z 1 upgraded, 112 newly installed, 0 to remove and 5 not upgraded.
2019-07-17T15:41:48.1273104Z Need to get 187 MB of archives.
2019-07-17T15:41:48.1273197Z After this operation, 968 MB of additional disk space will be used.
2019-07-17T15:41:48.1273996Z Get:1 http://archive.ubuntu.com/ubuntu xenial/main amd64 libgdbm3 amd64 1.8.3-13.1 [16.9 kB]
---
2019-07-17T15:41:52.4654099Z Get:97 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 zlib1g-dev amd64 1:1.2.8.dfsg-2ubuntu4.1 [168 kB]
2019-07-17T15:41:52.4704158Z Get:98 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 libssl-dev amd64 1.0.2g-1ubuntu4.15 [1344 kB]
2019-07-17T15:41:52.4938089Z Get:99 http://archive.ubuntu.com/ubuntu xenial/main amd64 pkg-config amd64 0.29.1-0ubuntu1 [45.0 kB]
2019-07-17T15:41:52.4938736Z Get:100 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 python2.7 amd64 2.7.12-1ubuntu0~16.04.4 [224 kB]
2019-07-17T15:41:52.4982237Z Get:101 http://archive.ubuntu.com/ubuntu xenial/universe amd64 binutils-mingw-w64-i686 amd64 2.26-3ubuntu1+6.6 [1782 kB]
2019-07-17T15:41:52.8112246Z Get:102 http://archive.ubuntu.com/ubuntu xenial/universe amd64 binutils-mingw-w64-x86-64 amd64 2.26-3ubuntu1+6.6 [2029 kB]
2019-07-17T15:41:53.7288337Z Get:103 http://archive.ubuntu.com/ubuntu xenial/universe amd64 mingw-w64-common all 4.0.4-2 [4787 kB]
2019-07-17T15:41:53.9354267Z Get:104 http://archive.ubuntu.com/ubuntu xenial/universe amd64 mingw-w64-i686-dev all 4.0.4-2 [2059 kB]
2019-07-17T15:41:54.0117559Z Get:105 http://archive.ubuntu.com/ubuntu xenial/universe amd64 gcc-mingw-w64-base amd64 5.3.1-8ubuntu3+17 [11.2 kB]
2019-07-17T15:41:54.0123179Z Get:106 http://archive.ubuntu.com/ubuntu xenial/universe amd64 gcc-mingw-w64-i686 amd64 5.3.1-8ubuntu3+17 [27.3 MB]
2019-07-17T15:41:55.0656006Z Get:107 http://archive.ubuntu.com/ubuntu xenial/universe amd64 g++-mingw-w64-i686 amd64 5.3.1-8ubuntu3+17 [19.8 MB]
2019-07-17T15:41:55.9112309Z Get:108 http://archive.ubuntu.com/ubuntu xenial/universe amd64 mingw-w64-x86-64-dev all 4.0.4-2 [3238 kB]
2019-07-17T15:41:55.9633935Z Get:109 http://archive.ubuntu.com/ubuntu xenial/universe amd64 gcc-mingw-w64-x86-64 amd64 5.3.1-8ubuntu3+17 [27.4 MB]
2019-07-17T15:41:57.2005411Z Get:110 http://archive.ubuntu.com/ubuntu xenial/universe amd64 g++-mingw-w64-x86-64 amd64 5.3.1-8ubuntu3+17 [20.4 MB]
2019-07-17T15:41:57.9615719Z Get:111 http://archive.ubuntu.com/ubuntu xenial/universe amd64 g++-mingw-w64 all 5.3.1-8ubuntu3+17 [10.7 kB]
2019-07-17T15:41:57.9620712Z Get:112 http://archive.ubuntu.com/ubuntu xenial/universe amd64 gcc-mingw-w64 all 5.3.1-8ubuntu3+17 [10.7 kB]
2019-07-17T15:41:57.9625072Z Get:113 http://archive.ubuntu.com/ubuntu xenial/universe amd64 mingw-w64 all 4.0.4-2 [9274 B]
2019-07-17T15:42:00.2419164Z debconf: delaying package configuration, since apt-utils is not installed
2019-07-17T15:42:00.2638069Z Fetched 187 MB in 10s (18.6 MB/s)
2019-07-17T15:42:00.3316851Z (Reading database ... 
2019-07-17T15:42:00.3316979Z (Reading database ... 5%
2019-07-17T15:42:00.3317058Z (Reading database ... 10%
2019-07-17T15:42:00.3317108Z (Reading database ... 15%
---
2019-07-17T15:42:26.4195076Z Unpacking pkg-config (0.29.1-0ubuntu1) ...
2019-07-17T15:42:26.5191393Z Selecting previously unselected package python2.7.
2019-07-17T15:42:26.5213196Z Preparing to unpack .../python2.7_2.7.12-1ubuntu0~16.04.4_amd64.deb ...
2019-07-17T15:42:26.5338517Z Unpacking python2.7 (2.7.12-1ubuntu0~16.04.4) ...
2019-07-17T15:42:26.6296105Z Selecting previously unselected package binutils-mingw-w64-i686.
2019-07-17T15:42:26.6318557Z Preparing to unpack .../binutils-mingw-w64-i686_2.26-3ubuntu1+6.6_amd64.deb ...
2019-07-17T15:42:26.6444545Z Unpacking binutils-mingw-w64-i686 (2.26-3ubuntu1+6.6) ...
2019-07-17T15:42:27.2042526Z Selecting previously unselected package binutils-mingw-w64-x86-64.
2019-07-17T15:42:27.2067397Z Preparing to unpack .../binutils-mingw-w64-x86-64_2.26-3ubuntu1+6.6_amd64.deb ...
2019-07-17T15:42:27.2185668Z Unpacking binutils-mingw-w64-x86-64 (2.26-3ubuntu1+6.6) ...
2019-07-17T15:42:27.7864937Z Selecting previously unselected package mingw-w64-common.
2019-07-17T15:42:27.7886736Z Preparing to unpack .../mingw-w64-common_4.0.4-2_all.deb ...
2019-07-17T15:42:27.8007823Z Unpacking mingw-w64-common (4.0.4-2) ...
2019-07-17T15:42:29.2971661Z Selecting previously unselected package mingw-w64-i686-dev.
2019-07-17T15:42:29.2995412Z Preparing to unpack .../mingw-w64-i686-dev_4.0.4-2_all.deb ...
2019-07-17T15:42:29.3115948Z Unpacking mingw-w64-i686-dev (4.0.4-2) ...
2019-07-17T15:42:30.4425577Z Selecting previously unselected package gcc-mingw-w64-base.
2019-07-17T15:42:30.4449313Z Preparing to unpack .../gcc-mingw-w64-base_5.3.1-8ubuntu3+17_amd64.deb ...
2019-07-17T15:42:30.4569948Z Unpacking gcc-mingw-w64-base (5.3.1-8ubuntu3+17) ...
2019-07-17T15:42:30.5479390Z Selecting previously unselected package gcc-mingw-w64-i686.
2019-07-17T15:42:30.5504650Z Preparing to unpack .../gcc-mingw-w64-i686_5.3.1-8ubuntu3+17_amd64.deb ...
2019-07-17T15:42:30.5653422Z Unpacking gcc-mingw-w64-i686 (5.3.1-8ubuntu3+17) ...
2019-07-17T15:42:34.8897929Z Selecting previously unselected package g++-mingw-w64-i686.
2019-07-17T15:42:34.8924465Z Preparing to unpack .../g++-mingw-w64-i686_5.3.1-8ubuntu3+17_amd64.deb ...
2019-07-17T15:42:34.9068867Z Unpacking g++-mingw-w64-i686 (5.3.1-8ubuntu3+17) ...
2019-07-17T15:42:38.9301467Z Selecting previously unselected package mingw-w64-x86-64-dev.
2019-07-17T15:42:38.9335276Z Preparing to unpack .../mingw-w64-x86-64-dev_4.0.4-2_all.deb ...
2019-07-17T15:42:38.9454130Z Unpacking mingw-w64-x86-64-dev (4.0.4-2) ...
2019-07-17T15:42:41.0396908Z Selecting previously unselected package gcc-mingw-w64-x86-64.
2019-07-17T15:42:41.0429904Z Preparing to unpack .../gcc-mingw-w64-x86-64_5.3.1-8ubuntu3+17_amd64.deb ...
2019-07-17T15:42:41.0572832Z Unpacking gcc-mingw-w64-x86-64 (5.3.1-8ubuntu3+17) ...
2019-07-17T15:42:45.8532970Z Selecting previously unselected package g++-mingw-w64-x86-64.
2019-07-17T15:42:45.8559171Z Preparing to unpack .../g++-mingw-w64-x86-64_5.3.1-8ubuntu3+17_amd64.deb ...
2019-07-17T15:42:45.8687655Z Unpacking g++-mingw-w64-x86-64 (5.3.1-8ubuntu3+17) ...
2019-07-17T15:42:50.1551960Z Selecting previously unselected package g++-mingw-w64.
2019-07-17T15:42:50.1583321Z Preparing to unpack .../g++-mingw-w64_5.3.1-8ubuntu3+17_all.deb ...
2019-07-17T15:42:50.1739501Z Unpacking g++-mingw-w64 (5.3.1-8ubuntu3+17) ...
2019-07-17T15:42:50.2767708Z Selecting previously unselected package gcc-mingw-w64.
2019-07-17T15:42:50.2794424Z Preparing to unpack .../gcc-mingw-w64_5.3.1-8ubuntu3+17_all.deb ...
2019-07-17T15:42:50.2959849Z Unpacking gcc-mingw-w64 (5.3.1-8ubuntu3+17) ...
2019-07-17T15:42:50.3769817Z Selecting previously unselected package mingw-w64.
2019-07-17T15:42:50.3800258Z Preparing to unpack .../mingw-w64_4.0.4-2_all.deb ...
2019-07-17T15:42:50.4008980Z Unpacking mingw-w64 (4.0.4-2) ...
2019-07-17T15:42:50.6565572Z Setting up libgdbm3:amd64 (1.8.3-13.1) ...
2019-07-17T15:42:50.7243197Z Setting up libffi6:amd64 (3.2.1-4) ...
2019-07-17T15:42:50.7616033Z Setting up libglib2.0-0:amd64 (2.48.2-0ubuntu4.3) ...
2019-07-17T15:42:50.7893460Z No schema files found: doing nothing.
---
2019-07-17T15:42:56.1294750Z Setting up zlib1g-dev:amd64 (1:1.2.8.dfsg-2ubuntu4.1) ...
2019-07-17T15:42:56.1680818Z Setting up libssl-dev:amd64 (1.0.2g-1ubuntu4.15) ...
2019-07-17T15:42:56.2076133Z Setting up pkg-config (0.29.1-0ubuntu1) ...
2019-07-17T15:42:56.2899051Z Setting up python2.7 (2.7.12-1ubuntu0~16.04.4) ...
2019-07-17T15:42:57.1667616Z Setting up binutils-mingw-w64-i686 (2.26-3ubuntu1+6.6) ...
2019-07-17T15:42:57.2081316Z Setting up binutils-mingw-w64-x86-64 (2.26-3ubuntu1+6.6) ...
2019-07-17T15:42:57.2476470Z Setting up mingw-w64-common (4.0.4-2) ...
2019-07-17T15:42:57.2888508Z Setting up mingw-w64-i686-dev (4.0.4-2) ...
2019-07-17T15:42:57.3306634Z Setting up gcc-mingw-w64-base (5.3.1-8ubuntu3+17) ...
2019-07-17T15:42:57.3691830Z Setting up gcc-mingw-w64-i686 (5.3.1-8ubuntu3+17) ...
2019-07-17T15:42:57.3965554Z update-alternatives: using /usr/bin/i686-w64-mingw32-gcc-posix to provide /usr/bin/i686-w64-mingw32-gcc (i686-w64-mingw32-gcc) in auto mode
2019-07-17T15:42:57.3966266Z update-alternatives: warning: skip creation of /usr/bin/i686-w64-mingw32-gcc-5 because associated file /usr/bin/i686-w64-mingw32-gcc-5-posix (of link group i686-w64-mingw32-gcc) doesn't exist
2019-07-17T15:42:57.4055899Z update-alternatives: using /usr/bin/i686-w64-mingw32-gcc-win32 to provide /usr/bin/i686-w64-mingw32-gcc (i686-w64-mingw32-gcc) in auto mode
2019-07-17T15:42:57.4056944Z update-alternatives: warning: skip creation of /usr/bin/i686-w64-mingw32-gcc-5 because associated file /usr/bin/i686-w64-mingw32-gcc-5-win32 (of link group i686-w64-mingw32-gcc) doesn't exist
2019-07-17T15:42:57.4327203Z Setting up g++-mingw-w64-i686 (5.3.1-8ubuntu3+17) ...
2019-07-17T15:42:57.4678977Z update-alternatives: using /usr/bin/i686-w64-mingw32-g++-posix to provide /usr/bin/i686-w64-mingw32-g++ (i686-w64-mingw32-g++) in auto mode
2019-07-17T15:42:57.4797184Z update-alternatives: using /usr/bin/i686-w64-mingw32-g++-win32 to provide /usr/bin/i686-w64-mingw32-g++ (i686-w64-mingw32-g++) in auto mode
2019-07-17T15:42:57.5065035Z Setting up mingw-w64-x86-64-dev (4.0.4-2) ...
2019-07-17T15:42:57.5437858Z Setting up gcc-mingw-w64-x86-64 (5.3.1-8ubuntu3+17) ...
2019-07-17T15:42:57.5713762Z update-alternatives: using /usr/bin/x86_64-w64-mingw32-gcc-posix to provide /usr/bin/x86_64-w64-mingw32-gcc (x86_64-w64-mingw32-gcc) in auto mode
2019-07-17T15:42:57.5715391Z update-alternatives: warning: skip creation of /usr/bin/x86_64-w64-mingw32-gcc-5 because associated file /usr/bin/x86_64-w64-mingw32-gcc-5-posix (of link group x86_64-w64-mingw32-gcc) doesn't exist
2019-07-17T15:42:57.5809766Z update-alternatives: using /usr/bin/x86_64-w64-mingw32-gcc-win32 to provide /usr/bin/x86_64-w64-mingw32-gcc (x86_64-w64-mingw32-gcc) in auto mode
2019-07-17T15:42:57.5813788Z update-alternatives: warning: skip creation of /usr/bin/x86_64-w64-mingw32-gcc-5 because associated file /usr/bin/x86_64-w64-mingw32-gcc-5-win32 (of link group x86_64-w64-mingw32-gcc) doesn't exist
2019-07-17T15:42:57.6082141Z Setting up g++-mingw-w64-x86-64 (5.3.1-8ubuntu3+17) ...
2019-07-17T15:42:57.6398340Z update-alternatives: using /usr/bin/x86_64-w64-mingw32-g++-posix to provide /usr/bin/x86_64-w64-mingw32-g++ (x86_64-w64-mingw32-g++) in auto mode
2019-07-17T15:42:57.6488666Z update-alternatives: using /usr/bin/x86_64-w64-mingw32-g++-win32 to provide /usr/bin/x86_64-w64-mingw32-g++ (x86_64-w64-mingw32-g++) in auto mode
2019-07-17T15:42:57.6745004Z Setting up g++-mingw-w64 (5.3.1-8ubuntu3+17) ...
2019-07-17T15:42:57.7154460Z Setting up gcc-mingw-w64 (5.3.1-8ubuntu3+17) ...
2019-07-17T15:42:57.7581748Z Setting up mingw-w64 (4.0.4-2) ...
2019-07-17T15:42:57.8545336Z Processing triggers for ca-certificates (20170717~16.04.2) ...
2019-07-17T15:42:57.8725827Z Updating certificates in /etc/ssl/certs...
2019-07-17T15:42:59.5132151Z 148 added, 0 removed; done.
2019-07-17T15:42:59.5134032Z Running hooks in /etc/ca-certificates/update.d...
---
2019-07-17T15:43:38.9535632Z Removing intermediate container 0e2e6ecd602f
2019-07-17T15:43:38.9537312Z  ---> 7566f487f965
2019-07-17T15:43:38.9586112Z Successfully built 7566f487f965
2019-07-17T15:43:39.1347280Z Successfully tagged rust-ci:latest
2019-07-17T15:43:39.2268512Z Built container sha256:7566f487f9656658e1e217e5a8423f5535b2f3c8c9d31c41a143cd3b33665bbc
2019-07-17T15:43:39.2328994Z Uploading finished image to https://.s3.amazonaws.com/docker/a4940e6914a5e1f6360ebc241d0d850aff083be5c43d268fd716ce48f63cb24e84238becb662f27f8c5d7740413ca17da24bb9cdc003ef32d7b03d2c9052b94d
2019-07-17T15:45:06.6547640Z upload failed: - to s3:///docker/a4940e6914a5e1f6360ebc241d0d850aff083be5c43d268fd716ce48f63cb24e84238becb662f27f8c5d7740413ca17da24bb9cdc003ef32d7b03d2c9052b94d Parameter validation failed:
2019-07-17T15:45:06.6548137Z Invalid bucket name "": Bucket name must match the regex "^[a-zA-Z0-9.\-_]{1,255}$"
2019-07-17T15:45:07.8089495Z [CI_JOB_NAME=mingw-check]
2019-07-17T15:45:07.8139999Z Starting sccache server...
2019-07-17T15:45:07.8762572Z configure: processing command line
2019-07-17T15:45:07.8762797Z configure: 
---
2019-07-17T15:47:28.1087828Z    Compiling autocfg v0.1.4
2019-07-17T15:47:29.9523928Z    Compiling unwind v0.0.0 (/checkout/src/libunwind)
2019-07-17T15:47:30.4986449Z    Compiling compiler_builtins v0.1.17
2019-07-17T15:47:33.4470440Z    Compiling cmake v0.1.38
2019-07-17T15:47:35.9966115Z error[E0723]: only int, `bool` and `char` operations are stable in const fn
2019-07-17T15:47:35.9986566Z    --> src/libcore/time.rs:516:30
2019-07-17T15:47:35.9994455Z     |
2019-07-17T15:47:36.0029934Z 516 |         (self.secs as f64) + (self.nanos as f64) / (NANOS_PER_SEC as f64)
2019-07-17T15:47:36.0030683Z     |
2019-07-17T15:47:36.0030683Z     |
2019-07-17T15:47:36.0031715Z     = note: for more information, see issue ***/issues/57563
2019-07-17T15:47:36.0032104Z     = help: add #![feature(const_fn)] to the crate attributes to enable
2019-07-17T15:47:36.0032149Z 
2019-07-17T15:47:36.0032452Z error[E0723]: only int, `bool` and `char` operations are stable in const fn
2019-07-17T15:47:36.0032777Z    --> src/libcore/time.rs:533:30
2019-07-17T15:47:36.0034207Z     |
2019-07-17T15:47:36.0034658Z 533 |         (self.secs as f32) + (self.nanos as f32) / (NANOS_PER_SEC as f32)
2019-07-17T15:47:36.0035313Z     |
2019-07-17T15:47:36.0035313Z     |
2019-07-17T15:47:36.0035731Z     = note: for more information, see issue ***/issues/57563
2019-07-17T15:47:36.0036087Z     = help: add #![feature(const_fn)] to the crate attributes to enable
2019-07-17T15:47:37.5510659Z error: aborting due to 2 previous errors
2019-07-17T15:47:37.5515838Z 
2019-07-17T15:47:37.5522164Z For more information about this error, try `rustc --explain E0723`.
2019-07-17T15:47:37.6392348Z    Compiling backtrace-sys v0.1.27
2019-07-17T15:47:37.6392348Z    Compiling backtrace-sys v0.1.27
2019-07-17T15:47:37.7075439Z error: Could not compile `core`.
2019-07-17T15:47:37.7075831Z warning: build failed, waiting for other jobs to finish...
2019-07-17T15:47:38.4632275Z error: build failed
2019-07-17T15:47:38.4662595Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
2019-07-17T15:47:38.4672651Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-07-17T15:47:38.4672718Z Build completed unsuccessfully in 0:02:30
2019-07-17T15:47:38.4672718Z Build completed unsuccessfully in 0:02:30
2019-07-17T15:47:40.8998682Z ##[error]Bash exited with code '1'.
2019-07-17T15:47:40.9033565Z ##[section]Starting: Checkout
2019-07-17T15:47:40.9035457Z ==============================================================================
2019-07-17T15:47:40.9035668Z Task         : Get sources
2019-07-17T15:47:40.9035740Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
