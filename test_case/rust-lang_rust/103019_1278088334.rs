plain
 ---> 30b2ec176512
Step 3/27 : RUN yum upgrade -y &&     yum install -y epel-release &&     yum install -y       automake       bzip2       file       cmake3       gcc       gcc-c++       git       glibc-devel.i686       glibc-devel.x86_64       libedit-devel       libstdc++-devel.i686       libstdc++-devel.x86_64       make       ncurses-devel       openssl-devel       patch       perl       pkgconfig       python3       unzip       wget       xz       zlib-devel.i686       zlib-devel.x86_64
 ---> Running in fe02d72037bb
Loaded plugins: fastestmirror, ovl
Determining fastest mirrors
 * base: mirror.sfo12.us.leaseweb.net
 * extras: mirror.fcix.net
 * updates: mirrors.xtom.com
--> Running transaction check
---> Package bash.x86_64 0:4.2.46-34.el7 will be updated
---> Package bash.x86_64 0:4.2.46-35.el7_9 will be an update
---> Package bind-license.noarch 32:9.11.4-26.P2.el7 will be updated
---
Upgrade  47 Packages

Total download size: 53 M
Downloading packages:
Delta RPMs disabled because /usr/bin/applydeltarpm not installed.
warning: /var/cache/yum/x86_64/7/updates/packages/bind-license-9.11.4-26.P2.el7_9.10.noarch.rpm: Header V3 RSA/SHA256 Signature, key ID f4a80eb5: NOKEY
--------------------------------------------------------------------------------
Total                                              126 MB/s |  53 MB  00:00     
Retrieving key from file:///etc/pki/rpm-gpg/RPM-GPG-KEY-CentOS-7
Importing GPG key 0xF4A80EB5:
Importing GPG key 0xF4A80EB5:
 Userid     : "CentOS-7 Key (CentOS 7 Official Signing Key) <security@centos.org>"
 Fingerprint: 6341 ab27 53d7 8a78 a7c2 7bb1 24c6 a8a7 f4a8 0eb5
 Package    : centos-release-7-9.2009.0.el7.centos.x86_64 (@CentOS)
Running transaction check
Running transaction test
Transaction test succeeded
Running transaction
---

Complete!
Loaded plugins: fastestmirror, ovl
Loading mirror speeds from cached hostfile
 * base: mirror.sfo12.us.leaseweb.net
 * extras: mirror.fcix.net
 * updates: mirrors.xtom.com
--> Running transaction check
---> Package epel-release.noarch 0:7-11 will be installed
--> Finished Dependency Resolution

---

Complete!
Loaded plugins: fastestmirror, ovl
Loading mirror speeds from cached hostfile
 * base: mirror.sfo12.us.leaseweb.net
 * epel: codingflyboy.mm.fcix.net
 * extras: mirror.fcix.net
 * updates: mirrors.xtom.com
http://mirror.es.its.nyu.edu/epel/7/x86_64/repodata/repomd.xml: [Errno 12] Timeout on http://mirror.es.its.nyu.edu/epel/7/x86_64/repodata/repomd.xml: (28, 'Connection timed out after 30001 milliseconds')
Trying other mirror.
Package 1:pkgconfig-0.27.1-4.el7.x86_64 already installed and latest version
Resolving Dependencies
--> Running transaction check
---> Package automake.noarch 0:1.13.4-3.el7 will be installed
--> Processing Dependency: autoconf >= 2.65 for package: automake-1.13.4-3.el7.noarch
---
Public key for cmake3-3.17.5-1.el7.x86_64.rpm is not installed
--------------------------------------------------------------------------------
Total                                               76 MB/s |  96 MB  00:01     
Retrieving key from file:///etc/pki/rpm-gpg/RPM-GPG-KEY-EPEL-7
Importing GPG key 0x352C64E5:
 Userid     : "Fedora EPEL (7) <epel@fedoraproject.org>"
 Fingerprint: 91e9 7d7c 4a5e 96f1 7f3e 888f 6a2f aea2 352c 64e5
 Package    : epel-release-7-11.noarch (@extras)
