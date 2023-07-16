plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:8e5e7e5ab8b370d6c329ec480221332ada57f0ab)
Download action repository 'rust-lang/simpleinfra@master' (SHA:4d706a8d448f000965ed9e883febfb36661202fe)
Complete job name: PR - mingw-check-tidy
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: mingw-check-tidy
---
Ign:136 http://archive.ubuntu.com/ubuntu jammy/universe amd64 mingw-w64 all 8.0.0-1
Ign:10 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libpython3.10-stdlib amd64 3.10.6-1~22.04.2ubuntu1
Ign:11 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 python3.10 amd64 3.10.6-1~22.04.2ubuntu1
Err:12 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libpython3-stdlib amd64 3.10.6-1~22.04
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:13 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 python3 amd64 3.10.6-1~22.04
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:14 http://archive.ubuntu.com/ubuntu jammy/main amd64 ucf all 3.0043
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:15 http://archive.ubuntu.com/ubuntu jammy/main amd64 libdebuginfod-common all 0.186-1build1
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:17 http://archive.ubuntu.com/ubuntu jammy/main amd64 libgdbm6 amd64 1.23-1
Err:17 http://archive.ubuntu.com/ubuntu jammy/main amd64 libgdbm6 amd64 1.23-1
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:18 http://archive.ubuntu.com/ubuntu jammy/main amd64 libgdbm-compat4 amd64 1.23-1
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Ign:20 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 perl amd64 5.34.0-3ubuntu1.1
Ign:21 http://archive.ubuntu.com/ubuntu jammy-updates/universe amd64 libpython2.7-minimal amd64 2.7.18-13ubuntu1.1
Ign:22 http://archive.ubuntu.com/ubuntu jammy-updates/universe amd64 python2.7-minimal amd64 2.7.18-13ubuntu1.1
Ign:23 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 openssl amd64 3.0.2-0ubuntu1.8
Ign:23 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 openssl amd64 3.0.2-0ubuntu1.8
Ign:24 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 ca-certificates all 20211016ubuntu0.22.04.1
Err:25 http://archive.ubuntu.com/ubuntu jammy/main amd64 libelf1 amd64 0.186-1build1
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:26 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libglib2.0-0 amd64 2.72.4-0ubuntu1
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:27 http://archive.ubuntu.com/ubuntu jammy/main amd64 libicu70 amd64 70.1-2
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Ign:29 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 python3-pkg-resources all 59.6.0-1.2ubuntu0.22.04.1
Ign:30 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 sudo amd64 1.9.9-1ubuntu2.4
Err:31 http://archive.ubuntu.com/ubuntu jammy/main amd64 libmagic-mgc amd64 1:5.41-3
Err:31 http://archive.ubuntu.com/ubuntu jammy/main amd64 libmagic-mgc amd64 1:5.41-3
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:32 http://archive.ubuntu.com/ubuntu jammy/main amd64 libmagic1 amd64 1:5.41-3
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:33 http://archive.ubuntu.com/ubuntu jammy/main amd64 file amd64 1:5.41-3
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:34 http://archive.ubuntu.com/ubuntu jammy/main amd64 libnghttp2-14 amd64 1.43.0-1build3
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:35 http://archive.ubuntu.com/ubuntu jammy/main amd64 libpsl5 amd64 0.21.0-1.2build2
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:36 http://archive.ubuntu.com/ubuntu jammy/main amd64 libuv1 amd64 1.43.0-1
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:37 http://archive.ubuntu.com/ubuntu jammy/main amd64 xz-utils amd64 5.2.5-2ubuntu1
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Ign:39 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libbinutils amd64 2.38-4ubuntu2.1
Ign:40 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libctf-nobfd0 amd64 2.38-4ubuntu2.1
Ign:41 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libctf0 amd64 2.38-4ubuntu2.1
Ign:42 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 binutils-x86-64-linux-gnu amd64 2.38-4ubuntu2.1
Ign:42 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 binutils-x86-64-linux-gnu amd64 2.38-4ubuntu2.1
Ign:43 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 binutils amd64 2.38-4ubuntu2.1
Err:44 http://archive.ubuntu.com/ubuntu jammy/main amd64 libarchive13 amd64 3.6.0-1ubuntu1
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:45 http://archive.ubuntu.com/ubuntu jammy/main amd64 libbrotli1 amd64 1.0.9-2build6
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:46 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libsasl2-modules-db amd64 2.1.27+dfsg2-3ubuntu1.2
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:47 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libsasl2-2 amd64 2.1.27+dfsg2-3ubuntu1.2
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:48 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libldap-2.5-0 amd64 2.5.14+dfsg-0ubuntu0.22.04.2
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:49 http://archive.ubuntu.com/ubuntu jammy/main amd64 librtmp1 amd64 2.4+20151223.gitfa8646d.1-2build4
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:50 http://archive.ubuntu.com/ubuntu jammy/main amd64 libssh-4 amd64 0.9.6-2build1
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:52 http://archive.ubuntu.com/ubuntu jammy/main amd64 libjsoncpp25 amd64 1.9.5-3
Err:52 http://archive.ubuntu.com/ubuntu jammy/main amd64 libjsoncpp25 amd64 1.9.5-3
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:53 http://archive.ubuntu.com/ubuntu jammy/main amd64 librhash0 amd64 1.4.2-1ubuntu1
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:54 http://archive.ubuntu.com/ubuntu jammy/main amd64 dh-elpa-helper all 2.0.9ubuntu1
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:55 http://archive.ubuntu.com/ubuntu jammy/main amd64 emacsen-common all 3.0.4
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:56 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 cmake-data all 3.22.1-1ubuntu1.22.04.1
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:57 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 cmake amd64 3.22.1-1ubuntu1.22.04.1
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:59 http://archive.ubuntu.com/ubuntu jammy/main amd64 libisl23 amd64 0.24-2build1
Err:59 http://archive.ubuntu.com/ubuntu jammy/main amd64 libisl23 amd64 0.24-2build1
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:60 http://archive.ubuntu.com/ubuntu jammy/main amd64 libmpfr6 amd64 4.1.0-3build3
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:61 http://archive.ubuntu.com/ubuntu jammy/main amd64 libmpc3 amd64 1.2.1-2build1
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:63 http://archive.ubuntu.com/ubuntu jammy/main amd64 cpp amd64 4:11.2.0-1ubuntu1
Err:63 http://archive.ubuntu.com/ubuntu jammy/main amd64 cpp amd64 4:11.2.0-1ubuntu1
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Ign:65 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libcc1-0 amd64 12.1.0-2ubuntu1~22.04
Ign:66 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libgomp1 amd64 12.1.0-2ubuntu1~22.04
Ign:67 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libitm1 amd64 12.1.0-2ubuntu1~22.04
Ign:68 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libatomic1 amd64 12.1.0-2ubuntu1~22.04
---
Ign:73 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libquadmath0 amd64 12.1.0-2ubuntu1~22.04
Ign:74 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libgcc-11-dev amd64 11.3.0-1ubuntu1~22.04
Ign:75 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 gcc-11 amd64 11.3.0-1ubuntu1~22.04
Err:76 http://archive.ubuntu.com/ubuntu jammy/main amd64 gcc amd64 4:11.2.0-1ubuntu1
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:77 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libc-dev-bin amd64 2.35-0ubuntu3.1
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:79 http://archive.ubuntu.com/ubuntu jammy/main amd64 libcrypt-dev amd64 1:4.4.27-1
Err:79 http://archive.ubuntu.com/ubuntu jammy/main amd64 libcrypt-dev amd64 1:4.4.27-1
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:80 http://archive.ubuntu.com/ubuntu jammy/main amd64 rpcsvc-proto amd64 1.4.2-0ubuntu6
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:82 http://archive.ubuntu.com/ubuntu jammy/main amd64 libnsl-dev amd64 1.3.0-2build2
Err:82 http://archive.ubuntu.com/ubuntu jammy/main amd64 libnsl-dev amd64 1.3.0-2build2
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:83 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libc6-dev amd64 2.35-0ubuntu3.1
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Ign:85 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 g++-11 amd64 11.3.0-1ubuntu1~22.04
Err:86 http://archive.ubuntu.com/ubuntu jammy/main amd64 g++ amd64 4:11.2.0-1ubuntu1
Err:86 http://archive.ubuntu.com/ubuntu jammy/main amd64 g++ amd64 4:11.2.0-1ubuntu1
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:87 http://archive.ubuntu.com/ubuntu jammy/universe amd64 binutils-mingw-w64-i686 amd64 2.38-3ubuntu1+9build1
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:88 http://archive.ubuntu.com/ubuntu jammy/universe amd64 mingw-w64-common all 8.0.0-1
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:89 http://archive.ubuntu.com/ubuntu jammy/universe amd64 mingw-w64-i686-dev all 8.0.0-1
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:90 http://archive.ubuntu.com/ubuntu jammy/universe amd64 gcc-mingw-w64-base amd64 10.3.0-14ubuntu1+24.3
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:91 http://archive.ubuntu.com/ubuntu jammy/universe amd64 gcc-mingw-w64-i686-posix-runtime amd64 10.3.0-14ubuntu1+24.3
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:92 http://archive.ubuntu.com/ubuntu jammy/universe amd64 gcc-mingw-w64-i686-posix amd64 10.3.0-14ubuntu1+24.3
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:93 http://archive.ubuntu.com/ubuntu jammy/universe amd64 g++-mingw-w64-i686-posix amd64 10.3.0-14ubuntu1+24.3
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:94 http://archive.ubuntu.com/ubuntu jammy/universe amd64 gcc-mingw-w64-i686-win32-runtime amd64 10.3.0-14ubuntu1+24.3
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:95 http://archive.ubuntu.com/ubuntu jammy/universe amd64 gcc-mingw-w64-i686-win32 amd64 10.3.0-14ubuntu1+24.3
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:96 http://archive.ubuntu.com/ubuntu jammy/universe amd64 g++-mingw-w64-i686-win32 amd64 10.3.0-14ubuntu1+24.3
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:97 http://archive.ubuntu.com/ubuntu jammy/universe amd64 binutils-mingw-w64-x86-64 amd64 2.38-3ubuntu1+9build1
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:98 http://archive.ubuntu.com/ubuntu jammy/universe amd64 mingw-w64-x86-64-dev all 8.0.0-1
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:99 http://archive.ubuntu.com/ubuntu jammy/universe amd64 gcc-mingw-w64-x86-64-posix-runtime amd64 10.3.0-14ubuntu1+24.3
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:100 http://archive.ubuntu.com/ubuntu jammy/universe amd64 gcc-mingw-w64-x86-64-posix amd64 10.3.0-14ubuntu1+24.3
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:101 http://archive.ubuntu.com/ubuntu jammy/universe amd64 g++-mingw-w64-x86-64-posix amd64 10.3.0-14ubuntu1+24.3
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:102 http://archive.ubuntu.com/ubuntu jammy/universe amd64 gcc-mingw-w64-x86-64-win32-runtime amd64 10.3.0-14ubuntu1+24.3
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:103 http://archive.ubuntu.com/ubuntu jammy/universe amd64 gcc-mingw-w64-x86-64-win32 amd64 10.3.0-14ubuntu1+24.3
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:104 http://archive.ubuntu.com/ubuntu jammy/universe amd64 g++-mingw-w64-x86-64-win32 amd64 10.3.0-14ubuntu1+24.3
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:105 http://archive.ubuntu.com/ubuntu jammy/main amd64 libdw1 amd64 0.186-1build1
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:106 http://archive.ubuntu.com/ubuntu jammy/main amd64 libbabeltrace1 amd64 1.5.8-2build1
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:108 http://archive.ubuntu.com/ubuntu jammy/main amd64 libdebuginfod1 amd64 0.186-1build1
Err:108 http://archive.ubuntu.com/ubuntu jammy/main amd64 libdebuginfod1 amd64 0.186-1build1
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:109 http://archive.ubuntu.com/ubuntu jammy/main amd64 libipt2 amd64 2.0.5-1
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:111 http://archive.ubuntu.com/ubuntu jammy/main amd64 libsource-highlight-common all 3.1.9-4.1build2
Err:111 http://archive.ubuntu.com/ubuntu jammy/main amd64 libsource-highlight-common all 3.1.9-4.1build2
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:112 http://archive.ubuntu.com/ubuntu jammy/main amd64 libboost-regex1.74.0 amd64 1.74.0-14ubuntu3
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:113 http://archive.ubuntu.com/ubuntu jammy/main amd64 libsource-highlight4v5 amd64 3.1.9-4.1build2
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:114 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 gdb amd64 12.1-0ubuntu1~22.04
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:115 http://archive.ubuntu.com/ubuntu jammy/main amd64 liberror-perl all 0.17029-1
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Ign:117 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 git amd64 1:2.34.1-1ubuntu1.8
Ign:118 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libdpkg-perl all 1.21.1ubuntu2.1
Ign:119 http://archive.ubuntu.com/ubuntu jammy-updates/universe amd64 libpython2.7-stdlib amd64 2.7.18-13ubuntu1.1
Ign:120 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libssl-dev amd64 3.0.2-0ubuntu1.8
Ign:120 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libssl-dev amd64 3.0.2-0ubuntu1.8
Err:121 http://archive.ubuntu.com/ubuntu jammy/main amd64 make amd64 4.3-4.1build1
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:122 http://archive.ubuntu.com/ubuntu jammy/universe amd64 ninja-build amd64 1.10.1-1
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:123 http://archive.ubuntu.com/ubuntu jammy/main amd64 pkg-config amd64 0.29.2-1ubuntu3
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:125 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 python3-lib2to3 all 3.10.6-1~22.04
Err:125 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 python3-lib2to3 all 3.10.6-1~22.04
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:126 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 python3-distutils all 3.10.6-1~22.04
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Ign:128 http://archive.ubuntu.com/ubuntu jammy-updates/universe amd64 python3-wheel all 0.37.1-2ubuntu0.22.04.1
Ign:129 http://archive.ubuntu.com/ubuntu jammy-updates/universe amd64 python3-pip all 22.0.2+dfsg-1ubuntu0.2
Err:130 http://archive.ubuntu.com/ubuntu jammy/universe amd64 g++-mingw-w64-i686 all 10.3.0-14ubuntu1+24.3
Err:130 http://archive.ubuntu.com/ubuntu jammy/universe amd64 g++-mingw-w64-i686 all 10.3.0-14ubuntu1+24.3
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:131 http://archive.ubuntu.com/ubuntu jammy/universe amd64 g++-mingw-w64-x86-64 all 10.3.0-14ubuntu1+24.3
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:132 http://archive.ubuntu.com/ubuntu jammy/universe amd64 g++-mingw-w64 all 10.3.0-14ubuntu1+24.3
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:133 http://archive.ubuntu.com/ubuntu jammy/universe amd64 gcc-mingw-w64-i686 all 10.3.0-14ubuntu1+24.3
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:134 http://archive.ubuntu.com/ubuntu jammy/universe amd64 gcc-mingw-w64-x86-64 all 10.3.0-14ubuntu1+24.3
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:135 http://archive.ubuntu.com/ubuntu jammy/universe amd64 gcc-mingw-w64 all 10.3.0-14ubuntu1+24.3
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:136 http://archive.ubuntu.com/ubuntu jammy/universe amd64 mingw-w64 all 8.0.0-1
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:10 http://security.ubuntu.com/ubuntu jammy-updates/main amd64 libpython3.10-stdlib amd64 3.10.6-1~22.04.2ubuntu1
  Error reading from server - read (104: Connection reset by peer) [IP: 185.125.190.36 80]
