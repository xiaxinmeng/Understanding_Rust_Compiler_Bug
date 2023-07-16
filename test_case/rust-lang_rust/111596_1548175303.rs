plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:8e5e7e5ab8b370d6c329ec480221332ada57f0ab)
Download action repository 'rust-lang/simpleinfra@master' (SHA:af9a3f1b7e12a54c737d8aa371acc8d05cb83a8f)
Complete job name: PR - mingw-check-tidy
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: mingw-check-tidy
---
Ign:134 http://archive.ubuntu.com/ubuntu jammy/universe amd64 gcc-mingw-w64-x86-64 all 10.3.0-14ubuntu1+24.3
Ign:135 http://archive.ubuntu.com/ubuntu jammy/universe amd64 gcc-mingw-w64 all 10.3.0-14ubuntu1+24.3
Ign:136 http://archive.ubuntu.com/ubuntu jammy/universe amd64 mingw-w64 all 8.0.0-1
Err:87 http://archive.ubuntu.com/ubuntu jammy/universe amd64 binutils-mingw-w64-i686 amd64 2.38-3ubuntu1+9build1
  Error reading from server - read (104: Connection reset by peer) [IP: 91.189.91.38 80]
Err:88 http://archive.ubuntu.com/ubuntu jammy/universe amd64 mingw-w64-common all 8.0.0-1
  Could not connect to archive.ubuntu.com:80 (91.189.91.39). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (91.189.91.38). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (185.125.190.39). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (185.125.190.36). - connect (111: Connection refused) [IP: 91.189.91.39 80]
Err:89 http://archive.ubuntu.com/ubuntu jammy/universe amd64 mingw-w64-i686-dev all 8.0.0-1
  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
Err:90 http://archive.ubuntu.com/ubuntu jammy/universe amd64 gcc-mingw-w64-base amd64 10.3.0-14ubuntu1+24.3
  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
Err:91 http://archive.ubuntu.com/ubuntu jammy/universe amd64 gcc-mingw-w64-i686-posix-runtime amd64 10.3.0-14ubuntu1+24.3
  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
Err:92 http://archive.ubuntu.com/ubuntu jammy/universe amd64 gcc-mingw-w64-i686-posix amd64 10.3.0-14ubuntu1+24.3
  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
Err:93 http://archive.ubuntu.com/ubuntu jammy/universe amd64 g++-mingw-w64-i686-posix amd64 10.3.0-14ubuntu1+24.3
  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
Err:94 http://archive.ubuntu.com/ubuntu jammy/universe amd64 gcc-mingw-w64-i686-win32-runtime amd64 10.3.0-14ubuntu1+24.3
  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
Err:95 http://archive.ubuntu.com/ubuntu jammy/universe amd64 gcc-mingw-w64-i686-win32 amd64 10.3.0-14ubuntu1+24.3
  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
Err:96 http://archive.ubuntu.com/ubuntu jammy/universe amd64 g++-mingw-w64-i686-win32 amd64 10.3.0-14ubuntu1+24.3
  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
Err:97 http://archive.ubuntu.com/ubuntu jammy/universe amd64 binutils-mingw-w64-x86-64 amd64 2.38-3ubuntu1+9build1
  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
Err:98 http://archive.ubuntu.com/ubuntu jammy/universe amd64 mingw-w64-x86-64-dev all 8.0.0-1
  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
Err:99 http://archive.ubuntu.com/ubuntu jammy/universe amd64 gcc-mingw-w64-x86-64-posix-runtime amd64 10.3.0-14ubuntu1+24.3
  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
Err:100 http://archive.ubuntu.com/ubuntu jammy/universe amd64 gcc-mingw-w64-x86-64-posix amd64 10.3.0-14ubuntu1+24.3
  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
Err:101 http://archive.ubuntu.com/ubuntu jammy/universe amd64 g++-mingw-w64-x86-64-posix amd64 10.3.0-14ubuntu1+24.3
  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
Err:102 http://archive.ubuntu.com/ubuntu jammy/universe amd64 gcc-mingw-w64-x86-64-win32-runtime amd64 10.3.0-14ubuntu1+24.3
  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
Err:103 http://archive.ubuntu.com/ubuntu jammy/universe amd64 gcc-mingw-w64-x86-64-win32 amd64 10.3.0-14ubuntu1+24.3
  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
