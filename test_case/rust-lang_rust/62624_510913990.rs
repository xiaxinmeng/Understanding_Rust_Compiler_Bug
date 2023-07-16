plain
2019-07-12T14:36:12.8526461Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-12T14:36:12.8723759Z ##[command]git config gc.auto 0
2019-07-12T14:36:12.8785447Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-12T14:36:12.8840132Z ##[command]git config --get-all http.proxy
2019-07-12T14:36:12.8969656Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62624/merge:refs/remotes/pull/62624/merge
---
2019-07-12T14:36:48.1048936Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-12T14:36:48.1056044Z 
2019-07-12T14:36:48.1056857Z   git checkout -b <new-branch-name>
2019-07-12T14:36:48.1057218Z 
2019-07-12T14:36:48.1057360Z HEAD is now at c1db21a14 Merge a2a36cc6bbec9063710da394a3beda5b8fe722ca into cd1381e91ff4889616eb0c87bf3c321ea2697d42
2019-07-12T14:36:48.1195932Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-12T14:36:48.1200115Z ==============================================================================
2019-07-12T14:36:48.1200172Z Task         : Bash
2019-07-12T14:36:48.1200217Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-12T14:36:48.6996155Z 320 ./src/librustc/mir
2019-07-12T14:36:48.6996422Z sort: write failed: 'standard output': Broken pipe
2019-07-12T14:36:48.6996483Z sort: write error
2019-07-12T14:36:48.7004682Z ##[section]Finishing: Show disk usage
2019-07-12T14:36:48.7052471Z ##[section]Starting: Disable git automatic line ending conversion (on C:/)
2019-07-12T14:36:48.7055628Z Task         : Bash
2019-07-12T14:36:48.7055684Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-12T14:36:48.7055733Z Version      : 3.151.2
2019-07-12T14:36:48.7055779Z Author       : Microsoft Corporation
2019-07-12T14:36:48.7055779Z Author       : Microsoft Corporation
2019-07-12T14:36:48.7055837Z Help         : https://docs.microsoft.com/azure/devops/pipelines/tasks/utility/bash
2019-07-12T14:36:48.7055893Z ==============================================================================
2019-07-12T14:36:48.8392341Z Generating script.
2019-07-12T14:36:48.8410133Z Script contents:
2019-07-12T14:36:48.8410632Z git config --replace-all --global core.autocrlf false
2019-07-12T14:36:48.8433347Z ========================== Starting Command Output ===========================
2019-07-12T14:36:48.8456206Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/8b97f87b-4fa4-4134-88a0-02d4ca645f7a.sh
2019-07-12T14:36:48.8602015Z ##[section]Finishing: Disable git automatic line ending conversion (on C:/)
2019-07-12T14:36:48.8643357Z ==============================================================================
2019-07-12T14:36:48.8643413Z Task         : Bash
2019-07-12T14:36:48.8643498Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-12T14:36:48.8643542Z Version      : 3.151.2
---
2019-07-12T14:38:41.9153388Z Attempting with retry: curl -y 30 -Y 10 --connect-timeout 30 -f -L -C - -o /tmp/rustci_docker_cache https://.s3.amazonaws.com/docker/a4940e6914a5e1f6360ebc241d0d850aff083be5c43d268fd716ce48f63cb24e84238becb662f27f8c5d7740413ca17da24bb9cdc003ef32d7b03d2c9052b94d
2019-07-12T14:38:41.9201971Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-12T14:38:41.9202399Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-12T14:38:41.9202612Z 
2019-07-12T14:38:41.9244447Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-12T14:38:42.9315051Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-12T14:38:42.9315372Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-12T14:38:42.9315504Z 
2019-07-12T14:38:42.9315504Z 
2019-07-12T14:38:42.9359665Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-12T14:38:44.9427512Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-12T14:38:44.9427595Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-12T14:38:44.9427657Z 
2019-07-12T14:38:44.9427657Z 
2019-07-12T14:38:44.9471107Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-12T14:38:47.9543503Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-12T14:38:47.9543632Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-12T14:38:47.9543672Z 
2019-07-12T14:38:47.9543672Z 
2019-07-12T14:38:47.9586388Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-12T14:38:51.9666649Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-12T14:38:51.9666863Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-12T14:38:51.9666895Z 
2019-07-12T14:38:51.9666895Z 
2019-07-12T14:38:51.9700614Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-12T14:38:51.9704847Z The command has failed after 5 attempts.
2019-07-12T14:38:52.0650890Z open /tmp/rustci_docker_cache: no such file or directory
2019-07-12T14:38:52.0692271Z Attempting with retry: docker build --rm -t rust-ci -f /home/vsts/work/1/s/src/ci/docker/mingw-check/Dockerfile /home/vsts/work/1/s/src/ci/docker
2019-07-12T14:38:52.1940844Z Sending build context to Docker daemon  521.7kB
2019-07-12T14:38:52.1941479Z 
2019-07-12T14:38:52.2162551Z Step 1/6 : FROM ubuntu:16.04
---
2019-07-12T14:39:08.9594298Z Reading package lists...
2019-07-12T14:39:09.8716744Z Reading package lists...
2019-07-12T14:39:10.0325908Z Building dependency tree...
2019-07-12T14:39:10.0326010Z Reading state information...
2019-07-12T14:39:10.1372080Z The following additional packages will be installed:
2019-07-12T14:39:10.1372909Z   binutils binutils-mingw-w64-i686 binutils-mingw-w64-x86-64 bzip2 cmake-data
2019-07-12T14:39:10.1373260Z   cpp cpp-5 dpkg-dev g++-5 g++-mingw-w64 g++-mingw-w64-i686
2019-07-12T14:39:10.1373533Z   g++-mingw-w64-x86-64 gcc gcc-5 gcc-mingw-w64 gcc-mingw-w64-base
2019-07-12T14:39:10.1374584Z   gcc-mingw-w64-i686 gcc-mingw-w64-x86-64 git-man libarchive13 libasan2
2019-07-12T14:39:10.1374833Z   libasn1-8-heimdal libatomic1 libbabeltrace-ctf1 libbabeltrace1 libbz2-1.0
2019-07-12T14:39:10.1375237Z   libdpkg-perl liberror-perl libexpat1 libffi6 libgcc-5-dev libgdbm3
2019-07-12T14:39:10.1375476Z   libglib2.0-0 libgmp10 libgnutls30 libgomp1 libgssapi-krb5-2
2019-07-12T14:39:10.1375678Z   libgssapi3-heimdal libhcrypto4-heimdal libheimbase1-heimdal
2019-07-12T14:39:10.1375877Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-12T14:39:10.1375877Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-12T14:39:10.1376078Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-12T14:39:10.1376325Z   libkrb5-3 libkrb5support0 libldap-2.4-2 liblsan0 liblzo2-2 libmagic1 libmpc3
2019-07-12T14:39:10.1376515Z   libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0 libperl5.22
2019-07-12T14:39:10.1376726Z   libpython2.7-minimal libpython2.7-stdlib libpython3.5 libpython3.5-minimal
2019-07-12T14:39:10.1376965Z   libpython3.5-stdlib libquadmath0 libroken18-heimdal librtmp1 libsasl2-2
2019-07-12T14:39:10.1377164Z   libsasl2-modules-db libsqlite3-0 libssl1.0.0 libstdc++-5-dev libtasn1-6
2019-07-12T14:39:10.1377360Z   libtsan0 libubsan0 libwind0-heimdal libxml2 linux-libc-dev mime-support
2019-07-12T14:39:10.1377613Z   mingw-w64-common mingw-w64-i686-dev mingw-w64-x86-64-dev openssl patch perl
2019-07-12T14:39:10.1377800Z   perl-modules-5.22 python2.7-minimal zlib1g-dev
2019-07-12T14:39:10.1397600Z Suggested packages:
2019-07-12T14:39:10.1397987Z   binutils-doc bzip2-doc codeblocks eclipse ninja-build cpp-doc gcc-5-locales
2019-07-12T14:39:10.1398192Z   debian-keyring g++-multilib g++-5-multilib gcc-5-doc libstdc++6-5-dbg
2019-07-12T14:39:10.1398424Z   gcc-multilib manpages-dev autoconf automake libtool flex bison gcc-doc
2019-07-12T14:39:10.1398831Z   libasan2-dbg liblsan0-dbg libtsan0-dbg libubsan0-dbg libcilkrts5-dbg
2019-07-12T14:39:10.1399055Z   libmpx0-dbg libquadmath0-dbg gdb-doc gettext-base git-daemon-run
2019-07-12T14:39:10.1399055Z   libmpx0-dbg libquadmath0-dbg gdb-doc gettext-base git-daemon-run
2019-07-12T14:39:10.1399256Z   | git-daemon-sysvinit git-doc git-el git-email git-gui gitk gitweb git-arch
2019-07-12T14:39:10.1399449Z   git-cvs git-mediawiki git-svn lrzip glibc-doc gnutls-bin krb5-doc krb5-user
2019-07-12T14:39:10.1399855Z   libstdc++-5-doc make-doc wine wine64 ed diffutils-doc perl-doc
2019-07-12T14:39:10.1400379Z   libterm-readline-gnu-perl | libterm-readline-perl-perl python2.7-doc
2019-07-12T14:39:10.1400577Z   binfmt-support
2019-07-12T14:39:10.1400667Z Recommended packages:
2019-07-12T14:39:10.1400917Z   build-essential fakeroot libalgorithm-merge-perl gfortran-mingw-w64
2019-07-12T14:39:10.1401359Z   gnat-mingw-w64 libc-dbg gdbserver less rsync ssh-client manpages
2019-07-12T14:39:10.1402034Z   manpages-dev libfile-fcntllock-perl libglib2.0-data shared-mime-info
2019-07-12T14:39:10.1402347Z   xdg-user-dirs krb5-locales libsasl2-modules libssl-doc xml-core netbase
2019-07-12T14:39:10.1402403Z   rename
2019-07-12T14:39:10.1402453Z The following NEW packages will be installed:
2019-07-12T14:39:10.1402786Z   binutils binutils-mingw-w64-i686 binutils-mingw-w64-x86-64 bzip2
2019-07-12T14:39:10.1403063Z   ca-certificates cmake cmake-data cpp cpp-5 curl dpkg-dev file g++ g++-5
2019-07-12T14:39:10.1403679Z   g++-mingw-w64 g++-mingw-w64-i686 g++-mingw-w64-x86-64 gcc gcc-5
2019-07-12T14:39:10.1404008Z   gcc-mingw-w64 gcc-mingw-w64-base gcc-mingw-w64-i686 gcc-mingw-w64-x86-64 gdb
2019-07-12T14:39:10.1404212Z   git git-man libarchive13 libasan2 libasn1-8-heimdal libatomic1
2019-07-12T14:39:10.1404409Z   libbabeltrace-ctf1 libbabeltrace1 libc-dev-bin libc6-dev libcc1-0
2019-07-12T14:39:10.1404870Z   libffi6 libgcc-5-dev libgdbm3 libglib2.0-0 libgmp10 libgnutls30 libgomp1
2019-07-12T14:39:10.1405357Z   libgssapi-krb5-2 libgssapi3-heimdal libhcrypto4-heimdal libheimbase1-heimdal
2019-07-12T14:39:10.1405606Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-12T14:39:10.1405811Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-12T14:39:10.1405811Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-12T14:39:10.1406015Z   libkrb5-3 libkrb5support0 libldap-2.4-2 liblsan0 liblzo2-2 libmagic1 libmpc3
2019-07-12T14:39:10.1406261Z   libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0 libperl5.22
2019-07-12T14:39:10.1406468Z   libpython2.7-minimal libpython2.7-stdlib libpython3.5 libpython3.5-minimal
2019-07-12T14:39:10.1406669Z   libpython3.5-stdlib libquadmath0 libroken18-heimdal librtmp1 libsasl2-2
2019-07-12T14:39:10.1406910Z   libsasl2-modules-db libsqlite3-0 libssl-dev libssl1.0.0 libstdc++-5-dev
2019-07-12T14:39:10.1407111Z   libtasn1-6 libtsan0 libubsan0 libwind0-heimdal libxml2 linux-libc-dev make
2019-07-12T14:39:10.1407303Z   mime-support mingw-w64 mingw-w64-common mingw-w64-i686-dev
2019-07-12T14:39:10.1407555Z   mingw-w64-x86-64-dev openssl patch perl perl-modules-5.22 pkg-config
2019-07-12T14:39:10.1407741Z   python2.7 python2.7-minimal sudo xz-utils zlib1g-dev
2019-07-12T14:39:10.1407784Z The following packages will be upgraded:
2019-07-12T14:39:10.8560198Z 1 upgraded, 112 newly installed, 0 to remove and 4 not upgraded.
2019-07-12T14:39:10.8561459Z Need to get 187 MB of archives.
2019-07-12T14:39:10.8561789Z After this operation, 968 MB of additional disk space will be used.
2019-07-12T14:39:10.8562584Z Get:1 http://archive.ubuntu.com/ubuntu xenial/main amd64 libgdbm3 amd64 1.8.3-13.1 [16.9 kB]
---
2019-07-12T14:39:14.9527331Z Get:97 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 zlib1g-dev amd64 1:1.2.8.dfsg-2ubuntu4.1 [168 kB]
2019-07-12T14:39:14.9553791Z Get:98 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 libssl-dev amd64 1.0.2g-1ubuntu4.15 [1344 kB]
2019-07-12T14:39:15.0251286Z Get:99 http://archive.ubuntu.com/ubuntu xenial/main amd64 pkg-config amd64 0.29.1-0ubuntu1 [45.0 kB]
2019-07-12T14:39:15.0264209Z Get:100 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 python2.7 amd64 2.7.12-1ubuntu0~16.04.4 [224 kB]
2019-07-12T14:39:15.0339748Z Get:101 http://archive.ubuntu.com/ubuntu xenial/universe amd64 binutils-mingw-w64-i686 amd64 2.26-3ubuntu1+6.6 [1782 kB]
2019-07-12T14:39:15.3048757Z Get:102 http://archive.ubuntu.com/ubuntu xenial/universe amd64 binutils-mingw-w64-x86-64 amd64 2.26-3ubuntu1+6.6 [2029 kB]
2019-07-12T14:39:16.2246080Z Get:103 http://archive.ubuntu.com/ubuntu xenial/universe amd64 mingw-w64-common all 4.0.4-2 [4787 kB]
2019-07-12T14:39:16.4326593Z Get:104 http://archive.ubuntu.com/ubuntu xenial/universe amd64 mingw-w64-i686-dev all 4.0.4-2 [2059 kB]
2019-07-12T14:39:16.5086435Z Get:105 http://archive.ubuntu.com/ubuntu xenial/universe amd64 gcc-mingw-w64-base amd64 5.3.1-8ubuntu3+17 [11.2 kB]
2019-07-12T14:39:16.5093389Z Get:106 http://archive.ubuntu.com/ubuntu xenial/universe amd64 gcc-mingw-w64-i686 amd64 5.3.1-8ubuntu3+17 [27.3 MB]
2019-07-12T14:39:17.9011125Z Get:107 http://archive.ubuntu.com/ubuntu xenial/universe amd64 g++-mingw-w64-i686 amd64 5.3.1-8ubuntu3+17 [19.8 MB]
2019-07-12T14:39:18.8509186Z Get:108 http://archive.ubuntu.com/ubuntu xenial/universe amd64 mingw-w64-x86-64-dev all 4.0.4-2 [3238 kB]
2019-07-12T14:39:18.8927443Z Get:109 http://archive.ubuntu.com/ubuntu xenial/universe amd64 gcc-mingw-w64-x86-64 amd64 5.3.1-8ubuntu3+17 [27.4 MB]
2019-07-12T14:39:20.0222325Z Get:110 http://archive.ubuntu.com/ubuntu xenial/universe amd64 g++-mingw-w64-x86-64 amd64 5.3.1-8ubuntu3+17 [20.4 MB]
2019-07-12T14:39:20.8411905Z Get:111 http://archive.ubuntu.com/ubuntu xenial/universe amd64 g++-mingw-w64 all 5.3.1-8ubuntu3+17 [10.7 kB]
2019-07-12T14:39:20.8416135Z Get:112 http://archive.ubuntu.com/ubuntu xenial/universe amd64 gcc-mingw-w64 all 5.3.1-8ubuntu3+17 [10.7 kB]
2019-07-12T14:39:20.8421373Z Get:113 http://archive.ubuntu.com/ubuntu xenial/universe amd64 mingw-w64 all 4.0.4-2 [9274 B]
2019-07-12T14:39:23.0937178Z debconf: delaying package configuration, since apt-utils is not installed
2019-07-12T14:39:23.1159732Z Fetched 187 MB in 10s (17.9 MB/s)
2019-07-12T14:39:23.1818350Z (Reading database ... 
2019-07-12T14:39:23.1818413Z (Reading database ... 5%
2019-07-12T14:39:23.1818485Z (Reading database ... 10%
2019-07-12T14:39:23.1818525Z (Reading database ... 15%
---
2019-07-12T14:39:48.3102488Z Unpacking git (1:2.7.4-0ubuntu1.6) ...
2019-07-12T14:39:49.0083981Z Selecting previously unselected package libpython2.7-stdlib:amd64.
2019-07-12T14:39:49.0099291Z Preparing to unpack .../libpython2.7-stdlib_2.7.12-1ubuntu0~16.04.4_amd64.deb ...
2019-07-12T14:39:49.0247938Z Unpacking libpython2.7-stdlib:amd64 (2.7.12-1ubuntu0~16.04.4) ...
2019-07-12T14:39:49.3584289Z Selecting previously unselected package zlib1g-dev:amd64.
2019-07-12T14:39:49.3602405Z Preparing to unpack .../zlib1g-dev_1%3a1.2.8.dfsg-2ubuntu4.1_amd64.deb ...
2019-07-12T14:39:49.3743755Z Unpacking zlib1g-dev:amd64 (1:1.2.8.dfsg-2ubuntu4.1) ...
2019-07-12T14:39:49.4783867Z Preparing to unpack .../libssl-dev_1.0.2g-1ubuntu4.15_amd64.deb ...
2019-07-12T14:39:49.4947694Z Unpacking libssl-dev:amd64 (1.0.2g-1ubuntu4.15) ...
2019-07-12T14:39:49.8237547Z Selecting previously unselected package pkg-config.
2019-07-12T14:39:49.8253841Z Preparing to unpack .../pkg-config_0.29.1-0ubuntu1_amd64.deb ...
2019-07-12T14:39:49.8253841Z Preparing to unpack .../pkg-config_0.29.1-0ubuntu1_amd64.deb ...
2019-07-12T14:39:49.8385353Z Unpacking pkg-config (0.29.1-0ubuntu1) ...
2019-07-12T14:39:49.9473312Z Selecting previously unselected package python2.7.
2019-07-12T14:39:49.9488333Z Preparing to unpack .../python2.7_2.7.12-1ubuntu0~16.04.4_amd64.deb ...
2019-07-12T14:39:49.9623957Z Unpacking python2.7 (2.7.12-1ubuntu0~16.04.4) ...
2019-07-12T14:39:50.0584116Z Selecting previously unselected package binutils-mingw-w64-i686.
2019-07-12T14:39:50.0598755Z Preparing to unpack .../binutils-mingw-w64-i686_2.26-3ubuntu1+6.6_amd64.deb ...
2019-07-12T14:39:50.0743654Z Unpacking binutils-mingw-w64-i686 (2.26-3ubuntu1+6.6) ...
2019-07-12T14:39:50.6038110Z Selecting previously unselected package binutils-mingw-w64-x86-64.
2019-07-12T14:39:50.6057772Z Preparing to unpack .../binutils-mingw-w64-x86-64_2.26-3ubuntu1+6.6_amd64.deb ...
2019-07-12T14:39:50.6179598Z Unpacking binutils-mingw-w64-x86-64 (2.26-3ubuntu1+6.6) ...
2019-07-12T14:39:51.1972328Z Selecting previously unselected package mingw-w64-common.
2019-07-12T14:39:51.1990040Z Preparing to unpack .../mingw-w64-common_4.0.4-2_all.deb ...
2019-07-12T14:39:51.2114709Z Unpacking mingw-w64-common (4.0.4-2) ...
2019-07-12T14:39:52.6452916Z Selecting previously unselected package mingw-w64-i686-dev.
2019-07-12T14:39:52.6479230Z Preparing to unpack .../mingw-w64-i686-dev_4.0.4-2_all.deb ...
2019-07-12T14:39:52.6656771Z Unpacking mingw-w64-i686-dev (4.0.4-2) ...
2019-07-12T14:39:53.8074977Z Selecting previously unselected package gcc-mingw-w64-base.
2019-07-12T14:39:53.8100810Z Preparing to unpack .../gcc-mingw-w64-base_5.3.1-8ubuntu3+17_amd64.deb ...
2019-07-12T14:39:53.8223639Z Unpacking gcc-mingw-w64-base (5.3.1-8ubuntu3+17) ...
2019-07-12T14:39:53.9160632Z Selecting previously unselected package gcc-mingw-w64-i686.
2019-07-12T14:39:53.9180910Z Preparing to unpack .../gcc-mingw-w64-i686_5.3.1-8ubuntu3+17_amd64.deb ...
2019-07-12T14:39:53.9332591Z Unpacking gcc-mingw-w64-i686 (5.3.1-8ubuntu3+17) ...
2019-07-12T14:39:58.0184678Z Selecting previously unselected package g++-mingw-w64-i686.
2019-07-12T14:39:58.0205748Z Preparing to unpack .../g++-mingw-w64-i686_5.3.1-8ubuntu3+17_amd64.deb ...
2019-07-12T14:39:58.0339356Z Unpacking g++-mingw-w64-i686 (5.3.1-8ubuntu3+17) ...
2019-07-12T14:40:01.8876209Z Selecting previously unselected package mingw-w64-x86-64-dev.
2019-07-12T14:40:01.8905189Z Preparing to unpack .../mingw-w64-x86-64-dev_4.0.4-2_all.deb ...
2019-07-12T14:40:01.9068013Z Unpacking mingw-w64-x86-64-dev (4.0.4-2) ...
2019-07-12T14:40:03.8957385Z Selecting previously unselected package gcc-mingw-w64-x86-64.
2019-07-12T14:40:03.8993651Z Preparing to unpack .../gcc-mingw-w64-x86-64_5.3.1-8ubuntu3+17_amd64.deb ...
2019-07-12T14:40:03.9120697Z Unpacking gcc-mingw-w64-x86-64 (5.3.1-8ubuntu3+17) ...
2019-07-12T14:40:08.3528161Z Selecting previously unselected package g++-mingw-w64-x86-64.
2019-07-12T14:40:08.3556947Z Preparing to unpack .../g++-mingw-w64-x86-64_5.3.1-8ubuntu3+17_amd64.deb ...
2019-07-12T14:40:08.3678918Z Unpacking g++-mingw-w64-x86-64 (5.3.1-8ubuntu3+17) ...
2019-07-12T14:40:12.6598179Z Selecting previously unselected package g++-mingw-w64.
2019-07-12T14:40:12.6629259Z Preparing to unpack .../g++-mingw-w64_5.3.1-8ubuntu3+17_all.deb ...
2019-07-12T14:40:12.7477926Z Unpacking g++-mingw-w64 (5.3.1-8ubuntu3+17) ...
2019-07-12T14:40:13.2089892Z Selecting previously unselected package gcc-mingw-w64.
2019-07-12T14:40:13.2120144Z Preparing to unpack .../gcc-mingw-w64_5.3.1-8ubuntu3+17_all.deb ...
2019-07-12T14:40:13.2840745Z Unpacking gcc-mingw-w64 (5.3.1-8ubuntu3+17) ...
2019-07-12T14:40:13.7093865Z Selecting previously unselected package mingw-w64.
2019-07-12T14:40:13.7127595Z Preparing to unpack .../mingw-w64_4.0.4-2_all.deb ...
2019-07-12T14:40:13.7475372Z Unpacking mingw-w64 (4.0.4-2) ...
2019-07-12T14:40:14.3698006Z Setting up libgdbm3:amd64 (1.8.3-13.1) ...
2019-07-12T14:40:14.6632793Z Setting up libffi6:amd64 (3.2.1-4) ...
2019-07-12T14:40:14.9112274Z Setting up libglib2.0-0:amd64 (2.48.2-0ubuntu4.3) ...
2019-07-12T14:40:15.0614426Z No schema files found: doing nothing.
---
2019-07-12T14:40:21.7779922Z Setting up zlib1g-dev:amd64 (1:1.2.8.dfsg-2ubuntu4.1) ...
2019-07-12T14:40:21.8194657Z Setting up libssl-dev:amd64 (1.0.2g-1ubuntu4.15) ...
2019-07-12T14:40:21.8586452Z Setting up pkg-config (0.29.1-0ubuntu1) ...
2019-07-12T14:40:21.9427052Z Setting up python2.7 (2.7.12-1ubuntu0~16.04.4) ...
2019-07-12T14:40:22.7290058Z Setting up binutils-mingw-w64-i686 (2.26-3ubuntu1+6.6) ...
2019-07-12T14:40:22.7665104Z Setting up binutils-mingw-w64-x86-64 (2.26-3ubuntu1+6.6) ...
2019-07-12T14:40:22.8054481Z Setting up mingw-w64-common (4.0.4-2) ...
2019-07-12T14:40:22.8483312Z Setting up mingw-w64-i686-dev (4.0.4-2) ...
2019-07-12T14:40:22.8920071Z Setting up gcc-mingw-w64-base (5.3.1-8ubuntu3+17) ...
2019-07-12T14:40:22.9314227Z Setting up gcc-mingw-w64-i686 (5.3.1-8ubuntu3+17) ...
2019-07-12T14:40:22.9614795Z update-alternatives: using /usr/bin/i686-w64-mingw32-gcc-posix to provide /usr/bin/i686-w64-mingw32-gcc (i686-w64-mingw32-gcc) in auto mode
2019-07-12T14:40:22.9618085Z update-alternatives: warning: skip creation of /usr/bin/i686-w64-mingw32-gcc-5 because associated file /usr/bin/i686-w64-mingw32-gcc-5-posix (of link group i686-w64-mingw32-gcc) doesn't exist
2019-07-12T14:40:22.9721140Z update-alternatives: using /usr/bin/i686-w64-mingw32-gcc-win32 to provide /usr/bin/i686-w64-mingw32-gcc (i686-w64-mingw32-gcc) in auto mode
2019-07-12T14:40:22.9724604Z update-alternatives: warning: skip creation of /usr/bin/i686-w64-mingw32-gcc-5 because associated file /usr/bin/i686-w64-mingw32-gcc-5-win32 (of link group i686-w64-mingw32-gcc) doesn't exist
2019-07-12T14:40:22.9979405Z Setting up g++-mingw-w64-i686 (5.3.1-8ubuntu3+17) ...
2019-07-12T14:40:23.0261483Z update-alternatives: using /usr/bin/i686-w64-mingw32-g++-posix to provide /usr/bin/i686-w64-mingw32-g++ (i686-w64-mingw32-g++) in auto mode
2019-07-12T14:40:23.0350135Z update-alternatives: using /usr/bin/i686-w64-mingw32-g++-win32 to provide /usr/bin/i686-w64-mingw32-g++ (i686-w64-mingw32-g++) in auto mode
2019-07-12T14:40:23.0588669Z Setting up mingw-w64-x86-64-dev (4.0.4-2) ...
2019-07-12T14:40:23.0975072Z Setting up gcc-mingw-w64-x86-64 (5.3.1-8ubuntu3+17) ...
2019-07-12T14:40:23.1255585Z update-alternatives: using /usr/bin/x86_64-w64-mingw32-gcc-posix to provide /usr/bin/x86_64-w64-mingw32-gcc (x86_64-w64-mingw32-gcc) in auto mode
2019-07-12T14:40:23.1260179Z update-alternatives: warning: skip creation of /usr/bin/x86_64-w64-mingw32-gcc-5 because associated file /usr/bin/x86_64-w64-mingw32-gcc-5-posix (of link group x86_64-w64-mingw32-gcc) doesn't exist
2019-07-12T14:40:23.1345581Z update-alternatives: using /usr/bin/x86_64-w64-mingw32-gcc-win32 to provide /usr/bin/x86_64-w64-mingw32-gcc (x86_64-w64-mingw32-gcc) in auto mode
2019-07-12T14:40:23.1348063Z update-alternatives: warning: skip creation of /usr/bin/x86_64-w64-mingw32-gcc-5 because associated file /usr/bin/x86_64-w64-mingw32-gcc-5-win32 (of link group x86_64-w64-mingw32-gcc) doesn't exist
2019-07-12T14:40:23.1584094Z Setting up g++-mingw-w64-x86-64 (5.3.1-8ubuntu3+17) ...
2019-07-12T14:40:23.1864822Z update-alternatives: using /usr/bin/x86_64-w64-mingw32-g++-posix to provide /usr/bin/x86_64-w64-mingw32-g++ (x86_64-w64-mingw32-g++) in auto mode
2019-07-12T14:40:23.1957534Z update-alternatives: using /usr/bin/x86_64-w64-mingw32-g++-win32 to provide /usr/bin/x86_64-w64-mingw32-g++ (x86_64-w64-mingw32-g++) in auto mode
2019-07-12T14:40:23.2231775Z Setting up g++-mingw-w64 (5.3.1-8ubuntu3+17) ...
2019-07-12T14:40:23.2678796Z Setting up gcc-mingw-w64 (5.3.1-8ubuntu3+17) ...
2019-07-12T14:40:23.3091238Z Setting up mingw-w64 (4.0.4-2) ...
2019-07-12T14:40:23.3858555Z Processing triggers for ca-certificates (20170717~16.04.2) ...
2019-07-12T14:40:23.4078116Z Updating certificates in /etc/ssl/certs...
2019-07-12T14:40:24.9634721Z 148 added, 0 removed; done.
2019-07-12T14:40:24.9635531Z Running hooks in /etc/ca-certificates/update.d...
---
2019-07-12T14:41:05.0172442Z  ---> 84babdf6a60a
2019-07-12T14:41:05.0214152Z Successfully built 84babdf6a60a
2019-07-12T14:41:05.2144170Z Successfully tagged rust-ci:latest
2019-07-12T14:41:05.3042394Z Built container sha256:84babdf6a60a9faf8be0f05266ba5ebc599a6e08bc6bd5fb49a6a729566517bb
2019-07-12T14:41:05.3057863Z Uploading finished image to https://.s3.amazonaws.com/docker/a4940e6914a5e1f6360ebc241d0d850aff083be5c43d268fd716ce48f63cb24e84238becb662f27f8c5d7740413ca17da24bb9cdc003ef32d7b03d2c9052b94d
2019-07-12T14:42:31.5254223Z upload failed: - to s3:///docker/a4940e6914a5e1f6360ebc241d0d850aff083be5c43d268fd716ce48f63cb24e84238becb662f27f8c5d7740413ca17da24bb9cdc003ef32d7b03d2c9052b94d Parameter validation failed:
2019-07-12T14:42:31.5255454Z Invalid bucket name "": Bucket name must match the regex "^[a-zA-Z0-9.\-_]{1,255}$"
2019-07-12T14:42:32.6223975Z [CI_JOB_NAME=mingw-check]
2019-07-12T14:42:32.6280564Z Starting sccache server...
2019-07-12T14:42:32.6810570Z configure: processing command line
2019-07-12T14:42:32.6810760Z configure: 
---
2019-07-12T14:44:45.5248646Z     Checking core v0.0.0 (/checkout/src/libcore)
2019-07-12T14:44:48.9528413Z error[E0547]: missing 'issue'
2019-07-12T14:44:48.9528866Z    --> src/libcore/option.rs:924:5
2019-07-12T14:44:48.9530790Z     |
2019-07-12T14:44:48.9531653Z 924 |     #[unstable(feature = "option_err_or")]
2019-07-12T14:44:48.9533211Z 
2019-07-12T14:44:48.9533546Z error[E0547]: missing 'issue'
2019-07-12T14:44:48.9533830Z    --> src/libcore/option.rs:951:5
2019-07-12T14:44:48.9534057Z     |
2019-07-12T14:44:48.9534057Z     |
2019-07-12T14:44:48.9534339Z 951 |     #[unstable(feature = "option_err_or")]
2019-07-12T14:44:48.9534679Z 
2019-07-12T14:44:48.9534679Z 
2019-07-12T14:44:50.4869192Z error[E0592]: duplicate definitions with name `ok_or_else`
2019-07-12T14:44:50.4869830Z     |
2019-07-12T14:44:50.4869830Z     |
2019-07-12T14:44:50.4870162Z 543 | /     pub fn ok_or_else<E, F: FnOnce() -> E>(self, err: F) -> Result<T, E> {
2019-07-12T14:44:50.4870704Z 544 | |         match self {
2019-07-12T14:44:50.4871067Z 545 | |             Some(v) => Ok(v),
2019-07-12T14:44:50.4871396Z 546 | |             None => Err(err()),
2019-07-12T14:44:50.4872508Z 548 | |     }
2019-07-12T14:44:50.4872508Z 548 | |     }
2019-07-12T14:44:50.4872852Z     | |_____^ duplicate definitions for `ok_or_else`
2019-07-12T14:44:50.4873059Z ...
2019-07-12T14:44:50.4873414Z 952 | /     pub fn ok_or_else<T, F: FnOnce() -> T>(self, ok: F) -> Result<T, E> {
2019-07-12T14:44:50.4873745Z 953 | |         match self {
2019-07-12T14:44:50.4874050Z 954 | |             Some(v) => Err(v),
2019-07-12T14:44:50.4874375Z 955 | |             None => Ok(ok()),
2019-07-12T14:44:50.4874965Z 957 | |     }
2019-07-12T14:44:50.4874965Z 957 | |     }
2019-07-12T14:44:50.4875289Z     | |_____- other definition for `ok_or_else`
2019-07-12T14:44:50.4903816Z error: aborting due to 3 previous errors
2019-07-12T14:44:50.4903890Z 
2019-07-12T14:44:50.4904223Z For more information about this error, try `rustc --explain E0592`.
2019-07-12T14:44:50.5849354Z error: Could not compile `core`.
2019-07-12T14:44:50.5849354Z error: Could not compile `core`.
2019-07-12T14:44:50.5865086Z warning: build failed, waiting for other jobs to finish...
2019-07-12T14:44:52.6482007Z error: build failed
2019-07-12T14:44:52.6504180Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
2019-07-12T14:44:52.6509221Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-07-12T14:44:52.6509565Z Build completed unsuccessfully in 0:02:19
2019-07-12T14:44:52.6509565Z Build completed unsuccessfully in 0:02:19
2019-07-12T14:45:06.1279884Z ##[error]Bash exited with code '1'.
2019-07-12T14:45:06.1334975Z ##[section]Starting: Checkout
2019-07-12T14:45:06.1336740Z ==============================================================================
2019-07-12T14:45:06.1336796Z Task         : Get sources
2019-07-12T14:45:06.1336844Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