Err:11 http://security.ubuntu.com/ubuntu jammy-updates/main amd64 python3.10 amd64 3.10.6-1~22.04.2ubuntu1
  Could not connect to archive.ubuntu.com:80 (2620:2d:4000:1::16). - connect (101: Network is unreachable) Could not connect to archive.ubuntu.com:80 (2620:2d:4000:1::19). - connect (101: Network is unreachable) Could not connect to archive.ubuntu.com:80 (2001:67c:1562::15). - connect (101: Network is unreachable) Could not connect to archive.ubuntu.com:80 (2001:67c:1562::18). - connect (101: Network is unreachable) Could not connect to archive.ubuntu.com:80 (91.189.91.38). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (185.125.190.39). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (185.125.190.36). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (91.189.91.39). - connect (111: Connection refused) [IP: 185.125.190.36 80]
Err:16 http://security.ubuntu.com/ubuntu jammy-updates/main amd64 perl-modules-5.34 all 5.34.0-3ubuntu1.1
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:19 http://security.ubuntu.com/ubuntu jammy-updates/main amd64 libperl5.34 amd64 5.34.0-3ubuntu1.1
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:20 http://security.ubuntu.com/ubuntu jammy-updates/main amd64 perl amd64 5.34.0-3ubuntu1.1
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:21 http://security.ubuntu.com/ubuntu jammy-updates/universe amd64 libpython2.7-minimal amd64 2.7.18-13ubuntu1.1
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:22 http://security.ubuntu.com/ubuntu jammy-updates/universe amd64 python2.7-minimal amd64 2.7.18-13ubuntu1.1
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:23 http://security.ubuntu.com/ubuntu jammy-updates/main amd64 openssl amd64 3.0.2-0ubuntu1.8
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:24 http://security.ubuntu.com/ubuntu jammy-updates/main amd64 ca-certificates all 20211016ubuntu0.22.04.1
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:28 http://security.ubuntu.com/ubuntu jammy-updates/main amd64 libxml2 amd64 2.9.13+dfsg-1ubuntu0.2
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:29 http://security.ubuntu.com/ubuntu jammy-updates/main amd64 python3-pkg-resources all 59.6.0-1.2ubuntu0.22.04.1
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:30 http://security.ubuntu.com/ubuntu jammy-updates/main amd64 sudo amd64 1.9.9-1ubuntu2.4
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:38 http://security.ubuntu.com/ubuntu jammy-updates/main amd64 binutils-common amd64 2.38-4ubuntu2.1
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:39 http://security.ubuntu.com/ubuntu jammy-updates/main amd64 libbinutils amd64 2.38-4ubuntu2.1
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:40 http://security.ubuntu.com/ubuntu jammy-updates/main amd64 libctf-nobfd0 amd64 2.38-4ubuntu2.1
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:41 http://security.ubuntu.com/ubuntu jammy-updates/main amd64 libctf0 amd64 2.38-4ubuntu2.1
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:42 http://security.ubuntu.com/ubuntu jammy-updates/main amd64 binutils-x86-64-linux-gnu amd64 2.38-4ubuntu2.1
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:43 http://security.ubuntu.com/ubuntu jammy-updates/main amd64 binutils amd64 2.38-4ubuntu2.1
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:51 http://security.ubuntu.com/ubuntu jammy-updates/main amd64 libcurl4 amd64 7.81.0-1ubuntu1.10
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:58 http://security.ubuntu.com/ubuntu jammy-updates/main amd64 gcc-11-base amd64 11.3.0-1ubuntu1~22.04
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:62 http://security.ubuntu.com/ubuntu jammy-updates/main amd64 cpp-11 amd64 11.3.0-1ubuntu1~22.04
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:64 http://security.ubuntu.com/ubuntu jammy-updates/main amd64 curl amd64 7.81.0-1ubuntu1.10
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:65 http://security.ubuntu.com/ubuntu jammy-updates/main amd64 libcc1-0 amd64 12.1.0-2ubuntu1~22.04
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:66 http://security.ubuntu.com/ubuntu jammy-updates/main amd64 libgomp1 amd64 12.1.0-2ubuntu1~22.04
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:67 http://security.ubuntu.com/ubuntu jammy-updates/main amd64 libitm1 amd64 12.1.0-2ubuntu1~22.04
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:68 http://security.ubuntu.com/ubuntu jammy-updates/main amd64 libatomic1 amd64 12.1.0-2ubuntu1~22.04
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:69 http://security.ubuntu.com/ubuntu jammy-updates/main amd64 libasan6 amd64 11.3.0-1ubuntu1~22.04
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:70 http://security.ubuntu.com/ubuntu jammy-updates/main amd64 liblsan0 amd64 12.1.0-2ubuntu1~22.04
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:71 http://security.ubuntu.com/ubuntu jammy-updates/main amd64 libtsan0 amd64 11.3.0-1ubuntu1~22.04
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:72 http://security.ubuntu.com/ubuntu jammy-updates/main amd64 libubsan1 amd64 12.1.0-2ubuntu1~22.04
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:73 http://security.ubuntu.com/ubuntu jammy-updates/main amd64 libquadmath0 amd64 12.1.0-2ubuntu1~22.04
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:74 http://security.ubuntu.com/ubuntu jammy-updates/main amd64 libgcc-11-dev amd64 11.3.0-1ubuntu1~22.04
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:75 http://security.ubuntu.com/ubuntu jammy-updates/main amd64 gcc-11 amd64 11.3.0-1ubuntu1~22.04
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:78 http://security.ubuntu.com/ubuntu jammy-updates/main amd64 linux-libc-dev amd64 5.15.0-70.77
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:81 http://security.ubuntu.com/ubuntu jammy-updates/main amd64 libtirpc-dev amd64 1.3.2-2ubuntu0.1
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:84 http://security.ubuntu.com/ubuntu jammy-updates/main amd64 libstdc++-11-dev amd64 11.3.0-1ubuntu1~22.04
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:85 http://security.ubuntu.com/ubuntu jammy-updates/main amd64 g++-11 amd64 11.3.0-1ubuntu1~22.04
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:107 http://security.ubuntu.com/ubuntu jammy-updates/main amd64 libcurl3-gnutls amd64 7.81.0-1ubuntu1.10
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:110 http://security.ubuntu.com/ubuntu jammy-updates/main amd64 libpython3.10 amd64 3.10.6-1~22.04.2ubuntu1
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:116 http://security.ubuntu.com/ubuntu jammy-updates/main amd64 git-man all 1:2.34.1-1ubuntu1.8
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:117 http://security.ubuntu.com/ubuntu jammy-updates/main amd64 git amd64 1:2.34.1-1ubuntu1.8
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:118 http://security.ubuntu.com/ubuntu jammy-updates/main amd64 libdpkg-perl all 1.21.1ubuntu2.1
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:119 http://security.ubuntu.com/ubuntu jammy-updates/universe amd64 libpython2.7-stdlib amd64 2.7.18-13ubuntu1.1
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:120 http://security.ubuntu.com/ubuntu jammy-updates/main amd64 libssl-dev amd64 3.0.2-0ubuntu1.8
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:124 http://security.ubuntu.com/ubuntu jammy-updates/universe amd64 python2.7 amd64 2.7.18-13ubuntu1.1
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:127 http://security.ubuntu.com/ubuntu jammy-updates/main amd64 python3-setuptools all 59.6.0-1.2ubuntu0.22.04.1
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:128 http://security.ubuntu.com/ubuntu jammy-updates/universe amd64 python3-wheel all 0.37.1-2ubuntu0.22.04.1
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Err:129 http://security.ubuntu.com/ubuntu jammy-updates/universe amd64 python3-pip all 22.0.2+dfsg-1ubuntu0.2
  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
