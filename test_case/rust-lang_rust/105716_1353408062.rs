plain
 ---> e32e6d65f7e9
Step 3/15 : RUN sh /scripts/android-base-apt-get.sh
 ---> Running in 9ebe8b9d5a3a
+ apt-get update
Get:1 http://security.ubuntu.com/ubuntu kinetic-security InRelease [109 kB]
Get:2 http://archive.ubuntu.com/ubuntu kinetic InRelease [267 kB]
Get:3 http://security.ubuntu.com/ubuntu kinetic-security/multiverse amd64 Packages [5516 B]
Get:4 http://archive.ubuntu.com/ubuntu kinetic-updates InRelease [109 kB]
Get:5 http://security.ubuntu.com/ubuntu kinetic-security/universe amd64 Packages [65.8 kB]
Get:6 http://security.ubuntu.com/ubuntu kinetic-security/main amd64 Packages [158 kB]
Get:7 http://archive.ubuntu.com/ubuntu kinetic-backports InRelease [99.9 kB]
Get:8 http://security.ubuntu.com/ubuntu kinetic-security/restricted amd64 Packages [171 kB]
Get:9 http://archive.ubuntu.com/ubuntu kinetic/restricted amd64 Packages [151 kB]
Get:10 http://archive.ubuntu.com/ubuntu kinetic/multiverse amd64 Packages [289 kB]
Get:11 http://archive.ubuntu.com/ubuntu kinetic/universe amd64 Packages [17.9 MB]
Get:12 http://archive.ubuntu.com/ubuntu kinetic/main amd64 Packages [1778 kB]
Get:13 http://archive.ubuntu.com/ubuntu kinetic-updates/universe amd64 Packages [89.7 kB]
Get:14 http://archive.ubuntu.com/ubuntu kinetic-updates/main amd64 Packages [183 kB]
Get:15 http://archive.ubuntu.com/ubuntu kinetic-updates/multiverse amd64 Packages [5516 B]
Get:16 http://archive.ubuntu.com/ubuntu kinetic-updates/restricted amd64 Packages [171 kB]
Get:17 http://archive.ubuntu.com/ubuntu kinetic-backports/universe amd64 Packages [1032 B]
Reading package lists...
+ apt-get install -y --no-install-recommends ca-certificates cmake curl file g++ git libssl-dev libncurses5 make ninja-build pkg-config python3 sudo unzip xz-utils
Reading package lists...
Building dependency tree...
Building dependency tree...
Reading state information...
The following additional packages will be installed:
  binutils binutils-common binutils-x86-64-linux-gnu cmake-data cpp cpp-12
  dh-elpa-helper emacsen-common g++-12 gcc gcc-12 git-man libapparmor1
  libarchive13 libargon2-1 libasan8 libatomic1 libbinutils libbrotli1
  libc-dev-bin libc6-dev libcc1-0 libcrypt-dev libcryptsetup12 libctf-nobfd0
  libctf0 libcurl3-gnutls libcurl4 libdevmapper1.02.1 libdpkg-perl
  liberror-perl libexpat1 libfdisk1 libgcc-12-dev libgdbm-compat4 libgdbm6
  libglib2.0-0 libgomp1 libgprofng0 libgssapi-krb5-2 libicu71 libip4tc2
  libkrb5-3 libkrb5support0 libldap-2.5-0 liblsan0 libmagic-mgc libmagic1
  libmpc3 libmpdec3 libmpfr6 libnghttp2-14 libnsl-dev libnsl2 libperl5.34
  libpsl5 libpython3-stdlib libpython3.10-minimal libpython3.10-stdlib
  libquadmath0 libreadline8 librhash0 librtmp1 libsasl2-2 libsasl2-modules-db
---
  gdbm-l10n krb5-doc krb5-user libssl-doc libstdc++-12-doc make-doc perl-doc
  libterm-readline-gnu-perl | libterm-readline-perl-perl
  libtap-harness-archive-perl dpkg-dev python3-doc python3-tk python3-venv
  python3.10-venv python3.10-doc binfmt-support readline-doc systemd-container
  systemd-homed systemd-userdbd systemd-boot libfido2-1 libtss2-esys-3.0.2-0
  libtss2-mu0 libtss2-rc0 policykit-1 zip
  patch less ssh-client manpages manpages-dev libc-devtools dmsetup
  libfile-fcntllock-perl liblocale-gettext-perl bzip2 libglib2.0-data
  shared-mime-info xdg-user-dirs krb5-locales libldap-common libgpm2
  publicsuffix libsasl2-modules netbase default-dbus-system-bus
---
  readline-common rpcsvc-proto sudo systemd unzip xz-utils
