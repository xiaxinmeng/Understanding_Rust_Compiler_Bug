plain
2019-07-10T10:41:38.8613733Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-10T10:41:38.8795047Z ##[command]git config gc.auto 0
2019-07-10T10:41:38.8870669Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-10T10:41:38.8927724Z ##[command]git config --get-all http.proxy
2019-07-10T10:41:38.9075165Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/61590/merge:refs/remotes/pull/61590/merge
---
2019-07-10T10:42:13.5209960Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-10T10:42:13.5210053Z 
2019-07-10T10:42:13.5210290Z   git checkout -b <new-branch-name>
2019-07-10T10:42:13.5210314Z 
2019-07-10T10:42:13.5210350Z HEAD is now at 050095660 Merge 151cf12677cee27e5b880473cd023dc66e08b7ca into 0324a2b309cd66cb7bd4a156bd0b84cb136e254f
2019-07-10T10:42:13.5363533Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-10T10:42:13.5366492Z ==============================================================================
2019-07-10T10:42:13.5366553Z Task         : Bash
2019-07-10T10:42:13.5366617Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-10T10:44:04.9683554Z Attempting with retry: curl -y 30 -Y 10 --connect-timeout 30 -f -L -C - -o /tmp/rustci_docker_cache https://.s3.amazonaws.com/docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624
2019-07-10T10:44:04.9743028Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-10T10:44:04.9747723Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-10T10:44:04.9747767Z 
2019-07-10T10:44:04.9748225Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-10T10:44:05.9816278Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-10T10:44:05.9816439Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-10T10:44:05.9816470Z 
2019-07-10T10:44:05.9816470Z 
2019-07-10T10:44:05.9861387Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-10T10:44:07.9936721Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-10T10:44:07.9937592Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-10T10:44:07.9937655Z 
2019-07-10T10:44:07.9937655Z 
2019-07-10T10:44:07.9979311Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-10T10:44:11.0050276Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-10T10:44:11.0050725Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-10T10:44:11.0050913Z 
2019-07-10T10:44:11.0050913Z 
2019-07-10T10:44:11.0100527Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-10T10:44:15.0175170Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-10T10:44:15.0175727Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-10T10:44:15.0188100Z 
2019-07-10T10:44:15.0188100Z 
2019-07-10T10:44:15.0218235Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-10T10:44:15.0224493Z The command has failed after 5 attempts.
2019-07-10T10:44:15.0467660Z open /tmp/rustci_docker_cache: no such file or directory
2019-07-10T10:44:15.0494243Z Attempting with retry: docker build --rm -t rust-ci -f /home/vsts/work/1/s/src/ci/docker/x86_64-gnu-llvm-6.0/Dockerfile /home/vsts/work/1/s/src/ci/docker
2019-07-10T10:44:15.2278163Z Sending build context to Docker daemon  521.7kB
2019-07-10T10:44:15.2278885Z 
2019-07-10T10:44:15.2520051Z Step 1/8 : FROM ubuntu:16.04
---
2019-07-10T10:44:33.7482317Z Reading package lists...
2019-07-10T10:44:34.6674107Z Reading package lists...
2019-07-10T10:44:34.8569945Z Building dependency tree...
2019-07-10T10:44:34.8570054Z Reading state information...
2019-07-10T10:44:34.9805270Z The following additional packages will be installed:
2019-07-10T10:44:34.9806092Z   binfmt-support binutils bzip2 cmake-data cpp cpp-5 dpkg-dev g++-5 gcc gcc-5
2019-07-10T10:44:34.9806432Z   git-man libarchive13 libasan2 libasn1-8-heimdal libatomic1
2019-07-10T10:44:34.9806708Z   libbabeltrace-ctf1 libbabeltrace1 libbsd-dev libbsd0 libbz2-1.0 libc-dev-bin
2019-07-10T10:44:34.9807263Z   libedit2 liberror-perl libexpat1 libffi-dev libffi6 libgcc-5-dev libgdbm3
2019-07-10T10:44:34.9807515Z   libglib2.0-0 libgmp10 libgnutls30 libgomp1 libgssapi-krb5-2
2019-07-10T10:44:34.9807783Z   libgssapi3-heimdal libhcrypto4-heimdal libheimbase1-heimdal
2019-07-10T10:44:34.9808102Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-10T10:44:34.9808102Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-10T10:44:34.9808577Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-10T10:44:34.9808804Z   libkrb5-3 libkrb5support0 libldap-2.4-2 libllvm6.0 liblsan0 liblzo2-2
2019-07-10T10:44:34.9809074Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-10T10:44:34.9809300Z   libperl5.22 libpipeline1 libpython-stdlib libpython2.7-minimal
2019-07-10T10:44:34.9809528Z   libpython2.7-stdlib libpython3.5 libpython3.5-minimal libpython3.5-stdlib
2019-07-10T10:44:34.9809802Z   libquadmath0 libroken18-heimdal librtmp1 libsasl2-2 libsasl2-modules-db
2019-07-10T10:44:34.9810037Z   libsqlite3-0 libssl1.0.0 libstdc++-5-dev libtasn1-6 libtinfo-dev libtsan0
2019-07-10T10:44:34.9810262Z   libubsan0 libwind0-heimdal libxml2 linux-libc-dev llvm-6.0 llvm-6.0-dev
2019-07-10T10:44:34.9810545Z   llvm-6.0-runtime mime-support openssl patch perl perl-modules-5.22 python
2019-07-10T10:44:34.9810755Z   python-minimal python2.7-minimal
2019-07-10T10:44:34.9810800Z Suggested packages:
2019-07-10T10:44:34.9811065Z   binutils-doc bzip2-doc codeblocks eclipse ninja-build cpp-doc gcc-5-locales
2019-07-10T10:44:34.9811296Z   debian-keyring g++-multilib g++-5-multilib gcc-5-doc libstdc++6-5-dbg
2019-07-10T10:44:34.9811516Z   gcc-multilib manpages-dev autoconf automake libtool flex bison gcc-doc
2019-07-10T10:44:34.9812013Z   libasan2-dbg liblsan0-dbg libtsan0-dbg libubsan0-dbg libcilkrts5-dbg
2019-07-10T10:44:34.9812232Z   libmpx0-dbg libquadmath0-dbg gdb-doc gettext-base git-daemon-run
2019-07-10T10:44:34.9812232Z   libmpx0-dbg libquadmath0-dbg gdb-doc gettext-base git-daemon-run
2019-07-10T10:44:34.9812455Z   | git-daemon-sysvinit git-doc git-el git-email git-gui gitk gitweb git-arch
2019-07-10T10:44:34.9812723Z   git-cvs git-mediawiki git-svn lrzip glibc-doc gnutls-bin krb5-doc krb5-user
2019-07-10T10:44:34.9812953Z   libstdc++-5-doc llvm-6.0-doc make-doc ed diffutils-doc perl-doc
2019-07-10T10:44:34.9813184Z   libterm-readline-gnu-perl | libterm-readline-perl-perl python-doc python-tk
2019-07-10T10:44:34.9813402Z   python2.7-doc
2019-07-10T10:44:34.9813446Z Recommended packages:
2019-07-10T10:44:34.9813663Z   build-essential fakeroot libalgorithm-merge-perl libc-dbg gdbserver less
2019-07-10T10:44:34.9813919Z   rsync ssh-client manpages manpages-dev libfile-fcntllock-perl
2019-07-10T10:44:34.9814148Z   libglib2.0-data shared-mime-info xdg-user-dirs krb5-locales libsasl2-modules
2019-07-10T10:44:34.9814342Z   libssl-doc xml-core netbase rename
2019-07-10T10:44:34.9814435Z The following NEW packages will be installed:
2019-07-10T10:44:34.9814790Z   binfmt-support binutils bzip2 ca-certificates cmake cmake-data cpp cpp-5
2019-07-10T10:44:34.9815225Z   curl dpkg-dev file g++ g++-5 gcc gcc-5 gdb git git-man libarchive13 libasan2
2019-07-10T10:44:34.9815537Z   libasn1-8-heimdal libatomic1 libbabeltrace-ctf1 libbabeltrace1 libbsd-dev
2019-07-10T10:44:34.9816212Z   libbsd0 libc-dev-bin libc6-dev libcc1-0 libcilkrts5 libcurl3 libcurl3-gnutls
2019-07-10T10:44:34.9816501Z   libdpkg-perl libedit-dev libedit2 liberror-perl libexpat1 libffi-dev libffi6
2019-07-10T10:44:34.9816814Z   libgcc-5-dev libgdbm3 libglib2.0-0 libgmp10 libgnutls30 libgomp1
2019-07-10T10:44:34.9817321Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-10T10:44:34.9817634Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-10T10:44:34.9817886Z   libkrb5-3 libkrb5support0 libldap-2.4-2 libllvm6.0 liblsan0 liblzo2-2
2019-07-10T10:44:34.9818134Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-10T10:44:34.9818134Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-10T10:44:34.9818745Z   libperl5.22 libpipeline1 libpython-stdlib libpython2.7-minimal
2019-07-10T10:44:34.9818974Z   libpython2.7-stdlib libpython3.5 libpython3.5-minimal libpython3.5-stdlib
2019-07-10T10:44:34.9819222Z   libquadmath0 libroken18-heimdal librtmp1 libsasl2-2 libsasl2-modules-db
2019-07-10T10:44:34.9819508Z   libsqlite3-0 libssl-dev libssl1.0.0 libstdc++-5-dev libtasn1-6 libtinfo-dev
2019-07-10T10:44:34.9819732Z   libtsan0 libubsan0 libwind0-heimdal libxml2 linux-libc-dev llvm-6.0
2019-07-10T10:44:34.9819960Z   llvm-6.0-dev llvm-6.0-runtime llvm-6.0-tools make mime-support openssl patch
2019-07-10T10:44:34.9820233Z   perl perl-modules-5.22 pkg-config python python-minimal python2.7
2019-07-10T10:44:34.9820434Z   python2.7-minimal sudo xz-utils zlib1g-dev
2019-07-10T10:44:34.9820481Z The following packages will be upgraded:
2019-07-10T10:44:35.3961894Z 1 upgraded, 115 newly installed, 0 to remove and 4 not upgraded.
2019-07-10T10:44:35.3963192Z Need to get 121 MB of archives.
2019-07-10T10:44:35.3963353Z After this operation, 592 MB of additional disk space will be used.
2019-07-10T10:44:35.3964404Z Get:1 http://archive.ubuntu.com/ubuntu xenial/main amd64 libgdbm3 amd64 1.8.3-13.1 [16.9 kB]
---
2019-07-10T10:44:37.8390411Z Get:61 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 sudo amd64 1.8.16-0ubuntu1.7 [390 kB]
2019-07-10T10:44:37.9114344Z Get:62 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 openssl amd64 1.0.2g-1ubuntu4.15 [492 kB]
2019-07-10T10:44:37.9186010Z Get:63 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 ca-certificates all 20170717~16.04.2 [167 kB]
2019-07-10T10:44:37.9217996Z Get:64 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 libcurl3-gnutls amd64 7.47.0-1ubuntu2.13 [184 kB]
2019-07-10T10:44:37.9249088Z Get:65 http://archive.ubuntu.com/ubuntu xenial/main amd64 libedit2 amd64 3.1-20150325-1ubuntu2 [76.5 kB]
2019-07-10T10:44:37.9268995Z Get:66 http://archive.ubuntu.com/ubuntu xenial/main amd64 libpipeline1 amd64 1.4.1-2 [24.6 kB]
2019-07-10T10:44:37.9270728Z Get:67 http://archive.ubuntu.com/ubuntu xenial/main amd64 binfmt-support amd64 2.1.6-1 [50.7 kB]
2019-07-10T10:44:38.0547483Z Get:69 http://archive.ubuntu.com/ubuntu xenial/main amd64 libisl15 amd64 0.16.1-1 [524 kB]
2019-07-10T10:44:38.0614803Z Get:70 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 cpp-5 amd64 5.4.0-6ubuntu1~16.04.11 [7660 kB]
2019-07-10T10:44:38.3878046Z Get:71 http://archive.ubuntu.com/ubuntu xenial/main amd64 cpp amd64 4:5.3.1-1ubuntu1 [27.7 kB]
2019-07-10T10:44:38.3885895Z Get:72 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 curl amd64 7.47.0-1ubuntu2.13 [139 kB]
---
2019-07-10T10:44:59.0852859Z Unpacking ca-certificates (20170717~16.04.2) ...
2019-07-10T10:44:59.2632870Z Selecting previously unselected package libcurl3-gnutls:amd64.
2019-07-10T10:44:59.2651986Z Preparing to unpack .../libcurl3-gnutls_7.47.0-1ubuntu2.13_amd64.deb ...
2019-07-10T10:44:59.2781078Z Unpacking libcurl3-gnutls:amd64 (7.47.0-1ubuntu2.13) ...
2019-07-10T10:44:59.4205756Z Selecting previously unselected package libedit2:amd64.
2019-07-10T10:44:59.4228343Z Preparing to unpack .../libedit2_3.1-20150325-1ubuntu2_amd64.deb ...
2019-07-10T10:44:59.4371723Z Unpacking libedit2:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-10T10:44:59.5685561Z Selecting previously unselected package libpipeline1:amd64.
2019-07-10T10:44:59.5704659Z Preparing to unpack .../libpipeline1_1.4.1-2_amd64.deb ...
2019-07-10T10:44:59.5838321Z Unpacking libpipeline1:amd64 (1.4.1-2) ...
2019-07-10T10:44:59.8281472Z Selecting previously unselected package binfmt-support.
2019-07-10T10:44:59.8297126Z Preparing to unpack .../binfmt-support_2.1.6-1_amd64.deb ...
2019-07-10T10:44:59.8431312Z Unpacking binfmt-support (2.1.6-1) ...
2019-07-10T10:44:59.9725124Z Preparing to unpack .../binutils_2.26.1-1ubuntu1~16.04.8_amd64.deb ...
2019-07-10T10:44:59.9879575Z Unpacking binutils (2.26.1-1ubuntu1~16.04.8) ...
2019-07-10T10:45:00.4592499Z Selecting previously unselected package libisl15:amd64.
2019-07-10T10:45:00.4612004Z Preparing to unpack .../libisl15_0.16.1-1_amd64.deb ...
---
2019-07-10T10:45:12.3739017Z Selecting previously unselected package libtinfo-dev:amd64.
2019-07-10T10:45:12.3761030Z Preparing to unpack .../libtinfo-dev_6.0+20160213-1ubuntu1_amd64.deb ...
2019-07-10T10:45:12.3913303Z Unpacking libtinfo-dev:amd64 (6.0+20160213-1ubuntu1) ...
2019-07-10T10:45:12.4995383Z Selecting previously unselected package libedit-dev:amd64.
2019-07-10T10:45:12.5019988Z Preparing to unpack .../libedit-dev_3.1-20150325-1ubuntu2_amd64.deb ...
2019-07-10T10:45:12.5175111Z Unpacking libedit-dev:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-10T10:45:12.6332698Z Selecting previously unselected package libllvm6.0:amd64.
2019-07-10T10:45:12.6355263Z Preparing to unpack .../libllvm6.0_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-10T10:45:12.6496862Z Unpacking libllvm6.0:amd64 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-10T10:45:15.4957180Z Selecting previously unselected package zlib1g-dev:amd64.
2019-07-10T10:45:15.4976629Z Preparing to unpack .../zlib1g-dev_1%3a1.2.8.dfsg-2ubuntu4.1_amd64.deb ...
2019-07-10T10:45:15.5105936Z Unpacking zlib1g-dev:amd64 (1:1.2.8.dfsg-2ubuntu4.1) ...
2019-07-10T10:45:15.6182620Z Preparing to unpack .../libssl-dev_1.0.2g-1ubuntu4.15_amd64.deb ...
2019-07-10T10:45:15.6322080Z Unpacking libssl-dev:amd64 (1.0.2g-1ubuntu4.15) ...
2019-07-10T10:45:15.9400917Z Selecting previously unselected package llvm-6.0-runtime.
2019-07-10T10:45:15.9400917Z Selecting previously unselected package llvm-6.0-runtime.
2019-07-10T10:45:15.9424037Z Preparing to unpack .../llvm-6.0-runtime_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-10T10:45:15.9565588Z Unpacking llvm-6.0-runtime (1:6.0-1ubuntu2~16.04.1) ...
2019-07-10T10:45:16.0867714Z Selecting previously unselected package llvm-6.0.
2019-07-10T10:45:16.0889704Z Preparing to unpack .../llvm-6.0_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-10T10:45:16.1036084Z Unpacking llvm-6.0 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-10T10:45:16.7158990Z Selecting previously unselected package libffi-dev:amd64.
2019-07-10T10:45:16.7177688Z Preparing to unpack .../libffi-dev_3.2.1-4_amd64.deb ...
2019-07-10T10:45:16.7313439Z Unpacking libffi-dev:amd64 (3.2.1-4) ...
2019-07-10T10:45:16.8595281Z Selecting previously unselected package llvm-6.0-dev.
2019-07-10T10:45:16.8616363Z Preparing to unpack .../llvm-6.0-dev_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-10T10:45:16.8769511Z Unpacking llvm-6.0-dev (1:6.0-1ubuntu2~16.04.1) ...
2019-07-10T10:45:21.3853196Z Selecting previously unselected package llvm-6.0-tools.
2019-07-10T10:45:21.3875797Z Preparing to unpack .../llvm-6.0-tools_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-10T10:45:21.4011271Z Unpacking llvm-6.0-tools (1:6.0-1ubuntu2~16.04.1) ...
2019-07-10T10:45:21.5618008Z Selecting previously unselected package pkg-config.
2019-07-10T10:45:21.5645292Z Preparing to unpack .../pkg-config_0.29.1-0ubuntu1_amd64.deb ...
2019-07-10T10:45:21.5784321Z Unpacking pkg-config (0.29.1-0ubuntu1) ...
2019-07-10T10:45:22.2115306Z Processing triggers for systemd (229-4ubuntu21.21) ...
2019-07-10T10:45:22.2116443Z Setting up libgdbm3:amd64 (1.8.3-13.1) ...
2019-07-10T10:45:22.2117461Z Setting up libffi6:amd64 (3.2.1-4) ...
2019-07-10T10:45:22.2117961Z Setting up libglib2.0-0:amd64 (2.48.2-0ubuntu4.3) ...
---
2019-07-10T10:45:26.0705710Z debconf: unable to initialize frontend: Dialog
2019-07-10T10:45:26.0706976Z debconf: (TERM is not set, so the dialog frontend is not usable.)
2019-07-10T10:45:26.0707369Z debconf: falling back to frontend: Readline
2019-07-10T10:45:26.6119560Z Setting up libcurl3-gnutls:amd64 (7.47.0-1ubuntu2.13) ...
2019-07-10T10:45:26.6518520Z Setting up libedit2:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-10T10:45:26.7034391Z Setting up libpipeline1:amd64 (1.4.1-2) ...
2019-07-10T10:45:26.7452967Z Setting up binfmt-support (2.1.6-1) ...
2019-07-10T10:45:26.8236987Z mount: permission denied
2019-07-10T10:45:26.8238631Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-10T10:45:26.8254043Z mount: permission denied
2019-07-10T10:45:26.8256722Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-10T10:45:26.9792693Z invoke-rc.d: could not determine current runlevel
2019-07-10T10:45:26.9823893Z invoke-rc.d: policy-rc.d denied execution of start.
2019-07-10T10:45:27.0486800Z Setting up libisl15:amd64 (0.16.1-1) ...
2019-07-10T10:45:27.0939254Z Setting up cpp-5 (5.4.0-6ubuntu1~16.04.11) ...
2019-07-10T10:45:27.1346056Z Setting up cpp (4:5.3.1-1ubuntu1) ...
2019-07-10T10:45:27.1878433Z Setting up curl (7.47.0-1ubuntu2.13) ...
---
2019-07-10T10:45:28.9226745Z Setting up libedit-dev:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-10T10:45:28.9616693Z Setting up libllvm6.0:amd64 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-10T10:45:29.0137609Z Setting up zlib1g-dev:amd64 (1:1.2.8.dfsg-2ubuntu4.1) ...
2019-07-10T10:45:29.0522709Z Setting up libssl-dev:amd64 (1.0.2g-1ubuntu4.15) ...
2019-07-10T10:45:29.0918736Z Setting up llvm-6.0-runtime (1:6.0-1ubuntu2~16.04.1) ...
2019-07-10T10:45:29.1232646Z mount: permission denied
2019-07-10T10:45:29.1241784Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-10T10:45:29.1461655Z Setting up llvm-6.0 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-10T10:45:29.1886185Z Setting up libffi-dev:amd64 (3.2.1-4) ...
2019-07-10T10:45:29.2289176Z Setting up llvm-6.0-dev (1:6.0-1ubuntu2~16.04.1) ...
2019-07-10T10:45:29.2746858Z Setting up llvm-6.0-tools (1:6.0-1ubuntu2~16.04.1) ...
2019-07-10T10:45:29.3205434Z Setting up pkg-config (0.29.1-0ubuntu1) ...
2019-07-10T10:45:29.4604697Z Processing triggers for ca-certificates (20170717~16.04.2) ...
2019-07-10T10:45:29.4774462Z Updating certificates in /etc/ssl/certs...
2019-07-10T10:45:31.0688317Z 148 added, 0 removed; done.
2019-07-10T10:45:31.0693095Z Running hooks in /etc/ca-certificates/update.d...
---
2019-07-10T10:46:04.4915412Z Removing intermediate container e0e0b8899d3a
2019-07-10T10:46:04.4916627Z  ---> a0cf3b9db550
2019-07-10T10:46:04.4953495Z Successfully built a0cf3b9db550
2019-07-10T10:46:04.7078493Z Successfully tagged rust-ci:latest
2019-07-10T10:46:04.7838268Z Built container sha256:a0cf3b9db5505c4e9cb33e856686cb48e128561ae2985ac2b9481917f55fcc7d
2019-07-10T10:46:04.7855435Z Uploading finished image to https://.s3.amazonaws.com/docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624
2019-07-10T10:47:05.2463584Z upload failed: - to s3:///docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624 Parameter validation failed:
2019-07-10T10:47:05.2464504Z Invalid bucket name "": Bucket name must match the regex "^[a-zA-Z0-9.\-_]{1,255}$"
2019-07-10T10:47:06.0368162Z [CI_JOB_NAME=x86_64-gnu-llvm-6.0]
2019-07-10T10:47:06.0428115Z Starting sccache server...
2019-07-10T10:47:06.0969082Z configure: processing command line
2019-07-10T10:47:06.0969220Z configure: 
---
2019-07-10T10:47:22.6177997Z ######################################################################## 100.0%
2019-07-10T10:47:23.5123312Z extracting /checkout/obj/build/cache/2019-07-04/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
2019-07-10T10:47:23.5507036Z warning: /checkout/src/tools/miri/Cargo.toml: the cargo feature `default-run` is now stable and is no longer necessary to be listed in the manifest
2019-07-10T10:47:24.7730770Z     Updating crates.io index
2019-07-10T10:47:44.8015564Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-07-10T10:47:44.8046185Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-07-10T10:47:44.8046441Z Build completed unsuccessfully in 0:00:38
2019-07-10T10:47:44.8098457Z make: *** [prepare] Error 1
2019-07-10T10:47:44.8099096Z Makefile:69: recipe for target 'prepare' failed
2019-07-10T10:47:45.8816294Z warning: /checkout/src/tools/miri/Cargo.toml: the cargo feature `default-run` is now stable and is no longer necessary to be listed in the manifest
2019-07-10T10:47:45.8816294Z warning: /checkout/src/tools/miri/Cargo.toml: the cargo feature `default-run` is now stable and is no longer necessary to be listed in the manifest
2019-07-10T10:47:46.0310515Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-07-10T10:47:46.0328203Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-07-10T10:47:46.0328279Z Build completed unsuccessfully in 0:00:00
2019-07-10T10:47:46.0379934Z make: *** [prepare] Error 1
2019-07-10T10:47:46.0383610Z Makefile:69: recipe for target 'prepare' failed
2019-07-10T10:47:48.1172018Z warning: /checkout/src/tools/miri/Cargo.toml: the cargo feature `default-run` is now stable and is no longer necessary to be listed in the manifest
2019-07-10T10:47:48.1172018Z warning: /checkout/src/tools/miri/Cargo.toml: the cargo feature `default-run` is now stable and is no longer necessary to be listed in the manifest
2019-07-10T10:47:48.2633008Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-07-10T10:47:48.2647107Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-07-10T10:47:48.2647178Z Build completed unsuccessfully in 0:00:00
2019-07-10T10:47:48.2694810Z Makefile:69: recipe for target 'prepare' failed
2019-07-10T10:47:48.2694875Z make: *** [prepare] Error 1
2019-07-10T10:47:51.3406937Z warning: /checkout/src/tools/miri/Cargo.toml: the cargo feature `default-run` is now stable and is no longer necessary to be listed in the manifest
2019-07-10T10:47:51.3406937Z warning: /checkout/src/tools/miri/Cargo.toml: the cargo feature `default-run` is now stable and is no longer necessary to be listed in the manifest
2019-07-10T10:47:51.4947661Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-07-10T10:47:51.4964980Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-07-10T10:47:51.4965913Z Build completed unsuccessfully in 0:00:00
2019-07-10T10:47:51.5017314Z Makefile:69: recipe for target 'prepare' failed
2019-07-10T10:47:51.5029652Z make: *** [prepare] Error 1
2019-07-10T10:47:55.5703071Z warning: /checkout/src/tools/miri/Cargo.toml: the cargo feature `default-run` is now stable and is no longer necessary to be listed in the manifest
2019-07-10T10:47:55.5703071Z warning: /checkout/src/tools/miri/Cargo.toml: the cargo feature `default-run` is now stable and is no longer necessary to be listed in the manifest
2019-07-10T10:47:55.7245146Z error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
2019-07-10T10:47:55.7266413Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-07-10T10:47:55.7266543Z Build completed unsuccessfully in 0:00:00
2019-07-10T10:47:55.7317094Z Makefile:69: recipe for target 'prepare' failed
2019-07-10T10:47:55.7318266Z make: *** [prepare] Error 1
2019-07-10T10:47:55.7323420Z The command has failed after 5 attempts.
2019-07-10T10:48:00.2407097Z ##[error]Bash exited with code '1'.
2019-07-10T10:48:00.2436725Z ##[section]Starting: Checkout
2019-07-10T10:48:00.2438146Z ==============================================================================
2019-07-10T10:48:00.2438189Z Task         : Get sources
2019-07-10T10:48:00.2438225Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