Err:104 http://archive.ubuntu.com/ubuntu jammy/universe amd64 g++-mingw-w64-x86-64-win32 amd64 10.3.0-14ubuntu1+24.3
  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
Err:105 http://archive.ubuntu.com/ubuntu jammy/main amd64 libdw1 amd64 0.186-1build1
  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
Err:106 http://archive.ubuntu.com/ubuntu jammy/main amd64 libbabeltrace1 amd64 1.5.8-2build1
  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
Err:108 http://archive.ubuntu.com/ubuntu jammy/main amd64 libdebuginfod1 amd64 0.186-1build1
Err:108 http://archive.ubuntu.com/ubuntu jammy/main amd64 libdebuginfod1 amd64 0.186-1build1
  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
Err:109 http://archive.ubuntu.com/ubuntu jammy/main amd64 libipt2 amd64 2.0.5-1
  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
Err:111 http://archive.ubuntu.com/ubuntu jammy/main amd64 libsource-highlight-common all 3.1.9-4.1build2
Err:111 http://archive.ubuntu.com/ubuntu jammy/main amd64 libsource-highlight-common all 3.1.9-4.1build2
  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
Err:112 http://archive.ubuntu.com/ubuntu jammy/main amd64 libboost-regex1.74.0 amd64 1.74.0-14ubuntu3
  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
Err:113 http://archive.ubuntu.com/ubuntu jammy/main amd64 libsource-highlight4v5 amd64 3.1.9-4.1build2
  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
Err:114 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 gdb amd64 12.1-0ubuntu1~22.04
  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
Err:115 http://archive.ubuntu.com/ubuntu jammy/main amd64 liberror-perl all 0.17029-1
  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
Ign:117 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 git amd64 1:2.34.1-1ubuntu1.9
Ign:118 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libdpkg-perl all 1.21.1ubuntu2.1
Ign:119 http://archive.ubuntu.com/ubuntu jammy-updates/universe amd64 libpython2.7-stdlib amd64 2.7.18-13ubuntu1.1
Ign:120 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libssl-dev amd64 3.0.2-0ubuntu1.9
Ign:120 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 libssl-dev amd64 3.0.2-0ubuntu1.9
Err:121 http://archive.ubuntu.com/ubuntu jammy/main amd64 make amd64 4.3-4.1build1
  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
Err:122 http://archive.ubuntu.com/ubuntu jammy/universe amd64 ninja-build amd64 1.10.1-1
  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
Err:123 http://archive.ubuntu.com/ubuntu jammy/main amd64 pkg-config amd64 0.29.2-1ubuntu3
  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
Err:125 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 python3-lib2to3 all 3.10.6-1~22.04
Err:125 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 python3-lib2to3 all 3.10.6-1~22.04
  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
Err:126 http://archive.ubuntu.com/ubuntu jammy-updates/main amd64 python3-distutils all 3.10.6-1~22.04
  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
Ign:128 http://archive.ubuntu.com/ubuntu jammy-updates/universe amd64 python3-wheel all 0.37.1-2ubuntu0.22.04.1
Ign:129 http://archive.ubuntu.com/ubuntu jammy-updates/universe amd64 python3-pip all 22.0.2+dfsg-1ubuntu0.2
Err:130 http://archive.ubuntu.com/ubuntu jammy/universe amd64 g++-mingw-w64-i686 all 10.3.0-14ubuntu1+24.3
Err:130 http://archive.ubuntu.com/ubuntu jammy/universe amd64 g++-mingw-w64-i686 all 10.3.0-14ubuntu1+24.3
  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
Err:131 http://archive.ubuntu.com/ubuntu jammy/universe amd64 g++-mingw-w64-x86-64 all 10.3.0-14ubuntu1+24.3
  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
Err:132 http://archive.ubuntu.com/ubuntu jammy/universe amd64 g++-mingw-w64 all 10.3.0-14ubuntu1+24.3
  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
Err:133 http://archive.ubuntu.com/ubuntu jammy/universe amd64 gcc-mingw-w64-i686 all 10.3.0-14ubuntu1+24.3
  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
Err:134 http://archive.ubuntu.com/ubuntu jammy/universe amd64 gcc-mingw-w64-x86-64 all 10.3.0-14ubuntu1+24.3
  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
Err:135 http://archive.ubuntu.com/ubuntu jammy/universe amd64 gcc-mingw-w64 all 10.3.0-14ubuntu1+24.3
  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