Fetched 4148 kB in 35s (119 kB/s)
E: Failed to fetch http://security.ubuntu.com/ubuntu/pool/main/p/python3.10/libpython3.10-stdlib_3.10.6-1%7e22.04.2ubuntu1_amd64.deb  Error reading from server - read (104: Connection reset by peer) [IP: 185.125.190.36 80]
E: Failed to fetch http://security.ubuntu.com/ubuntu/pool/main/p/python3.10/python3.10_3.10.6-1%7e22.04.2ubuntu1_amd64.deb  Could not connect to archive.ubuntu.com:80 (2620:2d:4000:1::16). - connect (101: Network is unreachable) Could not connect to archive.ubuntu.com:80 (2620:2d:4000:1::19). - connect (101: Network is unreachable) Could not connect to archive.ubuntu.com:80 (2001:67c:1562::15). - connect (101: Network is unreachable) Could not connect to archive.ubuntu.com:80 (2001:67c:1562::18). - connect (101: Network is unreachable) Could not connect to archive.ubuntu.com:80 (91.189.91.38). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (185.125.190.39). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (185.125.190.36). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (91.189.91.39). - connect (111: Connection refused) [IP: 185.125.190.36 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/main/p/python3-defaults/libpython3-stdlib_3.10.6-1%7e22.04_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/main/p/python3-defaults/python3_3.10.6-1%7e22.04_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/main/u/ucf/ucf_3.0043_all.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/main/e/elfutils/libdebuginfod-common_0.186-1build1_all.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://security.ubuntu.com/ubuntu/pool/main/p/perl/perl-modules-5.34_5.34.0-3ubuntu1.1_all.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/main/g/gdbm/libgdbm6_1.23-1_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/main/g/gdbm/libgdbm-compat4_1.23-1_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://security.ubuntu.com/ubuntu/pool/main/p/perl/libperl5.34_5.34.0-3ubuntu1.1_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://security.ubuntu.com/ubuntu/pool/main/p/perl/perl_5.34.0-3ubuntu1.1_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://security.ubuntu.com/ubuntu/pool/universe/p/python2.7/libpython2.7-minimal_2.7.18-13ubuntu1.1_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://security.ubuntu.com/ubuntu/pool/universe/p/python2.7/python2.7-minimal_2.7.18-13ubuntu1.1_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://security.ubuntu.com/ubuntu/pool/main/o/openssl/openssl_3.0.2-0ubuntu1.8_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://security.ubuntu.com/ubuntu/pool/main/c/ca-certificates/ca-certificates_20211016ubuntu0.22.04.1_all.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/main/e/elfutils/libelf1_0.186-1build1_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/main/g/glib2.0/libglib2.0-0_2.72.4-0ubuntu1_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/main/i/icu/libicu70_70.1-2_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://security.ubuntu.com/ubuntu/pool/main/libx/libxml2/libxml2_2.9.13%2bdfsg-1ubuntu0.2_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://security.ubuntu.com/ubuntu/pool/main/s/setuptools/python3-pkg-resources_59.6.0-1.2ubuntu0.22.04.1_all.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://security.ubuntu.com/ubuntu/pool/main/s/sudo/sudo_1.9.9-1ubuntu2.4_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/main/f/file/libmagic-mgc_5.41-3_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/main/f/file/libmagic1_5.41-3_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/main/f/file/file_5.41-3_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/main/n/nghttp2/libnghttp2-14_1.43.0-1build3_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/main/libp/libpsl/libpsl5_0.21.0-1.2build2_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/main/libu/libuv1/libuv1_1.43.0-1_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/main/x/xz-utils/xz-utils_5.2.5-2ubuntu1_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://security.ubuntu.com/ubuntu/pool/main/b/binutils/binutils-common_2.38-4ubuntu2.1_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://security.ubuntu.com/ubuntu/pool/main/b/binutils/libbinutils_2.38-4ubuntu2.1_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://security.ubuntu.com/ubuntu/pool/main/b/binutils/libctf-nobfd0_2.38-4ubuntu2.1_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://security.ubuntu.com/ubuntu/pool/main/b/binutils/libctf0_2.38-4ubuntu2.1_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://security.ubuntu.com/ubuntu/pool/main/b/binutils/binutils-x86-64-linux-gnu_2.38-4ubuntu2.1_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://security.ubuntu.com/ubuntu/pool/main/b/binutils/binutils_2.38-4ubuntu2.1_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/main/liba/libarchive/libarchive13_3.6.0-1ubuntu1_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/main/b/brotli/libbrotli1_1.0.9-2build6_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/main/c/cyrus-sasl2/libsasl2-modules-db_2.1.27%2bdfsg2-3ubuntu1.2_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/main/c/cyrus-sasl2/libsasl2-2_2.1.27%2bdfsg2-3ubuntu1.2_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/main/o/openldap/libldap-2.5-0_2.5.14%2bdfsg-0ubuntu0.22.04.2_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/main/r/rtmpdump/librtmp1_2.4%2b20151223.gitfa8646d.1-2build4_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/main/libs/libssh/libssh-4_0.9.6-2build1_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://security.ubuntu.com/ubuntu/pool/main/c/curl/libcurl4_7.81.0-1ubuntu1.10_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/main/libj/libjsoncpp/libjsoncpp25_1.9.5-3_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/main/r/rhash/librhash0_1.4.2-1ubuntu1_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/main/d/dh-elpa/dh-elpa-helper_2.0.9ubuntu1_all.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/main/e/emacsen-common/emacsen-common_3.0.4_all.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/main/c/cmake/cmake-data_3.22.1-1ubuntu1.22.04.1_all.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/main/c/cmake/cmake_3.22.1-1ubuntu1.22.04.1_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://security.ubuntu.com/ubuntu/pool/main/g/gcc-11/gcc-11-base_11.3.0-1ubuntu1%7e22.04_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/main/i/isl/libisl23_0.24-2build1_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/main/m/mpfr4/libmpfr6_4.1.0-3build3_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/main/m/mpclib3/libmpc3_1.2.1-2build1_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://security.ubuntu.com/ubuntu/pool/main/g/gcc-11/cpp-11_11.3.0-1ubuntu1%7e22.04_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/main/g/gcc-defaults/cpp_11.2.0-1ubuntu1_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://security.ubuntu.com/ubuntu/pool/main/c/curl/curl_7.81.0-1ubuntu1.10_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://security.ubuntu.com/ubuntu/pool/main/g/gcc-12/libcc1-0_12.1.0-2ubuntu1%7e22.04_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://security.ubuntu.com/ubuntu/pool/main/g/gcc-12/libgomp1_12.1.0-2ubuntu1%7e22.04_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://security.ubuntu.com/ubuntu/pool/main/g/gcc-12/libitm1_12.1.0-2ubuntu1%7e22.04_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://security.ubuntu.com/ubuntu/pool/main/g/gcc-12/libatomic1_12.1.0-2ubuntu1%7e22.04_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://security.ubuntu.com/ubuntu/pool/main/g/gcc-11/libasan6_11.3.0-1ubuntu1%7e22.04_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://security.ubuntu.com/ubuntu/pool/main/g/gcc-12/liblsan0_12.1.0-2ubuntu1%7e22.04_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://security.ubuntu.com/ubuntu/pool/main/g/gcc-11/libtsan0_11.3.0-1ubuntu1%7e22.04_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://security.ubuntu.com/ubuntu/pool/main/g/gcc-12/libubsan1_12.1.0-2ubuntu1%7e22.04_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://security.ubuntu.com/ubuntu/pool/main/g/gcc-12/libquadmath0_12.1.0-2ubuntu1%7e22.04_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://security.ubuntu.com/ubuntu/pool/main/g/gcc-11/libgcc-11-dev_11.3.0-1ubuntu1%7e22.04_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://security.ubuntu.com/ubuntu/pool/main/g/gcc-11/gcc-11_11.3.0-1ubuntu1%7e22.04_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/main/g/gcc-defaults/gcc_11.2.0-1ubuntu1_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/main/g/glibc/libc-dev-bin_2.35-0ubuntu3.1_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://security.ubuntu.com/ubuntu/pool/main/l/linux/linux-libc-dev_5.15.0-70.77_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/main/libx/libxcrypt/libcrypt-dev_4.4.27-1_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/main/r/rpcsvc-proto/rpcsvc-proto_1.4.2-0ubuntu6_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://security.ubuntu.com/ubuntu/pool/main/libt/libtirpc/libtirpc-dev_1.3.2-2ubuntu0.1_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/main/libn/libnsl/libnsl-dev_1.3.0-2build2_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/main/g/glibc/libc6-dev_2.35-0ubuntu3.1_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://security.ubuntu.com/ubuntu/pool/main/g/gcc-11/libstdc%2b%2b-11-dev_11.3.0-1ubuntu1%7e22.04_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://security.ubuntu.com/ubuntu/pool/main/g/gcc-11/g%2b%2b-11_11.3.0-1ubuntu1%7e22.04_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/main/g/gcc-defaults/g%2b%2b_11.2.0-1ubuntu1_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/universe/b/binutils-mingw-w64/binutils-mingw-w64-i686_2.38-3ubuntu1%2b9build1_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/universe/m/mingw-w64/mingw-w64-common_8.0.0-1_all.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/universe/m/mingw-w64/mingw-w64-i686-dev_8.0.0-1_all.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/universe/g/gcc-mingw-w64/gcc-mingw-w64-base_10.3.0-14ubuntu1%2b24.3_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/universe/g/gcc-mingw-w64/gcc-mingw-w64-i686-posix-runtime_10.3.0-14ubuntu1%2b24.3_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/universe/g/gcc-mingw-w64/gcc-mingw-w64-i686-posix_10.3.0-14ubuntu1%2b24.3_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/universe/g/gcc-mingw-w64/g%2b%2b-mingw-w64-i686-posix_10.3.0-14ubuntu1%2b24.3_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/universe/g/gcc-mingw-w64/gcc-mingw-w64-i686-win32-runtime_10.3.0-14ubuntu1%2b24.3_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/universe/g/gcc-mingw-w64/gcc-mingw-w64-i686-win32_10.3.0-14ubuntu1%2b24.3_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/universe/g/gcc-mingw-w64/g%2b%2b-mingw-w64-i686-win32_10.3.0-14ubuntu1%2b24.3_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/universe/b/binutils-mingw-w64/binutils-mingw-w64-x86-64_2.38-3ubuntu1%2b9build1_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/universe/m/mingw-w64/mingw-w64-x86-64-dev_8.0.0-1_all.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/universe/g/gcc-mingw-w64/gcc-mingw-w64-x86-64-posix-runtime_10.3.0-14ubuntu1%2b24.3_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/universe/g/gcc-mingw-w64/gcc-mingw-w64-x86-64-posix_10.3.0-14ubuntu1%2b24.3_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/universe/g/gcc-mingw-w64/g%2b%2b-mingw-w64-x86-64-posix_10.3.0-14ubuntu1%2b24.3_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/universe/g/gcc-mingw-w64/gcc-mingw-w64-x86-64-win32-runtime_10.3.0-14ubuntu1%2b24.3_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/universe/g/gcc-mingw-w64/gcc-mingw-w64-x86-64-win32_10.3.0-14ubuntu1%2b24.3_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/universe/g/gcc-mingw-w64/g%2b%2b-mingw-w64-x86-64-win32_10.3.0-14ubuntu1%2b24.3_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/main/e/elfutils/libdw1_0.186-1build1_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/main/b/babeltrace/libbabeltrace1_1.5.8-2build1_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://security.ubuntu.com/ubuntu/pool/main/c/curl/libcurl3-gnutls_7.81.0-1ubuntu1.10_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/main/e/elfutils/libdebuginfod1_0.186-1build1_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/main/i/intel-processor-trace/libipt2_2.0.5-1_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://security.ubuntu.com/ubuntu/pool/main/p/python3.10/libpython3.10_3.10.6-1%7e22.04.2ubuntu1_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/main/s/source-highlight/libsource-highlight-common_3.1.9-4.1build2_all.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/main/b/boost1.74/libboost-regex1.74.0_1.74.0-14ubuntu3_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/main/s/source-highlight/libsource-highlight4v5_3.1.9-4.1build2_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/main/g/gdb/gdb_12.1-0ubuntu1%7e22.04_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/main/libe/liberror-perl/liberror-perl_0.17029-1_all.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://security.ubuntu.com/ubuntu/pool/main/g/git/git-man_2.34.1-1ubuntu1.8_all.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://security.ubuntu.com/ubuntu/pool/main/g/git/git_2.34.1-1ubuntu1.8_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://security.ubuntu.com/ubuntu/pool/main/d/dpkg/libdpkg-perl_1.21.1ubuntu2.1_all.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://security.ubuntu.com/ubuntu/pool/universe/p/python2.7/libpython2.7-stdlib_2.7.18-13ubuntu1.1_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://security.ubuntu.com/ubuntu/pool/main/o/openssl/libssl-dev_3.0.2-0ubuntu1.8_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/main/m/make-dfsg/make_4.3-4.1build1_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/universe/n/ninja-build/ninja-build_1.10.1-1_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/main/p/pkg-config/pkg-config_0.29.2-1ubuntu3_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://security.ubuntu.com/ubuntu/pool/universe/p/python2.7/python2.7_2.7.18-13ubuntu1.1_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/main/p/python3-stdlib-extensions/python3-lib2to3_3.10.6-1%7e22.04_all.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/main/p/python3-stdlib-extensions/python3-distutils_3.10.6-1%7e22.04_all.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://security.ubuntu.com/ubuntu/pool/main/s/setuptools/python3-setuptools_59.6.0-1.2ubuntu0.22.04.1_all.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://security.ubuntu.com/ubuntu/pool/universe/w/wheel/python3-wheel_0.37.1-2ubuntu0.22.04.1_all.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://security.ubuntu.com/ubuntu/pool/universe/p/python-pip/python3-pip_22.0.2%2bdfsg-1ubuntu0.2_all.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/universe/g/gcc-mingw-w64/g%2b%2b-mingw-w64-i686_10.3.0-14ubuntu1%2b24.3_all.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/universe/g/gcc-mingw-w64/g%2b%2b-mingw-w64-x86-64_10.3.0-14ubuntu1%2b24.3_all.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/universe/g/gcc-mingw-w64/g%2b%2b-mingw-w64_10.3.0-14ubuntu1%2b24.3_all.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/universe/g/gcc-mingw-w64/gcc-mingw-w64-i686_10.3.0-14ubuntu1%2b24.3_all.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/universe/g/gcc-mingw-w64/gcc-mingw-w64-x86-64_10.3.0-14ubuntu1%2b24.3_all.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/universe/g/gcc-mingw-w64/gcc-mingw-w64_10.3.0-14ubuntu1%2b24.3_all.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/universe/m/mingw-w64/mingw-w64_8.0.0-1_all.deb  Unable to connect to archive.ubuntu.com:80: [IP: 185.125.190.36 80]
E: Unable to fetch some archives, maybe run apt-get update or try with --fix-missing?
The command '/bin/sh -c apt-get update && apt-get install -y --no-install-recommends   g++   make   ninja-build   file   curl   ca-certificates   python2.7   python3   python3-pip   python3-pkg-resources   git   cmake   sudo   gdb   xz-utils   libssl-dev   pkg-config   mingw-w64   && rm -rf /var/lib/apt/lists/*' returned a non-zero code: 100
Sending build context to Docker daemon  690.7kB

