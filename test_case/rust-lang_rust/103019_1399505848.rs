plain
 ---> Running in ba0efdcc5375
Loaded plugins: fastestmirror, ovl
Determining fastest mirrors
 * base: forksystems.mm.fcix.net
 * extras: mirror.cs.vt.edu
 * updates: mirror.umd.edu
--> Running transaction check
---> Package bash.x86_64 0:4.2.46-34.el7 will be updated
---> Package bash.x86_64 0:4.2.46-35.el7_9 will be an update
---> Package bind-license.noarch 32:9.11.4-26.P2.el7 will be updated
---
Complete!
Loaded plugins: fastestmirror, ovl
Loading mirror speeds from cached hostfile
 * base: forksystems.mm.fcix.net
 * extras: mirror.cs.vt.edu
 * updates: mirror.umd.edu
--> Running transaction check
---> Package epel-release.noarch 0:7-11 will be installed
--> Finished Dependency Resolution

---
Complete!
Loaded plugins: fastestmirror, ovl
Loading mirror speeds from cached hostfile
 * base: forksystems.mm.fcix.net
 * epel: dl.fedoraproject.org
 * extras: mirror.cs.vt.edu
 * updates: mirror.umd.edu
Trying other mirror.
Package 1:pkgconfig-0.27.1-4.el7.x86_64 already installed and latest version
Package xz-5.2.2-2.el7_9.x86_64 already installed and latest version
Resolving Dependencies
---
Step 21/27 : ENV HOSTS=x86_64-unknown-linux-gnu
 ---> Running in addbe4ba2d5d
Removing intermediate container addbe4ba2d5d
 ---> e9ab91f94e18
Step 22/27 : ENV RUST_CONFIGURE_ARGS       --enable-full-tools       --enable-sanitizers       --enable-profiler       --enable-compiler-docs       --set target.x86_64-unknown-linux-gnu.linker=clang       --set target.x86_64-unknown-linux-gnu.ar=/rustroot/bin/llvm-ar       --set target.x86_64-unknown-linux-gnu.ranlib=/rustroot/bin/llvm-ranlib       --set llvm.thin-lto=true       --set llvm.ninja=false       --set rust.jemalloc       --set rust.use-lld=true       --set rust.lto=thin       --set color=never
Removing intermediate container 83e3bf5d060a
 ---> 13d8917110c5
Step 23/27 : ENV SCRIPT python3 ../src/ci/stage-build.py python3 ../x.py dist     --host $HOSTS --target $HOSTS     --include-default-paths     build-manifest bootstrap
 ---> Running in d75d751b02be
---
Successfully built d3e1d82ec519
Successfully tagged rust-ci:latest
Built container sha256:d3e1d82ec519a94578c4ebb276e5afdda7cdfea8b4c3b7354b6abe34fd1066c1
Uploading finished image to https://ci-caches.rust-lang.org/docker/823103076290c69391baceb11cda9ade8a991dd74bc6e7ae3f2168b474a101bd0b1f17f7d0a09f1f9e72e2729029ec765f2affa4b7b37d180fa7b353616eaf97
upload failed: - to s3://rust-lang-ci-sccache2/docker/823103076290c69391baceb11cda9ade8a991dd74bc6e7ae3f2168b474a101bd0b1f17f7d0a09f1f9e72e2729029ec765f2affa4b7b37d180fa7b353616eaf97 Unable to locate credentials
useradd: warning: the home directory already exists.
Not copying any file from skel directory into it.
[CI_JOB_NAME=dist-x86_64-linux]
---
sccache: Starting the server...
Traceback (most recent call last):
configure: processing command line
configure: 
  File "/checkout/src/bootstrap/configure.py", line 465, in <module>
configure: rust.lld             := True
configure: rust.llvm-tools      := True
configure: rust.llvm-tools      := True
    raise RuntimeError("config key {} not in sections".format(section_key))
RuntimeError: config key color not in sections
configure: build.sanitizers     := True
configure: build.profiler       := True
configure: build.compiler-docs  := True
configure: target.x86_64-unknown-linux-gnu.linker := clang