Err:136 http://archive.ubuntu.com/ubuntu jammy/universe amd64 mingw-w64 all 8.0.0-1
  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
Err:107 http://security.ubuntu.com/ubuntu jammy-updates/main amd64 libcurl3-gnutls amd64 7.81.0-1ubuntu1.10
  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
Err:110 http://security.ubuntu.com/ubuntu jammy-updates/main amd64 libpython3.10 amd64 3.10.6-1~22.04.2ubuntu1
  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
Err:116 http://security.ubuntu.com/ubuntu jammy-updates/main amd64 git-man all 1:2.34.1-1ubuntu1.9
  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
Err:117 http://security.ubuntu.com/ubuntu jammy-updates/main amd64 git amd64 1:2.34.1-1ubuntu1.9
  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
Err:118 http://security.ubuntu.com/ubuntu jammy-updates/main amd64 libdpkg-perl all 1.21.1ubuntu2.1
  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
Err:119 http://security.ubuntu.com/ubuntu jammy-updates/universe amd64 libpython2.7-stdlib amd64 2.7.18-13ubuntu1.1
  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
Err:120 http://security.ubuntu.com/ubuntu jammy-updates/main amd64 libssl-dev amd64 3.0.2-0ubuntu1.9
  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
Err:124 http://security.ubuntu.com/ubuntu jammy-updates/universe amd64 python2.7 amd64 2.7.18-13ubuntu1.1
  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
Err:127 http://security.ubuntu.com/ubuntu jammy-updates/main amd64 python3-setuptools all 59.6.0-1.2ubuntu0.22.04.1
  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
Err:128 http://security.ubuntu.com/ubuntu jammy-updates/universe amd64 python3-wheel all 0.37.1-2ubuntu0.22.04.1
  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
Err:129 http://security.ubuntu.com/ubuntu jammy-updates/universe amd64 python3-pip all 22.0.2+dfsg-1ubuntu0.2
  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