Step 1/10 : FROM ubuntu:22.04
 ---> 08d22c0ceb15
---
Ign:2 http://archive.ubuntu.com/ubuntu jammy InRelease
Ign:3 http://archive.ubuntu.com/ubuntu jammy-updates InRelease
Ign:4 http://archive.ubuntu.com/ubuntu jammy-backports InRelease
Err:1 http://security.ubuntu.com/ubuntu jammy-security InRelease
  Could not connect to security.ubuntu.com:80 (91.189.91.38). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (185.125.190.39). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (185.125.190.36). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (2620:2d:4000:1::19). - connect (101: Network is unreachable) Could not connect to security.ubuntu.com:80 (91.189.91.39). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (2001:67c:1562::15). - connect (101: Network is unreachable) Could not connect to security.ubuntu.com:80 (2001:67c:1562::18). - connect (101: Network is unreachable) Could not connect to security.ubuntu.com:80 (2620:2d:4000:1::16). - connect (101: Network is unreachable)
Err:2 http://archive.ubuntu.com/ubuntu jammy InRelease
  Could not connect to archive.ubuntu.com:80 (185.125.190.36). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (91.189.91.39). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (91.189.91.38). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (2001:67c:1562::15). - connect (101: Network is unreachable) Could not connect to archive.ubuntu.com:80 (185.125.190.39). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (2620:2d:4000:1::16). - connect (101: Network is unreachable) Could not connect to archive.ubuntu.com:80 (2001:67c:1562::18). - connect (101: Network is unreachable) Could not connect to archive.ubuntu.com:80 (2620:2d:4000:1::19). - connect (101: Network is unreachable)
  Unable to connect to archive.ubuntu.com:80:
Err:4 http://archive.ubuntu.com/ubuntu jammy-backports InRelease
  Unable to connect to archive.ubuntu.com:80:
Reading package lists...
Reading package lists...
W: Failed to fetch http://archive.ubuntu.com/ubuntu/dists/jammy/InRelease  Could not connect to archive.ubuntu.com:80 (185.125.190.36). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (91.189.91.39). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (91.189.91.38). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (2001:67c:1562::15). - connect (101: Network is unreachable) Could not connect to archive.ubuntu.com:80 (185.125.190.39). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (2620:2d:4000:1::16). - connect (101: Network is unreachable) Could not connect to archive.ubuntu.com:80 (2001:67c:1562::18). - connect (101: Network is unreachable) Could not connect to archive.ubuntu.com:80 (2620:2d:4000:1::19). - connect (101: Network is unreachable)
W: Failed to fetch http://archive.ubuntu.com/ubuntu/dists/jammy-updates/InRelease  Unable to connect to archive.ubuntu.com:80:
W: Failed to fetch http://archive.ubuntu.com/ubuntu/dists/jammy-backports/InRelease  Unable to connect to archive.ubuntu.com:80:
W: Failed to fetch http://security.ubuntu.com/ubuntu/dists/jammy-security/InRelease  Could not connect to security.ubuntu.com:80 (91.189.91.38). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (185.125.190.39). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (185.125.190.36). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (2620:2d:4000:1::19). - connect (101: Network is unreachable) Could not connect to security.ubuntu.com:80 (91.189.91.39). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (2001:67c:1562::15). - connect (101: Network is unreachable) Could not connect to security.ubuntu.com:80 (2001:67c:1562::18). - connect (101: Network is unreachable) Could not connect to security.ubuntu.com:80 (2620:2d:4000:1::16). - connect (101: Network is unreachable)
W: Some index files failed to download. They have been ignored, or old ones used instead.
Building dependency tree...
Reading state information...
Package ca-certificates is not available, but is referred to by another package.
Package ca-certificates is not available, but is referred to by another package.
This may mean that the package is missing, has been obsoleted, or
is only available from another source
Package xz-utils is not available, but is referred to by another package.
Package xz-utils is not available, but is referred to by another package.
This may mean that the package is missing, has been obsoleted, or
is only available from another source
E: Unable to locate package make
E: Unable to locate package ninja-build
E: Unable to locate package file
E: Unable to locate package curl
E: Unable to locate package curl
E: Package 'ca-certificates' has no installation candidate
E: Unable to locate package python2.7
E: Couldn't find any package by glob 'python2.7'
E: Couldn't find any package by regex 'python2.7'
E: Unable to locate package python3-pip
E: Unable to locate package python3-pkg-resources
E: Unable to locate package git
E: Unable to locate package cmake
E: Unable to locate package cmake
E: Unable to locate package sudo
E: Unable to locate package gdb
E: Package 'xz-utils' has no installation candidate
E: Unable to locate package pkg-config
E: Unable to locate package mingw-w64
E: Unable to locate package mingw-w64
The command '/bin/sh -c apt-get update && apt-get install -y --no-install-recommends   g++   make   ninja-build   file   curl   ca-certificates   python2.7   python3   python3-pip   python3-pkg-resources   git   cmake   sudo   gdb   xz-utils   libssl-dev   pkg-config   mingw-w64   && rm -rf /var/lib/apt/lists/*' returned a non-zero code: 100
Sending build context to Docker daemon  690.7kB

