plain
2019-07-23T17:31:03.1011909Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-23T17:31:03.1309749Z ##[command]git config gc.auto 0
2019-07-23T17:31:03.1395713Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-23T17:31:03.1450887Z ##[command]git config --get-all http.proxy
2019-07-23T17:31:03.1590854Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62903/merge:refs/remotes/pull/62903/merge
---
2019-07-23T17:31:39.3417571Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-23T17:31:39.3417609Z 
2019-07-23T17:31:39.3417820Z   git checkout -b <new-branch-name>
2019-07-23T17:31:39.3417849Z 
2019-07-23T17:31:39.3417918Z HEAD is now at 422791fb1 Merge ccb5d0a7c170434bb2ced860ec7139f4c6f964ce into 3ebca72a11869f946b31f900faffb75c2bb2473a
2019-07-23T17:31:39.3553775Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-23T17:31:39.3556771Z ==============================================================================
2019-07-23T17:31:39.3556830Z Task         : Bash
2019-07-23T17:31:39.3556894Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-23T17:34:32.6632240Z Sending build context to Docker daemon  522.8kB
2019-07-23T17:34:32.6632456Z 
2019-07-23T17:34:32.6632598Z Step 1/6 : FROM ubuntu:16.04
2019-07-23T17:34:32.6632670Z 16.04: Pulling from library/ubuntu
2019-07-23T17:34:32.7616797Z f7277927d38a: Pulling fs layer
2019-07-23T17:34:32.7620618Z 8d3eac894db4: Pulling fs layer
2019-07-23T17:34:32.7622925Z edf72af6d627: Pulling fs layer
2019-07-23T17:34:32.7623709Z 3e4f86211d23: Pulling fs layer
2019-07-23T17:34:32.7625343Z 3e4f86211d23: Waiting
2019-07-23T17:34:33.0998821Z edf72af6d627: Verifying Checksum
2019-07-23T17:34:33.1000553Z edf72af6d627: Download complete
2019-07-23T17:34:33.1014777Z 8d3eac894db4: Verifying Checksum
2019-07-23T17:34:33.1014906Z 8d3eac894db4: Download complete
2019-07-23T17:34:33.4152707Z 3e4f86211d23: Verifying Checksum
2019-07-23T17:34:33.4154764Z 3e4f86211d23: Download complete
2019-07-23T17:34:33.4748717Z f7277927d38a: Verifying Checksum
2019-07-23T17:34:33.4748848Z f7277927d38a: Download complete
2019-07-23T17:34:35.3245534Z f7277927d38a: Pull complete
2019-07-23T17:34:35.4678540Z 8d3eac894db4: Pull complete
2019-07-23T17:34:35.6145099Z edf72af6d627: Pull complete
2019-07-23T17:34:35.7689989Z 3e4f86211d23: Pull complete
2019-07-23T17:34:35.7902027Z Digest: sha256:97b54e5692c27072234ff958a7442dde4266af21e7b688e7fca5dc5acc8ed7d9
2019-07-23T17:34:35.8066370Z  ---> 5e13f8dd4c1a
2019-07-23T17:34:35.8068617Z Step 2/6 : RUN apt-get update && apt-get install -y --no-install-recommends   g++   make   file   curl   ca-certificates   python2.7   git   cmake   sudo   gdb   xz-utils   libssl-dev   pkg-config   mingw-w64
2019-07-23T17:34:39.9555573Z  ---> Running in de6fa991633e
2019-07-23T17:34:41.3351143Z Get:1 http://security.ubuntu.com/ubuntu xenial-security InRelease [109 kB]
---
2019-07-23T17:34:44.9922433Z Reading package lists...
2019-07-23T17:34:45.9702887Z Reading package lists...
2019-07-23T17:34:46.1493042Z Building dependency tree...
2019-07-23T17:34:46.1493346Z Reading state information...
2019-07-23T17:34:46.2597417Z The following additional packages will be installed:
2019-07-23T17:34:46.2598584Z   binutils binutils-mingw-w64-i686 binutils-mingw-w64-x86-64 bzip2 cmake-data
2019-07-23T17:34:46.2603396Z   cpp cpp-5 dpkg-dev g++-5 g++-mingw-w64 g++-mingw-w64-i686
2019-07-23T17:34:46.2611897Z   g++-mingw-w64-x86-64 gcc gcc-5 gcc-mingw-w64 gcc-mingw-w64-base
2019-07-23T17:34:46.2612679Z   gcc-mingw-w64-i686 gcc-mingw-w64-x86-64 git-man libarchive13 libasan2
2019-07-23T17:34:46.2613022Z   libasn1-8-heimdal libatomic1 libbabeltrace-ctf1 libbabeltrace1 libc-dev-bin
2019-07-23T17:34:46.2613589Z   liberror-perl libexpat1 libffi6 libgcc-5-dev libgdbm3 libglib2.0-0 libgmp10
2019-07-23T17:34:46.2613856Z   libgnutls30 libgomp1 libgssapi-krb5-2 libgssapi3-heimdal libhcrypto4-heimdal
2019-07-23T17:34:46.2614115Z   libheimbase1-heimdal libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal
2019-07-23T17:34:46.2614218Z   libicu55 libidn11 libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1
2019-07-23T17:34:46.2614218Z   libicu55 libidn11 libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1
2019-07-23T17:34:46.2614476Z   libkrb5-26-heimdal libkrb5-3 libkrb5support0 libldap-2.4-2 liblsan0
2019-07-23T17:34:46.2614899Z   liblzo2-2 libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6
2019-07-23T17:34:46.2615200Z   libp11-kit0 libperl5.22 libpython2.7-minimal libpython2.7-stdlib
2019-07-23T17:34:46.2615510Z   libpython3.5 libpython3.5-minimal libpython3.5-stdlib libquadmath0
2019-07-23T17:34:46.2628734Z   libroken18-heimdal librtmp1 libsasl2-2 libsasl2-modules-db libsqlite3-0
2019-07-23T17:34:46.2629045Z   libssl1.0.0 libstdc++-5-dev libtasn1-6 libtsan0 libubsan0 libwind0-heimdal
2019-07-23T17:34:46.2629367Z   libxml2 linux-libc-dev mime-support mingw-w64-common mingw-w64-i686-dev
2019-07-23T17:34:46.2629627Z   mingw-w64-x86-64-dev openssl patch perl perl-modules-5.22 python2.7-minimal
2019-07-23T17:34:46.2629821Z   zlib1g-dev
2019-07-23T17:34:46.2629914Z Suggested packages:
2019-07-23T17:34:46.2630169Z   binutils-doc bzip2-doc codeblocks eclipse ninja-build cpp-doc gcc-5-locales
2019-07-23T17:34:46.2630417Z   debian-keyring g++-multilib g++-5-multilib gcc-5-doc libstdc++6-5-dbg
2019-07-23T17:34:46.2630724Z   gcc-multilib manpages-dev autoconf automake libtool flex bison gcc-doc
2019-07-23T17:34:46.2631229Z   libasan2-dbg liblsan0-dbg libtsan0-dbg libubsan0-dbg libcilkrts5-dbg
2019-07-23T17:34:46.2631511Z   libmpx0-dbg libquadmath0-dbg gdb-doc gettext-base git-daemon-run
2019-07-23T17:34:46.2631511Z   libmpx0-dbg libquadmath0-dbg gdb-doc gettext-base git-daemon-run
2019-07-23T17:34:46.2631764Z   | git-daemon-sysvinit git-doc git-el git-email git-gui gitk gitweb git-arch
2019-07-23T17:34:46.2632015Z   git-cvs git-mediawiki git-svn lrzip glibc-doc gnutls-bin krb5-doc krb5-user
2019-07-23T17:34:46.2632295Z   libstdc++-5-doc make-doc wine wine64 ed diffutils-doc perl-doc
2019-07-23T17:34:46.2632545Z   libterm-readline-gnu-perl | libterm-readline-perl-perl python2.7-doc
2019-07-23T17:34:46.2632740Z   binfmt-support
2019-07-23T17:34:46.2632829Z Recommended packages:
2019-07-23T17:34:46.2633072Z   build-essential fakeroot libalgorithm-merge-perl gfortran-mingw-w64
2019-07-23T17:34:46.2633322Z   gnat-mingw-w64 libc-dbg gdbserver less rsync ssh-client manpages
2019-07-23T17:34:46.2633607Z   manpages-dev libfile-fcntllock-perl libglib2.0-data shared-mime-info
2019-07-23T17:34:46.2633872Z   xdg-user-dirs krb5-locales libsasl2-modules libssl-doc xml-core netbase
2019-07-23T17:34:46.2633923Z   rename
2019-07-23T17:34:46.2633970Z The following NEW packages will be installed:
2019-07-23T17:34:46.2634263Z   binutils binutils-mingw-w64-i686 binutils-mingw-w64-x86-64 bzip2
2019-07-23T17:34:46.2634508Z   ca-certificates cmake cmake-data cpp cpp-5 curl dpkg-dev file g++ g++-5
2019-07-23T17:34:46.2634753Z   g++-mingw-w64 g++-mingw-w64-i686 g++-mingw-w64-x86-64 gcc gcc-5
2019-07-23T17:34:46.2635059Z   gcc-mingw-w64 gcc-mingw-w64-base gcc-mingw-w64-i686 gcc-mingw-w64-x86-64 gdb
2019-07-23T17:34:46.2635301Z   git git-man libarchive13 libasan2 libasn1-8-heimdal libatomic1
2019-07-23T17:34:46.2635576Z   libbabeltrace-ctf1 libbabeltrace1 libc-dev-bin libc6-dev libcc1-0
2019-07-23T17:34:46.2636144Z   libffi6 libgcc-5-dev libgdbm3 libglib2.0-0 libgmp10 libgnutls30 libgomp1
2019-07-23T17:34:46.2636541Z   libgssapi-krb5-2 libgssapi3-heimdal libhcrypto4-heimdal libheimbase1-heimdal
2019-07-23T17:34:46.2636899Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-23T17:34:46.2637154Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-23T17:34:46.2637154Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-23T17:34:46.2637410Z   libkrb5-3 libkrb5support0 libldap-2.4-2 liblsan0 liblzo2-2 libmagic1 libmpc3
2019-07-23T17:34:46.2638129Z   libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0 libperl5.22
2019-07-23T17:34:46.2638551Z   libpython2.7-minimal libpython2.7-stdlib libpython3.5 libpython3.5-minimal
2019-07-23T17:34:46.2638803Z   libpython3.5-stdlib libquadmath0 libroken18-heimdal librtmp1 libsasl2-2
2019-07-23T17:34:46.2639108Z   libsasl2-modules-db libsqlite3-0 libssl-dev libssl1.0.0 libstdc++-5-dev
2019-07-23T17:34:46.2639503Z   libtasn1-6 libtsan0 libubsan0 libwind0-heimdal libxml2 linux-libc-dev make
2019-07-23T17:34:46.2639743Z   mime-support mingw-w64 mingw-w64-common mingw-w64-i686-dev
2019-07-23T17:34:46.2640056Z   mingw-w64-x86-64-dev openssl patch perl perl-modules-5.22 pkg-config
2019-07-23T17:34:46.2640289Z   python2.7 python2.7-minimal sudo xz-utils zlib1g-dev
2019-07-23T17:34:46.7080727Z Need to get 187 MB of archives.
2019-07-23T17:34:46.7080844Z After this operation, 968 MB of additional disk space will be used.
2019-07-23T17:34:46.7081641Z Get:1 http://archive.ubuntu.com/ubuntu xenial/main amd64 libgdbm3 amd64 1.8.3-13.1 [16.9 kB]
2019-07-23T17:34:46.9273580Z Get:2 http://archive.ubuntu.com/ubuntu xenial/main amd64 libffi6 amd64 3.2.1-4 [17.8 kB]
---
2019-07-23T17:34:50.9058187Z Get:96 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 zlib1g-dev amd64 1:1.2.8.dfsg-2ubuntu4.1 [168 kB]
2019-07-23T17:34:50.9083032Z Get:97 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 libssl-dev amd64 1.0.2g-1ubuntu4.15 [1344 kB]
2019-07-23T17:34:50.9816540Z Get:98 http://archive.ubuntu.com/ubuntu xenial/main amd64 pkg-config amd64 0.29.1-0ubuntu1 [45.0 kB]
2019-07-23T17:34:50.9823651Z Get:99 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 python2.7 amd64 2.7.12-1ubuntu0~16.04.4 [224 kB]
2019-07-23T17:34:50.9876937Z Get:100 http://archive.ubuntu.com/ubuntu xenial/universe amd64 binutils-mingw-w64-i686 amd64 2.26-3ubuntu1+6.6 [1782 kB]
2019-07-23T17:34:51.0335889Z Get:101 http://archive.ubuntu.com/ubuntu xenial/universe amd64 binutils-mingw-w64-x86-64 amd64 2.26-3ubuntu1+6.6 [2029 kB]
2019-07-23T17:34:51.3185347Z Get:102 http://archive.ubuntu.com/ubuntu xenial/universe amd64 mingw-w64-common all 4.0.4-2 [4787 kB]
2019-07-23T17:34:52.3358777Z Get:103 http://archive.ubuntu.com/ubuntu xenial/universe amd64 mingw-w64-i686-dev all 4.0.4-2 [2059 kB]
2019-07-23T17:34:52.3733614Z Get:104 http://archive.ubuntu.com/ubuntu xenial/universe amd64 gcc-mingw-w64-base amd64 5.3.1-8ubuntu3+17 [11.2 kB]
2019-07-23T17:34:52.3734659Z Get:105 http://archive.ubuntu.com/ubuntu xenial/universe amd64 gcc-mingw-w64-i686 amd64 5.3.1-8ubuntu3+17 [27.3 MB]
2019-07-23T17:34:53.3528779Z Get:106 http://archive.ubuntu.com/ubuntu xenial/universe amd64 g++-mingw-w64-i686 amd64 5.3.1-8ubuntu3+17 [19.8 MB]
2019-07-23T17:34:54.0552570Z Get:107 http://archive.ubuntu.com/ubuntu xenial/universe amd64 mingw-w64-x86-64-dev all 4.0.4-2 [3238 kB]
2019-07-23T17:34:54.1632011Z Get:108 http://archive.ubuntu.com/ubuntu xenial/universe amd64 gcc-mingw-w64-x86-64 amd64 5.3.1-8ubuntu3+17 [27.4 MB]
2019-07-23T17:34:55.2987838Z Get:109 http://archive.ubuntu.com/ubuntu xenial/universe amd64 g++-mingw-w64-x86-64 amd64 5.3.1-8ubuntu3+17 [20.4 MB]
2019-07-23T17:34:56.0319867Z Get:110 http://archive.ubuntu.com/ubuntu xenial/universe amd64 g++-mingw-w64 all 5.3.1-8ubuntu3+17 [10.7 kB]
2019-07-23T17:34:56.0320939Z Get:111 http://archive.ubuntu.com/ubuntu xenial/universe amd64 gcc-mingw-w64 all 5.3.1-8ubuntu3+17 [10.7 kB]
2019-07-23T17:34:56.0321273Z Get:112 http://archive.ubuntu.com/ubuntu xenial/universe amd64 mingw-w64 all 4.0.4-2 [9274 B]
2019-07-23T17:34:58.2779072Z debconf: delaying package configuration, since apt-utils is not installed
2019-07-23T17:34:58.3004618Z Fetched 187 MB in 9s (19.6 MB/s)
2019-07-23T17:34:58.3628720Z (Reading database ... 
2019-07-23T17:34:58.3629029Z (Reading database ... 5%
2019-07-23T17:34:58.3629183Z (Reading database ... 10%
2019-07-23T17:34:58.3629319Z (Reading database ... 15%
---
2019-07-23T17:35:23.0706926Z Unpacking pkg-config (0.29.1-0ubuntu1) ...
2019-07-23T17:35:23.1658585Z Selecting previously unselected package python2.7.
2019-07-23T17:35:23.1678050Z Preparing to unpack .../python2.7_2.7.12-1ubuntu0~16.04.4_amd64.deb ...
2019-07-23T17:35:23.1884891Z Unpacking python2.7 (2.7.12-1ubuntu0~16.04.4) ...
2019-07-23T17:35:23.2791421Z Selecting previously unselected package binutils-mingw-w64-i686.
2019-07-23T17:35:23.2812248Z Preparing to unpack .../binutils-mingw-w64-i686_2.26-3ubuntu1+6.6_amd64.deb ...
2019-07-23T17:35:23.2933022Z Unpacking binutils-mingw-w64-i686 (2.26-3ubuntu1+6.6) ...
2019-07-23T17:35:23.8500554Z Selecting previously unselected package binutils-mingw-w64-x86-64.
2019-07-23T17:35:23.8514200Z Preparing to unpack .../binutils-mingw-w64-x86-64_2.26-3ubuntu1+6.6_amd64.deb ...
2019-07-23T17:35:23.8654937Z Unpacking binutils-mingw-w64-x86-64 (2.26-3ubuntu1+6.6) ...
2019-07-23T17:35:24.4848719Z Selecting previously unselected package mingw-w64-common.
2019-07-23T17:35:24.4867973Z Preparing to unpack .../mingw-w64-common_4.0.4-2_all.deb ...
2019-07-23T17:35:24.4987113Z Unpacking mingw-w64-common (4.0.4-2) ...
2019-07-23T17:35:25.9352923Z Selecting previously unselected package mingw-w64-i686-dev.
2019-07-23T17:35:25.9380718Z Preparing to unpack .../mingw-w64-i686-dev_4.0.4-2_all.deb ...
2019-07-23T17:35:25.9510135Z Unpacking mingw-w64-i686-dev (4.0.4-2) ...
2019-07-23T17:35:27.0779318Z Selecting previously unselected package gcc-mingw-w64-base.
2019-07-23T17:35:27.0795551Z Preparing to unpack .../gcc-mingw-w64-base_5.3.1-8ubuntu3+17_amd64.deb ...
2019-07-23T17:35:27.0913647Z Unpacking gcc-mingw-w64-base (5.3.1-8ubuntu3+17) ...
2019-07-23T17:35:27.1771362Z Selecting previously unselected package gcc-mingw-w64-i686.
2019-07-23T17:35:27.1828505Z Preparing to unpack .../gcc-mingw-w64-i686_5.3.1-8ubuntu3+17_amd64.deb ...
2019-07-23T17:35:27.1949556Z Unpacking gcc-mingw-w64-i686 (5.3.1-8ubuntu3+17) ...
2019-07-23T17:35:31.6908967Z Selecting previously unselected package g++-mingw-w64-i686.
2019-07-23T17:35:31.6915069Z Preparing to unpack .../g++-mingw-w64-i686_5.3.1-8ubuntu3+17_amd64.deb ...
2019-07-23T17:35:31.6915657Z Unpacking g++-mingw-w64-i686 (5.3.1-8ubuntu3+17) ...
2019-07-23T17:35:34.7155779Z Selecting previously unselected package mingw-w64-x86-64-dev.
2019-07-23T17:35:34.7182044Z Preparing to unpack .../mingw-w64-x86-64-dev_4.0.4-2_all.deb ...
2019-07-23T17:35:34.7306294Z Unpacking mingw-w64-x86-64-dev (4.0.4-2) ...
2019-07-23T17:35:36.6306492Z Selecting previously unselected package gcc-mingw-w64-x86-64.
2019-07-23T17:35:36.6337470Z Preparing to unpack .../gcc-mingw-w64-x86-64_5.3.1-8ubuntu3+17_amd64.deb ...
2019-07-23T17:35:36.6465195Z Unpacking gcc-mingw-w64-x86-64 (5.3.1-8ubuntu3+17) ...
2019-07-23T17:35:40.7587697Z Selecting previously unselected package g++-mingw-w64-x86-64.
2019-07-23T17:35:40.7613775Z Preparing to unpack .../g++-mingw-w64-x86-64_5.3.1-8ubuntu3+17_amd64.deb ...
2019-07-23T17:35:40.7734006Z Unpacking g++-mingw-w64-x86-64 (5.3.1-8ubuntu3+17) ...
2019-07-23T17:35:44.8406262Z Selecting previously unselected package g++-mingw-w64.
2019-07-23T17:35:44.8438484Z Preparing to unpack .../g++-mingw-w64_5.3.1-8ubuntu3+17_all.deb ...
2019-07-23T17:35:44.8593722Z Unpacking g++-mingw-w64 (5.3.1-8ubuntu3+17) ...
2019-07-23T17:35:44.9646085Z Selecting previously unselected package gcc-mingw-w64.
2019-07-23T17:35:44.9674640Z Preparing to unpack .../gcc-mingw-w64_5.3.1-8ubuntu3+17_all.deb ...
2019-07-23T17:35:44.9832144Z Unpacking gcc-mingw-w64 (5.3.1-8ubuntu3+17) ...
2019-07-23T17:35:45.0597067Z Selecting previously unselected package mingw-w64.
2019-07-23T17:35:45.0661165Z Preparing to unpack .../mingw-w64_4.0.4-2_all.deb ...
2019-07-23T17:35:45.0780514Z Unpacking mingw-w64 (4.0.4-2) ...
2019-07-23T17:35:45.3439067Z Setting up libgdbm3:amd64 (1.8.3-13.1) ...
2019-07-23T17:35:45.4129795Z Setting up libffi6:amd64 (3.2.1-4) ...
2019-07-23T17:35:45.4500614Z Setting up libglib2.0-0:amd64 (2.48.2-0ubuntu4.3) ...
2019-07-23T17:35:45.4773821Z No schema files found: doing nothing.
---
2019-07-23T17:35:51.1564377Z Setting up zlib1g-dev:amd64 (1:1.2.8.dfsg-2ubuntu4.1) ...
2019-07-23T17:35:51.1947245Z Setting up libssl-dev:amd64 (1.0.2g-1ubuntu4.15) ...
2019-07-23T17:35:51.2411333Z Setting up pkg-config (0.29.1-0ubuntu1) ...
2019-07-23T17:35:51.3250806Z Setting up python2.7 (2.7.12-1ubuntu0~16.04.4) ...
2019-07-23T17:35:52.1529143Z Setting up binutils-mingw-w64-i686 (2.26-3ubuntu1+6.6) ...
2019-07-23T17:35:52.1899290Z Setting up binutils-mingw-w64-x86-64 (2.26-3ubuntu1+6.6) ...
2019-07-23T17:35:52.2268990Z Setting up mingw-w64-common (4.0.4-2) ...
2019-07-23T17:35:52.2641675Z Setting up mingw-w64-i686-dev (4.0.4-2) ...
2019-07-23T17:35:52.3025068Z Setting up gcc-mingw-w64-base (5.3.1-8ubuntu3+17) ...
2019-07-23T17:35:52.3391926Z Setting up gcc-mingw-w64-i686 (5.3.1-8ubuntu3+17) ...
2019-07-23T17:35:52.3703668Z update-alternatives: using /usr/bin/i686-w64-mingw32-gcc-posix to provide /usr/bin/i686-w64-mingw32-gcc (i686-w64-mingw32-gcc) in auto mode
2019-07-23T17:35:52.3704259Z update-alternatives: warning: skip creation of /usr/bin/i686-w64-mingw32-gcc-5 because associated file /usr/bin/i686-w64-mingw32-gcc-5-posix (of link group i686-w64-mingw32-gcc) doesn't exist
2019-07-23T17:35:52.3789693Z update-alternatives: using /usr/bin/i686-w64-mingw32-gcc-win32 to provide /usr/bin/i686-w64-mingw32-gcc (i686-w64-mingw32-gcc) in auto mode
2019-07-23T17:35:52.3790194Z update-alternatives: warning: skip creation of /usr/bin/i686-w64-mingw32-gcc-5 because associated file /usr/bin/i686-w64-mingw32-gcc-5-win32 (of link group i686-w64-mingw32-gcc) doesn't exist
2019-07-23T17:35:52.4083390Z Setting up g++-mingw-w64-i686 (5.3.1-8ubuntu3+17) ...
2019-07-23T17:35:52.4332949Z update-alternatives: using /usr/bin/i686-w64-mingw32-g++-posix to provide /usr/bin/i686-w64-mingw32-g++ (i686-w64-mingw32-g++) in auto mode
2019-07-23T17:35:52.4411421Z update-alternatives: using /usr/bin/i686-w64-mingw32-g++-win32 to provide /usr/bin/i686-w64-mingw32-g++ (i686-w64-mingw32-g++) in auto mode
2019-07-23T17:35:52.4623320Z Setting up mingw-w64-x86-64-dev (4.0.4-2) ...
2019-07-23T17:35:52.4974839Z Setting up gcc-mingw-w64-x86-64 (5.3.1-8ubuntu3+17) ...
2019-07-23T17:35:52.5305553Z update-alternatives: using /usr/bin/x86_64-w64-mingw32-gcc-posix to provide /usr/bin/x86_64-w64-mingw32-gcc (x86_64-w64-mingw32-gcc) in auto mode
2019-07-23T17:35:52.5306180Z update-alternatives: warning: skip creation of /usr/bin/x86_64-w64-mingw32-gcc-5 because associated file /usr/bin/x86_64-w64-mingw32-gcc-5-posix (of link group x86_64-w64-mingw32-gcc) doesn't exist
2019-07-23T17:35:52.5406485Z update-alternatives: using /usr/bin/x86_64-w64-mingw32-gcc-win32 to provide /usr/bin/x86_64-w64-mingw32-gcc (x86_64-w64-mingw32-gcc) in auto mode
2019-07-23T17:35:52.5407061Z update-alternatives: warning: skip creation of /usr/bin/x86_64-w64-mingw32-gcc-5 because associated file /usr/bin/x86_64-w64-mingw32-gcc-5-win32 (of link group x86_64-w64-mingw32-gcc) doesn't exist
2019-07-23T17:35:52.5618589Z Setting up g++-mingw-w64-x86-64 (5.3.1-8ubuntu3+17) ...
2019-07-23T17:35:52.5879086Z update-alternatives: using /usr/bin/x86_64-w64-mingw32-g++-posix to provide /usr/bin/x86_64-w64-mingw32-g++ (x86_64-w64-mingw32-g++) in auto mode
2019-07-23T17:35:52.5973601Z update-alternatives: using /usr/bin/x86_64-w64-mingw32-g++-win32 to provide /usr/bin/x86_64-w64-mingw32-g++ (x86_64-w64-mingw32-g++) in auto mode
2019-07-23T17:35:52.6214114Z Setting up g++-mingw-w64 (5.3.1-8ubuntu3+17) ...
2019-07-23T17:35:52.6595683Z Setting up gcc-mingw-w64 (5.3.1-8ubuntu3+17) ...
2019-07-23T17:35:52.6997908Z Setting up mingw-w64 (4.0.4-2) ...
2019-07-23T17:35:52.7822176Z Processing triggers for ca-certificates (20170717~16.04.2) ...
2019-07-23T17:35:52.7992959Z Updating certificates in /etc/ssl/certs...
2019-07-23T17:35:54.3691843Z 148 added, 0 removed; done.
2019-07-23T17:35:54.3692868Z Running hooks in /etc/ca-certificates/update.d...
---
2019-07-23T17:36:33.4691672Z  ---> 8d3146c0af0f
2019-07-23T17:36:33.4729939Z Successfully built 8d3146c0af0f
2019-07-23T17:36:33.6802190Z Successfully tagged rust-ci:latest
2019-07-23T17:36:33.7328661Z Built container sha256:8d3146c0af0ff379ba114da78e565d78ad39d734834580f5c11ab36b0c33cb1f
2019-07-23T17:36:33.7368654Z Uploading finished image to https://rust-lang-ci-sccache2.s3.amazonaws.com/docker/a4940e6914a5e1f6360ebc241d0d850aff083be5c43d268fd716ce48f63cb24e84238becb662f27f8c5d7740413ca17da24bb9cdc003ef32d7b03d2c9052b94d
2019-07-23T17:37:54.9780442Z upload failed: - to s3://rust-lang-ci-sccache2/docker/a4940e6914a5e1f6360ebc241d0d850aff083be5c43d268fd716ce48f63cb24e84238becb662f27f8c5d7740413ca17da24bb9cdc003ef32d7b03d2c9052b94d Unable to locate credentials
2019-07-23T17:37:55.9196824Z [CI_JOB_NAME=mingw-check]
2019-07-23T17:37:55.9244264Z Starting sccache server...
2019-07-23T17:37:56.0173092Z configure: processing command line
2019-07-23T17:37:56.0173478Z configure: 
---
2019-07-23T17:42:20.4206061Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2019-07-23T17:42:20.6209185Z error[E0433]: failed to resolve: use of undeclared type or module `Path`
2019-07-23T17:42:20.6217518Z   --> src/librustc_target/spec/apple_ios_base.rs:32:28
2019-07-23T17:42:20.6222657Z    |
2019-07-23T17:42:20.6228017Z 32 |         let sdkroot_path = Path::new(sdkroot);
2019-07-23T17:42:20.6233036Z    |                            ^^^^ use of undeclared type or module `Path`
2019-07-23T17:42:20.6241050Z error[E0433]: failed to resolve: use of undeclared type or module `Path`
2019-07-23T17:42:20.6246215Z   --> src/librustc_target/spec/apple_ios_base.rs:33:58
2019-07-23T17:42:20.6298235Z    |
2019-07-23T17:42:20.6298235Z    |
2019-07-23T17:42:20.6301854Z 33 |         if sdkroot_path.is_absolute() && sdkroot_path != Path::new("/") && sdkroot_path.exists() {
2019-07-23T17:42:20.6302642Z    |                                                          ^^^^ use of undeclared type or module `Path`
2019-07-23T17:42:21.6640263Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2019-07-23T17:42:22.0371399Z error: aborting due to 2 previous errors
2019-07-23T17:42:22.0376099Z 
2019-07-23T17:42:22.0382421Z For more information about this error, try `rustc --explain E0433`.
2019-07-23T17:42:22.0382421Z For more information about this error, try `rustc --explain E0433`.
2019-07-23T17:42:22.0610739Z error: Could not compile `rustc_target`.
2019-07-23T17:42:22.0611435Z 
2019-07-23T17:42:22.0612305Z To learn more, run the command again with --verbose.
2019-07-23T17:42:22.0651126Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
2019-07-23T17:42:22.0663521Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-07-23T17:42:22.0663937Z Build completed unsuccessfully in 0:04:26
2019-07-23T17:42:22.0663937Z Build completed unsuccessfully in 0:04:26
2019-07-23T17:42:23.0976054Z ##[error]Bash exited with code '1'.
2019-07-23T17:42:23.1012763Z ##[section]Starting: Checkout
2019-07-23T17:42:23.1015472Z ==============================================================================
2019-07-23T17:42:23.1015528Z Task         : Get sources
2019-07-23T17:42:23.1015576Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