Fetched 104 MB in 4min 34s (378 kB/s)
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/universe/b/binutils-mingw-w64/binutils-mingw-w64-i686_2.38-3ubuntu1%2b9build1_amd64.deb  Error reading from server - read (104: Connection reset by peer) [IP: 91.189.91.38 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/universe/m/mingw-w64/mingw-w64-common_8.0.0-1_all.deb  Could not connect to archive.ubuntu.com:80 (91.189.91.39). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (91.189.91.38). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (185.125.190.39). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (185.125.190.36). - connect (111: Connection refused) [IP: 91.189.91.39 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/universe/m/mingw-w64/mingw-w64-i686-dev_8.0.0-1_all.deb  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/universe/g/gcc-mingw-w64/gcc-mingw-w64-base_10.3.0-14ubuntu1%2b24.3_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/universe/g/gcc-mingw-w64/gcc-mingw-w64-i686-posix-runtime_10.3.0-14ubuntu1%2b24.3_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/universe/g/gcc-mingw-w64/gcc-mingw-w64-i686-posix_10.3.0-14ubuntu1%2b24.3_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/universe/g/gcc-mingw-w64/g%2b%2b-mingw-w64-i686-posix_10.3.0-14ubuntu1%2b24.3_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/universe/g/gcc-mingw-w64/gcc-mingw-w64-i686-win32-runtime_10.3.0-14ubuntu1%2b24.3_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/universe/g/gcc-mingw-w64/gcc-mingw-w64-i686-win32_10.3.0-14ubuntu1%2b24.3_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/universe/g/gcc-mingw-w64/g%2b%2b-mingw-w64-i686-win32_10.3.0-14ubuntu1%2b24.3_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/universe/b/binutils-mingw-w64/binutils-mingw-w64-x86-64_2.38-3ubuntu1%2b9build1_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/universe/m/mingw-w64/mingw-w64-x86-64-dev_8.0.0-1_all.deb  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/universe/g/gcc-mingw-w64/gcc-mingw-w64-x86-64-posix-runtime_10.3.0-14ubuntu1%2b24.3_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/universe/g/gcc-mingw-w64/gcc-mingw-w64-x86-64-posix_10.3.0-14ubuntu1%2b24.3_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/universe/g/gcc-mingw-w64/g%2b%2b-mingw-w64-x86-64-posix_10.3.0-14ubuntu1%2b24.3_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/universe/g/gcc-mingw-w64/gcc-mingw-w64-x86-64-win32-runtime_10.3.0-14ubuntu1%2b24.3_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/universe/g/gcc-mingw-w64/gcc-mingw-w64-x86-64-win32_10.3.0-14ubuntu1%2b24.3_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/universe/g/gcc-mingw-w64/g%2b%2b-mingw-w64-x86-64-win32_10.3.0-14ubuntu1%2b24.3_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/main/e/elfutils/libdw1_0.186-1build1_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/main/b/babeltrace/libbabeltrace1_1.5.8-2build1_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
E: Failed to fetch http://security.ubuntu.com/ubuntu/pool/main/c/curl/libcurl3-gnutls_7.81.0-1ubuntu1.10_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/main/e/elfutils/libdebuginfod1_0.186-1build1_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/main/i/intel-processor-trace/libipt2_2.0.5-1_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
E: Failed to fetch http://security.ubuntu.com/ubuntu/pool/main/p/python3.10/libpython3.10_3.10.6-1%7e22.04.2ubuntu1_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/main/s/source-highlight/libsource-highlight-common_3.1.9-4.1build2_all.deb  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/main/b/boost1.74/libboost-regex1.74.0_1.74.0-14ubuntu3_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/main/s/source-highlight/libsource-highlight4v5_3.1.9-4.1build2_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/main/g/gdb/gdb_12.1-0ubuntu1%7e22.04_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/main/libe/liberror-perl/liberror-perl_0.17029-1_all.deb  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
E: Failed to fetch http://security.ubuntu.com/ubuntu/pool/main/g/git/git-man_2.34.1-1ubuntu1.9_all.deb  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
E: Failed to fetch http://security.ubuntu.com/ubuntu/pool/main/g/git/git_2.34.1-1ubuntu1.9_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
E: Failed to fetch http://security.ubuntu.com/ubuntu/pool/main/d/dpkg/libdpkg-perl_1.21.1ubuntu2.1_all.deb  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
E: Failed to fetch http://security.ubuntu.com/ubuntu/pool/universe/p/python2.7/libpython2.7-stdlib_2.7.18-13ubuntu1.1_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
E: Failed to fetch http://security.ubuntu.com/ubuntu/pool/main/o/openssl/libssl-dev_3.0.2-0ubuntu1.9_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/main/m/make-dfsg/make_4.3-4.1build1_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/universe/n/ninja-build/ninja-build_1.10.1-1_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/main/p/pkg-config/pkg-config_0.29.2-1ubuntu3_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
E: Failed to fetch http://security.ubuntu.com/ubuntu/pool/universe/p/python2.7/python2.7_2.7.18-13ubuntu1.1_amd64.deb  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/main/p/python3-stdlib-extensions/python3-lib2to3_3.10.6-1%7e22.04_all.deb  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/main/p/python3-stdlib-extensions/python3-distutils_3.10.6-1%7e22.04_all.deb  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
E: Failed to fetch http://security.ubuntu.com/ubuntu/pool/main/s/setuptools/python3-setuptools_59.6.0-1.2ubuntu0.22.04.1_all.deb  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
E: Failed to fetch http://security.ubuntu.com/ubuntu/pool/universe/w/wheel/python3-wheel_0.37.1-2ubuntu0.22.04.1_all.deb  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
E: Failed to fetch http://security.ubuntu.com/ubuntu/pool/universe/p/python-pip/python3-pip_22.0.2%2bdfsg-1ubuntu0.2_all.deb  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/universe/g/gcc-mingw-w64/g%2b%2b-mingw-w64-i686_10.3.0-14ubuntu1%2b24.3_all.deb  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/universe/g/gcc-mingw-w64/g%2b%2b-mingw-w64-x86-64_10.3.0-14ubuntu1%2b24.3_all.deb  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/universe/g/gcc-mingw-w64/g%2b%2b-mingw-w64_10.3.0-14ubuntu1%2b24.3_all.deb  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/universe/g/gcc-mingw-w64/gcc-mingw-w64-i686_10.3.0-14ubuntu1%2b24.3_all.deb  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/universe/g/gcc-mingw-w64/gcc-mingw-w64-x86-64_10.3.0-14ubuntu1%2b24.3_all.deb  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/universe/g/gcc-mingw-w64/gcc-mingw-w64_10.3.0-14ubuntu1%2b24.3_all.deb  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
E: Failed to fetch http://archive.ubuntu.com/ubuntu/pool/universe/m/mingw-w64/mingw-w64_8.0.0-1_all.deb  Unable to connect to archive.ubuntu.com:80: [IP: 91.189.91.39 80]
E: Unable to fetch some archives, maybe run apt-get update or try with --fix-missing?
The command '/bin/sh -c apt-get update && apt-get install -y --no-install-recommends   g++   make   ninja-build   file   curl   ca-certificates   python2.7   python3   python3-pip   python3-pkg-resources   git   cmake   sudo   gdb   xz-utils   libssl-dev   pkg-config   mingw-w64   && rm -rf /var/lib/apt/lists/*' returned a non-zero code: 100
Sending build context to Docker daemon  423.9kB

Step 1/10 : FROM ubuntu:22.04
 ---> 3b418d7b466a
---
Ign:2 http://archive.ubuntu.com/ubuntu jammy-updates InRelease
Ign:3 http://archive.ubuntu.com/ubuntu jammy-backports InRelease
Ign:4 http://security.ubuntu.com/ubuntu jammy-security InRelease
Err:1 http://archive.ubuntu.com/ubuntu jammy InRelease
  Could not connect to archive.ubuntu.com:80 (185.125.190.39). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (185.125.190.36). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (91.189.91.39). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (2620:2d:4000:1::19). - connect (101: Network is unreachable) Could not connect to archive.ubuntu.com:80 (91.189.91.38). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (2620:2d:4000:1::16). - connect (101: Network is unreachable) Could not connect to archive.ubuntu.com:80 (2001:67c:1562::18). - connect (101: Network is unreachable)
  Unable to connect to archive.ubuntu.com:80:
Err:3 http://archive.ubuntu.com/ubuntu jammy-backports InRelease
  Unable to connect to archive.ubuntu.com:80:
Err:4 http://security.ubuntu.com/ubuntu jammy-security InRelease
Err:4 http://security.ubuntu.com/ubuntu jammy-security InRelease
  Could not connect to security.ubuntu.com:80 (185.125.190.39). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (185.125.190.36). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (91.189.91.39). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (2620:2d:4000:1::19). - connect (101: Network is unreachable) Could not connect to security.ubuntu.com:80 (2620:2d:4000:1::16). - connect (101: Network is unreachable) Could not connect to security.ubuntu.com:80 (2001:67c:1562::18). - connect (101: Network is unreachable)
Reading package lists...
W: Failed to fetch http://archive.ubuntu.com/ubuntu/dists/jammy/InRelease  Could not connect to archive.ubuntu.com:80 (185.125.190.39). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (185.125.190.36). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (91.189.91.39). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (2620:2d:4000:1::19). - connect (101: Network is unreachable) Could not connect to archive.ubuntu.com:80 (91.189.91.38). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (2620:2d:4000:1::16). - connect (101: Network is unreachable) Could not connect to archive.ubuntu.com:80 (2001:67c:1562::18). - connect (101: Network is unreachable)
W: Failed to fetch http://archive.ubuntu.com/ubuntu/dists/jammy-updates/InRelease  Unable to connect to archive.ubuntu.com:80:
W: Failed to fetch http://archive.ubuntu.com/ubuntu/dists/jammy-backports/InRelease  Unable to connect to archive.ubuntu.com:80:
W: Failed to fetch http://security.ubuntu.com/ubuntu/dists/jammy-security/InRelease  Could not connect to security.ubuntu.com:80 (185.125.190.39). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (185.125.190.36). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (91.189.91.39). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (2620:2d:4000:1::19). - connect (101: Network is unreachable) Could not connect to security.ubuntu.com:80 (2620:2d:4000:1::16). - connect (101: Network is unreachable) Could not connect to security.ubuntu.com:80 (2001:67c:1562::18). - connect (101: Network is unreachable)
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
Sending build context to Docker daemon  423.9kB

Step 1/10 : FROM ubuntu:22.04
 ---> 3b418d7b466a
---
Ign:2 http://archive.ubuntu.com/ubuntu jammy-updates InRelease
Ign:3 http://archive.ubuntu.com/ubuntu jammy-backports InRelease
Ign:4 http://security.ubuntu.com/ubuntu jammy-security InRelease
Err:1 http://archive.ubuntu.com/ubuntu jammy InRelease
  Could not connect to archive.ubuntu.com:80 (185.125.190.39). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (91.189.91.38). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (91.189.91.39). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (2620:2d:4000:1::19). - connect (101: Network is unreachable) Could not connect to archive.ubuntu.com:80 (185.125.190.36). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (2620:2d:4000:1::16). - connect (101: Network is unreachable) Could not connect to archive.ubuntu.com:80 (2001:67c:1562::18). - connect (101: Network is unreachable)
  Unable to connect to archive.ubuntu.com:80:
Err:3 http://archive.ubuntu.com/ubuntu jammy-backports InRelease
  Unable to connect to archive.ubuntu.com:80:
Err:4 http://security.ubuntu.com/ubuntu jammy-security InRelease
Err:4 http://security.ubuntu.com/ubuntu jammy-security InRelease
  Could not connect to security.ubuntu.com:80 (185.125.190.36). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (91.189.91.39). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (185.125.190.39). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (2620:2d:4000:1::16). - connect (101: Network is unreachable) Could not connect to security.ubuntu.com:80 (2001:67c:1562::18). - connect (101: Network is unreachable) Could not connect to security.ubuntu.com:80 (2620:2d:4000:1::19). - connect (101: Network is unreachable)
Reading package lists...
W: Failed to fetch http://archive.ubuntu.com/ubuntu/dists/jammy/InRelease  Could not connect to archive.ubuntu.com:80 (185.125.190.39). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (91.189.91.38). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (91.189.91.39). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (2620:2d:4000:1::19). - connect (101: Network is unreachable) Could not connect to archive.ubuntu.com:80 (185.125.190.36). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (2620:2d:4000:1::16). - connect (101: Network is unreachable) Could not connect to archive.ubuntu.com:80 (2001:67c:1562::18). - connect (101: Network is unreachable)
W: Failed to fetch http://archive.ubuntu.com/ubuntu/dists/jammy-updates/InRelease  Unable to connect to archive.ubuntu.com:80:
W: Failed to fetch http://archive.ubuntu.com/ubuntu/dists/jammy-backports/InRelease  Unable to connect to archive.ubuntu.com:80:
W: Failed to fetch http://security.ubuntu.com/ubuntu/dists/jammy-security/InRelease  Could not connect to security.ubuntu.com:80 (185.125.190.36). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (91.189.91.39). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (185.125.190.39). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (2620:2d:4000:1::16). - connect (101: Network is unreachable) Could not connect to security.ubuntu.com:80 (2001:67c:1562::18). - connect (101: Network is unreachable) Could not connect to security.ubuntu.com:80 (2620:2d:4000:1::19). - connect (101: Network is unreachable)
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
Sending build context to Docker daemon  423.9kB

Step 1/10 : FROM ubuntu:22.04
 ---> 3b418d7b466a
---
Ign:3 http://archive.ubuntu.com/ubuntu jammy-updates InRelease
Ign:4 http://archive.ubuntu.com/ubuntu jammy-backports InRelease
Ign:2 http://security.ubuntu.com/ubuntu jammy-security InRelease
Err:1 http://archive.ubuntu.com/ubuntu jammy InRelease
  Could not connect to archive.ubuntu.com:80 (91.189.91.39). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (185.125.190.39). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (185.125.190.36). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (2001:67c:1562::18). - connect (101: Network is unreachable) Could not connect to archive.ubuntu.com:80 (91.189.91.38). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (2620:2d:4000:1::19). - connect (101: Network is unreachable) Could not connect to archive.ubuntu.com:80 (2620:2d:4000:1::16). - connect (101: Network is unreachable)
  Unable to connect to archive.ubuntu.com:80:
Err:4 http://archive.ubuntu.com/ubuntu jammy-backports InRelease
  Unable to connect to archive.ubuntu.com:80:
Err:2 http://security.ubuntu.com/ubuntu jammy-security InRelease
Err:2 http://security.ubuntu.com/ubuntu jammy-security InRelease
  Could not connect to security.ubuntu.com:80 (91.189.91.39). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (185.125.190.36). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (185.125.190.39). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (2001:67c:1562::18). - connect (101: Network is unreachable) Could not connect to security.ubuntu.com:80 (2620:2d:4000:1::19). - connect (101: Network is unreachable) Could not connect to security.ubuntu.com:80 (2620:2d:4000:1::16). - connect (101: Network is unreachable)
Reading package lists...
W: Failed to fetch http://archive.ubuntu.com/ubuntu/dists/jammy/InRelease  Could not connect to archive.ubuntu.com:80 (91.189.91.39). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (185.125.190.39). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (185.125.190.36). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (2001:67c:1562::18). - connect (101: Network is unreachable) Could not connect to archive.ubuntu.com:80 (91.189.91.38). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (2620:2d:4000:1::19). - connect (101: Network is unreachable) Could not connect to archive.ubuntu.com:80 (2620:2d:4000:1::16). - connect (101: Network is unreachable)
W: Failed to fetch http://archive.ubuntu.com/ubuntu/dists/jammy-updates/InRelease  Unable to connect to archive.ubuntu.com:80:
W: Failed to fetch http://archive.ubuntu.com/ubuntu/dists/jammy-backports/InRelease  Unable to connect to archive.ubuntu.com:80:
W: Failed to fetch http://security.ubuntu.com/ubuntu/dists/jammy-security/InRelease  Could not connect to security.ubuntu.com:80 (91.189.91.39). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (185.125.190.36). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (185.125.190.39). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (2001:67c:1562::18). - connect (101: Network is unreachable) Could not connect to security.ubuntu.com:80 (2620:2d:4000:1::19). - connect (101: Network is unreachable) Could not connect to security.ubuntu.com:80 (2620:2d:4000:1::16). - connect (101: Network is unreachable)
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
Sending build context to Docker daemon  423.9kB

Step 1/10 : FROM ubuntu:22.04
 ---> 3b418d7b466a
---
Ign:2 http://archive.ubuntu.com/ubuntu jammy-updates InRelease
Ign:3 http://security.ubuntu.com/ubuntu jammy-security InRelease
Ign:4 http://archive.ubuntu.com/ubuntu jammy-backports InRelease
Err:1 http://archive.ubuntu.com/ubuntu jammy InRelease
  Could not connect to archive.ubuntu.com:80 (91.189.91.39). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (185.125.190.36). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (185.125.190.39). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (2620:2d:4000:1::19). - connect (101: Network is unreachable) Could not connect to archive.ubuntu.com:80 (91.189.91.38). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (2620:2d:4000:1::16). - connect (101: Network is unreachable) Could not connect to archive.ubuntu.com:80 (2001:67c:1562::18). - connect (101: Network is unreachable)
  Unable to connect to archive.ubuntu.com:80:
Err:4 http://archive.ubuntu.com/ubuntu jammy-backports InRelease
  Unable to connect to archive.ubuntu.com:80:
Err:3 http://security.ubuntu.com/ubuntu jammy-security InRelease
Err:3 http://security.ubuntu.com/ubuntu jammy-security InRelease
  Could not connect to security.ubuntu.com:80 (185.125.190.39). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (91.189.91.39). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (185.125.190.36). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (2620:2d:4000:1::16). - connect (101: Network is unreachable) Could not connect to security.ubuntu.com:80 (2001:67c:1562::18). - connect (101: Network is unreachable) Could not connect to security.ubuntu.com:80 (2620:2d:4000:1::19). - connect (101: Network is unreachable)
Reading package lists...
W: Failed to fetch http://archive.ubuntu.com/ubuntu/dists/jammy/InRelease  Could not connect to archive.ubuntu.com:80 (91.189.91.39). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (185.125.190.36). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (185.125.190.39). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (2620:2d:4000:1::19). - connect (101: Network is unreachable) Could not connect to archive.ubuntu.com:80 (91.189.91.38). - connect (111: Connection refused) Could not connect to archive.ubuntu.com:80 (2620:2d:4000:1::16). - connect (101: Network is unreachable) Could not connect to archive.ubuntu.com:80 (2001:67c:1562::18). - connect (101: Network is unreachable)
W: Failed to fetch http://archive.ubuntu.com/ubuntu/dists/jammy-updates/InRelease  Unable to connect to archive.ubuntu.com:80:
W: Failed to fetch http://archive.ubuntu.com/ubuntu/dists/jammy-backports/InRelease  Unable to connect to archive.ubuntu.com:80:
W: Failed to fetch http://security.ubuntu.com/ubuntu/dists/jammy-security/InRelease  Could not connect to security.ubuntu.com:80 (185.125.190.39). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (91.189.91.39). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (185.125.190.36). - connect (111: Connection refused) Could not connect to security.ubuntu.com:80 (2620:2d:4000:1::16). - connect (101: Network is unreachable) Could not connect to security.ubuntu.com:80 (2001:67c:1562::18). - connect (101: Network is unreachable) Could not connect to security.ubuntu.com:80 (2620:2d:4000:1::19). - connect (101: Network is unreachable)
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
