plain
2019-07-16T21:10:15.7354554Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-16T21:10:15.7552269Z ##[command]git config gc.auto 0
2019-07-16T21:10:15.7617922Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-16T21:10:16.5528944Z ##[command]git config --get-all http.proxy
2019-07-16T21:10:16.5533631Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62733/merge:refs/remotes/pull/62733/merge
---
2019-07-16T21:10:50.4327113Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-16T21:10:50.4327159Z 
2019-07-16T21:10:50.4327541Z   git checkout -b <new-branch-name>
2019-07-16T21:10:50.4328058Z 
2019-07-16T21:10:50.4328116Z HEAD is now at 1def4da7a Merge 2ba88983ff2defdb131642809d0991203e1a731d into 96234d5363286700794973c36178c3df1d9d49d6
2019-07-16T21:10:50.4466922Z ##[section]Finishing: Checkout
2019-07-16T21:10:50.4473885Z ##[section]Starting: Decide whether to run this job
2019-07-16T21:10:50.4477102Z Task         : Bash
2019-07-16T21:10:50.4477162Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-16T21:10:50.4477202Z Version      : 3.151.2
2019-07-16T21:10:50.4477243Z Author       : Microsoft Corporation
2019-07-16T21:10:50.4477243Z Author       : Microsoft Corporation
2019-07-16T21:10:50.4477306Z Help         : https://docs.microsoft.com/azure/devops/pipelines/tasks/utility/bash
2019-07-16T21:10:50.4477531Z ==============================================================================
2019-07-16T21:10:50.5901394Z Generating script.
2019-07-16T21:10:50.5930981Z ========================== Starting Command Output ===========================
2019-07-16T21:10:50.5950351Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/2ec3bbb2-4ec5-4808-a3be-f729b71cfa01.sh
2019-07-16T21:10:50.7967385Z Executing the job since submodules are updated
2019-07-16T21:10:50.7990471Z ##[section]Finishing: Decide whether to run this job
2019-07-16T21:10:50.7999671Z ==============================================================================
2019-07-16T21:10:50.7999730Z Task         : Bash
2019-07-16T21:10:50.7999781Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-16T21:10:50.7999875Z Version      : 3.151.2
---
2019-07-16T21:12:42.5228827Z Attempting with retry: curl -y 30 -Y 10 --connect-timeout 30 -f -L -C - -o /tmp/rustci_docker_cache https://.s3.amazonaws.com/docker/5237541d5b559bcea2f168e4ed3efe600aec9344996d80ee3a8ea6b1b5d4d12b46df9cf61ac5cec12b0346064f67d51e8664869834273098748b7174157de4e3
2019-07-16T21:12:42.5288363Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-16T21:12:42.5297816Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-16T21:12:42.5298202Z 
2019-07-16T21:12:42.5330674Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-16T21:12:43.5391979Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-16T21:12:43.5392329Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-16T21:12:43.5392425Z 
2019-07-16T21:12:43.5392425Z 
2019-07-16T21:12:43.5435557Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-16T21:12:45.5522450Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-16T21:12:45.5522771Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-16T21:12:45.5522996Z 
2019-07-16T21:12:45.5522996Z 
2019-07-16T21:12:45.5524028Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-16T21:12:48.5590487Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-16T21:12:48.5590679Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-16T21:12:48.5590722Z 
2019-07-16T21:12:48.5590722Z 
2019-07-16T21:12:48.5633687Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-16T21:12:53.0929936Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-16T21:12:53.0930576Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-16T21:12:53.0930757Z 
2019-07-16T21:12:53.0930757Z 
2019-07-16T21:12:53.0931755Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-16T21:12:53.0932071Z The command has failed after 5 attempts.
2019-07-16T21:12:53.0932255Z open /tmp/rustci_docker_cache: no such file or directory
2019-07-16T21:12:53.0933296Z Attempting with retry: docker build --rm -t rust-ci -f /home/vsts/work/1/s/src/ci/docker/x86_64-gnu-tools/Dockerfile /home/vsts/work/1/s/src/ci/docker
2019-07-16T21:12:53.0933562Z Sending build context to Docker daemon  521.2kB
2019-07-16T21:12:53.0933773Z 
2019-07-16T21:12:53.0933937Z Step 1/9 : FROM ubuntu:16.04
---
2019-07-16T21:13:08.7262590Z Reading package lists...
2019-07-16T21:13:09.6167442Z Reading package lists...
2019-07-16T21:13:09.7757098Z Building dependency tree...
2019-07-16T21:13:09.7758575Z Reading state information...
2019-07-16T21:13:09.8802055Z The following additional packages will be installed:
2019-07-16T21:13:09.8805038Z   binutils bzip2 cmake-data cpp cpp-5 dpkg-dev g++-5 gcc gcc-5 git-man
2019-07-16T21:13:09.8806358Z   libarchive13 libasan2 libasn1-8-heimdal libatomic1 libbz2-1.0 libc-dev-bin
2019-07-16T21:13:09.8809700Z   liberror-perl libexpat1 libffi6 libgcc-5-dev libgdbm3 libglib2.0-0 libgmp10
2019-07-16T21:13:09.8810307Z   libgnutls30 libgomp1 libgssapi-krb5-2 libgssapi3-heimdal libhcrypto4-heimdal
2019-07-16T21:13:09.8810930Z   libheimbase1-heimdal libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal
2019-07-16T21:13:09.8811258Z   libicu55 libidn11 libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1
---
2019-07-16T21:13:09.8813483Z   libssl1.0.0 libstdc++-5-dev libtasn1-6 libtsan0 libubsan0 libwind0-heimdal
2019-07-16T21:13:09.8813742Z   libxml2 linux-libc-dev mime-support openssl patch perl perl-modules-5.22
2019-07-16T21:13:09.8814548Z   python2.7-minimal zlib1g-dev
2019-07-16T21:13:09.8846906Z Suggested packages:
2019-07-16T21:13:09.8848215Z   binutils-doc bzip2-doc codeblocks eclipse ninja-build cpp-doc gcc-5-locales
2019-07-16T21:13:09.8848559Z   debian-keyring g++-multilib g++-5-multilib gcc-5-doc libstdc++6-5-dbg
2019-07-16T21:13:09.8848821Z   gcc-multilib manpages-dev autoconf automake libtool flex bison gdb gcc-doc
2019-07-16T21:13:09.8849413Z   libasan2-dbg liblsan0-dbg libtsan0-dbg libubsan0-dbg libcilkrts5-dbg
2019-07-16T21:13:09.8849660Z   libmpx0-dbg libquadmath0-dbg gettext-base git-daemon-run
2019-07-16T21:13:09.8849660Z   libmpx0-dbg libquadmath0-dbg gettext-base git-daemon-run
2019-07-16T21:13:09.8850016Z   | git-daemon-sysvinit git-doc git-el git-email git-gui gitk gitweb git-arch
2019-07-16T21:13:09.8850282Z   git-cvs git-mediawiki git-svn lrzip glibc-doc gnutls-bin krb5-doc krb5-user
2019-07-16T21:13:09.8850545Z   libstdc++-5-doc make-doc ed diffutils-doc perl-doc libterm-readline-gnu-perl
2019-07-16T21:13:09.8850831Z   | libterm-readline-perl-perl python2.7-doc binfmt-support
2019-07-16T21:13:09.8850886Z Recommended packages:
2019-07-16T21:13:09.8851140Z   build-essential fakeroot libalgorithm-merge-perl less rsync ssh-client
2019-07-16T21:13:09.8851389Z   manpages manpages-dev libfile-fcntllock-perl libglib2.0-data
2019-07-16T21:13:09.8851983Z   shared-mime-info xdg-user-dirs krb5-locales libsasl2-modules libssl-doc
2019-07-16T21:13:09.8852170Z   xml-core netbase rename
2019-07-16T21:13:09.8852213Z The following NEW packages will be installed:
2019-07-16T21:13:09.8852478Z   binutils bzip2 ca-certificates cmake cmake-data cpp cpp-5 curl dpkg-dev file
2019-07-16T21:13:09.8852884Z   g++ g++-5 gcc gcc-5 git git-man libarchive13 libasan2 libasn1-8-heimdal
2019-07-16T21:13:09.8853545Z   libcurl3-gnutls libdpkg-perl liberror-perl libexpat1 libffi6 libgcc-5-dev
2019-07-16T21:13:09.8853763Z   libgdbm3 libglib2.0-0 libgmp10 libgnutls30 libgomp1 libgssapi-krb5-2
2019-07-16T21:13:09.8854129Z   libgssapi3-heimdal libhcrypto4-heimdal libheimbase1-heimdal
2019-07-16T21:13:09.8854380Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-16T21:13:09.8854380Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-16T21:13:09.8854775Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-16T21:13:09.8855173Z   libkrb5-3 libkrb5support0 libldap-2.4-2 liblsan0 liblzo2-2 libmagic1 libmpc3
2019-07-16T21:13:09.8855672Z   libmpfr4 libmpx0 libnettle6 libp11-kit0 libperl5.22 libpython2.7-minimal
2019-07-16T21:13:09.8856053Z   libpython2.7-stdlib libquadmath0 libroken18-heimdal librtmp1 libsasl2-2
2019-07-16T21:13:09.8856282Z   libsasl2-modules-db libsqlite3-0 libssl-dev libssl1.0.0 libstdc++-5-dev
2019-07-16T21:13:09.8856545Z   libtasn1-6 libtsan0 libubsan0 libwind0-heimdal libxml2 linux-libc-dev make
2019-07-16T21:13:09.8856763Z   mime-support openssl patch perl perl-modules-5.22 pkg-config python2.7
2019-07-16T21:13:09.8857134Z   python2.7-minimal sudo xz-utils zlib1g-dev
2019-07-16T21:13:09.8857594Z The following packages will be upgraded:
2019-07-16T21:13:10.2379180Z 1 upgraded, 92 newly installed, 0 to remove and 5 not upgraded.
2019-07-16T21:13:10.2379848Z Need to get 71.6 MB of archives.
2019-07-16T21:13:10.2380115Z After this operation, 310 MB of additional disk space will be used.
2019-07-16T21:13:10.2381415Z Get:1 http://archive.ubuntu.com/ubuntu xenial/main amd64 libgdbm3 amd64 1.8.3-13.1 [16.9 kB]
---
2019-07-16T21:14:08.0878938Z Removing intermediate container 9696b9cb188c
2019-07-16T21:14:08.0879696Z  ---> 40ad1ab5cdac
2019-07-16T21:14:08.0917480Z Successfully built 40ad1ab5cdac
2019-07-16T21:14:09.1110790Z Successfully tagged rust-ci:latest
2019-07-16T21:14:09.1111384Z Built container sha256:40ad1ab5cdac068e6db16b00d1a3800b28516e0c79d69236574e197a94c85832
2019-07-16T21:14:09.1111549Z Uploading finished image to https://.s3.amazonaws.com/docker/5237541d5b559bcea2f168e4ed3efe600aec9344996d80ee3a8ea6b1b5d4d12b46df9cf61ac5cec12b0346064f67d51e8664869834273098748b7174157de4e3
2019-07-16T21:14:48.1295837Z upload failed: - to s3:///docker/5237541d5b559bcea2f168e4ed3efe600aec9344996d80ee3a8ea6b1b5d4d12b46df9cf61ac5cec12b0346064f67d51e8664869834273098748b7174157de4e3 Parameter validation failed:
2019-07-16T21:14:48.1296508Z Invalid bucket name "": Bucket name must match the regex "^[a-zA-Z0-9.\-_]{1,255}$"
2019-07-16T21:14:48.1715830Z [CI_JOB_NAME=LinuxTools]
2019-07-16T21:14:48.3222169Z [CI_JOB_NAME=LinuxTools]
2019-07-16T21:14:48.3743152Z configure: processing command line
2019-07-16T21:14:48.3743703Z configure: 
2019-07-16T21:14:48.3744478Z configure: rust.dist-src        := False
2019-07-16T21:14:48.3744848Z configure: rust.save-toolstates := /tmp/toolstates.json
---
2019-07-17T00:34:20.6728405Z -args
2019-07-17T00:34:20.6728913Z -
2019-07-17T00:34:20.6737967Z 
2019-07-17T00:34:20.6738313Z The actual stdout differed from the expected stdout.
2019-07-17T00:34:20.6742600Z Actual stdout saved to /tmp/compiletest9Cm2qq/args.stdout
2019-07-17T00:34:20.6748421Z error[E0080]: Miri evaluation error: attempted to do invalid arithmetic on pointers that would leak base addresses, e.g., comparing pointers into different allocations
2019-07-17T00:34:20.6749171Z     --> /checkout/src/libcore/intrinsics.rs:1340:19
2019-07-17T00:34:20.6749527Z      |
2019-07-17T00:34:20.6749803Z 1340 |     let diff = if src_usize > dst_usize {
---
2019-07-17T00:34:20.6776460Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:20.6776702Z +
2019-07-17T00:34:20.6776823Z 
2019-07-17T00:34:20.6776984Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:20.6777573Z Actual stderr saved to /tmp/compiletest9Cm2qq/args.stderr
2019-07-17T00:34:20.6777731Z To update references, run this command from build directory:
2019-07-17T00:34:20.6778202Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'args.rs'
2019-07-17T00:34:20.6826306Z error: 2 errors occurred comparing output.
2019-07-17T00:34:20.6826525Z status: exit code: 1
2019-07-17T00:34:20.6826525Z status: exit code: 1
2019-07-17T00:34:20.6827639Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/args.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/args.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/args.stage-id.aux" "-A" "unused"
2019-07-17T00:34:20.6828342Z ------------------------------------------
2019-07-17T00:34:20.6828509Z 
2019-07-17T00:34:20.6828890Z ------------------------------------------
2019-07-17T00:34:20.6829385Z stderr:
---
2019-07-17T00:34:20.7239566Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:20.7239619Z +
2019-07-17T00:34:20.7239640Z 
2019-07-17T00:34:20.7239675Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:20.7239733Z Actual stderr saved to /tmp/compiletest9Cm2qq/arrays.stderr
2019-07-17T00:34:20.7239772Z To update references, run this command from build directory:
2019-07-17T00:34:20.7239977Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'arrays.rs'
2019-07-17T00:34:20.7240054Z error: 1 errors occurred comparing output.
2019-07-17T00:34:20.7240179Z status: exit code: 1
2019-07-17T00:34:20.7240179Z status: exit code: 1
2019-07-17T00:34:20.7240690Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/arrays.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/arrays.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/arrays.stage-id.aux" "-A" "unused"
2019-07-17T00:34:20.7240961Z ------------------------------------------
2019-07-17T00:34:20.7241006Z 
2019-07-17T00:34:20.7241181Z ------------------------------------------
2019-07-17T00:34:20.7241217Z stderr:
---
2019-07-17T00:34:20.8498381Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:20.8498430Z +
2019-07-17T00:34:20.8498458Z 
2019-07-17T00:34:20.8498522Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:20.8498576Z Actual stderr saved to /tmp/compiletest9Cm2qq/assume_bug.stderr
2019-07-17T00:34:20.8498627Z To update references, run this command from build directory:
2019-07-17T00:34:20.8498972Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'assume_bug.rs'
2019-07-17T00:34:20.8499066Z error: 1 errors occurred comparing output.
2019-07-17T00:34:20.8499129Z status: exit code: 1
2019-07-17T00:34:20.8499129Z status: exit code: 1
2019-07-17T00:34:20.8499877Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/assume_bug.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/assume_bug.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/assume_bug.stage-id.aux" "-A" "unused"
2019-07-17T00:34:20.8500248Z ------------------------------------------
2019-07-17T00:34:20.8500278Z 
2019-07-17T00:34:20.8500463Z ------------------------------------------
2019-07-17T00:34:20.8500521Z stderr:
---
2019-07-17T00:34:20.8508169Z ------------------------------------------
2019-07-17T00:34:20.8508202Z 
2019-07-17T00:34:20.8508230Z 
2019-07-17T00:34:20.8508276Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:20.8508547Z Actual stderr saved to /tmp/compiletest9Cm2qq/associated-const.stderr
2019-07-17T00:34:20.8508605Z To update references, run this command from build directory:
2019-07-17T00:34:20.8508871Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'associated-const.rs'
2019-07-17T00:34:20.8508971Z error: 1 errors occurred comparing output.
2019-07-17T00:34:20.8509016Z status: exit code: 1
2019-07-17T00:34:20.8509016Z status: exit code: 1
2019-07-17T00:34:20.8509831Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/associated-const.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/associated-const.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/associated-const.stage-id.aux" "-A" "unused"
2019-07-17T00:34:20.8510136Z ------------------------------------------
2019-07-17T00:34:20.8510165Z 
2019-07-17T00:34:20.8510349Z ------------------------------------------
2019-07-17T00:34:20.8510387Z stderr:
---
2019-07-17T00:34:20.9946987Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:20.9947035Z +
2019-07-17T00:34:20.9947063Z 
2019-07-17T00:34:20.9947237Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:20.9947526Z Actual stderr saved to /tmp/compiletest9Cm2qq/atomic-access-bool.stderr
2019-07-17T00:34:20.9947580Z To update references, run this command from build directory:
2019-07-17T00:34:20.9947857Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'atomic-access-bool.rs'
2019-07-17T00:34:20.9947961Z error: 1 errors occurred comparing output.
2019-07-17T00:34:20.9948006Z status: exit code: 1
2019-07-17T00:34:20.9948006Z status: exit code: 1
2019-07-17T00:34:20.9948685Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/atomic-access-bool.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/atomic-access-bool.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/atomic-access-bool.stage-id.aux" "-A" "unused"
2019-07-17T00:34:20.9949020Z ------------------------------------------
2019-07-17T00:34:20.9949054Z 
2019-07-17T00:34:20.9949437Z ------------------------------------------
2019-07-17T00:34:20.9949544Z stderr:
---
2019-07-17T00:34:21.0046820Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:21.0046916Z +
2019-07-17T00:34:21.0047136Z 
2019-07-17T00:34:21.0048043Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:21.0053844Z Actual stderr saved to /tmp/compiletest9Cm2qq/async-fn.stderr
2019-07-17T00:34:21.0054814Z To update references, run this command from build directory:
2019-07-17T00:34:21.0060826Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'async-fn.rs'
2019-07-17T00:34:21.0060941Z error: 1 errors occurred comparing output.
2019-07-17T00:34:21.0060978Z status: exit code: 1
2019-07-17T00:34:21.0060978Z status: exit code: 1
2019-07-17T00:34:21.0061494Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/async-fn.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/async-fn.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/async-fn.stage-id.aux" "-A" "unused"
2019-07-17T00:34:21.0061759Z ------------------------------------------
2019-07-17T00:34:21.0061793Z 
2019-07-17T00:34:21.0061990Z ------------------------------------------
2019-07-17T00:34:21.0062029Z stderr:
---
2019-07-17T00:34:21.1282795Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:21.1282856Z +
2019-07-17T00:34:21.1282880Z 
2019-07-17T00:34:21.1283046Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:21.1283098Z Actual stderr saved to /tmp/compiletest9Cm2qq/bad_substs.stderr
2019-07-17T00:34:21.1283156Z To update references, run this command from build directory:
2019-07-17T00:34:21.1283428Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'bad_substs.rs'
2019-07-17T00:34:21.1283511Z error: 1 errors occurred comparing output.
2019-07-17T00:34:21.1283550Z status: exit code: 1
2019-07-17T00:34:21.1283550Z status: exit code: 1
2019-07-17T00:34:21.1284061Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/bad_substs.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/bad_substs.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/bad_substs.stage-id.aux" "-A" "unused"
2019-07-17T00:34:21.1284523Z ------------------------------------------
2019-07-17T00:34:21.1284554Z 
2019-07-17T00:34:21.1284933Z ------------------------------------------
2019-07-17T00:34:21.1284972Z stderr:
---
2019-07-17T00:34:21.1883723Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:21.1883903Z +
2019-07-17T00:34:21.1883936Z 
2019-07-17T00:34:21.1883974Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:21.1884219Z Actual stderr saved to /tmp/compiletest9Cm2qq/atomic-compare_exchange.stderr
2019-07-17T00:34:21.1884291Z To update references, run this command from build directory:
2019-07-17T00:34:21.1884515Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'atomic-compare_exchange.rs'
2019-07-17T00:34:21.1884596Z error: 1 errors occurred comparing output.
2019-07-17T00:34:21.1884633Z status: exit code: 1
2019-07-17T00:34:21.1884633Z status: exit code: 1
2019-07-17T00:34:21.1885192Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/atomic-compare_exchange.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/atomic-compare_exchange.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/atomic-compare_exchange.stage-id.aux" "-A" "unused"
2019-07-17T00:34:21.1886229Z ------------------------------------------
2019-07-17T00:34:21.1886389Z 
2019-07-17T00:34:21.1886684Z ------------------------------------------
2019-07-17T00:34:21.1886732Z stderr:
---
2019-07-17T00:34:21.3233482Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:21.3233640Z +
2019-07-17T00:34:21.3233753Z 
2019-07-17T00:34:21.3233880Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:21.3233995Z Actual stderr saved to /tmp/compiletest9Cm2qq/binops.stderr
2019-07-17T00:34:21.3234127Z To update references, run this command from build directory:
2019-07-17T00:34:21.3234465Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'binops.rs'
2019-07-17T00:34:21.3234730Z error: 1 errors occurred comparing output.
2019-07-17T00:34:21.3234863Z status: exit code: 1
2019-07-17T00:34:21.3234863Z status: exit code: 1
2019-07-17T00:34:21.3236196Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/binops.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/binops.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/binops.stage-id.aux" "-A" "unused"
2019-07-17T00:34:21.3237042Z ------------------------------------------
2019-07-17T00:34:21.3237232Z 
2019-07-17T00:34:21.3237583Z ------------------------------------------
2019-07-17T00:34:21.3237753Z stderr:
---
2019-07-17T00:34:21.3564946Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:21.3564996Z +
2019-07-17T00:34:21.3565038Z 
2019-07-17T00:34:21.3565076Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:21.3565116Z Actual stderr saved to /tmp/compiletest9Cm2qq/bools.stderr
2019-07-17T00:34:21.3565172Z To update references, run this command from build directory:
2019-07-17T00:34:21.3565581Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'bools.rs'
2019-07-17T00:34:21.3566323Z error: 1 errors occurred comparing output.
2019-07-17T00:34:21.3566390Z status: exit code: 1
2019-07-17T00:34:21.3566390Z status: exit code: 1
2019-07-17T00:34:21.3567156Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/bools.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/bools.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/bools.stage-id.aux" "-A" "unused"
2019-07-17T00:34:21.3567644Z ------------------------------------------
2019-07-17T00:34:21.3567679Z 
2019-07-17T00:34:21.3567899Z ------------------------------------------
2019-07-17T00:34:21.3567966Z stderr:
---
2019-07-17T00:34:21.4683677Z -foo #1 = Foo(1337)
2019-07-17T00:34:21.4683825Z -
2019-07-17T00:34:21.4683849Z 
2019-07-17T00:34:21.4683885Z The actual stdout differed from the expected stdout.
2019-07-17T00:34:21.4684243Z Actual stdout saved to /tmp/compiletest9Cm2qq/box-pair-to-vec.stdout
2019-07-17T00:34:21.4684340Z error[E0080]: Miri evaluation error: attempted to do invalid arithmetic on pointers that would leak base addresses, e.g., comparing pointers into different allocations
2019-07-17T00:34:21.4684595Z     --> /checkout/src/libcore/intrinsics.rs:1340:19
2019-07-17T00:34:21.4684635Z      |
2019-07-17T00:34:21.4684675Z 1340 |     let diff = if src_usize > dst_usize {
---
2019-07-17T00:34:21.4688840Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:21.4688909Z +
2019-07-17T00:34:21.4688938Z 
2019-07-17T00:34:21.4688986Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:21.4689520Z Actual stderr saved to /tmp/compiletest9Cm2qq/box-pair-to-vec.stderr
2019-07-17T00:34:21.4689585Z To update references, run this command from build directory:
2019-07-17T00:34:21.4689959Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'box-pair-to-vec.rs'
2019-07-17T00:34:21.4690037Z error: 2 errors occurred comparing output.
2019-07-17T00:34:21.4690072Z status: exit code: 1
2019-07-17T00:34:21.4690072Z status: exit code: 1
2019-07-17T00:34:21.4690586Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/box-pair-to-vec.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/box-pair-to-vec.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/box-pair-to-vec.stage-id.aux" "-A" "unused"
2019-07-17T00:34:21.4690849Z ------------------------------------------
2019-07-17T00:34:21.4690875Z 
2019-07-17T00:34:21.4691064Z ------------------------------------------
2019-07-17T00:34:21.4691100Z stderr:
---
2019-07-17T00:34:21.4762898Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:21.4762951Z +
2019-07-17T00:34:21.4762972Z 
2019-07-17T00:34:21.4763007Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:21.4763049Z Actual stderr saved to /tmp/compiletest9Cm2qq/box_box_trait.stderr
2019-07-17T00:34:21.4763107Z To update references, run this command from build directory:
2019-07-17T00:34:21.4763320Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'box_box_trait.rs'
2019-07-17T00:34:21.4763383Z error: 1 errors occurred comparing output.
2019-07-17T00:34:21.4763435Z status: exit code: 1
2019-07-17T00:34:21.4763435Z status: exit code: 1
2019-07-17T00:34:21.4763956Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/box_box_trait.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/box_box_trait.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/box_box_trait.stage-id.aux" "-A" "unused"
2019-07-17T00:34:21.4764226Z ------------------------------------------
2019-07-17T00:34:21.4764252Z 
2019-07-17T00:34:21.4764443Z ------------------------------------------
2019-07-17T00:34:21.4764479Z stderr:
---
2019-07-17T00:34:21.6277915Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:21.6277983Z +
2019-07-17T00:34:21.6283445Z 
2019-07-17T00:34:21.6283531Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:21.6283592Z Actual stderr saved to /tmp/compiletest9Cm2qq/c_enums.stderr
2019-07-17T00:34:21.6283631Z To update references, run this command from build directory:
2019-07-17T00:34:21.6283978Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'c_enums.rs'
2019-07-17T00:34:21.6284061Z error: 1 errors occurred comparing output.
2019-07-17T00:34:21.6284096Z status: exit code: 1
2019-07-17T00:34:21.6284096Z status: exit code: 1
2019-07-17T00:34:21.6290477Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/c_enums.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/c_enums.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/c_enums.stage-id.aux" "-A" "unused"
2019-07-17T00:34:21.6290781Z ------------------------------------------
2019-07-17T00:34:21.6294484Z 
2019-07-17T00:34:21.6294824Z ------------------------------------------
2019-07-17T00:34:21.6294865Z stderr:
---
2019-07-17T00:34:21.6438565Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:21.6438616Z +
2019-07-17T00:34:21.6438643Z 
2019-07-17T00:34:21.6438708Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:21.6438761Z Actual stderr saved to /tmp/compiletest9Cm2qq/btreemap.stderr
2019-07-17T00:34:21.6438814Z To update references, run this command from build directory:
2019-07-17T00:34:21.6439121Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'btreemap.rs'
2019-07-17T00:34:21.6439204Z error: 1 errors occurred comparing output.
2019-07-17T00:34:21.6439251Z status: exit code: 1
2019-07-17T00:34:21.6439251Z status: exit code: 1
2019-07-17T00:34:21.6439961Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/btreemap.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/btreemap.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/btreemap.stage-id.aux" "-A" "unused"
2019-07-17T00:34:21.6440220Z ------------------------------------------
2019-07-17T00:34:21.6440246Z 
2019-07-17T00:34:21.6440420Z ------------------------------------------
2019-07-17T00:34:21.6440455Z stderr:
---
2019-07-17T00:34:21.7771679Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:21.7771717Z +
2019-07-17T00:34:21.7771737Z 
2019-07-17T00:34:21.7771771Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:21.7771830Z Actual stderr saved to /tmp/compiletest9Cm2qq/call_drop_on_fat_ptr_array_elements.stderr
2019-07-17T00:34:21.7771870Z To update references, run this command from build directory:
2019-07-17T00:34:21.7772097Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'call_drop_on_fat_ptr_array_elements.rs'
2019-07-17T00:34:21.7772184Z error: 1 errors occurred comparing output.
2019-07-17T00:34:21.7772219Z status: exit code: 1
2019-07-17T00:34:21.7772219Z status: exit code: 1
2019-07-17T00:34:21.7772787Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/call_drop_on_fat_ptr_array_elements.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/call_drop_on_fat_ptr_array_elements.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/call_drop_on_fat_ptr_array_elements.stage-id.aux" "-A" "unused"
2019-07-17T00:34:21.7773039Z ------------------------------------------
2019-07-17T00:34:21.7773084Z 
2019-07-17T00:34:21.7773256Z ------------------------------------------
2019-07-17T00:34:21.7773290Z stderr:
---
2019-07-17T00:34:21.7781559Z ------------------------------------------
2019-07-17T00:34:21.7781605Z 
2019-07-17T00:34:21.7781626Z 
2019-07-17T00:34:21.7781661Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:21.7781703Z Actual stderr saved to /tmp/compiletest9Cm2qq/call_drop_on_array_elements.stderr
2019-07-17T00:34:21.7781760Z To update references, run this command from build directory:
2019-07-17T00:34:21.7781977Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'call_drop_on_array_elements.rs'
2019-07-17T00:34:21.7782057Z error: 1 errors occurred comparing output.
2019-07-17T00:34:21.7782091Z status: exit code: 1
2019-07-17T00:34:21.7782091Z status: exit code: 1
2019-07-17T00:34:21.7782861Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/call_drop_on_array_elements.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/call_drop_on_array_elements.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/call_drop_on_array_elements.stage-id.aux" "-A" "unused"
2019-07-17T00:34:21.7783137Z ------------------------------------------
2019-07-17T00:34:21.7783178Z 
2019-07-17T00:34:21.7783351Z ------------------------------------------
2019-07-17T00:34:21.7783387Z stderr:
---
2019-07-17T00:34:21.9200693Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:21.9200732Z +
2019-07-17T00:34:21.9200752Z 
2019-07-17T00:34:21.9200787Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:21.9200851Z Actual stderr saved to /tmp/compiletest9Cm2qq/call_drop_through_owned_slice.stderr
2019-07-17T00:34:21.9200890Z To update references, run this command from build directory:
2019-07-17T00:34:21.9201110Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'call_drop_through_owned_slice.rs'
2019-07-17T00:34:21.9201190Z error: 1 errors occurred comparing output.
2019-07-17T00:34:21.9201226Z status: exit code: 1
2019-07-17T00:34:21.9201226Z status: exit code: 1
2019-07-17T00:34:21.9201782Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/call_drop_through_owned_slice.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/call_drop_through_owned_slice.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/call_drop_through_owned_slice.stage-id.aux" "-A" "unused"
2019-07-17T00:34:21.9202066Z ------------------------------------------
2019-07-17T00:34:21.9202094Z 
2019-07-17T00:34:21.9202279Z ------------------------------------------
2019-07-17T00:34:21.9202317Z stderr:
---
2019-07-17T00:34:21.9322242Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:21.9322310Z +
2019-07-17T00:34:21.9322331Z 
2019-07-17T00:34:21.9322368Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:21.9322427Z Actual stderr saved to /tmp/compiletest9Cm2qq/call_drop_on_zst_array_elements.stderr
2019-07-17T00:34:21.9322468Z To update references, run this command from build directory:
2019-07-17T00:34:21.9322692Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'call_drop_on_zst_array_elements.rs'
2019-07-17T00:34:21.9322775Z error: 1 errors occurred comparing output.
2019-07-17T00:34:21.9322810Z status: exit code: 1
2019-07-17T00:34:21.9322810Z status: exit code: 1
2019-07-17T00:34:21.9323351Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/call_drop_on_zst_array_elements.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/call_drop_on_zst_array_elements.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/call_drop_on_zst_array_elements.stage-id.aux" "-A" "unused"
2019-07-17T00:34:21.9323623Z ------------------------------------------
2019-07-17T00:34:21.9323666Z 
2019-07-17T00:34:21.9323842Z ------------------------------------------
2019-07-17T00:34:21.9323879Z stderr:
---
2019-07-17T00:34:22.0442456Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:22.0442504Z +
2019-07-17T00:34:22.0442528Z 
2019-07-17T00:34:22.0442585Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:22.0442628Z Actual stderr saved to /tmp/compiletest9Cm2qq/call_drop_through_trait_object.stderr
2019-07-17T00:34:22.0442672Z To update references, run this command from build directory:
2019-07-17T00:34:22.0442935Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'call_drop_through_trait_object.rs'
2019-07-17T00:34:22.0443003Z error: 1 errors occurred comparing output.
2019-07-17T00:34:22.0443040Z status: exit code: 1
2019-07-17T00:34:22.0443040Z status: exit code: 1
2019-07-17T00:34:22.0443647Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/call_drop_through_trait_object.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/call_drop_through_trait_object.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/call_drop_through_trait_object.stage-id.aux" "-A" "unused"
2019-07-17T00:34:22.0443937Z ------------------------------------------
2019-07-17T00:34:22.0443965Z 
2019-07-17T00:34:22.0444154Z ------------------------------------------
2019-07-17T00:34:22.0444212Z stderr:
---
2019-07-17T00:34:22.0601737Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:22.0601798Z +
2019-07-17T00:34:22.0601821Z 
2019-07-17T00:34:22.0601858Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:22.0601901Z Actual stderr saved to /tmp/compiletest9Cm2qq/call_drop_through_trait_object_rc.stderr
2019-07-17T00:34:22.0601960Z To update references, run this command from build directory:
2019-07-17T00:34:22.0602196Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'call_drop_through_trait_object_rc.rs'
2019-07-17T00:34:22.0602292Z error: 1 errors occurred comparing output.
2019-07-17T00:34:22.0602330Z status: exit code: 1
2019-07-17T00:34:22.0602330Z status: exit code: 1
2019-07-17T00:34:22.0602895Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/call_drop_through_trait_object_rc.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/call_drop_through_trait_object_rc.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/call_drop_through_trait_object_rc.stage-id.aux" "-A" "unused"
2019-07-17T00:34:22.0603176Z ------------------------------------------
2019-07-17T00:34:22.0603202Z 
2019-07-17T00:34:22.0603400Z ------------------------------------------
2019-07-17T00:34:22.0603436Z stderr:
---
2019-07-17T00:34:22.2041625Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:22.2041662Z +
2019-07-17T00:34:22.2041703Z 
2019-07-17T00:34:22.2041738Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:22.2041777Z Actual stderr saved to /tmp/compiletest9Cm2qq/calloc.stderr
2019-07-17T00:34:22.2041840Z To update references, run this command from build directory:
2019-07-17T00:34:22.2042040Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'calloc.rs'
2019-07-17T00:34:22.2042101Z error: 1 errors occurred comparing output.
2019-07-17T00:34:22.2042162Z status: exit code: 1
2019-07-17T00:34:22.2042162Z status: exit code: 1
2019-07-17T00:34:22.2042653Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/calloc.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/calloc.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/calloc.stage-id.aux" "-A" "unused"
2019-07-17T00:34:22.2042955Z ------------------------------------------
2019-07-17T00:34:22.2042984Z 
2019-07-17T00:34:22.2043157Z ------------------------------------------
2019-07-17T00:34:22.2043214Z stderr:
---
2019-07-17T00:34:22.2317464Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:22.2317517Z +
2019-07-17T00:34:22.2317568Z 
2019-07-17T00:34:22.2317617Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:22.2317669Z Actual stderr saved to /tmp/compiletest9Cm2qq/calls.stderr
2019-07-17T00:34:22.2317731Z To update references, run this command from build directory:
2019-07-17T00:34:22.2318051Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'calls.rs'
2019-07-17T00:34:22.2318145Z error: 1 errors occurred comparing output.
2019-07-17T00:34:22.2318210Z status: exit code: 1
2019-07-17T00:34:22.2318210Z status: exit code: 1
2019-07-17T00:34:22.2318858Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/calls.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/calls.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/calls.stage-id.aux" "-A" "unused"
2019-07-17T00:34:22.2319206Z ------------------------------------------
2019-07-17T00:34:22.2319243Z 
2019-07-17T00:34:22.2319892Z ------------------------------------------
2019-07-17T00:34:22.2319956Z stderr:
---
2019-07-17T00:34:22.3489621Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:22.3489661Z +
2019-07-17T00:34:22.3489684Z 
2019-07-17T00:34:22.3489737Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:22.3489786Z Actual stderr saved to /tmp/compiletest9Cm2qq/cast_fn_ptr.stderr
2019-07-17T00:34:22.3489827Z To update references, run this command from build directory:
2019-07-17T00:34:22.3490042Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'cast_fn_ptr.rs'
2019-07-17T00:34:22.3490132Z error: 1 errors occurred comparing output.
2019-07-17T00:34:22.3490168Z status: exit code: 1
2019-07-17T00:34:22.3490168Z status: exit code: 1
2019-07-17T00:34:22.3490701Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/cast_fn_ptr.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/cast_fn_ptr.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/cast_fn_ptr.stage-id.aux" "-A" "unused"
2019-07-17T00:34:22.3490959Z ------------------------------------------
2019-07-17T00:34:22.3491068Z 
2019-07-17T00:34:22.3491271Z ------------------------------------------
2019-07-17T00:34:22.3491308Z stderr:
---
2019-07-17T00:34:22.3515289Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:22.3515328Z +
2019-07-17T00:34:22.3515351Z 
2019-07-17T00:34:22.3515584Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:22.3516417Z Actual stderr saved to /tmp/compiletest9Cm2qq/cast-rfc0401-vtable-kinds.stderr
2019-07-17T00:34:22.3516480Z To update references, run this command from build directory:
2019-07-17T00:34:22.3516777Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'cast-rfc0401-vtable-kinds.rs'
2019-07-17T00:34:22.3516861Z error: 1 errors occurred comparing output.
2019-07-17T00:34:22.3516925Z status: exit code: 1
2019-07-17T00:34:22.3516925Z status: exit code: 1
2019-07-17T00:34:22.3517707Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/cast-rfc0401-vtable-kinds.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/cast-rfc0401-vtable-kinds.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/cast-rfc0401-vtable-kinds.stage-id.aux" "-A" "unused"
2019-07-17T00:34:22.3518074Z ------------------------------------------
2019-07-17T00:34:22.3518109Z 
2019-07-17T00:34:22.3518326Z ------------------------------------------
2019-07-17T00:34:22.3518389Z stderr:
---
2019-07-17T00:34:22.4759154Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:22.4759215Z +
2019-07-17T00:34:22.4759262Z 
2019-07-17T00:34:22.4759475Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:22.4759523Z Actual stderr saved to /tmp/compiletest9Cm2qq/cast_fn_ptr_unsafe.stderr
2019-07-17T00:34:22.4759587Z To update references, run this command from build directory:
2019-07-17T00:34:22.4759831Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'cast_fn_ptr_unsafe.rs'
2019-07-17T00:34:22.4759901Z error: 1 errors occurred comparing output.
2019-07-17T00:34:22.4759959Z status: exit code: 1
2019-07-17T00:34:22.4759959Z status: exit code: 1
2019-07-17T00:34:22.4760847Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/cast_fn_ptr_unsafe.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/cast_fn_ptr_unsafe.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/cast_fn_ptr_unsafe.stage-id.aux" "-A" "unused"
2019-07-17T00:34:22.4761182Z ------------------------------------------
2019-07-17T00:34:22.4761212Z 
2019-07-17T00:34:22.4761418Z ------------------------------------------
2019-07-17T00:34:22.4761459Z stderr:
---
2019-07-17T00:34:22.4829294Z -1
2019-07-17T00:34:22.4829626Z -
2019-07-17T00:34:22.4829719Z 
2019-07-17T00:34:22.4829760Z The actual stdout differed from the expected stdout.
2019-07-17T00:34:22.4829803Z Actual stdout saved to /tmp/compiletest9Cm2qq/catch.stdout
2019-07-17T00:34:22.4829931Z error[E0080]: Miri evaluation error: attempted to do invalid arithmetic on pointers that would leak base addresses, e.g., comparing pointers into different allocations
2019-07-17T00:34:22.4830153Z     --> /checkout/src/libcore/intrinsics.rs:1340:19
2019-07-17T00:34:22.4830214Z      |
2019-07-17T00:34:22.4830251Z 1340 |     let diff = if src_usize > dst_usize {
---
2019-07-17T00:34:22.4833352Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:22.4833391Z +
2019-07-17T00:34:22.4833435Z 
2019-07-17T00:34:22.4833474Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:22.4833515Z Actual stderr saved to /tmp/compiletest9Cm2qq/catch.stderr
2019-07-17T00:34:22.4833627Z To update references, run this command from build directory:
2019-07-17T00:34:22.4833891Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'catch.rs'
2019-07-17T00:34:22.4833965Z error: 2 errors occurred comparing output.
2019-07-17T00:34:22.4834019Z status: exit code: 1
2019-07-17T00:34:22.4834019Z status: exit code: 1
2019-07-17T00:34:22.4834533Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/catch.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/catch.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/catch.stage-id.aux" "-A" "unused"
2019-07-17T00:34:22.4834810Z ------------------------------------------
2019-07-17T00:34:22.4834839Z 
2019-07-17T00:34:22.4835035Z ------------------------------------------
2019-07-17T00:34:22.4835091Z stderr:
---
2019-07-17T00:34:22.6079838Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:22.6079957Z +
2019-07-17T00:34:22.6079986Z 
2019-07-17T00:34:22.6080042Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:22.6080274Z Actual stderr saved to /tmp/compiletest9Cm2qq/closure-drop.stderr
2019-07-17T00:34:22.6080330Z To update references, run this command from build directory:
2019-07-17T00:34:22.6080723Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'closure-drop.rs'
2019-07-17T00:34:22.6080792Z error: 1 errors occurred comparing output.
2019-07-17T00:34:22.6080831Z status: exit code: 1
2019-07-17T00:34:22.6080831Z status: exit code: 1
2019-07-17T00:34:22.6081400Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/closure-drop.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/closure-drop.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/closure-drop.stage-id.aux" "-A" "unused"
2019-07-17T00:34:22.6081684Z ------------------------------------------
2019-07-17T00:34:22.6081714Z 
2019-07-17T00:34:22.6082153Z ------------------------------------------
2019-07-17T00:34:22.6082192Z stderr:
---
2019-07-17T00:34:22.6109229Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:22.6109280Z +
2019-07-17T00:34:22.6109309Z 
2019-07-17T00:34:22.6109620Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:22.6109667Z Actual stderr saved to /tmp/compiletest9Cm2qq/char.stderr
2019-07-17T00:34:22.6109737Z To update references, run this command from build directory:
2019-07-17T00:34:22.6109996Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'char.rs'
2019-07-17T00:34:22.6110059Z error: 1 errors occurred comparing output.
2019-07-17T00:34:22.6110094Z status: exit code: 1
2019-07-17T00:34:22.6110094Z status: exit code: 1
2019-07-17T00:34:22.6110584Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/char.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/char.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/char.stage-id.aux" "-A" "unused"
2019-07-17T00:34:22.6110842Z ------------------------------------------
2019-07-17T00:34:22.6110869Z 
2019-07-17T00:34:22.6111039Z ------------------------------------------
2019-07-17T00:34:22.6111091Z stderr:
---
2019-07-17T00:34:22.7808501Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:22.7808554Z +
2019-07-17T00:34:22.7808583Z 
2019-07-17T00:34:22.7808780Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:22.7809113Z Actual stderr saved to /tmp/compiletest9Cm2qq/closure-field-ty.stderr
2019-07-17T00:34:22.7809170Z To update references, run this command from build directory:
2019-07-17T00:34:22.7809444Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'closure-field-ty.rs'
2019-07-17T00:34:22.7809704Z error: 1 errors occurred comparing output.
2019-07-17T00:34:22.7809740Z status: exit code: 1
2019-07-17T00:34:22.7809740Z status: exit code: 1
2019-07-17T00:34:22.7810271Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/closure-field-ty.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/closure-field-ty.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/closure-field-ty.stage-id.aux" "-A" "unused"
2019-07-17T00:34:22.7810531Z ------------------------------------------
2019-07-17T00:34:22.7810558Z 
2019-07-17T00:34:22.7810731Z ------------------------------------------
2019-07-17T00:34:22.7810849Z stderr:
---
2019-07-17T00:34:22.8241665Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:22.8241712Z +
2019-07-17T00:34:22.8241732Z 
2019-07-17T00:34:22.8241767Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:22.8241823Z Actual stderr saved to /tmp/compiletest9Cm2qq/closures.stderr
2019-07-17T00:34:22.8241870Z To update references, run this command from build directory:
2019-07-17T00:34:22.8242091Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'closures.rs'
2019-07-17T00:34:22.8242168Z error: 1 errors occurred comparing output.
2019-07-17T00:34:22.8242204Z status: exit code: 1
2019-07-17T00:34:22.8242204Z status: exit code: 1
2019-07-17T00:34:22.8242717Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/closures.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/closures.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/closures.stage-id.aux" "-A" "unused"
2019-07-17T00:34:22.8242960Z ------------------------------------------
2019-07-17T00:34:22.8243068Z 
2019-07-17T00:34:22.8243264Z ------------------------------------------
2019-07-17T00:34:22.8243300Z stderr:
---
2019-07-17T00:34:22.9087783Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:22.9090605Z +
2019-07-17T00:34:22.9090816Z 
2019-07-17T00:34:22.9090992Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:22.9091514Z Actual stderr saved to /tmp/compiletest9Cm2qq/const-vec-of-fns.stderr
2019-07-17T00:34:22.9091764Z To update references, run this command from build directory:
2019-07-17T00:34:22.9092192Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'const-vec-of-fns.rs'
2019-07-17T00:34:22.9092582Z error: 1 errors occurred comparing output.
2019-07-17T00:34:22.9092764Z status: exit code: 1
2019-07-17T00:34:22.9092764Z status: exit code: 1
2019-07-17T00:34:22.9093506Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/const-vec-of-fns.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/const-vec-of-fns.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/const-vec-of-fns.stage-id.aux" "-A" "unused"
2019-07-17T00:34:22.9096420Z ------------------------------------------
2019-07-17T00:34:22.9142970Z 
2019-07-17T00:34:22.9144104Z ------------------------------------------
2019-07-17T00:34:22.9145962Z stderr:
---
2019-07-17T00:34:22.9523444Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:22.9523607Z +
2019-07-17T00:34:22.9523704Z 
2019-07-17T00:34:22.9523837Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:22.9523954Z Actual stderr saved to /tmp/compiletest9Cm2qq/constants.stderr
2019-07-17T00:34:22.9524087Z To update references, run this command from build directory:
2019-07-17T00:34:22.9524440Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'constants.rs'
2019-07-17T00:34:22.9524778Z error: 1 errors occurred comparing output.
2019-07-17T00:34:22.9524921Z status: exit code: 1
2019-07-17T00:34:22.9524921Z status: exit code: 1
2019-07-17T00:34:22.9526242Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/constants.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/constants.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/constants.stage-id.aux" "-A" "unused"
2019-07-17T00:34:22.9527236Z ------------------------------------------
2019-07-17T00:34:22.9527398Z 
2019-07-17T00:34:22.9527772Z ------------------------------------------
2019-07-17T00:34:22.9527964Z stderr:
---
2019-07-17T00:34:23.0683886Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:23.0683925Z +
2019-07-17T00:34:23.0683947Z 
2019-07-17T00:34:23.0683999Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:23.0684041Z Actual stderr saved to /tmp/compiletest9Cm2qq/drop_empty_slice.stderr
2019-07-17T00:34:23.0684082Z To update references, run this command from build directory:
2019-07-17T00:34:23.0684305Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'drop_empty_slice.rs'
2019-07-17T00:34:23.0684386Z error: 1 errors occurred comparing output.
2019-07-17T00:34:23.0684423Z status: exit code: 1
2019-07-17T00:34:23.0684423Z status: exit code: 1
2019-07-17T00:34:23.0684971Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/drop_empty_slice.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/drop_empty_slice.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/drop_empty_slice.stage-id.aux" "-A" "unused"
2019-07-17T00:34:23.0685331Z ------------------------------------------
2019-07-17T00:34:23.0685360Z 
2019-07-17T00:34:23.0685727Z ------------------------------------------
2019-07-17T00:34:23.0686278Z stderr:
---
2019-07-17T00:34:23.2244214Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:23.2244254Z +
2019-07-17T00:34:23.2244295Z 
2019-07-17T00:34:23.2244334Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:23.2244566Z Actual stderr saved to /tmp/compiletest9Cm2qq/deriving-associated-types.stderr
2019-07-17T00:34:23.2244626Z To update references, run this command from build directory:
2019-07-17T00:34:23.2244864Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'deriving-associated-types.rs'
2019-07-17T00:34:23.2244933Z error: 1 errors occurred comparing output.
2019-07-17T00:34:23.2244992Z status: exit code: 1
2019-07-17T00:34:23.2244992Z status: exit code: 1
2019-07-17T00:34:23.2245736Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/deriving-associated-types.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/deriving-associated-types.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/deriving-associated-types.stage-id.aux" "-A" "unused"
2019-07-17T00:34:23.2246658Z ------------------------------------------
2019-07-17T00:34:23.2246695Z 
2019-07-17T00:34:23.2246939Z ------------------------------------------
2019-07-17T00:34:23.2246986Z stderr:
---
2019-07-17T00:34:23.2725054Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:23.2725092Z +
2019-07-17T00:34:23.2725115Z 
2019-07-17T00:34:23.2725168Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:23.2725384Z Actual stderr saved to /tmp/compiletest9Cm2qq/dst-field-align.stderr
2019-07-17T00:34:23.2725597Z To update references, run this command from build directory:
2019-07-17T00:34:23.2726478Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'dst-field-align.rs'
2019-07-17T00:34:23.2726570Z error: 1 errors occurred comparing output.
2019-07-17T00:34:23.2726615Z status: exit code: 1
2019-07-17T00:34:23.2726615Z status: exit code: 1
2019-07-17T00:34:23.2727321Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/dst-field-align.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/dst-field-align.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/dst-field-align.stage-id.aux" "-A" "unused"
2019-07-17T00:34:23.2727789Z ------------------------------------------
2019-07-17T00:34:23.2727823Z 
2019-07-17T00:34:23.2728045Z ------------------------------------------
2019-07-17T00:34:23.2728108Z stderr:
---
2019-07-17T00:34:23.3566863Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:23.3566911Z +
2019-07-17T00:34:23.3566959Z 
2019-07-17T00:34:23.3567016Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:23.3567280Z Actual stderr saved to /tmp/compiletest9Cm2qq/dst-irrefutable-bind.stderr
2019-07-17T00:34:23.3567355Z To update references, run this command from build directory:
2019-07-17T00:34:23.3567762Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'dst-irrefutable-bind.rs'
2019-07-17T00:34:23.3567844Z error: 1 errors occurred comparing output.
2019-07-17T00:34:23.3567911Z status: exit code: 1
2019-07-17T00:34:23.3567911Z status: exit code: 1
2019-07-17T00:34:23.3568582Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/dst-irrefutable-bind.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/dst-irrefutable-bind.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/dst-irrefutable-bind.stage-id.aux" "-A" "unused"
2019-07-17T00:34:23.3568925Z ------------------------------------------
2019-07-17T00:34:23.3568960Z 
2019-07-17T00:34:23.3569199Z ------------------------------------------
2019-07-17T00:34:23.3569256Z stderr:
---
2019-07-17T00:34:23.4963993Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:23.4964050Z +
2019-07-17T00:34:23.4964071Z 
2019-07-17T00:34:23.4964107Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:23.4964327Z Actual stderr saved to /tmp/compiletest9Cm2qq/dst-raw.stderr
2019-07-17T00:34:23.4964372Z To update references, run this command from build directory:
2019-07-17T00:34:23.4964576Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'dst-raw.rs'
2019-07-17T00:34:23.4964723Z error: 1 errors occurred comparing output.
2019-07-17T00:34:23.4964759Z status: exit code: 1
2019-07-17T00:34:23.4964759Z status: exit code: 1
2019-07-17T00:34:23.4965464Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/dst-raw.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/dst-raw.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/dst-raw.stage-id.aux" "-A" "unused"
2019-07-17T00:34:23.4966295Z ------------------------------------------
2019-07-17T00:34:23.4966404Z 
2019-07-17T00:34:23.4966669Z ------------------------------------------
2019-07-17T00:34:23.4966716Z stderr:
---
2019-07-17T00:34:23.5530399Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:23.5530632Z +
2019-07-17T00:34:23.5530801Z 
2019-07-17T00:34:23.5530968Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:23.5531502Z Actual stderr saved to /tmp/compiletest9Cm2qq/dst-struct-sole.stderr
2019-07-17T00:34:23.5531752Z To update references, run this command from build directory:
2019-07-17T00:34:23.5532141Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'dst-struct-sole.rs'
2019-07-17T00:34:23.5532522Z error: 1 errors occurred comparing output.
2019-07-17T00:34:23.5532686Z status: exit code: 1
2019-07-17T00:34:23.5532686Z status: exit code: 1
2019-07-17T00:34:23.5533386Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/dst-struct-sole.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/dst-struct-sole.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/dst-struct-sole.stage-id.aux" "-A" "unused"
2019-07-17T00:34:23.5534024Z ------------------------------------------
2019-07-17T00:34:23.5534240Z 
2019-07-17T00:34:23.5534597Z ------------------------------------------
2019-07-17T00:34:23.5534808Z stderr:
---
2019-07-17T00:34:23.6793286Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:23.6793408Z +
2019-07-17T00:34:23.6793431Z 
2019-07-17T00:34:23.6793471Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:23.6793749Z Actual stderr saved to /tmp/compiletest9Cm2qq/enum-nullable-const-null-with-fields.stderr
2019-07-17T00:34:23.6793796Z To update references, run this command from build directory:
2019-07-17T00:34:23.6794039Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'enum-nullable-const-null-with-fields.rs'
2019-07-17T00:34:23.6794125Z error: 1 errors occurred comparing output.
2019-07-17T00:34:23.6794162Z status: exit code: 1
2019-07-17T00:34:23.6794162Z status: exit code: 1
2019-07-17T00:34:23.6794777Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/enum-nullable-const-null-with-fields.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/enum-nullable-const-null-with-fields.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/enum-nullable-const-null-with-fields.stage-id.aux" "-A" "unused"
2019-07-17T00:34:23.6795242Z ------------------------------------------
2019-07-17T00:34:23.6795270Z 
2019-07-17T00:34:23.6796464Z ------------------------------------------
2019-07-17T00:34:23.6796525Z stderr:
---
2019-07-17T00:34:23.7605159Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:23.7605202Z +
2019-07-17T00:34:23.7605225Z 
2019-07-17T00:34:23.7605282Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:23.7605677Z Actual stderr saved to /tmp/compiletest9Cm2qq/dst-struct.stderr
2019-07-17T00:34:23.7605726Z To update references, run this command from build directory:
2019-07-17T00:34:23.7606682Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'dst-struct.rs'
2019-07-17T00:34:23.7606770Z error: 1 errors occurred comparing output.
2019-07-17T00:34:23.7606827Z status: exit code: 1
2019-07-17T00:34:23.7606827Z status: exit code: 1
2019-07-17T00:34:23.7607504Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/dst-struct.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/dst-struct.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/dst-struct.stage-id.aux" "-A" "unused"
2019-07-17T00:34:23.7607841Z ------------------------------------------
2019-07-17T00:34:23.7607876Z 
2019-07-17T00:34:23.7608097Z ------------------------------------------
2019-07-17T00:34:23.7608161Z stderr:
---
2019-07-17T00:34:23.8563985Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:23.8564025Z +
2019-07-17T00:34:23.8564067Z 
2019-07-17T00:34:23.8564105Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:23.8564148Z Actual stderr saved to /tmp/compiletest9Cm2qq/enums.stderr
2019-07-17T00:34:23.8564207Z To update references, run this command from build directory:
2019-07-17T00:34:23.8564434Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'enums.rs'
2019-07-17T00:34:23.8564499Z error: 1 errors occurred comparing output.
2019-07-17T00:34:23.8564554Z status: exit code: 1
2019-07-17T00:34:23.8564554Z status: exit code: 1
2019-07-17T00:34:23.8565086Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/enums.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/enums.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/enums.stage-id.aux" "-A" "unused"
2019-07-17T00:34:23.8565376Z ------------------------------------------
2019-07-17T00:34:23.8565405Z 
2019-07-17T00:34:23.8565796Z ------------------------------------------
2019-07-17T00:34:23.8565834Z stderr:
---
2019-07-17T00:34:23.9082696Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:23.9082737Z +
2019-07-17T00:34:23.9082762Z 
2019-07-17T00:34:23.9082800Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:23.9082858Z Actual stderr saved to /tmp/compiletest9Cm2qq/env.stderr
2019-07-17T00:34:23.9082900Z To update references, run this command from build directory:
2019-07-17T00:34:23.9083124Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'env.rs'
2019-07-17T00:34:23.9083387Z error: 1 errors occurred comparing output.
2019-07-17T00:34:23.9083424Z status: exit code: 1
2019-07-17T00:34:23.9083424Z status: exit code: 1
2019-07-17T00:34:23.9083971Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/env.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/env.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/env.stage-id.aux" "-A" "unused"
2019-07-17T00:34:23.9084235Z ------------------------------------------
2019-07-17T00:34:23.9084279Z 
2019-07-17T00:34:23.9084463Z ------------------------------------------
2019-07-17T00:34:23.9084500Z stderr:
---
2019-07-17T00:34:23.9900094Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:23.9900132Z +
2019-07-17T00:34:23.9900154Z 
2019-07-17T00:34:23.9900206Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:23.9900246Z Actual stderr saved to /tmp/compiletest9Cm2qq/exit.stderr
2019-07-17T00:34:23.9900284Z To update references, run this command from build directory:
2019-07-17T00:34:23.9900499Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'exit.rs'
2019-07-17T00:34:23.9900571Z error: 1 errors occurred comparing output.
2019-07-17T00:34:23.9900607Z status: exit code: 1
2019-07-17T00:34:23.9900607Z status: exit code: 1
2019-07-17T00:34:23.9901110Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/exit.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/exit.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/exit.stage-id.aux" "-A" "unused"
2019-07-17T00:34:23.9901374Z ------------------------------------------
2019-07-17T00:34:23.9901399Z 
2019-07-17T00:34:23.9901571Z ------------------------------------------
2019-07-17T00:34:23.9901610Z stderr:
---
2019-07-17T00:34:24.0242629Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:24.0242686Z +
2019-07-17T00:34:24.0242708Z 
2019-07-17T00:34:24.0242745Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:24.0242805Z Actual stderr saved to /tmp/compiletest9Cm2qq/extern_types.stderr
2019-07-17T00:34:24.0242847Z To update references, run this command from build directory:
2019-07-17T00:34:24.0243078Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'extern_types.rs'
2019-07-17T00:34:24.0243162Z error: 1 errors occurred comparing output.
2019-07-17T00:34:24.0243199Z status: exit code: 1
2019-07-17T00:34:24.0243199Z status: exit code: 1
2019-07-17T00:34:24.0243727Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/extern_types.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/extern_types.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/extern_types.stage-id.aux" "-A" "unused"
2019-07-17T00:34:24.0244003Z ------------------------------------------
2019-07-17T00:34:24.0244030Z 
2019-07-17T00:34:24.0244237Z ------------------------------------------
2019-07-17T00:34:24.0244274Z stderr:
---
2019-07-17T00:34:24.1448226Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:24.1448280Z +
2019-07-17T00:34:24.1448327Z 
2019-07-17T00:34:24.1448377Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:24.1448431Z Actual stderr saved to /tmp/compiletest9Cm2qq/float_fast_math.stderr
2019-07-17T00:34:24.1448502Z To update references, run this command from build directory:
2019-07-17T00:34:24.1448821Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'float_fast_math.rs'
2019-07-17T00:34:24.1448918Z error: 1 errors occurred comparing output.
2019-07-17T00:34:24.1448983Z status: exit code: 1
2019-07-17T00:34:24.1448983Z status: exit code: 1
2019-07-17T00:34:24.1449681Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/float_fast_math.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/float_fast_math.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/float_fast_math.stage-id.aux" "-A" "unused"
2019-07-17T00:34:24.1450391Z ------------------------------------------
2019-07-17T00:34:24.1450586Z 
2019-07-17T00:34:24.1450941Z ------------------------------------------
2019-07-17T00:34:24.1450978Z stderr:
---
2019-07-17T00:34:24.2708304Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:24.2708356Z +
2019-07-17T00:34:24.2708385Z 
2019-07-17T00:34:24.2708434Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:24.2708505Z Actual stderr saved to /tmp/compiletest9Cm2qq/floats.stderr
2019-07-17T00:34:24.2708557Z To update references, run this command from build directory:
2019-07-17T00:34:24.2708849Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'floats.rs'
2019-07-17T00:34:24.2708949Z error: 1 errors occurred comparing output.
2019-07-17T00:34:24.2708996Z status: exit code: 1
2019-07-17T00:34:24.2708996Z status: exit code: 1
2019-07-17T00:34:24.2709680Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/floats.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/floats.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/floats.stage-id.aux" "-A" "unused"
2019-07-17T00:34:24.2710149Z ------------------------------------------
2019-07-17T00:34:24.2710193Z 
2019-07-17T00:34:24.2710400Z ------------------------------------------
2019-07-17T00:34:24.2710438Z stderr:
---
2019-07-17T00:34:24.3109104Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:24.3109159Z +
2019-07-17T00:34:24.3109202Z 
2019-07-17T00:34:24.3109265Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:24.3109587Z Actual stderr saved to /tmp/compiletest9Cm2qq/foreign-fn-linkname.stderr
2019-07-17T00:34:24.3109842Z To update references, run this command from build directory:
2019-07-17T00:34:24.3110273Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'foreign-fn-linkname.rs'
2019-07-17T00:34:24.3110367Z error: 1 errors occurred comparing output.
2019-07-17T00:34:24.3110421Z status: exit code: 1
2019-07-17T00:34:24.3110421Z status: exit code: 1
2019-07-17T00:34:24.3110958Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/foreign-fn-linkname.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/foreign-fn-linkname.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/foreign-fn-linkname.stage-id.aux" "-A" "unused"
2019-07-17T00:34:24.3111242Z ------------------------------------------
2019-07-17T00:34:24.3111269Z 
2019-07-17T00:34:24.3111471Z ------------------------------------------
2019-07-17T00:34:24.3111508Z stderr:
---
2019-07-17T00:34:24.4080797Z -hello00000
2019-07-17T00:34:24.4080969Z -
2019-07-17T00:34:24.4080993Z 
2019-07-17T00:34:24.4081031Z The actual stdout differed from the expected stdout.
2019-07-17T00:34:24.4081091Z Actual stdout saved to /tmp/compiletest9Cm2qq/format.stdout
2019-07-17T00:34:24.4081176Z error[E0080]: Miri evaluation error: attempted to do invalid arithmetic on pointers that would leak base addresses, e.g., comparing pointers into different allocations
2019-07-17T00:34:24.4081408Z     --> /checkout/src/libcore/intrinsics.rs:1340:19
2019-07-17T00:34:24.4081457Z      |
2019-07-17T00:34:24.4081492Z 1340 |     let diff = if src_usize > dst_usize {
---
2019-07-17T00:34:24.4084698Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:24.4084740Z +
2019-07-17T00:34:24.4084780Z 
2019-07-17T00:34:24.4084820Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:24.4084863Z Actual stderr saved to /tmp/compiletest9Cm2qq/format.stderr
2019-07-17T00:34:24.4084927Z To update references, run this command from build directory:
2019-07-17T00:34:24.4085178Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'format.rs'
2019-07-17T00:34:24.4085245Z error: 2 errors occurred comparing output.
2019-07-17T00:34:24.4085300Z status: exit code: 1
2019-07-17T00:34:24.4085300Z status: exit code: 1
2019-07-17T00:34:24.4086491Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/format.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/format.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/format.stage-id.aux" "-A" "unused"
2019-07-17T00:34:24.4086860Z ------------------------------------------
2019-07-17T00:34:24.4086895Z 
2019-07-17T00:34:24.4087123Z ------------------------------------------
2019-07-17T00:34:24.4087190Z stderr:
---
2019-07-17T00:34:24.4170592Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:24.4170650Z +
2019-07-17T00:34:24.4170671Z 
2019-07-17T00:34:24.4170708Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:24.4170766Z Actual stderr saved to /tmp/compiletest9Cm2qq/from_utf8.stderr
2019-07-17T00:34:24.4170807Z To update references, run this command from build directory:
2019-07-17T00:34:24.4171023Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'from_utf8.rs'
2019-07-17T00:34:24.4171111Z error: 1 errors occurred comparing output.
2019-07-17T00:34:24.4171149Z status: exit code: 1
2019-07-17T00:34:24.4171149Z status: exit code: 1
2019-07-17T00:34:24.4171660Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/from_utf8.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/from_utf8.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/from_utf8.stage-id.aux" "-A" "unused"
2019-07-17T00:34:24.4172180Z ------------------------------------------
2019-07-17T00:34:24.4172208Z 
2019-07-17T00:34:24.4172414Z ------------------------------------------
2019-07-17T00:34:24.4172451Z stderr:
---
2019-07-17T00:34:24.5923625Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:24.5923666Z +
2019-07-17T00:34:24.5923688Z 
2019-07-17T00:34:24.5923724Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:24.5923790Z Actual stderr saved to /tmp/compiletest9Cm2qq/function_pointers.stderr
2019-07-17T00:34:24.5923834Z To update references, run this command from build directory:
2019-07-17T00:34:24.5924086Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'function_pointers.rs'
2019-07-17T00:34:24.5924177Z error: 1 errors occurred comparing output.
2019-07-17T00:34:24.5924214Z status: exit code: 1
2019-07-17T00:34:24.5924214Z status: exit code: 1
2019-07-17T00:34:24.5924771Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/function_pointers.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/function_pointers.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/function_pointers.stage-id.aux" "-A" "unused"
2019-07-17T00:34:24.5925047Z ------------------------------------------
2019-07-17T00:34:24.5925162Z 
2019-07-17T00:34:24.5925562Z ------------------------------------------
2019-07-17T00:34:24.5925600Z stderr:
---
2019-07-17T00:34:24.6045240Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:24.6045284Z +
2019-07-17T00:34:24.6045307Z 
2019-07-17T00:34:24.6045344Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:24.6045403Z Actual stderr saved to /tmp/compiletest9Cm2qq/generator.stderr
2019-07-17T00:34:24.6045457Z To update references, run this command from build directory:
2019-07-17T00:34:24.6045860Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'generator.rs'
2019-07-17T00:34:24.6046784Z error: 1 errors occurred comparing output.
2019-07-17T00:34:24.6046843Z status: exit code: 1
2019-07-17T00:34:24.6046843Z status: exit code: 1
2019-07-17T00:34:24.6047575Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/generator.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/generator.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/generator.stage-id.aux" "-A" "unused"
2019-07-17T00:34:24.6047897Z ------------------------------------------
2019-07-17T00:34:24.6047950Z 
2019-07-17T00:34:24.6048183Z ------------------------------------------
2019-07-17T00:34:24.6048378Z stderr:
---
2019-07-17T00:34:24.7251521Z -Hello, world!
2019-07-17T00:34:24.7254782Z -
2019-07-17T00:34:24.7259365Z 
2019-07-17T00:34:24.7262754Z The actual stdout differed from the expected stdout.
2019-07-17T00:34:24.7265478Z Actual stdout saved to /tmp/compiletest9Cm2qq/hello.stdout
2019-07-17T00:34:24.7284883Z error[E0080]: Miri evaluation error: attempted to do invalid arithmetic on pointers that would leak base addresses, e.g., comparing pointers into different allocations
2019-07-17T00:34:24.7285333Z     --> /checkout/src/libcore/intrinsics.rs:1340:19
2019-07-17T00:34:24.7285381Z      |
2019-07-17T00:34:24.7285422Z 1340 |     let diff = if src_usize > dst_usize {
---
2019-07-17T00:34:24.7290002Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:24.7290043Z +
2019-07-17T00:34:24.7290082Z 
2019-07-17T00:34:24.7290119Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:24.7290162Z Actual stderr saved to /tmp/compiletest9Cm2qq/hello.stderr
2019-07-17T00:34:24.7290221Z To update references, run this command from build directory:
2019-07-17T00:34:24.7290452Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'hello.rs'
2019-07-17T00:34:24.7290520Z error: 2 errors occurred comparing output.
2019-07-17T00:34:24.7290573Z status: exit code: 1
2019-07-17T00:34:24.7290573Z status: exit code: 1
2019-07-17T00:34:24.7291172Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/hello.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/hello.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/hello.stage-id.aux" "-A" "unused"
2019-07-17T00:34:24.7291487Z ------------------------------------------
2019-07-17T00:34:24.7291517Z 
2019-07-17T00:34:24.7291708Z ------------------------------------------
2019-07-17T00:34:24.7291763Z stderr:
---
2019-07-17T00:34:24.7559884Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:24.7559922Z +
2019-07-17T00:34:24.7559943Z 
2019-07-17T00:34:24.7559995Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:24.7560034Z Actual stderr saved to /tmp/compiletest9Cm2qq/heap.stderr
2019-07-17T00:34:24.7560073Z To update references, run this command from build directory:
2019-07-17T00:34:24.7560362Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'heap.rs'
2019-07-17T00:34:24.7560436Z error: 1 errors occurred comparing output.
2019-07-17T00:34:24.7560476Z status: exit code: 1
2019-07-17T00:34:24.7560476Z status: exit code: 1
2019-07-17T00:34:24.7561035Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/heap.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/heap.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/heap.stage-id.aux" "-A" "unused"
2019-07-17T00:34:24.7561329Z ------------------------------------------
2019-07-17T00:34:24.7561357Z 
2019-07-17T00:34:24.7561554Z ------------------------------------------
2019-07-17T00:34:24.7561592Z stderr:
---
2019-07-17T00:34:25.3110692Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:25.3110732Z +
2019-07-17T00:34:25.3110753Z 
2019-07-17T00:34:25.3110809Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:25.3111024Z Actual stderr saved to /tmp/compiletest9Cm2qq/integer-ops.stderr
2019-07-17T00:34:25.3111068Z To update references, run this command from build directory:
2019-07-17T00:34:25.3111302Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'integer-ops.rs'
2019-07-17T00:34:25.3111367Z error: 1 errors occurred comparing output.
2019-07-17T00:34:25.3111488Z status: exit code: 1
2019-07-17T00:34:25.3111488Z status: exit code: 1
2019-07-17T00:34:25.3112052Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/integer-ops.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/integer-ops.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/integer-ops.stage-id.aux" "-A" "unused"
2019-07-17T00:34:25.3112338Z ------------------------------------------
2019-07-17T00:34:25.3112366Z 
2019-07-17T00:34:25.3112552Z ------------------------------------------
2019-07-17T00:34:25.3112607Z stderr:
---
2019-07-17T00:34:25.7883532Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:25.7883592Z +
2019-07-17T00:34:25.7883616Z 
2019-07-17T00:34:25.7883655Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:25.7883907Z Actual stderr saved to /tmp/compiletest9Cm2qq/intrinsics-math.stderr
2019-07-17T00:34:25.7883969Z To update references, run this command from build directory:
2019-07-17T00:34:25.7884214Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'intrinsics-math.rs'
2019-07-17T00:34:25.7884443Z error: 1 errors occurred comparing output.
2019-07-17T00:34:25.7884481Z status: exit code: 1
2019-07-17T00:34:25.7884481Z status: exit code: 1
2019-07-17T00:34:25.7885062Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/intrinsics-math.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/intrinsics-math.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/intrinsics-math.stage-id.aux" "-A" "unused"
2019-07-17T00:34:25.7885371Z ------------------------------------------
2019-07-17T00:34:25.7885400Z 
2019-07-17T00:34:25.7885623Z ------------------------------------------
2019-07-17T00:34:25.7885831Z stderr:
---
2019-07-17T00:34:25.9525357Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:25.9525416Z +
2019-07-17T00:34:25.9525440Z 
2019-07-17T00:34:25.9525477Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:25.9525519Z Actual stderr saved to /tmp/compiletest9Cm2qq/intrinsics.stderr
2019-07-17T00:34:25.9525576Z To update references, run this command from build directory:
2019-07-17T00:34:25.9526752Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'intrinsics.rs'
2019-07-17T00:34:25.9526866Z error: 1 errors occurred comparing output.
2019-07-17T00:34:25.9526931Z status: exit code: 1
2019-07-17T00:34:25.9526931Z status: exit code: 1
2019-07-17T00:34:25.9527661Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/intrinsics.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/intrinsics.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/intrinsics.stage-id.aux" "-A" "unused"
2019-07-17T00:34:25.9528002Z ------------------------------------------
2019-07-17T00:34:25.9528037Z 
2019-07-17T00:34:25.9528288Z ------------------------------------------
2019-07-17T00:34:25.9528335Z stderr:
---
2019-07-17T00:34:26.1563611Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:26.1563653Z +
2019-07-17T00:34:26.1563677Z 
2019-07-17T00:34:26.1563715Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:26.1563774Z Actual stderr saved to /tmp/compiletest9Cm2qq/ints.stderr
2019-07-17T00:34:26.1563943Z To update references, run this command from build directory:
2019-07-17T00:34:26.1564234Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'ints.rs'
2019-07-17T00:34:26.1564320Z error: 1 errors occurred comparing output.
2019-07-17T00:34:26.1564365Z status: exit code: 1
2019-07-17T00:34:26.1564365Z status: exit code: 1
2019-07-17T00:34:26.1564904Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/ints.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/ints.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/ints.stage-id.aux" "-A" "unused"
2019-07-17T00:34:26.1565168Z ------------------------------------------
2019-07-17T00:34:26.1565216Z 
2019-07-17T00:34:26.1565417Z ------------------------------------------
2019-07-17T00:34:26.1565455Z stderr:
---
2019-07-17T00:34:26.2842799Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:26.2842842Z +
2019-07-17T00:34:26.2842865Z 
2019-07-17T00:34:26.2842923Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:26.2843152Z Actual stderr saved to /tmp/compiletest9Cm2qq/issue-15063.stderr
2019-07-17T00:34:26.2843326Z To update references, run this command from build directory:
2019-07-17T00:34:26.2843616Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'issue-15063.rs'
2019-07-17T00:34:26.2843692Z error: 1 errors occurred comparing output.
2019-07-17T00:34:26.2843746Z status: exit code: 1
2019-07-17T00:34:26.2843746Z status: exit code: 1
2019-07-17T00:34:26.2844291Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-15063.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/issue-15063.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/issue-15063.stage-id.aux" "-A" "unused"
2019-07-17T00:34:26.2844573Z ------------------------------------------
2019-07-17T00:34:26.2844601Z 
2019-07-17T00:34:26.2844802Z ------------------------------------------
2019-07-17T00:34:26.2844858Z stderr:
---
2019-07-17T00:34:26.4484397Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:26.4484450Z +
2019-07-17T00:34:26.4484473Z 
2019-07-17T00:34:26.4484531Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:26.4484789Z Actual stderr saved to /tmp/compiletest9Cm2qq/issue-15080.stderr
2019-07-17T00:34:26.4484844Z To update references, run this command from build directory:
2019-07-17T00:34:26.4485090Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'issue-15080.rs'
2019-07-17T00:34:26.4485157Z error: 1 errors occurred comparing output.
2019-07-17T00:34:26.4485211Z status: exit code: 1
2019-07-17T00:34:26.4485211Z status: exit code: 1
2019-07-17T00:34:26.4485926Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-15080.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/issue-15080.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/issue-15080.stage-id.aux" "-A" "unused"
2019-07-17T00:34:26.4486854Z ------------------------------------------
2019-07-17T00:34:26.4487015Z 
2019-07-17T00:34:26.4487269Z ------------------------------------------
2019-07-17T00:34:26.4487335Z stderr:
---
2019-07-17T00:34:26.6122914Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:26.6122972Z +
2019-07-17T00:34:26.6122995Z 
2019-07-17T00:34:26.6123032Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:26.6123254Z Actual stderr saved to /tmp/compiletest9Cm2qq/issue-15523-big.stderr
2019-07-17T00:34:26.6123315Z To update references, run this command from build directory:
2019-07-17T00:34:26.6123538Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'issue-15523-big.rs'
2019-07-17T00:34:26.6123601Z error: 1 errors occurred comparing output.
2019-07-17T00:34:26.6123653Z status: exit code: 1
2019-07-17T00:34:26.6123653Z status: exit code: 1
2019-07-17T00:34:26.6124180Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-15523-big.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/issue-15523-big.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/issue-15523-big.stage-id.aux" "-A" "unused"
2019-07-17T00:34:26.6124543Z ------------------------------------------
2019-07-17T00:34:26.6124570Z 
2019-07-17T00:34:26.6124774Z ------------------------------------------
2019-07-17T00:34:26.6124811Z stderr:
---
2019-07-17T00:34:26.7705852Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:26.7705915Z +
2019-07-17T00:34:26.7706113Z 
2019-07-17T00:34:26.7706151Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:26.7706879Z Actual stderr saved to /tmp/compiletest9Cm2qq/issue-17877.stderr
2019-07-17T00:34:26.7706956Z To update references, run this command from build directory:
2019-07-17T00:34:26.7707240Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'issue-17877.rs'
2019-07-17T00:34:26.7707322Z error: 1 errors occurred comparing output.
2019-07-17T00:34:26.7707377Z status: exit code: 1
2019-07-17T00:34:26.7707377Z status: exit code: 1
2019-07-17T00:34:26.7708053Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-17877.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/issue-17877.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/issue-17877.stage-id.aux" "-A" "unused"
2019-07-17T00:34:26.7708529Z ------------------------------------------
2019-07-17T00:34:26.7708564Z 
2019-07-17T00:34:26.7708809Z ------------------------------------------
2019-07-17T00:34:26.7708855Z stderr:
---
2019-07-17T00:34:26.8986825Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:26.8986883Z +
2019-07-17T00:34:26.8986910Z 
2019-07-17T00:34:26.8986956Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:26.8987265Z Actual stderr saved to /tmp/compiletest9Cm2qq/issue-20575.stderr
2019-07-17T00:34:26.8987322Z To update references, run this command from build directory:
2019-07-17T00:34:26.8987598Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'issue-20575.rs'
2019-07-17T00:34:26.8987696Z error: 1 errors occurred comparing output.
2019-07-17T00:34:26.8987741Z status: exit code: 1
2019-07-17T00:34:26.8987741Z status: exit code: 1
2019-07-17T00:34:26.8988415Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-20575.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/issue-20575.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/issue-20575.stage-id.aux" "-A" "unused"
2019-07-17T00:34:26.8988882Z ------------------------------------------
2019-07-17T00:34:26.8988936Z 
2019-07-17T00:34:26.8989169Z ------------------------------------------
2019-07-17T00:34:26.8989215Z stderr:
---
2019-07-17T00:34:27.0684226Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:27.0684298Z +
2019-07-17T00:34:27.0684322Z 
2019-07-17T00:34:27.0684360Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:27.0684625Z Actual stderr saved to /tmp/compiletest9Cm2qq/issue-23261.stderr
2019-07-17T00:34:27.0684671Z To update references, run this command from build directory:
2019-07-17T00:34:27.0684917Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'issue-23261.rs'
2019-07-17T00:34:27.0685005Z error: 1 errors occurred comparing output.
2019-07-17T00:34:27.0685042Z status: exit code: 1
2019-07-17T00:34:27.0685042Z status: exit code: 1
2019-07-17T00:34:27.0685593Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-23261.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/issue-23261.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/issue-23261.stage-id.aux" "-A" "unused"
2019-07-17T00:34:27.0686894Z ------------------------------------------
2019-07-17T00:34:27.0686939Z 
2019-07-17T00:34:27.0687319Z ------------------------------------------
2019-07-17T00:34:27.0687367Z stderr:
---
2019-07-17T00:34:27.2324237Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:27.2324291Z +
2019-07-17T00:34:27.2324316Z 
2019-07-17T00:34:27.2324374Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:27.2324639Z Actual stderr saved to /tmp/compiletest9Cm2qq/issue-26709.stderr
2019-07-17T00:34:27.2324686Z To update references, run this command from build directory:
2019-07-17T00:34:27.2324954Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'issue-26709.rs'
2019-07-17T00:34:27.2325024Z error: 1 errors occurred comparing output.
2019-07-17T00:34:27.2325080Z status: exit code: 1
2019-07-17T00:34:27.2325080Z status: exit code: 1
2019-07-17T00:34:27.2325643Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-26709.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/issue-26709.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/issue-26709.stage-id.aux" "-A" "unused"
2019-07-17T00:34:27.2326837Z ------------------------------------------
2019-07-17T00:34:27.2326883Z 
2019-07-17T00:34:27.2327142Z ------------------------------------------
2019-07-17T00:34:27.2327209Z stderr:
---
2019-07-17T00:34:27.3719226Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:27.3719279Z +
2019-07-17T00:34:27.3719330Z 
2019-07-17T00:34:27.3719378Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:27.3719669Z Actual stderr saved to /tmp/compiletest9Cm2qq/issue-27901.stderr
2019-07-17T00:34:27.3719744Z To update references, run this command from build directory:
2019-07-17T00:34:27.3720351Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'issue-27901.rs'
2019-07-17T00:34:27.3720421Z error: 1 errors occurred comparing output.
2019-07-17T00:34:27.3720473Z status: exit code: 1
2019-07-17T00:34:27.3720473Z status: exit code: 1
2019-07-17T00:34:27.3720993Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-27901.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/issue-27901.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/issue-27901.stage-id.aux" "-A" "unused"
2019-07-17T00:34:27.3721366Z ------------------------------------------
2019-07-17T00:34:27.3721394Z 
2019-07-17T00:34:27.3721578Z ------------------------------------------
2019-07-17T00:34:27.3721631Z stderr:
---
2019-07-17T00:34:27.5324446Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:27.5324487Z +
2019-07-17T00:34:27.5324528Z 
2019-07-17T00:34:27.5324566Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:27.5324804Z Actual stderr saved to /tmp/compiletest9Cm2qq/issue-29746.stderr
2019-07-17T00:34:27.5324847Z To update references, run this command from build directory:
2019-07-17T00:34:27.5325106Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'issue-29746.rs'
2019-07-17T00:34:27.5325174Z error: 1 errors occurred comparing output.
2019-07-17T00:34:27.5325228Z status: exit code: 1
2019-07-17T00:34:27.5325228Z status: exit code: 1
2019-07-17T00:34:27.5326044Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-29746.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/issue-29746.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/issue-29746.stage-id.aux" "-A" "unused"
2019-07-17T00:34:27.5327063Z ------------------------------------------
2019-07-17T00:34:27.5327104Z 
2019-07-17T00:34:27.5327341Z ------------------------------------------
2019-07-17T00:34:27.5327409Z stderr:
---
2019-07-17T00:34:27.5459693Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:27.5459743Z +
2019-07-17T00:34:27.5459771Z 
2019-07-17T00:34:27.5459995Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:27.5460361Z Actual stderr saved to /tmp/compiletest9Cm2qq/intrinsics-integer.stderr
2019-07-17T00:34:27.5460412Z To update references, run this command from build directory:
2019-07-17T00:34:27.5460651Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'intrinsics-integer.rs'
2019-07-17T00:34:27.5460714Z error: 1 errors occurred comparing output.
2019-07-17T00:34:27.5460846Z status: exit code: 1
2019-07-17T00:34:27.5460846Z status: exit code: 1
2019-07-17T00:34:27.5461379Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/intrinsics-integer.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/intrinsics-integer.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/intrinsics-integer.stage-id.aux" "-A" "unused"
2019-07-17T00:34:27.5461645Z ------------------------------------------
2019-07-17T00:34:27.5461672Z 
2019-07-17T00:34:27.5461860Z ------------------------------------------
2019-07-17T00:34:27.5461912Z stderr:
---
2019-07-17T00:34:27.6465370Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:27.6465408Z +
2019-07-17T00:34:27.6465430Z 
2019-07-17T00:34:27.6465468Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:27.6465721Z Actual stderr saved to /tmp/compiletest9Cm2qq/issue-30530.stderr
2019-07-17T00:34:27.6465765Z To update references, run this command from build directory:
2019-07-17T00:34:27.6466571Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'issue-30530.rs'
2019-07-17T00:34:27.6466794Z error: 1 errors occurred comparing output.
2019-07-17T00:34:27.6466840Z status: exit code: 1
2019-07-17T00:34:27.6466840Z status: exit code: 1
2019-07-17T00:34:27.6467704Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-30530.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/issue-30530.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/issue-30530.stage-id.aux" "-A" "unused"
2019-07-17T00:34:27.6468051Z ------------------------------------------
2019-07-17T00:34:27.6468105Z 
2019-07-17T00:34:27.6468342Z ------------------------------------------
2019-07-17T00:34:27.6468389Z stderr:
---
2019-07-17T00:34:27.6924418Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:27.6924474Z +
2019-07-17T00:34:27.6924497Z 
2019-07-17T00:34:27.6924541Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:27.6924782Z Actual stderr saved to /tmp/compiletest9Cm2qq/issue-31267-additional.stderr
2019-07-17T00:34:27.6924845Z To update references, run this command from build directory:
2019-07-17T00:34:27.6925176Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'issue-31267-additional.rs'
2019-07-17T00:34:27.6925259Z error: 1 errors occurred comparing output.
2019-07-17T00:34:27.6925297Z status: exit code: 1
2019-07-17T00:34:27.6925297Z status: exit code: 1
2019-07-17T00:34:27.6926043Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-31267-additional.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/issue-31267-additional.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/issue-31267-additional.stage-id.aux" "-A" "unused"
2019-07-17T00:34:27.6926917Z ------------------------------------------
2019-07-17T00:34:27.6926980Z 
2019-07-17T00:34:27.6927221Z ------------------------------------------
2019-07-17T00:34:27.6927279Z stderr:
---
2019-07-17T00:34:27.7707951Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:27.7708002Z +
2019-07-17T00:34:27.7708030Z 
2019-07-17T00:34:27.7708107Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:27.7708394Z Actual stderr saved to /tmp/compiletest9Cm2qq/issue-33387.stderr
2019-07-17T00:34:27.7708451Z To update references, run this command from build directory:
2019-07-17T00:34:27.7708878Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'issue-33387.rs'
2019-07-17T00:34:27.7708963Z error: 1 errors occurred comparing output.
2019-07-17T00:34:27.7709009Z status: exit code: 1
2019-07-17T00:34:27.7709009Z status: exit code: 1
2019-07-17T00:34:27.7709707Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-33387.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/issue-33387.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/issue-33387.stage-id.aux" "-A" "unused"
2019-07-17T00:34:27.7710202Z ------------------------------------------
2019-07-17T00:34:27.7710230Z 
2019-07-17T00:34:27.7710418Z ------------------------------------------
2019-07-17T00:34:27.7710455Z stderr:
---
2019-07-17T00:34:27.8081132Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:27.8081173Z +
2019-07-17T00:34:27.8081267Z 
2019-07-17T00:34:27.8081305Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:27.8081576Z Actual stderr saved to /tmp/compiletest9Cm2qq/issue-34571.stderr
2019-07-17T00:34:27.8081622Z To update references, run this command from build directory:
2019-07-17T00:34:27.8081856Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'issue-34571.rs'
2019-07-17T00:34:27.8081939Z error: 1 errors occurred comparing output.
2019-07-17T00:34:27.8081976Z status: exit code: 1
2019-07-17T00:34:27.8081976Z status: exit code: 1
2019-07-17T00:34:27.8082539Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-34571.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/issue-34571.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/issue-34571.stage-id.aux" "-A" "unused"
2019-07-17T00:34:27.8082846Z ------------------------------------------
2019-07-17T00:34:27.8082875Z 
2019-07-17T00:34:27.8083074Z ------------------------------------------
2019-07-17T00:34:27.8083112Z stderr:
---
2019-07-17T00:34:27.9071421Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:27.9071461Z +
2019-07-17T00:34:27.9071483Z 
2019-07-17T00:34:27.9071536Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:27.9071854Z Actual stderr saved to /tmp/compiletest9Cm2qq/issue-35815.stderr
2019-07-17T00:34:27.9071898Z To update references, run this command from build directory:
2019-07-17T00:34:27.9072150Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'issue-35815.rs'
2019-07-17T00:34:27.9072215Z error: 1 errors occurred comparing output.
2019-07-17T00:34:27.9072266Z status: exit code: 1
2019-07-17T00:34:27.9072266Z status: exit code: 1
2019-07-17T00:34:27.9072801Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-35815.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/issue-35815.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/issue-35815.stage-id.aux" "-A" "unused"
2019-07-17T00:34:27.9073075Z ------------------------------------------
2019-07-17T00:34:27.9073110Z 
2019-07-17T00:34:27.9073304Z ------------------------------------------
2019-07-17T00:34:27.9073355Z stderr:
---
2019-07-17T00:34:27.9560799Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:27.9560856Z +
2019-07-17T00:34:27.9560879Z 
2019-07-17T00:34:27.9561039Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:27.9561321Z Actual stderr saved to /tmp/compiletest9Cm2qq/issue-36278-prefix-nesting.stderr
2019-07-17T00:34:27.9561367Z To update references, run this command from build directory:
2019-07-17T00:34:27.9561594Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'issue-36278-prefix-nesting.rs'
2019-07-17T00:34:27.9561675Z error: 1 errors occurred comparing output.
2019-07-17T00:34:27.9561710Z status: exit code: 1
2019-07-17T00:34:27.9561710Z status: exit code: 1
2019-07-17T00:34:27.9562265Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-36278-prefix-nesting.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/issue-36278-prefix-nesting.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/issue-36278-prefix-nesting.stage-id.aux" "-A" "unused"
2019-07-17T00:34:27.9562543Z ------------------------------------------
2019-07-17T00:34:27.9562587Z 
2019-07-17T00:34:27.9562772Z ------------------------------------------
2019-07-17T00:34:27.9562809Z stderr:
---
2019-07-17T00:34:28.0440976Z -S { s: 5 }
2019-07-17T00:34:28.0442088Z -
2019-07-17T00:34:28.0444901Z 
2019-07-17T00:34:28.0446813Z The actual stdout differed from the expected stdout.
2019-07-17T00:34:28.0450636Z Actual stdout saved to /tmp/compiletest9Cm2qq/issue-3794.stdout
2019-07-17T00:34:28.0454471Z error[E0080]: Miri evaluation error: attempted to do invalid arithmetic on pointers that would leak base addresses, e.g., comparing pointers into different allocations
2019-07-17T00:34:28.0455038Z     --> /checkout/src/libcore/intrinsics.rs:1340:19
2019-07-17T00:34:28.0455105Z      |
2019-07-17T00:34:28.0455167Z 1340 |     let diff = if src_usize > dst_usize {
---
2019-07-17T00:34:28.0479355Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:28.0479423Z +
2019-07-17T00:34:28.0504943Z 
2019-07-17T00:34:28.0505709Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:28.0507136Z Actual stderr saved to /tmp/compiletest9Cm2qq/issue-3794.stderr
2019-07-17T00:34:28.0507263Z To update references, run this command from build directory:
2019-07-17T00:34:28.0507588Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'issue-3794.rs'
2019-07-17T00:34:28.0507673Z error: 2 errors occurred comparing output.
2019-07-17T00:34:28.0507719Z status: exit code: 1
2019-07-17T00:34:28.0507719Z status: exit code: 1
2019-07-17T00:34:28.0508404Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-3794.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/issue-3794.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/issue-3794.stage-id.aux" "-A" "unused"
2019-07-17T00:34:28.0508737Z ------------------------------------------
2019-07-17T00:34:28.0508771Z 
2019-07-17T00:34:28.0508995Z ------------------------------------------
2019-07-17T00:34:28.0509070Z stderr:
---
2019-07-17T00:34:28.0724351Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:28.0724398Z +
2019-07-17T00:34:28.0724420Z 
2019-07-17T00:34:28.0724478Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:28.0724719Z Actual stderr saved to /tmp/compiletest9Cm2qq/issue-53728.stderr
2019-07-17T00:34:28.0724762Z To update references, run this command from build directory:
2019-07-17T00:34:28.0725009Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'issue-53728.rs'
2019-07-17T00:34:28.0727862Z error: 1 errors occurred comparing output.
2019-07-17T00:34:28.0742484Z status: exit code: 1
2019-07-17T00:34:28.0742484Z status: exit code: 1
2019-07-17T00:34:28.0743390Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-53728.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/issue-53728.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/issue-53728.stage-id.aux" "-A" "unused"
2019-07-17T00:34:28.0744059Z ------------------------------------------
2019-07-17T00:34:28.0744205Z 
2019-07-17T00:34:28.0744517Z ------------------------------------------
2019-07-17T00:34:28.0744669Z stderr:
---
2019-07-17T00:34:28.1946480Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:28.1946558Z +
2019-07-17T00:34:28.1946588Z 
2019-07-17T00:34:28.1946636Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:28.1946912Z Actual stderr saved to /tmp/compiletest9Cm2qq/issue-miri-184.stderr
2019-07-17T00:34:28.1946966Z To update references, run this command from build directory:
2019-07-17T00:34:28.1947230Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'issue-miri-184.rs'
2019-07-17T00:34:28.1947329Z error: 1 errors occurred comparing output.
2019-07-17T00:34:28.1947375Z status: exit code: 1
2019-07-17T00:34:28.1947375Z status: exit code: 1
2019-07-17T00:34:28.1948046Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-miri-184.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/issue-miri-184.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/issue-miri-184.stage-id.aux" "-A" "unused"
2019-07-17T00:34:28.1948382Z ------------------------------------------
2019-07-17T00:34:28.1948416Z 
2019-07-17T00:34:28.1948656Z ------------------------------------------
2019-07-17T00:34:28.1948704Z stderr:
---
2019-07-17T00:34:28.2039733Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:28.2039788Z +
2019-07-17T00:34:28.2039814Z 
2019-07-17T00:34:28.2039859Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:28.2040415Z Actual stderr saved to /tmp/compiletest9Cm2qq/issue-5917.stderr
2019-07-17T00:34:28.2040458Z To update references, run this command from build directory:
2019-07-17T00:34:28.2040665Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'issue-5917.rs'
2019-07-17T00:34:28.2040745Z error: 1 errors occurred comparing output.
2019-07-17T00:34:28.2040781Z status: exit code: 1
2019-07-17T00:34:28.2040781Z status: exit code: 1
2019-07-17T00:34:28.2041326Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-5917.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/issue-5917.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/issue-5917.stage-id.aux" "-A" "unused"
2019-07-17T00:34:28.2041602Z ------------------------------------------
2019-07-17T00:34:28.2041629Z 
2019-07-17T00:34:28.2041808Z ------------------------------------------
2019-07-17T00:34:28.2041845Z stderr:
---
2019-07-17T00:34:28.3482751Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:28.3482793Z +
2019-07-17T00:34:28.3482815Z 
2019-07-17T00:34:28.3482853Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:28.3483096Z Actual stderr saved to /tmp/compiletest9Cm2qq/last-use-in-cap-clause.stderr
2019-07-17T00:34:28.3483143Z To update references, run this command from build directory:
2019-07-17T00:34:28.3483373Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'last-use-in-cap-clause.rs'
2019-07-17T00:34:28.3483462Z error: 1 errors occurred comparing output.
2019-07-17T00:34:28.3483498Z status: exit code: 1
2019-07-17T00:34:28.3483498Z status: exit code: 1
2019-07-17T00:34:28.3484071Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/last-use-in-cap-clause.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/last-use-in-cap-clause.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/last-use-in-cap-clause.stage-id.aux" "-A" "unused"
2019-07-17T00:34:28.3484361Z ------------------------------------------
2019-07-17T00:34:28.3484390Z 
2019-07-17T00:34:28.3484584Z ------------------------------------------
2019-07-17T00:34:28.3484622Z stderr:
---
2019-07-17T00:34:28.3852705Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:28.3852744Z +
2019-07-17T00:34:28.3852767Z 
2019-07-17T00:34:28.3852828Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:28.3852868Z Actual stderr saved to /tmp/compiletest9Cm2qq/iter.stderr
2019-07-17T00:34:28.3852908Z To update references, run this command from build directory:
2019-07-17T00:34:28.3853147Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'iter.rs'
2019-07-17T00:34:28.3853210Z error: 1 errors occurred comparing output.
2019-07-17T00:34:28.3853269Z status: exit code: 1
2019-07-17T00:34:28.3853269Z status: exit code: 1
2019-07-17T00:34:28.3853782Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/iter.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/iter.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/iter.stage-id.aux" "-A" "unused"
2019-07-17T00:34:28.3854066Z ------------------------------------------
2019-07-17T00:34:28.3854093Z 
2019-07-17T00:34:28.3854280Z ------------------------------------------
2019-07-17T00:34:28.3854334Z stderr:
---
2019-07-17T00:34:28.5299076Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:28.5299142Z +
2019-07-17T00:34:28.5299169Z 
2019-07-17T00:34:28.5299213Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:28.5299460Z Actual stderr saved to /tmp/compiletest9Cm2qq/linked-list.stderr
2019-07-17T00:34:28.5299535Z To update references, run this command from build directory:
2019-07-17T00:34:28.5299792Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'linked-list.rs'
2019-07-17T00:34:28.5299902Z error: 1 errors occurred comparing output.
2019-07-17T00:34:28.5300125Z status: exit code: 1
2019-07-17T00:34:28.5300125Z status: exit code: 1
2019-07-17T00:34:28.5300787Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/linked-list.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/linked-list.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/linked-list.stage-id.aux" "-A" "unused"
2019-07-17T00:34:28.5301051Z ------------------------------------------
2019-07-17T00:34:28.5301076Z 
2019-07-17T00:34:28.5301265Z ------------------------------------------
2019-07-17T00:34:28.5301300Z stderr:
---
2019-07-17T00:34:28.6209758Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:28.6209814Z +
2019-07-17T00:34:28.6209843Z 
2019-07-17T00:34:28.6209889Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:28.6210343Z Actual stderr saved to /tmp/compiletest9Cm2qq/loop-break-value.stderr
2019-07-17T00:34:28.6210389Z To update references, run this command from build directory:
2019-07-17T00:34:28.6210609Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'loop-break-value.rs'
2019-07-17T00:34:28.6210752Z error: 1 errors occurred comparing output.
2019-07-17T00:34:28.6210789Z status: exit code: 1
2019-07-17T00:34:28.6210789Z status: exit code: 1
2019-07-17T00:34:28.6211546Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/loop-break-value.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/loop-break-value.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/loop-break-value.stage-id.aux" "-A" "unused"
2019-07-17T00:34:28.6212251Z ------------------------------------------
2019-07-17T00:34:28.6212526Z 
2019-07-17T00:34:28.6212774Z ------------------------------------------
2019-07-17T00:34:28.6212814Z stderr:
---
2019-07-17T00:34:28.7164419Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:28.7164460Z +
2019-07-17T00:34:28.7164483Z 
2019-07-17T00:34:28.7164548Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:28.7164591Z Actual stderr saved to /tmp/compiletest9Cm2qq/loops.stderr
2019-07-17T00:34:28.7164634Z To update references, run this command from build directory:
2019-07-17T00:34:28.7164892Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'loops.rs'
2019-07-17T00:34:28.7164959Z error: 1 errors occurred comparing output.
2019-07-17T00:34:28.7164997Z status: exit code: 1
2019-07-17T00:34:28.7164997Z status: exit code: 1
2019-07-17T00:34:28.7165541Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/loops.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/loops.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/loops.stage-id.aux" "-A" "unused"
2019-07-17T00:34:28.7165887Z ------------------------------------------
2019-07-17T00:34:28.7166096Z 
2019-07-17T00:34:28.7166734Z ------------------------------------------
2019-07-17T00:34:28.7166793Z stderr:
---
2019-07-17T00:34:28.7454057Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:28.7454141Z +
2019-07-17T00:34:28.7454188Z 
2019-07-17T00:34:28.7454227Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:28.7454269Z Actual stderr saved to /tmp/compiletest9Cm2qq/main_fn.stderr
2019-07-17T00:34:28.7454342Z To update references, run this command from build directory:
2019-07-17T00:34:28.7454583Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'main_fn.rs'
2019-07-17T00:34:28.7454656Z error: 1 errors occurred comparing output.
2019-07-17T00:34:28.7454709Z status: exit code: 1
2019-07-17T00:34:28.7454709Z status: exit code: 1
2019-07-17T00:34:28.7455235Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/main_fn.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/main_fn.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/main_fn.stage-id.aux" "-A" "unused"
2019-07-17T00:34:28.7455507Z ------------------------------------------
2019-07-17T00:34:28.7455534Z 
2019-07-17T00:34:28.7455872Z ------------------------------------------
2019-07-17T00:34:28.7456091Z stderr:
---
2019-07-17T00:34:28.8585513Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:28.8585568Z +
2019-07-17T00:34:28.8585590Z 
2019-07-17T00:34:28.8585627Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:28.8585675Z Actual stderr saved to /tmp/compiletest9Cm2qq/many_shr_bor.stderr
2019-07-17T00:34:28.8585734Z To update references, run this command from build directory:
2019-07-17T00:34:28.8586140Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'many_shr_bor.rs'
2019-07-17T00:34:28.8586273Z error: 1 errors occurred comparing output.
2019-07-17T00:34:28.8586312Z status: exit code: 1
2019-07-17T00:34:28.8586312Z status: exit code: 1
2019-07-17T00:34:28.8587347Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/many_shr_bor.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/many_shr_bor.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/many_shr_bor.stage-id.aux" "-A" "unused"
2019-07-17T00:34:28.8587682Z ------------------------------------------
2019-07-17T00:34:28.8587827Z 
2019-07-17T00:34:28.8588103Z ------------------------------------------
2019-07-17T00:34:28.8588150Z stderr:
---
2019-07-17T00:34:28.8679312Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:28.8679362Z +
2019-07-17T00:34:28.8679391Z 
2019-07-17T00:34:28.8679465Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:28.8679519Z Actual stderr saved to /tmp/compiletest9Cm2qq/match_slice.stderr
2019-07-17T00:34:28.8679572Z To update references, run this command from build directory:
2019-07-17T00:34:28.8679890Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'match_slice.rs'
2019-07-17T00:34:28.8679974Z error: 1 errors occurred comparing output.
2019-07-17T00:34:28.8680020Z status: exit code: 1
2019-07-17T00:34:28.8680020Z status: exit code: 1
2019-07-17T00:34:28.8680718Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/match_slice.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/match_slice.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/match_slice.stage-id.aux" "-A" "unused"
2019-07-17T00:34:28.8681062Z ------------------------------------------
2019-07-17T00:34:28.8681090Z 
2019-07-17T00:34:28.8681265Z ------------------------------------------
2019-07-17T00:34:28.8681318Z stderr:
---
2019-07-17T00:34:29.0761727Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:29.0761797Z +
2019-07-17T00:34:29.0763425Z 
2019-07-17T00:34:29.0763475Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:29.0763515Z Actual stderr saved to /tmp/compiletest9Cm2qq/memchr.stderr
2019-07-17T00:34:29.0763567Z To update references, run this command from build directory:
2019-07-17T00:34:29.0763915Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'memchr.rs'
2019-07-17T00:34:29.0764038Z error: 1 errors occurred comparing output.
2019-07-17T00:34:29.0764325Z 
2019-07-17T00:34:29.0764373Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:29.0764373Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:29.0764414Z Actual stderr saved to /tmp/compiletest9Cm2qq/mir_coercions.stderr
2019-07-17T00:34:29.0764472Z To update references, run this command from build directory:
2019-07-17T00:34:29.0764741Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'mir_coercions.rs'
2019-07-17T00:34:29.0764813Z error: 1 errors occurred comparing output.
2019-07-17T00:34:29.0765345Z thread '[ui] run-pass/mir_coercions.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
2019-07-17T00:34:29.0765533Z status: exit code: 1
2019-07-17T00:34:29.0765533Z status: exit code: 1
2019-07-17T00:34:29.0766254Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/mir_coercions.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/mir_coercions.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/mir_coercions.stage-id.aux" "-A" "unused"
2019-07-17T00:34:29.0767023Z ------------------------------------------
2019-07-17T00:34:29.0767078Z 
2019-07-17T00:34:29.0767302Z ------------------------------------------
2019-07-17T00:34:29.0767348Z stderr:
---
2019-07-17T00:34:29.0790588Z 
2019-07-17T00:34:29.0790766Z test [ui] run-pass/mir_coercions.rs ... FAILED
2019-07-17T00:34:29.0791027Z thread '[ui] run-pass/memchr.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
2019-07-17T00:34:29.0791087Z status: exit code: 1
2019-07-17T00:34:29.0791634Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/memchr.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/memchr.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/memchr.stage-id.aux" "-A" "unused"
2019-07-17T00:34:29.0791916Z ------------------------------------------
2019-07-17T00:34:29.0791943Z 
2019-07-17T00:34:29.0792112Z ------------------------------------------
2019-07-17T00:34:29.0792170Z stderr:
---
2019-07-17T00:34:29.2127588Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:29.2127900Z +
2019-07-17T00:34:29.2128084Z 
2019-07-17T00:34:29.2128291Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:29.2128817Z Actual stderr saved to /tmp/compiletest9Cm2qq/miri-issue-133.stderr
2019-07-17T00:34:29.2129099Z To update references, run this command from build directory:
2019-07-17T00:34:29.2129604Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'miri-issue-133.rs'
2019-07-17T00:34:29.2130073Z error: 1 errors occurred comparing output.
2019-07-17T00:34:29.2130420Z status: exit code: 1
2019-07-17T00:34:29.2130420Z status: exit code: 1
2019-07-17T00:34:29.2131382Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/miri-issue-133.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/miri-issue-133.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/miri-issue-133.stage-id.aux" "-A" "unused"
2019-07-17T00:34:29.2132278Z ------------------------------------------
2019-07-17T00:34:29.2132492Z 
2019-07-17T00:34:29.2132878Z ------------------------------------------
2019-07-17T00:34:29.2133104Z stderr:
---
2019-07-17T00:34:29.2320156Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:29.2323131Z +
2019-07-17T00:34:29.2324111Z thread '[ui] run-pass/mir_fat_ptr.rs
2019-07-17T00:34:29.2324772Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:29.2324938Z Actual stderr saved to /tmp/compiletest9Cm2qq/mir_fat_ptr.stderr
2019-07-17T00:34:29.2325085Z To update references, run this command from build directory:
2019-07-17T00:34:29.2325493Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'mir_fat_ptr.rs'
2019-07-17T00:34:29.2330109Z error: 1 errors occurred comparing output.
2019-07-17T00:34:29.2330288Z status: exit code: 1
2019-07-17T00:34:29.2330288Z status: exit code: 1
2019-07-17T00:34:29.2331394Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/mir_fat_ptr.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/mir_fat_ptr.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/mir_fat_ptr.stage-id.aux" "-A" "unused"
2019-07-17T00:34:29.2331991Z ------------------------------------------
2019-07-17T00:34:29.2332125Z 
2019-07-17T00:34:29.2332417Z ------------------------------------------
2019-07-17T00:34:29.2332558Z stderr:
---
2019-07-17T00:34:29.3441342Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:29.3441381Z +
2019-07-17T00:34:29.3441403Z 
2019-07-17T00:34:29.3441440Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:29.3441675Z Actual stderr saved to /tmp/compiletest9Cm2qq/move-arg-2-unique.stderr
2019-07-17T00:34:29.3441719Z To update references, run this command from build directory:
2019-07-17T00:34:29.3441938Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'move-arg-2-unique.rs'
2019-07-17T00:34:29.3442019Z error: 1 errors occurred comparing output.
2019-07-17T00:34:29.3442056Z status: exit code: 1
2019-07-17T00:34:29.3442056Z status: exit code: 1
2019-07-17T00:34:29.3442677Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/move-arg-2-unique.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/move-arg-2-unique.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/move-arg-2-unique.stage-id.aux" "-A" "unused"
2019-07-17T00:34:29.3442987Z ------------------------------------------
2019-07-17T00:34:29.3443016Z 
2019-07-17T00:34:29.3443203Z ------------------------------------------
2019-07-17T00:34:29.3443239Z stderr:
---
2019-07-17T00:34:29.3951301Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:29.3951458Z +
2019-07-17T00:34:29.3951576Z 
2019-07-17T00:34:29.3951691Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:29.3952029Z Actual stderr saved to /tmp/compiletest9Cm2qq/move-arg-3-unique.stderr
2019-07-17T00:34:29.3952204Z To update references, run this command from build directory:
2019-07-17T00:34:29.3952548Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'move-arg-3-unique.rs'
2019-07-17T00:34:29.3952817Z error: 1 errors occurred comparing output.
2019-07-17T00:34:29.3953035Z status: exit code: 1
2019-07-17T00:34:29.3953035Z status: exit code: 1
2019-07-17T00:34:29.3953757Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/move-arg-3-unique.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/move-arg-3-unique.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/move-arg-3-unique.stage-id.aux" "-A" "unused"
2019-07-17T00:34:29.3954281Z ------------------------------------------
2019-07-17T00:34:29.3954434Z 
2019-07-17T00:34:29.3954746Z ------------------------------------------
2019-07-17T00:34:29.3954913Z stderr:
---
2019-07-17T00:34:29.4671299Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:29.4671338Z +
2019-07-17T00:34:29.4671360Z 
2019-07-17T00:34:29.4671417Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:29.4671633Z Actual stderr saved to /tmp/compiletest9Cm2qq/move-undef-primval.stderr
2019-07-17T00:34:29.4671676Z To update references, run this command from build directory:
2019-07-17T00:34:29.4671918Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'move-undef-primval.rs'
2019-07-17T00:34:29.4672060Z error: 1 errors occurred comparing output.
2019-07-17T00:34:29.4672117Z status: exit code: 1
2019-07-17T00:34:29.4672117Z status: exit code: 1
2019-07-17T00:34:29.4672683Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/move-undef-primval.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/move-undef-primval.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/move-undef-primval.stage-id.aux" "-A" "unused"
2019-07-17T00:34:29.4672967Z ------------------------------------------
2019-07-17T00:34:29.4672995Z 
2019-07-17T00:34:29.4673178Z ------------------------------------------
2019-07-17T00:34:29.4673235Z stderr:
---
2019-07-17T00:34:29.5725290Z +
2019-07-17T00:34:29.5725574Z thread '[ui] run-pass/mpsc.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
2019-07-17T00:34:29.5725621Z 
2019-07-17T00:34:29.5725744Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:29.5725792Z Actual stderr saved to /tmp/compiletest9Cm2qq/mpsc.stderr
2019-07-17T00:34:29.5725838Z To update references, run this command from build directory:
2019-07-17T00:34:29.5726261Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'mpsc.rs'
2019-07-17T00:34:29.5726884Z error: 1 errors occurred comparing output.
2019-07-17T00:34:29.5726940Z status: exit code: 1
2019-07-17T00:34:29.5726940Z status: exit code: 1
2019-07-17T00:34:29.5727607Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/mpsc.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/mpsc.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/mpsc.stage-id.aux" "-A" "unused"
2019-07-17T00:34:29.5727928Z ------------------------------------------
2019-07-17T00:34:29.5727963Z 
2019-07-17T00:34:29.5728194Z ------------------------------------------
2019-07-17T00:34:29.5728241Z stderr:
---
2019-07-17T00:34:29.6013772Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:29.6013925Z +
2019-07-17T00:34:29.6013971Z 
2019-07-17T00:34:29.6014169Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:29.6014222Z Actual stderr saved to /tmp/compiletest9Cm2qq/multi_arg_closure.stderr
2019-07-17T00:34:29.6014374Z To update references, run this command from build directory:
2019-07-17T00:34:29.6014747Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'multi_arg_closure.rs'
2019-07-17T00:34:29.6014938Z error: 1 errors occurred comparing output.
2019-07-17T00:34:29.6015020Z status: exit code: 1
2019-07-17T00:34:29.6015020Z status: exit code: 1
2019-07-17T00:34:29.6015622Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/multi_arg_closure.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/multi_arg_closure.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/multi_arg_closure.stage-id.aux" "-A" "unused"
2019-07-17T00:34:29.6016237Z ------------------------------------------
2019-07-17T00:34:29.6016757Z 
2019-07-17T00:34:29.6017131Z ------------------------------------------
2019-07-17T00:34:29.6017488Z stderr:
---
2019-07-17T00:34:29.7044213Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:29.7044392Z +
2019-07-17T00:34:29.7044484Z 
2019-07-17T00:34:29.7044592Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:29.7044849Z Actual stderr saved to /tmp/compiletest9Cm2qq/negative_discriminant.stderr
2019-07-17T00:34:29.7045035Z To update references, run this command from build directory:
2019-07-17T00:34:29.7045403Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'negative_discriminant.rs'
2019-07-17T00:34:29.7045670Z error: 1 errors occurred comparing output.
2019-07-17T00:34:29.7045797Z status: exit code: 1
2019-07-17T00:34:29.7045797Z status: exit code: 1
2019-07-17T00:34:29.7047419Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/negative_discriminant.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/negative_discriminant.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/negative_discriminant.stage-id.aux" "-A" "unused"
2019-07-17T00:34:29.7050967Z ------------------------------------------
2019-07-17T00:34:29.7051106Z 
2019-07-17T00:34:29.7051388Z ------------------------------------------
2019-07-17T00:34:29.7051543Z stderr:
---
2019-07-17T00:34:29.7497418Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:29.7497648Z +
2019-07-17T00:34:29.7497765Z 
2019-07-17T00:34:29.7497906Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:29.7498071Z Actual stderr saved to /tmp/compiletest9Cm2qq/non_capture_closure_to_fn_ptr.stderr
2019-07-17T00:34:29.7498213Z To update references, run this command from build directory:
2019-07-17T00:34:29.7498648Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'non_capture_closure_to_fn_ptr.rs'
2019-07-17T00:34:29.7498955Z error: 1 errors occurred comparing output.
2019-07-17T00:34:29.7499090Z status: exit code: 1
2019-07-17T00:34:29.7499090Z status: exit code: 1
2019-07-17T00:34:29.7500054Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/non_capture_closure_to_fn_ptr.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/non_capture_closure_to_fn_ptr.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/non_capture_closure_to_fn_ptr.stage-id.aux" "-A" "unused"
2019-07-17T00:34:29.7500888Z ------------------------------------------
2019-07-17T00:34:29.7501644Z 
2019-07-17T00:34:29.7501906Z ------------------------------------------
2019-07-17T00:34:29.7501945Z stderr:
---
2019-07-17T00:34:29.8360719Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:29.8360772Z +
2019-07-17T00:34:29.8360793Z 
2019-07-17T00:34:29.8360828Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:29.8360867Z Actual stderr saved to /tmp/compiletest9Cm2qq/observed_local_mut.stderr
2019-07-17T00:34:29.8360926Z To update references, run this command from build directory:
2019-07-17T00:34:29.8361134Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'observed_local_mut.rs'
2019-07-17T00:34:29.8361213Z error: 1 errors occurred comparing output.
2019-07-17T00:34:29.8361249Z status: exit code: 1
2019-07-17T00:34:29.8361249Z status: exit code: 1
2019-07-17T00:34:29.8361794Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/observed_local_mut.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/observed_local_mut.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-Zmiri-disable-validation" "-L" "/tmp/compiletest9Cm2qq/observed_local_mut.stage-id.aux" "-A" "unused"
2019-07-17T00:34:29.8362138Z ------------------------------------------
2019-07-17T00:34:29.8362165Z 
2019-07-17T00:34:29.8362353Z ------------------------------------------
2019-07-17T00:34:29.8362388Z stderr:
---
2019-07-17T00:34:29.8840374Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:29.8840429Z +
2019-07-17T00:34:29.8840452Z 
2019-07-17T00:34:29.8840489Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:29.8840532Z Actual stderr saved to /tmp/compiletest9Cm2qq/option_box_transmute_ptr.stderr
2019-07-17T00:34:29.8840590Z To update references, run this command from build directory:
2019-07-17T00:34:29.8840820Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'option_box_transmute_ptr.rs'
2019-07-17T00:34:29.8840889Z error: 1 errors occurred comparing output.
2019-07-17T00:34:29.8840941Z status: exit code: 1
2019-07-17T00:34:29.8840941Z status: exit code: 1
2019-07-17T00:34:29.8841494Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/option_box_transmute_ptr.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/option_box_transmute_ptr.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/option_box_transmute_ptr.stage-id.aux" "-A" "unused"
2019-07-17T00:34:29.8841857Z ------------------------------------------
2019-07-17T00:34:29.8841885Z 
2019-07-17T00:34:29.8842080Z ------------------------------------------
2019-07-17T00:34:29.8842119Z stderr:
---
2019-07-17T00:34:29.9905397Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:29.9905440Z +
2019-07-17T00:34:29.9905463Z 
2019-07-17T00:34:29.9905522Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:29.9905567Z Actual stderr saved to /tmp/compiletest9Cm2qq/option_eq.stderr
2019-07-17T00:34:29.9905617Z To update references, run this command from build directory:
2019-07-17T00:34:29.9905871Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'option_eq.rs'
2019-07-17T00:34:29.9905939Z error: 1 errors occurred comparing output.
2019-07-17T00:34:29.9906241Z status: exit code: 1
2019-07-17T00:34:29.9906241Z status: exit code: 1
2019-07-17T00:34:29.9907456Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/option_eq.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/option_eq.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/option_eq.stage-id.aux" "-A" "unused"
2019-07-17T00:34:29.9907802Z ------------------------------------------
2019-07-17T00:34:29.9907837Z 
2019-07-17T00:34:29.9908059Z ------------------------------------------
2019-07-17T00:34:29.9908135Z stderr:
---
2019-07-17T00:34:30.0370419Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:30.0370457Z +
2019-07-17T00:34:30.0370479Z 
2019-07-17T00:34:30.0370529Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:30.0370740Z Actual stderr saved to /tmp/compiletest9Cm2qq/overloaded-calls-simple.stderr
2019-07-17T00:34:30.0370784Z To update references, run this command from build directory:
2019-07-17T00:34:30.0371015Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'overloaded-calls-simple.rs'
2019-07-17T00:34:30.0371079Z error: 1 errors occurred comparing output.
2019-07-17T00:34:30.0371129Z status: exit code: 1
2019-07-17T00:34:30.0371129Z status: exit code: 1
2019-07-17T00:34:30.0371752Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/overloaded-calls-simple.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/overloaded-calls-simple.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/overloaded-calls-simple.stage-id.aux" "-A" "unused"
2019-07-17T00:34:30.0372010Z ------------------------------------------
2019-07-17T00:34:30.0378681Z thread '[ui] run-pass/overloaded-calls-simple.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
2019-07-17T00:34:30.0378819Z 
2019-07-17T00:34:30.0379136Z ------------------------------------------
---
2019-07-17T00:34:30.1240785Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:30.1240840Z +
2019-07-17T00:34:30.1240863Z 
2019-07-17T00:34:30.1240900Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:30.1240959Z Actual stderr saved to /tmp/compiletest9Cm2qq/packed_static.stderr
2019-07-17T00:34:30.1241007Z To update references, run this command from build directory:
2019-07-17T00:34:30.1241237Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'packed_static.rs'
2019-07-17T00:34:30.1241395Z error: 1 errors occurred comparing output.
2019-07-17T00:34:30.1241432Z status: exit code: 1
2019-07-17T00:34:30.1241432Z status: exit code: 1
2019-07-17T00:34:30.1241969Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/packed_static.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/packed_static.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/packed_static.stage-id.aux" "-A" "unused"
2019-07-17T00:34:30.1242246Z ------------------------------------------
2019-07-17T00:34:30.1242290Z 
2019-07-17T00:34:30.1242490Z ------------------------------------------
2019-07-17T00:34:30.1242528Z stderr:
---
2019-07-17T00:34:30.2248939Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:30.2249042Z +
2019-07-17T00:34:30.2249071Z 
2019-07-17T00:34:30.2249137Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:30.2249191Z Actual stderr saved to /tmp/compiletest9Cm2qq/packed_struct.stderr
2019-07-17T00:34:30.2249409Z To update references, run this command from build directory:
2019-07-17T00:34:30.2249736Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'packed_struct.rs'
2019-07-17T00:34:30.2249843Z error: 1 errors occurred comparing output.
2019-07-17T00:34:30.2249890Z status: exit code: 1
2019-07-17T00:34:30.2249890Z status: exit code: 1
2019-07-17T00:34:30.2250800Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/packed_struct.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/packed_struct.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/packed_struct.stage-id.aux" "-A" "unused"
2019-07-17T00:34:30.2251069Z ------------------------------------------
2019-07-17T00:34:30.2251104Z 
2019-07-17T00:34:30.2251283Z ------------------------------------------
2019-07-17T00:34:30.2251320Z stderr:
---
2019-07-17T00:34:30.3123287Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:30.3123343Z +
2019-07-17T00:34:30.3123365Z 
2019-07-17T00:34:30.3123465Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:30.3123505Z Actual stderr saved to /tmp/compiletest9Cm2qq/pointers.stderr
2019-07-17T00:34:30.3123563Z To update references, run this command from build directory:
2019-07-17T00:34:30.3123801Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'pointers.rs'
2019-07-17T00:34:30.3123883Z error: 1 errors occurred comparing output.
2019-07-17T00:34:30.3123920Z status: exit code: 1
2019-07-17T00:34:30.3123920Z status: exit code: 1
2019-07-17T00:34:30.3124434Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/pointers.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/pointers.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/pointers.stage-id.aux" "-A" "unused"
2019-07-17T00:34:30.3124713Z ------------------------------------------
2019-07-17T00:34:30.3124741Z 
2019-07-17T00:34:30.3124946Z ------------------------------------------
2019-07-17T00:34:30.3124984Z stderr:
---
2019-07-17T00:34:30.3885736Z +
2019-07-17T00:34:30.3886051Z thread '[ui] run-pass/products.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
2019-07-17T00:34:30.3886354Z 
2019-07-17T00:34:30.3886804Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:30.3886873Z Actual stderr saved to /tmp/compiletest9Cm2qq/products.stderr
2019-07-17T00:34:30.3886925Z To update references, run this command from build directory:
2019-07-17T00:34:30.3887279Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'products.rs'
2019-07-17T00:34:30.3887379Z error: 1 errors occurred comparing output.
2019-07-17T00:34:30.3887425Z status: exit code: 1
2019-07-17T00:34:30.3887425Z status: exit code: 1
2019-07-17T00:34:30.3888093Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/products.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/products.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/products.stage-id.aux" "-A" "unused"
2019-07-17T00:34:30.3888432Z ------------------------------------------
2019-07-17T00:34:30.3888466Z 
2019-07-17T00:34:30.3888687Z ------------------------------------------
2019-07-17T00:34:30.3888734Z stderr:
---
2019-07-17T00:34:30.4401892Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:30.4401933Z +
2019-07-17T00:34:30.4401955Z 
2019-07-17T00:34:30.4402008Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:30.4402123Z Actual stderr saved to /tmp/compiletest9Cm2qq/ptr_arith_offset.stderr
2019-07-17T00:34:30.4402164Z To update references, run this command from build directory:
2019-07-17T00:34:30.4402433Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'ptr_arith_offset.rs'
2019-07-17T00:34:30.4402498Z error: 1 errors occurred comparing output.
2019-07-17T00:34:30.4402551Z status: exit code: 1
2019-07-17T00:34:30.4402551Z status: exit code: 1
2019-07-17T00:34:30.4403089Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/ptr_arith_offset.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/ptr_arith_offset.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/ptr_arith_offset.stage-id.aux" "-A" "unused"
2019-07-17T00:34:30.4403374Z ------------------------------------------
2019-07-17T00:34:30.4403402Z 
2019-07-17T00:34:30.4403592Z ------------------------------------------
2019-07-17T00:34:30.4403644Z stderr:
---
2019-07-17T00:34:30.5287010Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:30.5287087Z +
2019-07-17T00:34:30.5287116Z 
2019-07-17T00:34:30.5287163Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:30.5287217Z Actual stderr saved to /tmp/compiletest9Cm2qq/ptr_arith_offset_overflow.stderr
2019-07-17T00:34:30.5287286Z To update references, run this command from build directory:
2019-07-17T00:34:30.5287594Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'ptr_arith_offset_overflow.rs'
2019-07-17T00:34:30.5287695Z error: 1 errors occurred comparing output.
2019-07-17T00:34:30.5287742Z status: exit code: 1
2019-07-17T00:34:30.5287742Z status: exit code: 1
2019-07-17T00:34:30.5288439Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/ptr_arith_offset_overflow.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/ptr_arith_offset_overflow.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/ptr_arith_offset_overflow.stage-id.aux" "-A" "unused"
2019-07-17T00:34:30.5288782Z ------------------------------------------
2019-07-17T00:34:30.5288816Z 
2019-07-17T00:34:30.5289056Z ------------------------------------------
2019-07-17T00:34:30.5289103Z stderr:
---
2019-07-17T00:34:30.5991342Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:30.5991510Z +
2019-07-17T00:34:30.5991683Z 
2019-07-17T00:34:30.5991806Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:30.5991938Z Actual stderr saved to /tmp/compiletest9Cm2qq/ptr_int_casts.stderr
2019-07-17T00:34:30.5992073Z To update references, run this command from build directory:
2019-07-17T00:34:30.5992415Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'ptr_int_casts.rs'
2019-07-17T00:34:30.5992687Z error: 1 errors occurred comparing output.
2019-07-17T00:34:30.5999425Z status: exit code: 1
2019-07-17T00:34:30.5999425Z status: exit code: 1
2019-07-17T00:34:30.6001002Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/ptr_int_casts.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/ptr_int_casts.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/ptr_int_casts.stage-id.aux" "-A" "unused"
2019-07-17T00:34:30.6001808Z ------------------------------------------
2019-07-17T00:34:30.6001975Z 
2019-07-17T00:34:30.6002506Z ------------------------------------------
2019-07-17T00:34:30.6002700Z stderr:
---
2019-07-17T00:34:30.6918387Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:30.6918461Z +
2019-07-17T00:34:30.6918491Z 
2019-07-17T00:34:30.6918541Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:30.6918596Z Actual stderr saved to /tmp/compiletest9Cm2qq/ptr_int_ops.stderr
2019-07-17T00:34:30.6918668Z To update references, run this command from build directory:
2019-07-17T00:34:30.6918977Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'ptr_int_ops.rs'
2019-07-17T00:34:30.6919082Z error: 1 errors occurred comparing output.
2019-07-17T00:34:30.6919140Z status: exit code: 1
2019-07-17T00:34:30.6919140Z status: exit code: 1
2019-07-17T00:34:30.6919821Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/ptr_int_ops.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/ptr_int_ops.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/ptr_int_ops.stage-id.aux" "-A" "unused"
2019-07-17T00:34:30.6920186Z ------------------------------------------
2019-07-17T00:34:30.6920223Z 
2019-07-17T00:34:30.6920499Z ------------------------------------------
2019-07-17T00:34:30.6920547Z stderr:
---
2019-07-17T00:34:30.7465275Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:30.7465315Z +
2019-07-17T00:34:30.7465336Z 
2019-07-17T00:34:30.7465391Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:30.7465432Z Actual stderr saved to /tmp/compiletest9Cm2qq/ptr_offset.stderr
2019-07-17T00:34:30.7465483Z To update references, run this command from build directory:
2019-07-17T00:34:30.7465717Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'ptr_offset.rs'
2019-07-17T00:34:30.7465783Z error: 1 errors occurred comparing output.
2019-07-17T00:34:30.7465827Z status: exit code: 1
2019-07-17T00:34:30.7465827Z status: exit code: 1
2019-07-17T00:34:30.7467238Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/ptr_offset.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/ptr_offset.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/ptr_offset.stage-id.aux" "-A" "unused"
2019-07-17T00:34:30.7467692Z ------------------------------------------
2019-07-17T00:34:30.7467728Z 
2019-07-17T00:34:30.7467954Z ------------------------------------------
2019-07-17T00:34:30.7468020Z stderr:
---
2019-07-17T00:34:30.8523750Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:30.8523789Z +
2019-07-17T00:34:30.8523813Z 
2019-07-17T00:34:30.8523868Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:30.8523917Z Actual stderr saved to /tmp/compiletest9Cm2qq/raw.stderr
2019-07-17T00:34:30.8523957Z To update references, run this command from build directory:
2019-07-17T00:34:30.8524197Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'raw.rs'
2019-07-17T00:34:30.8524271Z error: 1 errors occurred comparing output.
2019-07-17T00:34:30.8524308Z status: exit code: 1
2019-07-17T00:34:30.8524308Z status: exit code: 1
2019-07-17T00:34:30.8524827Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/raw.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/raw.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/raw.stage-id.aux" "-A" "unused"
2019-07-17T00:34:30.8525097Z ------------------------------------------
2019-07-17T00:34:30.8525125Z 
2019-07-17T00:34:30.8525317Z ------------------------------------------
2019-07-17T00:34:30.8525370Z stderr:
---
2019-07-17T00:34:30.9684038Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:30.9684098Z +
2019-07-17T00:34:30.9684121Z 
2019-07-17T00:34:30.9684162Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:30.9684215Z Actual stderr saved to /tmp/compiletest9Cm2qq/rc.stderr
2019-07-17T00:34:30.9684276Z To update references, run this command from build directory:
2019-07-17T00:34:30.9684513Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'rc.rs'
2019-07-17T00:34:30.9684607Z error: 1 errors occurred comparing output.
2019-07-17T00:34:30.9684648Z status: exit code: 1
2019-07-17T00:34:30.9684648Z status: exit code: 1
2019-07-17T00:34:30.9685196Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/rc.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/rc.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/rc.stage-id.aux" "-A" "unused"
2019-07-17T00:34:30.9685492Z ------------------------------------------
2019-07-17T00:34:30.9685524Z 
2019-07-17T00:34:30.9685752Z ------------------------------------------
2019-07-17T00:34:30.9685793Z stderr:
---
2019-07-17T00:34:30.9968641Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:30.9968690Z +
2019-07-17T00:34:30.9968719Z 
2019-07-17T00:34:30.9968776Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:30.9968846Z Actual stderr saved to /tmp/compiletest9Cm2qq/recursive_static.stderr
2019-07-17T00:34:30.9968899Z To update references, run this command from build directory:
2019-07-17T00:34:30.9969174Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'recursive_static.rs'
2019-07-17T00:34:30.9969281Z error: 1 errors occurred comparing output.
2019-07-17T00:34:30.9969326Z status: exit code: 1
2019-07-17T00:34:30.9969326Z status: exit code: 1
2019-07-17T00:34:30.9970012Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/recursive_static.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/recursive_static.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/recursive_static.stage-id.aux" "-A" "unused"
2019-07-17T00:34:30.9970503Z ------------------------------------------
2019-07-17T00:34:30.9970535Z 
2019-07-17T00:34:30.9970726Z ------------------------------------------
2019-07-17T00:34:30.9970767Z stderr:
---
2019-07-17T00:34:31.0923770Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:31.0923820Z +
2019-07-17T00:34:31.0923847Z 
2019-07-17T00:34:31.0923911Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:31.0924172Z Actual stderr saved to /tmp/compiletest9Cm2qq/ref-invalid-ptr.stderr
2019-07-17T00:34:31.0924228Z To update references, run this command from build directory:
2019-07-17T00:34:31.0924514Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'ref-invalid-ptr.rs'
2019-07-17T00:34:31.0924602Z error: 1 errors occurred comparing output.
2019-07-17T00:34:31.0924667Z status: exit code: 1
2019-07-17T00:34:31.0924667Z status: exit code: 1
2019-07-17T00:34:31.0925369Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/ref-invalid-ptr.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/ref-invalid-ptr.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-Zmiri-disable-validation" "-L" "/tmp/compiletest9Cm2qq/ref-invalid-ptr.stage-id.aux" "-A" "unused"
2019-07-17T00:34:31.0925706Z ------------------------------------------
2019-07-17T00:34:31.0925740Z 
2019-07-17T00:34:31.0925962Z ------------------------------------------
2019-07-17T00:34:31.0926028Z stderr:
---
2019-07-17T00:34:31.1412106Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:31.1412357Z +
2019-07-17T00:34:31.1412538Z 
2019-07-17T00:34:31.1412718Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:31.1412897Z Actual stderr saved to /tmp/compiletest9Cm2qq/refcell.stderr
2019-07-17T00:34:31.1413089Z To update references, run this command from build directory:
2019-07-17T00:34:31.1413512Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'refcell.rs'
2019-07-17T00:34:31.1413926Z error: 1 errors occurred comparing output.
2019-07-17T00:34:31.1414096Z status: exit code: 1
2019-07-17T00:34:31.1414096Z status: exit code: 1
2019-07-17T00:34:31.1414848Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/refcell.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/refcell.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/refcell.stage-id.aux" "-A" "unused"
2019-07-17T00:34:31.1415925Z ------------------------------------------
2019-07-17T00:34:31.1416127Z 
2019-07-17T00:34:31.1417242Z ------------------------------------------
2019-07-17T00:34:31.1417461Z stderr:
---
2019-07-17T00:34:31.2280544Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:31.2280610Z +
2019-07-17T00:34:31.2280634Z 
2019-07-17T00:34:31.2280672Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:31.2280897Z Actual stderr saved to /tmp/compiletest9Cm2qq/regions-lifetime-nonfree-late-bound.stderr
2019-07-17T00:34:31.2280970Z To update references, run this command from build directory:
2019-07-17T00:34:31.2281205Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'regions-lifetime-nonfree-late-bound.rs'
2019-07-17T00:34:31.2281291Z error: 1 errors occurred comparing output.
2019-07-17T00:34:31.2281329Z status: exit code: 1
2019-07-17T00:34:31.2281329Z status: exit code: 1
2019-07-17T00:34:31.2281918Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/regions-lifetime-nonfree-late-bound.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/regions-lifetime-nonfree-late-bound.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/regions-lifetime-nonfree-late-bound.stage-id.aux" "-A" "unused"
2019-07-17T00:34:31.2282201Z ------------------------------------------
2019-07-17T00:34:31.2282249Z 
2019-07-17T00:34:31.2282433Z ------------------------------------------
2019-07-17T00:34:31.2282470Z stderr:
---
2019-07-17T00:34:31.2795728Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:31.2795776Z +
2019-07-17T00:34:31.2795799Z 
2019-07-17T00:34:31.2795837Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:31.2796064Z Actual stderr saved to /tmp/compiletest9Cm2qq/regions-mock-trans.stderr
2019-07-17T00:34:31.2796108Z To update references, run this command from build directory:
2019-07-17T00:34:31.2797093Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'regions-mock-trans.rs'
2019-07-17T00:34:31.2797207Z error: 1 errors occurred comparing output.
2019-07-17T00:34:31.2797254Z status: exit code: 1
2019-07-17T00:34:31.2797254Z status: exit code: 1
2019-07-17T00:34:31.2797989Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/regions-mock-trans.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/regions-mock-trans.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/regions-mock-trans.stage-id.aux" "-A" "unused"
2019-07-17T00:34:31.2798317Z ------------------------------------------
2019-07-17T00:34:31.2798372Z 
2019-07-17T00:34:31.2798597Z ------------------------------------------
2019-07-17T00:34:31.2798645Z stderr:
---
2019-07-17T00:34:31.3652946Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:31.3652992Z +
2019-07-17T00:34:31.3653016Z 
2019-07-17T00:34:31.3653056Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:31.3653117Z Actual stderr saved to /tmp/compiletest9Cm2qq/rfc1623.stderr
2019-07-17T00:34:31.3653159Z To update references, run this command from build directory:
2019-07-17T00:34:31.3653411Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'rfc1623.rs'
2019-07-17T00:34:31.3653496Z error: 1 errors occurred comparing output.
2019-07-17T00:34:31.3653534Z status: exit code: 1
2019-07-17T00:34:31.3653534Z status: exit code: 1
2019-07-17T00:34:31.3654111Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/rfc1623.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/rfc1623.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/rfc1623.stage-id.aux" "-A" "unused"
2019-07-17T00:34:31.3654389Z ------------------------------------------
2019-07-17T00:34:31.3654435Z 
2019-07-17T00:34:31.3654637Z ------------------------------------------
2019-07-17T00:34:31.3654675Z stderr:
---
2019-07-17T00:34:31.4350287Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:31.4350328Z +
2019-07-17T00:34:31.4350365Z 
2019-07-17T00:34:31.4350403Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:31.4350611Z Actual stderr saved to /tmp/compiletest9Cm2qq/rust-lang-org.stderr
2019-07-17T00:34:31.4350670Z To update references, run this command from build directory:
2019-07-17T00:34:31.4350885Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'rust-lang-org.rs'
2019-07-17T00:34:31.4350951Z error: 1 errors occurred comparing output.
2019-07-17T00:34:31.4351004Z status: exit code: 1
2019-07-17T00:34:31.4351004Z status: exit code: 1
2019-07-17T00:34:31.4351539Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/rust-lang-org.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/rust-lang-org.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/rust-lang-org.stage-id.aux" "-A" "unused"
2019-07-17T00:34:31.4351812Z ------------------------------------------
2019-07-17T00:34:31.4351840Z 
2019-07-17T00:34:31.4352036Z ------------------------------------------
2019-07-17T00:34:31.4352074Z stderr:
---
2019-07-17T00:34:31.5325653Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:31.5325694Z +
2019-07-17T00:34:31.5325717Z 
2019-07-17T00:34:31.5325772Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:31.5326007Z Actual stderr saved to /tmp/compiletest9Cm2qq/send-is-not-static-par-for.stderr
2019-07-17T00:34:31.5326051Z To update references, run this command from build directory:
2019-07-17T00:34:31.5327019Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'send-is-not-static-par-for.rs'
2019-07-17T00:34:31.5327135Z error: 1 errors occurred comparing output.
2019-07-17T00:34:31.5327202Z status: exit code: 1
2019-07-17T00:34:31.5327202Z status: exit code: 1
2019-07-17T00:34:31.5328062Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/send-is-not-static-par-for.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/send-is-not-static-par-for.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/send-is-not-static-par-for.stage-id.aux" "-A" "unused"
2019-07-17T00:34:31.5328404Z ------------------------------------------
2019-07-17T00:34:31.5328438Z 
2019-07-17T00:34:31.5328660Z ------------------------------------------
2019-07-17T00:34:31.5328726Z stderr:
---
2019-07-17T00:34:31.5561957Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:31.5561994Z +
2019-07-17T00:34:31.5562031Z 
2019-07-17T00:34:31.5562074Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:31.5562270Z Actual stderr saved to /tmp/compiletest9Cm2qq/sendable-class.stderr
2019-07-17T00:34:31.5562331Z To update references, run this command from build directory:
2019-07-17T00:34:31.5562547Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'sendable-class.rs'
2019-07-17T00:34:31.5562609Z error: 1 errors occurred comparing output.
2019-07-17T00:34:31.5562662Z status: exit code: 1
2019-07-17T00:34:31.5562662Z status: exit code: 1
2019-07-17T00:34:31.5563161Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/sendable-class.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/sendable-class.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/sendable-class.stage-id.aux" "-A" "unused"
2019-07-17T00:34:31.5563474Z ------------------------------------------
2019-07-17T00:34:31.5563507Z 
2019-07-17T00:34:31.5563716Z ------------------------------------------
2019-07-17T00:34:31.5563754Z stderr:
---
2019-07-17T00:34:31.7045017Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:31.7045059Z +
2019-07-17T00:34:31.7045082Z 
2019-07-17T00:34:31.7045136Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:31.7045361Z Actual stderr saved to /tmp/compiletest9Cm2qq/simd-intrinsic-generic-elements.stderr
2019-07-17T00:34:31.7045406Z To update references, run this command from build directory:
2019-07-17T00:34:31.7045654Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'simd-intrinsic-generic-elements.rs'
2019-07-17T00:34:31.7045727Z error: 1 errors occurred comparing output.
2019-07-17T00:34:31.7045763Z status: exit code: 1
2019-07-17T00:34:31.7045763Z status: exit code: 1
2019-07-17T00:34:31.7046519Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/simd-intrinsic-generic-elements.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/simd-intrinsic-generic-elements.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/simd-intrinsic-generic-elements.stage-id.aux" "-A" "unused"
2019-07-17T00:34:31.7047374Z ------------------------------------------
2019-07-17T00:34:31.7047415Z 
2019-07-17T00:34:31.7047663Z ------------------------------------------
2019-07-17T00:34:31.7047727Z stderr:
---
2019-07-17T00:34:31.8385025Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:31.8385085Z +
2019-07-17T00:34:31.8385108Z 
2019-07-17T00:34:31.8385146Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:31.8385190Z Actual stderr saved to /tmp/compiletest9Cm2qq/small_enum_size_bug.stderr
2019-07-17T00:34:31.8385255Z To update references, run this command from build directory:
2019-07-17T00:34:31.8385493Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'small_enum_size_bug.rs'
2019-07-17T00:34:31.8385560Z error: 1 errors occurred comparing output.
2019-07-17T00:34:31.8385612Z status: exit code: 1
2019-07-17T00:34:31.8385612Z status: exit code: 1
2019-07-17T00:34:31.8386805Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/small_enum_size_bug.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/small_enum_size_bug.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/small_enum_size_bug.stage-id.aux" "-A" "unused"
2019-07-17T00:34:31.8387227Z ------------------------------------------
2019-07-17T00:34:31.8387274Z 
2019-07-17T00:34:31.8387519Z ------------------------------------------
2019-07-17T00:34:31.8387566Z stderr:
---
2019-07-17T00:34:31.8761259Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:31.8761301Z +
2019-07-17T00:34:31.8761341Z 
2019-07-17T00:34:31.8761380Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:31.8761422Z Actual stderr saved to /tmp/compiletest9Cm2qq/slices.stderr
2019-07-17T00:34:31.8761480Z To update references, run this command from build directory:
2019-07-17T00:34:31.8761706Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'slices.rs'
2019-07-17T00:34:31.8761772Z error: 1 errors occurred comparing output.
2019-07-17T00:34:31.8761826Z status: exit code: 1
2019-07-17T00:34:31.8761826Z status: exit code: 1
2019-07-17T00:34:31.8762409Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/slices.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/slices.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/slices.stage-id.aux" "-A" "unused"
2019-07-17T00:34:31.8762721Z ------------------------------------------
2019-07-17T00:34:31.8762750Z 
2019-07-17T00:34:31.8762960Z ------------------------------------------
2019-07-17T00:34:31.8762999Z stderr:
---
2019-07-17T00:34:31.9804541Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:31.9804593Z +
2019-07-17T00:34:31.9804615Z 
2019-07-17T00:34:31.9804653Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:31.9804714Z Actual stderr saved to /tmp/compiletest9Cm2qq/specialization.stderr
2019-07-17T00:34:31.9804755Z To update references, run this command from build directory:
2019-07-17T00:34:31.9804999Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'specialization.rs'
2019-07-17T00:34:31.9805083Z error: 1 errors occurred comparing output.
2019-07-17T00:34:31.9805120Z status: exit code: 1
2019-07-17T00:34:31.9805120Z status: exit code: 1
2019-07-17T00:34:31.9805800Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/specialization.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/specialization.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/specialization.stage-id.aux" "-A" "unused"
2019-07-17T00:34:31.9806775Z ------------------------------------------
2019-07-17T00:34:31.9806842Z 
2019-07-17T00:34:31.9807132Z ------------------------------------------
2019-07-17T00:34:31.9807180Z stderr:
---
2019-07-17T00:34:32.0160717Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:32.0160762Z +
2019-07-17T00:34:32.0160784Z 
2019-07-17T00:34:32.0160834Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:32.0161048Z Actual stderr saved to /tmp/compiletest9Cm2qq/stacked-borrows/2phase.stderr
2019-07-17T00:34:32.0161091Z To update references, run this command from build directory:
2019-07-17T00:34:32.0161316Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'stacked-borrows/2phase.rs'
2019-07-17T00:34:32.0161379Z error: 1 errors occurred comparing output.
2019-07-17T00:34:32.0161422Z status: exit code: 1
2019-07-17T00:34:32.0161422Z status: exit code: 1
2019-07-17T00:34:32.0162100Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/stacked-borrows/2phase.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/stacked-borrows/2phase.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/stacked-borrows/2phase.stage-id.aux" "-A" "unused"
2019-07-17T00:34:32.0162398Z ------------------------------------------
2019-07-17T00:34:32.0162426Z 
2019-07-17T00:34:32.0162601Z ------------------------------------------
2019-07-17T00:34:32.0162643Z stderr:
---
2019-07-17T00:34:32.1485115Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:32.1485158Z +
2019-07-17T00:34:32.1485202Z 
2019-07-17T00:34:32.1485241Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:32.1485491Z Actual stderr saved to /tmp/compiletest9Cm2qq/stacked-borrows/interior_mutability.stderr
2019-07-17T00:34:32.1485536Z To update references, run this command from build directory:
2019-07-17T00:34:32.1485803Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'stacked-borrows/interior_mutability.rs'
2019-07-17T00:34:32.1486019Z error: 1 errors occurred comparing output.
2019-07-17T00:34:32.1486087Z status: exit code: 1
2019-07-17T00:34:32.1486087Z status: exit code: 1
2019-07-17T00:34:32.1487650Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/stacked-borrows/interior_mutability.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/stacked-borrows/interior_mutability.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/stacked-borrows/interior_mutability.stage-id.aux" "-A" "unused"
2019-07-17T00:34:32.1488020Z ------------------------------------------
2019-07-17T00:34:32.1488055Z 
2019-07-17T00:34:32.1488296Z ------------------------------------------
2019-07-17T00:34:32.1488343Z stderr:
---
2019-07-17T00:34:32.2402482Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:32.2402538Z +
2019-07-17T00:34:32.2402560Z 
2019-07-17T00:34:32.2402670Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:32.2402923Z Actual stderr saved to /tmp/compiletest9Cm2qq/stacked-borrows/stacked-borrows.stderr
2019-07-17T00:34:32.2402986Z To update references, run this command from build directory:
2019-07-17T00:34:32.2403229Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'stacked-borrows/stacked-borrows.rs'
2019-07-17T00:34:32.2403313Z error: 1 errors occurred comparing output.
2019-07-17T00:34:32.2403353Z status: exit code: 1
2019-07-17T00:34:32.2403353Z status: exit code: 1
2019-07-17T00:34:32.2403919Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/stacked-borrows/stacked-borrows.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/stacked-borrows/stacked-borrows.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/stacked-borrows/stacked-borrows.stage-id.aux" "-A" "unused"
2019-07-17T00:34:32.2404197Z ------------------------------------------
2019-07-17T00:34:32.2404226Z 
2019-07-17T00:34:32.2404519Z ------------------------------------------
2019-07-17T00:34:32.2404557Z stderr:
---
2019-07-17T00:34:32.2968960Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:32.2969009Z +
2019-07-17T00:34:32.2969039Z 
2019-07-17T00:34:32.2969104Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:32.2969168Z Actual stderr saved to /tmp/compiletest9Cm2qq/static_memory_modification.stderr
2019-07-17T00:34:32.2969222Z To update references, run this command from build directory:
2019-07-17T00:34:32.2969523Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'static_memory_modification.rs'
2019-07-17T00:34:32.2969605Z error: 1 errors occurred comparing output.
2019-07-17T00:34:32.2969667Z status: exit code: 1
2019-07-17T00:34:32.2969667Z status: exit code: 1
2019-07-17T00:34:32.2970492Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/static_memory_modification.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/static_memory_modification.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/static_memory_modification.stage-id.aux" "-A" "unused"
2019-07-17T00:34:32.2970852Z ------------------------------------------
2019-07-17T00:34:32.2970878Z 
2019-07-17T00:34:32.2971051Z ------------------------------------------
2019-07-17T00:34:32.2971105Z stderr:
---
2019-07-17T00:34:32.3832614Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:32.3832833Z +
2019-07-17T00:34:32.3832855Z 
2019-07-17T00:34:32.3832891Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:32.3832931Z Actual stderr saved to /tmp/compiletest9Cm2qq/static_mut.stderr
2019-07-17T00:34:32.3832990Z To update references, run this command from build directory:
2019-07-17T00:34:32.3833204Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'static_mut.rs'
2019-07-17T00:34:32.3833283Z error: 1 errors occurred comparing output.
2019-07-17T00:34:32.3833320Z status: exit code: 1
2019-07-17T00:34:32.3833320Z status: exit code: 1
2019-07-17T00:34:32.3833836Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/static_mut.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/static_mut.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/static_mut.stage-id.aux" "-A" "unused"
2019-07-17T00:34:32.3834182Z ------------------------------------------
2019-07-17T00:34:32.3834210Z 
2019-07-17T00:34:32.3834407Z ------------------------------------------
2019-07-17T00:34:32.3834444Z stderr:
---
2019-07-17T00:34:32.4548102Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:32.4548300Z +
2019-07-17T00:34:32.4548455Z 
2019-07-17T00:34:32.4548599Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:32.4548740Z Actual stderr saved to /tmp/compiletest9Cm2qq/strings.stderr
2019-07-17T00:34:32.4548898Z To update references, run this command from build directory:
2019-07-17T00:34:32.4549302Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'strings.rs'
2019-07-17T00:34:32.4549622Z error: 1 errors occurred comparing output.
2019-07-17T00:34:32.4549763Z status: exit code: 1
2019-07-17T00:34:32.4549763Z status: exit code: 1
2019-07-17T00:34:32.4550669Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/strings.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/strings.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/strings.stage-id.aux" "-A" "unused"
2019-07-17T00:34:32.4551334Z ------------------------------------------
2019-07-17T00:34:32.4551490Z 
2019-07-17T00:34:32.4551781Z ------------------------------------------
2019-07-17T00:34:32.4551925Z stderr:
---
2019-07-17T00:34:32.5279574Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:32.5279640Z +
2019-07-17T00:34:32.5279670Z 
2019-07-17T00:34:32.5279737Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:32.5279792Z Actual stderr saved to /tmp/compiletest9Cm2qq/subslice_array.stderr
2019-07-17T00:34:32.5279846Z To update references, run this command from build directory:
2019-07-17T00:34:32.5280304Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'subslice_array.rs'
2019-07-17T00:34:32.5280372Z error: 1 errors occurred comparing output.
2019-07-17T00:34:32.5280410Z status: exit code: 1
2019-07-17T00:34:32.5280410Z status: exit code: 1
2019-07-17T00:34:32.5280985Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/subslice_array.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/subslice_array.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/subslice_array.stage-id.aux" "-A" "unused"
2019-07-17T00:34:32.5281375Z ------------------------------------------
2019-07-17T00:34:32.5281405Z 
2019-07-17T00:34:32.5281589Z ------------------------------------------
2019-07-17T00:34:32.5281644Z stderr:
---
2019-07-17T00:34:32.6304677Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:32.6304737Z +
2019-07-17T00:34:32.6304759Z 
2019-07-17T00:34:32.6304797Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:32.6305014Z Actual stderr saved to /tmp/compiletest9Cm2qq/sums.stderr
2019-07-17T00:34:32.6305076Z To update references, run this command from build directory:
2019-07-17T00:34:32.6305297Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'sums.rs'
2019-07-17T00:34:32.6305382Z error: 1 errors occurred comparing output.
2019-07-17T00:34:32.6305420Z status: exit code: 1
2019-07-17T00:34:32.6305420Z status: exit code: 1
2019-07-17T00:34:32.6305953Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/sums.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/sums.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/sums.stage-id.aux" "-A" "unused"
2019-07-17T00:34:32.6306857Z ------------------------------------------
2019-07-17T00:34:32.6306900Z 
2019-07-17T00:34:32.6307209Z ------------------------------------------
2019-07-17T00:34:32.6307256Z stderr:
---
2019-07-17T00:34:32.6597807Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:32.6597876Z +
2019-07-17T00:34:32.6597906Z 
2019-07-17T00:34:32.6597952Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:32.6598003Z Actual stderr saved to /tmp/compiletest9Cm2qq/sync.stderr
2019-07-17T00:34:32.6598070Z To update references, run this command from build directory:
2019-07-17T00:34:32.6598337Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'sync.rs'
2019-07-17T00:34:32.6598417Z error: 1 errors occurred comparing output.
2019-07-17T00:34:32.6598479Z status: exit code: 1
2019-07-17T00:34:32.6598479Z status: exit code: 1
2019-07-17T00:34:32.6599132Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/sync.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/sync.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/sync.stage-id.aux" "-A" "unused"
2019-07-17T00:34:32.6599589Z ------------------------------------------
2019-07-17T00:34:32.6599623Z 
2019-07-17T00:34:32.6599866Z ------------------------------------------
2019-07-17T00:34:32.6599912Z stderr:
---
2019-07-17T00:34:32.7523418Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:32.7523457Z +
2019-07-17T00:34:32.7523494Z 
2019-07-17T00:34:32.7523531Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:32.7523748Z Actual stderr saved to /tmp/compiletest9Cm2qq/tag-align-dyn-u64.stderr
2019-07-17T00:34:32.7523807Z To update references, run this command from build directory:
2019-07-17T00:34:32.7524040Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'tag-align-dyn-u64.rs'
2019-07-17T00:34:32.7524106Z error: 1 errors occurred comparing output.
2019-07-17T00:34:32.7524157Z status: exit code: 1
2019-07-17T00:34:32.7524157Z status: exit code: 1
2019-07-17T00:34:32.7524700Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/tag-align-dyn-u64.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/tag-align-dyn-u64.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/tag-align-dyn-u64.stage-id.aux" "-A" "unused"
2019-07-17T00:34:32.7525069Z ------------------------------------------
2019-07-17T00:34:32.7525098Z 
2019-07-17T00:34:32.7525308Z ------------------------------------------
2019-07-17T00:34:32.7525347Z stderr:
---
2019-07-17T00:34:32.8368652Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:32.8368704Z +
2019-07-17T00:34:32.8368752Z 
2019-07-17T00:34:32.8368803Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:32.8369084Z Actual stderr saved to /tmp/compiletest9Cm2qq/thread-local.stderr
2019-07-17T00:34:32.8369170Z To update references, run this command from build directory:
2019-07-17T00:34:32.8369462Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'thread-local.rs'
2019-07-17T00:34:32.8369548Z error: 1 errors occurred comparing output.
2019-07-17T00:34:32.8369829Z status: exit code: 1
2019-07-17T00:34:32.8369829Z status: exit code: 1
2019-07-17T00:34:32.8370599Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/thread-local.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/thread-local.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/thread-local.stage-id.aux" "-A" "unused"
2019-07-17T00:34:32.8370871Z ------------------------------------------
2019-07-17T00:34:32.8370899Z 
2019-07-17T00:34:32.8371092Z ------------------------------------------
2019-07-17T00:34:32.8371138Z stderr:
---
2019-07-17T00:34:32.8601162Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:32.8601219Z +
2019-07-17T00:34:32.8601242Z 
2019-07-17T00:34:32.8601279Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:32.8601527Z Actual stderr saved to /tmp/compiletest9Cm2qq/too-large-primval-write-problem.stderr
2019-07-17T00:34:32.8601572Z To update references, run this command from build directory:
2019-07-17T00:34:32.8601818Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'too-large-primval-write-problem.rs'
2019-07-17T00:34:32.8601901Z error: 1 errors occurred comparing output.
2019-07-17T00:34:32.8601939Z status: exit code: 1
2019-07-17T00:34:32.8601939Z status: exit code: 1
2019-07-17T00:34:32.8602617Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/too-large-primval-write-problem.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/too-large-primval-write-problem.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/too-large-primval-write-problem.stage-id.aux" "-A" "unused"
2019-07-17T00:34:32.8602903Z ------------------------------------------
2019-07-17T00:34:32.8602948Z 
2019-07-17T00:34:32.8603140Z ------------------------------------------
2019-07-17T00:34:32.8603185Z stderr:
---
2019-07-17T00:34:32.9852242Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:32.9852470Z +
2019-07-17T00:34:32.9852621Z 
2019-07-17T00:34:32.9852789Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:32.9852991Z Actual stderr saved to /tmp/compiletest9Cm2qq/transmute_fat.stderr
2019-07-17T00:34:32.9853169Z To update references, run this command from build directory:
2019-07-17T00:34:32.9853597Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'transmute_fat.rs'
2019-07-17T00:34:32.9854119Z error: 1 errors occurred comparing output.
2019-07-17T00:34:32.9854303Z status: exit code: 1
2019-07-17T00:34:32.9854303Z status: exit code: 1
2019-07-17T00:34:32.9855144Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/transmute_fat.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/transmute_fat.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-Zmiri-disable-validation" "-L" "/tmp/compiletest9Cm2qq/transmute_fat.stage-id.aux" "-A" "unused"
2019-07-17T00:34:32.9855827Z ------------------------------------------
2019-07-17T00:34:32.9856043Z 
2019-07-17T00:34:32.9856450Z ------------------------------------------
2019-07-17T00:34:32.9856673Z stderr:
---
2019-07-17T00:34:33.0205678Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:33.0205933Z +
2019-07-17T00:34:33.0206107Z 
2019-07-17T00:34:33.0206802Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:33.0207047Z Actual stderr saved to /tmp/compiletest9Cm2qq/traits.stderr
2019-07-17T00:34:33.0207261Z To update references, run this command from build directory:
2019-07-17T00:34:33.0208001Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'traits.rs'
2019-07-17T00:34:33.0208468Z error: 1 errors occurred comparing output.
2019-07-17T00:34:33.0208693Z status: exit code: 1
2019-07-17T00:34:33.0208693Z status: exit code: 1
2019-07-17T00:34:33.0209544Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/traits.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/traits.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/traits.stage-id.aux" "-A" "unused"
2019-07-17T00:34:33.0210737Z ------------------------------------------
2019-07-17T00:34:33.0210941Z 
2019-07-17T00:34:33.0211328Z ------------------------------------------
2019-07-17T00:34:33.0211560Z stderr:
---
2019-07-17T00:34:33.1158815Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:33.1158886Z +
2019-07-17T00:34:33.1158914Z 
2019-07-17T00:34:33.1158962Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:33.1159014Z Actual stderr saved to /tmp/compiletest9Cm2qq/trivial.stderr
2019-07-17T00:34:33.1159167Z To update references, run this command from build directory:
2019-07-17T00:34:33.1159470Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'trivial.rs'
2019-07-17T00:34:33.1159574Z error: 1 errors occurred comparing output.
2019-07-17T00:34:33.1159622Z status: exit code: 1
2019-07-17T00:34:33.1159622Z status: exit code: 1
2019-07-17T00:34:33.1160456Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/trivial.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/trivial.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/trivial.stage-id.aux" "-A" "unused"
2019-07-17T00:34:33.1160745Z ------------------------------------------
2019-07-17T00:34:33.1160773Z 
2019-07-17T00:34:33.1160974Z ------------------------------------------
2019-07-17T00:34:33.1161019Z stderr:
---
2019-07-17T00:34:33.1641680Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:33.1641721Z +
2019-07-17T00:34:33.1641761Z 
2019-07-17T00:34:33.1641799Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:33.1642114Z Actual stderr saved to /tmp/compiletest9Cm2qq/try-operator-custom.stderr
2019-07-17T00:34:33.1642158Z To update references, run this command from build directory:
2019-07-17T00:34:33.1642624Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'try-operator-custom.rs'
2019-07-17T00:34:33.1642692Z error: 1 errors occurred comparing output.
2019-07-17T00:34:33.1642747Z status: exit code: 1
2019-07-17T00:34:33.1642747Z status: exit code: 1
2019-07-17T00:34:33.1643327Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/try-operator-custom.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/try-operator-custom.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/try-operator-custom.stage-id.aux" "-A" "unused"
2019-07-17T00:34:33.1643617Z ------------------------------------------
2019-07-17T00:34:33.1643645Z 
2019-07-17T00:34:33.1643832Z ------------------------------------------
2019-07-17T00:34:33.1643886Z stderr:
---
2019-07-17T00:34:33.2824026Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:33.2824080Z +
2019-07-17T00:34:33.2824102Z 
2019-07-17T00:34:33.2824138Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:33.2824180Z Actual stderr saved to /tmp/compiletest9Cm2qq/tuple_like_enum_variant_constructor.stderr
2019-07-17T00:34:33.2824226Z To update references, run this command from build directory:
2019-07-17T00:34:33.2824463Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'tuple_like_enum_variant_constructor.rs'
2019-07-17T00:34:33.2824532Z error: 1 errors occurred comparing output.
2019-07-17T00:34:33.2824581Z status: exit code: 1
2019-07-17T00:34:33.2824581Z status: exit code: 1
2019-07-17T00:34:33.2825140Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/tuple_like_enum_variant_constructor.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/tuple_like_enum_variant_constructor.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/tuple_like_enum_variant_constructor.stage-id.aux" "-A" "unused"
2019-07-17T00:34:33.2825410Z ------------------------------------------
2019-07-17T00:34:33.2825436Z 
2019-07-17T00:34:33.2825620Z ------------------------------------------
2019-07-17T00:34:33.2825656Z stderr:
---
2019-07-17T00:34:33.3365483Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:33.3365543Z +
2019-07-17T00:34:33.3365566Z 
2019-07-17T00:34:33.3365603Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:33.3365646Z Actual stderr saved to /tmp/compiletest9Cm2qq/tuple_like_enum_variant_constructor_pointer_opt.stderr
2019-07-17T00:34:33.3365704Z To update references, run this command from build directory:
2019-07-17T00:34:33.3365966Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'tuple_like_enum_variant_constructor_pointer_opt.rs'
2019-07-17T00:34:33.3366059Z error: 1 errors occurred comparing output.
2019-07-17T00:34:33.3366094Z status: exit code: 1
2019-07-17T00:34:33.3366094Z status: exit code: 1
2019-07-17T00:34:33.3367658Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/tuple_like_enum_variant_constructor_pointer_opt.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/tuple_like_enum_variant_constructor_pointer_opt.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/tuple_like_enum_variant_constructor_pointer_opt.stage-id.aux" "-A" "unused"
2019-07-17T00:34:33.3368050Z ------------------------------------------
2019-07-17T00:34:33.3368096Z 
2019-07-17T00:34:33.3368320Z ------------------------------------------
2019-07-17T00:34:33.3368366Z stderr:
---
2019-07-17T00:34:33.4563735Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:33.4563773Z +
2019-07-17T00:34:33.4563792Z 
2019-07-17T00:34:33.4563846Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:33.4563889Z Actual stderr saved to /tmp/compiletest9Cm2qq/tuple_like_enum_variant_constructor_struct_pointer_opt.stderr
2019-07-17T00:34:33.4563937Z To update references, run this command from build directory:
2019-07-17T00:34:33.4564191Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'tuple_like_enum_variant_constructor_struct_pointer_opt.rs'
2019-07-17T00:34:33.4564255Z error: 1 errors occurred comparing output.
2019-07-17T00:34:33.4564291Z status: exit code: 1
2019-07-17T00:34:33.4564291Z status: exit code: 1
2019-07-17T00:34:33.4564927Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/tuple_like_enum_variant_constructor_struct_pointer_opt.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/tuple_like_enum_variant_constructor_struct_pointer_opt.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/tuple_like_enum_variant_constructor_struct_pointer_opt.stage-id.aux" "-A" "unused"
2019-07-17T00:34:33.4565198Z ------------------------------------------
2019-07-17T00:34:33.4565223Z 
2019-07-17T00:34:33.4565390Z ------------------------------------------
2019-07-17T00:34:33.4565442Z stderr:
---
2019-07-17T00:34:33.4844117Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:33.4844160Z +
2019-07-17T00:34:33.4844202Z 
2019-07-17T00:34:33.4844249Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:33.4844293Z Actual stderr saved to /tmp/compiletest9Cm2qq/tuple_like_struct_constructor.stderr
2019-07-17T00:34:33.4844356Z To update references, run this command from build directory:
2019-07-17T00:34:33.4844610Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'tuple_like_struct_constructor.rs'
2019-07-17T00:34:33.4844677Z error: 1 errors occurred comparing output.
2019-07-17T00:34:33.4844732Z status: exit code: 1
2019-07-17T00:34:33.4844732Z status: exit code: 1
2019-07-17T00:34:33.4845312Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/tuple_like_struct_constructor.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/tuple_like_struct_constructor.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/tuple_like_struct_constructor.stage-id.aux" "-A" "unused"
2019-07-17T00:34:33.4845610Z ------------------------------------------
2019-07-17T00:34:33.4845638Z 
2019-07-17T00:34:33.4845852Z ------------------------------------------
2019-07-17T00:34:33.4845890Z stderr:
---
2019-07-17T00:34:33.6400818Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:33.6400859Z +
2019-07-17T00:34:33.6400881Z 
2019-07-17T00:34:33.6400935Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:33.6401142Z Actual stderr saved to /tmp/compiletest9Cm2qq/union-overwrite.stderr
2019-07-17T00:34:33.6401186Z To update references, run this command from build directory:
2019-07-17T00:34:33.6401414Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'union-overwrite.rs'
2019-07-17T00:34:33.6401480Z error: 1 errors occurred comparing output.
2019-07-17T00:34:33.6401518Z status: exit code: 1
2019-07-17T00:34:33.6401518Z status: exit code: 1
2019-07-17T00:34:33.6402083Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/union-overwrite.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/union-overwrite.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/union-overwrite.stage-id.aux" "-A" "unused"
2019-07-17T00:34:33.6402364Z ------------------------------------------
2019-07-17T00:34:33.6402394Z 
2019-07-17T00:34:33.6402572Z ------------------------------------------
2019-07-17T00:34:33.6402609Z stderr:
---
2019-07-17T00:34:33.8044712Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:33.8044753Z +
2019-07-17T00:34:33.8044795Z 
2019-07-17T00:34:33.8044835Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:33.8044876Z Actual stderr saved to /tmp/compiletest9Cm2qq/union.stderr
2019-07-17T00:34:33.8044917Z To update references, run this command from build directory:
2019-07-17T00:34:33.8045172Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'union.rs'
2019-07-17T00:34:33.8045239Z error: 1 errors occurred comparing output.
2019-07-17T00:34:33.8045302Z status: exit code: 1
2019-07-17T00:34:33.8045302Z status: exit code: 1
2019-07-17T00:34:33.8046014Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/union.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/union.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/union.stage-id.aux" "-A" "unused"
2019-07-17T00:34:33.8046300Z ------------------------------------------
2019-07-17T00:34:33.8046329Z 
2019-07-17T00:34:33.8046692Z ------------------------------------------
2019-07-17T00:34:33.8047148Z stderr:
---
2019-07-17T00:34:33.8323616Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:33.8323656Z +
2019-07-17T00:34:33.8323677Z 
2019-07-17T00:34:33.8323733Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:33.8323775Z Actual stderr saved to /tmp/compiletest9Cm2qq/u128.stderr
2019-07-17T00:34:33.8323816Z To update references, run this command from build directory:
2019-07-17T00:34:33.8324052Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'u128.rs'
2019-07-17T00:34:33.8324116Z error: 1 errors occurred comparing output.
2019-07-17T00:34:33.8324177Z status: exit code: 1
2019-07-17T00:34:33.8324177Z status: exit code: 1
2019-07-17T00:34:33.8324691Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/u128.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/u128.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/u128.stage-id.aux" "-A" "unused"
2019-07-17T00:34:33.8324970Z ------------------------------------------
2019-07-17T00:34:33.8324997Z 
2019-07-17T00:34:33.8325176Z ------------------------------------------
2019-07-17T00:34:33.8325230Z stderr:
---
2019-07-17T00:34:33.9604919Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:33.9604983Z +
2019-07-17T00:34:33.9605006Z 
2019-07-17T00:34:33.9605044Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:33.9605108Z Actual stderr saved to /tmp/compiletest9Cm2qq/unops.stderr
2019-07-17T00:34:33.9605148Z To update references, run this command from build directory:
2019-07-17T00:34:33.9605557Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'unops.rs'
2019-07-17T00:34:33.9605647Z error: 1 errors occurred comparing output.
2019-07-17T00:34:33.9605683Z status: exit code: 1
2019-07-17T00:34:33.9605683Z status: exit code: 1
2019-07-17T00:34:33.9606201Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/unops.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/unops.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/unops.stage-id.aux" "-A" "unused"
2019-07-17T00:34:33.9606653Z ------------------------------------------
2019-07-17T00:34:33.9606696Z 
2019-07-17T00:34:33.9607722Z ------------------------------------------
2019-07-17T00:34:33.9607777Z stderr:
---
2019-07-17T00:34:33.9894070Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:33.9894112Z +
2019-07-17T00:34:33.9894133Z 
2019-07-17T00:34:33.9894188Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:33.9894407Z Actual stderr saved to /tmp/compiletest9Cm2qq/unsized-tuple-impls.stderr
2019-07-17T00:34:33.9894452Z To update references, run this command from build directory:
2019-07-17T00:34:33.9894702Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'unsized-tuple-impls.rs'
2019-07-17T00:34:33.9894768Z error: 1 errors occurred comparing output.
2019-07-17T00:34:33.9894822Z status: exit code: 1
2019-07-17T00:34:33.9894822Z status: exit code: 1
2019-07-17T00:34:33.9895372Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/unsized-tuple-impls.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/unsized-tuple-impls.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/unsized-tuple-impls.stage-id.aux" "-A" "unused"
2019-07-17T00:34:33.9895660Z ------------------------------------------
2019-07-17T00:34:33.9895688Z 
2019-07-17T00:34:33.9895876Z ------------------------------------------
2019-07-17T00:34:33.9895931Z stderr:
---
2019-07-17T00:34:34.0925717Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:34.0925759Z +
2019-07-17T00:34:34.0925782Z 
2019-07-17T00:34:34.0925838Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:34.0925880Z Actual stderr saved to /tmp/compiletest9Cm2qq/validation_lifetime_resolution.stderr
2019-07-17T00:34:34.0925930Z To update references, run this command from build directory:
2019-07-17T00:34:34.0926190Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'validation_lifetime_resolution.rs'
2019-07-17T00:34:34.0926261Z error: 1 errors occurred comparing output.
2019-07-17T00:34:34.0926315Z status: exit code: 1
2019-07-17T00:34:34.0926315Z status: exit code: 1
2019-07-17T00:34:34.0927676Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/validation_lifetime_resolution.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/validation_lifetime_resolution.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/validation_lifetime_resolution.stage-id.aux" "-A" "unused"
2019-07-17T00:34:34.0928049Z ------------------------------------------
2019-07-17T00:34:34.0928200Z 
2019-07-17T00:34:34.0928453Z ------------------------------------------
2019-07-17T00:34:34.0928521Z stderr:
---
2019-07-17T00:34:34.1602686Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:34.1602724Z +
2019-07-17T00:34:34.1602745Z 
2019-07-17T00:34:34.1602804Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:34.1603003Z Actual stderr saved to /tmp/compiletest9Cm2qq/vec-matching-fold.stderr
2019-07-17T00:34:34.1603044Z To update references, run this command from build directory:
2019-07-17T00:34:34.1603272Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'vec-matching-fold.rs'
2019-07-17T00:34:34.1603334Z error: 1 errors occurred comparing output.
2019-07-17T00:34:34.1603369Z status: exit code: 1
2019-07-17T00:34:34.1603369Z status: exit code: 1
2019-07-17T00:34:34.1603986Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/vec-matching-fold.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/vec-matching-fold.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/vec-matching-fold.stage-id.aux" "-A" "unused"
2019-07-17T00:34:34.1604272Z ------------------------------------------
2019-07-17T00:34:34.1604299Z 
2019-07-17T00:34:34.1604467Z ------------------------------------------
2019-07-17T00:34:34.1604527Z stderr:
---
2019-07-17T00:34:34.2846295Z -Iter([], [])
2019-07-17T00:34:34.2850608Z -
2019-07-17T00:34:34.2855596Z 
2019-07-17T00:34:34.2855676Z The actual stdout differed from the expected stdout.
2019-07-17T00:34:34.2860825Z Actual stdout saved to /tmp/compiletest9Cm2qq/vecdeque.stdout
2019-07-17T00:34:34.2882652Z error[E0080]: Miri evaluation error: attempted to do invalid arithmetic on pointers that would leak base addresses, e.g., comparing pointers into different allocations
2019-07-17T00:34:34.2883074Z     --> /checkout/src/libcore/intrinsics.rs:1340:19
2019-07-17T00:34:34.2883143Z      |
2019-07-17T00:34:34.2883180Z 1340 |     let diff = if src_usize > dst_usize {
---
2019-07-17T00:34:34.2886271Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:34.2886309Z +
2019-07-17T00:34:34.2886350Z 
2019-07-17T00:34:34.2886386Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:34.2886597Z Actual stderr saved to /tmp/compiletest9Cm2qq/vecdeque.stderr
2019-07-17T00:34:34.2886730Z To update references, run this command from build directory:
2019-07-17T00:34:34.2887450Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'vecdeque.rs'
2019-07-17T00:34:34.2887545Z error: 2 errors occurred comparing output.
2019-07-17T00:34:34.2887623Z status: exit code: 1
2019-07-17T00:34:34.2887623Z status: exit code: 1
2019-07-17T00:34:34.2888267Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/vecdeque.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/vecdeque.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/vecdeque.stage-id.aux" "-A" "unused"
2019-07-17T00:34:34.2888595Z ------------------------------------------
2019-07-17T00:34:34.2888629Z 
2019-07-17T00:34:34.2888880Z ------------------------------------------
2019-07-17T00:34:34.2888927Z stderr:
---
2019-07-17T00:34:34.4162885Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:34.4162925Z +
2019-07-17T00:34:34.4162947Z 
2019-07-17T00:34:34.4162985Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:34.4163111Z Actual stderr saved to /tmp/compiletest9Cm2qq/vecs.stderr
2019-07-17T00:34:34.4163159Z To update references, run this command from build directory:
2019-07-17T00:34:34.4163395Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'vecs.rs'
2019-07-17T00:34:34.4163488Z error: 1 errors occurred comparing output.
2019-07-17T00:34:34.4163525Z status: exit code: 1
2019-07-17T00:34:34.4163525Z status: exit code: 1
2019-07-17T00:34:34.4164059Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/vecs.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/vecs.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/vecs.stage-id.aux" "-A" "unused"
2019-07-17T00:34:34.4164331Z ------------------------------------------
2019-07-17T00:34:34.4164359Z 
2019-07-17T00:34:34.4164557Z ------------------------------------------
2019-07-17T00:34:34.4164596Z stderr:
---
2019-07-17T00:34:34.4243422Z +
2019-07-17T00:34:34.4244018Z thread '[ui] run-pass/volatile.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
2019-07-17T00:34:34.4244526Z 
2019-07-17T00:34:34.4244718Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:34.4244892Z Actual stderr saved to /tmp/compiletest9Cm2qq/volatile.stderr
2019-07-17T00:34:34.4245096Z To update references, run this command from build directory:
2019-07-17T00:34:34.4245507Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'volatile.rs'
2019-07-17T00:34:34.4245910Z error: 1 errors occurred comparing output.
2019-07-17T00:34:34.4246072Z status: exit code: 1
2019-07-17T00:34:34.4246072Z status: exit code: 1
2019-07-17T00:34:34.4247711Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/volatile.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/volatile.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/volatile.stage-id.aux" "-A" "unused"
2019-07-17T00:34:34.4248580Z ------------------------------------------
2019-07-17T00:34:34.4248836Z 
2019-07-17T00:34:34.4249471Z ------------------------------------------
2019-07-17T00:34:34.4249759Z stderr:
---
2019-07-17T00:34:34.5642355Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:34.5642410Z +
2019-07-17T00:34:34.5642431Z 
2019-07-17T00:34:34.5642537Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:34.5642763Z Actual stderr saved to /tmp/compiletest9Cm2qq/without-validation.stderr
2019-07-17T00:34:34.5642823Z To update references, run this command from build directory:
2019-07-17T00:34:34.5643046Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'without-validation.rs'
2019-07-17T00:34:34.5643126Z error: 1 errors occurred comparing output.
2019-07-17T00:34:34.5643163Z status: exit code: 1
2019-07-17T00:34:34.5643163Z status: exit code: 1
2019-07-17T00:34:34.5643721Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/without-validation.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/without-validation.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-Zmiri-disable-validation" "-L" "/tmp/compiletest9Cm2qq/without-validation.stage-id.aux" "-A" "unused"
2019-07-17T00:34:34.5643989Z ------------------------------------------
2019-07-17T00:34:34.5644016Z 
2019-07-17T00:34:34.5644209Z ------------------------------------------
2019-07-17T00:34:34.5644311Z stderr:
---
2019-07-17T00:34:34.6063657Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:34.6063707Z +
2019-07-17T00:34:34.6063729Z 
2019-07-17T00:34:34.6063789Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:34.6064045Z Actual stderr saved to /tmp/compiletest9Cm2qq/write-bytes.stderr
2019-07-17T00:34:34.6064090Z To update references, run this command from build directory:
2019-07-17T00:34:34.6064334Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'write-bytes.rs'
2019-07-17T00:34:34.6064400Z error: 1 errors occurred comparing output.
2019-07-17T00:34:34.6064435Z status: exit code: 1
2019-07-17T00:34:34.6064435Z status: exit code: 1
2019-07-17T00:34:34.6064972Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/write-bytes.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/write-bytes.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/write-bytes.stage-id.aux" "-A" "unused"
2019-07-17T00:34:34.6065247Z ------------------------------------------
2019-07-17T00:34:34.6065352Z 
2019-07-17T00:34:34.6065560Z ------------------------------------------
2019-07-17T00:34:34.6065617Z stderr:
---
2019-07-17T00:34:34.7120121Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:34.7120189Z +
2019-07-17T00:34:34.7120218Z 
2019-07-17T00:34:34.7120275Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:34.7120722Z Actual stderr saved to /tmp/compiletest9Cm2qq/zero-sized-binary-heap-push.stderr
2019-07-17T00:34:34.7120946Z To update references, run this command from build directory:
2019-07-17T00:34:34.7121157Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'zero-sized-binary-heap-push.rs'
2019-07-17T00:34:34.7121233Z error: 1 errors occurred comparing output.
2019-07-17T00:34:34.7121269Z status: exit code: 1
2019-07-17T00:34:34.7121269Z status: exit code: 1
2019-07-17T00:34:34.7121810Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/zero-sized-binary-heap-push.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/zero-sized-binary-heap-push.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/zero-sized-binary-heap-push.stage-id.aux" "-A" "unused"
2019-07-17T00:34:34.7122160Z ------------------------------------------
2019-07-17T00:34:34.7122201Z 
2019-07-17T00:34:34.7122372Z ------------------------------------------
2019-07-17T00:34:34.7122406Z stderr:
---
2019-07-17T00:34:34.7650262Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:34.7650313Z +
2019-07-17T00:34:34.7650342Z 
2019-07-17T00:34:34.7650574Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:34.7650615Z Actual stderr saved to /tmp/compiletest9Cm2qq/zst.stderr
2019-07-17T00:34:34.7650654Z To update references, run this command from build directory:
2019-07-17T00:34:34.7650876Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'zst.rs'
2019-07-17T00:34:34.7650945Z error: 1 errors occurred comparing output.
2019-07-17T00:34:34.7650981Z status: exit code: 1
2019-07-17T00:34:34.7650981Z status: exit code: 1
2019-07-17T00:34:34.7651483Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/zst.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/zst.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/zst.stage-id.aux" "-A" "unused"
2019-07-17T00:34:34.7651846Z ------------------------------------------
2019-07-17T00:34:34.7651873Z 
2019-07-17T00:34:34.7652044Z ------------------------------------------
2019-07-17T00:34:34.7652079Z stderr:
---
2019-07-17T00:34:34.8185235Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:34.8185291Z +
2019-07-17T00:34:34.8185314Z 
2019-07-17T00:34:34.8185351Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:34.8185391Z Actual stderr saved to /tmp/compiletest9Cm2qq/zst_box.stderr
2019-07-17T00:34:34.8185449Z To update references, run this command from build directory:
2019-07-17T00:34:34.8185679Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'zst_box.rs'
2019-07-17T00:34:34.8185745Z error: 1 errors occurred comparing output.
2019-07-17T00:34:34.8185798Z status: exit code: 1
2019-07-17T00:34:34.8185798Z status: exit code: 1
2019-07-17T00:34:34.8186309Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/zst_box.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/zst_box.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/zst_box.stage-id.aux" "-A" "unused"
2019-07-17T00:34:34.8187401Z ------------------------------------------
2019-07-17T00:34:34.8187445Z 
2019-07-17T00:34:34.8187682Z ------------------------------------------
2019-07-17T00:34:34.8187759Z stderr:
---
2019-07-17T00:34:34.8822834Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T00:34:34.8822874Z +
2019-07-17T00:34:34.8822915Z 
2019-07-17T00:34:34.8822953Z The actual stderr differed from the expected stderr.
2019-07-17T00:34:34.8822996Z Actual stderr saved to /tmp/compiletest9Cm2qq/zst_variant_drop.stderr
2019-07-17T00:34:34.8823036Z To update references, run this command from build directory:
2019-07-17T00:34:34.8823288Z tests/run-pass/update-references.sh '/tmp/compiletest9Cm2qq' 'zst_variant_drop.rs'
2019-07-17T00:34:34.8823354Z error: 1 errors occurred comparing output.
2019-07-17T00:34:34.8823418Z status: exit code: 1
2019-07-17T00:34:34.8823418Z status: exit code: 1
2019-07-17T00:34:34.8823964Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/zst_variant_drop.rs" "-L" "/tmp/compiletest9Cm2qq" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest9Cm2qq/zst_variant_drop.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest9Cm2qq/zst_variant_drop.stage-id.aux" "-A" "unused"
2019-07-17T00:34:34.8824365Z ------------------------------------------
2019-07-17T00:34:34.8824394Z 
2019-07-17T00:34:34.8824586Z ------------------------------------------
2019-07-17T00:34:34.8824642Z stderr:
---
2019-07-17T00:34:34.9142570Z This PR updated 'src/doc/embedded-book', verifying if status is 'test-pass'...
2019-07-17T00:34:34.9151267Z Verifying status of rustc-guide...
2019-07-17T00:34:34.9162386Z This PR updated 'src/doc/rustc-guide', verifying if status is 'test-pass'...
2019-07-17T00:34:34.9172663Z 
2019-07-17T00:34:34.9178335Z ⚠️ We detected that this PR updated 'rustc-guide', but its tests failed.
2019-07-17T00:34:34.9178458Z 
2019-07-17T00:34:34.9178782Z If you do intend to update 'rustc-guide', please check the error messages above and
2019-07-17T00:34:34.9178867Z commit another update.
2019-07-17T00:34:34.9178896Z 
2019-07-17T00:34:34.9179161Z If you do NOT intend to update 'rustc-guide', please ensure you did not accidentally
2019-07-17T00:34:34.9179425Z change the submodule at 'src/doc/rustc-guide'. You may ask your reviewer for the
2019-07-17T00:34:34.9179496Z proper steps.
2019-07-17T00:34:35.5893723Z ##[error]Bash exited with code '3'.
2019-07-17T00:34:35.5932298Z ##[section]Starting: Checkout
2019-07-17T00:34:35.5933814Z ==============================================================================
2019-07-17T00:34:35.5933859Z Task         : Get sources
2019-07-17T00:34:35.5933933Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
