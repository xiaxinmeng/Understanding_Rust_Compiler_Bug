plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 40336865fe7d4a01139a3336639c6971647e885c and d32415a030e2b80ec662393f22f37e96a98f7127
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
Removing intermediate container e6da3b629bae
 ---> 9898c327e9bd
Step 3/14 : RUN apt-get update && apt-get install -y --no-install-recommends   g++   make   ninja-build   file   curl   ca-certificates   python3   git   cmake   libssl-dev   sudo   xz-utils   tidy
 ---> Running in 63097580139f
Get:1 http://archive.ubuntu.com/ubuntu jammy InRelease [270 kB]
Get:2 http://security.ubuntu.com/ubuntu jammy-security InRelease [110 kB]
Get:3 http://archive.ubuntu.com/ubuntu jammy-updates InRelease [114 kB]
Get:4 http://archive.ubuntu.com/ubuntu jammy-backports InRelease [99.8 kB]
Get:5 http://security.ubuntu.com/ubuntu jammy-security/universe amd64 Packages [131 kB]
Get:6 http://security.ubuntu.com/ubuntu jammy-security/restricted amd64 Packages [313 kB]
Get:7 http://security.ubuntu.com/ubuntu jammy-security/multiverse amd64 Packages [4644 B]
Get:8 http://security.ubuntu.com/ubuntu jammy-security/main amd64 Packages [332 kB]
Get:9 http://archive.ubuntu.com/ubuntu jammy/universe amd64 Packages [17.5 MB]
Get:10 http://archive.ubuntu.com/ubuntu jammy/restricted amd64 Packages [164 kB]
Get:11 http://archive.ubuntu.com/ubuntu jammy/multiverse amd64 Packages [266 kB]
Get:12 http://archive.ubuntu.com/ubuntu jammy/main amd64 Packages [1792 kB]
Get:13 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 Packages [633 kB]
Get:14 http://archive.ubuntu.com/ubuntu jammy-updates/universe amd64 Packages [261 kB]
Get:15 http://archive.ubuntu.com/ubuntu jammy-updates/multiverse amd64 Packages [7791 B]
Get:16 http://archive.ubuntu.com/ubuntu jammy-updates/restricted amd64 Packages [354 kB]
Get:17 http://archive.ubuntu.com/ubuntu jammy-backports/universe amd64 Packages [5814 B]
Reading package lists...
Reading package lists...
Building dependency tree...
Reading state information...
Reading state information...
The following additional packages will be installed:
  binutils binutils-common binutils-x86-64-linux-gnu cmake-data cpp cpp-11
  dh-elpa-helper emacsen-common g++-11 gcc gcc-11 gcc-11-base git-man
  libc6-dev libcc1-0 libcrypt-dev libctf-nobfd0 libctf0 libcurl3-gnutls
  libcurl4 liberror-perl libexpat1 libgcc-11-dev libgdbm-compat4 libgdbm6
  libgomp1 libicu70 libisl23 libitm1 libjsoncpp25 libldap-2.5-0 liblsan0
  libmagic-mgc libmagic1 libmpc3 libmpdec3 libmpfr6 libnghttp2-14 libnsl-dev
  libmagic-mgc libmagic1 libmpc3 libmpdec3 libmpfr6 libnghttp2-14 libnsl-dev
  libperl5.34 libpsl5 libpython3-stdlib libpython3.10-minimal
  libpython3.10-stdlib libquadmath0 libreadline8 librhash0 librtmp1 libsasl2-2
  libsasl2-modules-db libsqlite3-0 libssh-4 libstdc++-11-dev libtidy5deb1
  libtirpc-dev libtsan0 libubsan1 libuv1 libxml2 linux-libc-dev media-types
  openssl perl perl-modules-5.34 python3-minimal python3.10 python3.10-minimal
  readline-common rpcsvc-proto
  binutils-doc cmake-doc cmake-format cpp-doc gcc-11-locales g++-multilib
  g++-11-multilib gcc-11-doc gcc-multilib manpages-dev autoconf automake
  libtool flex bison gdb gcc-doc gcc-11-multilib gettext-base git-daemon-run
  | git-daemon-sysvinit git-doc git-email git-gui gitk gitweb git-cvs
  | git-daemon-sysvinit git-doc git-email git-gui gitk gitweb git-cvs
  git-mediawiki git-svn lrzip glibc-doc gdbm-l10n libssl-doc libstdc++-11-doc
  make-doc perl-doc libterm-readline-gnu-perl | libterm-readline-perl-perl
  libtap-harness-archive-perl python3-doc python3-tk python3-venv
Recommended packages:
  patch less ssh-client manpages manpages-dev libc-devtools libldap-common
  publicsuffix libsasl2-modules netbase
The following NEW packages will be installed:
The following NEW packages will be installed:
  binutils binutils-common binutils-x86-64-linux-gnu ca-certificates cmake
  cmake-data cpp cpp-11 curl dh-elpa-helper emacsen-common file g++ g++-11 gcc
  libbrotli1 libc-dev-bin libc6-dev libcc1-0 libcrypt-dev libctf-nobfd0
  libctf0 libcurl3-gnutls libcurl4 liberror-perl libexpat1 libgcc-11-dev
  libgdbm-compat4 libgdbm6 libgomp1 libicu70 libisl23 libitm1 libjsoncpp25
  libldap-2.5-0 liblsan0 libmagic-mgc libmagic1 libmpc3 libmpdec3 libmpfr6
  libldap-2.5-0 liblsan0 libmagic-mgc libmagic1 libmpc3 libmpdec3 libmpfr6
  libnghttp2-14 libnsl-dev libperl5.34 libpsl5 libpython3-stdlib
  libpython3.10-minimal libpython3.10-stdlib libquadmath0 libreadline8
  librhash0 librtmp1 libsasl2-2 libsasl2-modules-db libsqlite3-0 libssh-4
  libssl-dev libstdc++-11-dev libtidy5deb1 libtirpc-dev libtsan0 libubsan1
  libuv1 libxml2 linux-libc-dev make media-types ninja-build openssl perl
  perl-modules-5.34 python3 python3-minimal python3.10 python3.10-minimal
  readline-common rpcsvc-proto sudo tidy xz-utils
