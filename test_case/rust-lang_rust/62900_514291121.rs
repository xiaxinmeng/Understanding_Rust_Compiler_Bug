plain
2019-07-23T16:16:39.5965657Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-23T16:16:40.2075883Z ##[command]git config gc.auto 0
2019-07-23T16:16:40.2080036Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-23T16:16:40.2083623Z ##[command]git config --get-all http.proxy
2019-07-23T16:16:40.2087072Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62900/merge:refs/remotes/pull/62900/merge
---
2019-07-23T16:17:13.8065249Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-23T16:17:13.8065280Z 
2019-07-23T16:17:13.8065509Z   git checkout -b <new-branch-name>
2019-07-23T16:17:13.8065539Z 
2019-07-23T16:17:13.8065613Z HEAD is now at c40a7396d Merge 65fac3709ac86e0f0ddea4a8a6d866f96c71970c into 3ebca72a11869f946b31f900faffb75c2bb2473a
2019-07-23T16:17:13.8208266Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-23T16:17:13.8210728Z ==============================================================================
2019-07-23T16:17:13.8210780Z Task         : Bash
2019-07-23T16:17:13.8210822Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-23T16:19:57.5636668Z Sending build context to Docker daemon  522.8kB
2019-07-23T16:19:57.5637019Z 
2019-07-23T16:19:57.5810975Z Step 1/6 : FROM ubuntu:16.04
2019-07-23T16:19:58.2504842Z 16.04: Pulling from library/ubuntu
2019-07-23T16:19:58.4693809Z f7277927d38a: Pulling fs layer
2019-07-23T16:19:58.4700185Z 8d3eac894db4: Pulling fs layer
2019-07-23T16:19:58.4700854Z edf72af6d627: Pulling fs layer
2019-07-23T16:19:58.4701101Z 3e4f86211d23: Pulling fs layer
2019-07-23T16:19:58.4703002Z 3e4f86211d23: Waiting
2019-07-23T16:19:58.7867838Z 8d3eac894db4: Verifying Checksum
2019-07-23T16:19:58.7868799Z 8d3eac894db4: Download complete
2019-07-23T16:19:58.8128817Z edf72af6d627: Download complete
2019-07-23T16:19:59.0992659Z 3e4f86211d23: Verifying Checksum
2019-07-23T16:19:59.0997642Z 3e4f86211d23: Download complete
2019-07-23T16:19:59.2383684Z f7277927d38a: Verifying Checksum
2019-07-23T16:19:59.2383802Z f7277927d38a: Download complete
2019-07-23T16:20:10.4358642Z f7277927d38a: Pull complete
2019-07-23T16:20:10.6075228Z 8d3eac894db4: Pull complete
2019-07-23T16:20:10.9524742Z edf72af6d627: Pull complete
2019-07-23T16:20:11.0796235Z 3e4f86211d23: Pull complete
2019-07-23T16:20:11.1204733Z Status: Downloaded newer image for ubuntu:16.04
2019-07-23T16:20:11.1211853Z  ---> 5e13f8dd4c1a
2019-07-23T16:20:11.1212393Z Step 2/6 : RUN apt-get update && apt-get install -y --no-install-recommends   g++   make   file   curl   ca-certificates   python2.7   git   cmake   sudo   gdb   xz-utils   libssl-dev   pkg-config   mingw-w64
2019-07-23T16:20:14.8146008Z  ---> Running in d564a97338cc
---
2019-07-23T16:20:19.9129378Z Reading package lists...
2019-07-23T16:20:20.9124122Z Reading package lists...
2019-07-23T16:20:21.0885136Z Building dependency tree...
2019-07-23T16:20:21.0885397Z Reading state information...
2019-07-23T16:20:21.2057689Z The following additional packages will be installed:
2019-07-23T16:20:21.2061950Z   binutils binutils-mingw-w64-i686 binutils-mingw-w64-x86-64 bzip2 cmake-data
2019-07-23T16:20:21.2062596Z   cpp cpp-5 dpkg-dev g++-5 g++-mingw-w64 g++-mingw-w64-i686
2019-07-23T16:20:21.2062885Z   g++-mingw-w64-x86-64 gcc gcc-5 gcc-mingw-w64 gcc-mingw-w64-base
2019-07-23T16:20:21.2063827Z   gcc-mingw-w64-i686 gcc-mingw-w64-x86-64 git-man libarchive13 libasan2
2019-07-23T16:20:21.2064126Z   libasn1-8-heimdal libatomic1 libbabeltrace-ctf1 libbabeltrace1 libc-dev-bin
2019-07-23T16:20:21.2075510Z   liberror-perl libexpat1 libffi6 libgcc-5-dev libgdbm3 libglib2.0-0 libgmp10
2019-07-23T16:20:21.2076051Z   libgnutls30 libgomp1 libgssapi-krb5-2 libgssapi3-heimdal libhcrypto4-heimdal
2019-07-23T16:20:21.2076978Z   libheimbase1-heimdal libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal
2019-07-23T16:20:21.2077835Z   libicu55 libidn11 libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1
2019-07-23T16:20:21.2077835Z   libicu55 libidn11 libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1
2019-07-23T16:20:21.2078174Z   libkrb5-26-heimdal libkrb5-3 libkrb5support0 libldap-2.4-2 liblsan0
2019-07-23T16:20:21.2078456Z   liblzo2-2 libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6
2019-07-23T16:20:21.2079278Z   libp11-kit0 libperl5.22 libpython2.7-minimal libpython2.7-stdlib
2019-07-23T16:20:21.2079633Z   libpython3.5 libpython3.5-minimal libpython3.5-stdlib libquadmath0
2019-07-23T16:20:21.2079970Z   libroken18-heimdal librtmp1 libsasl2-2 libsasl2-modules-db libsqlite3-0
2019-07-23T16:20:21.2080285Z   libssl1.0.0 libstdc++-5-dev libtasn1-6 libtsan0 libubsan0 libwind0-heimdal
2019-07-23T16:20:21.2080589Z   libxml2 linux-libc-dev mime-support mingw-w64-common mingw-w64-i686-dev
2019-07-23T16:20:21.2080920Z   mingw-w64-x86-64-dev openssl patch perl perl-modules-5.22 python2.7-minimal
2019-07-23T16:20:21.2081185Z   zlib1g-dev
2019-07-23T16:20:21.2083435Z Suggested packages:
2019-07-23T16:20:21.2083853Z   binutils-doc bzip2-doc codeblocks eclipse ninja-build cpp-doc gcc-5-locales
2019-07-23T16:20:21.2084117Z   debian-keyring g++-multilib g++-5-multilib gcc-5-doc libstdc++6-5-dbg
2019-07-23T16:20:21.2084374Z   gcc-multilib manpages-dev autoconf automake libtool flex bison gcc-doc
2019-07-23T16:20:21.2084931Z   libasan2-dbg liblsan0-dbg libtsan0-dbg libubsan0-dbg libcilkrts5-dbg
2019-07-23T16:20:21.2085191Z   libmpx0-dbg libquadmath0-dbg gdb-doc gettext-base git-daemon-run
2019-07-23T16:20:21.2085191Z   libmpx0-dbg libquadmath0-dbg gdb-doc gettext-base git-daemon-run
2019-07-23T16:20:21.2085637Z   | git-daemon-sysvinit git-doc git-el git-email git-gui gitk gitweb git-arch
2019-07-23T16:20:21.2085888Z   git-cvs git-mediawiki git-svn lrzip glibc-doc gnutls-bin krb5-doc krb5-user
2019-07-23T16:20:21.2086130Z   libstdc++-5-doc make-doc wine wine64 ed diffutils-doc perl-doc
2019-07-23T16:20:21.2086402Z   libterm-readline-gnu-perl | libterm-readline-perl-perl python2.7-doc
2019-07-23T16:20:21.2086618Z   binfmt-support
2019-07-23T16:20:21.2086662Z Recommended packages:
2019-07-23T16:20:21.2086925Z   build-essential fakeroot libalgorithm-merge-perl gfortran-mingw-w64
2019-07-23T16:20:21.2087173Z   gnat-mingw-w64 libc-dbg gdbserver less rsync ssh-client manpages
2019-07-23T16:20:21.2087420Z   manpages-dev libfile-fcntllock-perl libglib2.0-data shared-mime-info
2019-07-23T16:20:21.2087690Z   xdg-user-dirs krb5-locales libsasl2-modules libssl-doc xml-core netbase
2019-07-23T16:20:21.2087745Z   rename
2019-07-23T16:20:21.2087788Z The following NEW packages will be installed:
2019-07-23T16:20:21.2088059Z   binutils binutils-mingw-w64-i686 binutils-mingw-w64-x86-64 bzip2
2019-07-23T16:20:21.2089948Z   ca-certificates cmake cmake-data cpp cpp-5 curl dpkg-dev file g++ g++-5
2019-07-23T16:20:21.2090260Z   g++-mingw-w64 g++-mingw-w64-i686 g++-mingw-w64-x86-64 gcc gcc-5
2019-07-23T16:20:21.2090576Z   gcc-mingw-w64 gcc-mingw-w64-base gcc-mingw-w64-i686 gcc-mingw-w64-x86-64 gdb
2019-07-23T16:20:21.2090877Z   git git-man libarchive13 libasan2 libasn1-8-heimdal libatomic1
2019-07-23T16:20:21.2091152Z   libbabeltrace-ctf1 libbabeltrace1 libc-dev-bin libc6-dev libcc1-0
2019-07-23T16:20:21.2091738Z   libffi6 libgcc-5-dev libgdbm3 libglib2.0-0 libgmp10 libgnutls30 libgomp1
2019-07-23T16:20:21.2092022Z   libgssapi-krb5-2 libgssapi3-heimdal libhcrypto4-heimdal libheimbase1-heimdal
2019-07-23T16:20:21.2092490Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-23T16:20:21.2094454Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-23T16:20:21.2094454Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-23T16:20:21.2094969Z   libkrb5-3 libkrb5support0 libldap-2.4-2 liblsan0 liblzo2-2 libmagic1 libmpc3
2019-07-23T16:20:21.2095252Z   libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0 libperl5.22
2019-07-23T16:20:21.2095526Z   libpython2.7-minimal libpython2.7-stdlib libpython3.5 libpython3.5-minimal
2019-07-23T16:20:21.2096237Z   libpython3.5-stdlib libquadmath0 libroken18-heimdal librtmp1 libsasl2-2
2019-07-23T16:20:21.2096533Z   libsasl2-modules-db libsqlite3-0 libssl-dev libssl1.0.0 libstdc++-5-dev
2019-07-23T16:20:21.2096967Z   libtasn1-6 libtsan0 libubsan0 libwind0-heimdal libxml2 linux-libc-dev make
2019-07-23T16:20:21.2097385Z   mime-support mingw-w64 mingw-w64-common mingw-w64-i686-dev
2019-07-23T16:20:21.2097815Z   mingw-w64-x86-64-dev openssl patch perl perl-modules-5.22 pkg-config
2019-07-23T16:20:21.2098061Z   python2.7 python2.7-minimal sudo xz-utils zlib1g-dev
2019-07-23T16:20:21.6954248Z Need to get 187 MB of archives.
2019-07-23T16:20:21.6954369Z After this operation, 968 MB of additional disk space will be used.
2019-07-23T16:20:21.6955284Z Get:1 http://archive.ubuntu.com/ubuntu xenial/main amd64 libgdbm3 amd64 1.8.3-13.1 [16.9 kB]
2019-07-23T16:20:21.9076712Z Get:2 http://archive.ubuntu.com/ubuntu xenial/main amd64 libffi6 amd64 3.2.1-4 [17.8 kB]
---
2019-07-23T16:20:25.8219836Z Get:96 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 zlib1g-dev amd64 1:1.2.8.dfsg-2ubuntu4.1 [168 kB]
2019-07-23T16:20:25.8248355Z Get:97 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 libssl-dev amd64 1.0.2g-1ubuntu4.15 [1344 kB]
2019-07-23T16:20:25.8765383Z Get:98 http://archive.ubuntu.com/ubuntu xenial/main amd64 pkg-config amd64 0.29.1-0ubuntu1 [45.0 kB]
2019-07-23T16:20:25.8776703Z Get:99 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 python2.7 amd64 2.7.12-1ubuntu0~16.04.4 [224 kB]
2019-07-23T16:20:25.8814285Z Get:100 http://archive.ubuntu.com/ubuntu xenial/universe amd64 binutils-mingw-w64-i686 amd64 2.26-3ubuntu1+6.6 [1782 kB]
2019-07-23T16:20:25.9365658Z Get:101 http://archive.ubuntu.com/ubuntu xenial/universe amd64 binutils-mingw-w64-x86-64 amd64 2.26-3ubuntu1+6.6 [2029 kB]
2019-07-23T16:20:26.2424494Z Get:102 http://archive.ubuntu.com/ubuntu xenial/universe amd64 mingw-w64-common all 4.0.4-2 [4787 kB]
2019-07-23T16:20:27.2685130Z Get:103 http://archive.ubuntu.com/ubuntu xenial/universe amd64 mingw-w64-i686-dev all 4.0.4-2 [2059 kB]
2019-07-23T16:20:27.3000161Z Get:104 http://archive.ubuntu.com/ubuntu xenial/universe amd64 gcc-mingw-w64-base amd64 5.3.1-8ubuntu3+17 [11.2 kB]
2019-07-23T16:20:27.3008430Z Get:105 http://archive.ubuntu.com/ubuntu xenial/universe amd64 gcc-mingw-w64-i686 amd64 5.3.1-8ubuntu3+17 [27.3 MB]
2019-07-23T16:20:28.2837893Z Get:106 http://archive.ubuntu.com/ubuntu xenial/universe amd64 g++-mingw-w64-i686 amd64 5.3.1-8ubuntu3+17 [19.8 MB]
2019-07-23T16:20:29.0210627Z Get:107 http://archive.ubuntu.com/ubuntu xenial/universe amd64 mingw-w64-x86-64-dev all 4.0.4-2 [3238 kB]
2019-07-23T16:20:29.1332066Z Get:108 http://archive.ubuntu.com/ubuntu xenial/universe amd64 gcc-mingw-w64-x86-64 amd64 5.3.1-8ubuntu3+17 [27.4 MB]
2019-07-23T16:20:30.1165225Z Get:109 http://archive.ubuntu.com/ubuntu xenial/universe amd64 g++-mingw-w64-x86-64 amd64 5.3.1-8ubuntu3+17 [20.4 MB]
2019-07-23T16:20:30.8649397Z Get:110 http://archive.ubuntu.com/ubuntu xenial/universe amd64 g++-mingw-w64 all 5.3.1-8ubuntu3+17 [10.7 kB]
2019-07-23T16:20:30.8656970Z Get:111 http://archive.ubuntu.com/ubuntu xenial/universe amd64 gcc-mingw-w64 all 5.3.1-8ubuntu3+17 [10.7 kB]
2019-07-23T16:20:30.8657528Z Get:112 http://archive.ubuntu.com/ubuntu xenial/universe amd64 mingw-w64 all 4.0.4-2 [9274 B]
2019-07-23T16:20:33.1441267Z debconf: delaying package configuration, since apt-utils is not installed
2019-07-23T16:20:33.1669691Z Fetched 187 MB in 9s (19.9 MB/s)
2019-07-23T16:20:33.2394778Z (Reading database ... 
2019-07-23T16:20:33.2395087Z (Reading database ... 5%
2019-07-23T16:20:33.2395280Z (Reading database ... 10%
2019-07-23T16:20:33.2395489Z (Reading database ... 15%
---
2019-07-23T16:21:00.1008249Z Unpacking pkg-config (0.29.1-0ubuntu1) ...
2019-07-23T16:21:00.2211941Z Selecting previously unselected package python2.7.
2019-07-23T16:21:00.2231552Z Preparing to unpack .../python2.7_2.7.12-1ubuntu0~16.04.4_amd64.deb ...
2019-07-23T16:21:00.2394382Z Unpacking python2.7 (2.7.12-1ubuntu0~16.04.4) ...
2019-07-23T16:21:00.3452910Z Selecting previously unselected package binutils-mingw-w64-i686.
2019-07-23T16:21:00.3468527Z Preparing to unpack .../binutils-mingw-w64-i686_2.26-3ubuntu1+6.6_amd64.deb ...
2019-07-23T16:21:00.3610353Z Unpacking binutils-mingw-w64-i686 (2.26-3ubuntu1+6.6) ...
2019-07-23T16:21:00.9078928Z Selecting previously unselected package binutils-mingw-w64-x86-64.
2019-07-23T16:21:00.9096433Z Preparing to unpack .../binutils-mingw-w64-x86-64_2.26-3ubuntu1+6.6_amd64.deb ...
2019-07-23T16:21:00.9245204Z Unpacking binutils-mingw-w64-x86-64 (2.26-3ubuntu1+6.6) ...
2019-07-23T16:21:01.5880532Z Selecting previously unselected package mingw-w64-common.
2019-07-23T16:21:01.5906249Z Preparing to unpack .../mingw-w64-common_4.0.4-2_all.deb ...
2019-07-23T16:21:01.6055904Z Unpacking mingw-w64-common (4.0.4-2) ...
2019-07-23T16:21:03.0517515Z Selecting previously unselected package mingw-w64-i686-dev.
2019-07-23T16:21:03.0544703Z Preparing to unpack .../mingw-w64-i686-dev_4.0.4-2_all.deb ...
2019-07-23T16:21:03.0688314Z Unpacking mingw-w64-i686-dev (4.0.4-2) ...
2019-07-23T16:21:04.2167957Z Selecting previously unselected package gcc-mingw-w64-base.
2019-07-23T16:21:04.2190744Z Preparing to unpack .../gcc-mingw-w64-base_5.3.1-8ubuntu3+17_amd64.deb ...
2019-07-23T16:21:04.2337065Z Unpacking gcc-mingw-w64-base (5.3.1-8ubuntu3+17) ...
2019-07-23T16:21:04.3531992Z Selecting previously unselected package gcc-mingw-w64-i686.
2019-07-23T16:21:04.3551668Z Preparing to unpack .../gcc-mingw-w64-i686_5.3.1-8ubuntu3+17_amd64.deb ...
2019-07-23T16:21:04.3719786Z Unpacking gcc-mingw-w64-i686 (5.3.1-8ubuntu3+17) ...
2019-07-23T16:21:08.4622959Z Selecting previously unselected package g++-mingw-w64-i686.
2019-07-23T16:21:08.4648309Z Preparing to unpack .../g++-mingw-w64-i686_5.3.1-8ubuntu3+17_amd64.deb ...
2019-07-23T16:21:08.4795943Z Unpacking g++-mingw-w64-i686 (5.3.1-8ubuntu3+17) ...
2019-07-23T16:21:12.0878206Z Selecting previously unselected package mingw-w64-x86-64-dev.
2019-07-23T16:21:12.0904975Z Preparing to unpack .../mingw-w64-x86-64-dev_4.0.4-2_all.deb ...
2019-07-23T16:21:12.1048219Z Unpacking mingw-w64-x86-64-dev (4.0.4-2) ...
2019-07-23T16:21:14.0550761Z Selecting previously unselected package gcc-mingw-w64-x86-64.
2019-07-23T16:21:14.0582918Z Preparing to unpack .../gcc-mingw-w64-x86-64_5.3.1-8ubuntu3+17_amd64.deb ...
2019-07-23T16:21:14.0723692Z Unpacking gcc-mingw-w64-x86-64 (5.3.1-8ubuntu3+17) ...
2019-07-23T16:21:18.3232367Z Selecting previously unselected package g++-mingw-w64-x86-64.
2019-07-23T16:21:18.3257594Z Preparing to unpack .../g++-mingw-w64-x86-64_5.3.1-8ubuntu3+17_amd64.deb ...
2019-07-23T16:21:18.3402512Z Unpacking g++-mingw-w64-x86-64 (5.3.1-8ubuntu3+17) ...
2019-07-23T16:21:22.2089678Z Selecting previously unselected package g++-mingw-w64.
2019-07-23T16:21:22.2124702Z Preparing to unpack .../g++-mingw-w64_5.3.1-8ubuntu3+17_all.deb ...
2019-07-23T16:21:22.2297935Z Unpacking g++-mingw-w64 (5.3.1-8ubuntu3+17) ...
2019-07-23T16:21:22.3508168Z Selecting previously unselected package gcc-mingw-w64.
2019-07-23T16:21:22.3540478Z Preparing to unpack .../gcc-mingw-w64_5.3.1-8ubuntu3+17_all.deb ...
2019-07-23T16:21:22.3718607Z Unpacking gcc-mingw-w64 (5.3.1-8ubuntu3+17) ...
2019-07-23T16:21:22.4588123Z Selecting previously unselected package mingw-w64.
2019-07-23T16:21:22.4615817Z Preparing to unpack .../mingw-w64_4.0.4-2_all.deb ...
2019-07-23T16:21:22.4779708Z Unpacking mingw-w64 (4.0.4-2) ...
2019-07-23T16:21:22.7781553Z Setting up libgdbm3:amd64 (1.8.3-13.1) ...
2019-07-23T16:21:22.8621332Z Setting up libffi6:amd64 (3.2.1-4) ...
2019-07-23T16:21:22.9033183Z Setting up libglib2.0-0:amd64 (2.48.2-0ubuntu4.3) ...
2019-07-23T16:21:22.9350030Z No schema files found: doing nothing.
---
2019-07-23T16:21:28.7477170Z Setting up zlib1g-dev:amd64 (1:1.2.8.dfsg-2ubuntu4.1) ...
2019-07-23T16:21:28.7960798Z Setting up libssl-dev:amd64 (1.0.2g-1ubuntu4.15) ...
2019-07-23T16:21:28.8470467Z Setting up pkg-config (0.29.1-0ubuntu1) ...
2019-07-23T16:21:28.9421073Z Setting up python2.7 (2.7.12-1ubuntu0~16.04.4) ...
2019-07-23T16:21:29.7937741Z Setting up binutils-mingw-w64-i686 (2.26-3ubuntu1+6.6) ...
2019-07-23T16:21:29.8387584Z Setting up binutils-mingw-w64-x86-64 (2.26-3ubuntu1+6.6) ...
2019-07-23T16:21:29.8868787Z Setting up mingw-w64-common (4.0.4-2) ...
2019-07-23T16:21:29.9318418Z Setting up mingw-w64-i686-dev (4.0.4-2) ...
2019-07-23T16:21:29.9758619Z Setting up gcc-mingw-w64-base (5.3.1-8ubuntu3+17) ...
2019-07-23T16:21:30.0190337Z Setting up gcc-mingw-w64-i686 (5.3.1-8ubuntu3+17) ...
2019-07-23T16:21:30.0498914Z update-alternatives: using /usr/bin/i686-w64-mingw32-gcc-posix to provide /usr/bin/i686-w64-mingw32-gcc (i686-w64-mingw32-gcc) in auto mode
2019-07-23T16:21:30.0504903Z update-alternatives: warning: skip creation of /usr/bin/i686-w64-mingw32-gcc-5 because associated file /usr/bin/i686-w64-mingw32-gcc-5-posix (of link group i686-w64-mingw32-gcc) doesn't exist
2019-07-23T16:21:30.0598240Z update-alternatives: using /usr/bin/i686-w64-mingw32-gcc-win32 to provide /usr/bin/i686-w64-mingw32-gcc (i686-w64-mingw32-gcc) in auto mode
2019-07-23T16:21:30.0599951Z update-alternatives: warning: skip creation of /usr/bin/i686-w64-mingw32-gcc-5 because associated file /usr/bin/i686-w64-mingw32-gcc-5-win32 (of link group i686-w64-mingw32-gcc) doesn't exist
2019-07-23T16:21:30.0926451Z Setting up g++-mingw-w64-i686 (5.3.1-8ubuntu3+17) ...
2019-07-23T16:21:30.1231183Z update-alternatives: using /usr/bin/i686-w64-mingw32-g++-posix to provide /usr/bin/i686-w64-mingw32-g++ (i686-w64-mingw32-g++) in auto mode
2019-07-23T16:21:30.1325633Z update-alternatives: using /usr/bin/i686-w64-mingw32-g++-win32 to provide /usr/bin/i686-w64-mingw32-g++ (i686-w64-mingw32-g++) in auto mode
2019-07-23T16:21:30.1596006Z Setting up mingw-w64-x86-64-dev (4.0.4-2) ...
2019-07-23T16:21:30.2046852Z Setting up gcc-mingw-w64-x86-64 (5.3.1-8ubuntu3+17) ...
2019-07-23T16:21:30.2360189Z update-alternatives: using /usr/bin/x86_64-w64-mingw32-gcc-posix to provide /usr/bin/x86_64-w64-mingw32-gcc (x86_64-w64-mingw32-gcc) in auto mode
2019-07-23T16:21:30.2364820Z update-alternatives: warning: skip creation of /usr/bin/x86_64-w64-mingw32-gcc-5 because associated file /usr/bin/x86_64-w64-mingw32-gcc-5-posix (of link group x86_64-w64-mingw32-gcc) doesn't exist
2019-07-23T16:21:30.2536699Z update-alternatives: using /usr/bin/x86_64-w64-mingw32-gcc-win32 to provide /usr/bin/x86_64-w64-mingw32-gcc (x86_64-w64-mingw32-gcc) in auto mode
2019-07-23T16:21:30.2537828Z update-alternatives: warning: skip creation of /usr/bin/x86_64-w64-mingw32-gcc-5 because associated file /usr/bin/x86_64-w64-mingw32-gcc-5-win32 (of link group x86_64-w64-mingw32-gcc) doesn't exist
2019-07-23T16:21:30.2795852Z Setting up g++-mingw-w64-x86-64 (5.3.1-8ubuntu3+17) ...
2019-07-23T16:21:30.3127629Z update-alternatives: using /usr/bin/x86_64-w64-mingw32-g++-posix to provide /usr/bin/x86_64-w64-mingw32-g++ (x86_64-w64-mingw32-g++) in auto mode
2019-07-23T16:21:30.3219520Z update-alternatives: using /usr/bin/x86_64-w64-mingw32-g++-win32 to provide /usr/bin/x86_64-w64-mingw32-g++ (x86_64-w64-mingw32-g++) in auto mode
2019-07-23T16:21:30.3475029Z Setting up g++-mingw-w64 (5.3.1-8ubuntu3+17) ...
2019-07-23T16:21:30.3952505Z Setting up gcc-mingw-w64 (5.3.1-8ubuntu3+17) ...
2019-07-23T16:21:30.4416984Z Setting up mingw-w64 (4.0.4-2) ...
2019-07-23T16:21:30.5329716Z Processing triggers for ca-certificates (20170717~16.04.2) ...
2019-07-23T16:21:30.5520759Z Updating certificates in /etc/ssl/certs...
2019-07-23T16:21:32.1686579Z 148 added, 0 removed; done.
2019-07-23T16:21:32.1688046Z Running hooks in /etc/ca-certificates/update.d...
---
2019-07-23T16:22:15.3152182Z Removing intermediate container 6f2f6d965218
2019-07-23T16:22:15.3153131Z  ---> 2b57eddb7b18
2019-07-23T16:22:15.3199742Z Successfully built 2b57eddb7b18
2019-07-23T16:22:15.7000093Z Successfully tagged rust-ci:latest
2019-07-23T16:22:15.7840925Z Built container sha256:2b57eddb7b1887b2388d427c5e85e94de933296a29c4ac39cf224e6f3d86c5b0
2019-07-23T16:22:15.7870542Z Uploading finished image to https://rust-lang-ci-sccache2.s3.amazonaws.com/docker/a4940e6914a5e1f6360ebc241d0d850aff083be5c43d268fd716ce48f63cb24e84238becb662f27f8c5d7740413ca17da24bb9cdc003ef32d7b03d2c9052b94d
2019-07-23T16:23:43.9561017Z upload failed: - to s3://rust-lang-ci-sccache2/docker/a4940e6914a5e1f6360ebc241d0d850aff083be5c43d268fd716ce48f63cb24e84238becb662f27f8c5d7740413ca17da24bb9cdc003ef32d7b03d2c9052b94d Unable to locate credentials
2019-07-23T16:23:44.9890772Z [CI_JOB_NAME=mingw-check]
2019-07-23T16:23:44.9944486Z Starting sccache server...
2019-07-23T16:23:45.0902442Z configure: processing command line
2019-07-23T16:23:45.0902609Z configure: 
---
2019-07-23T16:28:21.0238134Z     Checking syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
2019-07-23T16:28:22.5413866Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2019-07-23T16:28:23.8159103Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2019-07-23T16:28:37.2275932Z     Checking syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
2019-07-23T16:28:42.6265455Z error[E0121]: the type placeholder `_` is not allowed within types on item signatures
2019-07-23T16:28:42.6267070Z      |
2019-07-23T16:28:42.6267070Z      |
2019-07-23T16:28:42.6268031Z 1153 |         msg: PanicMessage<_>,
2019-07-23T16:28:42.6268742Z      |                           ^ not allowed in type signatures
2019-07-23T16:28:43.0017862Z error: aborting due to previous error
2019-07-23T16:28:43.0018645Z 
2019-07-23T16:28:43.0019196Z For more information about this error, try `rustc --explain E0121`.
2019-07-23T16:28:43.1493999Z error: Could not compile `rustc`.
2019-07-23T16:28:43.1493999Z error: Could not compile `rustc`.
2019-07-23T16:28:43.1494079Z 
2019-07-23T16:28:43.1494562Z To learn more, run the command again with --verbose.
2019-07-23T16:28:43.1522983Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
2019-07-23T16:28:43.1534595Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-07-23T16:28:43.1534678Z Build completed unsuccessfully in 0:04:58
2019-07-23T16:28:43.1534678Z Build completed unsuccessfully in 0:04:58
2019-07-23T16:28:44.1473699Z ##[error]Bash exited with code '1'.
2019-07-23T16:28:44.1508767Z ##[section]Starting: Checkout
2019-07-23T16:28:44.1511368Z ==============================================================================
2019-07-23T16:28:44.1511445Z Task         : Get sources
2019-07-23T16:28:44.1511494Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
