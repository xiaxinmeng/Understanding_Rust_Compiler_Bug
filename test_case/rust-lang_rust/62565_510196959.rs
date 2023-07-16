plain
2019-07-10T19:24:03.0450470Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-10T19:24:03.0641116Z ##[command]git config gc.auto 0
2019-07-10T19:24:03.0722842Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-10T19:24:03.0788882Z ##[command]git config --get-all http.proxy
2019-07-10T19:24:03.0930088Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62565/merge:refs/remotes/pull/62565/merge
---
2019-07-10T19:24:38.0324324Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-10T19:24:38.0324377Z 
2019-07-10T19:24:38.0324597Z   git checkout -b <new-branch-name>
2019-07-10T19:24:38.0324627Z 
2019-07-10T19:24:38.0324675Z HEAD is now at 1cb6f8e82 Merge 67b23c3684fee05edf532d6f1eb1df589dc372ed into c6a9e766f90a5271c2356fbc7941e38559200ab5
2019-07-10T19:24:38.0464820Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-10T19:24:38.0552772Z ==============================================================================
2019-07-10T19:24:38.0552849Z Task         : Bash
2019-07-10T19:24:38.0552898Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-10T19:26:32.8005371Z Attempting with retry: curl -y 30 -Y 10 --connect-timeout 30 -f -L -C - -o /tmp/rustci_docker_cache https://.s3.amazonaws.com/docker/a4940e6914a5e1f6360ebc241d0d850aff083be5c43d268fd716ce48f63cb24e84238becb662f27f8c5d7740413ca17da24bb9cdc003ef32d7b03d2c9052b94d
2019-07-10T19:26:32.8069681Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-10T19:26:32.8070370Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-10T19:26:32.8070871Z 
2019-07-10T19:26:32.8071935Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-10T19:26:33.8160127Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-10T19:26:33.8160686Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-10T19:26:33.8161031Z 
2019-07-10T19:26:33.8161031Z 
2019-07-10T19:26:33.8162352Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-10T19:26:35.8228581Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-10T19:26:35.8229024Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-10T19:26:35.8229230Z 
2019-07-10T19:26:35.8229230Z 
2019-07-10T19:26:35.8271551Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-10T19:26:38.8341809Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-10T19:26:38.8342228Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-10T19:26:38.8342266Z 
2019-07-10T19:26:38.8342266Z 
2019-07-10T19:26:38.8384524Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-10T19:26:42.8483832Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-10T19:26:42.8484452Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-10T19:26:42.8493872Z 
2019-07-10T19:26:42.8493872Z 
2019-07-10T19:26:42.8526640Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-10T19:26:42.8531618Z The command has failed after 5 attempts.
2019-07-10T19:26:43.0097869Z open /tmp/rustci_docker_cache: no such file or directory
2019-07-10T19:26:43.0127479Z Attempting with retry: docker build --rm -t rust-ci -f /home/vsts/work/1/s/src/ci/docker/mingw-check/Dockerfile /home/vsts/work/1/s/src/ci/docker
2019-07-10T19:26:43.1604051Z Sending build context to Docker daemon  521.7kB
2019-07-10T19:26:43.1604793Z 
2019-07-10T19:26:43.1695524Z Step 1/6 : FROM ubuntu:16.04
---
2019-07-10T19:27:03.1004691Z Reading package lists...
2019-07-10T19:27:04.0658465Z Reading package lists...
2019-07-10T19:27:05.0498184Z Building dependency tree...
2019-07-10T19:27:05.0498712Z Reading state information...
2019-07-10T19:27:05.0499203Z The following additional packages will be installed:
2019-07-10T19:27:05.0500239Z   binutils binutils-mingw-w64-i686 binutils-mingw-w64-x86-64 bzip2 cmake-data
2019-07-10T19:27:05.0500633Z   cpp cpp-5 dpkg-dev g++-5 g++-mingw-w64 g++-mingw-w64-i686
2019-07-10T19:27:05.0501088Z   g++-mingw-w64-x86-64 gcc gcc-5 gcc-mingw-w64 gcc-mingw-w64-base
2019-07-10T19:27:05.0501355Z   gcc-mingw-w64-i686 gcc-mingw-w64-x86-64 git-man libarchive13 libasan2
2019-07-10T19:27:05.0501620Z   libasn1-8-heimdal libatomic1 libbabeltrace-ctf1 libbabeltrace1 libbz2-1.0
2019-07-10T19:27:05.0502184Z   libdpkg-perl liberror-perl libexpat1 libffi6 libgcc-5-dev libgdbm3
2019-07-10T19:27:05.0502433Z   libglib2.0-0 libgmp10 libgnutls30 libgomp1 libgssapi-krb5-2
2019-07-10T19:27:05.0502746Z   libgssapi3-heimdal libhcrypto4-heimdal libheimbase1-heimdal
2019-07-10T19:27:05.0503010Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-10T19:27:05.0503010Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-10T19:27:05.0503283Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-10T19:27:05.0503601Z   libkrb5-3 libkrb5support0 libldap-2.4-2 liblsan0 liblzo2-2 libmagic1 libmpc3
2019-07-10T19:27:05.0503852Z   libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0 libperl5.22
2019-07-10T19:27:05.0504116Z   libpython2.7-minimal libpython2.7-stdlib libpython3.5 libpython3.5-minimal
2019-07-10T19:27:05.0504428Z   libpython3.5-stdlib libquadmath0 libroken18-heimdal librtmp1 libsasl2-2
2019-07-10T19:27:05.0504689Z   libsasl2-modules-db libsqlite3-0 libssl1.0.0 libstdc++-5-dev libtasn1-6
2019-07-10T19:27:05.0504945Z   libtsan0 libubsan0 libwind0-heimdal libxml2 linux-libc-dev mime-support
2019-07-10T19:27:05.0505261Z   mingw-w64-common mingw-w64-i686-dev mingw-w64-x86-64-dev openssl patch perl
2019-07-10T19:27:05.0505509Z   perl-modules-5.22 python2.7-minimal zlib1g-dev
2019-07-10T19:27:05.0505562Z Suggested packages:
2019-07-10T19:27:05.0505884Z   binutils-doc bzip2-doc codeblocks eclipse ninja-build cpp-doc gcc-5-locales
2019-07-10T19:27:05.0506142Z   debian-keyring g++-multilib g++-5-multilib gcc-5-doc libstdc++6-5-dbg
2019-07-10T19:27:05.0506396Z   gcc-multilib manpages-dev autoconf automake libtool flex bison gcc-doc
2019-07-10T19:27:05.0506957Z   libasan2-dbg liblsan0-dbg libtsan0-dbg libubsan0-dbg libcilkrts5-dbg
2019-07-10T19:27:05.0507210Z   libmpx0-dbg libquadmath0-dbg gdb-doc gettext-base git-daemon-run
2019-07-10T19:27:05.0507210Z   libmpx0-dbg libquadmath0-dbg gdb-doc gettext-base git-daemon-run
2019-07-10T19:27:05.0507516Z   | git-daemon-sysvinit git-doc git-el git-email git-gui gitk gitweb git-arch
2019-07-10T19:27:05.0507778Z   git-cvs git-mediawiki git-svn lrzip glibc-doc gnutls-bin krb5-doc krb5-user
2019-07-10T19:27:05.0508027Z   libstdc++-5-doc make-doc wine wine64 ed diffutils-doc perl-doc
2019-07-10T19:27:05.0508338Z   libterm-readline-gnu-perl | libterm-readline-perl-perl python2.7-doc
2019-07-10T19:27:05.0508810Z   binfmt-support
2019-07-10T19:27:05.0508862Z Recommended packages:
2019-07-10T19:27:05.0509169Z   build-essential fakeroot libalgorithm-merge-perl gfortran-mingw-w64
2019-07-10T19:27:05.0509429Z   gnat-mingw-w64 libc-dbg gdbserver less rsync ssh-client manpages
2019-07-10T19:27:05.0509682Z   manpages-dev libfile-fcntllock-perl libglib2.0-data shared-mime-info
2019-07-10T19:27:05.0509982Z   xdg-user-dirs krb5-locales libsasl2-modules libssl-doc xml-core netbase
2019-07-10T19:27:05.0510033Z   rename
2019-07-10T19:27:05.0510078Z The following NEW packages will be installed:
2019-07-10T19:27:05.0510379Z   binutils binutils-mingw-w64-i686 binutils-mingw-w64-x86-64 bzip2
2019-07-10T19:27:05.0510773Z   ca-certificates cmake cmake-data cpp cpp-5 curl dpkg-dev file g++ g++-5
2019-07-10T19:27:05.0511039Z   g++-mingw-w64 g++-mingw-w64-i686 g++-mingw-w64-x86-64 gcc gcc-5
2019-07-10T19:27:05.0511473Z   gcc-mingw-w64 gcc-mingw-w64-base gcc-mingw-w64-i686 gcc-mingw-w64-x86-64 gdb
2019-07-10T19:27:05.0511782Z   git git-man libarchive13 libasan2 libasn1-8-heimdal libatomic1
2019-07-10T19:27:05.0512089Z   libbabeltrace-ctf1 libbabeltrace1 libc-dev-bin libc6-dev libcc1-0
2019-07-10T19:27:05.0512701Z   libffi6 libgcc-5-dev libgdbm3 libglib2.0-0 libgmp10 libgnutls30 libgomp1
2019-07-10T19:27:05.0512964Z   libgssapi-krb5-2 libgssapi3-heimdal libhcrypto4-heimdal libheimbase1-heimdal
2019-07-10T19:27:05.0513264Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-10T19:27:05.0513533Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-10T19:27:05.0513533Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-10T19:27:05.0513799Z   libkrb5-3 libkrb5support0 libldap-2.4-2 liblsan0 liblzo2-2 libmagic1 libmpc3
2019-07-10T19:27:05.0514091Z   libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0 libperl5.22
2019-07-10T19:27:05.0514369Z   libpython2.7-minimal libpython2.7-stdlib libpython3.5 libpython3.5-minimal
2019-07-10T19:27:05.0514633Z   libpython3.5-stdlib libquadmath0 libroken18-heimdal librtmp1 libsasl2-2
2019-07-10T19:27:05.0514944Z   libsasl2-modules-db libsqlite3-0 libssl-dev libssl1.0.0 libstdc++-5-dev
2019-07-10T19:27:05.0515211Z   libtasn1-6 libtsan0 libubsan0 libwind0-heimdal libxml2 linux-libc-dev make
2019-07-10T19:27:05.0515458Z   mime-support mingw-w64 mingw-w64-common mingw-w64-i686-dev
2019-07-10T19:27:05.0515756Z   mingw-w64-x86-64-dev openssl patch perl perl-modules-5.22 pkg-config
2019-07-10T19:27:05.0516003Z   python2.7 python2.7-minimal sudo xz-utils zlib1g-dev
2019-07-10T19:27:05.0516057Z The following packages will be upgraded:
2019-07-10T19:27:05.0516351Z 1 upgraded, 112 newly installed, 0 to remove and 4 not upgraded.
2019-07-10T19:27:05.0516399Z Need to get 187 MB of archives.
2019-07-10T19:27:05.0516448Z After this operation, 968 MB of additional disk space will be used.
2019-07-10T19:27:05.0516784Z Get:1 http://archive.ubuntu.com/ubuntu xenial/main amd64 libgdbm3 amd64 1.8.3-13.1 [16.9 kB]
---
2019-07-10T19:27:09.0699796Z Get:97 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 zlib1g-dev amd64 1:1.2.8.dfsg-2ubuntu4.1 [168 kB]
2019-07-10T19:27:09.0725453Z Get:98 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 libssl-dev amd64 1.0.2g-1ubuntu4.15 [1344 kB]
2019-07-10T19:27:09.1633929Z Get:99 http://archive.ubuntu.com/ubuntu xenial/main amd64 pkg-config amd64 0.29.1-0ubuntu1 [45.0 kB]
2019-07-10T19:27:09.1643532Z Get:100 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 python2.7 amd64 2.7.12-1ubuntu0~16.04.4 [224 kB]
2019-07-10T19:27:09.1682159Z Get:101 http://archive.ubuntu.com/ubuntu xenial/universe amd64 binutils-mingw-w64-i686 amd64 2.26-3ubuntu1+6.6 [1782 kB]
2019-07-10T19:27:09.4134640Z Get:102 http://archive.ubuntu.com/ubuntu xenial/universe amd64 binutils-mingw-w64-x86-64 amd64 2.26-3ubuntu1+6.6 [2029 kB]
2019-07-10T19:27:10.3431768Z Get:103 http://archive.ubuntu.com/ubuntu xenial/universe amd64 mingw-w64-common all 4.0.4-2 [4787 kB]
2019-07-10T19:27:10.5512638Z Get:104 http://archive.ubuntu.com/ubuntu xenial/universe amd64 mingw-w64-i686-dev all 4.0.4-2 [2059 kB]
2019-07-10T19:27:10.6283661Z Get:105 http://archive.ubuntu.com/ubuntu xenial/universe amd64 gcc-mingw-w64-base amd64 5.3.1-8ubuntu3+17 [11.2 kB]
2019-07-10T19:27:10.6289495Z Get:106 http://archive.ubuntu.com/ubuntu xenial/universe amd64 gcc-mingw-w64-i686 amd64 5.3.1-8ubuntu3+17 [27.3 MB]
2019-07-10T19:27:11.6661033Z Get:107 http://archive.ubuntu.com/ubuntu xenial/universe amd64 g++-mingw-w64-i686 amd64 5.3.1-8ubuntu3+17 [19.8 MB]
2019-07-10T19:27:12.3950249Z Get:108 http://archive.ubuntu.com/ubuntu xenial/universe amd64 mingw-w64-x86-64-dev all 4.0.4-2 [3238 kB]
2019-07-10T19:27:12.6617005Z Get:109 http://archive.ubuntu.com/ubuntu xenial/universe amd64 gcc-mingw-w64-x86-64 amd64 5.3.1-8ubuntu3+17 [27.4 MB]
2019-07-10T19:27:13.8311027Z Get:110 http://archive.ubuntu.com/ubuntu xenial/universe amd64 g++-mingw-w64-x86-64 amd64 5.3.1-8ubuntu3+17 [20.4 MB]
2019-07-10T19:27:14.5954597Z Get:111 http://archive.ubuntu.com/ubuntu xenial/universe amd64 g++-mingw-w64 all 5.3.1-8ubuntu3+17 [10.7 kB]
2019-07-10T19:27:14.5960636Z Get:112 http://archive.ubuntu.com/ubuntu xenial/universe amd64 gcc-mingw-w64 all 5.3.1-8ubuntu3+17 [10.7 kB]
2019-07-10T19:27:14.5965136Z Get:113 http://archive.ubuntu.com/ubuntu xenial/universe amd64 mingw-w64 all 4.0.4-2 [9274 B]
2019-07-10T19:27:16.8554283Z debconf: delaying package configuration, since apt-utils is not installed
2019-07-10T19:27:16.8763313Z Fetched 187 MB in 9s (18.7 MB/s)
2019-07-10T19:27:16.9316297Z (Reading database ... 
2019-07-10T19:27:16.9316826Z (Reading database ... 5%
2019-07-10T19:27:16.9316911Z (Reading database ... 10%
2019-07-10T19:27:16.9316986Z (Reading database ... 15%
---
2019-07-10T19:27:40.5256732Z Unpacking git (1:2.7.4-0ubuntu1.6) ...
2019-07-10T19:27:41.2159766Z Selecting previously unselected package libpython2.7-stdlib:amd64.
2019-07-10T19:27:41.2179587Z Preparing to unpack .../libpython2.7-stdlib_2.7.12-1ubuntu0~16.04.4_amd64.deb ...
2019-07-10T19:27:41.2308799Z Unpacking libpython2.7-stdlib:amd64 (2.7.12-1ubuntu0~16.04.4) ...
2019-07-10T19:27:41.5856289Z Selecting previously unselected package zlib1g-dev:amd64.
2019-07-10T19:27:41.5878589Z Preparing to unpack .../zlib1g-dev_1%3a1.2.8.dfsg-2ubuntu4.1_amd64.deb ...
2019-07-10T19:27:41.6000275Z Unpacking zlib1g-dev:amd64 (1:1.2.8.dfsg-2ubuntu4.1) ...
2019-07-10T19:27:41.7023431Z Preparing to unpack .../libssl-dev_1.0.2g-1ubuntu4.15_amd64.deb ...
2019-07-10T19:27:41.7190111Z Unpacking libssl-dev:amd64 (1.0.2g-1ubuntu4.15) ...
2019-07-10T19:27:42.0297678Z Selecting previously unselected package pkg-config.
2019-07-10T19:27:42.0314910Z Preparing to unpack .../pkg-config_0.29.1-0ubuntu1_amd64.deb ...
2019-07-10T19:27:42.0314910Z Preparing to unpack .../pkg-config_0.29.1-0ubuntu1_amd64.deb ...
2019-07-10T19:27:42.0464711Z Unpacking pkg-config (0.29.1-0ubuntu1) ...
2019-07-10T19:27:42.1531534Z Selecting previously unselected package python2.7.
2019-07-10T19:27:42.1553021Z Preparing to unpack .../python2.7_2.7.12-1ubuntu0~16.04.4_amd64.deb ...
2019-07-10T19:27:42.1676047Z Unpacking python2.7 (2.7.12-1ubuntu0~16.04.4) ...
2019-07-10T19:27:42.2690115Z Selecting previously unselected package binutils-mingw-w64-i686.
2019-07-10T19:27:42.2705971Z Preparing to unpack .../binutils-mingw-w64-i686_2.26-3ubuntu1+6.6_amd64.deb ...
2019-07-10T19:27:42.2839214Z Unpacking binutils-mingw-w64-i686 (2.26-3ubuntu1+6.6) ...
2019-07-10T19:27:42.8125979Z Selecting previously unselected package binutils-mingw-w64-x86-64.
2019-07-10T19:27:42.8142614Z Preparing to unpack .../binutils-mingw-w64-x86-64_2.26-3ubuntu1+6.6_amd64.deb ...
2019-07-10T19:27:42.8264335Z Unpacking binutils-mingw-w64-x86-64 (2.26-3ubuntu1+6.6) ...
2019-07-10T19:27:43.4176065Z Selecting previously unselected package mingw-w64-common.
2019-07-10T19:27:43.4196152Z Preparing to unpack .../mingw-w64-common_4.0.4-2_all.deb ...
2019-07-10T19:27:43.4320786Z Unpacking mingw-w64-common (4.0.4-2) ...
2019-07-10T19:27:44.9023913Z Selecting previously unselected package mingw-w64-i686-dev.
2019-07-10T19:27:44.9047827Z Preparing to unpack .../mingw-w64-i686-dev_4.0.4-2_all.deb ...
2019-07-10T19:27:44.9172115Z Unpacking mingw-w64-i686-dev (4.0.4-2) ...
2019-07-10T19:27:46.0128816Z Selecting previously unselected package gcc-mingw-w64-base.
2019-07-10T19:27:46.0154022Z Preparing to unpack .../gcc-mingw-w64-base_5.3.1-8ubuntu3+17_amd64.deb ...
2019-07-10T19:27:46.0316278Z Unpacking gcc-mingw-w64-base (5.3.1-8ubuntu3+17) ...
2019-07-10T19:27:46.1269494Z Selecting previously unselected package gcc-mingw-w64-i686.
2019-07-10T19:27:46.1292623Z Preparing to unpack .../gcc-mingw-w64-i686_5.3.1-8ubuntu3+17_amd64.deb ...
2019-07-10T19:27:46.1421161Z Unpacking gcc-mingw-w64-i686 (5.3.1-8ubuntu3+17) ...
2019-07-10T19:27:50.1111675Z Selecting previously unselected package g++-mingw-w64-i686.
2019-07-10T19:27:50.1133359Z Preparing to unpack .../g++-mingw-w64-i686_5.3.1-8ubuntu3+17_amd64.deb ...
2019-07-10T19:27:50.1266905Z Unpacking g++-mingw-w64-i686 (5.3.1-8ubuntu3+17) ...
2019-07-10T19:27:53.8025671Z Selecting previously unselected package mingw-w64-x86-64-dev.
2019-07-10T19:27:53.8050309Z Preparing to unpack .../mingw-w64-x86-64-dev_4.0.4-2_all.deb ...
2019-07-10T19:27:53.8238790Z Unpacking mingw-w64-x86-64-dev (4.0.4-2) ...
2019-07-10T19:27:55.7540881Z Selecting previously unselected package gcc-mingw-w64-x86-64.
2019-07-10T19:27:55.7571003Z Preparing to unpack .../gcc-mingw-w64-x86-64_5.3.1-8ubuntu3+17_amd64.deb ...
2019-07-10T19:27:55.7693949Z Unpacking gcc-mingw-w64-x86-64 (5.3.1-8ubuntu3+17) ...
2019-07-10T19:28:00.0062132Z Selecting previously unselected package g++-mingw-w64-x86-64.
2019-07-10T19:28:00.0087170Z Preparing to unpack .../g++-mingw-w64-x86-64_5.3.1-8ubuntu3+17_amd64.deb ...
2019-07-10T19:28:00.0211116Z Unpacking g++-mingw-w64-x86-64 (5.3.1-8ubuntu3+17) ...
2019-07-10T19:28:03.7724994Z Selecting previously unselected package g++-mingw-w64.
2019-07-10T19:28:03.7755118Z Preparing to unpack .../g++-mingw-w64_5.3.1-8ubuntu3+17_all.deb ...
2019-07-10T19:28:03.7913642Z Unpacking g++-mingw-w64 (5.3.1-8ubuntu3+17) ...
2019-07-10T19:28:03.8926908Z Selecting previously unselected package gcc-mingw-w64.
2019-07-10T19:28:03.8955142Z Preparing to unpack .../gcc-mingw-w64_5.3.1-8ubuntu3+17_all.deb ...
2019-07-10T19:28:03.9111711Z Unpacking gcc-mingw-w64 (5.3.1-8ubuntu3+17) ...
2019-07-10T19:28:03.9898308Z Selecting previously unselected package mingw-w64.
2019-07-10T19:28:03.9927143Z Preparing to unpack .../mingw-w64_4.0.4-2_all.deb ...
2019-07-10T19:28:04.0044360Z Unpacking mingw-w64 (4.0.4-2) ...
2019-07-10T19:28:04.2442205Z Setting up libgdbm3:amd64 (1.8.3-13.1) ...
2019-07-10T19:28:04.3140631Z Setting up libffi6:amd64 (3.2.1-4) ...
2019-07-10T19:28:04.3511717Z Setting up libglib2.0-0:amd64 (2.48.2-0ubuntu4.3) ...
2019-07-10T19:28:04.3778101Z No schema files found: doing nothing.
---
2019-07-10T19:28:09.4605178Z Setting up zlib1g-dev:amd64 (1:1.2.8.dfsg-2ubuntu4.1) ...
2019-07-10T19:28:09.5003987Z Setting up libssl-dev:amd64 (1.0.2g-1ubuntu4.15) ...
2019-07-10T19:28:09.5397057Z Setting up pkg-config (0.29.1-0ubuntu1) ...
2019-07-10T19:28:09.6202314Z Setting up python2.7 (2.7.12-1ubuntu0~16.04.4) ...
2019-07-10T19:28:10.4349228Z Setting up binutils-mingw-w64-i686 (2.26-3ubuntu1+6.6) ...
2019-07-10T19:28:10.4729493Z Setting up binutils-mingw-w64-x86-64 (2.26-3ubuntu1+6.6) ...
2019-07-10T19:28:10.5123674Z Setting up mingw-w64-common (4.0.4-2) ...
2019-07-10T19:28:10.5501618Z Setting up mingw-w64-i686-dev (4.0.4-2) ...
2019-07-10T19:28:11.0666516Z Setting up gcc-mingw-w64-base (5.3.1-8ubuntu3+17) ...
2019-07-10T19:28:11.0666813Z Setting up gcc-mingw-w64-i686 (5.3.1-8ubuntu3+17) ...
2019-07-10T19:28:11.0667476Z update-alternatives: using /usr/bin/i686-w64-mingw32-gcc-posix to provide /usr/bin/i686-w64-mingw32-gcc (i686-w64-mingw32-gcc) in auto mode
2019-07-10T19:28:11.0667888Z update-alternatives: warning: skip creation of /usr/bin/i686-w64-mingw32-gcc-5 because associated file /usr/bin/i686-w64-mingw32-gcc-5-posix (of link group i686-w64-mingw32-gcc) doesn't exist
2019-07-10T19:28:11.0668310Z update-alternatives: using /usr/bin/i686-w64-mingw32-gcc-win32 to provide /usr/bin/i686-w64-mingw32-gcc (i686-w64-mingw32-gcc) in auto mode
2019-07-10T19:28:11.0668931Z update-alternatives: warning: skip creation of /usr/bin/i686-w64-mingw32-gcc-5 because associated file /usr/bin/i686-w64-mingw32-gcc-5-win32 (of link group i686-w64-mingw32-gcc) doesn't exist
2019-07-10T19:28:11.0669251Z Setting up g++-mingw-w64-i686 (5.3.1-8ubuntu3+17) ...
2019-07-10T19:28:11.0669619Z update-alternatives: using /usr/bin/i686-w64-mingw32-g++-posix to provide /usr/bin/i686-w64-mingw32-g++ (i686-w64-mingw32-g++) in auto mode
2019-07-10T19:28:11.0669952Z update-alternatives: using /usr/bin/i686-w64-mingw32-g++-win32 to provide /usr/bin/i686-w64-mingw32-g++ (i686-w64-mingw32-g++) in auto mode
2019-07-10T19:28:11.0670192Z Setting up mingw-w64-x86-64-dev (4.0.4-2) ...
2019-07-10T19:28:11.0670454Z Setting up gcc-mingw-w64-x86-64 (5.3.1-8ubuntu3+17) ...
2019-07-10T19:28:11.0670783Z update-alternatives: using /usr/bin/x86_64-w64-mingw32-gcc-posix to provide /usr/bin/x86_64-w64-mingw32-gcc (x86_64-w64-mingw32-gcc) in auto mode
2019-07-10T19:28:11.0671179Z update-alternatives: warning: skip creation of /usr/bin/x86_64-w64-mingw32-gcc-5 because associated file /usr/bin/x86_64-w64-mingw32-gcc-5-posix (of link group x86_64-w64-mingw32-gcc) doesn't exist
2019-07-10T19:28:11.0671542Z update-alternatives: using /usr/bin/x86_64-w64-mingw32-gcc-win32 to provide /usr/bin/x86_64-w64-mingw32-gcc (x86_64-w64-mingw32-gcc) in auto mode
2019-07-10T19:28:11.0671939Z update-alternatives: warning: skip creation of /usr/bin/x86_64-w64-mingw32-gcc-5 because associated file /usr/bin/x86_64-w64-mingw32-gcc-5-win32 (of link group x86_64-w64-mingw32-gcc) doesn't exist
2019-07-10T19:28:11.0672212Z Setting up g++-mingw-w64-x86-64 (5.3.1-8ubuntu3+17) ...
2019-07-10T19:28:11.0672541Z update-alternatives: using /usr/bin/x86_64-w64-mingw32-g++-posix to provide /usr/bin/x86_64-w64-mingw32-g++ (x86_64-w64-mingw32-g++) in auto mode
2019-07-10T19:28:11.0672983Z update-alternatives: using /usr/bin/x86_64-w64-mingw32-g++-win32 to provide /usr/bin/x86_64-w64-mingw32-g++ (x86_64-w64-mingw32-g++) in auto mode
2019-07-10T19:28:11.0673259Z Setting up g++-mingw-w64 (5.3.1-8ubuntu3+17) ...
2019-07-10T19:28:11.0673499Z Setting up gcc-mingw-w64 (5.3.1-8ubuntu3+17) ...
2019-07-10T19:28:11.0673729Z Setting up mingw-w64 (4.0.4-2) ...
2019-07-10T19:28:11.0796078Z Processing triggers for ca-certificates (20170717~16.04.2) ...
2019-07-10T19:28:11.0956159Z Updating certificates in /etc/ssl/certs...
2019-07-10T19:28:12.6096376Z 148 added, 0 removed; done.
2019-07-10T19:28:12.6097219Z Running hooks in /etc/ca-certificates/update.d...
---
2019-07-10T19:28:52.0988752Z  ---> 17d50bb8bc42
2019-07-10T19:28:52.1026736Z Successfully built 17d50bb8bc42
2019-07-10T19:28:52.2813444Z Successfully tagged rust-ci:latest
2019-07-10T19:28:52.3350521Z Built container sha256:17d50bb8bc4277f2059a8e89e1233d507cc9cff45e3d9d6e363c33f70053eed5
2019-07-10T19:28:52.3369333Z Uploading finished image to https://.s3.amazonaws.com/docker/a4940e6914a5e1f6360ebc241d0d850aff083be5c43d268fd716ce48f63cb24e84238becb662f27f8c5d7740413ca17da24bb9cdc003ef32d7b03d2c9052b94d
2019-07-10T19:30:16.6776261Z upload failed: - to s3:///docker/a4940e6914a5e1f6360ebc241d0d850aff083be5c43d268fd716ce48f63cb24e84238becb662f27f8c5d7740413ca17da24bb9cdc003ef32d7b03d2c9052b94d Parameter validation failed:
2019-07-10T19:30:16.6778711Z Invalid bucket name "": Bucket name must match the regex "^[a-zA-Z0-9.\-_]{1,255}$"
2019-07-10T19:30:17.6681137Z [CI_JOB_NAME=mingw-check]
2019-07-10T19:30:17.6729786Z Starting sccache server...
2019-07-10T19:30:17.7225705Z configure: processing command line
2019-07-10T19:30:17.7226442Z configure: 
---
2019-07-10T19:30:37.0554094Z ########################################################                  77.9%
2019-07-10T19:30:37.0554185Z ######################################################################## 100.0%
2019-07-10T19:30:37.7574082Z extracting /checkout/obj/build/cache/2019-07-04/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
2019-07-10T19:30:37.8005882Z warning: /checkout/src/tools/miri/Cargo.toml: the cargo feature `default-run` is now stable and is no longer necessary to be listed in the manifest
2019-07-10T19:30:37.8358375Z error: failed to resolve patches for `https://github.com/rust-lang/cargo`
2019-07-10T19:30:37.8358638Z Caused by:
2019-07-10T19:30:37.8358638Z Caused by:
2019-07-10T19:30:37.8359131Z   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
2019-07-10T19:30:37.8367183Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml
2019-07-10T19:30:37.8367317Z Build completed unsuccessfully in 0:00:20
2019-07-10T19:30:51.4873606Z ##[error]Bash exited with code '1'.
2019-07-10T19:30:51.4917424Z ##[section]Starting: Checkout
2019-07-10T19:30:51.4919429Z ==============================================================================
2019-07-10T19:30:51.4919485Z Task         : Get sources
2019-07-10T19:30:51.4919532Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
