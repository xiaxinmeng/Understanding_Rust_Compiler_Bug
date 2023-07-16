plain
2019-07-14T16:23:03.7007773Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-14T16:23:03.7193190Z ##[command]git config gc.auto 0
2019-07-14T16:23:03.7278895Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-14T16:23:03.7337988Z ##[command]git config --get-all http.proxy
2019-07-14T16:23:03.7470418Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62655/merge:refs/remotes/pull/62655/merge
---
2019-07-14T16:23:37.3875334Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-14T16:23:37.3876578Z 
2019-07-14T16:23:37.3877980Z   git checkout -b <new-branch-name>
2019-07-14T16:23:37.3879445Z 
2019-07-14T16:23:37.3880150Z HEAD is now at 45fe28d0e Merge 4ad12fb8806a9366ddba46a51dc42fc3c14e8f15 into 7d41ebf768faca490addc7c616b3a9274621f0e9
2019-07-14T16:23:37.4010998Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-14T16:23:37.4013246Z ==============================================================================
2019-07-14T16:23:37.4013311Z Task         : Bash
2019-07-14T16:23:37.4013348Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-14T16:25:24.3027305Z Attempting with retry: curl -y 30 -Y 10 --connect-timeout 30 -f -L -C - -o /tmp/rustci_docker_cache https://.s3.amazonaws.com/docker/a4940e6914a5e1f6360ebc241d0d850aff083be5c43d268fd716ce48f63cb24e84238becb662f27f8c5d7740413ca17da24bb9cdc003ef32d7b03d2c9052b94d
2019-07-14T16:25:24.3091704Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-14T16:25:24.3092185Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-14T16:25:24.3092277Z 
2019-07-14T16:25:24.3093194Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-14T16:25:25.3180471Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-14T16:25:25.3181012Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-14T16:25:25.3181261Z 
2019-07-14T16:25:25.3181261Z 
2019-07-14T16:25:25.3183149Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-14T16:25:27.3259097Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-14T16:25:27.3259174Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-14T16:25:27.3259205Z 
2019-07-14T16:25:27.3259205Z 
2019-07-14T16:25:27.3294716Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-14T16:25:30.3382123Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-14T16:25:30.3382805Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-14T16:25:30.3383104Z 
2019-07-14T16:25:30.3383104Z 
2019-07-14T16:25:30.3384027Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-14T16:25:34.3460838Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-14T16:25:34.3461361Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-14T16:25:34.3461589Z 
2019-07-14T16:25:34.3461589Z 
2019-07-14T16:25:34.3502439Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-14T16:25:34.3503684Z The command has failed after 5 attempts.
2019-07-14T16:25:34.3863170Z open /tmp/rustci_docker_cache: no such file or directory
2019-07-14T16:25:34.3884193Z Attempting with retry: docker build --rm -t rust-ci -f /home/vsts/work/1/s/src/ci/docker/mingw-check/Dockerfile /home/vsts/work/1/s/src/ci/docker
2019-07-14T16:25:34.6046250Z Sending build context to Docker daemon  521.7kB
2019-07-14T16:25:34.6047008Z 
2019-07-14T16:25:34.6062837Z Step 1/6 : FROM ubuntu:16.04
---
2019-07-14T16:25:50.3991591Z Reading package lists...
2019-07-14T16:25:51.2257561Z Reading package lists...
2019-07-14T16:25:51.3663827Z Building dependency tree...
2019-07-14T16:25:51.3663996Z Reading state information...
2019-07-14T16:25:51.4639889Z The following additional packages will be installed:
2019-07-14T16:25:51.4641212Z   binutils binutils-mingw-w64-i686 binutils-mingw-w64-x86-64 bzip2 cmake-data
2019-07-14T16:25:51.4641507Z   cpp cpp-5 dpkg-dev g++-5 g++-mingw-w64 g++-mingw-w64-i686
2019-07-14T16:25:51.4641886Z   g++-mingw-w64-x86-64 gcc gcc-5 gcc-mingw-w64 gcc-mingw-w64-base
2019-07-14T16:25:51.4642360Z   gcc-mingw-w64-i686 gcc-mingw-w64-x86-64 git-man libarchive13 libasan2
2019-07-14T16:25:51.4642964Z   libasn1-8-heimdal libatomic1 libbabeltrace-ctf1 libbabeltrace1 libbz2-1.0
2019-07-14T16:25:51.4644096Z   libdpkg-perl liberror-perl libexpat1 libffi6 libgcc-5-dev libgdbm3
2019-07-14T16:25:51.4644522Z   libglib2.0-0 libgmp10 libgnutls30 libgomp1 libgssapi-krb5-2
2019-07-14T16:25:51.4644890Z   libgssapi3-heimdal libhcrypto4-heimdal libheimbase1-heimdal
2019-07-14T16:25:51.4645257Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-14T16:25:51.4645257Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-14T16:25:51.4645869Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-14T16:25:51.4667332Z   libkrb5-3 libkrb5support0 libldap-2.4-2 liblsan0 liblzo2-2 libmagic1 libmpc3
2019-07-14T16:25:51.4668154Z   libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0 libperl5.22
2019-07-14T16:25:51.4668555Z   libpython2.7-minimal libpython2.7-stdlib libpython3.5 libpython3.5-minimal
2019-07-14T16:25:51.4669113Z   libpython3.5-stdlib libquadmath0 libroken18-heimdal librtmp1 libsasl2-2
2019-07-14T16:25:51.4670101Z   libsasl2-modules-db libsqlite3-0 libssl1.0.0 libstdc++-5-dev libtasn1-6
2019-07-14T16:25:51.4670544Z   libtsan0 libubsan0 libwind0-heimdal libxml2 linux-libc-dev mime-support
2019-07-14T16:25:51.4671069Z   mingw-w64-common mingw-w64-i686-dev mingw-w64-x86-64-dev openssl patch perl
2019-07-14T16:25:51.4671513Z   perl-modules-5.22 python2.7-minimal zlib1g-dev
2019-07-14T16:25:51.4671704Z Suggested packages:
2019-07-14T16:25:51.4672232Z   binutils-doc bzip2-doc codeblocks eclipse ninja-build cpp-doc gcc-5-locales
2019-07-14T16:25:51.4672508Z   debian-keyring g++-multilib g++-5-multilib gcc-5-doc libstdc++6-5-dbg
2019-07-14T16:25:51.4672777Z   gcc-multilib manpages-dev autoconf automake libtool flex bison gcc-doc
2019-07-14T16:25:51.4673524Z   libasan2-dbg liblsan0-dbg libtsan0-dbg libubsan0-dbg libcilkrts5-dbg
2019-07-14T16:25:51.4673747Z   libmpx0-dbg libquadmath0-dbg gdb-doc gettext-base git-daemon-run
2019-07-14T16:25:51.4673747Z   libmpx0-dbg libquadmath0-dbg gdb-doc gettext-base git-daemon-run
2019-07-14T16:25:51.4674027Z   | git-daemon-sysvinit git-doc git-el git-email git-gui gitk gitweb git-arch
2019-07-14T16:25:51.4674257Z   git-cvs git-mediawiki git-svn lrzip glibc-doc gnutls-bin krb5-doc krb5-user
2019-07-14T16:25:51.4674490Z   libstdc++-5-doc make-doc wine wine64 ed diffutils-doc perl-doc
2019-07-14T16:25:51.4674768Z   libterm-readline-gnu-perl | libterm-readline-perl-perl python2.7-doc
2019-07-14T16:25:51.4674950Z   binfmt-support
2019-07-14T16:25:51.4674991Z Recommended packages:
2019-07-14T16:25:51.4675261Z   build-essential fakeroot libalgorithm-merge-perl gfortran-mingw-w64
2019-07-14T16:25:51.4675644Z   gnat-mingw-w64 libc-dbg gdbserver less rsync ssh-client manpages
2019-07-14T16:25:51.4675871Z   manpages-dev libfile-fcntllock-perl libglib2.0-data shared-mime-info
2019-07-14T16:25:51.4676139Z   xdg-user-dirs krb5-locales libsasl2-modules libssl-doc xml-core netbase
2019-07-14T16:25:51.4676180Z   rename
2019-07-14T16:25:51.4676217Z The following NEW packages will be installed:
2019-07-14T16:25:51.4676480Z   binutils binutils-mingw-w64-i686 binutils-mingw-w64-x86-64 bzip2
2019-07-14T16:25:51.4676700Z   ca-certificates cmake cmake-data cpp cpp-5 curl dpkg-dev file g++ g++-5
2019-07-14T16:25:51.4677034Z   g++-mingw-w64 g++-mingw-w64-i686 g++-mingw-w64-x86-64 gcc gcc-5
2019-07-14T16:25:51.4677344Z   gcc-mingw-w64 gcc-mingw-w64-base gcc-mingw-w64-i686 gcc-mingw-w64-x86-64 gdb
2019-07-14T16:25:51.4677569Z   git git-man libarchive13 libasan2 libasn1-8-heimdal libatomic1
2019-07-14T16:25:51.4677945Z   libbabeltrace-ctf1 libbabeltrace1 libc-dev-bin libc6-dev libcc1-0
2019-07-14T16:25:51.4678438Z   libffi6 libgcc-5-dev libgdbm3 libglib2.0-0 libgmp10 libgnutls30 libgomp1
2019-07-14T16:25:51.4678656Z   libgssapi-krb5-2 libgssapi3-heimdal libhcrypto4-heimdal libheimbase1-heimdal
2019-07-14T16:25:51.4678912Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-14T16:25:51.4679563Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-14T16:25:51.4679563Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-14T16:25:51.4679855Z   libkrb5-3 libkrb5support0 libldap-2.4-2 liblsan0 liblzo2-2 libmagic1 libmpc3
2019-07-14T16:25:51.4680185Z   libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0 libperl5.22
2019-07-14T16:25:51.4680480Z   libpython2.7-minimal libpython2.7-stdlib libpython3.5 libpython3.5-minimal
2019-07-14T16:25:51.4680755Z   libpython3.5-stdlib libquadmath0 libroken18-heimdal librtmp1 libsasl2-2
2019-07-14T16:25:51.4681074Z   libsasl2-modules-db libsqlite3-0 libssl-dev libssl1.0.0 libstdc++-5-dev
2019-07-14T16:25:51.4681358Z   libtasn1-6 libtsan0 libubsan0 libwind0-heimdal libxml2 linux-libc-dev make
2019-07-14T16:25:51.4681743Z   mime-support mingw-w64 mingw-w64-common mingw-w64-i686-dev
2019-07-14T16:25:51.4682062Z   mingw-w64-x86-64-dev openssl patch perl perl-modules-5.22 pkg-config
2019-07-14T16:25:51.4683024Z   python2.7 python2.7-minimal sudo xz-utils zlib1g-dev
2019-07-14T16:25:51.4683196Z The following packages will be upgraded:
2019-07-14T16:25:51.7726207Z 1 upgraded, 112 newly installed, 0 to remove and 4 not upgraded.
2019-07-14T16:25:51.7726923Z Need to get 187 MB of archives.
2019-07-14T16:25:51.7727435Z After this operation, 968 MB of additional disk space will be used.
2019-07-14T16:25:51.7728197Z Get:1 http://archive.ubuntu.com/ubuntu xenial/main amd64 libgdbm3 amd64 1.8.3-13.1 [16.9 kB]
---
2019-07-14T16:25:53.7391298Z Get:97 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 zlib1g-dev amd64 1:1.2.8.dfsg-2ubuntu4.1 [168 kB]
2019-07-14T16:25:53.7493044Z Get:98 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 libssl-dev amd64 1.0.2g-1ubuntu4.15 [1344 kB]
2019-07-14T16:25:53.7666036Z Get:99 http://archive.ubuntu.com/ubuntu xenial/main amd64 pkg-config amd64 0.29.1-0ubuntu1 [45.0 kB]
2019-07-14T16:25:53.7673637Z Get:100 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 python2.7 amd64 2.7.12-1ubuntu0~16.04.4 [224 kB]
2019-07-14T16:25:53.7706619Z Get:101 http://archive.ubuntu.com/ubuntu xenial/universe amd64 binutils-mingw-w64-i686 amd64 2.26-3ubuntu1+6.6 [1782 kB]
2019-07-14T16:25:53.9011766Z Get:102 http://archive.ubuntu.com/ubuntu xenial/universe amd64 binutils-mingw-w64-x86-64 amd64 2.26-3ubuntu1+6.6 [2029 kB]
2019-07-14T16:25:54.3112614Z Get:103 http://archive.ubuntu.com/ubuntu xenial/universe amd64 mingw-w64-common all 4.0.4-2 [4787 kB]
2019-07-14T16:25:54.4015308Z Get:104 http://archive.ubuntu.com/ubuntu xenial/universe amd64 mingw-w64-i686-dev all 4.0.4-2 [2059 kB]
2019-07-14T16:25:54.4392924Z Get:105 http://archive.ubuntu.com/ubuntu xenial/universe amd64 gcc-mingw-w64-base amd64 5.3.1-8ubuntu3+17 [11.2 kB]
2019-07-14T16:25:54.4393346Z Get:106 http://archive.ubuntu.com/ubuntu xenial/universe amd64 gcc-mingw-w64-i686 amd64 5.3.1-8ubuntu3+17 [27.3 MB]
2019-07-14T16:25:54.9738184Z Get:107 http://archive.ubuntu.com/ubuntu xenial/universe amd64 g++-mingw-w64-i686 amd64 5.3.1-8ubuntu3+17 [19.8 MB]
2019-07-14T16:25:55.2982889Z Get:108 http://archive.ubuntu.com/ubuntu xenial/universe amd64 mingw-w64-x86-64-dev all 4.0.4-2 [3238 kB]
2019-07-14T16:25:55.3472029Z Get:109 http://archive.ubuntu.com/ubuntu xenial/universe amd64 gcc-mingw-w64-x86-64 amd64 5.3.1-8ubuntu3+17 [27.4 MB]
2019-07-14T16:25:55.8041326Z Get:110 http://archive.ubuntu.com/ubuntu xenial/universe amd64 g++-mingw-w64-x86-64 amd64 5.3.1-8ubuntu3+17 [20.4 MB]
2019-07-14T16:25:56.1361477Z Get:111 http://archive.ubuntu.com/ubuntu xenial/universe amd64 g++-mingw-w64 all 5.3.1-8ubuntu3+17 [10.7 kB]
2019-07-14T16:25:56.1361875Z Get:112 http://archive.ubuntu.com/ubuntu xenial/universe amd64 gcc-mingw-w64 all 5.3.1-8ubuntu3+17 [10.7 kB]
2019-07-14T16:25:56.1362971Z Get:113 http://archive.ubuntu.com/ubuntu xenial/universe amd64 mingw-w64 all 4.0.4-2 [9274 B]
2019-07-14T16:25:58.6677720Z debconf: delaying package configuration, since apt-utils is not installed
2019-07-14T16:25:58.6678691Z Fetched 187 MB in 4s (41.9 MB/s)
2019-07-14T16:25:58.6678873Z (Reading database ... 
2019-07-14T16:25:58.6678959Z (Reading database ... 5%
2019-07-14T16:25:58.6679020Z (Reading database ... 10%
2019-07-14T16:25:58.6679054Z (Reading database ... 15%
---
2019-07-14T16:26:20.5718798Z Unpacking git (1:2.7.4-0ubuntu1.6) ...
2019-07-14T16:26:21.2245404Z Selecting previously unselected package libpython2.7-stdlib:amd64.
2019-07-14T16:26:21.2263284Z Preparing to unpack .../libpython2.7-stdlib_2.7.12-1ubuntu0~16.04.4_amd64.deb ...
2019-07-14T16:26:21.2378387Z Unpacking libpython2.7-stdlib:amd64 (2.7.12-1ubuntu0~16.04.4) ...
2019-07-14T16:26:22.0246650Z Selecting previously unselected package zlib1g-dev:amd64.
2019-07-14T16:26:22.0263908Z Preparing to unpack .../zlib1g-dev_1%3a1.2.8.dfsg-2ubuntu4.1_amd64.deb ...
2019-07-14T16:26:22.0376234Z Unpacking zlib1g-dev:amd64 (1:1.2.8.dfsg-2ubuntu4.1) ...
2019-07-14T16:26:22.1290057Z Preparing to unpack .../libssl-dev_1.0.2g-1ubuntu4.15_amd64.deb ...
2019-07-14T16:26:22.1405009Z Unpacking libssl-dev:amd64 (1.0.2g-1ubuntu4.15) ...
2019-07-14T16:26:22.4355509Z Selecting previously unselected package pkg-config.
2019-07-14T16:26:22.4372349Z Preparing to unpack .../pkg-config_0.29.1-0ubuntu1_amd64.deb ...
2019-07-14T16:26:22.4372349Z Preparing to unpack .../pkg-config_0.29.1-0ubuntu1_amd64.deb ...
2019-07-14T16:26:22.4485951Z Unpacking pkg-config (0.29.1-0ubuntu1) ...
2019-07-14T16:26:22.5362882Z Selecting previously unselected package python2.7.
2019-07-14T16:26:22.5384014Z Preparing to unpack .../python2.7_2.7.12-1ubuntu0~16.04.4_amd64.deb ...
2019-07-14T16:26:22.5505933Z Unpacking python2.7 (2.7.12-1ubuntu0~16.04.4) ...
2019-07-14T16:26:22.6323730Z Selecting previously unselected package binutils-mingw-w64-i686.
2019-07-14T16:26:22.6341444Z Preparing to unpack .../binutils-mingw-w64-i686_2.26-3ubuntu1+6.6_amd64.deb ...
2019-07-14T16:26:22.6451013Z Unpacking binutils-mingw-w64-i686 (2.26-3ubuntu1+6.6) ...
2019-07-14T16:26:23.1690543Z Selecting previously unselected package binutils-mingw-w64-x86-64.
2019-07-14T16:26:23.1718168Z Preparing to unpack .../binutils-mingw-w64-x86-64_2.26-3ubuntu1+6.6_amd64.deb ...
2019-07-14T16:26:23.1832967Z Unpacking binutils-mingw-w64-x86-64 (2.26-3ubuntu1+6.6) ...
2019-07-14T16:26:23.7623697Z Selecting previously unselected package mingw-w64-common.
2019-07-14T16:26:23.7637859Z Preparing to unpack .../mingw-w64-common_4.0.4-2_all.deb ...
2019-07-14T16:26:23.7757815Z Unpacking mingw-w64-common (4.0.4-2) ...
2019-07-14T16:26:25.2254999Z Selecting previously unselected package mingw-w64-i686-dev.
2019-07-14T16:26:25.2274797Z Preparing to unpack .../mingw-w64-i686-dev_4.0.4-2_all.deb ...
2019-07-14T16:26:25.2380919Z Unpacking mingw-w64-i686-dev (4.0.4-2) ...
2019-07-14T16:26:26.3670841Z Selecting previously unselected package gcc-mingw-w64-base.
2019-07-14T16:26:26.3694276Z Preparing to unpack .../gcc-mingw-w64-base_5.3.1-8ubuntu3+17_amd64.deb ...
2019-07-14T16:26:26.3870219Z Unpacking gcc-mingw-w64-base (5.3.1-8ubuntu3+17) ...
2019-07-14T16:26:26.4680011Z Selecting previously unselected package gcc-mingw-w64-i686.
2019-07-14T16:26:26.4696705Z Preparing to unpack .../gcc-mingw-w64-i686_5.3.1-8ubuntu3+17_amd64.deb ...
2019-07-14T16:26:26.4810339Z Unpacking gcc-mingw-w64-i686 (5.3.1-8ubuntu3+17) ...
2019-07-14T16:26:30.3581033Z Selecting previously unselected package g++-mingw-w64-i686.
2019-07-14T16:26:30.3605264Z Preparing to unpack .../g++-mingw-w64-i686_5.3.1-8ubuntu3+17_amd64.deb ...
2019-07-14T16:26:30.3762821Z Unpacking g++-mingw-w64-i686 (5.3.1-8ubuntu3+17) ...
2019-07-14T16:26:34.1461443Z Selecting previously unselected package mingw-w64-x86-64-dev.
2019-07-14T16:26:34.1485532Z Preparing to unpack .../mingw-w64-x86-64-dev_4.0.4-2_all.deb ...
2019-07-14T16:26:34.1597433Z Unpacking mingw-w64-x86-64-dev (4.0.4-2) ...
2019-07-14T16:26:36.1591086Z Selecting previously unselected package gcc-mingw-w64-x86-64.
2019-07-14T16:26:36.1617751Z Preparing to unpack .../gcc-mingw-w64-x86-64_5.3.1-8ubuntu3+17_amd64.deb ...
2019-07-14T16:26:36.1728352Z Unpacking gcc-mingw-w64-x86-64 (5.3.1-8ubuntu3+17) ...
2019-07-14T16:26:40.0726917Z Selecting previously unselected package g++-mingw-w64-x86-64.
2019-07-14T16:26:40.0748607Z Preparing to unpack .../g++-mingw-w64-x86-64_5.3.1-8ubuntu3+17_amd64.deb ...
2019-07-14T16:26:40.0856229Z Unpacking g++-mingw-w64-x86-64 (5.3.1-8ubuntu3+17) ...
2019-07-14T16:26:43.9415405Z Selecting previously unselected package g++-mingw-w64.
2019-07-14T16:26:43.9445211Z Preparing to unpack .../g++-mingw-w64_5.3.1-8ubuntu3+17_all.deb ...
2019-07-14T16:26:43.9599614Z Unpacking g++-mingw-w64 (5.3.1-8ubuntu3+17) ...
2019-07-14T16:26:44.6826282Z Selecting previously unselected package gcc-mingw-w64.
2019-07-14T16:26:44.6828887Z Preparing to unpack .../gcc-mingw-w64_5.3.1-8ubuntu3+17_all.deb ...
2019-07-14T16:26:44.6829428Z Unpacking gcc-mingw-w64 (5.3.1-8ubuntu3+17) ...
2019-07-14T16:26:44.6829899Z Selecting previously unselected package mingw-w64.
2019-07-14T16:26:44.6830323Z Preparing to unpack .../mingw-w64_4.0.4-2_all.deb ...
2019-07-14T16:26:44.6830730Z Unpacking mingw-w64 (4.0.4-2) ...
2019-07-14T16:26:44.6831583Z Setting up libgdbm3:amd64 (1.8.3-13.1) ...
2019-07-14T16:26:44.6832038Z Setting up libffi6:amd64 (3.2.1-4) ...
2019-07-14T16:26:44.6832469Z Setting up libglib2.0-0:amd64 (2.48.2-0ubuntu4.3) ...
2019-07-14T16:26:44.6832665Z No schema files found: doing nothing.
---
2019-07-14T16:26:50.0803074Z Setting up zlib1g-dev:amd64 (1:1.2.8.dfsg-2ubuntu4.1) ...
2019-07-14T16:26:50.1166399Z Setting up libssl-dev:amd64 (1.0.2g-1ubuntu4.15) ...
2019-07-14T16:26:50.1592716Z Setting up pkg-config (0.29.1-0ubuntu1) ...
2019-07-14T16:26:50.2342861Z Setting up python2.7 (2.7.12-1ubuntu0~16.04.4) ...
2019-07-14T16:26:50.9540995Z Setting up binutils-mingw-w64-i686 (2.26-3ubuntu1+6.6) ...
2019-07-14T16:26:50.9974380Z Setting up binutils-mingw-w64-x86-64 (2.26-3ubuntu1+6.6) ...
2019-07-14T16:26:51.0455842Z Setting up mingw-w64-common (4.0.4-2) ...
2019-07-14T16:26:51.1342837Z Setting up mingw-w64-i686-dev (4.0.4-2) ...
2019-07-14T16:26:51.1717839Z Setting up gcc-mingw-w64-base (5.3.1-8ubuntu3+17) ...
2019-07-14T16:26:51.2123312Z Setting up gcc-mingw-w64-i686 (5.3.1-8ubuntu3+17) ...
2019-07-14T16:26:51.2394265Z update-alternatives: using /usr/bin/i686-w64-mingw32-gcc-posix to provide /usr/bin/i686-w64-mingw32-gcc (i686-w64-mingw32-gcc) in auto mode
2019-07-14T16:26:51.2394887Z update-alternatives: warning: skip creation of /usr/bin/i686-w64-mingw32-gcc-5 because associated file /usr/bin/i686-w64-mingw32-gcc-5-posix (of link group i686-w64-mingw32-gcc) doesn't exist
2019-07-14T16:26:51.2501906Z update-alternatives: using /usr/bin/i686-w64-mingw32-gcc-win32 to provide /usr/bin/i686-w64-mingw32-gcc (i686-w64-mingw32-gcc) in auto mode
2019-07-14T16:26:51.2504245Z update-alternatives: warning: skip creation of /usr/bin/i686-w64-mingw32-gcc-5 because associated file /usr/bin/i686-w64-mingw32-gcc-5-win32 (of link group i686-w64-mingw32-gcc) doesn't exist
2019-07-14T16:26:51.2901588Z Setting up g++-mingw-w64-i686 (5.3.1-8ubuntu3+17) ...
2019-07-14T16:26:51.3227050Z update-alternatives: using /usr/bin/i686-w64-mingw32-g++-posix to provide /usr/bin/i686-w64-mingw32-g++ (i686-w64-mingw32-g++) in auto mode
2019-07-14T16:26:51.3329324Z update-alternatives: using /usr/bin/i686-w64-mingw32-g++-win32 to provide /usr/bin/i686-w64-mingw32-g++ (i686-w64-mingw32-g++) in auto mode
2019-07-14T16:26:51.3594171Z Setting up mingw-w64-x86-64-dev (4.0.4-2) ...
2019-07-14T16:26:51.3998726Z Setting up gcc-mingw-w64-x86-64 (5.3.1-8ubuntu3+17) ...
2019-07-14T16:26:51.4272299Z update-alternatives: using /usr/bin/x86_64-w64-mingw32-gcc-posix to provide /usr/bin/x86_64-w64-mingw32-gcc (x86_64-w64-mingw32-gcc) in auto mode
2019-07-14T16:26:51.4275866Z update-alternatives: warning: skip creation of /usr/bin/x86_64-w64-mingw32-gcc-5 because associated file /usr/bin/x86_64-w64-mingw32-gcc-5-posix (of link group x86_64-w64-mingw32-gcc) doesn't exist
2019-07-14T16:26:51.4361440Z update-alternatives: using /usr/bin/x86_64-w64-mingw32-gcc-win32 to provide /usr/bin/x86_64-w64-mingw32-gcc (x86_64-w64-mingw32-gcc) in auto mode
2019-07-14T16:26:51.4363034Z update-alternatives: warning: skip creation of /usr/bin/x86_64-w64-mingw32-gcc-5 because associated file /usr/bin/x86_64-w64-mingw32-gcc-5-win32 (of link group x86_64-w64-mingw32-gcc) doesn't exist
2019-07-14T16:26:51.4600833Z Setting up g++-mingw-w64-x86-64 (5.3.1-8ubuntu3+17) ...
2019-07-14T16:26:51.4863736Z update-alternatives: using /usr/bin/x86_64-w64-mingw32-g++-posix to provide /usr/bin/x86_64-w64-mingw32-g++ (x86_64-w64-mingw32-g++) in auto mode
2019-07-14T16:26:51.4951577Z update-alternatives: using /usr/bin/x86_64-w64-mingw32-g++-win32 to provide /usr/bin/x86_64-w64-mingw32-g++ (x86_64-w64-mingw32-g++) in auto mode
2019-07-14T16:26:51.5174156Z Setting up g++-mingw-w64 (5.3.1-8ubuntu3+17) ...
2019-07-14T16:26:51.5605359Z Setting up gcc-mingw-w64 (5.3.1-8ubuntu3+17) ...
2019-07-14T16:26:51.6073177Z Setting up mingw-w64 (4.0.4-2) ...
2019-07-14T16:26:51.6958699Z Processing triggers for ca-certificates (20170717~16.04.2) ...
2019-07-14T16:26:51.7106910Z Updating certificates in /etc/ssl/certs...
2019-07-14T16:26:53.0777072Z 148 added, 0 removed; done.
2019-07-14T16:26:53.0778508Z Running hooks in /etc/ca-certificates/update.d...
---
2019-07-14T16:27:30.3989607Z Removing intermediate container 00448ceaff53
2019-07-14T16:27:30.3990962Z  ---> 707c1ec03720
2019-07-14T16:27:30.4034525Z Successfully built 707c1ec03720
2019-07-14T16:27:30.5264548Z Successfully tagged rust-ci:latest
2019-07-14T16:27:30.6017511Z Built container sha256:707c1ec03720255934dc2d72d6d1df85cf1dba7a8472c14fa9fabfbd119df0f9
2019-07-14T16:27:30.6034399Z Uploading finished image to https://.s3.amazonaws.com/docker/a4940e6914a5e1f6360ebc241d0d850aff083be5c43d268fd716ce48f63cb24e84238becb662f27f8c5d7740413ca17da24bb9cdc003ef32d7b03d2c9052b94d
2019-07-14T16:28:52.3393694Z upload failed: - to s3:///docker/a4940e6914a5e1f6360ebc241d0d850aff083be5c43d268fd716ce48f63cb24e84238becb662f27f8c5d7740413ca17da24bb9cdc003ef32d7b03d2c9052b94d Parameter validation failed:
2019-07-14T16:28:52.3395213Z Invalid bucket name "": Bucket name must match the regex "^[a-zA-Z0-9.\-_]{1,255}$"
2019-07-14T16:28:53.2416553Z [CI_JOB_NAME=mingw-check]
2019-07-14T16:28:53.2464499Z Starting sccache server...
2019-07-14T16:28:53.2970386Z configure: processing command line
2019-07-14T16:28:53.2970785Z configure: 
---
2019-07-14T16:33:20.7184886Z     Checking syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
2019-07-14T16:34:05.3262851Z     Checking rustc_allocator v0.0.0 (/checkout/src/librustc_allocator)
2019-07-14T16:34:05.5811329Z     Checking rustc_mir v0.0.0 (/checkout/src/librustc_mir)
2019-07-14T16:34:06.7612472Z     Checking rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
2019-07-14T16:34:13.9185862Z error[E0027]: pattern does not mention field `size`
2019-07-14T16:34:13.9189057Z    --> src/librustc_mir/interpret/snapshot.rs:289:13
2019-07-14T16:34:13.9189792Z     |
2019-07-14T16:34:13.9190584Z 289 |         let Allocation { alloc, relocations, align, mutability, extra: () } = self;
2019-07-14T16:34:13.9191420Z     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing field `size`
2019-07-14T16:34:13.9199638Z error[E0308]: mismatched types
2019-07-14T16:34:13.9200419Z    --> src/librustc_mir/interpret/snapshot.rs:290:13
2019-07-14T16:34:13.9201094Z     |
2019-07-14T16:34:13.9201094Z     |
2019-07-14T16:34:13.9201811Z 290 |         let AllocationBytes { bytes, undef_mask } = alloc;
2019-07-14T16:34:13.9203007Z     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `std::option::Option`, found struct `rustc::mir::interpret::AllocationBytes`
2019-07-14T16:34:13.9203569Z     |
2019-07-14T16:34:13.9204247Z     = note: expected type `std::option::Option<rustc::mir::interpret::AllocationBytes>`
2019-07-14T16:34:13.9204825Z                found type `rustc::mir::interpret::AllocationBytes`
2019-07-14T16:34:14.1352568Z     Checking rustc_lint v0.0.0 (/checkout/src/librustc_lint)
2019-07-14T16:34:15.2558203Z error: aborting due to 2 previous errors
2019-07-14T16:34:15.2559751Z 
2019-07-14T16:34:15.2560470Z Some errors have detailed explanations: E0027, E0308.
2019-07-14T16:34:15.2560470Z Some errors have detailed explanations: E0027, E0308.
2019-07-14T16:34:15.2564726Z For more information about an error, try `rustc --explain E0027`.
2019-07-14T16:34:15.3202516Z error: Could not compile `rustc_mir`.
2019-07-14T16:34:15.3203050Z warning: build failed, waiting for other jobs to finish...
2019-07-14T16:34:15.5603752Z error: build failed
2019-07-14T16:34:15.5625217Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
2019-07-14T16:34:15.5632602Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-07-14T16:34:15.5632911Z Build completed unsuccessfully in 0:05:22
2019-07-14T16:34:15.5632911Z Build completed unsuccessfully in 0:05:22
2019-07-14T16:34:16.6832284Z ##[error]Bash exited with code '1'.
2019-07-14T16:34:16.6859888Z ##[section]Starting: Checkout
2019-07-14T16:34:16.6861519Z ==============================================================================
2019-07-14T16:34:16.6861575Z Task         : Get sources
2019-07-14T16:34:16.6861623Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
