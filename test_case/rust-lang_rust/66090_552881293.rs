plain
2019-11-12T11:41:39.4075373Z Help         : https://docs.microsoft.com/azure/devops/pipelines/tasks/utility/bash
2019-11-12T11:41:39.4075464Z ==============================================================================
2019-11-12T11:41:40.0408652Z Generating script.
2019-11-12T11:41:40.0409346Z Script contents:
2019-11-12T11:41:40.0410032Z src/ci/scripts/run-build-from-ci.sh
2019-11-12T11:41:40.0414170Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/efa71543-cf03-418a-ac0d-fe8cbce76875.sh
2019-11-12T11:41:40.0414530Z info: removing rustup home
2019-11-12T11:41:40.0414649Z info: removing cargo home
2019-11-12T11:41:40.3868668Z info: removing rustup binaries
---
2019-11-12T11:45:18.2269218Z Selecting previously unselected package rpm2cpio.
2019-11-12T11:45:18.2288163Z Preparing to unpack .../rpm2cpio_4.12.0.1+dfsg1-3build3_amd64.deb ...
2019-11-12T11:45:18.2465805Z Unpacking rpm2cpio (4.12.0.1+dfsg1-3build3) ...
2019-11-12T11:45:18.3071887Z Processing triggers for libc-bin (2.23-0ubuntu11) ...
2019-11-12T11:45:18.4777881Z Setting up libpopt0:amd64 (1.16-10) ...
2019-11-12T11:45:18.5743987Z Setting up cpio (2.11+dfsg-5ubuntu1.1) ...
2019-11-12T11:45:18.6154998Z update-alternatives: using /bin/mt-gnu to provide /bin/mt (mt) in auto mode
2019-11-12T11:45:18.6905822Z Setting up libelf1:amd64 (0.165-3ubuntu1.2) ...
2019-11-12T11:45:18.7361502Z Setting up liblua5.2-0:amd64 (5.2.4-1ubuntu1) ...
2019-11-12T11:45:18.7788526Z Setting up libnspr4:amd64 (2:4.13.1-0ubuntu0.16.04.1) ...
2019-11-12T11:45:18.8416017Z Setting up libnss3-nssdb (2:3.28.4-0ubuntu0.16.04.6) ...
---
2019-11-12T11:47:31.5431054Z + set +x
2019-11-12T11:47:43.9470108Z + mkdir ../gcc-build
2019-11-12T11:47:43.9488501Z ./build-powerpc64le-toolchain.sh: line 49: 37373 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
2019-11-12T11:47:43.9492691Z + cd ../gcc-build
2019-11-12T11:47:43.9499029Z + hide_output ../gcc-5.3.0/configure --enable-languages=c,c++ --target=powerpc64le-linux-gnu --with-cpu=power8 --with-sysroot=/usr/local/powerpc64le-linux-gnu/sysroot --disable-libcilkrts --disable-multilib --disable-nls --disable-libgomp --disable-libquadmath --disable-libssp --disable-libvtv --disable-libcilkrt --disable-libada --disable-libsanitizer --disable-libquadmath-support --disable-lto
2019-11-12T11:47:46.2156571Z + hide_output hide_output make -j10
2019-11-12T11:47:46.2158044Z + set +x
2019-11-12T11:48:16.2231099Z Tue Nov 12 11:48:16 UTC 2019 - building ...
2019-11-12T11:48:46.2394841Z Tue Nov 12 11:48:46 UTC 2019 - building ...
---
2019-11-12T11:57:16.8537209Z Tue Nov 12 11:57:16 UTC 2019 - building ...
2019-11-12T11:57:46.8966122Z Tue Nov 12 11:57:46 UTC 2019 - building ...
2019-11-12T11:58:16.9433131Z Tue Nov 12 11:58:16 UTC 2019 - building ...
2019-11-12T11:58:46.9828815Z Tue Nov 12 11:58:46 UTC 2019 - building ...
2019-11-12T11:58:50.5496512Z + trap - ERR
2019-11-12T11:58:50.5497378Z + kill 38900
2019-11-12T11:58:50.5498055Z shared.sh: line 13: kill: (38900) - No such process
2019-11-12T11:58:52.4330172Z The command '/bin/sh -c ./build-powerpc64le-toolchain.sh' returned a non-zero code: 1
2019-11-12T11:58:53.5570731Z Sending build context to Docker daemon  527.9kB
2019-11-12T11:58:53.5570817Z 
2019-11-12T11:58:53.5828366Z Step 1/21 : FROM ubuntu:16.04
2019-11-12T11:58:53.5838522Z  ---> 5f2bf26e3524
---
2019-11-12T12:01:06.5478691Z + set +x
2019-11-12T12:01:15.4927703Z + mkdir ../gcc-build
2019-11-12T12:01:15.4940649Z ./build-powerpc64le-toolchain.sh: line 49: 37378 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
2019-11-12T12:01:15.4942465Z + cd ../gcc-build
2019-11-12T12:01:15.4948795Z + hide_output ../gcc-5.3.0/configure --enable-languages=c,c++ --target=powerpc64le-linux-gnu --with-cpu=power8 --with-sysroot=/usr/local/powerpc64le-linux-gnu/sysroot --disable-libcilkrts --disable-multilib --disable-nls --disable-libgomp --disable-libquadmath --disable-libssp --disable-libvtv --disable-libcilkrt --disable-libada --disable-libsanitizer --disable-libquadmath-support --disable-lto
2019-11-12T12:01:17.7360894Z + hide_output hide_output make -j10
2019-11-12T12:01:17.7361275Z + set +x
2019-11-12T12:01:47.7541267Z Tue Nov 12 12:01:47 UTC 2019 - building ...
2019-11-12T12:02:17.7875351Z Tue Nov 12 12:02:17 UTC 2019 - building ...
---
2019-11-12T12:10:48.4229723Z Tue Nov 12 12:10:48 UTC 2019 - building ...
2019-11-12T12:11:18.4342853Z Tue Nov 12 12:11:18 UTC 2019 - building ...
2019-11-12T12:11:48.4637243Z Tue Nov 12 12:11:48 UTC 2019 - building ...
2019-11-12T12:12:18.5117955Z Tue Nov 12 12:12:18 UTC 2019 - building ...
2019-11-12T12:12:43.2326319Z + trap - ERR
2019-11-12T12:12:43.2328386Z + kill 38905
2019-11-12T12:12:43.2328804Z shared.sh: line 13: kill: (38905) - No such process
2019-11-12T12:12:44.9724430Z The command '/bin/sh -c ./build-powerpc64le-toolchain.sh' returned a non-zero code: 1
2019-11-12T12:12:47.0878729Z Sending build context to Docker daemon  527.9kB
2019-11-12T12:12:47.0879742Z 
2019-11-12T12:12:47.1148162Z Step 1/21 : FROM ubuntu:16.04
2019-11-12T12:12:47.1150807Z  ---> 5f2bf26e3524
---
2019-11-12T12:14:56.5460494Z + set +x
2019-11-12T12:15:05.4290253Z + mkdir ../gcc-build
2019-11-12T12:15:05.4305491Z ./build-powerpc64le-toolchain.sh: line 49: 37376 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
2019-11-12T12:15:05.4305881Z + cd ../gcc-build
2019-11-12T12:15:05.4311363Z + hide_output ../gcc-5.3.0/configure --enable-languages=c,c++ --target=powerpc64le-linux-gnu --with-cpu=power8 --with-sysroot=/usr/local/powerpc64le-linux-gnu/sysroot --disable-libcilkrts --disable-multilib --disable-nls --disable-libgomp --disable-libquadmath --disable-libssp --disable-libvtv --disable-libcilkrt --disable-libada --disable-libsanitizer --disable-libquadmath-support --disable-lto
2019-11-12T12:15:07.6943319Z + hide_output hide_output make -j10
2019-11-12T12:15:07.6943473Z + set +x
2019-11-12T12:15:37.7073347Z Tue Nov 12 12:15:37 UTC 2019 - building ...
2019-11-12T12:16:07.7295700Z Tue Nov 12 12:16:07 UTC 2019 - building ...
---
2019-11-12T12:24:38.3396475Z Tue Nov 12 12:24:38 UTC 2019 - building ...
2019-11-12T12:25:08.3583007Z Tue Nov 12 12:25:08 UTC 2019 - building ...
2019-11-12T12:25:38.4001111Z Tue Nov 12 12:25:38 UTC 2019 - building ...
2019-11-12T12:26:08.4248980Z Tue Nov 12 12:26:08 UTC 2019 - building ...
2019-11-12T12:26:25.8702562Z + trap - ERR
2019-11-12T12:26:25.8703476Z + kill 38903
2019-11-12T12:26:25.8704301Z shared.sh: line 13: kill: (38903) - No such process
2019-11-12T12:26:27.7060830Z The command '/bin/sh -c ./build-powerpc64le-toolchain.sh' returned a non-zero code: 1
2019-11-12T12:26:30.8191761Z Sending build context to Docker daemon  527.9kB
2019-11-12T12:26:30.8192569Z 
2019-11-12T12:26:30.8429799Z Step 1/21 : FROM ubuntu:16.04
2019-11-12T12:26:30.8435949Z  ---> 5f2bf26e3524
---
2019-11-12T12:28:41.7329838Z + set +x
2019-11-12T12:28:50.6907320Z + mkdir ../gcc-build
2019-11-12T12:28:50.6919852Z ./build-powerpc64le-toolchain.sh: line 49: 37376 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
2019-11-12T12:28:50.6923721Z + cd ../gcc-build
2019-11-12T12:28:50.6924813Z + hide_output ../gcc-5.3.0/configure --enable-languages=c,c++ --target=powerpc64le-linux-gnu --with-cpu=power8 --with-sysroot=/usr/local/powerpc64le-linux-gnu/sysroot --disable-libcilkrts --disable-multilib --disable-nls --disable-libgomp --disable-libquadmath --disable-libssp --disable-libvtv --disable-libcilkrt --disable-libada --disable-libsanitizer --disable-libquadmath-support --disable-lto
2019-11-12T12:28:52.9188137Z + hide_output hide_output make -j10
2019-11-12T12:28:52.9188477Z + set +x
2019-11-12T12:29:22.9442595Z Tue Nov 12 12:29:22 UTC 2019 - building ...
2019-11-12T12:29:52.9643715Z Tue Nov 12 12:29:52 UTC 2019 - building ...
---
2019-11-12T12:38:23.6398311Z Tue Nov 12 12:38:23 UTC 2019 - building ...
2019-11-12T12:38:53.6605224Z Tue Nov 12 12:38:53 UTC 2019 - building ...
2019-11-12T12:39:23.6832929Z Tue Nov 12 12:39:23 UTC 2019 - building ...
2019-11-12T12:39:53.6965107Z Tue Nov 12 12:39:53 UTC 2019 - building ...
2019-11-12T12:40:06.9957489Z + trap - ERR
2019-11-12T12:40:06.9961312Z + kill 38903
2019-11-12T12:40:06.9962117Z shared.sh: line 13: kill: (38903) - No such process
2019-11-12T12:40:08.6841131Z The command '/bin/sh -c ./build-powerpc64le-toolchain.sh' returned a non-zero code: 1
2019-11-12T12:40:12.7759363Z Sending build context to Docker daemon  527.9kB
2019-11-12T12:40:12.7764602Z 
2019-11-12T12:40:12.7949990Z Step 1/21 : FROM ubuntu:16.04
2019-11-12T12:40:12.7953182Z  ---> 5f2bf26e3524
---
2019-11-12T12:42:25.0961076Z + set +x
2019-11-12T12:42:34.3015059Z + mkdir ../gcc-build
2019-11-12T12:42:34.3031645Z ./build-powerpc64le-toolchain.sh: line 49: 37376 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
2019-11-12T12:42:34.3033425Z + cd ../gcc-build
2019-11-12T12:42:34.3038681Z + hide_output ../gcc-5.3.0/configure --enable-languages=c,c++ --target=powerpc64le-linux-gnu --with-cpu=power8 --with-sysroot=/usr/local/powerpc64le-linux-gnu/sysroot --disable-libcilkrts --disable-multilib --disable-nls --disable-libgomp --disable-libquadmath --disable-libssp --disable-libvtv --disable-libcilkrt --disable-libada --disable-libsanitizer --disable-libquadmath-support --disable-lto
2019-11-12T12:42:36.5835687Z + hide_output hide_output make -j10
2019-11-12T12:42:36.5836484Z + set +x
2019-11-12T12:43:06.5965440Z Tue Nov 12 12:43:06 UTC 2019 - building ...
2019-11-12T12:43:36.6237443Z Tue Nov 12 12:43:36 UTC 2019 - building ...
---
2019-11-12T12:52:07.2753825Z Tue Nov 12 12:52:07 UTC 2019 - building ...
2019-11-12T12:52:37.2903151Z Tue Nov 12 12:52:37 UTC 2019 - building ...
2019-11-12T12:53:07.3160369Z Tue Nov 12 12:53:07 UTC 2019 - building ...
2019-11-12T12:53:37.3370554Z Tue Nov 12 12:53:37 UTC 2019 - building ...
2019-11-12T12:53:50.4370894Z + trap - ERR
2019-11-12T12:53:50.4372714Z + kill 38903
2019-11-12T12:53:50.4377412Z shared.sh: line 13: kill: (38903) - No such process
2019-11-12T12:53:52.1324483Z The command '/bin/sh -c ./build-powerpc64le-toolchain.sh' returned a non-zero code: 1
2019-11-12T12:53:52.1382282Z 
2019-11-12T12:53:52.1382282Z 
2019-11-12T12:53:52.1486362Z ##[error]Bash exited with code '1'.
2019-11-12T12:53:52.1577426Z ##[section]Starting: Checkout
2019-11-12T12:53:52.1580216Z ==============================================================================
2019-11-12T12:53:52.1580362Z Task         : Get sources
2019-11-12T12:53:52.1580457Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
