plain
2019-07-17T16:53:57.1472603Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-17T16:53:57.1674834Z ##[command]git config gc.auto 0
2019-07-17T16:53:57.1734618Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-17T16:53:57.1779108Z ##[command]git config --get-all http.proxy
2019-07-17T16:53:57.1901999Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62760/merge:refs/remotes/pull/62760/merge
---
2019-07-17T16:54:31.4851039Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-17T16:54:31.4851074Z 
2019-07-17T16:54:31.4851305Z   git checkout -b <new-branch-name>
2019-07-17T16:54:31.4851361Z 
2019-07-17T16:54:31.4851425Z HEAD is now at 314918eea Merge 3fae49d152cba13cd7f42e70108a5e4c16a7fb7c into e34d4ae8692f93155cbf1135ebea4c3fd37abf4e
2019-07-17T16:54:31.4975079Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-17T16:54:31.4977292Z ==============================================================================
2019-07-17T16:54:31.4977453Z Task         : Bash
2019-07-17T16:54:31.4977504Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-17T16:56:12.2694518Z Attempting with retry: curl -y 30 -Y 10 --connect-timeout 30 -f -L -C - -o /tmp/rustci_docker_cache https://.s3.amazonaws.com/docker/a4940e6914a5e1f6360ebc241d0d850aff083be5c43d268fd716ce48f63cb24e84238becb662f27f8c5d7740413ca17da24bb9cdc003ef32d7b03d2c9052b94d
2019-07-17T16:56:12.2751399Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-17T16:56:12.2751626Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-17T16:56:12.2751806Z 
2019-07-17T16:56:12.2752403Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-17T16:56:13.2813748Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-17T16:56:13.2813979Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-17T16:56:13.2814172Z 
2019-07-17T16:56:13.2814172Z 
2019-07-17T16:56:13.2866520Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-17T16:56:15.2921582Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-17T16:56:15.2921934Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-17T16:56:15.2922064Z 
2019-07-17T16:56:15.2922064Z 
2019-07-17T16:56:15.2966524Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-17T16:56:18.3028849Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-17T16:56:18.3029095Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-17T16:56:18.3030362Z 
2019-07-17T16:56:18.3030362Z 
2019-07-17T16:56:18.3071946Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-17T16:56:22.3149949Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-17T16:56:22.3150072Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-17T16:56:22.3150172Z 
2019-07-17T16:56:22.3150172Z 
2019-07-17T16:56:22.3151119Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-17T16:56:22.3157277Z The command has failed after 5 attempts.
2019-07-17T16:56:22.4331508Z open /tmp/rustci_docker_cache: no such file or directory
2019-07-17T16:56:22.4352231Z Attempting with retry: docker build --rm -t rust-ci -f /home/vsts/work/1/s/src/ci/docker/mingw-check/Dockerfile /home/vsts/work/1/s/src/ci/docker
2019-07-17T16:56:22.5768061Z Sending build context to Docker daemon  521.2kB
2019-07-17T16:56:22.5768702Z 
2019-07-17T16:56:22.5892998Z Step 1/6 : FROM ubuntu:16.04
---
2019-07-17T16:56:37.7884739Z Reading package lists...
2019-07-17T16:56:39.4392965Z Reading package lists...
2019-07-17T16:56:39.4393875Z Building dependency tree...
2019-07-17T16:56:39.4393999Z Reading state information...
2019-07-17T16:56:39.4394036Z The following additional packages will be installed:
2019-07-17T16:56:39.4394778Z   binutils binutils-mingw-w64-i686 binutils-mingw-w64-x86-64 bzip2 cmake-data
2019-07-17T16:56:39.4394979Z   cpp cpp-5 dpkg-dev g++-5 g++-mingw-w64 g++-mingw-w64-i686
2019-07-17T16:56:39.4395171Z   g++-mingw-w64-x86-64 gcc gcc-5 gcc-mingw-w64 gcc-mingw-w64-base
2019-07-17T16:56:39.4395453Z   gcc-mingw-w64-i686 gcc-mingw-w64-x86-64 git-man libarchive13 libasan2
2019-07-17T16:56:39.4395642Z   libasn1-8-heimdal libatomic1 libbabeltrace-ctf1 libbabeltrace1 libbz2-1.0
2019-07-17T16:56:39.4396051Z   libdpkg-perl liberror-perl libexpat1 libffi6 libgcc-5-dev libgdbm3
2019-07-17T16:56:39.4396242Z   libglib2.0-0 libgmp10 libgnutls30 libgomp1 libgssapi-krb5-2
2019-07-17T16:56:39.4396425Z   libgssapi3-heimdal libhcrypto4-heimdal libheimbase1-heimdal
2019-07-17T16:56:39.4396657Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-17T16:56:39.4396657Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-17T16:56:39.4396845Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-17T16:56:39.4397034Z   libkrb5-3 libkrb5support0 libldap-2.4-2 liblsan0 liblzo2-2 libmagic1 libmpc3
2019-07-17T16:56:39.4397645Z   libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0 libperl5.22
2019-07-17T16:56:39.4398167Z   libpython2.7-minimal libpython2.7-stdlib libpython3.5 libpython3.5-minimal
2019-07-17T16:56:39.4398425Z   libpython3.5-stdlib libquadmath0 libroken18-heimdal librtmp1 libsasl2-2
2019-07-17T16:56:39.4398743Z   libsasl2-modules-db libsqlite3-0 libssl1.0.0 libstdc++-5-dev libtasn1-6
2019-07-17T16:56:39.4398994Z   libtsan0 libubsan0 libwind0-heimdal libxml2 linux-libc-dev mime-support
2019-07-17T16:56:39.4399260Z   mingw-w64-common mingw-w64-i686-dev mingw-w64-x86-64-dev openssl patch perl
2019-07-17T16:56:39.4399550Z   perl-modules-5.22 python2.7-minimal zlib1g-dev
2019-07-17T16:56:39.4399602Z Suggested packages:
2019-07-17T16:56:39.4399852Z   binutils-doc bzip2-doc codeblocks eclipse ninja-build cpp-doc gcc-5-locales
2019-07-17T16:56:39.4400148Z   debian-keyring g++-multilib g++-5-multilib gcc-5-doc libstdc++6-5-dbg
2019-07-17T16:56:39.4400399Z   gcc-multilib manpages-dev autoconf automake libtool flex bison gcc-doc
2019-07-17T16:56:39.4401239Z   libasan2-dbg liblsan0-dbg libtsan0-dbg libubsan0-dbg libcilkrts5-dbg
2019-07-17T16:56:39.4401440Z   libmpx0-dbg libquadmath0-dbg gdb-doc gettext-base git-daemon-run
2019-07-17T16:56:39.4401440Z   libmpx0-dbg libquadmath0-dbg gdb-doc gettext-base git-daemon-run
2019-07-17T16:56:39.4401641Z   | git-daemon-sysvinit git-doc git-el git-email git-gui gitk gitweb git-arch
2019-07-17T16:56:39.4401886Z   git-cvs git-mediawiki git-svn lrzip glibc-doc gnutls-bin krb5-doc krb5-user
2019-07-17T16:56:39.4402093Z   libstdc++-5-doc make-doc wine wine64 ed diffutils-doc perl-doc
2019-07-17T16:56:39.4402290Z   libterm-readline-gnu-perl | libterm-readline-perl-perl python2.7-doc
2019-07-17T16:56:39.4402486Z   binfmt-support
2019-07-17T16:56:39.4402525Z Recommended packages:
2019-07-17T16:56:39.4402722Z   build-essential fakeroot libalgorithm-merge-perl gfortran-mingw-w64
2019-07-17T16:56:39.4402915Z   gnat-mingw-w64 libc-dbg gdbserver less rsync ssh-client manpages
2019-07-17T16:56:39.4403161Z   manpages-dev libfile-fcntllock-perl libglib2.0-data shared-mime-info
2019-07-17T16:56:39.4403368Z   xdg-user-dirs krb5-locales libsasl2-modules libssl-doc xml-core netbase
2019-07-17T16:56:39.4403407Z   rename
2019-07-17T16:56:39.4403530Z The following NEW packages will be installed:
2019-07-17T16:56:39.4403771Z   binutils binutils-mingw-w64-i686 binutils-mingw-w64-x86-64 bzip2
2019-07-17T16:56:39.4404036Z   ca-certificates cmake cmake-data cpp cpp-5 curl dpkg-dev file g++ g++-5
2019-07-17T16:56:39.4404668Z   g++-mingw-w64 g++-mingw-w64-i686 g++-mingw-w64-x86-64 gcc gcc-5
2019-07-17T16:56:39.4405077Z   gcc-mingw-w64 gcc-mingw-w64-base gcc-mingw-w64-i686 gcc-mingw-w64-x86-64 gdb
2019-07-17T16:56:39.4405322Z   git git-man libarchive13 libasan2 libasn1-8-heimdal libatomic1
2019-07-17T16:56:39.4405516Z   libbabeltrace-ctf1 libbabeltrace1 libc-dev-bin libc6-dev libcc1-0
2019-07-17T16:56:39.4405926Z   libffi6 libgcc-5-dev libgdbm3 libglib2.0-0 libgmp10 libgnutls30 libgomp1
2019-07-17T16:56:39.4406133Z   libgssapi-krb5-2 libgssapi3-heimdal libhcrypto4-heimdal libheimbase1-heimdal
2019-07-17T16:56:39.4406319Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-17T16:56:39.4406567Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-17T16:56:39.4406567Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-17T16:56:39.4406764Z   libkrb5-3 libkrb5support0 libldap-2.4-2 liblsan0 liblzo2-2 libmagic1 libmpc3
2019-07-17T16:56:39.4406951Z   libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0 libperl5.22
2019-07-17T16:56:39.4407444Z   libpython2.7-minimal libpython2.7-stdlib libpython3.5 libpython3.5-minimal
2019-07-17T16:56:39.4407720Z   libpython3.5-stdlib libquadmath0 libroken18-heimdal librtmp1 libsasl2-2
2019-07-17T16:56:39.4407973Z   libsasl2-modules-db libsqlite3-0 libssl-dev libssl1.0.0 libstdc++-5-dev
2019-07-17T16:56:39.4408274Z   libtasn1-6 libtsan0 libubsan0 libwind0-heimdal libxml2 linux-libc-dev make
2019-07-17T16:56:39.4408524Z   mime-support mingw-w64 mingw-w64-common mingw-w64-i686-dev
2019-07-17T16:56:39.4408914Z   mingw-w64-x86-64-dev openssl patch perl perl-modules-5.22 pkg-config
2019-07-17T16:56:39.4409201Z   python2.7 python2.7-minimal sudo xz-utils zlib1g-dev
2019-07-17T16:56:39.4409258Z The following packages will be upgraded:
2019-07-17T16:56:39.4409508Z 1 upgraded, 112 newly installed, 0 to remove and 5 not upgraded.
2019-07-17T16:56:39.4409605Z Need to get 187 MB of archives.
2019-07-17T16:56:39.4409665Z After this operation, 968 MB of additional disk space will be used.
2019-07-17T16:56:39.4409949Z Get:1 http://archive.ubuntu.com/ubuntu xenial/main amd64 libgdbm3 amd64 1.8.3-13.1 [16.9 kB]
---
2019-07-17T16:56:43.4483693Z Get:97 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 zlib1g-dev amd64 1:1.2.8.dfsg-2ubuntu4.1 [168 kB]
2019-07-17T16:56:43.4681350Z Get:98 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 libssl-dev amd64 1.0.2g-1ubuntu4.15 [1344 kB]
2019-07-17T16:56:43.4872836Z Get:99 http://archive.ubuntu.com/ubuntu xenial/main amd64 pkg-config amd64 0.29.1-0ubuntu1 [45.0 kB]
2019-07-17T16:56:43.4882420Z Get:100 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 python2.7 amd64 2.7.12-1ubuntu0~16.04.4 [224 kB]
2019-07-17T16:56:43.4929245Z Get:101 http://archive.ubuntu.com/ubuntu xenial/universe amd64 binutils-mingw-w64-i686 amd64 2.26-3ubuntu1+6.6 [1782 kB]
2019-07-17T16:56:43.8085173Z Get:102 http://archive.ubuntu.com/ubuntu xenial/universe amd64 binutils-mingw-w64-x86-64 amd64 2.26-3ubuntu1+6.6 [2029 kB]
2019-07-17T16:56:44.7547193Z Get:103 http://archive.ubuntu.com/ubuntu xenial/universe amd64 mingw-w64-common all 4.0.4-2 [4787 kB]
2019-07-17T16:56:44.9640199Z Get:104 http://archive.ubuntu.com/ubuntu xenial/universe amd64 mingw-w64-i686-dev all 4.0.4-2 [2059 kB]
2019-07-17T16:56:45.0393535Z Get:105 http://archive.ubuntu.com/ubuntu xenial/universe amd64 gcc-mingw-w64-base amd64 5.3.1-8ubuntu3+17 [11.2 kB]
2019-07-17T16:56:45.0399498Z Get:106 http://archive.ubuntu.com/ubuntu xenial/universe amd64 gcc-mingw-w64-i686 amd64 5.3.1-8ubuntu3+17 [27.3 MB]
2019-07-17T16:56:46.0868760Z Get:107 http://archive.ubuntu.com/ubuntu xenial/universe amd64 g++-mingw-w64-i686 amd64 5.3.1-8ubuntu3+17 [19.8 MB]
2019-07-17T16:56:46.8390657Z Get:108 http://archive.ubuntu.com/ubuntu xenial/universe amd64 mingw-w64-x86-64-dev all 4.0.4-2 [3238 kB]
2019-07-17T16:56:47.1021316Z Get:109 http://archive.ubuntu.com/ubuntu xenial/universe amd64 gcc-mingw-w64-x86-64 amd64 5.3.1-8ubuntu3+17 [27.4 MB]
2019-07-17T16:56:48.1422338Z Get:110 http://archive.ubuntu.com/ubuntu xenial/universe amd64 g++-mingw-w64-x86-64 amd64 5.3.1-8ubuntu3+17 [20.4 MB]
2019-07-17T16:56:48.8815787Z Get:111 http://archive.ubuntu.com/ubuntu xenial/universe amd64 g++-mingw-w64 all 5.3.1-8ubuntu3+17 [10.7 kB]
2019-07-17T16:56:48.8819925Z Get:112 http://archive.ubuntu.com/ubuntu xenial/universe amd64 gcc-mingw-w64 all 5.3.1-8ubuntu3+17 [10.7 kB]
2019-07-17T16:56:48.8828289Z Get:113 http://archive.ubuntu.com/ubuntu xenial/universe amd64 mingw-w64 all 4.0.4-2 [9274 B]
2019-07-17T16:56:51.1414909Z debconf: delaying package configuration, since apt-utils is not installed
2019-07-17T16:56:51.1639933Z Fetched 187 MB in 9s (18.9 MB/s)
2019-07-17T16:56:51.2258476Z (Reading database ... 
2019-07-17T16:56:51.2258806Z (Reading database ... 5%
2019-07-17T16:56:51.2258971Z (Reading database ... 10%
2019-07-17T16:56:51.2259062Z (Reading database ... 15%
---
2019-07-17T16:57:15.4551546Z Unpacking pkg-config (0.29.1-0ubuntu1) ...
2019-07-17T16:57:15.5537120Z Selecting previously unselected package python2.7.
2019-07-17T16:57:15.5552730Z Preparing to unpack .../python2.7_2.7.12-1ubuntu0~16.04.4_amd64.deb ...
2019-07-17T16:57:15.5677746Z Unpacking python2.7 (2.7.12-1ubuntu0~16.04.4) ...
2019-07-17T16:57:15.6583333Z Selecting previously unselected package binutils-mingw-w64-i686.
2019-07-17T16:57:15.6597776Z Preparing to unpack .../binutils-mingw-w64-i686_2.26-3ubuntu1+6.6_amd64.deb ...
2019-07-17T16:57:15.6721437Z Unpacking binutils-mingw-w64-i686 (2.26-3ubuntu1+6.6) ...
2019-07-17T16:57:16.2075044Z Selecting previously unselected package binutils-mingw-w64-x86-64.
2019-07-17T16:57:16.2094587Z Preparing to unpack .../binutils-mingw-w64-x86-64_2.26-3ubuntu1+6.6_amd64.deb ...
2019-07-17T16:57:16.2216300Z Unpacking binutils-mingw-w64-x86-64 (2.26-3ubuntu1+6.6) ...
2019-07-17T16:57:16.8061304Z Selecting previously unselected package mingw-w64-common.
2019-07-17T16:57:16.8081990Z Preparing to unpack .../mingw-w64-common_4.0.4-2_all.deb ...
2019-07-17T16:57:16.8206166Z Unpacking mingw-w64-common (4.0.4-2) ...
2019-07-17T16:57:18.2531407Z Selecting previously unselected package mingw-w64-i686-dev.
2019-07-17T16:57:18.2551247Z Preparing to unpack .../mingw-w64-i686-dev_4.0.4-2_all.deb ...
2019-07-17T16:57:18.2687487Z Unpacking mingw-w64-i686-dev (4.0.4-2) ...
2019-07-17T16:57:19.3978685Z Selecting previously unselected package gcc-mingw-w64-base.
2019-07-17T16:57:19.4001655Z Preparing to unpack .../gcc-mingw-w64-base_5.3.1-8ubuntu3+17_amd64.deb ...
2019-07-17T16:57:19.4139497Z Unpacking gcc-mingw-w64-base (5.3.1-8ubuntu3+17) ...
2019-07-17T16:57:19.4931192Z Selecting previously unselected package gcc-mingw-w64-i686.
2019-07-17T16:57:19.4950764Z Preparing to unpack .../gcc-mingw-w64-i686_5.3.1-8ubuntu3+17_amd64.deb ...
2019-07-17T16:57:19.5068473Z Unpacking gcc-mingw-w64-i686 (5.3.1-8ubuntu3+17) ...
2019-07-17T16:57:23.1450175Z Selecting previously unselected package g++-mingw-w64-i686.
2019-07-17T16:57:23.1472653Z Preparing to unpack .../g++-mingw-w64-i686_5.3.1-8ubuntu3+17_amd64.deb ...
2019-07-17T16:57:23.1635168Z Unpacking g++-mingw-w64-i686 (5.3.1-8ubuntu3+17) ...
2019-07-17T16:57:26.4303577Z Selecting previously unselected package mingw-w64-x86-64-dev.
2019-07-17T16:57:26.4329417Z Preparing to unpack .../mingw-w64-x86-64-dev_4.0.4-2_all.deb ...
2019-07-17T16:57:26.4474720Z Unpacking mingw-w64-x86-64-dev (4.0.4-2) ...
2019-07-17T16:57:28.3443912Z Selecting previously unselected package gcc-mingw-w64-x86-64.
2019-07-17T16:57:28.3477817Z Preparing to unpack .../gcc-mingw-w64-x86-64_5.3.1-8ubuntu3+17_amd64.deb ...
2019-07-17T16:57:28.3606817Z Unpacking gcc-mingw-w64-x86-64 (5.3.1-8ubuntu3+17) ...
2019-07-17T16:57:32.3761686Z Selecting previously unselected package g++-mingw-w64-x86-64.
2019-07-17T16:57:32.3788646Z Preparing to unpack .../g++-mingw-w64-x86-64_5.3.1-8ubuntu3+17_amd64.deb ...
2019-07-17T16:57:32.3908062Z Unpacking g++-mingw-w64-x86-64 (5.3.1-8ubuntu3+17) ...
2019-07-17T16:57:36.4344051Z Selecting previously unselected package g++-mingw-w64.
2019-07-17T16:57:36.4373711Z Preparing to unpack .../g++-mingw-w64_5.3.1-8ubuntu3+17_all.deb ...
2019-07-17T16:57:36.4527397Z Unpacking g++-mingw-w64 (5.3.1-8ubuntu3+17) ...
2019-07-17T16:57:36.5579065Z Selecting previously unselected package gcc-mingw-w64.
2019-07-17T16:57:36.5597819Z Preparing to unpack .../gcc-mingw-w64_5.3.1-8ubuntu3+17_all.deb ...
2019-07-17T16:57:36.5760798Z Unpacking gcc-mingw-w64 (5.3.1-8ubuntu3+17) ...
2019-07-17T16:57:36.6553011Z Selecting previously unselected package mingw-w64.
2019-07-17T16:57:36.6575297Z Preparing to unpack .../mingw-w64_4.0.4-2_all.deb ...
2019-07-17T16:57:36.6701886Z Unpacking mingw-w64 (4.0.4-2) ...
2019-07-17T16:57:36.9079230Z Setting up libgdbm3:amd64 (1.8.3-13.1) ...
2019-07-17T16:57:36.9743691Z Setting up libffi6:amd64 (3.2.1-4) ...
2019-07-17T16:57:37.0116553Z Setting up libglib2.0-0:amd64 (2.48.2-0ubuntu4.3) ...
2019-07-17T16:57:37.0397530Z No schema files found: doing nothing.
---
2019-07-17T16:57:41.9636393Z Setting up zlib1g-dev:amd64 (1:1.2.8.dfsg-2ubuntu4.1) ...
2019-07-17T16:57:42.0022844Z Setting up libssl-dev:amd64 (1.0.2g-1ubuntu4.15) ...
2019-07-17T16:57:42.0385501Z Setting up pkg-config (0.29.1-0ubuntu1) ...
2019-07-17T16:57:42.1145813Z Setting up python2.7 (2.7.12-1ubuntu0~16.04.4) ...
2019-07-17T16:57:42.8200774Z Setting up binutils-mingw-w64-i686 (2.26-3ubuntu1+6.6) ...
2019-07-17T16:57:42.8551156Z Setting up binutils-mingw-w64-x86-64 (2.26-3ubuntu1+6.6) ...
2019-07-17T16:57:42.8920923Z Setting up mingw-w64-common (4.0.4-2) ...
2019-07-17T16:57:42.9281618Z Setting up mingw-w64-i686-dev (4.0.4-2) ...
2019-07-17T16:57:42.9663790Z Setting up gcc-mingw-w64-base (5.3.1-8ubuntu3+17) ...
2019-07-17T16:57:43.0044571Z Setting up gcc-mingw-w64-i686 (5.3.1-8ubuntu3+17) ...
2019-07-17T16:57:43.0299211Z update-alternatives: using /usr/bin/i686-w64-mingw32-gcc-posix to provide /usr/bin/i686-w64-mingw32-gcc (i686-w64-mingw32-gcc) in auto mode
2019-07-17T16:57:43.0301031Z update-alternatives: warning: skip creation of /usr/bin/i686-w64-mingw32-gcc-5 because associated file /usr/bin/i686-w64-mingw32-gcc-5-posix (of link group i686-w64-mingw32-gcc) doesn't exist
2019-07-17T16:57:43.0383817Z update-alternatives: using /usr/bin/i686-w64-mingw32-gcc-win32 to provide /usr/bin/i686-w64-mingw32-gcc (i686-w64-mingw32-gcc) in auto mode
2019-07-17T16:57:43.0385103Z update-alternatives: warning: skip creation of /usr/bin/i686-w64-mingw32-gcc-5 because associated file /usr/bin/i686-w64-mingw32-gcc-5-win32 (of link group i686-w64-mingw32-gcc) doesn't exist
2019-07-17T16:57:43.0604389Z Setting up g++-mingw-w64-i686 (5.3.1-8ubuntu3+17) ...
2019-07-17T16:57:43.0908566Z update-alternatives: using /usr/bin/i686-w64-mingw32-g++-posix to provide /usr/bin/i686-w64-mingw32-g++ (i686-w64-mingw32-g++) in auto mode
2019-07-17T16:57:43.1001573Z update-alternatives: using /usr/bin/i686-w64-mingw32-g++-win32 to provide /usr/bin/i686-w64-mingw32-g++ (i686-w64-mingw32-g++) in auto mode
2019-07-17T16:57:43.1216728Z Setting up mingw-w64-x86-64-dev (4.0.4-2) ...
2019-07-17T16:57:43.1592004Z Setting up gcc-mingw-w64-x86-64 (5.3.1-8ubuntu3+17) ...
2019-07-17T16:57:43.1860256Z update-alternatives: using /usr/bin/x86_64-w64-mingw32-gcc-posix to provide /usr/bin/x86_64-w64-mingw32-gcc (x86_64-w64-mingw32-gcc) in auto mode
2019-07-17T16:57:43.1860611Z update-alternatives: warning: skip creation of /usr/bin/x86_64-w64-mingw32-gcc-5 because associated file /usr/bin/x86_64-w64-mingw32-gcc-5-posix (of link group x86_64-w64-mingw32-gcc) doesn't exist
2019-07-17T16:57:43.1945962Z update-alternatives: using /usr/bin/x86_64-w64-mingw32-gcc-win32 to provide /usr/bin/x86_64-w64-mingw32-gcc (x86_64-w64-mingw32-gcc) in auto mode
2019-07-17T16:57:43.1946707Z update-alternatives: warning: skip creation of /usr/bin/x86_64-w64-mingw32-gcc-5 because associated file /usr/bin/x86_64-w64-mingw32-gcc-5-win32 (of link group x86_64-w64-mingw32-gcc) doesn't exist
2019-07-17T16:57:43.2169114Z Setting up g++-mingw-w64-x86-64 (5.3.1-8ubuntu3+17) ...
2019-07-17T16:57:43.2429908Z update-alternatives: using /usr/bin/x86_64-w64-mingw32-g++-posix to provide /usr/bin/x86_64-w64-mingw32-g++ (x86_64-w64-mingw32-g++) in auto mode
2019-07-17T16:57:43.2515679Z update-alternatives: using /usr/bin/x86_64-w64-mingw32-g++-win32 to provide /usr/bin/x86_64-w64-mingw32-g++ (x86_64-w64-mingw32-g++) in auto mode
2019-07-17T16:57:43.2760863Z Setting up g++-mingw-w64 (5.3.1-8ubuntu3+17) ...
2019-07-17T16:57:43.3157606Z Setting up gcc-mingw-w64 (5.3.1-8ubuntu3+17) ...
2019-07-17T16:57:43.3557933Z Setting up mingw-w64 (4.0.4-2) ...
2019-07-17T16:57:43.4349095Z Processing triggers for ca-certificates (20170717~16.04.2) ...
2019-07-17T16:57:43.4518517Z Updating certificates in /etc/ssl/certs...
2019-07-17T16:57:44.8181112Z 148 added, 0 removed; done.
2019-07-17T16:57:44.8181991Z Running hooks in /etc/ca-certificates/update.d...
---
2019-07-17T16:58:22.8390931Z  ---> df7d2765611a
2019-07-17T16:58:22.8427426Z Successfully built df7d2765611a
2019-07-17T16:58:23.0039456Z Successfully tagged rust-ci:latest
2019-07-17T16:58:23.0565457Z Built container sha256:df7d2765611a80de6f7c63dbd992cd7e4a0daf65d9c27badf38324b32b2bf3bb
2019-07-17T16:58:23.0578131Z Uploading finished image to https://.s3.amazonaws.com/docker/a4940e6914a5e1f6360ebc241d0d850aff083be5c43d268fd716ce48f63cb24e84238becb662f27f8c5d7740413ca17da24bb9cdc003ef32d7b03d2c9052b94d
2019-07-17T16:59:41.1486118Z upload failed: - to s3:///docker/a4940e6914a5e1f6360ebc241d0d850aff083be5c43d268fd716ce48f63cb24e84238becb662f27f8c5d7740413ca17da24bb9cdc003ef32d7b03d2c9052b94d Parameter validation failed:
2019-07-17T16:59:41.1488040Z Invalid bucket name "": Bucket name must match the regex "^[a-zA-Z0-9.\-_]{1,255}$"
2019-07-17T16:59:42.1230756Z [CI_JOB_NAME=mingw-check]
2019-07-17T16:59:42.1265769Z Starting sccache server...
2019-07-17T16:59:42.1735778Z configure: processing command line
2019-07-17T16:59:42.1737349Z configure: 
---
2019-07-17T17:03:51.7883044Z     Checking syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
2019-07-17T17:04:33.8387508Z     Checking rustc_allocator v0.0.0 (/checkout/src/librustc_allocator)
2019-07-17T17:04:34.0728425Z     Checking rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
2019-07-17T17:04:36.4105979Z     Checking rustc_mir v0.0.0 (/checkout/src/librustc_mir)
2019-07-17T17:04:37.9525296Z error: lifetime parameter `'mir` never used
2019-07-17T17:04:37.9532803Z    --> src/librustc_mir/const_eval.rs:530:33
2019-07-17T17:04:37.9538909Z     |
2019-07-17T17:04:37.9545766Z 530 | pub fn undefined_behavior_error<'mir>() -> String {
2019-07-17T17:04:37.9552345Z     |                                -^^^^- help: elide the unused lifetime
2019-07-17T17:04:37.9612366Z note: lint level defined here
2019-07-17T17:04:37.9612651Z    --> src/librustc_mir/lib.rs:30:9
2019-07-17T17:04:37.9612950Z     |
2019-07-17T17:04:37.9612950Z     |
2019-07-17T17:04:37.9613253Z 30  | #![deny(unused_lifetimes)]
2019-07-17T17:04:37.9613738Z 
2019-07-17T17:04:38.0809382Z error: aborting due to previous error
2019-07-17T17:04:38.0813965Z 
2019-07-17T17:04:38.1248317Z error: Could not compile `rustc_mir`.
2019-07-17T17:04:38.1248317Z error: Could not compile `rustc_mir`.
2019-07-17T17:04:38.1249317Z warning: build failed, waiting for other jobs to finish...
2019-07-17T17:04:40.7462946Z error: build failed
2019-07-17T17:04:40.7487658Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
2019-07-17T17:04:40.7500696Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-07-17T17:04:40.7500769Z Build completed unsuccessfully in 0:04:58
2019-07-17T17:04:40.7500769Z Build completed unsuccessfully in 0:04:58
2019-07-17T17:04:41.8137958Z ##[error]Bash exited with code '1'.
2019-07-17T17:04:41.8167667Z ##[section]Starting: Checkout
2019-07-17T17:04:41.8169059Z ==============================================================================
2019-07-17T17:04:41.8169102Z Task         : Get sources
2019-07-17T17:04:41.8169137Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