0 upgraded, 111 newly installed, 0 to remove and 0 not upgraded.
Need to get 125 MB of archives.
After this operation, 488 MB of additional disk space will be used.
Get:1 http://archive.ubuntu.com/ubuntu kinetic-updates/main amd64 libssl3 amd64 3.0.5-2ubuntu2 [1893 kB]
Get:2 http://archive.ubuntu.com/ubuntu kinetic-updates/main amd64 libpython3.10-minimal amd64 3.10.7-1ubuntu0.2 [803 kB]
Get:3 http://archive.ubuntu.com/ubuntu kinetic-updates/main amd64 libexpat1 amd64 2.4.8-2ubuntu0.22.10.1 [84.3 kB]
Get:4 http://archive.ubuntu.com/ubuntu kinetic-updates/main amd64 python3.10-minimal amd64 3.10.7-1ubuntu0.2 [2115 kB]
Get:5 http://archive.ubuntu.com/ubuntu kinetic/main amd64 python3-minimal amd64 3.10.6-1 [24.1 kB]
Get:6 http://archive.ubuntu.com/ubuntu kinetic/main amd64 media-types all 8.0.0 [24.9 kB]
Get:7 http://archive.ubuntu.com/ubuntu kinetic/main amd64 libmpdec3 amd64 2.5.1-2build2 [86.8 kB]
Get:8 http://archive.ubuntu.com/ubuntu kinetic/main amd64 libkrb5support0 amd64 1.20-1 [31.2 kB]
Get:9 http://archive.ubuntu.com/ubuntu kinetic/main amd64 libk5crypto3 amd64 1.20-1 [81.2 kB]
Get:10 http://archive.ubuntu.com/ubuntu kinetic/main amd64 libkeyutils1 amd64 1.6.3-1 [10.7 kB]
Get:11 http://archive.ubuntu.com/ubuntu kinetic/main amd64 libkrb5-3 amd64 1.20-1 [346 kB]
Get:12 http://archive.ubuntu.com/ubuntu kinetic/main amd64 libgssapi-krb5-2 amd64 1.20-1 [142 kB]
Get:13 http://archive.ubuntu.com/ubuntu kinetic/main amd64 libtirpc-common all 1.3.3+ds-1 [7790 B]
Get:14 http://archive.ubuntu.com/ubuntu kinetic/main amd64 libtirpc3 amd64 1.3.3+ds-1 [80.9 kB]
Get:15 http://archive.ubuntu.com/ubuntu kinetic/main amd64 libnsl2 amd64 1.3.0-2build2 [42.3 kB]
Get:16 http://archive.ubuntu.com/ubuntu kinetic/main amd64 readline-common all 8.2-1 [55.1 kB]
Get:17 http://archive.ubuntu.com/ubuntu kinetic/main amd64 libreadline8 amd64 8.2-1 [150 kB]
Get:18 http://archive.ubuntu.com/ubuntu kinetic/main amd64 libsqlite3-0 amd64 3.39.3-1 [655 kB]
Get:19 http://archive.ubuntu.com/ubuntu kinetic-updates/main amd64 libpython3.10-stdlib amd64 3.10.7-1ubuntu0.2 [1784 kB]
Get:20 http://archive.ubuntu.com/ubuntu kinetic-updates/main amd64 python3.10 amd64 3.10.7-1ubuntu0.2 [497 kB]
Get:21 http://archive.ubuntu.com/ubuntu kinetic/main amd64 libpython3-stdlib amd64 3.10.6-1 [6848 B]
Get:22 http://archive.ubuntu.com/ubuntu kinetic/main amd64 python3 amd64 3.10.6-1 [22.3 kB]
Get:23 http://archive.ubuntu.com/ubuntu kinetic/main amd64 libargon2-1 amd64 0~20171227-0.3 [19.5 kB]
Get:24 http://archive.ubuntu.com/ubuntu kinetic/main amd64 libdevmapper1.02.1 amd64 2:1.02.185-1ubuntu1 [138 kB]
Get:25 http://archive.ubuntu.com/ubuntu kinetic/main amd64 libjson-c5 amd64 0.16-1 [33.7 kB]
Get:26 http://archive.ubuntu.com/ubuntu kinetic/main amd64 libcryptsetup12 amd64 2:2.5.0-2ubuntu1 [221 kB]
Get:27 http://archive.ubuntu.com/ubuntu kinetic/main amd64 libfdisk1 amd64 2.38-4ubuntu1 [144 kB]
Get:28 http://archive.ubuntu.com/ubuntu kinetic/main amd64 libkmod2 amd64 30+20220630-3ubuntu1 [48.9 kB]
Get:29 http://archive.ubuntu.com/ubuntu kinetic/main amd64 libapparmor1 amd64 3.0.7-1ubuntu2 [36.7 kB]
Get:30 http://archive.ubuntu.com/ubuntu kinetic/main amd64 libip4tc2 amd64 1.8.7-1ubuntu6 [19.7 kB]
Get:31 http://archive.ubuntu.com/ubuntu kinetic/main amd64 libsystemd-shared amd64 251.4-1ubuntu7 [1703 kB]
Get:32 http://archive.ubuntu.com/ubuntu kinetic/main amd64 systemd amd64 251.4-1ubuntu7 [2808 kB]
Get:33 http://archive.ubuntu.com/ubuntu kinetic-updates/main amd64 perl-modules-5.34 all 5.34.0-5ubuntu1.1 [2952 kB]
Get:34 http://archive.ubuntu.com/ubuntu kinetic/main amd64 libgdbm6 amd64 1.23-1 [33.9 kB]
Get:35 http://archive.ubuntu.com/ubuntu kinetic/main amd64 libgdbm-compat4 amd64 1.23-1 [6606 B]
Get:36 http://archive.ubuntu.com/ubuntu kinetic-updates/main amd64 libperl5.34 amd64 5.34.0-5ubuntu1.1 [4713 kB]
Get:37 http://archive.ubuntu.com/ubuntu kinetic-updates/main amd64 perl amd64 5.34.0-5ubuntu1.1 [231 kB]
Get:38 http://archive.ubuntu.com/ubuntu kinetic-updates/main amd64 openssl amd64 3.0.5-2ubuntu2 [1178 kB]
Get:39 http://archive.ubuntu.com/ubuntu kinetic-updates/main amd64 ca-certificates all 20211016ubuntu0.22.10.1 [143 kB]
Get:40 http://archive.ubuntu.com/ubuntu kinetic/main amd64 libglib2.0-0 amd64 2.74.0-3 [1471 kB]
Get:41 http://archive.ubuntu.com/ubuntu kinetic/main amd64 libicu71 amd64 71.1-3ubuntu1 [10.6 MB]
Get:42 http://archive.ubuntu.com/ubuntu kinetic-updates/main amd64 libxml2 amd64 2.9.14+dfsg-1ubuntu0.1 [763 kB]
Get:43 http://archive.ubuntu.com/ubuntu kinetic/main amd64 sudo amd64 1.9.11p3-1ubuntu1 [839 kB]
Get:44 http://archive.ubuntu.com/ubuntu kinetic/main amd64 libmagic-mgc amd64 1:5.41-4 [257 kB]
Get:45 http://archive.ubuntu.com/ubuntu kinetic/main amd64 libmagic1 amd64 1:5.41-4 [87.3 kB]
Get:46 http://archive.ubuntu.com/ubuntu kinetic/main amd64 file amd64 1:5.41-4 [21.5 kB]
Get:47 http://archive.ubuntu.com/ubuntu kinetic/main amd64 libnghttp2-14 amd64 1.49.0-1 [73.5 kB]
Get:48 http://archive.ubuntu.com/ubuntu kinetic/main amd64 libpsl5 amd64 0.21.0-1.2build2 [58.4 kB]
Get:49 http://archive.ubuntu.com/ubuntu kinetic/main amd64 libuv1 amd64 1.44.2-1 [91.0 kB]
Get:50 http://archive.ubuntu.com/ubuntu kinetic/main amd64 xz-utils amd64 5.2.5-2.1 [83.8 kB]
Get:51 http://archive.ubuntu.com/ubuntu kinetic-updates/main amd64 binutils-common amd64 2.39-3ubuntu1.1 [225 kB]
Get:52 http://archive.ubuntu.com/ubuntu kinetic-updates/main amd64 libbinutils amd64 2.39-3ubuntu1.1 [610 kB]
Get:53 http://archive.ubuntu.com/ubuntu kinetic-updates/main amd64 libctf-nobfd0 amd64 2.39-3ubuntu1.1 [99.8 kB]
Get:54 http://archive.ubuntu.com/ubuntu kinetic-updates/main amd64 libctf0 amd64 2.39-3ubuntu1.1 [97.4 kB]
Get:55 http://archive.ubuntu.com/ubuntu kinetic-updates/main amd64 libgprofng0 amd64 2.39-3ubuntu1.1 [897 kB]
Get:56 http://archive.ubuntu.com/ubuntu kinetic-updates/main amd64 binutils-x86-64-linux-gnu amd64 2.39-3ubuntu1.1 [2435 kB]
Get:57 http://archive.ubuntu.com/ubuntu kinetic-updates/main amd64 binutils amd64 2.39-3ubuntu1.1 [3306 B]
Get:58 http://archive.ubuntu.com/ubuntu kinetic/main amd64 libarchive13 amd64 3.6.0-1ubuntu1 [368 kB]
Get:59 http://archive.ubuntu.com/ubuntu kinetic/main amd64 libbrotli1 amd64 1.0.9-2build6 [315 kB]
Get:60 http://archive.ubuntu.com/ubuntu kinetic/main amd64 libsasl2-modules-db amd64 2.1.28+dfsg-6ubuntu2 [20.3 kB]
Get:61 http://archive.ubuntu.com/ubuntu kinetic/main amd64 libsasl2-2 amd64 2.1.28+dfsg-6ubuntu2 [56.6 kB]
Get:62 http://archive.ubuntu.com/ubuntu kinetic/main amd64 libldap-2.5-0 amd64 2.5.13+dfsg-1ubuntu1 [183 kB]
Get:63 http://archive.ubuntu.com/ubuntu kinetic/main amd64 librtmp1 amd64 2.4+20151223.gitfa8646d.1-2build4 [58.2 kB]
Get:64 http://archive.ubuntu.com/ubuntu kinetic/main amd64 libssh-4 amd64 0.9.6-2build1 [184 kB]
Get:65 http://archive.ubuntu.com/ubuntu kinetic-updates/main amd64 libcurl4 amd64 7.85.0-1ubuntu0.1 [293 kB]
Get:66 http://archive.ubuntu.com/ubuntu kinetic/main amd64 libjsoncpp25 amd64 1.9.5-4 [80.3 kB]
Get:67 http://archive.ubuntu.com/ubuntu kinetic/main amd64 librhash0 amd64 1.4.2-1ubuntu1 [125 kB]
Get:68 http://archive.ubuntu.com/ubuntu kinetic/main amd64 dh-elpa-helper all 2.0.9ubuntu1 [7610 B]
Get:69 http://archive.ubuntu.com/ubuntu kinetic/main amd64 emacsen-common all 3.0.4 [14.9 kB]
Get:70 http://archive.ubuntu.com/ubuntu kinetic/main amd64 cmake-data all 3.24.2-1ubuntu1 [2024 kB]
Get:71 http://archive.ubuntu.com/ubuntu kinetic/main amd64 cmake amd64 3.24.2-1ubuntu1 [8978 kB]
Get:72 http://archive.ubuntu.com/ubuntu kinetic/main amd64 libisl23 amd64 0.25-1 [740 kB]
Get:73 http://archive.ubuntu.com/ubuntu kinetic/main amd64 libmpfr6 amd64 4.1.0-3build3 [1425 kB]
Get:74 http://archive.ubuntu.com/ubuntu kinetic/main amd64 libmpc3 amd64 1.2.1-2build1 [46.9 kB]
Get:75 http://archive.ubuntu.com/ubuntu kinetic/main amd64 cpp-12 amd64 12.2.0-3ubuntu1 [10.6 MB]
Get:76 http://archive.ubuntu.com/ubuntu kinetic/main amd64 cpp amd64 4:12.2.0-1ubuntu1 [27.3 kB]
Get:77 http://archive.ubuntu.com/ubuntu kinetic-updates/main amd64 curl amd64 7.85.0-1ubuntu0.1 [199 kB]
Get:78 http://archive.ubuntu.com/ubuntu kinetic/main amd64 libcc1-0 amd64 12.2.0-3ubuntu1 [46.4 kB]
Get:79 http://archive.ubuntu.com/ubuntu kinetic/main amd64 libgomp1 amd64 12.2.0-3ubuntu1 [125 kB]
Get:80 http://archive.ubuntu.com/ubuntu kinetic/main amd64 libitm1 amd64 12.2.0-3ubuntu1 [29.4 kB]
Get:81 http://archive.ubuntu.com/ubuntu kinetic/main amd64 libatomic1 amd64 12.2.0-3ubuntu1 [10.4 kB]
Get:82 http://archive.ubuntu.com/ubuntu kinetic/main amd64 libasan8 amd64 12.2.0-3ubuntu1 [2433 kB]
Get:83 http://archive.ubuntu.com/ubuntu kinetic/main amd64 liblsan0 amd64 12.2.0-3ubuntu1 [1060 kB]
Get:84 http://archive.ubuntu.com/ubuntu kinetic/main amd64 libtsan2 amd64 12.2.0-3ubuntu1 [2456 kB]
Get:85 http://archive.ubuntu.com/ubuntu kinetic/main amd64 libubsan1 amd64 12.2.0-3ubuntu1 [969 kB]
Get:86 http://archive.ubuntu.com/ubuntu kinetic/main amd64 libquadmath0 amd64 12.2.0-3ubuntu1 [152 kB]
Get:87 http://archive.ubuntu.com/ubuntu kinetic/main amd64 libgcc-12-dev amd64 12.2.0-3ubuntu1 [2577 kB]
Get:88 http://archive.ubuntu.com/ubuntu kinetic/main amd64 gcc-12 amd64 12.2.0-3ubuntu1 [21.4 MB]
Get:89 http://archive.ubuntu.com/ubuntu kinetic/main amd64 gcc amd64 4:12.2.0-1ubuntu1 [5104 B]
Get:90 http://archive.ubuntu.com/ubuntu kinetic/main amd64 libc-dev-bin amd64 2.36-0ubuntu4 [20.0 kB]
Get:91 http://archive.ubuntu.com/ubuntu kinetic-updates/main amd64 linux-libc-dev amd64 5.19.0-26.27 [1345 kB]
Get:92 http://archive.ubuntu.com/ubuntu kinetic/main amd64 libcrypt-dev amd64 1:4.4.28-2 [112 kB]
Get:93 http://archive.ubuntu.com/ubuntu kinetic/main amd64 rpcsvc-proto amd64 1.4.2-0ubuntu6 [68.5 kB]
Get:94 http://archive.ubuntu.com/ubuntu kinetic/main amd64 libtirpc-dev amd64 1.3.3+ds-1 [193 kB]
Get:95 http://archive.ubuntu.com/ubuntu kinetic/main amd64 libnsl-dev amd64 1.3.0-2build2 [71.3 kB]
Get:96 http://archive.ubuntu.com/ubuntu kinetic/main amd64 libc6-dev amd64 2.36-0ubuntu4 [2088 kB]
Get:97 http://archive.ubuntu.com/ubuntu kinetic/main amd64 libstdc++-12-dev amd64 12.2.0-3ubuntu1 [2158 kB]
Get:98 http://archive.ubuntu.com/ubuntu kinetic/main amd64 g++-12 amd64 12.2.0-3ubuntu1 [12.0 MB]
Get:99 http://archive.ubuntu.com/ubuntu kinetic/main amd64 g++ amd64 4:12.2.0-1ubuntu1 [1406 B]
Get:100 http://archive.ubuntu.com/ubuntu kinetic-updates/main amd64 libcurl3-gnutls amd64 7.85.0-1ubuntu0.1 [288 kB]
Get:101 http://archive.ubuntu.com/ubuntu kinetic/main amd64 liberror-perl all 0.17029-1 [26.5 kB]
Get:102 http://archive.ubuntu.com/ubuntu kinetic-updates/main amd64 git-man all 1:2.37.2-1ubuntu1.1 [975 kB]
Get:103 http://archive.ubuntu.com/ubuntu kinetic-updates/main amd64 git amd64 1:2.37.2-1ubuntu1.1 [3296 kB]
Get:104 http://archive.ubuntu.com/ubuntu kinetic/main amd64 libdpkg-perl all 1.21.9ubuntu1 [237 kB]
Get:105 http://archive.ubuntu.com/ubuntu kinetic/universe amd64 libtinfo5 amd64 6.3+20220423-2 [95.4 kB]
Get:106 http://archive.ubuntu.com/ubuntu kinetic/universe amd64 libncurses5 amd64 6.3+20220423-2 [106 kB]
Get:107 http://archive.ubuntu.com/ubuntu kinetic-updates/main amd64 libssl-dev amd64 3.0.5-2ubuntu2 [2375 kB]
Get:108 http://archive.ubuntu.com/ubuntu kinetic/main amd64 make amd64 4.3-4.1build1 [180 kB]
Get:109 http://archive.ubuntu.com/ubuntu kinetic/universe amd64 ninja-build amd64 1.11.0-1 [124 kB]
Get:110 http://archive.ubuntu.com/ubuntu kinetic/main amd64 pkg-config amd64 0.29.2-1ubuntu3 [48.2 kB]
Get:111 http://archive.ubuntu.com/ubuntu kinetic/main amd64 unzip amd64 6.0-27ubuntu1 [174 kB]
Fetched 125 MB in 6s (21.8 MB/s)
Selecting previously unselected package libssl3:amd64.
(Reading database ... 
(Reading database ... 5%
---
Unpacking libjson-c5:amd64 (0.16-1) ...
Selecting previously unselected package libcryptsetup12:amd64.
Preparing to unpack .../04-libcryptsetup12_2%3a2.5.0-2ubuntu1_amd64.deb ...
Unpacking libcryptsetup12:amd64 (2:2.5.0-2ubuntu1) ...
Selecting previously unselected package libfdisk1:amd64.
Preparing to unpack .../05-libfdisk1_2.38-4ubuntu1_amd64.deb ...
Unpacking libfdisk1:amd64 (2.38-4ubuntu1) ...
Preparing to unpack .../06-libkmod2_30+20220630-3ubuntu1_amd64.deb ...
Unpacking libkmod2:amd64 (30+20220630-3ubuntu1) ...
Selecting previously unselected package libapparmor1:amd64.
Preparing to unpack .../07-libapparmor1_3.0.7-1ubuntu2_amd64.deb ...
---
Unpacking libctf-nobfd0:amd64 (2.39-3ubuntu1.1) ...
Selecting previously unselected package libctf0:amd64.
Preparing to unpack .../32-libctf0_2.39-3ubuntu1.1_amd64.deb ...
Unpacking libctf0:amd64 (2.39-3ubuntu1.1) ...
Selecting previously unselected package libgprofng0:amd64.
Preparing to unpack .../33-libgprofng0_2.39-3ubuntu1.1_amd64.deb ...
Unpacking libgprofng0:amd64 (2.39-3ubuntu1.1) ...
Preparing to unpack .../34-binutils-x86-64-linux-gnu_2.39-3ubuntu1.1_amd64.deb ...
Unpacking binutils-x86-64-linux-gnu (2.39-3ubuntu1.1) ...
Selecting previously unselected package binutils.
Preparing to unpack .../35-binutils_2.39-3ubuntu1.1_amd64.deb ...
---
Setting up libatomic1:amd64 (12.2.0-3ubuntu1) ...
Setting up libjsoncpp25:amd64 (1.9.5-4) ...
Setting up libk5crypto3:amd64 (1.20-1) ...
Setting up libsasl2-2:amd64 (2.1.28+dfsg-6ubuntu2) ...
Setting up libfdisk1:amd64 (2.38-4ubuntu1) ...
Setting up libdevmapper1.02.1:amd64 (2:1.02.185-1ubuntu1) ...
Setting up librhash0:amd64 (1.4.2-1ubuntu1) ...
Setting up libcrypt-dev:amd64 (1:4.4.28-2) ...
Setting up libasan8:amd64 (12.2.0-3ubuntu1) ...
---
debconf: (TERM is not set, so the dialog frontend is not usable.)
debconf: falling back to frontend: Readline
Updating certificates in /etc/ssl/certs...
124 added, 0 removed; done.
Setting up libgprofng0:amd64 (2.39-3ubuntu1.1) ...
Setting up libgssapi-krb5-2:amd64 (1.20-1) ...
Setting up libsystemd-shared:amd64 (251.4-1ubuntu7) ...
Setting up libgdbm-compat4:amd64 (1.23-1) ...
Setting up libssh-4:amd64 (0.9.6-2build1) ...
---
Successfully built 8c09855708fc
Successfully tagged rust-ci:latest
Built container sha256:8c09855708fc025ff4b42268172117ed9bfed4c8602de82d64e0b70eb5b4a972
Uploading finished image to https://ci-caches.rust-lang.org/docker/db6c563648d3633919052077e0a3cf4cc8c685e88dd56e949902aee58e2b6a187766cf4ced4e381d1e83d05539c96fd02bd9d6a9a0a78a87f629566cabf527a7
upload failed: - to s3://rust-lang-ci-sccache2/docker/db6c563648d3633919052077e0a3cf4cc8c685e88dd56e949902aee58e2b6a187766cf4ced4e381d1e83d05539c96fd02bd9d6a9a0a78a87f629566cabf527a7 Unable to locate credentials
[CI_JOB_NAME=dist-android]
---
[693/3027] Building CXX object lib/CodeGen/SelectionDAG/CMakeFiles/LLVMSelectionDAG.dir/TargetLowering.cpp.o
[694/3027] Building CXX object lib/CodeGen/GlobalISel/CMakeFiles/LLVMGlobalISel.dir/GlobalISel.cpp.o
[695/3027] Building CXX object lib/CodeGen/SelectionDAG/CMakeFiles/LLVMSelectionDAG.dir/SelectionDAG.cpp.o
In file included from /checkout/src/llvm-project/llvm/lib/CodeGen/SelectionDAG/SelectionDAG.cpp:13:
In constructor ‘llvm::SelectionDAG::DAGUpdateListener::DAGUpdateListener(llvm::SelectionDAG&)’,
    inlined from ‘{anonymous}::RAUWUpdateListener::RAUWUpdateListener(llvm::SelectionDAG&, llvm::SDNode::use_iterator&, llvm::SDNode::use_iterator&)’ at /checkout/src/llvm-project/llvm/lib/CodeGen/SelectionDAG/SelectionDAG.cpp:10076:56,
    inlined from ‘void llvm::SelectionDAG::ReplaceAllUsesWith(llvm::SDNode*, llvm::SDNode*)’ at /checkout/src/llvm-project/llvm/lib/CodeGen/SelectionDAG/SelectionDAG.cpp:10159:44:
/checkout/src/llvm-project/llvm/include/llvm/CodeGen/SelectionDAG.h:306:27: warning: storing the address of local variable ‘Listener’ in ‘*this.llvm::SelectionDAG::UpdateListeners’ [-Wdangling-pointer=]
  306 |       DAG.UpdateListeners = this;
/checkout/src/llvm-project/llvm/lib/CodeGen/SelectionDAG/SelectionDAG.cpp: In member function ‘void llvm::SelectionDAG::ReplaceAllUsesWith(llvm::SDNode*, llvm::SDNode*)’:
/checkout/src/llvm-project/llvm/lib/CodeGen/SelectionDAG/SelectionDAG.cpp:10159:22: note: ‘Listener’ declared here
/checkout/src/llvm-project/llvm/lib/CodeGen/SelectionDAG/SelectionDAG.cpp:10159:22: note: ‘Listener’ declared here
10159 |   RAUWUpdateListener Listener(*this, UI, UE);
      |                      ^~~~~~~~
/checkout/src/llvm-project/llvm/lib/CodeGen/SelectionDAG/SelectionDAG.cpp:10159:22: note: ‘<unknown>’ declared here
In constructor ‘llvm::SelectionDAG::DAGUpdateListener::DAGUpdateListener(llvm::SelectionDAG&)’,
    inlined from ‘{anonymous}::RAUWUpdateListener::RAUWUpdateListener(llvm::SelectionDAG&, llvm::SDNode::use_iterator&, llvm::SDNode::use_iterator&)’ at /checkout/src/llvm-project/llvm/lib/CodeGen/SelectionDAG/SelectionDAG.cpp:10076:56,
    inlined from ‘void llvm::SelectionDAG::ReplaceAllUsesWith(llvm::SDValue, llvm::SDValue)’ at /checkout/src/llvm-project/llvm/lib/CodeGen/SelectionDAG/SelectionDAG.cpp:10103:44:
/checkout/src/llvm-project/llvm/include/llvm/CodeGen/SelectionDAG.h:306:27: warning: storing the address of local variable ‘Listener’ in ‘*this.llvm::SelectionDAG::UpdateListeners’ [-Wdangling-pointer=]
  306 |       DAG.UpdateListeners = this;
/checkout/src/llvm-project/llvm/lib/CodeGen/SelectionDAG/SelectionDAG.cpp: In member function ‘void llvm::SelectionDAG::ReplaceAllUsesWith(llvm::SDValue, llvm::SDValue)’:
/checkout/src/llvm-project/llvm/lib/CodeGen/SelectionDAG/SelectionDAG.cpp:10103:22: note: ‘Listener’ declared here
/checkout/src/llvm-project/llvm/lib/CodeGen/SelectionDAG/SelectionDAG.cpp:10103:22: note: ‘Listener’ declared here
10103 |   RAUWUpdateListener Listener(*this, UI, UE);
      |                      ^~~~~~~~
/checkout/src/llvm-project/llvm/lib/CodeGen/SelectionDAG/SelectionDAG.cpp:10103:22: note: ‘<unknown>’ declared here
In constructor ‘llvm::SelectionDAG::DAGUpdateListener::DAGUpdateListener(llvm::SelectionDAG&)’,
    inlined from ‘{anonymous}::RAUWUpdateListener::RAUWUpdateListener(llvm::SelectionDAG&, llvm::SDNode::use_iterator&, llvm::SDNode::use_iterator&)’ at /checkout/src/llvm-project/llvm/lib/CodeGen/SelectionDAG/SelectionDAG.cpp:10076:56,
    inlined from ‘void llvm::SelectionDAG::ReplaceAllUsesOfValueWith(llvm::SDValue, llvm::SDValue)’ at /checkout/src/llvm-project/llvm/lib/CodeGen/SelectionDAG/SelectionDAG.cpp:10257:44:
/checkout/src/llvm-project/llvm/include/llvm/CodeGen/SelectionDAG.h:306:27: warning: storing the address of local variable ‘Listener’ in ‘*this.llvm::SelectionDAG::UpdateListeners’ [-Wdangling-pointer=]
  306 |       DAG.UpdateListeners = this;
      |       ~~~~~~~~~~~~~~~~~~~~^~~~~~
/checkout/src/llvm-project/llvm/lib/CodeGen/SelectionDAG/SelectionDAG.cpp: In member function ‘void llvm::SelectionDAG::ReplaceAllUsesOfValueWith(llvm::SDValue, llvm::SDValue)’:
/checkout/src/llvm-project/llvm/lib/CodeGen/SelectionDAG/SelectionDAG.cpp:10257:22: note: ‘Listener’ declared here
10257 |   RAUWUpdateListener Listener(*this, UI, UE);
      |                      ^~~~~~~~
/checkout/src/llvm-project/llvm/lib/CodeGen/SelectionDAG/SelectionDAG.cpp:10257:22: note: ‘<unknown>’ declared here
In constructor ‘llvm::SelectionDAG::DAGUpdateListener::DAGUpdateListener(llvm::SelectionDAG&)’,
    inlined from ‘{anonymous}::RAUOVWUpdateListener::RAUOVWUpdateListener(llvm::SelectionDAG&, llvm::SmallVector<{anonymous}::UseMemo, 4>&)’ at /checkout/src/llvm-project/llvm/lib/CodeGen/SelectionDAG/SelectionDAG.cpp:10331:54,
    inlined from ‘void llvm::SelectionDAG::ReplaceAllUsesOfValuesWith(const llvm::SDValue*, const llvm::SDValue*, unsigned int)’ at /checkout/src/llvm-project/llvm/lib/CodeGen/SelectionDAG/SelectionDAG.cpp:10425:44:
/checkout/src/llvm-project/llvm/include/llvm/CodeGen/SelectionDAG.h:306:27: warning: storing the address of local variable ‘Listener’ in ‘*this.llvm::SelectionDAG::UpdateListeners’ [-Wdangling-pointer=]
  306 |       DAG.UpdateListeners = this;
      |       ~~~~~~~~~~~~~~~~~~~~^~~~~~
/checkout/src/llvm-project/llvm/lib/CodeGen/SelectionDAG/SelectionDAG.cpp: In member function ‘void llvm::SelectionDAG::ReplaceAllUsesOfValuesWith(const llvm::SDValue*, const llvm::SDValue*, unsigned int)’:
/checkout/src/llvm-project/llvm/lib/CodeGen/SelectionDAG/SelectionDAG.cpp:10425:24: note: ‘Listener’ declared here
10425 |   RAUOVWUpdateListener Listener(*this, Uses);
      |                        ^~~~~~~~
/checkout/src/llvm-project/llvm/lib/CodeGen/SelectionDAG/SelectionDAG.cpp:10425:24: note: ‘<unknown>’ declared here
In constructor ‘llvm::SelectionDAG::DAGUpdateListener::DAGUpdateListener(llvm::SelectionDAG&)’,
    inlined from ‘{anonymous}::RAUWUpdateListener::RAUWUpdateListener(llvm::SelectionDAG&, llvm::SDNode::use_iterator&, llvm::SDNode::use_iterator&)’ at /checkout/src/llvm-project/llvm/lib/CodeGen/SelectionDAG/SelectionDAG.cpp:10076:56,
    inlined from ‘void llvm::SelectionDAG::ReplaceAllUsesWith(llvm::SDNode*, const llvm::SDValue*)’ at /checkout/src/llvm-project/llvm/lib/CodeGen/SelectionDAG/SelectionDAG.cpp:10204:44:
/checkout/src/llvm-project/llvm/include/llvm/CodeGen/SelectionDAG.h:306:27: warning: storing the address of local variable ‘Listener’ in ‘*this.llvm::SelectionDAG::UpdateListeners’ [-Wdangling-pointer=]
  306 |       DAG.UpdateListeners = this;
/checkout/src/llvm-project/llvm/lib/CodeGen/SelectionDAG/SelectionDAG.cpp: In member function ‘void llvm::SelectionDAG::ReplaceAllUsesWith(llvm::SDNode*, const llvm::SDValue*)’:
/checkout/src/llvm-project/llvm/lib/CodeGen/SelectionDAG/SelectionDAG.cpp:10204:22: note: ‘Listener’ declared here
/checkout/src/llvm-project/llvm/lib/CodeGen/SelectionDAG/SelectionDAG.cpp:10204:22: note: ‘Listener’ declared here
10204 |   RAUWUpdateListener Listener(*this, UI, UE);
      |                      ^~~~~~~~
/checkout/src/llvm-project/llvm/lib/CodeGen/SelectionDAG/SelectionDAG.cpp:10204:22: note: ‘<unknown>’ declared here
[697/3027] Building CXX object lib/CodeGen/AsmPrinter/CMakeFiles/LLVMAsmPrinter.dir/DwarfUnit.cpp.o
[698/3027] Building CXX object lib/CodeGen/AsmPrinter/CMakeFiles/LLVMAsmPrinter.dir/AsmPrinter.cpp.o
[699/3027] Building CXX object lib/CodeGen/GlobalISel/CMakeFiles/LLVMGlobalISel.dir/CSEInfo.cpp.o
[700/3027] Building CXX object lib/CodeGen/AsmPrinter/CMakeFiles/LLVMAsmPrinter.dir/WinException.cpp.o
---
/checkout/src/llvm-project/llvm/include/llvm/Support/Endian.h: In member function ‘void llvm::RuntimeDyldELF::resolveAArch64Relocation(const llvm::SectionEntry&, uint64_t, uint64_t, uint32_t, int64_t)’:
/checkout/src/llvm-project/llvm/include/llvm/Support/Endian.h:99:9: warning: writing 8 bytes into a region of size 4 [-Wstringop-overflow=]
   99 |   memcpy(LLVM_ASSUME_ALIGNED(
      |         ^
/checkout/src/llvm-project/llvm/include/llvm/Support/Endian.h:247:25: note: destination object ‘llvm::support::detail::packed_endian_specific_integral<unsigned int, llvm::support::little, 1>::<unnamed struct>::buffer’ of size 4
  247 |     alignas(ALIGN) char buffer[sizeof(value_type)];
/checkout/src/llvm-project/llvm/include/llvm/Support/Endian.h:99:9: warning: writing 8 bytes into a region of size 4 [-Wstringop-overflow=]
   99 |   memcpy(LLVM_ASSUME_ALIGNED(
      |         ^
      |         ^
/checkout/src/llvm-project/llvm/include/llvm/Support/Endian.h:247:25: note: destination object ‘llvm::support::detail::packed_endian_specific_integral<unsigned int, llvm::support::little, 1>::<unnamed struct>::buffer’ of size 4
  247 |     alignas(ALIGN) char buffer[sizeof(value_type)];
/checkout/src/llvm-project/llvm/include/llvm/Support/Endian.h:99:9: warning: writing 8 bytes into a region of size 4 [-Wstringop-overflow=]
   99 |   memcpy(LLVM_ASSUME_ALIGNED(
      |         ^
      |         ^
/checkout/src/llvm-project/llvm/include/llvm/Support/Endian.h:247:25: note: destination object ‘llvm::support::detail::packed_endian_specific_integral<unsigned int, llvm::support::little, 1>::<unnamed struct>::buffer’ of size 4
  247 |     alignas(ALIGN) char buffer[sizeof(value_type)];
/checkout/src/llvm-project/llvm/include/llvm/Support/Endian.h:99:9: warning: writing 8 bytes into a region of size 4 [-Wstringop-overflow=]
   99 |   memcpy(LLVM_ASSUME_ALIGNED(
      |         ^
      |         ^
/checkout/src/llvm-project/llvm/include/llvm/Support/Endian.h:247:25: note: destination object ‘llvm::support::detail::packed_endian_specific_integral<unsigned int, llvm::support::little, 1>::<unnamed struct>::buffer’ of size 4
  247 |     alignas(ALIGN) char buffer[sizeof(value_type)];
/checkout/src/llvm-project/llvm/include/llvm/Support/Endian.h: In member function ‘void llvm::RuntimeDyldELF::resolveBPFRelocation(const llvm::SectionEntry&, uint64_t, uint64_t, uint32_t, int64_t)’:
/checkout/src/llvm-project/llvm/include/llvm/Support/Endian.h:99:9: warning: writing 8 bytes into a region of size 4 [-Wstringop-overflow=]
   99 |   memcpy(LLVM_ASSUME_ALIGNED(
      |         ^
      |         ^
/checkout/src/llvm-project/llvm/include/llvm/Support/Endian.h:247:25: note: destination object ‘llvm::support::detail::packed_endian_specific_integral<unsigned int, llvm::support::little, 1>::<unnamed struct>::buffer’ of size 4
  247 |     alignas(ALIGN) char buffer[sizeof(value_type)];
/checkout/src/llvm-project/llvm/include/llvm/Support/Endian.h:99:9: warning: writing 8 bytes into a region of size 4 [-Wstringop-overflow=]
   99 |   memcpy(LLVM_ASSUME_ALIGNED(
      |         ^
      |         ^
/checkout/src/llvm-project/llvm/include/llvm/Support/Endian.h:247:25: note: destination object ‘llvm::support::detail::packed_endian_specific_integral<unsigned int, llvm::support::little, 1>::<unnamed struct>::buffer’ of size 4
  247 |     alignas(ALIGN) char buffer[sizeof(value_type)];
[1708/3027] Building AVRGenAsmWriter.inc...
[1709/3027] Building X86GenDisassemblerTables.inc...
[1710/3027] Building AVRGenCallingConv.inc...
[1711/3027] Building AVRGenDisassemblerTables.inc...
---
[1930/3027] Building CXX object lib/Target/ARM/Disassembler/CMakeFiles/LLVMARMDisassembler.dir/ARMDisassembler.cpp.o
[1931/3027] Linking CXX static library lib/libLLVMARMDisassembler.a
[1932/3027] Building CXX object lib/Target/Hexagon/CMakeFiles/LLVMHexagonCodeGen.dir/HexagonBranchRelaxation.cpp.o
[1933/3027] Building CXX object lib/Target/Hexagon/CMakeFiles/LLVMHexagonCodeGen.dir/BitTracker.cpp.o
In file included from /checkout/src/llvm-project/llvm/lib/Target/Hexagon/BitTracker.cpp:55:
/checkout/src/llvm-project/llvm/lib/Target/Hexagon/BitTracker.h: In constructor ‘llvm::BitTracker::UseQueueType::UseQueueType()’:
/checkout/src/llvm-project/llvm/lib/Target/Hexagon/BitTracker.h:75:27: warning: member ‘llvm::BitTracker::UseQueueType::Dist’ is used uninitialized [-Wuninitialized]
   75 |     UseQueueType() : Uses(Dist) {}
[1934/3027] Building CXX object lib/Target/BPF/CMakeFiles/LLVMBPFCodeGen.dir/BTFDebug.cpp.o
[1935/3027] Building CXX object lib/Target/Hexagon/CMakeFiles/LLVMHexagonCodeGen.dir/HexagonBlockRanges.cpp.o
[1936/3027] Building CXX object lib/Target/Hexagon/CMakeFiles/LLVMHexagonCodeGen.dir/HexagonAsmPrinter.cpp.o
[1937/3027] Building CXX object lib/Target/Hexagon/CMakeFiles/LLVMHexagonCodeGen.dir/HexagonBitTracker.cpp.o
[1937/3027] Building CXX object lib/Target/Hexagon/CMakeFiles/LLVMHexagonCodeGen.dir/HexagonBitTracker.cpp.o
In file included from /checkout/src/llvm-project/llvm/lib/Target/Hexagon/HexagonBitTracker.h:12,
                 from /checkout/src/llvm-project/llvm/lib/Target/Hexagon/HexagonBitTracker.cpp:9:
/checkout/src/llvm-project/llvm/lib/Target/Hexagon/BitTracker.h: In constructor ‘llvm::BitTracker::UseQueueType::UseQueueType()’:
/checkout/src/llvm-project/llvm/lib/Target/Hexagon/BitTracker.h:75:27: warning: member ‘llvm::BitTracker::UseQueueType::Dist’ is used uninitialized [-Wuninitialized]
   75 |     UseQueueType() : Uses(Dist) {}
[1938/3027] Building CXX object lib/Target/Hexagon/CMakeFiles/LLVMHexagonCodeGen.dir/HexagonCommonGEP.cpp.o
[1939/3027] Building CXX object lib/Target/Hexagon/CMakeFiles/LLVMHexagonCodeGen.dir/HexagonCopyToCombine.cpp.o
[1940/3027] Building CXX object lib/Target/Hexagon/CMakeFiles/LLVMHexagonCodeGen.dir/HexagonEarlyIfConv.cpp.o
[1941/3027] Building CXX object lib/Target/Hexagon/CMakeFiles/LLVMHexagonCodeGen.dir/HexagonBitSimplify.cpp.o
[1941/3027] Building CXX object lib/Target/Hexagon/CMakeFiles/LLVMHexagonCodeGen.dir/HexagonBitSimplify.cpp.o
In file included from /checkout/src/llvm-project/llvm/lib/Target/Hexagon/HexagonBitSimplify.cpp:9:
/checkout/src/llvm-project/llvm/lib/Target/Hexagon/BitTracker.h: In constructor ‘llvm::BitTracker::UseQueueType::UseQueueType()’:
/checkout/src/llvm-project/llvm/lib/Target/Hexagon/BitTracker.h:75:27: warning: member ‘llvm::BitTracker::UseQueueType::Dist’ is used uninitialized [-Wuninitialized]
   75 |     UseQueueType() : Uses(Dist) {}
[1942/3027] Building CXX object lib/Target/Hexagon/CMakeFiles/LLVMHexagonCodeGen.dir/HexagonExpandCondsets.cpp.o
[1943/3027] Building CXX object lib/Target/Hexagon/CMakeFiles/LLVMHexagonCodeGen.dir/HexagonConstPropagation.cpp.o
[1944/3027] Building CXX object lib/Target/BPF/CMakeFiles/LLVMBPFCodeGen.dir/BPFTargetMachine.cpp.o
[1945/3027] Building CXX object lib/Target/Hexagon/CMakeFiles/LLVMHexagonCodeGen.dir/HexagonConstExtenders.cpp.o
---
[1953/3027] Building CXX object lib/Target/Hexagon/CMakeFiles/LLVMHexagonCodeGen.dir/HexagonOptimizeSZextends.cpp.o
[1954/3027] Building CXX object lib/Target/Hexagon/CMakeFiles/LLVMHexagonCodeGen.dir/HexagonMachineScheduler.cpp.o
[1955/3027] Building CXX object lib/Target/Hexagon/CMakeFiles/LLVMHexagonCodeGen.dir/HexagonGenInsert.cpp.o
In file included from /checkout/src/llvm-project/llvm/lib/Target/Hexagon/HexagonGenInsert.cpp:9:
/checkout/src/llvm-project/llvm/lib/Target/Hexagon/BitTracker.h: In constructor ‘llvm::BitTracker::UseQueueType::UseQueueType()’:
/checkout/src/llvm-project/llvm/lib/Target/Hexagon/BitTracker.h:75:27: warning: member ‘llvm::BitTracker::UseQueueType::Dist’ is used uninitialized [-Wuninitialized]
   75 |     UseQueueType() : Uses(Dist) {}
[1956/3027] Building CXX object lib/Target/Hexagon/CMakeFiles/LLVMHexagonCodeGen.dir/HexagonMCInstLower.cpp.o
[1957/3027] Building CXX object lib/Target/Hexagon/CMakeFiles/LLVMHexagonCodeGen.dir/HexagonNewValueJump.cpp.o
[1958/3027] Building CXX object lib/Target/Hexagon/CMakeFiles/LLVMHexagonCodeGen.dir/HexagonFrameLowering.cpp.o
[1959/3027] Building CXX object lib/Target/Hexagon/CMakeFiles/LLVMHexagonCodeGen.dir/HexagonISelLoweringHVX.cpp.o
---
                 from /checkout/src/llvm-project/llvm/lib/Target/Hexagon/HexagonDepArch.h:15,
                 from /checkout/src/llvm-project/llvm/lib/Target/Hexagon/HexagonSubtarget.h:16,
                 from /checkout/src/llvm-project/llvm/lib/Target/Hexagon/HexagonISelDAGToDAG.h:15,
                 from /checkout/src/llvm-project/llvm/lib/Target/Hexagon/HexagonISelDAGToDAGHVX.cpp:10:
In static member function ‘static _Tp* std::__copy_move<_IsMove, true, std::random_access_iterator_tag>::__copy_m(const _Tp*, const _Tp*, _Tp*) [with _Tp = {anonymous}::OpRef; bool _IsMove = false]’,
    inlined from ‘_OI std::__copy_move_a2(_II, _II, _OI) [with bool _IsMove = false; _II = const {anonymous}::OpRef*; _OI = {anonymous}::OpRef*]’ at /usr/include/c++/12/bits/stl_algobase.h:495:30,
    inlined from ‘_OI std::__copy_move_a1(_II, _II, _OI) [with bool _IsMove = false; _II = const {anonymous}::OpRef*; _OI = {anonymous}::OpRef*]’ at /usr/include/c++/12/bits/stl_algobase.h:522:42,
    inlined from ‘_OI std::__copy_move_a(_II, _II, _OI) [with bool _IsMove = false; _II = const {anonymous}::OpRef*; _OI = {anonymous}::OpRef*]’ at /usr/include/c++/12/bits/stl_algobase.h:529:31,
    inlined from ‘_OI std::copy(_II, _II, _OI) [with _II = const {anonymous}::OpRef*; _OI = {anonymous}::OpRef*]’ at /usr/include/c++/12/bits/stl_algobase.h:620:7,
    inlined from ‘void std::vector<_Tp, _Alloc>::_M_assign_aux(_ForwardIterator, _ForwardIterator, std::forward_iterator_tag) [with _ForwardIterator = const {anonymous}::OpRef*; _Tp = {anonymous}::OpRef; _Alloc = std::allocator<{anonymous}::OpRef>]’ at /usr/include/c++/12/bits/vector.tcc:330:19:
/usr/include/c++/12/bits/stl_algobase.h:431:30: warning: argument 1 null where non-null expected [-Wnonnull]
  431 |             __builtin_memmove(__result, __first, sizeof(_Tp) * _Num);
      |             ~~~~~~~~~~~~~~~~~^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/usr/include/c++/12/bits/stl_algobase.h:431:30: note: in a call to built-in function ‘void* __builtin_memmove(void*, const void*, long unsigned int)’
In static member function ‘static _Tp* std::__copy_move<_IsMove, true, std::random_access_iterator_tag>::__copy_m(const _Tp*, const _Tp*, _Tp*) [with _Tp = {anonymous}::OpRef; bool _IsMove = false]’,
    inlined from ‘_OI std::__copy_move_a2(_II, _II, _OI) [with bool _IsMove = false; _II = const {anonymous}::OpRef*; _OI = {anonymous}::OpRef*]’ at /usr/include/c++/12/bits/stl_algobase.h:495:30,
    inlined from ‘_OI std::__copy_move_a1(_II, _II, _OI) [with bool _IsMove = false; _II = const {anonymous}::OpRef*; _OI = {anonymous}::OpRef*]’ at /usr/include/c++/12/bits/stl_algobase.h:522:42,
    inlined from ‘_OI std::__copy_move_a(_II, _II, _OI) [with bool _IsMove = false; _II = const {anonymous}::OpRef*; _OI = {anonymous}::OpRef*]’ at /usr/include/c++/12/bits/stl_algobase.h:529:31,
    inlined from ‘_OI std::copy(_II, _II, _OI) [with _II = const {anonymous}::OpRef*; _OI = {anonymous}::OpRef*]’ at /usr/include/c++/12/bits/stl_algobase.h:620:7,
    inlined from ‘void std::vector<_Tp, _Alloc>::_M_assign_aux(_ForwardIterator, _ForwardIterator, std::forward_iterator_tag) [with _ForwardIterator = const {anonymous}::OpRef*; _Tp = {anonymous}::OpRef; _Alloc = std::allocator<{anonymous}::OpRef>]’ at /usr/include/c++/12/bits/vector.tcc:335:15:
/usr/include/c++/12/bits/stl_algobase.h:431:30: warning: argument 1 null where non-null expected [-Wnonnull]
  431 |             __builtin_memmove(__result, __first, sizeof(_Tp) * _Num);
      |             ~~~~~~~~~~~~~~~~~^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/usr/include/c++/12/bits/stl_algobase.h:431:30: note: in a call to built-in function ‘void* __builtin_memmove(void*, const void*, long unsigned int)’
[1962/3027] Building CXX object lib/Target/Hexagon/CMakeFiles/LLVMHexagonCodeGen.dir/HexagonPeephole.cpp.o
[1963/3027] Building CXX object lib/Target/Hexagon/CMakeFiles/LLVMHexagonCodeGen.dir/HexagonISelDAGToDAG.cpp.o
[1964/3027] Building CXX object lib/Target/Hexagon/CMakeFiles/LLVMHexagonCodeGen.dir/HexagonInstrInfo.cpp.o
[1965/3027] Building CXX object lib/Target/Hexagon/CMakeFiles/LLVMHexagonCodeGen.dir/HexagonRegisterInfo.cpp.o
---
  230 |     if (!hasAttributes())
      | 
/checkout/src/llvm-project/llvm/include/llvm/IR/GlobalVariable.h:230: note: adding ‘-flarge-source-files’ will allow for more column-tracking support, at the expense of compilation time and memory
[2369/3027] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/X86FloatingPoint.cpp.o
In member function ‘void {anonymous}::FPS::finishBlockStack()’,
    inlined from ‘bool {anonymous}::FPS::processBasicBlock(llvm::MachineFunction&, llvm::MachineBasicBlock&)’ at /checkout/src/llvm-project/llvm/lib/Target/X86/X86FloatingPoint.cpp:499:19:
/checkout/src/llvm-project/llvm/lib/Target/X86/X86FloatingPoint.cpp:573:26: warning: writing 1 byte into a region of size 0 [-Wstringop-overflow=]
  573 |       Bundle.FixStack[i] = getStackEntry(i);
/checkout/src/llvm-project/llvm/lib/Target/X86/X86FloatingPoint.cpp: In function ‘bool {anonymous}::FPS::processBasicBlock(llvm::MachineFunction&, llvm::MachineBasicBlock&)’:
/checkout/src/llvm-project/llvm/lib/Target/X86/X86FloatingPoint.cpp: In function ‘bool {anonymous}::FPS::processBasicBlock(llvm::MachineFunction&, llvm::MachineBasicBlock&)’:
/checkout/src/llvm-project/llvm/lib/Target/X86/X86FloatingPoint.cpp:110:21: note: at offset 8 into destination object ‘{anonymous}::FPS::LiveBundle::FixStack’ of size 8
  110 |       unsigned char FixStack[8];
[2370/3027] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/X86AvoidStoreForwardingBlocks.cpp.o
[2371/3027] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/X86FlagsCopyLowering.cpp.o
[2372/3027] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/X86InstructionSelector.cpp.o
In file included from /checkout/src/llvm-project/llvm/lib/Target/X86/X86InstructionSelector.cpp:150:
---
[2661/3027] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/X86ISelLowering.cpp.o
In file included from /checkout/src/llvm-project/llvm/include/llvm/CodeGen/TargetLowering.h:35,
                 from /checkout/src/llvm-project/llvm/lib/Target/X86/X86ISelLowering.h:18,
                 from /checkout/src/llvm-project/llvm/lib/Target/X86/X86ISelLowering.cpp:14:
In member function ‘void llvm::SelectionDAG::setFlagInserter(FlagInserter*)’,
    inlined from ‘llvm::SelectionDAG::FlagInserter::FlagInserter(llvm::SelectionDAG&, llvm::SDNodeFlags)’ at /checkout/src/llvm-project/llvm/include/llvm/CodeGen/SelectionDAG.h:350:27,
    inlined from ‘llvm::SDValue llvm::X86TargetLowering::LowerINTRINSIC_WO_CHAIN(llvm::SDValue, llvm::SelectionDAG&) const’ at /checkout/src/llvm-project/llvm/lib/Target/X86/X86ISelLowering.cpp:26511:63:
/checkout/src/llvm-project/llvm/include/llvm/CodeGen/SelectionDAG.h:468:53: warning: storing the address of local variable ‘FlagsInserter’ in ‘*DAG.llvm::SelectionDAG::Inserter’ [-Wdangling-pointer=]
  468 |   void setFlagInserter(FlagInserter *FI) { Inserter = FI; }
/checkout/src/llvm-project/llvm/lib/Target/X86/X86ISelLowering.cpp: In member function ‘llvm::SDValue llvm::X86TargetLowering::LowerINTRINSIC_WO_CHAIN(llvm::SDValue, llvm::SelectionDAG&) const’:
/checkout/src/llvm-project/llvm/lib/Target/X86/X86ISelLowering.cpp:26511:30: note: ‘FlagsInserter’ declared here
/checkout/src/llvm-project/llvm/lib/Target/X86/X86ISelLowering.cpp:26511:30: note: ‘FlagsInserter’ declared here
26511 |   SelectionDAG::FlagInserter FlagsInserter(DAG, Op->getFlags());
/checkout/src/llvm-project/llvm/lib/Target/X86/X86ISelLowering.cpp:26511:30: note: ‘DAG’ declared here
[2662/3027] Linking CXX static library lib/libLLVMX86CodeGen.a
[2663/3027] Building CXX object tools/llvm-cov/CMakeFiles/llvm-cov.dir/gcov.cpp.o
[2664/3027] Linking CXX executable bin/llvm-cat
---
  IMAGE: dist-android
  AWS_ACCESS_KEY_ID: 
  AWS_SECRET_ACCESS_KEY: 
##[endgroup]
cp: cannot stat 'obj/build/metrics.json': No such file or directory
##[error]Process completed with exit code 1.