Step 1/10 : FROM ubuntu:22.04
 ---> 08d22c0ceb15
---
Ign:4 http://security.ubuntu.com/ubuntu jammy-security InRelease
Ign:2 http://archive.ubuntu.com/ubuntu jammy-updates InRelease
Ign:3 http://archive.ubuntu.com/ubuntu jammy-backports InRelease
Err:4 http://security.ubuntu.com/ubuntu jammy-security InRelease
  Could not connect to security.ubuntu.com:80 (185.125.190.36). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (91.189.91.38). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (91.189.91.39). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (2001:67c:1562::15). - connect (101: Network is unreachable) Could not connect to security.ubuntu.com:80 (185.125.190.39). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (2001:67c:1562::18). - connect (101: Network is unreachable) Could not connect to security.ubuntu.com:80 (2620:2d:4000:1::16). - connect (101: Network is unreachable) Could not connect to security.ubuntu.com:80 (2620:2d:4000:1::19). - connect (101: Network is unreachable)
Err:1 http://archive.ubuntu.com/ubuntu jammy InRelease
  Could not connect to archive.ubuntu.com:80 (91.189.91.38). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (185.125.190.39). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (185.125.190.36). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (2620:2d:4000:1::19). - connect (101: Network is unreachable) Could not connect to archive.ubuntu.com:80 (91.189.91.39). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (2001:67c:1562::15). - connect (101: Network is unreachable) Could not connect to archive.ubuntu.com:80 (2620:2d:4000:1::16). - connect (101: Network is unreachable) Could not connect to archive.ubuntu.com:80 (2001:67c:1562::18). - connect (101: Network is unreachable)
  Unable to connect to archive.ubuntu.com:80:
