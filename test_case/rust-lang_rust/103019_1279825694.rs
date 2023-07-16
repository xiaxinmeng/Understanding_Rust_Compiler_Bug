plain
 ---> e2a294b608ef
Step 3/27 : RUN yum upgrade -y &&     yum install -y epel-release &&     yum install -y       automake       bzip2       file       cmake3       gcc       gcc-c++       git       glibc-devel.i686       glibc-devel.x86_64       libedit-devel       libstdc++-devel.i686       libstdc++-devel.x86_64       make       ncurses-devel       openssl-devel       patch       perl       pkgconfig       python3       unzip       wget       xz       zlib-devel.i686       zlib-devel.x86_64
 ---> Running in de75f8739f56
Loaded plugins: fastestmirror, ovl
Determining fastest mirrors
 * base: mirror.fcix.net
 * extras: mirror.facebook.net
 * updates: mirror.keystealth.org
--> Running transaction check
---> Package bash.x86_64 0:4.2.46-34.el7 will be updated
---> Package bash.x86_64 0:4.2.46-35.el7_9 will be an update
---> Package bind-license.noarch 32:9.11.4-26.P2.el7 will be updated
---

Complete!
Loaded plugins: fastestmirror, ovl
Loading mirror speeds from cached hostfile
 * base: mirror.fcix.net
 * extras: mirror.facebook.net
 * updates: mirror.keystealth.org
--> Running transaction check
---> Package epel-release.noarch 0:7-11 will be installed
--> Finished Dependency Resolution

---

Complete!
Loaded plugins: fastestmirror, ovl
Loading mirror speeds from cached hostfile
 * base: mirror.fcix.net
 * epel: mirror.sfo12.us.leaseweb.net
 * extras: mirror.facebook.net
 * updates: mirror.keystealth.org
http://mirror.es.its.nyu.edu/epel/7/x86_64/repodata/repomd.xml: [Errno 12] Timeout on http://mirror.es.its.nyu.edu/epel/7/x86_64/repodata/repomd.xml: (28, 'Connection timed out after 30001 milliseconds')
Package 1:pkgconfig-0.27.1-4.el7.x86_64 already installed and latest version
Package xz-5.2.2-2.el7_9.x86_64 already installed and latest version
Resolving Dependencies
--> Running transaction check
---
Public key for cmake3-3.17.5-1.el7.x86_64.rpm is not installed
--------------------------------------------------------------------------------
Total                                               66 MB/s |  96 MB  00:01     
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
./build-gcc.sh: line 37: 425311 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
+ cd ..
+ rm -rf gcc-build
+ rm -rf gcc-7.5.0
+ ln /rustroot/lib/libasan.a /rustroot/lib/libatomic.a /rustroot/lib/libcilkrts.a /rustroot/lib/libgomp.a /rustroot/lib/libitm.a /rustroot/lib/libmpx.a /rustroot/lib/libmpxwrappers.a /rustroot/lib/libquadmath.a /rustroot/lib/libssp.a /rustroot/lib/libssp_nonshared.a /rustroot/lib/libstdc++.a /rustroot/lib/libstdc++fs.a /rustroot/lib/libsupc++.a /rustroot/lib/libubsan.a /rustroot/lib/libasan.so /rustroot/lib/libatomic.so /rustroot/lib/libcilkrts.so /rustroot/lib/libgcc_s.so /rustroot/lib/libgomp.so /rustroot/lib/libitm.so /rustroot/lib/libmpx.so /rustroot/lib/libmpxwrappers.so /rustroot/lib/libquadmath.so /rustroot/lib/libssp.so /rustroot/lib/libstdc++.so /rustroot/lib/libubsan.so -rst /rustroot/lib/gcc/x86_64-pc-linux-gnu/7.5.0/32/
Resolving Dependencies
--> Running transaction check
---> Package gcc.x86_64 0:4.8.5-44.el7 will be erased
---> Package gcc-c++.x86_64 0:4.8.5-44.el7 will be erased
---
  0     0    0  158M    0     0  5033k      0 --:--:--  0:00:32 --:--:-- 6601k
+ mkdir clang-build
+ cd clang-build
+ INC=/rustroot/include:/usr/include
+ hide_output cmake ../llvm -DCMAKE_C_COMPILER=/rustroot/bin/gcc -DCMAKE_CXX_COMPILER=/rustroot/bin/g++ -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/rustroot -DCOMPILER_RT_BUILD_SANITIZERS=OFF -DCOMPILER_RT_BUILD_XRAY=OFF -DCOMPILER_RT_BUILD_MEMPROF=OFF -DLLVM_TARGETS_TO_BUILD=X86 -DLLVM_INCLUDE_BENCHMARKS=OFF -DLLVM_INCLUDE_TESTS=OFF -DLLVM_INCLUDE_EXAMPLES=OFF '-DLLVM_ENABLE_PROJECTS=clang;lld;compiler-rt;bolt' -DC_INCLUDE_DIRS=/rustroot/include:/usr/include
++ nproc
+ hide_output make -j16
+ set +x
Sat Oct 15 19:50:29 UTC 2022 - building ...
---
Successfully built 3320157a55ba
Successfully tagged rust-ci:latest
Built container sha256:3320157a55bad184443a6084abb8bd3ee7d41b62562c9802975264ba91a1087b
Uploading finished image to https://ci-caches.rust-lang.org/docker/5c565f2401206e82917b0efc91b7a535775e0ec5e6034fd4bb75ceda242a2cd19b52f90ad85075fc45b5659c1ed544194a3bd502d0927ee2eaea133d8f35f4e8
upload failed: - to s3://rust-lang-ci-sccache2/docker/5c565f2401206e82917b0efc91b7a535775e0ec5e6034fd4bb75ceda242a2cd19b52f90ad85075fc45b5659c1ed544194a3bd502d0927ee2eaea133d8f35f4e8 Unable to locate credentials
useradd: warning: the home directory already exists.
Not copying any file from skel directory into it.
[CI_JOB_NAME=dist-x86_64-linux]
---
Hugetlb:               0 kB
DirectMap4k:      221120 kB
DirectMap2M:     9216000 kB
DirectMap1G:    50331648 kB
DEBUG:root:Running multi-stage build using Python 3.6.8 (default, Nov 16 2020, 16:55:22) 
[GCC 4.8.5 20150623 (Red Hat 4.8.5-44)]
Traceback (most recent call last):
  File "../src/ci/stage-build.py", line 282, in <module>
    pipeline.run(timer, build_args)
  File "../src/ci/stage-build.py", line 106, in run
    self.build_rustc_perf()
  File "../src/ci/stage-build.py", line 137, in build_rustc_perf
    cmd(["chown", "-R", f"{getpass.getuser()}:", self.rustc_perf_dir()])
  File "../src/ci/stage-build.py", line 183, in cmd
    logging.debug(f"Running `{' '.join(args)}`")
TypeError: sequence item 3: expected str instance, PosixPath found