Running transaction check
Running transaction test
Transaction test succeeded
Running transaction
---
./build-gcc.sh: line 37: 425293 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
+ cd ..
+ rm -rf gcc-build
+ rm -rf gcc-7.5.0
+ ln /rustroot/lib/libasan.a /rustroot/lib/libatomic.a /rustroot/lib/libcilkrts.a /rustroot/lib/libgomp.a /rustroot/lib/libitm.a /rustroot/lib/libmpx.a /rustroot/lib/libmpxwrappers.a /rustroot/lib/libquadmath.a /rustroot/lib/libssp.a /rustroot/lib/libssp_nonshared.a /rustroot/lib/libstdc++.a /rustroot/lib/libstdc++fs.a /rustroot/lib/libsupc++.a /rustroot/lib/libubsan.a /rustroot/lib/libasan.so /rustroot/lib/libatomic.so /rustroot/lib/libcilkrts.so /rustroot/lib/libgcc_s.so /rustroot/lib/libgomp.so /rustroot/lib/libitm.so /rustroot/lib/libmpx.so /rustroot/lib/libmpxwrappers.so /rustroot/lib/libquadmath.so /rustroot/lib/libssp.so /rustroot/lib/libstdc++.so /rustroot/lib/libubsan.so -rst /rustroot/lib/gcc/x86_64-pc-linux-gnu/7.5.0/32/
Resolving Dependencies
--> Running transaction check
---> Package gcc.x86_64 0:4.8.5-44.el7 will be erased
---> Package gcc-c++.x86_64 0:4.8.5-44.el7 will be erased
---
  0     0    0  158M    0     0  4854k      0 --:--:--  0:00:33 --:--:-- 5982k
+ mkdir clang-build
+ cd clang-build
+ INC=/rustroot/include:/usr/include
+ hide_output cmake ../llvm -DCMAKE_C_COMPILER=/rustroot/bin/gcc -DCMAKE_CXX_COMPILER=/rustroot/bin/g++ -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/rustroot -DCOMPILER_RT_BUILD_SANITIZERS=OFF -DCOMPILER_RT_BUILD_XRAY=OFF -DCOMPILER_RT_BUILD_MEMPROF=OFF -DLLVM_TARGETS_TO_BUILD=X86 -DLLVM_INCLUDE_BENCHMARKS=OFF -DLLVM_INCLUDE_TESTS=OFF -DLLVM_INCLUDE_EXAMPLES=OFF '-DLLVM_ENABLE_PROJECTS=clang;lld;compiler-rt;bolt' -DC_INCLUDE_DIRS=/rustroot/include:/usr/include
++ nproc
+ hide_output make -j16
+ set +x
Thu Oct 13 18:29:48 UTC 2022 - building ...
---
Successfully built 62ba7a54178d
Successfully tagged rust-ci:latest
Built container sha256:62ba7a54178db1e8bc5c8ce95b9eaf5a45992b7b762604f799c12218dd6ac5c7
Uploading finished image to https://ci-caches.rust-lang.org/docker/5c565f2401206e82917b0efc91b7a535775e0ec5e6034fd4bb75ceda242a2cd19b52f90ad85075fc45b5659c1ed544194a3bd502d0927ee2eaea133d8f35f4e8
upload failed: - to s3://rust-lang-ci-sccache2/docker/5c565f2401206e82917b0efc91b7a535775e0ec5e6034fd4bb75ceda242a2cd19b52f90ad85075fc45b5659c1ed544194a3bd502d0927ee2eaea133d8f35f4e8 Unable to locate credentials
useradd: warning: the home directory already exists.
Not copying any file from skel directory into it.
[CI_JOB_NAME=dist-x86_64-linux]
---
DirectMap2M:     9211904 kB
DirectMap1G:    51380224 kB
+ python3 ../src/ci/stage-build.py python3 ../x.py dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu --include-default-paths build-manifest bootstrap
Traceback (most recent call last):
  File "../src/ci/stage-build.py", line 14, in <module>
    from typing import Dict, List, Literal, Optional, Union
ImportError: cannot import name 'Literal'