Err:3 http://archive.ubuntu.com/ubuntu jammy-backports InRelease
  Unable to connect to archive.ubuntu.com:80:
Reading package lists...
Reading package lists...
W: Failed to fetch http://archive.ubuntu.com/ubuntu/dists/jammy/InRelease  Could not connect to archive.ubuntu.com:80 (91.189.91.38). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (185.125.190.39). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (185.125.190.36). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (2620:2d:4000:1::19). - connect (101: Network is unreachable) Could not connect to archive.ubuntu.com:80 (91.189.91.39). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (2001:67c:1562::15). - connect (101: Network is unreachable) Could not connect to archive.ubuntu.com:80 (2620:2d:4000:1::16). - connect (101: Network is unreachable) Could not connect to archive.ubuntu.com:80 (2001:67c:1562::18). - connect (101: Network is unreachable)
W: Failed to fetch http://archive.ubuntu.com/ubuntu/dists/jammy-updates/InRelease  Unable to connect to archive.ubuntu.com:80:
W: Failed to fetch http://archive.ubuntu.com/ubuntu/dists/jammy-backports/InRelease  Unable to connect to archive.ubuntu.com:80:
W: Failed to fetch http://security.ubuntu.com/ubuntu/dists/jammy-security/InRelease  Could not connect to security.ubuntu.com:80 (185.125.190.36). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (91.189.91.38). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (91.189.91.39). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (2001:67c:1562::15). - connect (101: Network is unreachable) Could not connect to security.ubuntu.com:80 (185.125.190.39). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (2001:67c:1562::18). - connect (101: Network is unreachable) Could not connect to security.ubuntu.com:80 (2620:2d:4000:1::16). - connect (101: Network is unreachable) Could not connect to security.ubuntu.com:80 (2620:2d:4000:1::19). - connect (101: Network is unreachable)
W: Some index files failed to download. They have been ignored, or old ones used instead.
Building dependency tree...
Reading state information...
Package ca-certificates is not available, but is referred to by another package.
Package ca-certificates is not available, but is referred to by another package.
This may mean that the package is missing, has been obsoleted, or
is only available from another source
Package xz-utils is not available, but is referred to by another package.
Package xz-utils is not available, but is referred to by another package.
This may mean that the package is missing, has been obsoleted, or
is only available from another source
E: Unable to locate package make
E: Unable to locate package ninja-build
E: Unable to locate package file
E: Unable to locate package curl
E: Unable to locate package curl
E: Package 'ca-certificates' has no installation candidate
E: Unable to locate package python2.7
E: Couldn't find any package by glob 'python2.7'
E: Couldn't find any package by regex 'python2.7'
E: Unable to locate package python3-pip
E: Unable to locate package python3-pkg-resources
E: Unable to locate package git
E: Unable to locate package cmake
E: Unable to locate package cmake
E: Unable to locate package sudo
E: Unable to locate package gdb
E: Package 'xz-utils' has no installation candidate
E: Unable to locate package pkg-config
E: Unable to locate package mingw-w64
E: Unable to locate package mingw-w64
The command '/bin/sh -c apt-get update && apt-get install -y --no-install-recommends   g++   make   ninja-build   file   curl   ca-certificates   python2.7   python3   python3-pip   python3-pkg-resources   git   cmake   sudo   gdb   xz-utils   libssl-dev   pkg-config   mingw-w64   && rm -rf /var/lib/apt/lists/*' returned a non-zero code: 100
Sending build context to Docker daemon  690.7kB

Step 1/10 : FROM ubuntu:22.04
 ---> 08d22c0ceb15
---
Ign:4 http://security.ubuntu.com/ubuntu jammy-security InRelease
Ign:2 http://archive.ubuntu.com/ubuntu jammy-updates InRelease
Ign:3 http://archive.ubuntu.com/ubuntu jammy-backports InRelease
Err:1 http://archive.ubuntu.com/ubuntu jammy InRelease
  Could not connect to archive.ubuntu.com:80 (91.189.91.38). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (91.189.91.39). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (185.125.190.36). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (2620:2d:4000:1::19). - connect (101: Network is unreachable) Could not connect to archive.ubuntu.com:80 (185.125.190.39). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (2001:67c:1562::15). - connect (101: Network is unreachable) Could not connect to archive.ubuntu.com:80 (2001:67c:1562::18). - connect (101: Network is unreachable) Could not connect to archive.ubuntu.com:80 (2620:2d:4000:1::16). - connect (101: Network is unreachable)
Err:4 http://security.ubuntu.com/ubuntu jammy-security InRelease
  Could not connect to security.ubuntu.com:80 (185.125.190.39). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (91.189.91.39). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (91.189.91.38). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (2620:2d:4000:1::16). - connect (101: Network is unreachable) Could not connect to security.ubuntu.com:80 (185.125.190.36). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (2001:67c:1562::18). - connect (101: Network is unreachable) Could not connect to security.ubuntu.com:80 (2620:2d:4000:1::19). - connect (101: Network is unreachable) Could not connect to security.ubuntu.com:80 (2001:67c:1562::15). - connect (101: Network is unreachable)
  Unable to connect to archive.ubuntu.com:80:
Err:3 http://archive.ubuntu.com/ubuntu jammy-backports InRelease
  Unable to connect to archive.ubuntu.com:80:
Reading package lists...
Reading package lists...
W: Failed to fetch http://archive.ubuntu.com/ubuntu/dists/jammy/InRelease  Could not connect to archive.ubuntu.com:80 (91.189.91.38). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (91.189.91.39). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (185.125.190.36). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (2620:2d:4000:1::19). - connect (101: Network is unreachable) Could not connect to archive.ubuntu.com:80 (185.125.190.39). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (2001:67c:1562::15). - connect (101: Network is unreachable) Could not connect to archive.ubuntu.com:80 (2001:67c:1562::18). - connect (101: Network is unreachable) Could not connect to archive.ubuntu.com:80 (2620:2d:4000:1::16). - connect (101: Network is unreachable)
W: Failed to fetch http://archive.ubuntu.com/ubuntu/dists/jammy-updates/InRelease  Unable to connect to archive.ubuntu.com:80:
W: Failed to fetch http://archive.ubuntu.com/ubuntu/dists/jammy-backports/InRelease  Unable to connect to archive.ubuntu.com:80:
W: Failed to fetch http://security.ubuntu.com/ubuntu/dists/jammy-security/InRelease  Could not connect to security.ubuntu.com:80 (185.125.190.39). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (91.189.91.39). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (91.189.91.38). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (2620:2d:4000:1::16). - connect (101: Network is unreachable) Could not connect to security.ubuntu.com:80 (185.125.190.36). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (2001:67c:1562::18). - connect (101: Network is unreachable) Could not connect to security.ubuntu.com:80 (2620:2d:4000:1::19). - connect (101: Network is unreachable) Could not connect to security.ubuntu.com:80 (2001:67c:1562::15). - connect (101: Network is unreachable)
W: Some index files failed to download. They have been ignored, or old ones used instead.
Building dependency tree...
Reading state information...
Package ca-certificates is not available, but is referred to by another package.
Package ca-certificates is not available, but is referred to by another package.
This may mean that the package is missing, has been obsoleted, or
is only available from another source
Package xz-utils is not available, but is referred to by another package.
Package xz-utils is not available, but is referred to by another package.
This may mean that the package is missing, has been obsoleted, or
is only available from another source
E: Unable to locate package make
E: Unable to locate package ninja-build
E: Unable to locate package file
E: Unable to locate package curl
E: Unable to locate package curl
E: Package 'ca-certificates' has no installation candidate
E: Unable to locate package python2.7
E: Couldn't find any package by glob 'python2.7'
E: Couldn't find any package by regex 'python2.7'
E: Unable to locate package python3-pip
E: Unable to locate package python3-pkg-resources
E: Unable to locate package git
E: Unable to locate package cmake
E: Unable to locate package cmake
E: Unable to locate package sudo
E: Unable to locate package gdb
E: Package 'xz-utils' has no installation candidate
E: Unable to locate package pkg-config
E: Unable to locate package mingw-w64
E: Unable to locate package mingw-w64
The command '/bin/sh -c apt-get update && apt-get install -y --no-install-recommends   g++   make   ninja-build   file   curl   ca-certificates   python2.7   python3   python3-pip   python3-pkg-resources   git   cmake   sudo   gdb   xz-utils   libssl-dev   pkg-config   mingw-w64   && rm -rf /var/lib/apt/lists/*' returned a non-zero code: 100
Sending build context to Docker daemon  690.7kB

Step 1/10 : FROM ubuntu:22.04
 ---> 08d22c0ceb15
---
Ign:3 http://archive.ubuntu.com/ubuntu jammy-updates InRelease
Ign:2 http://security.ubuntu.com/ubuntu jammy-security InRelease
Ign:4 http://archive.ubuntu.com/ubuntu jammy-backports InRelease
Err:1 http://archive.ubuntu.com/ubuntu jammy InRelease
  Could not connect to archive.ubuntu.com:80 (185.125.190.39). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (185.125.190.36). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (91.189.91.39). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (2620:2d:4000:1::19). - connect (101: Network is unreachable) Could not connect to archive.ubuntu.com:80 (91.189.91.38). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (2620:2d:4000:1::16). - connect (101: Network is unreachable) Could not connect to archive.ubuntu.com:80 (2001:67c:1562::15). - connect (101: Network is unreachable) Could not connect to archive.ubuntu.com:80 (2001:67c:1562::18). - connect (101: Network is unreachable)
Err:2 http://security.ubuntu.com/ubuntu jammy-security InRelease
  Could not connect to security.ubuntu.com:80 (185.125.190.39). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (91.189.91.38). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (91.189.91.39). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (2620:2d:4000:1::19). - connect (101: Network is unreachable) Could not connect to security.ubuntu.com:80 (185.125.190.36). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (2620:2d:4000:1::16). - connect (101: Network is unreachable) Could not connect to security.ubuntu.com:80 (2001:67c:1562::18). - connect (101: Network is unreachable) Could not connect to security.ubuntu.com:80 (2001:67c:1562::15). - connect (101: Network is unreachable)
  Unable to connect to archive.ubuntu.com:80:
Err:4 http://archive.ubuntu.com/ubuntu jammy-backports InRelease
  Unable to connect to archive.ubuntu.com:80:
Reading package lists...
Reading package lists...
W: Failed to fetch http://archive.ubuntu.com/ubuntu/dists/jammy/InRelease  Could not connect to archive.ubuntu.com:80 (185.125.190.39). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (185.125.190.36). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (91.189.91.39). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (2620:2d:4000:1::19). - connect (101: Network is unreachable) Could not connect to archive.ubuntu.com:80 (91.189.91.38). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (2620:2d:4000:1::16). - connect (101: Network is unreachable) Could not connect to archive.ubuntu.com:80 (2001:67c:1562::15). - connect (101: Network is unreachable) Could not connect to archive.ubuntu.com:80 (2001:67c:1562::18). - connect (101: Network is unreachable)
W: Failed to fetch http://archive.ubuntu.com/ubuntu/dists/jammy-updates/InRelease  Unable to connect to archive.ubuntu.com:80:
W: Failed to fetch http://archive.ubuntu.com/ubuntu/dists/jammy-backports/InRelease  Unable to connect to archive.ubuntu.com:80:
W: Failed to fetch http://security.ubuntu.com/ubuntu/dists/jammy-security/InRelease  Could not connect to security.ubuntu.com:80 (185.125.190.39). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (91.189.91.38). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (91.189.91.39). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (2620:2d:4000:1::19). - connect (101: Network is unreachable) Could not connect to security.ubuntu.com:80 (185.125.190.36). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (2620:2d:4000:1::16). - connect (101: Network is unreachable) Could not connect to security.ubuntu.com:80 (2001:67c:1562::18). - connect (101: Network is unreachable) Could not connect to security.ubuntu.com:80 (2001:67c:1562::15). - connect (101: Network is unreachable)
W: Some index files failed to download. They have been ignored, or old ones used instead.
Building dependency tree...
Reading state information...
Package ca-certificates is not available, but is referred to by another package.
Package ca-certificates is not available, but is referred to by another package.
This may mean that the package is missing, has been obsoleted, or
is only available from another source
Package xz-utils is not available, but is referred to by another package.
Package xz-utils is not available, but is referred to by another package.
This may mean that the package is missing, has been obsoleted, or
is only available from another source
E: Unable to locate package make
E: Unable to locate package ninja-build
E: Unable to locate package file
E: Unable to locate package curl
E: Unable to locate package curl
E: Package 'ca-certificates' has no installation candidate
E: Unable to locate package python2.7
E: Couldn't find any package by glob 'python2.7'
E: Couldn't find any package by regex 'python2.7'
E: Unable to locate package python3-pip
E: Unable to locate package python3-pkg-resources
E: Unable to locate package git
E: Unable to locate package cmake
E: Unable to locate package cmake
E: Unable to locate package sudo
E: Unable to locate package gdb
E: Package 'xz-utils' has no installation candidate
E: Unable to locate package pkg-config
E: Unable to locate package mingw-w64
E: Unable to locate package mingw-w64
The command '/bin/sh -c apt-get update && apt-get install -y --no-install-recommends   g++   make   ninja-build   file   curl   ca-certificates   python2.7   python3   python3-pip   python3-pkg-resources   git   cmake   sudo   gdb   xz-utils   libssl-dev   pkg-config   mingw-w64   && rm -rf /var/lib/apt/lists/*' returned a non-zero code: 100
##[error]Process completed with exit code 1.
Post job cleanup.