Need to get 107 MB of archives.
After this operation, 403 MB of additional disk space will be used.
After this operation, 403 MB of additional disk space will be used.
Get:1 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libpython3.10-minimal amd64 3.10.4-3ubuntu0.1 [809 kB]
Get:2 http://archive.ubuntu.com/ubuntu jammy/main amd64 libexpat1 amd64 2.4.7-1 [90.7 kB]
Get:3 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 python3.10-minimal amd64 3.10.4-3ubuntu0.1 [2258 kB]
Get:4 http://archive.ubuntu.com/ubuntu jammy/main amd64 python3-minimal amd64 3.10.4-0ubuntu2 [24.4 kB]
Get:5 http://archive.ubuntu.com/ubuntu jammy/main amd64 media-types all 7.0.0 [25.5 kB]
Get:6 http://archive.ubuntu.com/ubuntu jammy/main amd64 libmpdec3 amd64 2.5.1-2build2 [86.8 kB]
Get:7 http://archive.ubuntu.com/ubuntu jammy/main amd64 readline-common all 8.1.2-1 [53.5 kB]
Get:8 http://archive.ubuntu.com/ubuntu jammy/main amd64 libreadline8 amd64 8.1.2-1 [153 kB]
Get:9 http://archive.ubuntu.com/ubuntu jammy/main amd64 libsqlite3-0 amd64 3.37.2-2 [643 kB]
Get:10 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libpython3.10-stdlib amd64 3.10.4-3ubuntu0.1 [1830 kB]
Get:11 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 python3.10 amd64 3.10.4-3ubuntu0.1 [488 kB]
Get:12 http://archive.ubuntu.com/ubuntu jammy/main amd64 libpython3-stdlib amd64 3.10.4-0ubuntu2 [6990 B]
Get:13 http://archive.ubuntu.com/ubuntu jammy/main amd64 python3 amd64 3.10.4-0ubuntu2 [22.8 kB]
Get:14 http://archive.ubuntu.com/ubuntu jammy/main amd64 perl-modules-5.34 all 5.34.0-3ubuntu1 [2975 kB]
Get:15 http://archive.ubuntu.com/ubuntu jammy/main amd64 libgdbm6 amd64 1.23-1 [33.9 kB]
Get:16 http://archive.ubuntu.com/ubuntu jammy/main amd64 libgdbm-compat4 amd64 1.23-1 [6606 B]
Get:17 http://archive.ubuntu.com/ubuntu jammy/main amd64 libperl5.34 amd64 5.34.0-3ubuntu1 [4809 kB]
Get:18 http://archive.ubuntu.com/ubuntu jammy/main amd64 perl amd64 5.34.0-3ubuntu1 [232 kB]
Get:19 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 openssl amd64 3.0.2-0ubuntu1.6 [1184 kB]
Get:20 http://archive.ubuntu.com/ubuntu jammy/main amd64 ca-certificates all 20211016 [148 kB]
Get:21 http://archive.ubuntu.com/ubuntu jammy/main amd64 libicu70 amd64 70.1-2 [10.6 MB]
Get:22 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libxml2 amd64 2.9.13+dfsg-1ubuntu0.1 [763 kB]
Get:23 http://archive.ubuntu.com/ubuntu jammy/main amd64 sudo amd64 1.9.9-1ubuntu2 [820 kB]
Get:24 http://archive.ubuntu.com/ubuntu jammy/main amd64 libmagic-mgc amd64 1:5.41-3 [257 kB]
Get:25 http://archive.ubuntu.com/ubuntu jammy/main amd64 libmagic1 amd64 1:5.41-3 [87.2 kB]
Get:26 http://archive.ubuntu.com/ubuntu jammy/main amd64 file amd64 1:5.41-3 [21.5 kB]
Get:27 http://archive.ubuntu.com/ubuntu jammy/main amd64 libnghttp2-14 amd64 1.43.0-1build3 [76.3 kB]
Get:28 http://archive.ubuntu.com/ubuntu jammy/main amd64 libpsl5 amd64 0.21.0-1.2build2 [58.4 kB]
Get:29 http://archive.ubuntu.com/ubuntu jammy/main amd64 libuv1 amd64 1.43.0-1 [93.1 kB]
Get:30 http://archive.ubuntu.com/ubuntu jammy/main amd64 xz-utils amd64 5.2.5-2ubuntu1 [84.8 kB]
Get:31 http://archive.ubuntu.com/ubuntu jammy/main amd64 binutils-common amd64 2.38-3ubuntu1 [221 kB]
Get:32 http://archive.ubuntu.com/ubuntu jammy/main amd64 libbinutils amd64 2.38-3ubuntu1 [662 kB]
Get:33 http://archive.ubuntu.com/ubuntu jammy/main amd64 libctf-nobfd0 amd64 2.38-3ubuntu1 [106 kB]
Get:34 http://archive.ubuntu.com/ubuntu jammy/main amd64 libctf0 amd64 2.38-3ubuntu1 [103 kB]
Get:35 http://archive.ubuntu.com/ubuntu jammy/main amd64 binutils-x86-64-linux-gnu amd64 2.38-3ubuntu1 [2328 kB]
Get:36 http://archive.ubuntu.com/ubuntu jammy/main amd64 binutils amd64 2.38-3ubuntu1 [3186 B]
Get:37 http://archive.ubuntu.com/ubuntu jammy/main amd64 libarchive13 amd64 3.6.0-1ubuntu1 [368 kB]
Get:38 http://archive.ubuntu.com/ubuntu jammy/main amd64 libbrotli1 amd64 1.0.9-2build6 [315 kB]
Get:39 http://archive.ubuntu.com/ubuntu jammy/main amd64 libsasl2-modules-db amd64 2.1.27+dfsg2-3ubuntu1 [20.8 kB]
Get:40 http://archive.ubuntu.com/ubuntu jammy/main amd64 libsasl2-2 amd64 2.1.27+dfsg2-3ubuntu1 [53.9 kB]
Get:41 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libldap-2.5-0 amd64 2.5.12+dfsg-0ubuntu0.22.04.1 [184 kB]
Get:42 http://archive.ubuntu.com/ubuntu jammy/main amd64 librtmp1 amd64 2.4+20151223.gitfa8646d.1-2build4 [58.2 kB]
Get:43 http://archive.ubuntu.com/ubuntu jammy/main amd64 libssh-4 amd64 0.9.6-2build1 [184 kB]
Get:44 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libcurl4 amd64 7.81.0-1ubuntu1.3 [290 kB]
Get:45 http://archive.ubuntu.com/ubuntu jammy/main amd64 libjsoncpp25 amd64 1.9.5-3 [80.0 kB]
Get:46 http://archive.ubuntu.com/ubuntu jammy/main amd64 librhash0 amd64 1.4.2-1ubuntu1 [125 kB]
Get:47 http://archive.ubuntu.com/ubuntu jammy/main amd64 dh-elpa-helper all 2.0.9ubuntu1 [7610 B]
Get:48 http://archive.ubuntu.com/ubuntu jammy/main amd64 emacsen-common all 3.0.4 [14.9 kB]
Get:49 http://archive.ubuntu.com/ubuntu jammy/main amd64 cmake-data all 3.22.1-1ubuntu1 [1912 kB]
Get:50 http://archive.ubuntu.com/ubuntu jammy/main amd64 cmake amd64 3.22.1-1ubuntu1 [5012 kB]
Get:51 http://archive.ubuntu.com/ubuntu jammy/main amd64 gcc-11-base amd64 11.2.0-19ubuntu1 [20.8 kB]
Get:52 http://archive.ubuntu.com/ubuntu jammy/main amd64 libisl23 amd64 0.24-2build1 [727 kB]
Get:53 http://archive.ubuntu.com/ubuntu jammy/main amd64 libmpfr6 amd64 4.1.0-3build3 [1425 kB]
Get:54 http://archive.ubuntu.com/ubuntu jammy/main amd64 libmpc3 amd64 1.2.1-2build1 [46.9 kB]
Get:55 http://archive.ubuntu.com/ubuntu jammy/main amd64 cpp-11 amd64 11.2.0-19ubuntu1 [9966 kB]
Get:56 http://archive.ubuntu.com/ubuntu jammy/main amd64 cpp amd64 4:11.2.0-1ubuntu1 [27.7 kB]
Get:57 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 curl amd64 7.81.0-1ubuntu1.3 [194 kB]
Get:58 http://archive.ubuntu.com/ubuntu jammy/main amd64 libcc1-0 amd64 12-20220319-1ubuntu1 [47.2 kB]
Get:59 http://archive.ubuntu.com/ubuntu jammy/main amd64 libgomp1 amd64 12-20220319-1ubuntu1 [126 kB]
Get:60 http://archive.ubuntu.com/ubuntu jammy/main amd64 libitm1 amd64 12-20220319-1ubuntu1 [30.2 kB]
Get:61 http://archive.ubuntu.com/ubuntu jammy/main amd64 libatomic1 amd64 12-20220319-1ubuntu1 [10.4 kB]
Get:62 http://archive.ubuntu.com/ubuntu jammy/main amd64 libasan6 amd64 11.2.0-19ubuntu1 [2283 kB]
Get:63 http://archive.ubuntu.com/ubuntu jammy/main amd64 liblsan0 amd64 12-20220319-1ubuntu1 [1069 kB]
Get:64 http://archive.ubuntu.com/ubuntu jammy/main amd64 libtsan0 amd64 11.2.0-19ubuntu1 [2261 kB]
Get:65 http://archive.ubuntu.com/ubuntu jammy/main amd64 libubsan1 amd64 12-20220319-1ubuntu1 [976 kB]
Get:66 http://archive.ubuntu.com/ubuntu jammy/main amd64 libquadmath0 amd64 12-20220319-1ubuntu1 [154 kB]
Get:67 http://archive.ubuntu.com/ubuntu jammy/main amd64 libgcc-11-dev amd64 11.2.0-19ubuntu1 [2526 kB]
Get:68 http://archive.ubuntu.com/ubuntu jammy/main amd64 gcc-11 amd64 11.2.0-19ubuntu1 [20.1 MB]
Get:69 http://archive.ubuntu.com/ubuntu jammy/main amd64 gcc amd64 4:11.2.0-1ubuntu1 [5112 B]
Get:70 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libc-dev-bin amd64 2.35-0ubuntu3.1 [20.4 kB]
Get:71 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 linux-libc-dev amd64 5.15.0-46.49 [1298 kB]
Get:72 http://archive.ubuntu.com/ubuntu jammy/main amd64 libcrypt-dev amd64 1:4.4.27-1 [112 kB]
Get:73 http://archive.ubuntu.com/ubuntu jammy/main amd64 rpcsvc-proto amd64 1.4.2-0ubuntu6 [68.5 kB]
Get:74 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libtirpc-dev amd64 1.3.2-2ubuntu0.1 [192 kB]
Get:75 http://archive.ubuntu.com/ubuntu jammy/main amd64 libnsl-dev amd64 1.3.0-2build2 [71.3 kB]
Get:76 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libc6-dev amd64 2.35-0ubuntu3.1 [2099 kB]
Get:77 http://archive.ubuntu.com/ubuntu jammy/main amd64 libstdc++-11-dev amd64 11.2.0-19ubuntu1 [2083 kB]
Get:78 http://archive.ubuntu.com/ubuntu jammy/main amd64 g++-11 amd64 11.2.0-19ubuntu1 [11.4 MB]
Get:79 http://archive.ubuntu.com/ubuntu jammy/main amd64 g++ amd64 4:11.2.0-1ubuntu1 [1412 B]
Get:80 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libcurl3-gnutls amd64 7.81.0-1ubuntu1.3 [284 kB]
Get:81 http://archive.ubuntu.com/ubuntu jammy/main amd64 liberror-perl all 0.17029-1 [26.5 kB]
Get:82 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 git-man all 1:2.34.1-1ubuntu1.4 [952 kB]
Get:83 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 git amd64 1:2.34.1-1ubuntu1.4 [3131 kB]
Get:84 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libssl-dev amd64 3.0.2-0ubuntu1.6 [2370 kB]
Get:85 http://archive.ubuntu.com/ubuntu jammy/main amd64 libtidy5deb1 amd64 2:5.6.0-11build2 [236 kB]
Get:86 http://archive.ubuntu.com/ubuntu jammy/main amd64 make amd64 4.3-4.1build1 [180 kB]
Get:87 http://archive.ubuntu.com/ubuntu jammy/universe amd64 ninja-build amd64 1.10.1-1 [111 kB]
Get:88 http://archive.ubuntu.com/ubuntu jammy/universe amd64 tidy amd64 2:5.6.0-11build2 [29.7 kB]
Fetched 107 MB in 1s (90.5 MB/s)
Selecting previously unselected package libpython3.10-minimal:amd64.
(Reading database ... 
(Reading database ... 5%
---
(Reading database ... 100%
(Reading database ... 4697 files and directories currently installed.)
Preparing to unpack .../0-python3-minimal_3.10.4-0ubuntu2_amd64.deb ...
Unpacking python3-minimal (3.10.4-0ubuntu2) ...
Selecting previously unselected package media-types.
Preparing to unpack .../1-media-types_7.0.0_all.deb ...
Unpacking media-types (7.0.0) ...
Preparing to unpack .../2-libmpdec3_2.5.1-2build2_amd64.deb ...
Unpacking libmpdec3:amd64 (2.5.1-2build2) ...
Selecting previously unselected package readline-common.
Preparing to unpack .../3-readline-common_8.1.2-1_all.deb ...
---
Unpacking libjsoncpp25:amd64 (1.9.5-3) ...
Selecting previously unselected package librhash0:amd64.
Preparing to unpack .../33-librhash0_1.4.2-1ubuntu1_amd64.deb ...
Unpacking librhash0:amd64 (1.4.2-1ubuntu1) ...
Selecting previously unselected package dh-elpa-helper.
Preparing to unpack .../34-dh-elpa-helper_2.0.9ubuntu1_all.deb ...
Unpacking dh-elpa-helper (2.0.9ubuntu1) ...
Selecting previously unselected package emacsen-common.
Preparing to unpack .../35-emacsen-common_3.0.4_all.deb ...
Unpacking emacsen-common (3.0.4) ...
Preparing to unpack .../36-cmake-data_3.22.1-1ubuntu1_all.deb ...
Unpacking cmake-data (3.22.1-1ubuntu1) ...
Selecting previously unselected package cmake.
Preparing to unpack .../37-cmake_3.22.1-1ubuntu1_amd64.deb ...
---
Unpacking linux-libc-dev:amd64 (5.15.0-46.49) ...
Selecting previously unselected package libcrypt-dev:amd64.
Preparing to unpack .../59-libcrypt-dev_1%3a4.4.27-1_amd64.deb ...
Unpacking libcrypt-dev:amd64 (1:4.4.27-1) ...
Selecting previously unselected package rpcsvc-proto.
Preparing to unpack .../60-rpcsvc-proto_1.4.2-0ubuntu6_amd64.deb ...
Unpacking rpcsvc-proto (1.4.2-0ubuntu6) ...
Selecting previously unselected package libtirpc-dev:amd64.
Preparing to unpack .../61-libtirpc-dev_1.3.2-2ubuntu0.1_amd64.deb ...
Unpacking libtirpc-dev:amd64 (1.3.2-2ubuntu0.1) ...
Selecting previously unselected package libnsl-dev:amd64.
Preparing to unpack .../62-libnsl-dev_1.3.0-2build2_amd64.deb ...
Unpacking libnsl-dev:amd64 (1.3.0-2build2) ...
Preparing to unpack .../63-libc6-dev_2.35-0ubuntu3.1_amd64.deb ...
Unpacking libc6-dev:amd64 (2.35-0ubuntu3.1) ...
Selecting previously unselected package libstdc++-11-dev:amd64.
Preparing to unpack .../64-libstdc++-11-dev_11.2.0-19ubuntu1_amd64.deb ...
---
Unpacking git (1:2.34.1-1ubuntu1.4) ...
Selecting previously unselected package libssl-dev:amd64.
Preparing to unpack .../71-libssl-dev_3.0.2-0ubuntu1.6_amd64.deb ...
Unpacking libssl-dev:amd64 (3.0.2-0ubuntu1.6) ...
Selecting previously unselected package libtidy5deb1:amd64.
Preparing to unpack .../72-libtidy5deb1_2%3a5.6.0-11build2_amd64.deb ...
Unpacking libtidy5deb1:amd64 (2:5.6.0-11build2) ...
Preparing to unpack .../73-make_4.3-4.1build1_amd64.deb ...
Unpacking make (4.3-4.1build1) ...
Selecting previously unselected package ninja-build.
Preparing to unpack .../74-ninja-build_1.10.1-1_amd64.deb ...
Preparing to unpack .../74-ninja-build_1.10.1-1_amd64.deb ...
Unpacking ninja-build (1.10.1-1) ...
Selecting previously unselected package tidy.
Preparing to unpack .../75-tidy_2%3a5.6.0-11build2_amd64.deb ...
Unpacking tidy (2:5.6.0-11build2) ...
Setting up media-types (7.0.0) ...
Setting up libpsl5:amd64 (0.21.0-1.2build2) ...
Setting up libmagic-mgc (1:5.41-3) ...
Setting up libbrotli1:amd64 (1.0.9-2build6) ...
Setting up libsqlite3-0:amd64 (3.37.2-2) ...
---
Setting up libasan6:amd64 (11.2.0-19ubuntu1) ...
Setting up libsasl2-modules-db:amd64 (2.1.27+dfsg2-3ubuntu1) ...
Setting up libtirpc-dev:amd64 (1.3.2-2ubuntu0.1) ...
Setting up libuv1:amd64 (1.43.0-1) ...
Setting up rpcsvc-proto (1.4.2-0ubuntu6) ...
Setting up emacsen-common (3.0.4) ...
Setting up libmpfr6:amd64 (4.1.0-3build3) ...
Setting up librtmp1:amd64 (2.4+20151223.gitfa8646d.1-2build4) ...
Setting up librtmp1:amd64 (2.4+20151223.gitfa8646d.1-2build4) ...
Setting up dh-elpa-helper (2.0.9ubuntu1) ...
update-alternatives: using /usr/bin/xz to provide /usr/bin/lzma (lzma) in auto mode
update-alternatives: warning: skip creation of /usr/share/man/man1/lzma.1.gz because associated file /usr/share/man/man1/xz.1.gz (of link group lzma) doesn't exist
update-alternatives: warning: skip creation of /usr/share/man/man1/unlzma.1.gz because associated file /usr/share/man/man1/unxz.1.gz (of link group lzma) doesn't exist
update-alternatives: warning: skip creation of /usr/share/man/man1/lzcat.1.gz because associated file /usr/share/man/man1/xzcat.1.gz (of link group lzma) doesn't exist
---
Setting up cmake-data (3.22.1-1ubuntu1) ...
Setting up libbinutils:amd64 (2.38-3ubuntu1) ...
Setting up libisl23:amd64 (0.24-2build1) ...
Setting up libc-dev-bin (2.35-0ubuntu3.1) ...
Setting up libtidy5deb1:amd64 (2:5.6.0-11build2) ...
Setting up readline-common (8.1.2-1) ...
Setting up libcc1-0:amd64 (12-20220319-1ubuntu1) ...
Setting up liblsan0:amd64 (12-20220319-1ubuntu1) ...
Setting up libitm1:amd64 (12-20220319-1ubuntu1) ...
---
libgcc-s1 is already the newest version (12-20220319-1ubuntu1).
libstdc++6 is already the newest version (12-20220319-1ubuntu1).
libc6 is already the newest version (2.35-0ubuntu3.1).
The following additional packages will be installed:
  adwaita-icon-theme alsa-topology-conf alsa-ucm-conf at-spi2-core dbus
  dmsetup fontconfig fontconfig-config gconf-service-backend gconf2-common
  dmsetup fontconfig fontconfig-config gconf-service-backend gconf2-common
  gir1.2-glib-2.0 gsettings-desktop-schemas gtk-update-icon-cache
  hicolor-icon-theme humanity-icon-theme libapparmor1 libargon2-1
  libavahi-client3 libavahi-common-data libavahi-common3 libbsd0
  libcairo-gobject2 libclone-perl libcolord2 libcryptsetup12 libdata-dump-perl
  libdatrie1 libdbus-glib-1-2 libdbusmenu-glib4 libdbusmenu-gtk4 libdconf1
  libdeflate0 libdevmapper1.02.1 libdrm-amdgpu1 libdrm-common libdrm-intel1
  libdeflate0 libdevmapper1.02.1 libdrm-amdgpu1 libdrm-common libdrm-intel1
  libdrm-nouveau2 libdrm-radeon1 libdrm2 libedit2 libelf1
  libencode-locale-perl libepoxy0 libfile-basedir-perl
  libfile-desktopentry-perl libfile-listing-perl libfile-mimeinfo-perl
  libfont-afm-perl libfontenc1 libfreetype6 libfribidi0 libgail-common
  libgail18 libgdk-pixbuf-2.0-0 libgdk-pixbuf-xlib-2.0-0 libgdk-pixbuf2.0-bin
  libgdk-pixbuf2.0-common libgirepository-1.0-1 libgl1 libgl1-amber-dri
  libgl1-mesa-dri libglapi-mesa libglib2.0-data libglvnd0 libglx-mesa0 libglx0
  libgtk2.0-common libgtkd-3-0 libharfbuzz0b libhtml-form-perl
  libhtml-format-perl libhtml-parser-perl libhtml-tagset-perl
  libhtml-tree-perl libhttp-cookies-perl libhttp-daemon-perl libhttp-date-perl
  libhttp-message-perl libhttp-negotiate-perl libice6 libio-html-perl
  libhttp-message-perl libhttp-negotiate-perl libice6 libio-html-perl
  libio-socket-ssl-perl libio-stringy-perl libip4tc2 libipc-system-simple-perl
  libjbig0 libjpeg-turbo8 libjpeg8 libjson-c5 libkmod2 liblcms2-2 libllvm11
  libllvm13 liblwp-mediatypes-perl liblwp-protocol-https-perl
  libmailtools-perl libmd0 libnet-dbus-perl libnet-http-perl
  libnet-smtp-ssl-perl libnet-ssleay-perl libnss-systemd libpam-systemd
  libpangoft2-1.0-0 libpciaccess0 libphobos2-ldc-shared98 libpixman-1-0
  libpng16-16 librsvg2-2 librsvg2-common libsensors-config libsensors5 libsm6
  libtext-iconv-perl libthai-data libthai0 libtie-ixhash-perl libtiff5
  libtimedate-perl libtry-tiny-perl libunwind8 liburi-perl libvte-2.91-0
  libvte-2.91-common libvted-3-0 libvulkan1 libwayland-client0
  libwayland-cursor0 libwayland-egl1 libwayland-server0 libwebp7 libwww-perl
  libxcb-dri2-0 libxcb-dri3-0 libxcb-glx0 libxcb-present0 libxcb-randr0
  libxcb-render0 libxcb-shape0 libxcb-shm0 libxcb-sync1 libxcb-xfixes0
  libxdmcp6 libxft2 libxinerama1 libxkbcommon0 libxkbfile1 libxml-parser-perl
  libxml-twig-perl libxml-xpathengine-perl libxmu6 libxmuu1 libxpm4
  libxml-twig-perl libxml-xpathengine-perl libxmu6 libxmuu1 libxpm4
  libxshmfence1 libxt6 libxv1 libxxf86dga1 libxxf86vm1 mesa-vulkan-drivers
  netbase networkd-dispatcher perl-openssl-defaults python3-dbus python3-gi
  session-migration shared-mime-info systemd systemd-sysv systemd-timesyncd
  tilix tilix-common ubuntu-mono ucf x11-common x11-utils x11-xserver-utils
Suggested packages:
  indicator-application libasound2-plugins alsa-utils libdigest-hmac-perl
  indicator-application libasound2-plugins alsa-utils libdigest-hmac-perl
  libgssapi-perl colord cups-common gvfs liblcms2-utils libcrypt-ssleay-perl
  pciutils librsvg2-bin lm-sensors libsub-name-perl libbusiness-isbn-perl
  libauthen-ntlm-perl libunicode-map8-perl libunicode-string-perl
  xml-twig-tools iw | wireless-tools python-dbus-doc systemd-container
  libfido2-1 libtss2-esys-3.0.2-0 libtss2-mu0 libtss2-rc0 policykit-1
  python-nautilus mesa-utils nickle cairo-5c xorg-docs-core
The following NEW packages will be installed:
  adwaita-icon-theme alsa-topology-conf alsa-ucm-conf at-spi2-core dbus
  dmsetup fontconfig fontconfig-config fonts-liberation gconf-service
  gconf-service-backend gconf2-common gir1.2-glib-2.0
  gconf-service-backend gconf2-common gir1.2-glib-2.0
  gsettings-desktop-schemas gtk-update-icon-cache hicolor-icon-theme
  humanity-icon-theme libapparmor1 libappindicator1 libargon2-1 libasound2
  libauthen-sasl-perl libavahi-client3 libavahi-common-data libavahi-common3
  libbsd0 libcairo-gobject2 libcairo2 libclone-perl libcolord2 libcryptsetup12
  libcups2 libdata-dump-perl libdatrie1 libdbus-1-3 libdbus-glib-1-2
  libdbusmenu-glib4 libdbusmenu-gtk4 libdconf1 libdeflate0 libdevmapper1.02.1
  libdbusmenu-glib4 libdbusmenu-gtk4 libdconf1 libdeflate0 libdevmapper1.02.1
  libdrm-amdgpu1 libdrm-common libdrm-intel1 libdrm-nouveau2 libdrm-radeon1
  libdrm2 libedit2 libelf1 libencode-locale-perl libepoxy0
  libfile-basedir-perl libfile-desktopentry-perl libfile-listing-perl
  libfile-mimeinfo-perl libfont-afm-perl libfontconfig1 libfontenc1
  libfreetype6 libfribidi0 libgail-common libgail18 libgbm1 libgconf-2-4
  libgdk-pixbuf-2.0-0 libgdk-pixbuf-xlib-2.0-0 libgdk-pixbuf2.0-0
  libgdk-pixbuf2.0-bin libgdk-pixbuf2.0-common libgirepository-1.0-1 libgl1
  libgl1-amber-dri libgl1-mesa-dri libglapi-mesa libglib2.0-0 libglib2.0-data
  libglvnd0 libglx-mesa0 libglx0 libgraphite2-3 libgtk-3-0 libgtk-3-bin
  libgtk-3-common libgtk2.0-0 libgtk2.0-bin libgtk2.0-common libgtkd-3-0
  libhtml-tagset-perl libhtml-tree-perl libhttp-cookies-perl
  libhttp-daemon-perl libhttp-date-perl libhttp-message-perl
  libhttp-negotiate-perl libice6 libio-html-perl libio-socket-ssl-perl
  libio-stringy-perl libip4tc2 libipc-system-simple-perl libjbig0
  libio-stringy-perl libip4tc2 libipc-system-simple-perl libjbig0
  libjpeg-turbo8 libjpeg8 libjson-c5 libkmod2 liblcms2-2 libllvm11 libllvm13
  liblwp-mediatypes-perl liblwp-protocol-https-perl libmailtools-perl libmd0
  libnet-dbus-perl libnet-http-perl libnet-smtp-ssl-perl libnet-ssleay-perl
  libnspr4 libnss-systemd libnss3 libpam-systemd libpango-1.0-0
  libpangocairo-1.0-0 libpangoft2-1.0-0 libpciaccess0 libphobos2-ldc-shared98
  libpixman-1-0 libpng16-16 librsvg2-2 librsvg2-common libsensors-config
  libsensors5 libsm6 libtext-iconv-perl libthai-data libthai0
  libtie-ixhash-perl libtiff5 libtimedate-perl libtry-tiny-perl libunwind8
  liburi-perl libvte-2.91-0 libvte-2.91-common libvted-3-0 libvulkan1
  libwayland-client0 libwayland-cursor0 libwayland-egl1 libwayland-server0
  libwebp7 libwww-perl libwww-robotrules-perl libx11-6 libx11-data
  libxcb-glx0 libxcb-present0 libxcb-randr0 libxcb-render0 libxcb-shape0
  libxcb-shm0 libxcb-sync1 libxcb-xfixes0 libxcb1 libxcomposite1 libxcursor1
  libxdamage1 libxdmcp6 libxext6 libxfixes3 libxft2 libxi6 libxinerama1
  libxkbcommon0 libxkbfile1 libxml-parser-perl libxml-twig-perl
  libxkbcommon0 libxkbfile1 libxml-parser-perl libxml-twig-perl
  libxml-xpathengine-perl libxmu6 libxmuu1 libxpm4 libxrandr2 libxrender1
  libxshmfence1 libxss1 libxt6 libxtst6 libxv1 libxxf86dga1 libxxf86vm1
  lsb-release mesa-vulkan-drivers netbase networkd-dispatcher
  perl-openssl-defaults python3-dbus python3-gi session-migration
  shared-mime-info systemd systemd-sysv systemd-timesyncd tilix tilix-common
  ubuntu-mono ucf wget x11-common x11-utils x11-xserver-utils xdg-user-dirs
0 upgraded, 231 newly installed, 0 to remove and 3 not upgraded.
Need to get 101 MB of archives.
After this operation, 428 MB of additional disk space will be used.
After this operation, 428 MB of additional disk space will be used.
Get:1 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libapparmor1 amd64 3.0.4-2ubuntu2.1 [38.7 kB]
Get:2 http://archive.ubuntu.com/ubuntu jammy/main amd64 libargon2-1 amd64 0~20171227-0.3 [19.5 kB]
Get:3 http://archive.ubuntu.com/ubuntu jammy/main amd64 libdevmapper1.02.1 amd64 2:1.02.175-2.1ubuntu4 [139 kB]
Get:4 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libjson-c5 amd64 0.15-3~ubuntu1.22.04.1 [33.5 kB]
Get:5 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libcryptsetup12 amd64 2:2.4.3-1ubuntu1.1 [211 kB]
Get:6 http://archive.ubuntu.com/ubuntu jammy/main amd64 libip4tc2 amd64 1.8.7-1ubuntu5 [19.7 kB]
Get:7 http://archive.ubuntu.com/ubuntu jammy/main amd64 libkmod2 amd64 29-1ubuntu1 [48.0 kB]
Get:8 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 systemd amd64 249.11-0ubuntu3.4 [4580 kB]
Get:9 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 systemd-sysv amd64 249.11-0ubuntu3.4 [10.5 kB]
Get:10 http://archive.ubuntu.com/ubuntu jammy/main amd64 libdbus-1-3 amd64 1.12.20-2ubuntu4 [188 kB]
Get:11 http://archive.ubuntu.com/ubuntu jammy/main amd64 dbus amd64 1.12.20-2ubuntu4 [158 kB]
Get:12 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 distro-info-data all 0.52ubuntu0.1 [5124 B]
Get:13 http://archive.ubuntu.com/ubuntu jammy/main amd64 dmsetup amd64 2:1.02.175-2.1ubuntu4 [81.7 kB]
Get:14 http://archive.ubuntu.com/ubuntu jammy/main amd64 libglib2.0-0 amd64 2.72.1-1 [1460 kB]
Get:15 http://archive.ubuntu.com/ubuntu jammy/main amd64 libgirepository-1.0-1 amd64 1.72.0-1 [55.6 kB]
Get:16 http://archive.ubuntu.com/ubuntu jammy/main amd64 gir1.2-glib-2.0 amd64 1.72.0-1 [164 kB]
Get:17 http://archive.ubuntu.com/ubuntu jammy/main amd64 libmd0 amd64 1.0.4-1build1 [23.0 kB]
Get:18 http://archive.ubuntu.com/ubuntu jammy/main amd64 libbsd0 amd64 0.11.5-1 [44.8 kB]
Get:19 http://archive.ubuntu.com/ubuntu jammy/main amd64 libelf1 amd64 0.186-1build1 [51.0 kB]
Get:20 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libfribidi0 amd64 1.0.8-2ubuntu3.1 [26.1 kB]
Get:21 http://archive.ubuntu.com/ubuntu jammy/main amd64 libglib2.0-data all 2.72.1-1 [4908 B]
Get:22 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libnss-systemd amd64 249.11-0ubuntu3.4 [133 kB]
Get:23 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libpam-systemd amd64 249.11-0ubuntu3.4 [203 kB]
Get:24 http://archive.ubuntu.com/ubuntu jammy/main amd64 libtext-iconv-perl amd64 1.7-7build3 [14.3 kB]
Get:25 http://archive.ubuntu.com/ubuntu jammy/main amd64 lsb-release all 11.1.0ubuntu4 [10.8 kB]
Get:26 http://archive.ubuntu.com/ubuntu jammy/main amd64 netbase all 6.3 [12.9 kB]
Get:27 http://archive.ubuntu.com/ubuntu jammy/main amd64 python3-dbus amd64 1.2.18-3build1 [99.5 kB]
Get:28 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 python3-gi amd64 3.42.1-0ubuntu1 [229 kB]
Get:29 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 networkd-dispatcher all 2.1-2ubuntu0.22.04.2 [15.8 kB]
Get:30 http://archive.ubuntu.com/ubuntu jammy/main amd64 shared-mime-info amd64 2.1-2 [454 kB]
Get:31 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 systemd-timesyncd amd64 249.11-0ubuntu3.4 [31.2 kB]
Get:32 http://archive.ubuntu.com/ubuntu jammy/main amd64 ucf all 3.0043 [56.1 kB]
Get:33 http://archive.ubuntu.com/ubuntu jammy/main amd64 xdg-user-dirs amd64 0.17-2ubuntu4 [53.9 kB]
Get:34 http://archive.ubuntu.com/ubuntu jammy/main amd64 xkb-data all 2.33-1 [394 kB]
Get:35 http://archive.ubuntu.com/ubuntu jammy/main amd64 libdrm-common all 2.4.110-1ubuntu1 [5360 B]
Get:36 http://archive.ubuntu.com/ubuntu jammy/main amd64 libdrm2 amd64 2.4.110-1ubuntu1 [37.7 kB]
Get:37 http://archive.ubuntu.com/ubuntu jammy/main amd64 libedit2 amd64 3.1-20210910-1build1 [96.8 kB]
Get:38 http://archive.ubuntu.com/ubuntu jammy/main amd64 libpng16-16 amd64 1.6.37-3build5 [191 kB]
Get:39 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxau6 amd64 1:1.0.9-1build5 [7634 B]
Get:40 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxdmcp6 amd64 1:1.1.3-0ubuntu5 [10.9 kB]
Get:41 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxcb1 amd64 1.14-3ubuntu3 [49.0 kB]
Get:42 http://archive.ubuntu.com/ubuntu jammy/main amd64 libx11-data all 2:1.7.5-1 [119 kB]
Get:43 http://archive.ubuntu.com/ubuntu jammy/main amd64 libx11-6 amd64 2:1.7.5-1 [666 kB]
Get:44 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxext6 amd64 2:1.3.4-1build1 [31.8 kB]
Get:45 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxmuu1 amd64 2:1.1.3-3 [10.2 kB]
Get:46 http://archive.ubuntu.com/ubuntu jammy/main amd64 wget amd64 1.21.2-2ubuntu1 [367 kB]
Get:47 http://archive.ubuntu.com/ubuntu jammy/main amd64 hicolor-icon-theme all 0.17-2 [9976 B]
Get:48 http://archive.ubuntu.com/ubuntu jammy/main amd64 libgdk-pixbuf2.0-common all 2.42.8+dfsg-1 [5880 B]
Get:49 http://archive.ubuntu.com/ubuntu jammy/main amd64 libjpeg-turbo8 amd64 2.1.2-0ubuntu1 [134 kB]
Get:50 http://archive.ubuntu.com/ubuntu jammy/main amd64 libjpeg8 amd64 8c-2ubuntu10 [2264 B]
Get:51 http://archive.ubuntu.com/ubuntu jammy/main amd64 libdeflate0 amd64 1.10-2 [70.9 kB]
Get:52 http://archive.ubuntu.com/ubuntu jammy/main amd64 libjbig0 amd64 2.1-3.1build3 [28.9 kB]
Get:53 http://archive.ubuntu.com/ubuntu jammy/main amd64 libwebp7 amd64 1.2.2-2 [206 kB]
Get:54 http://archive.ubuntu.com/ubuntu jammy/main amd64 libtiff5 amd64 4.3.0-6 [183 kB]
Get:55 http://archive.ubuntu.com/ubuntu jammy/main amd64 libgdk-pixbuf-2.0-0 amd64 2.42.8+dfsg-1 [148 kB]
Get:56 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 gtk-update-icon-cache amd64 3.24.33-1ubuntu2 [31.4 kB]
Get:57 http://archive.ubuntu.com/ubuntu jammy/main amd64 humanity-icon-theme all 0.6.16 [1282 kB]
Get:58 http://archive.ubuntu.com/ubuntu jammy/main amd64 ubuntu-mono all 20.10-0ubuntu2 [153 kB]
Get:59 http://archive.ubuntu.com/ubuntu jammy/main amd64 adwaita-icon-theme all 41.0-1ubuntu1 [3444 kB]
Get:60 http://archive.ubuntu.com/ubuntu jammy/main amd64 alsa-topology-conf all 1.2.5.1-2 [15.5 kB]
Get:61 http://archive.ubuntu.com/ubuntu jammy/main amd64 libasound2-data all 1.2.6.1-1ubuntu1 [19.1 kB]
Get:62 http://archive.ubuntu.com/ubuntu jammy/main amd64 libasound2 amd64 1.2.6.1-1ubuntu1 [390 kB]
Get:63 http://archive.ubuntu.com/ubuntu jammy/main amd64 alsa-ucm-conf all 1.2.6.3-1ubuntu1 [41.0 kB]
Get:64 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxi6 amd64 2:1.8-1build1 [32.6 kB]
Get:65 http://archive.ubuntu.com/ubuntu jammy/main amd64 libatspi2.0-0 amd64 2.44.0-3 [80.9 kB]
Get:66 http://archive.ubuntu.com/ubuntu jammy/main amd64 x11-common all 1:7.7+23ubuntu2 [23.4 kB]
Get:67 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxtst6 amd64 2:1.2.3-1build4 [13.4 kB]
Get:68 http://archive.ubuntu.com/ubuntu jammy/main amd64 dbus-user-session amd64 1.12.20-2ubuntu4 [9430 B]
Get:69 http://archive.ubuntu.com/ubuntu jammy/main amd64 libdconf1 amd64 0.40.0-3 [40.5 kB]
Get:70 http://archive.ubuntu.com/ubuntu jammy/main amd64 dconf-service amd64 0.40.0-3 [28.5 kB]
Get:71 http://archive.ubuntu.com/ubuntu jammy/main amd64 dconf-gsettings-backend amd64 0.40.0-3 [22.8 kB]
Get:72 http://archive.ubuntu.com/ubuntu jammy/main amd64 session-migration amd64 0.3.6 [9774 B]
Get:73 http://archive.ubuntu.com/ubuntu jammy/main amd64 gsettings-desktop-schemas all 42.0-1ubuntu1 [31.1 kB]
Get:74 http://archive.ubuntu.com/ubuntu jammy/main amd64 at-spi2-core amd64 2.44.0-3 [54.4 kB]
Get:75 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libfreetype6 amd64 2.11.1+dfsg-1ubuntu0.1 [389 kB]
Get:76 http://archive.ubuntu.com/ubuntu jammy/main amd64 fonts-liberation all 1:1.07.4-11 [822 kB]
Get:77 http://archive.ubuntu.com/ubuntu jammy/main amd64 fontconfig-config all 2.13.1-4.2ubuntu5 [29.1 kB]
Get:78 http://archive.ubuntu.com/ubuntu jammy/main amd64 libfontconfig1 amd64 2.13.1-4.2ubuntu5 [131 kB]
Get:79 http://archive.ubuntu.com/ubuntu jammy/main amd64 fontconfig amd64 2.13.1-4.2ubuntu5 [177 kB]
Get:80 http://archive.ubuntu.com/ubuntu jammy/main amd64 libdbus-glib-1-2 amd64 0.112-2build1 [65.4 kB]
Get:81 http://archive.ubuntu.com/ubuntu jammy/universe amd64 gconf2-common all 3.2.6-7ubuntu2 [698 kB]
Get:82 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libgconf-2-4 amd64 3.2.6-7ubuntu2 [86.0 kB]
Get:83 http://archive.ubuntu.com/ubuntu jammy/universe amd64 gconf-service-backend amd64 3.2.6-7ubuntu2 [59.3 kB]
Get:84 http://archive.ubuntu.com/ubuntu jammy/universe amd64 gconf-service amd64 3.2.6-7ubuntu2 [17.4 kB]
Get:85 http://archive.ubuntu.com/ubuntu jammy/main amd64 libdbusmenu-glib4 amd64 16.04.1+18.10.20180917-0ubuntu8 [45.4 kB]
Get:86 http://archive.ubuntu.com/ubuntu jammy/main amd64 libatk1.0-data all 2.36.0-3build1 [2824 B]
Get:87 http://archive.ubuntu.com/ubuntu jammy/main amd64 libatk1.0-0 amd64 2.36.0-3build1 [51.9 kB]
Get:88 http://archive.ubuntu.com/ubuntu jammy/main amd64 libgraphite2-3 amd64 1.3.14-1build2 [71.3 kB]
Get:89 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libharfbuzz0b amd64 2.7.4-1ubuntu3.1 [352 kB]
Get:90 http://archive.ubuntu.com/ubuntu jammy/main amd64 libthai-data all 0.1.29-1build1 [162 kB]
Get:91 http://archive.ubuntu.com/ubuntu jammy/main amd64 libdatrie1 amd64 0.2.13-2 [19.9 kB]
Get:92 http://archive.ubuntu.com/ubuntu jammy/main amd64 libthai0 amd64 0.1.29-1build1 [19.2 kB]
Get:93 http://archive.ubuntu.com/ubuntu jammy/main amd64 libpango-1.0-0 amd64 1.50.6+ds-2 [230 kB]
Get:94 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libdbusmenu-gtk4 amd64 16.04.1+18.10.20180917-0ubuntu8 [30.9 kB]
Get:95 http://archive.ubuntu.com/ubuntu jammy/main amd64 libgtk2.0-common all 2.24.33-2ubuntu2 [125 kB]
Get:96 http://archive.ubuntu.com/ubuntu jammy/main amd64 libpixman-1-0 amd64 0.40.0-1build4 [264 kB]
Get:97 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxcb-render0 amd64 1.14-3ubuntu3 [16.4 kB]
Get:98 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxcb-shm0 amd64 1.14-3ubuntu3 [5780 B]
Get:99 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxrender1 amd64 1:0.9.10-1build4 [19.7 kB]
Get:100 http://archive.ubuntu.com/ubuntu jammy/main amd64 libcairo2 amd64 1.16.0-5ubuntu2 [628 kB]
Get:101 http://archive.ubuntu.com/ubuntu jammy/main amd64 libavahi-common-data amd64 0.8-5ubuntu5 [23.9 kB]
Get:102 http://archive.ubuntu.com/ubuntu jammy/main amd64 libavahi-common3 amd64 0.8-5ubuntu5 [23.7 kB]
Get:103 http://archive.ubuntu.com/ubuntu jammy/main amd64 libavahi-client3 amd64 0.8-5ubuntu5 [28.1 kB]
Get:104 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libcups2 amd64 2.4.1op1-1ubuntu4.1 [264 kB]
Get:105 http://archive.ubuntu.com/ubuntu jammy/main amd64 libpangoft2-1.0-0 amd64 1.50.6+ds-2 [53.9 kB]
Get:106 http://archive.ubuntu.com/ubuntu jammy/main amd64 libpangocairo-1.0-0 amd64 1.50.6+ds-2 [39.8 kB]
Get:107 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxcomposite1 amd64 1:0.4.5-1build2 [7192 B]
Get:108 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxfixes3 amd64 1:6.0.0-1 [11.7 kB]
Get:109 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxcursor1 amd64 1:1.2.0-2build4 [20.9 kB]
Get:110 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxdamage1 amd64 1:1.1.5-2build2 [7154 B]
Get:111 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxinerama1 amd64 2:1.1.4-3 [7382 B]
Get:112 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxrandr2 amd64 2:1.5.2-1build1 [20.4 kB]
Get:113 http://archive.ubuntu.com/ubuntu jammy/main amd64 libgtk2.0-0 amd64 2.24.33-2ubuntu2 [2037 kB]
Get:114 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libappindicator1 amd64 12.10.1+20.10.20200706.1-0ubuntu1 [23.1 kB]
Get:115 http://archive.ubuntu.com/ubuntu jammy/main amd64 libatk-bridge2.0-0 amd64 2.38.0-3 [66.6 kB]
Get:116 http://archive.ubuntu.com/ubuntu jammy/main amd64 libcairo-gobject2 amd64 1.16.0-5ubuntu2 [19.4 kB]
Get:117 http://archive.ubuntu.com/ubuntu jammy/main amd64 libclone-perl amd64 0.45-1build3 [11.0 kB]
Get:118 http://archive.ubuntu.com/ubuntu jammy/main amd64 liblcms2-2 amd64 2.12~rc1-2build2 [159 kB]
Get:119 http://archive.ubuntu.com/ubuntu jammy/main amd64 libcolord2 amd64 1.4.6-1 [155 kB]
Get:120 http://archive.ubuntu.com/ubuntu jammy/main amd64 libdata-dump-perl all 1.25-1 [25.9 kB]
Get:121 http://archive.ubuntu.com/ubuntu jammy/main amd64 libdrm-amdgpu1 amd64 2.4.110-1ubuntu1 [20.0 kB]
Get:122 http://archive.ubuntu.com/ubuntu jammy/main amd64 libpciaccess0 amd64 0.16-3 [19.1 kB]
Get:123 http://archive.ubuntu.com/ubuntu jammy/main amd64 libdrm-intel1 amd64 2.4.110-1ubuntu1 [66.7 kB]
Get:124 http://archive.ubuntu.com/ubuntu jammy/main amd64 libdrm-nouveau2 amd64 2.4.110-1ubuntu1 [17.4 kB]
Get:125 http://archive.ubuntu.com/ubuntu jammy/main amd64 libdrm-radeon1 amd64 2.4.110-1ubuntu1 [21.5 kB]
Get:126 http://archive.ubuntu.com/ubuntu jammy/main amd64 libencode-locale-perl all 1.05-1.1 [11.8 kB]
Get:127 http://archive.ubuntu.com/ubuntu jammy/main amd64 libepoxy0 amd64 1.5.10-1 [237 kB]
Get:128 http://archive.ubuntu.com/ubuntu jammy/main amd64 libipc-system-simple-perl all 1.30-1 [23.2 kB]
Get:129 http://archive.ubuntu.com/ubuntu jammy/main amd64 libfile-basedir-perl all 0.09-1 [15.7 kB]
Get:130 http://archive.ubuntu.com/ubuntu jammy/main amd64 liburi-perl all 5.10-1 [78.8 kB]
Get:131 http://archive.ubuntu.com/ubuntu jammy/main amd64 libfile-desktopentry-perl all 0.22-2 [17.8 kB]
Get:132 http://archive.ubuntu.com/ubuntu jammy/main amd64 libtimedate-perl all 2.3300-2 [34.0 kB]
Get:133 http://archive.ubuntu.com/ubuntu jammy/main amd64 libhttp-date-perl all 6.05-1 [9920 B]
Get:134 http://archive.ubuntu.com/ubuntu jammy/main amd64 libfile-listing-perl all 6.14-1 [11.2 kB]
Get:135 http://archive.ubuntu.com/ubuntu jammy/main amd64 libfile-mimeinfo-perl all 0.31-1 [43.5 kB]
Get:136 http://archive.ubuntu.com/ubuntu jammy/main amd64 libfont-afm-perl all 1.20-3 [13.6 kB]
Get:137 http://archive.ubuntu.com/ubuntu jammy/main amd64 libfontenc1 amd64 1:1.1.4-1build3 [14.7 kB]
Get:138 http://archive.ubuntu.com/ubuntu jammy/main amd64 libgail18 amd64 2.24.33-2ubuntu2 [15.9 kB]
Get:139 http://archive.ubuntu.com/ubuntu jammy/main amd64 libgail-common amd64 2.24.33-2ubuntu2 [132 kB]
Get:140 http://archive.ubuntu.com/ubuntu jammy/main amd64 libwayland-server0 amd64 1.20.0-1 [34.2 kB]
Get:141 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libgbm1 amd64 22.0.5-0ubuntu0.1 [32.6 kB]
Get:142 http://archive.ubuntu.com/ubuntu jammy/main amd64 libgdk-pixbuf-xlib-2.0-0 amd64 2.40.2-2build4 [42.6 kB]
Get:143 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libgdk-pixbuf2.0-0 amd64 2.40.2-2build4 [2454 B]
Get:144 http://archive.ubuntu.com/ubuntu jammy/main amd64 libgdk-pixbuf2.0-bin amd64 2.42.8+dfsg-1 [14.1 kB]
Get:145 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libglapi-mesa amd64 22.0.5-0ubuntu0.1 [35.3 kB]
Get:146 http://archive.ubuntu.com/ubuntu jammy/main amd64 libgl1-amber-dri amd64 21.3.7-0ubuntu1 [4433 kB]
Get:147 http://archive.ubuntu.com/ubuntu jammy/main amd64 libllvm13 amd64 1:13.0.1-2ubuntu2 [22.1 MB]
Get:148 http://archive.ubuntu.com/ubuntu jammy/main amd64 libsensors-config all 1:3.6.0-7ubuntu1 [5274 B]
Get:149 http://archive.ubuntu.com/ubuntu jammy/main amd64 libsensors5 amd64 1:3.6.0-7ubuntu1 [26.3 kB]
Get:150 http://archive.ubuntu.com/ubuntu jammy/main amd64 libvulkan1 amd64 1.3.204.1-2 [128 kB]
Get:151 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libgl1-mesa-dri amd64 22.0.5-0ubuntu0.1 [7471 kB]
Get:152 http://archive.ubuntu.com/ubuntu jammy/main amd64 libx11-xcb1 amd64 2:1.7.5-1 [7790 B]
Get:153 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxcb-dri2-0 amd64 1.14-3ubuntu3 [7206 B]
Get:154 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxcb-dri3-0 amd64 1.14-3ubuntu3 [6968 B]
Get:155 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxcb-glx0 amd64 1.14-3ubuntu3 [25.9 kB]
Get:156 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxcb-present0 amd64 1.14-3ubuntu3 [5734 B]
Get:157 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxcb-sync1 amd64 1.14-3ubuntu3 [9416 B]
Get:158 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxcb-xfixes0 amd64 1.14-3ubuntu3 [9996 B]
Get:159 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxshmfence1 amd64 1.3-1build4 [5394 B]
Get:160 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxxf86vm1 amd64 1:1.1.4-1build3 [10.4 kB]
Get:161 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libglx-mesa0 amd64 22.0.5-0ubuntu0.1 [156 kB]
Get:162 http://archive.ubuntu.com/ubuntu jammy/main amd64 libwayland-client0 amd64 1.20.0-1 [25.9 kB]
Get:163 http://archive.ubuntu.com/ubuntu jammy/main amd64 libwayland-cursor0 amd64 1.20.0-1 [10.7 kB]
Get:164 http://archive.ubuntu.com/ubuntu jammy/main amd64 libwayland-egl1 amd64 1.20.0-1 [5590 B]
Get:165 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxkbcommon0 amd64 1.4.0-1 [125 kB]
Get:166 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libgtk-3-common all 3.24.33-1ubuntu2 [239 kB]
Get:167 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libgtk-3-0 amd64 3.24.33-1ubuntu2 [3053 kB]
Get:168 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libgtk-3-bin amd64 3.24.33-1ubuntu2 [69.5 kB]
Get:169 http://archive.ubuntu.com/ubuntu jammy/main amd64 libgtk2.0-bin amd64 2.24.33-2ubuntu2 [7932 B]
Get:170 http://archive.ubuntu.com/ubuntu jammy/main amd64 librsvg2-2 amd64 2.52.5+dfsg-3 [3020 kB]
Get:171 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libllvm11 amd64 1:11.1.0-6 [19.6 MB]
Get:172 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libphobos2-ldc-shared98 amd64 1:1.28.0-1ubuntu1 [1487 kB]
Get:173 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libgtkd-3-0 amd64 3.10.0-1ubuntu1 [1963 kB]
Get:174 http://archive.ubuntu.com/ubuntu jammy/main amd64 libhtml-tagset-perl all 3.20-4 [12.5 kB]
Get:175 http://archive.ubuntu.com/ubuntu jammy/main amd64 libhtml-parser-perl amd64 3.76-1build2 [88.4 kB]
Get:176 http://archive.ubuntu.com/ubuntu jammy/main amd64 libio-html-perl all 1.004-2 [15.4 kB]
Get:177 http://archive.ubuntu.com/ubuntu jammy/main amd64 liblwp-mediatypes-perl all 6.04-1 [19.5 kB]
Get:178 http://archive.ubuntu.com/ubuntu jammy/main amd64 libhttp-message-perl all 6.36-1 [76.8 kB]
Get:179 http://archive.ubuntu.com/ubuntu jammy/main amd64 libhtml-form-perl all 6.07-1 [22.2 kB]
Get:180 http://archive.ubuntu.com/ubuntu jammy/main amd64 libhtml-tree-perl all 5.07-2 [200 kB]
Get:181 http://archive.ubuntu.com/ubuntu jammy/main amd64 libhtml-format-perl all 2.12-1.1 [41.3 kB]
Get:182 http://archive.ubuntu.com/ubuntu jammy/main amd64 libhttp-cookies-perl all 6.10-1 [18.4 kB]
Get:183 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libhttp-daemon-perl all 6.13-1ubuntu0.1 [22.9 kB]
Get:184 http://archive.ubuntu.com/ubuntu jammy/main amd64 libhttp-negotiate-perl all 6.01-1 [12.5 kB]
Get:185 http://archive.ubuntu.com/ubuntu jammy/main amd64 libice6 amd64 2:1.0.10-1build2 [42.6 kB]
Get:186 http://archive.ubuntu.com/ubuntu jammy/main amd64 perl-openssl-defaults amd64 5build2 [7542 B]
Get:187 http://archive.ubuntu.com/ubuntu jammy/main amd64 libnet-ssleay-perl amd64 1.92-1build2 [327 kB]
Get:188 http://archive.ubuntu.com/ubuntu jammy/main amd64 libio-socket-ssl-perl all 2.074-2 [192 kB]
Get:189 http://archive.ubuntu.com/ubuntu jammy/main amd64 libio-stringy-perl all 2.111-3 [55.8 kB]
Get:190 http://archive.ubuntu.com/ubuntu jammy/main amd64 libnet-http-perl all 6.22-1 [23.2 kB]
Get:191 http://archive.ubuntu.com/ubuntu jammy/main amd64 libtry-tiny-perl all 0.31-1 [21.8 kB]
Get:192 http://archive.ubuntu.com/ubuntu jammy/main amd64 libwww-robotrules-perl all 6.02-1 [12.6 kB]
Get:193 http://archive.ubuntu.com/ubuntu jammy/main amd64 libwww-perl all 6.61-1 [141 kB]
Get:194 http://archive.ubuntu.com/ubuntu jammy/main amd64 liblwp-protocol-https-perl all 6.10-1 [10.9 kB]
Get:195 http://archive.ubuntu.com/ubuntu jammy/main amd64 libnet-smtp-ssl-perl all 1.04-1 [5948 B]
Get:196 http://archive.ubuntu.com/ubuntu jammy/main amd64 libmailtools-perl all 2.21-1 [80.7 kB]
Get:197 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxml-parser-perl amd64 2.46-3build1 [212 kB]
Get:198 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxml-twig-perl all 1:3.52-1 [157 kB]
Get:199 http://archive.ubuntu.com/ubuntu jammy/main amd64 libnet-dbus-perl amd64 1.2.0-1build3 [181 kB]
Get:200 http://archive.ubuntu.com/ubuntu jammy/main amd64 libnspr4 amd64 2:4.32-3build1 [119 kB]
Get:201 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libnss3 amd64 2:3.68.2-0ubuntu1.1 [1280 kB]
Get:202 http://archive.ubuntu.com/ubuntu jammy/main amd64 librsvg2-common amd64 2.52.5+dfsg-3 [17.7 kB]
Get:203 http://archive.ubuntu.com/ubuntu jammy/main amd64 libsm6 amd64 2:1.2.3-1build2 [16.7 kB]
Get:204 http://archive.ubuntu.com/ubuntu jammy/main amd64 libtie-ixhash-perl all 1.23-2.1 [11.3 kB]
Get:205 http://archive.ubuntu.com/ubuntu jammy/main amd64 libunwind8 amd64 1.3.2-2build2 [54.5 kB]
Get:206 http://archive.ubuntu.com/ubuntu jammy/main amd64 libvte-2.91-common amd64 0.68.0-1 [9216 B]
Get:207 http://archive.ubuntu.com/ubuntu jammy/main amd64 libvte-2.91-0 amd64 0.68.0-1 [214 kB]
Get:208 http://archive.ubuntu.com/ubuntu jammy/universe amd64 libvted-3-0 amd64 3.10.0-1ubuntu1 [54.1 kB]
Get:209 http://archive.ubuntu.com/ubuntu jammy/main amd64 libx11-protocol-perl all 0.56-7.1 [138 kB]
Get:210 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxt6 amd64 1:1.2.1-1 [177 kB]
Get:211 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxmu6 amd64 2:1.1.3-3 [49.6 kB]
Get:212 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxpm4 amd64 1:3.5.12-1build2 [36.2 kB]
Get:213 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxaw7 amd64 2:1.0.14-1 [191 kB]
Get:214 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxcb-randr0 amd64 1.14-3ubuntu3 [18.3 kB]
Get:215 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxcb-shape0 amd64 1.14-3ubuntu3 [6158 B]
Get:216 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxft2 amd64 2.3.4-1 [41.8 kB]
Get:217 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxkbfile1 amd64 1:1.1.0-1build3 [71.8 kB]
Get:218 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxml-xpathengine-perl all 0.14-1 [31.8 kB]
Get:219 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxss1 amd64 1:1.2.3-1build2 [8476 B]
Get:220 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxv1 amd64 2:1.0.11-1build2 [11.2 kB]
Get:221 http://archive.ubuntu.com/ubuntu jammy/main amd64 libxxf86dga1 amd64 2:1.1.5-0ubuntu3 [12.6 kB]
Get:222 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 mesa-vulkan-drivers amd64 22.0.5-0ubuntu0.1 [4973 kB]
Get:223 http://archive.ubuntu.com/ubuntu jammy/universe amd64 tilix-common all 1.9.4-2build1 [281 kB]
Get:224 http://archive.ubuntu.com/ubuntu jammy/universe amd64 tilix amd64 1.9.4-2build1 [681 kB]
Get:225 http://archive.ubuntu.com/ubuntu jammy/main amd64 libglvnd0 amd64 1.4.0-1 [73.6 kB]
Get:226 http://archive.ubuntu.com/ubuntu jammy/main amd64 libglx0 amd64 1.4.0-1 [41.0 kB]
Get:227 http://archive.ubuntu.com/ubuntu jammy/main amd64 libgl1 amd64 1.4.0-1 [110 kB]
Get:228 http://archive.ubuntu.com/ubuntu jammy/main amd64 x11-utils amd64 7.7+5build2 [206 kB]
Get:229 http://archive.ubuntu.com/ubuntu jammy/main amd64 x11-xserver-utils amd64 7.7+9build1 [170 kB]
Get:230 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 xdg-utils all 1.1.3-4.1ubuntu1.22.04.1 [62.0 kB]
Get:231 http://archive.ubuntu.com/ubuntu jammy/main amd64 libauthen-sasl-perl all 2.1600-1.1 [43.1 kB]
Fetched 101 MB in 5s (19.6 MB/s)
Selecting previously unselected package libapparmor1:amd64.
(Reading database ... 
(Reading database ... 5%
---
Unpacking libbsd0:amd64 (0.11.5-1) ...
Selecting previously unselected package libelf1:amd64.
Preparing to unpack .../010-libelf1_0.186-1build1_amd64.deb ...
Unpacking libelf1:amd64 (0.186-1build1) ...
Selecting previously unselected package libfribidi0:amd64.
Preparing to unpack .../011-libfribidi0_1.0.8-2ubuntu3.1_amd64.deb ...
Unpacking libfribidi0:amd64 (1.0.8-2ubuntu3.1) ...
Preparing to unpack .../012-libglib2.0-data_2.72.1-1_all.deb ...
Unpacking libglib2.0-data (2.72.1-1) ...
Selecting previously unselected package libnss-systemd:amd64.
Preparing to unpack .../013-libnss-systemd_249.11-0ubuntu3.4_amd64.deb ...
---
Unpacking libdeflate0:amd64 (1.10-2) ...
Selecting previously unselected package libjbig0:amd64.
Preparing to unpack .../043-libjbig0_2.1-3.1build3_amd64.deb ...
Unpacking libjbig0:amd64 (2.1-3.1build3) ...
Selecting previously unselected package libwebp7:amd64.
Preparing to unpack .../044-libwebp7_1.2.2-2_amd64.deb ...
Unpacking libwebp7:amd64 (1.2.2-2) ...
Preparing to unpack .../045-libtiff5_4.3.0-6_amd64.deb ...
Unpacking libtiff5:amd64 (4.3.0-6) ...
Selecting previously unselected package libgdk-pixbuf-2.0-0:amd64.
Preparing to unpack .../046-libgdk-pixbuf-2.0-0_2.42.8+dfsg-1_amd64.deb ...
Preparing to unpack .../046-libgdk-pixbuf-2.0-0_2.42.8+dfsg-1_amd64.deb ...
Unpacking libgdk-pixbuf-2.0-0:amd64 (2.42.8+dfsg-1) ...
Selecting previously unselected package gtk-update-icon-cache.
Preparing to unpack .../047-gtk-update-icon-cache_3.24.33-1ubuntu2_amd64.deb ...
Unpacking gtk-update-icon-cache (3.24.33-1ubuntu2) ...
Preparing to unpack .../048-humanity-icon-theme_0.6.16_all.deb ...
Unpacking humanity-icon-theme (0.6.16) ...
Selecting previously unselected package ubuntu-mono.
Preparing to unpack .../049-ubuntu-mono_20.10-0ubuntu2_all.deb ...
---
Selecting previously unselected package libthai0:amd64.
Preparing to unpack .../083-libthai0_0.1.29-1build1_amd64.deb ...
Unpacking libthai0:amd64 (0.1.29-1build1) ...
Selecting previously unselected package libpango-1.0-0:amd64.
Preparing to unpack .../084-libpango-1.0-0_1.50.6+ds-2_amd64.deb ...
Unpacking libpango-1.0-0:amd64 (1.50.6+ds-2) ...
Preparing to unpack .../085-libdbusmenu-gtk4_16.04.1+18.10.20180917-0ubuntu8_amd64.deb ...
Unpacking libdbusmenu-gtk4:amd64 (16.04.1+18.10.20180917-0ubuntu8) ...
Selecting previously unselected package libgtk2.0-common.
Preparing to unpack .../086-libgtk2.0-common_2.24.33-2ubuntu2_all.deb ...
---
Selecting previously unselected package libavahi-client3:amd64.
Preparing to unpack .../094-libavahi-client3_0.8-5ubuntu5_amd64.deb ...
Unpacking libavahi-client3:amd64 (0.8-5ubuntu5) ...
Selecting previously unselected package libcups2:amd64.
Preparing to unpack .../095-libcups2_2.4.1op1-1ubuntu4.1_amd64.deb ...
Unpacking libcups2:amd64 (2.4.1op1-1ubuntu4.1) ...
Selecting previously unselected package libpangoft2-1.0-0:amd64.
Preparing to unpack .../096-libpangoft2-1.0-0_1.50.6+ds-2_amd64.deb ...
Unpacking libpangoft2-1.0-0:amd64 (1.50.6+ds-2) ...
Selecting previously unselected package libpangocairo-1.0-0:amd64.
Preparing to unpack .../097-libpangocairo-1.0-0_1.50.6+ds-2_amd64.deb ...
Unpacking libpangocairo-1.0-0:amd64 (1.50.6+ds-2) ...
Preparing to unpack .../098-libxcomposite1_1%3a0.4.5-1build2_amd64.deb ...
Unpacking libxcomposite1:amd64 (1:0.4.5-1build2) ...
Selecting previously unselected package libxfixes3:amd64.
Preparing to unpack .../099-libxfixes3_1%3a6.0.0-1_amd64.deb ...
---
Selecting previously unselected package libclone-perl.
Preparing to unpack .../108-libclone-perl_0.45-1build3_amd64.deb ...
Unpacking libclone-perl (0.45-1build3) ...
Selecting previously unselected package liblcms2-2:amd64.
Preparing to unpack .../109-liblcms2-2_2.12~rc1-2build2_amd64.deb ...
Unpacking liblcms2-2:amd64 (2.12~rc1-2build2) ...
Preparing to unpack .../110-libcolord2_1.4.6-1_amd64.deb ...
Unpacking libcolord2:amd64 (1.4.6-1) ...
Selecting previously unselected package libdata-dump-perl.
Preparing to unpack .../111-libdata-dump-perl_1.25-1_all.deb ...
---
Unpacking libfont-afm-perl (1.20-3) ...
Selecting previously unselected package libfontenc1:amd64.
Preparing to unpack .../128-libfontenc1_1%3a1.1.4-1build3_amd64.deb ...
Unpacking libfontenc1:amd64 (1:1.1.4-1build3) ...
Selecting previously unselected package libgail18:amd64.
Preparing to unpack .../129-libgail18_2.24.33-2ubuntu2_amd64.deb ...
Unpacking libgail18:amd64 (2.24.33-2ubuntu2) ...
Selecting previously unselected package libgail-common:amd64.
Preparing to unpack .../130-libgail-common_2.24.33-2ubuntu2_amd64.deb ...
Unpacking libgail-common:amd64 (2.24.33-2ubuntu2) ...
Preparing to unpack .../131-libwayland-server0_1.20.0-1_amd64.deb ...
Unpacking libwayland-server0:amd64 (1.20.0-1) ...
Selecting previously unselected package libgbm1:amd64.
Preparing to unpack .../132-libgbm1_22.0.5-0ubuntu0.1_amd64.deb ...
Preparing to unpack .../132-libgbm1_22.0.5-0ubuntu0.1_amd64.deb ...
Unpacking libgbm1:amd64 (22.0.5-0ubuntu0.1) ...
Selecting previously unselected package libgdk-pixbuf-xlib-2.0-0:amd64.
Preparing to unpack .../133-libgdk-pixbuf-xlib-2.0-0_2.40.2-2build4_amd64.deb ...
Unpacking libgdk-pixbuf-xlib-2.0-0:amd64 (2.40.2-2build4) ...
Preparing to unpack .../134-libgdk-pixbuf2.0-0_2.40.2-2build4_amd64.deb ...
Unpacking libgdk-pixbuf2.0-0:amd64 (2.40.2-2build4) ...
Selecting previously unselected package libgdk-pixbuf2.0-bin.
Preparing to unpack .../135-libgdk-pixbuf2.0-bin_2.42.8+dfsg-1_amd64.deb ...
---
Unpacking libsensors-config (1:3.6.0-7ubuntu1) ...
Selecting previously unselected package libsensors5:amd64.
Preparing to unpack .../140-libsensors5_1%3a3.6.0-7ubuntu1_amd64.deb ...
Unpacking libsensors5:amd64 (1:3.6.0-7ubuntu1) ...
Selecting previously unselected package libvulkan1:amd64.
Preparing to unpack .../141-libvulkan1_1.3.204.1-2_amd64.deb ...
Unpacking libvulkan1:amd64 (1.3.204.1-2) ...
Preparing to unpack .../142-libgl1-mesa-dri_22.0.5-0ubuntu0.1_amd64.deb ...
Unpacking libgl1-mesa-dri:amd64 (22.0.5-0ubuntu0.1) ...
Selecting previously unselected package libx11-xcb1:amd64.
Preparing to unpack .../143-libx11-xcb1_2%3a1.7.5-1_amd64.deb ...
---
Unpacking libxshmfence1:amd64 (1.3-1build4) ...
Selecting previously unselected package libxxf86vm1:amd64.
Preparing to unpack .../151-libxxf86vm1_1%3a1.1.4-1build3_amd64.deb ...
Unpacking libxxf86vm1:amd64 (1:1.1.4-1build3) ...
Selecting previously unselected package libglx-mesa0:amd64.
Preparing to unpack .../152-libglx-mesa0_22.0.5-0ubuntu0.1_amd64.deb ...
Unpacking libglx-mesa0:amd64 (22.0.5-0ubuntu0.1) ...
Preparing to unpack .../153-libwayland-client0_1.20.0-1_amd64.deb ...
Unpacking libwayland-client0:amd64 (1.20.0-1) ...
Selecting previously unselected package libwayland-cursor0:amd64.
Preparing to unpack .../154-libwayland-cursor0_1.20.0-1_amd64.deb ...
---
Unpacking librsvg2-2:amd64 (2.52.5+dfsg-3) ...
Selecting previously unselected package libllvm11:amd64.
Preparing to unpack .../162-libllvm11_1%3a11.1.0-6_amd64.deb ...
Unpacking libllvm11:amd64 (1:11.1.0-6) ...
Selecting previously unselected package libphobos2-ldc-shared98:amd64.
Preparing to unpack .../163-libphobos2-ldc-shared98_1%3a1.28.0-1ubuntu1_amd64.deb ...
Unpacking libphobos2-ldc-shared98:amd64 (1:1.28.0-1ubuntu1) ...
Selecting previously unselected package libgtkd-3-0:amd64.
Preparing to unpack .../164-libgtkd-3-0_3.10.0-1ubuntu1_amd64.deb ...
Unpacking libgtkd-3-0:amd64 (3.10.0-1ubuntu1) ...
Preparing to unpack .../165-libhtml-tagset-perl_3.20-4_all.deb ...
Unpacking libhtml-tagset-perl (3.20-4) ...
Selecting previously unselected package libhtml-parser-perl:amd64.
Preparing to unpack .../166-libhtml-parser-perl_3.76-1build2_amd64.deb ...
---
Unpacking libhttp-negotiate-perl (6.01-1) ...
Selecting previously unselected package libice6:amd64.
Preparing to unpack .../176-libice6_2%3a1.0.10-1build2_amd64.deb ...
Unpacking libice6:amd64 (2:1.0.10-1build2) ...
Selecting previously unselected package perl-openssl-defaults:amd64.
Preparing to unpack .../177-perl-openssl-defaults_5build2_amd64.deb ...
Unpacking perl-openssl-defaults:amd64 (5build2) ...
Selecting previously unselected package libnet-ssleay-perl:amd64.
Unpacking libnet-ssleay-perl:amd64 (1.92-1build2) ...
Selecting previously unselected package libio-socket-ssl-perl.
Preparing to unpack .../179-libio-socket-ssl-perl_2.074-2_all.deb ...
Unpacking libio-socket-ssl-perl (2.074-2) ...
Unpacking libio-socket-ssl-perl (2.074-2) ...
Selecting previously unselected package libio-stringy-perl.
Preparing to unpack .../180-libio-stringy-perl_2.111-3_all.deb ...
Unpacking libio-stringy-perl (2.111-3) ...
Selecting previously unselected package libnet-http-perl.
Preparing to unpack .../181-libnet-http-perl_6.22-1_all.deb ...
Unpacking libnet-http-perl (6.22-1) ...
Selecting previously unselected package libtry-tiny-perl.
Preparing to unpack .../182-libtry-tiny-perl_0.31-1_all.deb ...
Unpacking libtry-tiny-perl (0.31-1) ...
Preparing to unpack .../183-libwww-robotrules-perl_6.02-1_all.deb ...
Unpacking libwww-robotrules-perl (6.02-1) ...
Selecting previously unselected package libwww-perl.
Preparing to unpack .../184-libwww-perl_6.61-1_all.deb ...
---
Unpacking libxv1:amd64 (2:1.0.11-1build2) ...
Selecting previously unselected package libxxf86dga1:amd64.
Preparing to unpack .../212-libxxf86dga1_2%3a1.1.5-0ubuntu3_amd64.deb ...
Unpacking libxxf86dga1:amd64 (2:1.1.5-0ubuntu3) ...
Selecting previously unselected package mesa-vulkan-drivers:amd64.
Preparing to unpack .../213-mesa-vulkan-drivers_22.0.5-0ubuntu0.1_amd64.deb ...
Unpacking mesa-vulkan-drivers:amd64 (22.0.5-0ubuntu0.1) ...
Selecting previously unselected package tilix-common.
Preparing to unpack .../214-tilix-common_1.9.4-2build1_all.deb ...
Unpacking tilix-common (1.9.4-2build1) ...
Selecting previously unselected package tilix.
Preparing to unpack .../215-tilix_1.9.4-2build1_amd64.deb ...
Unpacking tilix (1.9.4-2build1) ...
Selecting previously unselected package libglvnd0:amd64.
Preparing to unpack .../216-libglvnd0_1.4.0-1_amd64.deb ...
Unpacking libglvnd0:amd64 (1.4.0-1) ...
Selecting previously unselected package libglx0:amd64.
Preparing to unpack .../217-libglx0_1.4.0-1_amd64.deb ...
Unpacking libglx0:amd64 (1.4.0-1) ...
Selecting previously unselected package libgl1:amd64.
Preparing to unpack .../218-libgl1_1.4.0-1_amd64.deb ...
Unpacking libgl1:amd64 (1.4.0-1) ...
Selecting previously unselected package x11-utils.
Preparing to unpack .../219-x11-utils_7.7+5build2_amd64.deb ...
Unpacking x11-utils (7.7+5build2) ...
Selecting previously unselected package x11-xserver-utils.
Preparing to unpack .../220-x11-xserver-utils_7.7+9build1_amd64.deb ...
Unpacking x11-xserver-utils (7.7+9build1) ...
Preparing to unpack .../221-xdg-utils_1.1.3-4.1ubuntu1.22.04.1_all.deb ...
Unpacking xdg-utils (1.1.3-4.1ubuntu1.22.04.1) ...
Selecting previously unselected package libauthen-sasl-perl.
Preparing to unpack .../222-libauthen-sasl-perl_2.1600-1.1_all.deb ...
---
Setting up libxau6:amd64 (1:1.0.9-1build5) ...
Setting up libtie-ixhash-perl (1.23-2.1) ...
Setting up wget (1.21.2-2ubuntu1) ...
Setting up hicolor-icon-theme (0.17-2) ...
Setting up tilix-common (1.9.4-2build1) ...
Setting up libvte-2.91-common (0.68.0-1) ...
Setting up libdatrie1:amd64 (0.2.13-2) ...
Setting up xdg-user-dirs (0.17-2ubuntu4) ...
Setting up libclone-perl (0.45-1build3) ...
Setting up libclone-perl (0.45-1build3) ...
Setting up libglib2.0-0:amd64 (2.72.1-1) ...
Setting up distro-info-data (0.52ubuntu0.1) ...
Setting up libglvnd0:amd64 (1.4.0-1) ...
Setting up libio-stringy-perl (2.111-3) ...
Setting up libauthen-sasl-perl (2.1600-1.1) ...
Setting up libdbusmenu-glib4:amd64 (16.04.1+18.10.20180917-0ubuntu8) ...
Setting up liblwp-mediatypes-perl (6.04-1) ...
Setting up libgdk-pixbuf2.0-common (2.42.8+dfsg-1) ...
Setting up libgdk-pixbuf2.0-common (2.42.8+dfsg-1) ...
Setting up x11-common (1:7.7+23ubuntu2) ...
invoke-rc.d: could not determine current runlevel
invoke-rc.d: policy-rc.d denied execution of start.
Setting up libtry-tiny-perl (0.31-1) ...
Setting up libsensors-config (1:3.6.0-7ubuntu1) ...
Setting up libdeflate0:amd64 (1.10-2) ...
Setting up perl-openssl-defaults:amd64 (5build2) ...
First installation detected...
Checking NSS setup...
Setting up xkb-data (2.33-1) ...
Setting up libencode-locale-perl (1.05-1.1) ...
---
Setting up libxml-xpathengine-perl (0.14-1) ...
Setting up libavahi-common-data:amd64 (0.8-5ubuntu5) ...
Setting up libdbus-1-3:amd64 (1.12.20-2ubuntu4) ...
Setting up dbus (1.12.20-2ubuntu4) ...
Setting up libfribidi0:amd64 (1.0.8-2ubuntu3.1) ...
Setting up libpng16-16:amd64 (1.6.37-3build5) ...
Setting up systemd-timesyncd (249.11-0ubuntu3.4) ...
Created symlink /etc/systemd/system/dbus-org.freedesktop.timesync1.service → /lib/systemd/system/systemd-timesyncd.service.
Created symlink /etc/systemd/system/sysinit.target.wants/systemd-timesyncd.service → /lib/systemd/system/systemd-timesyncd.service.
Created symlink /etc/systemd/system/sysinit.target.wants/systemd-timesyncd.service → /lib/systemd/system/systemd-timesyncd.service.
Setting up libio-html-perl (1.004-2) ...
Setting up ucf (3.0043) ...
Setting up libsensors5:amd64 (1:3.6.0-7ubuntu1) ...
Setting up libjpeg-turbo8:amd64 (2.1.2-0ubuntu1) ...
Setting up libglapi-mesa:amd64 (22.0.5-0ubuntu0.1) ...
Setting up libvulkan1:amd64 (1.3.204.1-2) ...
Setting up libwebp7:amd64 (1.2.2-2) ...
Setting up libatk1.0-data (2.36.0-3build1) ...
Setting up libmd0:amd64 (1.0.4-1build1) ...
Setting up alsa-topology-conf (1.2.5.1-2) ...
Setting up dmsetup (2:1.02.175-2.1ubuntu4) ...
---
Setting up libbsd0:amd64 (0.11.5-1) ...
Setting up libdrm-common (2.4.110-1ubuntu1) ...
Setting up libelf1:amd64 (0.186-1build1) ...
Setting up xdg-utils (1.1.3-4.1ubuntu1.22.04.1) ...
update-alternatives: using /usr/bin/xdg-open to provide /usr/bin/open (open) in auto mode
update-alternatives: warning: skip creation of /usr/share/man/man1/open.1.gz because associated file /usr/share/man/man1/xdg-open.1.gz (of link group open) doesn't exist
Setting up libx11-protocol-perl (0.56-7.1) ...
Setting up libxkbcommon0:amd64 (1.4.0-1) ...
Setting up libwayland-client0:amd64 (1.20.0-1) ...
Setting up libnet-ssleay-perl:amd64 (1.92-1build2) ...
Setting up libnet-ssleay-perl:amd64 (1.92-1build2) ...
Setting up libjpeg8:amd64 (8c-2ubuntu10) ...
Setting up libice6:amd64 (2:1.0.10-1build2) ...
Setting up libhttp-date-perl (6.05-1) ...
Setting up session-migration (0.3.6) ...
Created symlink /etc/systemd/user/graphical-session-pre.target.wants/session-migration.service → /usr/lib/systemd/user/session-migration.service.
Setting up python3-dbus (1.2.18-3build1) ...
Setting up libxcb1:amd64 (1.14-3ubuntu3) ...
Setting up libfile-basedir-perl (0.09-1) ...
Setting up libxcb-xfixes0:amd64 (1.14-3ubuntu3) ...
---
Setting up libhttp-negotiate-perl (6.01-1) ...
Setting up fontconfig (2.13.1-4.2ubuntu5) ...
Regenerating fonts cache... done.
Setting up libdrm-nouveau2:amd64 (2.4.110-1ubuntu1) ...
Setting up libphobos2-ldc-shared98:amd64 (1:1.28.0-1ubuntu1) ...
Setting up libxpm4:amd64 (1:3.5.12-1build2) ...
Setting up libxrender1:amd64 (1:0.9.10-1build4) ...
Setting up libgconf-2-4:amd64 (3.2.6-7ubuntu2) ...
Setting up libgbm1:amd64 (22.0.5-0ubuntu0.1) ...
Setting up libgbm1:amd64 (22.0.5-0ubuntu0.1) ...
Setting up libhttp-cookies-perl (6.10-1) ...
Setting up libdrm-radeon1:amd64 (2.4.110-1ubuntu1) ...
Setting up libhtml-tree-perl (5.07-2) ...
Setting up libpango-1.0-0:amd64 (1.50.6+ds-2) ...
Setting up libgl1-mesa-dri:amd64 (22.0.5-0ubuntu0.1) ...
Setting up libxext6:amd64 (2:1.3.4-1build1) ...
Setting up dconf-service (0.40.0-3) ...
Setting up libhtml-format-perl (2.12-1.1) ...
---
Setting up libhttp-daemon-perl (6.13-1ubuntu0.1) ...
Setting up libgdk-pixbuf-2.0-0:amd64 (2.42.8+dfsg-1) ...
Setting up libcairo-gobject2:amd64 (1.16.0-5ubuntu2) ...
Setting up libxss1:amd64 (1:1.2.3-1build2) ...
Setting up libpangoft2-1.0-0:amd64 (1.50.6+ds-2) ...
Setting up libdbusmenu-gtk4:amd64 (16.04.1+18.10.20180917-0ubuntu8) ...
Setting up libpangocairo-1.0-0:amd64 (1.50.6+ds-2) ...
Created symlink /etc/systemd/system/multi-user.target.wants/networkd-dispatcher.service → /lib/systemd/system/networkd-dispatcher.service.
Setting up libgl1-amber-dri:amd64 (21.3.7-0ubuntu1) ...
Setting up libgl1-amber-dri:amd64 (21.3.7-0ubuntu1) ...
Setting up mesa-vulkan-drivers:amd64 (22.0.5-0ubuntu0.1) ...
Setting up gtk-update-icon-cache (3.24.33-1ubuntu2) ...
Setting up libxmu6:amd64 (2:1.1.3-3) ...
Setting up libxmu6:amd64 (2:1.1.3-3) ...
Setting up libglx-mesa0:amd64 (22.0.5-0ubuntu0.1) ...
Setting up libgdk-pixbuf-xlib-2.0-0:amd64 (2.40.2-2build4) ...
Setting up libxi6:amd64 (2:1.8-1build1) ...
Setting up libglx0:amd64 (1.4.0-1) ...
Setting up libxtst6:amd64 (2:1.2.3-1build4) ...
Setting up libxcursor1:amd64 (1:1.2.0-2build4) ...
Setting up libxxf86dga1:amd64 (2:1.1.5-0ubuntu3) ...
Setting up libxaw7:amd64 (2:1.0.14-1) ...
---
Setting up libgtk-3-bin (3.24.33-1ubuntu2) ...
Setting up libappindicator1 (12.10.1+20.10.20200706.1-0ubuntu1) ...
Setting up libvte-2.91-0:amd64 (0.68.0-1) ...
Setting up humanity-icon-theme (0.6.16) ...
Setting up libvted-3-0:amd64 (3.10.0-1ubuntu1) ...
Setting up libgail18:amd64 (2.24.33-2ubuntu2) ...
Setting up libgtkd-3-0:amd64 (3.10.0-1ubuntu1) ...
Setting up tilix (1.9.4-2build1) ...
Setting up tilix (1.9.4-2build1) ...
update-alternatives: using /usr/bin/tilix.wrapper to provide /usr/bin/x-terminal-emulator (x-terminal-emulator) in auto mode
update-alternatives: warning: skip creation of /usr/share/man/man1/x-terminal-emulator.1.gz because associated file /usr/share/man/man1/tilix.1.gz (of link group x-terminal-emulator) doesn't exist
Setting up libgail-common:amd64 (2.24.33-2ubuntu2) ...
Processing triggers for libc-bin (2.35-0ubuntu3.1) ...
Processing triggers for libgdk-pixbuf-2.0-0:amd64 (2.42.8+dfsg-1) ...
Removing intermediate container beb06c210930
 ---> 0c0e81ed4fce
---
Successfully built 40f6e4088dab
Successfully tagged rust-ci:latest
Built container sha256:40f6e4088dabcf9e11bc8281c66c6a8d5b6906790703dc7f15eddc9647d87cf6
Uploading finished image to https://ci-caches.rust-lang.org/docker/e3b7aef275eb52025f6b70532261a4dfe77644ab47b41b3bdf2d0407d7e773fa39c76440dc462ed1ba98bead95cf2edd123888d64931060c943e826f7cd7b756
upload failed: - to s3://rust-lang-ci-sccache2/docker/e3b7aef275eb52025f6b70532261a4dfe77644ab47b41b3bdf2d0407d7e773fa39c76440dc462ed1ba98bead95cf2edd123888d64931060c943e826f7cd7b756 Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-tools]
---
[390/3027] Building CXX object lib/IR/CMakeFiles/LLVMCore.dir/IRPrintingPasses.cpp.o
[391/3027] Building CXX object lib/IR/CMakeFiles/LLVMCore.dir/Core.cpp.o
/checkout/src/llvm-project/llvm/lib/IR/Core.cpp: In function ‘void LLVMContextSetDiagnosticHandler(LLVMContextRef, LLVMDiagnosticHandler, void*)’:
/checkout/src/llvm-project/llvm/lib/IR/Core.cpp:92:22: warning: cast between incompatible function types from ‘LLVMDiagnosticHandler’ {aka ‘void (*)(LLVMOpaqueDiagnosticInfo*, void*)’} to ‘llvm::DiagnosticHandler::DiagnosticHandlerTy’ {aka ‘void (*)(const llvm::DiagnosticInfo&, void*)’} [-Wcast-function-type]
   92 |       LLVM_EXTENSION reinterpret_cast<DiagnosticHandler::DiagnosticHandlerTy>(
   93 |           Handler),
      |           ~~~~~~~~    
/checkout/src/llvm-project/llvm/lib/IR/Core.cpp: In function ‘void (* LLVMContextGetDiagnosticHandler(LLVMContextRef))(LLVMDiagnosticInfoRef, void*)’:
/checkout/src/llvm-project/llvm/lib/IR/Core.cpp:98:25: warning: cast between incompatible function types from ‘llvm::DiagnosticHandler::DiagnosticHandlerTy’ {aka ‘void (*)(const llvm::DiagnosticInfo&, void*)’} to ‘LLVMDiagnosticHandler’ {aka ‘void (*)(LLVMOpaqueDiagnosticInfo*, void*)’} [-Wcast-function-type]
---
[532/3027] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/MachineModuleInfoImpls.cpp.o
[533/3027] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/MachineLoopUtils.cpp.o
[534/3027] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/MachineBlockPlacement.cpp.o
[535/3027] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/MachineModuleInfo.cpp.o
In constructor ‘llvm::MachineModuleInfo::MachineModuleInfo(const llvm::LLVMTargetMachine*)’,
    inlined from ‘llvm::MachineModuleInfoWrapperPass::MachineModuleInfoWrapperPass(const llvm::LLVMTargetMachine*)’ at /checkout/src/llvm-project/llvm/lib/CodeGen/MachineModuleInfo.cpp:181:26,
    inlined from ‘llvm::Pass* llvm::callDefaultCtor() [with PassName = llvm::MachineModuleInfoWrapperPass]’ at /checkout/src/llvm-project/llvm/include/llvm/PassSupport.h:80:76:
/checkout/src/llvm-project/llvm/lib/CodeGen/MachineModuleInfo.cpp:76:51: warning: ‘this’ pointer is null [-Wnonnull]
   76 |   Context.setObjectFileInfo(TM->getObjFileLowering());
[536/3027] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/MachineModuleSlotTracker.cpp.o
[537/3027] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/MachineOptimizationRemarkEmitter.cpp.o
[538/3027] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/MachineLoopInfo.cpp.o
[539/3027] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/MachineFunction.cpp.o
---
[1775/3027] Building CXX object lib/Target/AArch64/CMakeFiles/LLVMAArch64CodeGen.dir/GISel/AArch64InstructionSelector.cpp.o
In file included from /checkout/src/llvm-project/llvm/lib/Target/AArch64/GISel/AArch64InstructionSelector.cpp:491:
/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lib/Target/AArch64/AArch64GenGlobalISel.inc: In member function ‘{anonymous}::PredicateBitset {anonymous}::AArch64InstructionSelector::computeAvailableModuleFeatures(const llvm::AArch64Subtarget*) const’:
/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lib/Target/AArch64/AArch64GenGlobalISel.inc:93: note: ‘-Wmisleading-indentation’ is disabled from this point onwards, since column-tracking was disabled due to the size of the code/headers
   93 |   if (Subtarget->hasPAuth())
      | 
/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lib/Target/AArch64/AArch64GenGlobalISel.inc:93: note: adding ‘-flarge-source-files’ will allow for more column-tracking support, at the expense of compilation time and memory
[1777/3027] Building CXX object lib/Target/AArch64/CMakeFiles/LLVMAArch64CodeGen.dir/AArch64CallingConvention.cpp.o
[1778/3027] Building CXX object lib/ProfileData/CMakeFiles/LLVMProfileData.dir/GCOV.cpp.o
[1779/3027] Building CXX object lib/Target/AArch64/CMakeFiles/LLVMAArch64CodeGen.dir/AArch64CollectLOH.cpp.o
[1780/3027] Building CXX object lib/Target/AArch64/CMakeFiles/LLVMAArch64CodeGen.dir/AArch64ConditionalCompares.cpp.o
---
[2207/3027] Building CXX object lib/Target/RISCV/CMakeFiles/LLVMRISCVCodeGen.dir/RISCVMergeBaseOffset.cpp.o
[2208/3027] Building CXX object lib/Target/RISCV/CMakeFiles/LLVMRISCVCodeGen.dir/RISCVInstructionSelector.cpp.o
/checkout/src/llvm-project/llvm/lib/Target/RISCV/RISCVInstructionSelector.cpp: In member function ‘virtual bool {anonymous}::RISCVInstructionSelector::select(llvm::MachineInstr&)’:
/checkout/src/llvm-project/llvm/lib/Target/RISCV/RISCVInstructionSelector.cpp:90: note: ‘-Wmisleading-indentation’ is disabled from this point onwards, since column-tracking was disabled due to the size of the code/headers
   90 |   if (selectImpl(I, *CoverageInfo))
      | 
/checkout/src/llvm-project/llvm/lib/Target/RISCV/RISCVInstructionSelector.cpp:90: note: adding ‘-flarge-source-files’ will allow for more column-tracking support, at the expense of compilation time and memory
[2210/3027] Building CXX object lib/Target/RISCV/MCTargetDesc/CMakeFiles/LLVMRISCVDesc.dir/RISCVELFObjectWriter.cpp.o
[2211/3027] Building CXX object lib/Target/RISCV/MCTargetDesc/CMakeFiles/LLVMRISCVDesc.dir/RISCVMCAsmInfo.cpp.o
[2212/3027] Building CXX object lib/Target/RISCV/MCTargetDesc/CMakeFiles/LLVMRISCVDesc.dir/RISCVAsmBackend.cpp.o
[2213/3027] Building CXX object lib/Target/RISCV/Disassembler/CMakeFiles/LLVMRISCVDisassembler.dir/RISCVDisassembler.cpp.o
---
                 from /checkout/src/llvm-project/llvm/include/llvm/CodeGen/LiveIntervals.h:25,
                 from /checkout/src/llvm-project/llvm/lib/Target/X86/X86TileConfig.cpp:25:
/checkout/src/llvm-project/llvm/include/llvm/ADT/IntervalMap.h: In function ‘void llvm::IntervalMapImpl::adjustSiblingSizes(NodeT**, unsigned int, unsigned int*, const unsigned int*)’:
/checkout/src/llvm-project/llvm/include/llvm/ADT/IntervalMap.h:344: note: ‘-Wmisleading-indentation’ is disabled from this point onwards, since column-tracking was disabled due to the size of the code/headers
  344 |     if (CurSize[n] == NewSize[n])
      | 
/checkout/src/llvm-project/llvm/include/llvm/ADT/IntervalMap.h:344: note: adding ‘-flarge-source-files’ will allow for more column-tracking support, at the expense of compilation time and memory
In file included from /checkout/src/llvm-project/llvm/lib/Target/X86/X86FastPreTileConfig.cpp:23:
/checkout/src/llvm-project/llvm/include/llvm/ADT/DepthFirstIterator.h: In member function ‘void llvm::df_iterator<GraphT, SetType, ExtStorage, GT>::toNext()’:
/checkout/src/llvm-project/llvm/include/llvm/ADT/DepthFirstIterator.h:129: note: ‘-Wmisleading-indentation’ is disabled from this point onwards, since column-tracking was disabled due to the size of the code/headers
  129 |       if (!Opt)
  129 |       if (!Opt)
      | 
/checkout/src/llvm-project/llvm/include/llvm/ADT/DepthFirstIterator.h:129: note: adding ‘-flarge-source-files’ will allow for more column-tracking support, at the expense of compilation time and memory
/checkout/src/llvm-project/llvm/lib/Target/X86/X86FastTileConfig.cpp: In function ‘bool isTileDef(llvm::MachineRegisterInfo*, llvm::MachineInstr&)’:
/checkout/src/llvm-project/llvm/lib/Target/X86/X86FastTileConfig.cpp:88: note: ‘-Wmisleading-indentation’ is disabled from this point onwards, since column-tracking was disabled due to the size of the code/headers
/checkout/src/llvm-project/llvm/lib/Target/X86/X86FastTileConfig.cpp:88: note: ‘-Wmisleading-indentation’ is disabled from this point onwards, since column-tracking was disabled due to the size of the code/headers
   88 |   if (MI.isDebugInstr() || MI.isCopy() || MI.getNumOperands() < 3 ||
      | 
/checkout/src/llvm-project/llvm/lib/Target/X86/X86FastTileConfig.cpp:88: note: adding ‘-flarge-source-files’ will allow for more column-tracking support, at the expense of compilation time and memory
[2370/3027] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/X86PreTileConfig.cpp.o
In file included from /checkout/src/llvm-project/llvm/include/llvm/IR/Module.h:29,
                 from /checkout/src/llvm-project/llvm/include/llvm/IR/PassManager.h:46,
                 from /checkout/src/llvm-project/llvm/include/llvm/Analysis/LoopInfo.h:48,
                 from /checkout/src/llvm-project/llvm/include/llvm/Analysis/LoopInfo.h:48,
                 from /checkout/src/llvm-project/llvm/include/llvm/CodeGen/MachineLoopInfo.h:32,
                 from /checkout/src/llvm-project/llvm/lib/Target/X86/X86PreTileConfig.cpp:33:
/checkout/src/llvm-project/llvm/include/llvm/IR/GlobalVariable.h: In member function ‘llvm::AttributeList llvm::GlobalVariable::getAttributesAsList(unsigned int) const’:
/checkout/src/llvm-project/llvm/include/llvm/IR/GlobalVariable.h:230: note: ‘-Wmisleading-indentation’ is disabled from this point onwards, since column-tracking was disabled due to the size of the code/headers
  230 |     if (!hasAttributes())
      | 
/checkout/src/llvm-project/llvm/include/llvm/IR/GlobalVariable.h:230: note: adding ‘-flarge-source-files’ will allow for more column-tracking support, at the expense of compilation time and memory
[2372/3027] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/X86FixupBWInsts.cpp.o
[2373/3027] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/X86DynAllocaExpander.cpp.o
[2374/3027] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/X86FixupSetCC.cpp.o
[2375/3027] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/X86FloatingPoint.cpp.o
---
/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lib/Target/X86/X86GenGlobalISel.inc: In member function ‘{anonymous}::PredicateBitset {anonymous}::X86InstructionSelector::computeAvailableModuleFeatures(const llvm::X86Subtarget*) const’:
/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lib/Target/X86/X86GenGlobalISel.inc:164: note: ‘-Wmisleading-indentation’ is disabled from this point onwards, since column-tracking was disabled due to the size of the code/headers
  164 |   if (true)
      | 
/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lib/Target/X86/X86GenGlobalISel.inc:164: note: adding ‘-flarge-source-files’ will allow for more column-tracking support, at the expense of compilation time and memory
[2386/3027] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/X86FastISel.cpp.o
[2387/3027] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/X86EvexToVex.cpp.o
[2388/3027] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/X86InterleavedAccess.cpp.o
[2389/3027] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/X86InsertPrefetch.cpp.o
---
[2396/3027] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/X86ISelDAGToDAG.cpp.o
In file included from /checkout/src/llvm-project/llvm/lib/Target/X86/X86ISelDAGToDAG.cpp:202:
/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lib/Target/X86/X86GenDAGISel.inc: In member function ‘virtual bool {anonymous}::X86DAGToDAGISel::CheckNodePredicate(llvm::SDNode*, unsigned int) const’:
/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lib/Target/X86/X86GenDAGISel.inc:284155: note: ‘-Wmisleading-indentation’ is disabled from this point onwards, since column-tracking was disabled due to the size of the code/headers
284155 | if (cast<StoreSDNode>(N)->getAddressingMode() != ISD::UNINDEXED) return false;
       | 
/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lib/Target/X86/X86GenDAGISel.inc:284155: note: adding ‘-flarge-source-files’ will allow for more column-tracking support, at the expense of compilation time and memory
[2398/3027] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/X86TargetObjectFile.cpp.o
[2399/3027] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/X86OptimizeLEAs.cpp.o
[2400/3027] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/X86ReturnThunks.cpp.o
[2401/3027] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/X86MCInstLower.cpp.o
[2401/3027] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/X86MCInstLower.cpp.o
[2402/3027] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/X86InstrInfo.cpp.o
/checkout/src/llvm-project/llvm/lib/Target/X86/X86InstrInfo.cpp: In member function ‘virtual bool llvm::X86InstrInfo::isCoalescableExtInstr(const llvm::MachineInstr&, llvm::Register&, llvm::Register&, unsigned int&) const’:
/checkout/src/llvm-project/llvm/lib/Target/X86/X86InstrInfo.cpp:104: note: ‘-Wmisleading-indentation’ is disabled from this point onwards, since column-tracking was disabled due to the size of the code/headers
  104 |     if (!Subtarget.is64Bit())
      | 
/checkout/src/llvm-project/llvm/lib/Target/X86/X86InstrInfo.cpp:104: note: adding ‘-flarge-source-files’ will allow for more column-tracking support, at the expense of compilation time and memory
[2404/3027] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/X86PartialReduction.cpp.o
[2405/3027] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/X86RegisterInfo.cpp.o
[2406/3027] Building CXX object lib/Target/X86/MCTargetDesc/CMakeFiles/LLVMX86Desc.dir/X86ATTInstPrinter.cpp.o
[2407/3027] Building CXX object lib/Target/X86/MCTargetDesc/CMakeFiles/LLVMX86Desc.dir/X86IntelInstPrinter.cpp.o
---
In file included from /checkout/src/llvm-project/llvm/include/llvm/Support/FormatVariadic.h:35,
                 from /checkout/src/llvm-project/llvm/tools/llvm-exegesis/lib/X86/Target.cpp:24:
/checkout/src/llvm-project/llvm/include/llvm/Support/FormatProviders.h: In static member function ‘static llvm::Optional<long unsigned int> llvm::detail::HelperFunctions::parseNumericPrecision(llvm::StringRef)’:
/checkout/src/llvm-project/llvm/include/llvm/Support/FormatProviders.h:67: note: ‘-Wmisleading-indentation’ is disabled from this point onwards, since column-tracking was disabled due to the size of the code/headers
   67 |     else if (Str.getAsInteger(10, Prec)) {
      | 
/checkout/src/llvm-project/llvm/include/llvm/Support/FormatProviders.h:67: note: adding ‘-flarge-source-files’ will allow for more column-tracking support, at the expense of compilation time and memory
[2791/3027] Linking CXX executable bin/llvm-mc
[2792/3027] Linking CXX executable bin/llvm-link
[2793/3027] Building CXX object tools/llvm-mca/CMakeFiles/llvm-mca.dir/Views/InstructionView.cpp.o
[2794/3027] Building CXX object tools/llvm-lipo/CMakeFiles/llvm-lipo.dir/llvm-lipo.cpp.o
---
 Documenting implementors v0.1.0 (/checkout/src/test/rustdoc-gui/src/lib2/implementors)
 Documenting lib2 v0.1.0 (/checkout/src/test/rustdoc-gui/src/lib2)
    Finished dev [unoptimized + debuginfo] target(s) in 1.37s
Running 68 rustdoc-gui (16 concurrently) ...
ERROR: puppeteer failed when trying to create a new page. Please try again with `--no-sandbox`

`browser-ui-test` crashed unexpectedly. Please try again with adding `--test-args --no-sandbox` at the end. For example: `x.py test src/test/rustdoc-gui --test-args --no-sandbox`
Build completed unsuccessfully in 0:00:12
