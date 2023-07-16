plain
[00:01:22] Status: Downloaded newer image for ubuntu:18.04
[00:01:22]  ---> eb780ea9cf9e
[00:01:22] Step 2/10 : RUN apt-get update && apt-get install -y --no-install-recommends   clang   make   file   curl   ca-certificates   python2.7   git   cmake   sudo   bzip2   xz-utils   wget   libssl-dev   pkg-config
[00:01:33]  ---> Running in 04aba428576a
[00:01:34] standard_init_linux.go:190: exec user process caused "exec format error"
[00:01:34] The command '/bin/sh -c apt-get update && apt-get install -y --no-install-recommends   clang   make   file   curl   ca-certificates   python2.7   git   cmake   sudo   bzip2   xz-utils   wget   libssl-dev   pkg-config' returned a non-zero code: 1
[00:01:35] Sending build context to Docker daemon  520.7kB
[00:01:35] Step 1/10 : FROM ubuntu:18.04
[00:01:35]  ---> eb780ea9cf9e
[00:01:35] Step 2/10 : RUN apt-get update && apt-get install -y --no-install-recommends   clang   make   file   curl   ca-certificates   python2.7   git   cmake   sudo   bzip2   xz-utils   wget   libssl-dev   pkg-config
[00:01:35] Step 2/10 : RUN apt-get update && apt-get install -y --no-install-recommends   clang   make   file   curl   ca-certificates   python2.7   git   cmake   sudo   bzip2   xz-utils   wget   libssl-dev   pkg-config
[00:01:35]  ---> Running in 9751e7028309
[00:01:36] standard_init_linux.go:190: exec user process caused "exec format error"
[00:01:36] The command '/bin/sh -c apt-get update && apt-get install -y --no-install-recommends   clang   make   file   curl   ca-certificates   python2.7   git   cmake   sudo   bzip2   xz-utils   wget   libssl-dev   pkg-config' returned a non-zero code: 1
[00:01:38] Sending build context to Docker daemon  520.7kB
[00:01:38] Step 1/10 : FROM ubuntu:18.04
[00:01:38]  ---> eb780ea9cf9e
[00:01:38] Step 2/10 : RUN apt-get update && apt-get install -y --no-install-recommends   clang   make   file   curl   ca-certificates   python2.7   git   cmake   sudo   bzip2   xz-utils   wget   libssl-dev   pkg-config
[00:01:38] Step 2/10 : RUN apt-get update && apt-get install -y --no-install-recommends   clang   make   file   curl   ca-certificates   python2.7   git   cmake   sudo   bzip2   xz-utils   wget   libssl-dev   pkg-config
[00:01:38]  ---> Running in fe353080ff30
[00:01:39] standard_init_linux.go:190: exec user process caused "exec format error"
[00:01:39] The command '/bin/sh -c apt-get update && apt-get install -y --no-install-recommends   clang   make   file   curl   ca-certificates   python2.7   git   cmake   sudo   bzip2   xz-utils   wget   libssl-dev   pkg-config' returned a non-zero code: 1
[00:01:42] Sending build context to Docker daemon  520.7kB
[00:01:42] Step 1/10 : FROM ubuntu:18.04
[00:01:42]  ---> eb780ea9cf9e
[00:01:42] Step 2/10 : RUN apt-get update && apt-get install -y --no-install-recommends   clang   make   file   curl   ca-certificates   python2.7   git   cmake   sudo   bzip2   xz-utils   wget   libssl-dev   pkg-config
[00:01:42] Step 2/10 : RUN apt-get update && apt-get install -y --no-install-recommends   clang   make   file   curl   ca-certificates   python2.7   git   cmake   sudo   bzip2   xz-utils   wget   libssl-dev   pkg-config
[00:01:42]  ---> Running in ea698d382b11
[00:01:43] standard_init_linux.go:190: exec user process caused "exec format error"
[00:01:43] The command '/bin/sh -c apt-get update && apt-get install -y --no-install-recommends   clang   make   file   curl   ca-certificates   python2.7   git   cmake   sudo   bzip2   xz-utils   wget   libssl-dev   pkg-config' returned a non-zero code: 1
[00:01:47] Sending build context to Docker daemon  520.7kB
[00:01:47] Step 1/10 : FROM ubuntu:18.04
[00:01:47]  ---> eb780ea9cf9e
[00:01:47] Step 2/10 : RUN apt-get update && apt-get install -y --no-install-recommends   clang   make   file   curl   ca-certificates   python2.7   git   cmake   sudo   bzip2   xz-utils   wget   libssl-dev   pkg-config
[00:01:47] Step 2/10 : RUN apt-get update && apt-get install -y --no-install-recommends   clang   make   file   curl   ca-certificates   python2.7   git   cmake   sudo   bzip2   xz-utils   wget   libssl-dev   pkg-config
[00:01:47]  ---> Running in 9727439223ca
[00:01:48] standard_init_linux.go:190: exec user process caused "exec format error"
[00:01:48] The command '/bin/sh -c apt-get update && apt-get install -y --no-install-recommends   clang   make   file   curl   ca-certificates   python2.7   git   cmake   sudo   bzip2   xz-utils   wget   libssl-dev   pkg-config' returned a non-zero code: 1
travis_time:end:0122438c:start=1556366839822472916,finish=1556366948350177214,duration=108527704298
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
travis_time:start:263e0803
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:015b43b0:start=1556366949137144103,finish=1556366949144083876,duration=6939773
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2784e6dc
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:022872df
travis_time:start:022872df
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0bd16470
$ dmesg | grep -i kill
