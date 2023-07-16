plain
2019-07-17T01:18:38.1027571Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-17T01:18:38.1223954Z ##[command]git config gc.auto 0
2019-07-17T01:18:38.1324731Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-17T01:18:38.1354823Z ##[command]git config --get-all http.proxy
2019-07-17T01:18:38.1496211Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62733/merge:refs/remotes/pull/62733/merge
---
2019-07-17T01:19:12.4552636Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-17T01:19:12.4552957Z 
2019-07-17T01:19:12.4553480Z   git checkout -b <new-branch-name>
2019-07-17T01:19:12.4553788Z 
2019-07-17T01:19:12.4554053Z HEAD is now at cad8f54ed Merge 0c271b2b20371a5db83adc8469c73c94ee35c534 into 07e0c3651ce2a7b326f7678e135d8d5bbbbe5d18
2019-07-17T01:19:12.4694925Z ##[section]Finishing: Checkout
2019-07-17T01:19:12.4701094Z ##[section]Starting: Decide whether to run this job
2019-07-17T01:19:12.4703778Z Task         : Bash
2019-07-17T01:19:12.4703819Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-17T01:19:12.4703858Z Version      : 3.151.2
2019-07-17T01:19:12.4703913Z Author       : Microsoft Corporation
2019-07-17T01:19:12.4703913Z Author       : Microsoft Corporation
2019-07-17T01:19:12.4703960Z Help         : https://docs.microsoft.com/azure/devops/pipelines/tasks/utility/bash
2019-07-17T01:19:12.4704006Z ==============================================================================
2019-07-17T01:19:12.6187742Z Generating script.
2019-07-17T01:19:12.6216930Z ========================== Starting Command Output ===========================
2019-07-17T01:19:12.6244343Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/cb5d4e6d-b259-4bd8-99ec-ab101455847a.sh
2019-07-17T01:19:12.8436222Z Executing the job since submodules are updated
2019-07-17T01:19:12.8534533Z ##[section]Finishing: Decide whether to run this job
2019-07-17T01:19:12.8545535Z ==============================================================================
2019-07-17T01:19:12.8545598Z Task         : Bash
2019-07-17T01:19:12.8545700Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-17T01:19:12.8545751Z Version      : 3.151.2
---
2019-07-17T01:21:04.5348444Z Attempting with retry: curl -y 30 -Y 10 --connect-timeout 30 -f -L -C - -o /tmp/rustci_docker_cache https://.s3.amazonaws.com/docker/5237541d5b559bcea2f168e4ed3efe600aec9344996d80ee3a8ea6b1b5d4d12b46df9cf61ac5cec12b0346064f67d51e8664869834273098748b7174157de4e3
2019-07-17T01:21:04.5405855Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-17T01:21:04.5406158Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-17T01:21:04.5406417Z 
2019-07-17T01:21:04.5451022Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-17T01:21:05.5536128Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-17T01:21:05.5544877Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-17T01:21:05.5545691Z 
2019-07-17T01:21:05.5545691Z 
2019-07-17T01:21:05.5578066Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-17T01:21:07.5650044Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-17T01:21:07.5661495Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-17T01:21:07.5661635Z 
2019-07-17T01:21:07.5661635Z 
2019-07-17T01:21:07.5693660Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-17T01:21:10.5769018Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-17T01:21:10.5769391Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-17T01:21:10.5769436Z 
2019-07-17T01:21:10.5769436Z 
2019-07-17T01:21:10.5817318Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-17T01:21:14.5889528Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-17T01:21:14.5889824Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-17T01:21:14.5890028Z 
2019-07-17T01:21:14.5890028Z 
2019-07-17T01:21:14.5948060Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-17T01:21:14.5958687Z The command has failed after 5 attempts.
2019-07-17T01:21:14.6421403Z open /tmp/rustci_docker_cache: no such file or directory
2019-07-17T01:21:14.6451943Z Attempting with retry: docker build --rm -t rust-ci -f /home/vsts/work/1/s/src/ci/docker/x86_64-gnu-tools/Dockerfile /home/vsts/work/1/s/src/ci/docker
2019-07-17T01:21:14.8506104Z Sending build context to Docker daemon  521.2kB
2019-07-17T01:21:14.8506840Z 
2019-07-17T01:21:14.8723122Z Step 1/9 : FROM ubuntu:16.04
---
2019-07-17T01:21:31.3131616Z Reading package lists...
2019-07-17T01:21:32.2797518Z Reading package lists...
2019-07-17T01:21:32.4469441Z Building dependency tree...
2019-07-17T01:21:32.4469574Z Reading state information...
2019-07-17T01:21:32.5544736Z The following additional packages will be installed:
2019-07-17T01:21:32.5545642Z   binutils bzip2 cmake-data cpp cpp-5 dpkg-dev g++-5 gcc gcc-5 git-man
2019-07-17T01:21:32.5545970Z   libarchive13 libasan2 libasn1-8-heimdal libatomic1 libbz2-1.0 libc-dev-bin
2019-07-17T01:21:32.5546646Z   liberror-perl libexpat1 libffi6 libgcc-5-dev libgdbm3 libglib2.0-0 libgmp10
2019-07-17T01:21:32.5547787Z   libgnutls30 libgomp1 libgssapi-krb5-2 libgssapi3-heimdal libhcrypto4-heimdal
2019-07-17T01:21:32.5548193Z   libheimbase1-heimdal libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal
2019-07-17T01:21:32.5548259Z   libicu55 libidn11 libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1
---
2019-07-17T01:21:32.5549882Z   libssl1.0.0 libstdc++-5-dev libtasn1-6 libtsan0 libubsan0 libwind0-heimdal
2019-07-17T01:21:32.5550176Z   libxml2 linux-libc-dev mime-support openssl patch perl perl-modules-5.22
2019-07-17T01:21:32.5550430Z   python2.7-minimal zlib1g-dev
2019-07-17T01:21:32.5550538Z Suggested packages:
2019-07-17T01:21:32.5550872Z   binutils-doc bzip2-doc codeblocks eclipse ninja-build cpp-doc gcc-5-locales
2019-07-17T01:21:32.5551179Z   debian-keyring g++-multilib g++-5-multilib gcc-5-doc libstdc++6-5-dbg
2019-07-17T01:21:32.5551530Z   gcc-multilib manpages-dev autoconf automake libtool flex bison gdb gcc-doc
2019-07-17T01:21:32.5552122Z   libasan2-dbg liblsan0-dbg libtsan0-dbg libubsan0-dbg libcilkrts5-dbg
2019-07-17T01:21:32.5552466Z   libmpx0-dbg libquadmath0-dbg gettext-base git-daemon-run
2019-07-17T01:21:32.5552466Z   libmpx0-dbg libquadmath0-dbg gettext-base git-daemon-run
2019-07-17T01:21:32.5553456Z   | git-daemon-sysvinit git-doc git-el git-email git-gui gitk gitweb git-arch
2019-07-17T01:21:32.5553832Z   git-cvs git-mediawiki git-svn lrzip glibc-doc gnutls-bin krb5-doc krb5-user
2019-07-17T01:21:32.5554214Z   libstdc++-5-doc make-doc ed diffutils-doc perl-doc libterm-readline-gnu-perl
2019-07-17T01:21:32.5554496Z   | libterm-readline-perl-perl python2.7-doc binfmt-support
2019-07-17T01:21:32.5554567Z Recommended packages:
2019-07-17T01:21:32.5554910Z   build-essential fakeroot libalgorithm-merge-perl less rsync ssh-client
2019-07-17T01:21:32.5555399Z   manpages manpages-dev libfile-fcntllock-perl libglib2.0-data
2019-07-17T01:21:32.5555697Z   shared-mime-info xdg-user-dirs krb5-locales libsasl2-modules libssl-doc
2019-07-17T01:21:32.5555980Z   xml-core netbase rename
2019-07-17T01:21:32.5556050Z The following NEW packages will be installed:
2019-07-17T01:21:32.5556363Z   binutils bzip2 ca-certificates cmake cmake-data cpp cpp-5 curl dpkg-dev file
2019-07-17T01:21:32.5556715Z   g++ g++-5 gcc gcc-5 git git-man libarchive13 libasan2 libasn1-8-heimdal
2019-07-17T01:21:32.5557649Z   libcurl3-gnutls libdpkg-perl liberror-perl libexpat1 libffi6 libgcc-5-dev
2019-07-17T01:21:32.5557993Z   libgdbm3 libglib2.0-0 libgmp10 libgnutls30 libgomp1 libgssapi-krb5-2
2019-07-17T01:21:32.5558288Z   libgssapi3-heimdal libhcrypto4-heimdal libheimbase1-heimdal
2019-07-17T01:21:32.5558600Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-17T01:21:32.5558600Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-17T01:21:32.5558951Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-17T01:21:32.5559264Z   libkrb5-3 libkrb5support0 libldap-2.4-2 liblsan0 liblzo2-2 libmagic1 libmpc3
2019-07-17T01:21:32.5559714Z   libmpfr4 libmpx0 libnettle6 libp11-kit0 libperl5.22 libpython2.7-minimal
2019-07-17T01:21:32.5560213Z   libpython2.7-stdlib libquadmath0 libroken18-heimdal librtmp1 libsasl2-2
2019-07-17T01:21:32.5560520Z   libsasl2-modules-db libsqlite3-0 libssl-dev libssl1.0.0 libstdc++-5-dev
2019-07-17T01:21:32.5560842Z   libtasn1-6 libtsan0 libubsan0 libwind0-heimdal libxml2 linux-libc-dev make
2019-07-17T01:21:32.5561169Z   mime-support openssl patch perl perl-modules-5.22 pkg-config python2.7
2019-07-17T01:21:32.5561444Z   python2.7-minimal sudo xz-utils zlib1g-dev
2019-07-17T01:21:32.5561502Z The following packages will be upgraded:
2019-07-17T01:21:32.9162615Z 1 upgraded, 92 newly installed, 0 to remove and 5 not upgraded.
2019-07-17T01:21:32.9162776Z Need to get 71.6 MB of archives.
2019-07-17T01:21:33.1485238Z After this operation, 310 MB of additional disk space will be used.
2019-07-17T01:21:33.1486484Z Get:1 http://archive.ubuntu.com/ubuntu xenial/main amd64 libgdbm3 amd64 1.8.3-13.1 [16.9 kB]
---
2019-07-17T01:22:55.3504135Z  ---> de150b3b8ab1
2019-07-17T01:22:55.3548617Z Successfully built de150b3b8ab1
2019-07-17T01:22:55.5436312Z Successfully tagged rust-ci:latest
2019-07-17T01:22:55.6276374Z Built container sha256:de150b3b8ab114cfb00fa5c926475f47fdd290248c84bd3aa6417632f6e2b306
2019-07-17T01:22:55.6299966Z Uploading finished image to https://.s3.amazonaws.com/docker/5237541d5b559bcea2f168e4ed3efe600aec9344996d80ee3a8ea6b1b5d4d12b46df9cf61ac5cec12b0346064f67d51e8664869834273098748b7174157de4e3
2019-07-17T01:23:36.3071873Z upload failed: - to s3:///docker/5237541d5b559bcea2f168e4ed3efe600aec9344996d80ee3a8ea6b1b5d4d12b46df9cf61ac5cec12b0346064f67d51e8664869834273098748b7174157de4e3 Parameter validation failed:
2019-07-17T01:23:36.3072424Z Invalid bucket name "": Bucket name must match the regex "^[a-zA-Z0-9.\-_]{1,255}$"
2019-07-17T01:23:37.4768812Z [CI_JOB_NAME=LinuxTools]
2019-07-17T01:23:38.3493332Z [CI_JOB_NAME=LinuxTools]
2019-07-17T01:23:38.4091070Z configure: processing command line
2019-07-17T01:23:38.4092828Z configure: 
2019-07-17T01:23:38.4093645Z configure: rust.dist-src        := False
2019-07-17T01:23:38.4094014Z configure: rust.save-toolstates := /tmp/toolstates.json
---
2019-07-17T04:54:06.3445051Z thread panicked while panicking. aborting.
2019-07-17T04:54:06.5703742Z [2019-07-17T04:54:06Z ERROR rls::server] Can't read message
2019-07-17T04:54:06.5705653Z thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }', src/libcore/result.rs:1051:5
2019-07-17T04:54:06.5706117Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-07-17T04:54:06.5707017Z error: process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/client-31058eff894e71cc` (signal: 4, SIGILL: illegal instruction)
2019-07-17T04:54:06.5722899Z 
2019-07-17T04:54:06.5723732Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/rls/Cargo.toml" "--features" "rustc-workspace-hack/all-static" "--"
2019-07-17T04:54:06.5724175Z expected success, got: exit code: 101
2019-07-17T04:54:06.5724996Z 
---
2019-07-17T05:00:43.9461088Z 
2019-07-17T05:00:43.9461140Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:43.9461213Z Actual stderr saved to /tmp/compiletest4X6PS3/args.stderr
2019-07-17T05:00:43.9461271Z To update references, run this command from build directory:
2019-07-17T05:00:43.9461742Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'args.rs'
2019-07-17T05:00:43.9461853Z error: 2 errors occurred comparing output.
2019-07-17T05:00:43.9461905Z status: exit code: 1
2019-07-17T05:00:43.9461905Z status: exit code: 1
2019-07-17T05:00:43.9462639Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/args.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/args.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/args.stage-id.aux" "-A" "unused"
2019-07-17T05:00:43.9463041Z ------------------------------------------
2019-07-17T05:00:43.9463080Z 
2019-07-17T05:00:43.9463380Z ------------------------------------------
2019-07-17T05:00:43.9463432Z stderr:
---
2019-07-17T05:00:44.0024109Z 
2019-07-17T05:00:44.0024152Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:44.0024199Z Actual stderr saved to /tmp/compiletest4X6PS3/arrays.stderr
2019-07-17T05:00:44.0024410Z To update references, run this command from build directory:
2019-07-17T05:00:44.0024727Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'arrays.rs'
2019-07-17T05:00:44.0024802Z error: 1 errors occurred comparing output.
2019-07-17T05:00:44.0024865Z status: exit code: 1
2019-07-17T05:00:44.0024865Z status: exit code: 1
2019-07-17T05:00:44.0025489Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/arrays.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/arrays.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/arrays.stage-id.aux" "-A" "unused"
2019-07-17T05:00:44.0025818Z ------------------------------------------
2019-07-17T05:00:44.0025850Z 
2019-07-17T05:00:44.0026110Z ------------------------------------------
2019-07-17T05:00:44.0026153Z stderr:
---
2019-07-17T05:00:44.1252064Z 
2019-07-17T05:00:44.1252228Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:44.1252843Z Actual stderr saved to /tmp/compiletest4X6PS3/associated-const.stderr
2019-07-17T05:00:44.1252905Z To update references, run this command from build directory:
2019-07-17T05:00:44.1253236Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'associated-const.rs'
2019-07-17T05:00:44.1253333Z error: 1 errors occurred comparing output.
2019-07-17T05:00:44.1253398Z status: exit code: 1
2019-07-17T05:00:44.1253398Z status: exit code: 1
2019-07-17T05:00:44.1259260Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/associated-const.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/associated-const.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/associated-const.stage-id.aux" "-A" "unused"
2019-07-17T05:00:44.1259766Z ------------------------------------------
2019-07-17T05:00:44.1259807Z 
2019-07-17T05:00:44.1260081Z ------------------------------------------
2019-07-17T05:00:44.1260133Z stderr:
---
2019-07-17T05:00:44.1301596Z 
2019-07-17T05:00:44.1302914Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:44.1304125Z Actual stderr saved to /tmp/compiletest4X6PS3/assume_bug.stderr
2019-07-17T05:00:44.1305306Z To update references, run this command from build directory:
2019-07-17T05:00:44.1306835Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'assume_bug.rs'
2019-07-17T05:00:44.1312764Z error: 1 errors occurred comparing output.
2019-07-17T05:00:44.1319847Z status: exit code: 1
2019-07-17T05:00:44.1319847Z status: exit code: 1
2019-07-17T05:00:44.1321949Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/assume_bug.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/assume_bug.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/assume_bug.stage-id.aux" "-A" "unused"
2019-07-17T05:00:44.1322749Z ------------------------------------------
2019-07-17T05:00:44.1322955Z 
2019-07-17T05:00:44.1323386Z ------------------------------------------
2019-07-17T05:00:44.1323624Z stderr:
---
2019-07-17T05:00:44.2994057Z 
2019-07-17T05:00:44.2994119Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:44.2994433Z Actual stderr saved to /tmp/compiletest4X6PS3/async-fn.stderr
2019-07-17T05:00:44.2994488Z To update references, run this command from build directory:
2019-07-17T05:00:44.2994790Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'async-fn.rs'
2019-07-17T05:00:44.2994871Z error: 1 errors occurred comparing output.
2019-07-17T05:00:44.2994913Z status: exit code: 1
2019-07-17T05:00:44.2994913Z status: exit code: 1
2019-07-17T05:00:44.2995757Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/async-fn.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/async-fn.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/async-fn.stage-id.aux" "-A" "unused"
2019-07-17T05:00:44.2996125Z ------------------------------------------
2019-07-17T05:00:44.2996160Z 
2019-07-17T05:00:44.2997276Z ------------------------------------------
2019-07-17T05:00:44.2997341Z stderr:
---
2019-07-17T05:00:44.3236658Z 
2019-07-17T05:00:44.3237347Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:44.3237915Z Actual stderr saved to /tmp/compiletest4X6PS3/atomic-access-bool.stderr
2019-07-17T05:00:44.3237995Z To update references, run this command from build directory:
2019-07-17T05:00:44.3238783Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'atomic-access-bool.rs'
2019-07-17T05:00:44.3238899Z error: 1 errors occurred comparing output.
2019-07-17T05:00:44.3238947Z status: exit code: 1
2019-07-17T05:00:44.3238947Z status: exit code: 1
2019-07-17T05:00:44.3239652Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/atomic-access-bool.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/atomic-access-bool.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/atomic-access-bool.stage-id.aux" "-A" "unused"
2019-07-17T05:00:44.3240021Z ------------------------------------------
2019-07-17T05:00:44.3240059Z 
2019-07-17T05:00:44.3240330Z ------------------------------------------
2019-07-17T05:00:44.3240379Z stderr:
---
2019-07-17T05:00:44.4632296Z 
2019-07-17T05:00:44.4632362Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:44.4632413Z Actual stderr saved to /tmp/compiletest4X6PS3/bad_substs.stderr
2019-07-17T05:00:44.4632462Z To update references, run this command from build directory:
2019-07-17T05:00:44.4632792Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'bad_substs.rs'
2019-07-17T05:00:44.4632873Z error: 1 errors occurred comparing output.
2019-07-17T05:00:44.4632915Z status: exit code: 1
2019-07-17T05:00:44.4632915Z status: exit code: 1
2019-07-17T05:00:44.4633602Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/bad_substs.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/bad_substs.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/bad_substs.stage-id.aux" "-A" "unused"
2019-07-17T05:00:44.4633953Z ------------------------------------------
2019-07-17T05:00:44.4633989Z 
2019-07-17T05:00:44.4634233Z ------------------------------------------
2019-07-17T05:00:44.4634298Z stderr:
---
2019-07-17T05:00:44.5084724Z 
2019-07-17T05:00:44.5084781Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:44.5085051Z Actual stderr saved to /tmp/compiletest4X6PS3/atomic-compare_exchange.stderr
2019-07-17T05:00:44.5085124Z To update references, run this command from build directory:
2019-07-17T05:00:44.5085391Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'atomic-compare_exchange.rs'
2019-07-17T05:00:44.5085467Z error: 1 errors occurred comparing output.
2019-07-17T05:00:44.5085527Z status: exit code: 1
2019-07-17T05:00:44.5085527Z status: exit code: 1
2019-07-17T05:00:44.5086166Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/atomic-compare_exchange.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/atomic-compare_exchange.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/atomic-compare_exchange.stage-id.aux" "-A" "unused"
2019-07-17T05:00:44.5086505Z ------------------------------------------
2019-07-17T05:00:44.5086540Z 
2019-07-17T05:00:44.5086779Z ------------------------------------------
2019-07-17T05:00:44.5086824Z stderr:
---
2019-07-17T05:00:44.6675798Z 
2019-07-17T05:00:44.6675840Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:44.6675904Z Actual stderr saved to /tmp/compiletest4X6PS3/bools.stderr
2019-07-17T05:00:44.6675953Z To update references, run this command from build directory:
2019-07-17T05:00:44.6676221Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'bools.rs'
2019-07-17T05:00:44.6676313Z error: 1 errors occurred comparing output.
2019-07-17T05:00:44.6676365Z status: exit code: 1
2019-07-17T05:00:44.6676365Z status: exit code: 1
2019-07-17T05:00:44.6676968Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/bools.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/bools.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/bools.stage-id.aux" "-A" "unused"
2019-07-17T05:00:44.6677295Z ------------------------------------------
2019-07-17T05:00:44.6677343Z 
2019-07-17T05:00:44.6677573Z ------------------------------------------
2019-07-17T05:00:44.6677617Z stderr:
---
2019-07-17T05:00:44.6976513Z 
2019-07-17T05:00:44.6976571Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:44.6976619Z Actual stderr saved to /tmp/compiletest4X6PS3/binops.stderr
2019-07-17T05:00:44.6976667Z To update references, run this command from build directory:
2019-07-17T05:00:44.6976947Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'binops.rs'
2019-07-17T05:00:44.6977032Z error: 1 errors occurred comparing output.
2019-07-17T05:00:44.6977090Z status: exit code: 1
2019-07-17T05:00:44.6977090Z status: exit code: 1
2019-07-17T05:00:44.6977697Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/binops.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/binops.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/binops.stage-id.aux" "-A" "unused"
2019-07-17T05:00:44.6978137Z ------------------------------------------
2019-07-17T05:00:44.6978171Z 
2019-07-17T05:00:44.6981107Z ------------------------------------------
2019-07-17T05:00:44.6981220Z stderr:
---
2019-07-17T05:00:44.8355248Z 
2019-07-17T05:00:44.8355314Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:44.8355375Z Actual stderr saved to /tmp/compiletest4X6PS3/box_box_trait.stderr
2019-07-17T05:00:44.8355424Z To update references, run this command from build directory:
2019-07-17T05:00:44.8355760Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'box_box_trait.rs'
2019-07-17T05:00:44.8355836Z error: 1 errors occurred comparing output.
2019-07-17T05:00:44.8355998Z status: exit code: 1
2019-07-17T05:00:44.8355998Z status: exit code: 1
2019-07-17T05:00:44.8356997Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/box_box_trait.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/box_box_trait.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/box_box_trait.stage-id.aux" "-A" "unused"
2019-07-17T05:00:44.8357322Z ------------------------------------------
2019-07-17T05:00:44.8357505Z 
2019-07-17T05:00:44.8357766Z ------------------------------------------
2019-07-17T05:00:44.8357827Z stderr:
---
2019-07-17T05:00:44.8366386Z 
2019-07-17T05:00:44.8366441Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:44.8366664Z Actual stderr saved to /tmp/compiletest4X6PS3/box-pair-to-vec.stderr
2019-07-17T05:00:44.8366713Z To update references, run this command from build directory:
2019-07-17T05:00:44.8366973Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'box-pair-to-vec.rs'
2019-07-17T05:00:44.8367160Z error: 2 errors occurred comparing output.
2019-07-17T05:00:44.8367217Z status: exit code: 1
2019-07-17T05:00:44.8367217Z status: exit code: 1
2019-07-17T05:00:44.8368279Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/box-pair-to-vec.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/box-pair-to-vec.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/box-pair-to-vec.stage-id.aux" "-A" "unused"
2019-07-17T05:00:44.8368717Z ------------------------------------------
2019-07-17T05:00:44.8368750Z 
2019-07-17T05:00:44.8368975Z ------------------------------------------
2019-07-17T05:00:44.8369047Z stderr:
---
2019-07-17T05:00:44.9981735Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T05:00:44.9981784Z +
2019-07-17T05:00:44.9981811Z 
2019-07-17T05:00:44.9981857Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:44.9981942Z Actual stderr saved to /tmp/compiletest4X6PS3/c_enums.stderr
2019-07-17T05:00:44.9981993Z To update references, run this command from build directory:
2019-07-17T05:00:44.9982482Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'c_enums.rs'
2019-07-17T05:00:44.9982571Z error: 1 errors occurred comparing output.
2019-07-17T05:00:44.9982621Z status: exit code: 1
2019-07-17T05:00:44.9982621Z status: exit code: 1
2019-07-17T05:00:44.9983226Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/c_enums.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/c_enums.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/c_enums.stage-id.aux" "-A" "unused"
2019-07-17T05:00:44.9983527Z ------------------------------------------
2019-07-17T05:00:44.9983567Z 
2019-07-17T05:00:44.9983781Z ------------------------------------------
2019-07-17T05:00:44.9983824Z stderr:
---
2019-07-17T05:00:45.0161379Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T05:00:45.0161503Z +
2019-07-17T05:00:45.0161537Z 
2019-07-17T05:00:45.0161591Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:45.0161730Z Actual stderr saved to /tmp/compiletest4X6PS3/btreemap.stderr
2019-07-17T05:00:45.0161806Z To update references, run this command from build directory:
2019-07-17T05:00:45.0162184Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'btreemap.rs'
2019-07-17T05:00:45.0162294Z error: 1 errors occurred comparing output.
2019-07-17T05:00:45.0162349Z status: exit code: 1
2019-07-17T05:00:45.0162349Z status: exit code: 1
2019-07-17T05:00:45.0163082Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/btreemap.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/btreemap.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/btreemap.stage-id.aux" "-A" "unused"
2019-07-17T05:00:45.0163481Z ------------------------------------------
2019-07-17T05:00:45.0163521Z 
2019-07-17T05:00:45.0163818Z ------------------------------------------
2019-07-17T05:00:45.0163871Z stderr:
---
2019-07-17T05:00:45.1617554Z 
2019-07-17T05:00:45.1617595Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:45.1617660Z Actual stderr saved to /tmp/compiletest4X6PS3/call_drop_on_array_elements.stderr
2019-07-17T05:00:45.1617718Z To update references, run this command from build directory:
2019-07-17T05:00:45.1617986Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'call_drop_on_array_elements.rs'
2019-07-17T05:00:45.1618191Z error: 1 errors occurred comparing output.
2019-07-17T05:00:45.1618234Z status: exit code: 1
2019-07-17T05:00:45.1618234Z status: exit code: 1
2019-07-17T05:00:45.1619474Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/call_drop_on_array_elements.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/call_drop_on_array_elements.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/call_drop_on_array_elements.stage-id.aux" "-A" "unused"
2019-07-17T05:00:45.1619825Z ------------------------------------------
2019-07-17T05:00:45.1619869Z 
2019-07-17T05:00:45.1620097Z ------------------------------------------
2019-07-17T05:00:45.1620142Z stderr:
---
2019-07-17T05:00:45.1964818Z 
2019-07-17T05:00:45.1964862Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:45.1964932Z Actual stderr saved to /tmp/compiletest4X6PS3/call_drop_on_fat_ptr_array_elements.stderr
2019-07-17T05:00:45.1964982Z To update references, run this command from build directory:
2019-07-17T05:00:45.1965286Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'call_drop_on_fat_ptr_array_elements.rs'
2019-07-17T05:00:45.1965384Z error: 1 errors occurred comparing output.
2019-07-17T05:00:45.1996215Z status: exit code: 1
2019-07-17T05:00:45.1996779Z thread '[ui] run-pass/call_drop_on_fat_ptr_array_elements.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
2019-07-17T05:00:45.1996779Z thread '[ui] run-pass/call_drop_on_fat_ptr_array_elements.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
2019-07-17T05:00:45.1997479Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/call_drop_on_fat_ptr_array_elements.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/call_drop_on_fat_ptr_array_elements.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/call_drop_on_fat_ptr_array_elements.stage-id.aux" "-A" "unused"
2019-07-17T05:00:45.1997948Z ------------------------------------------
2019-07-17T05:00:45.1997996Z 
2019-07-17T05:00:45.1998841Z ------------------------------------------
2019-07-17T05:00:45.1998900Z stderr:
---
2019-07-17T05:00:45.3298903Z 
2019-07-17T05:00:45.3298971Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:45.3299038Z Actual stderr saved to /tmp/compiletest4X6PS3/call_drop_on_zst_array_elements.stderr
2019-07-17T05:00:45.3299091Z To update references, run this command from build directory:
2019-07-17T05:00:45.3299472Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'call_drop_on_zst_array_elements.rs'
2019-07-17T05:00:45.3299554Z error: 1 errors occurred comparing output.
2019-07-17T05:00:45.3299599Z status: exit code: 1
2019-07-17T05:00:45.3299599Z status: exit code: 1
2019-07-17T05:00:45.3300331Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/call_drop_on_zst_array_elements.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/call_drop_on_zst_array_elements.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/call_drop_on_zst_array_elements.stage-id.aux" "-A" "unused"
2019-07-17T05:00:45.3300874Z ------------------------------------------
2019-07-17T05:00:45.3300910Z 
2019-07-17T05:00:45.3301138Z ------------------------------------------
2019-07-17T05:00:45.3301202Z stderr:
---
2019-07-17T05:00:45.3404370Z 
2019-07-17T05:00:45.3404415Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:45.3404484Z Actual stderr saved to /tmp/compiletest4X6PS3/call_drop_through_owned_slice.stderr
2019-07-17T05:00:45.3404535Z To update references, run this command from build directory:
2019-07-17T05:00:45.3404834Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'call_drop_through_owned_slice.rs'
2019-07-17T05:00:45.3404932Z error: 1 errors occurred comparing output.
2019-07-17T05:00:45.3404984Z status: exit code: 1
2019-07-17T05:00:45.3404984Z status: exit code: 1
2019-07-17T05:00:45.3405790Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/call_drop_through_owned_slice.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/call_drop_through_owned_slice.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/call_drop_through_owned_slice.stage-id.aux" "-A" "unused"
2019-07-17T05:00:45.3406180Z ------------------------------------------
2019-07-17T05:00:45.3406215Z 
2019-07-17T05:00:45.3406449Z ------------------------------------------
2019-07-17T05:00:45.3406496Z stderr:
---
2019-07-17T05:00:45.4739617Z 
2019-07-17T05:00:45.4739680Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:45.4739734Z Actual stderr saved to /tmp/compiletest4X6PS3/call_drop_through_trait_object.stderr
2019-07-17T05:00:45.4739787Z To update references, run this command from build directory:
2019-07-17T05:00:45.4740101Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'call_drop_through_trait_object.rs'
2019-07-17T05:00:45.4740193Z error: 1 errors occurred comparing output.
2019-07-17T05:00:45.4740239Z status: exit code: 1
2019-07-17T05:00:45.4740239Z status: exit code: 1
2019-07-17T05:00:45.4741132Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/call_drop_through_trait_object.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/call_drop_through_trait_object.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/call_drop_through_trait_object.stage-id.aux" "-A" "unused"
2019-07-17T05:00:45.4741539Z ------------------------------------------
2019-07-17T05:00:45.4741574Z 
2019-07-17T05:00:45.4741799Z ------------------------------------------
2019-07-17T05:00:45.4742086Z stderr:
---
2019-07-17T05:00:45.4824165Z 
2019-07-17T05:00:45.4824206Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:45.4824280Z Actual stderr saved to /tmp/compiletest4X6PS3/call_drop_through_trait_object_rc.stderr
2019-07-17T05:00:45.4824328Z To update references, run this command from build directory:
2019-07-17T05:00:45.4824597Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'call_drop_through_trait_object_rc.rs'
2019-07-17T05:00:45.4824769Z error: 1 errors occurred comparing output.
2019-07-17T05:00:45.4824820Z status: exit code: 1
2019-07-17T05:00:45.4824820Z status: exit code: 1
2019-07-17T05:00:45.4825541Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/call_drop_through_trait_object_rc.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/call_drop_through_trait_object_rc.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/call_drop_through_trait_object_rc.stage-id.aux" "-A" "unused"
2019-07-17T05:00:45.4825971Z ------------------------------------------
2019-07-17T05:00:45.4826004Z 
2019-07-17T05:00:45.4826214Z ------------------------------------------
2019-07-17T05:00:45.4826256Z stderr:
---
2019-07-17T05:00:45.6390213Z 
2019-07-17T05:00:45.6390259Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:45.6390331Z Actual stderr saved to /tmp/compiletest4X6PS3/calloc.stderr
2019-07-17T05:00:45.6390383Z To update references, run this command from build directory:
2019-07-17T05:00:45.6390801Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'calloc.rs'
2019-07-17T05:00:45.6390927Z error: 1 errors occurred comparing output.
2019-07-17T05:00:45.6390973Z status: exit code: 1
2019-07-17T05:00:45.6390973Z status: exit code: 1
2019-07-17T05:00:45.6391685Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/calloc.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/calloc.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/calloc.stage-id.aux" "-A" "unused"
2019-07-17T05:00:45.6392180Z ------------------------------------------
2019-07-17T05:00:45.6392241Z 
2019-07-17T05:00:45.6392497Z ------------------------------------------
2019-07-17T05:00:45.6392547Z stderr:
---
2019-07-17T05:00:45.6621179Z 
2019-07-17T05:00:45.6621274Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:45.6621326Z Actual stderr saved to /tmp/compiletest4X6PS3/calls.stderr
2019-07-17T05:00:45.6621504Z To update references, run this command from build directory:
2019-07-17T05:00:45.6621963Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'calls.rs'
2019-07-17T05:00:45.6622048Z error: 1 errors occurred comparing output.
2019-07-17T05:00:45.6622112Z status: exit code: 1
2019-07-17T05:00:45.6622112Z status: exit code: 1
2019-07-17T05:00:45.6622876Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/calls.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/calls.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/calls.stage-id.aux" "-A" "unused"
2019-07-17T05:00:45.6623376Z ------------------------------------------
2019-07-17T05:00:45.6623410Z 
2019-07-17T05:00:45.6623626Z ------------------------------------------
2019-07-17T05:00:45.6623687Z stderr:
---
2019-07-17T05:00:45.7955952Z 
2019-07-17T05:00:45.7956041Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:45.7956106Z Actual stderr saved to /tmp/compiletest4X6PS3/cast_fn_ptr.stderr
2019-07-17T05:00:45.7956285Z To update references, run this command from build directory:
2019-07-17T05:00:45.7956695Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'cast_fn_ptr.rs'
2019-07-17T05:00:45.7956787Z error: 1 errors occurred comparing output.
2019-07-17T05:00:45.7956827Z status: exit code: 1
2019-07-17T05:00:45.7956827Z status: exit code: 1
2019-07-17T05:00:45.7957421Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/cast_fn_ptr.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/cast_fn_ptr.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/cast_fn_ptr.stage-id.aux" "-A" "unused"
2019-07-17T05:00:45.7957841Z ------------------------------------------
2019-07-17T05:00:45.7957873Z 
2019-07-17T05:00:45.7958084Z ------------------------------------------
2019-07-17T05:00:45.7958125Z stderr:
---
2019-07-17T05:00:45.8055972Z 
2019-07-17T05:00:45.8056013Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:45.8056356Z Actual stderr saved to /tmp/compiletest4X6PS3/cast-rfc0401-vtable-kinds.stderr
2019-07-17T05:00:45.8056434Z To update references, run this command from build directory:
2019-07-17T05:00:45.8056734Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'cast-rfc0401-vtable-kinds.rs'
2019-07-17T05:00:45.8056824Z error: 1 errors occurred comparing output.
2019-07-17T05:00:45.8056865Z status: exit code: 1
2019-07-17T05:00:45.8056865Z status: exit code: 1
2019-07-17T05:00:45.8057485Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/cast-rfc0401-vtable-kinds.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/cast-rfc0401-vtable-kinds.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/cast-rfc0401-vtable-kinds.stage-id.aux" "-A" "unused"
2019-07-17T05:00:45.8057926Z ------------------------------------------
2019-07-17T05:00:45.8057958Z 
2019-07-17T05:00:45.8058429Z ------------------------------------------
2019-07-17T05:00:45.8058473Z stderr:
---
2019-07-17T05:00:45.9415725Z 
2019-07-17T05:00:45.9415777Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:45.9415836Z Actual stderr saved to /tmp/compiletest4X6PS3/cast_fn_ptr_unsafe.stderr
2019-07-17T05:00:45.9415910Z To update references, run this command from build directory:
2019-07-17T05:00:45.9416292Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'cast_fn_ptr_unsafe.rs'
2019-07-17T05:00:45.9416384Z error: 1 errors occurred comparing output.
2019-07-17T05:00:45.9416451Z status: exit code: 1
2019-07-17T05:00:45.9416451Z status: exit code: 1
2019-07-17T05:00:45.9417374Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/cast_fn_ptr_unsafe.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/cast_fn_ptr_unsafe.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/cast_fn_ptr_unsafe.stage-id.aux" "-A" "unused"
2019-07-17T05:00:45.9417765Z ------------------------------------------
2019-07-17T05:00:45.9417804Z 
2019-07-17T05:00:45.9418096Z ------------------------------------------
2019-07-17T05:00:45.9418150Z stderr:
---
2019-07-17T05:00:45.9556033Z 
2019-07-17T05:00:45.9556077Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:45.9556125Z Actual stderr saved to /tmp/compiletest4X6PS3/catch.stderr
2019-07-17T05:00:45.9556198Z To update references, run this command from build directory:
2019-07-17T05:00:45.9556462Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'catch.rs'
2019-07-17T05:00:45.9556537Z error: 2 errors occurred comparing output.
2019-07-17T05:00:45.9556598Z status: exit code: 1
2019-07-17T05:00:45.9556598Z status: exit code: 1
2019-07-17T05:00:45.9557204Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/catch.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/catch.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/catch.stage-id.aux" "-A" "unused"
2019-07-17T05:00:45.9557529Z ------------------------------------------
2019-07-17T05:00:45.9557561Z 
2019-07-17T05:00:45.9557902Z ------------------------------------------
2019-07-17T05:00:45.9557965Z stderr:
---
2019-07-17T05:00:46.0975971Z 
2019-07-17T05:00:46.0976020Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:46.0976307Z Actual stderr saved to /tmp/compiletest4X6PS3/closure-drop.stderr
2019-07-17T05:00:46.0976364Z To update references, run this command from build directory:
2019-07-17T05:00:46.0976665Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'closure-drop.rs'
2019-07-17T05:00:46.0976748Z error: 1 errors occurred comparing output.
2019-07-17T05:00:46.0976809Z status: exit code: 1
2019-07-17T05:00:46.0976809Z status: exit code: 1
2019-07-17T05:00:46.0977483Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/closure-drop.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/closure-drop.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/closure-drop.stage-id.aux" "-A" "unused"
2019-07-17T05:00:46.0977836Z ------------------------------------------
2019-07-17T05:00:46.0977872Z 
2019-07-17T05:00:46.0978116Z ------------------------------------------
2019-07-17T05:00:46.0978178Z stderr:
---
2019-07-17T05:00:46.1093567Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T05:00:46.1093628Z +
2019-07-17T05:00:46.1093655Z 
2019-07-17T05:00:46.1093697Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:46.1093754Z Actual stderr saved to /tmp/compiletest4X6PS3/char.stderr
2019-07-17T05:00:46.1093819Z To update references, run this command from build directory:
2019-07-17T05:00:46.1094074Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'char.rs'
2019-07-17T05:00:46.1094164Z error: 1 errors occurred comparing output.
2019-07-17T05:00:46.1094208Z status: exit code: 1
2019-07-17T05:00:46.1094208Z status: exit code: 1
2019-07-17T05:00:46.1094808Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/char.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/char.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/char.stage-id.aux" "-A" "unused"
2019-07-17T05:00:46.1095127Z ------------------------------------------
2019-07-17T05:00:46.1095167Z 
2019-07-17T05:00:46.1095400Z ------------------------------------------
2019-07-17T05:00:46.1095443Z stderr:
---
2019-07-17T05:00:46.2967477Z 
2019-07-17T05:00:46.2967521Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:46.2967780Z Actual stderr saved to /tmp/compiletest4X6PS3/closure-field-ty.stderr
2019-07-17T05:00:46.2967847Z To update references, run this command from build directory:
2019-07-17T05:00:46.2968230Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'closure-field-ty.rs'
2019-07-17T05:00:46.2968326Z error: 1 errors occurred comparing output.
2019-07-17T05:00:46.2968496Z status: exit code: 1
2019-07-17T05:00:46.2968496Z status: exit code: 1
2019-07-17T05:00:46.2969623Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/closure-field-ty.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/closure-field-ty.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/closure-field-ty.stage-id.aux" "-A" "unused"
2019-07-17T05:00:46.2969978Z ------------------------------------------
2019-07-17T05:00:46.2970012Z 
2019-07-17T05:00:46.2970254Z ------------------------------------------
2019-07-17T05:00:46.2970299Z stderr:
---
2019-07-17T05:00:46.3536864Z 
2019-07-17T05:00:46.3536905Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:46.3536965Z Actual stderr saved to /tmp/compiletest4X6PS3/closures.stderr
2019-07-17T05:00:46.3537010Z To update references, run this command from build directory:
2019-07-17T05:00:46.3537265Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'closures.rs'
2019-07-17T05:00:46.3537353Z error: 1 errors occurred comparing output.
2019-07-17T05:00:46.3537394Z status: exit code: 1
2019-07-17T05:00:46.3537394Z status: exit code: 1
2019-07-17T05:00:46.3537986Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/closures.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/closures.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/closures.stage-id.aux" "-A" "unused"
2019-07-17T05:00:46.3538281Z ------------------------------------------
2019-07-17T05:00:46.3538328Z 
2019-07-17T05:00:46.3538664Z ------------------------------------------
2019-07-17T05:00:46.3538706Z stderr:
---
2019-07-17T05:00:46.4375098Z 
2019-07-17T05:00:46.4375146Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:46.4375464Z Actual stderr saved to /tmp/compiletest4X6PS3/const-vec-of-fns.stderr
2019-07-17T05:00:46.4375522Z To update references, run this command from build directory:
2019-07-17T05:00:46.4375819Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'const-vec-of-fns.rs'
2019-07-17T05:00:46.4375919Z error: 1 errors occurred comparing output.
2019-07-17T05:00:46.4375975Z status: exit code: 1
2019-07-17T05:00:46.4375975Z status: exit code: 1
2019-07-17T05:00:46.4376907Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/const-vec-of-fns.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/const-vec-of-fns.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/const-vec-of-fns.stage-id.aux" "-A" "unused"
2019-07-17T05:00:46.4377248Z ------------------------------------------
2019-07-17T05:00:46.4377283Z 
2019-07-17T05:00:46.4377511Z ------------------------------------------
2019-07-17T05:00:46.4377556Z stderr:
---
2019-07-17T05:00:46.5176024Z 
2019-07-17T05:00:46.5176067Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:46.5176129Z Actual stderr saved to /tmp/compiletest4X6PS3/constants.stderr
2019-07-17T05:00:46.5176188Z To update references, run this command from build directory:
2019-07-17T05:00:46.5176458Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'constants.rs'
2019-07-17T05:00:46.5176550Z error: 1 errors occurred comparing output.
2019-07-17T05:00:46.5176594Z status: exit code: 1
2019-07-17T05:00:46.5176594Z status: exit code: 1
2019-07-17T05:00:46.5177233Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/constants.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/constants.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/constants.stage-id.aux" "-A" "unused"
2019-07-17T05:00:46.5177546Z ------------------------------------------
2019-07-17T05:00:46.5177594Z 
2019-07-17T05:00:46.5177823Z ------------------------------------------
2019-07-17T05:00:46.5177877Z stderr:
---
2019-07-17T05:00:46.6427074Z 
2019-07-17T05:00:46.6427141Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:46.6427202Z Actual stderr saved to /tmp/compiletest4X6PS3/drop_empty_slice.stderr
2019-07-17T05:00:46.6427261Z To update references, run this command from build directory:
2019-07-17T05:00:46.6427623Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'drop_empty_slice.rs'
2019-07-17T05:00:46.6427716Z error: 1 errors occurred comparing output.
2019-07-17T05:00:46.6427768Z status: exit code: 1
2019-07-17T05:00:46.6427768Z status: exit code: 1
2019-07-17T05:00:46.6428592Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/drop_empty_slice.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/drop_empty_slice.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/drop_empty_slice.stage-id.aux" "-A" "unused"
2019-07-17T05:00:46.6429401Z ------------------------------------------
2019-07-17T05:00:46.6429451Z 
2019-07-17T05:00:46.6429713Z ------------------------------------------
2019-07-17T05:00:46.6429785Z stderr:
---
2019-07-17T05:00:46.7735754Z 
2019-07-17T05:00:46.7735800Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:46.7736091Z Actual stderr saved to /tmp/compiletest4X6PS3/deriving-associated-types.stderr
2019-07-17T05:00:46.7736173Z To update references, run this command from build directory:
2019-07-17T05:00:46.7736474Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'deriving-associated-types.rs'
2019-07-17T05:00:46.7743836Z thread '[ui] run-pass/deriving-associated-types.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
2019-07-17T05:00:46.7743918Z error: 1 errors occurred comparing output.
2019-07-17T05:00:46.7743967Z status: exit code: 1
2019-07-17T05:00:46.7743967Z status: exit code: 1
2019-07-17T05:00:46.7744693Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/deriving-associated-types.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/deriving-associated-types.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/deriving-associated-types.stage-id.aux" "-A" "unused"
2019-07-17T05:00:46.7745041Z ------------------------------------------
2019-07-17T05:00:46.7745077Z 
2019-07-17T05:00:46.7745302Z ------------------------------------------
2019-07-17T05:00:46.7745366Z stderr:
---
2019-07-17T05:00:46.8556694Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T05:00:46.8556771Z +
2019-07-17T05:00:46.8556846Z 
2019-07-17T05:00:46.8556893Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:46.8557194Z Actual stderr saved to /tmp/compiletest4X6PS3/dst-field-align.stderr
2019-07-17T05:00:46.8557266Z To update references, run this command from build directory:
2019-07-17T05:00:46.8557528Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'dst-field-align.rs'
2019-07-17T05:00:46.8557618Z error: 1 errors occurred comparing output.
2019-07-17T05:00:46.8557677Z status: exit code: 1
2019-07-17T05:00:46.8557677Z status: exit code: 1
2019-07-17T05:00:46.8558306Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/dst-field-align.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/dst-field-align.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/dst-field-align.stage-id.aux" "-A" "unused"
2019-07-17T05:00:46.8559352Z ------------------------------------------
2019-07-17T05:00:46.8559431Z 
2019-07-17T05:00:46.8559979Z ------------------------------------------
2019-07-17T05:00:46.8560037Z stderr:
---
2019-07-17T05:00:46.9258370Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T05:00:46.9258432Z +
2019-07-17T05:00:46.9258457Z 
2019-07-17T05:00:46.9258500Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:46.9258883Z Actual stderr saved to /tmp/compiletest4X6PS3/dst-irrefutable-bind.stderr
2019-07-17T05:00:46.9259540Z To update references, run this command from build directory:
2019-07-17T05:00:46.9259896Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'dst-irrefutable-bind.rs'
2019-07-17T05:00:46.9259995Z error: 1 errors occurred comparing output.
2019-07-17T05:00:46.9260042Z status: exit code: 1
2019-07-17T05:00:46.9260042Z status: exit code: 1
2019-07-17T05:00:46.9260715Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/dst-irrefutable-bind.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/dst-irrefutable-bind.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/dst-irrefutable-bind.stage-id.aux" "-A" "unused"
2019-07-17T05:00:46.9261067Z ------------------------------------------
2019-07-17T05:00:46.9261102Z 
2019-07-17T05:00:46.9261344Z ------------------------------------------
2019-07-17T05:00:46.9261389Z stderr:
---
2019-07-17T05:00:47.1116649Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T05:00:47.1116706Z +
2019-07-17T05:00:47.1116739Z 
2019-07-17T05:00:47.1116807Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:47.1117116Z Actual stderr saved to /tmp/compiletest4X6PS3/dst-raw.stderr
2019-07-17T05:00:47.1117176Z To update references, run this command from build directory:
2019-07-17T05:00:47.1117487Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'dst-raw.rs'
2019-07-17T05:00:47.1117577Z error: 1 errors occurred comparing output.
2019-07-17T05:00:47.1117627Z status: exit code: 1
2019-07-17T05:00:47.1117627Z status: exit code: 1
2019-07-17T05:00:47.1118350Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/dst-raw.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/dst-raw.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/dst-raw.stage-id.aux" "-A" "unused"
2019-07-17T05:00:47.1118733Z ------------------------------------------
2019-07-17T05:00:47.1118771Z 
2019-07-17T05:00:47.1119376Z ------------------------------------------
2019-07-17T05:00:47.1119466Z stderr:
---
2019-07-17T05:00:47.1486694Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T05:00:47.1486769Z +
2019-07-17T05:00:47.1486801Z 
2019-07-17T05:00:47.1486853Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:47.1487174Z Actual stderr saved to /tmp/compiletest4X6PS3/dst-struct-sole.stderr
2019-07-17T05:00:47.1487258Z To update references, run this command from build directory:
2019-07-17T05:00:47.1487587Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'dst-struct-sole.rs'
2019-07-17T05:00:47.1487699Z error: 1 errors occurred comparing output.
2019-07-17T05:00:47.1487751Z status: exit code: 1
2019-07-17T05:00:47.1487751Z status: exit code: 1
2019-07-17T05:00:47.1488495Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/dst-struct-sole.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/dst-struct-sole.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/dst-struct-sole.stage-id.aux" "-A" "unused"
2019-07-17T05:00:47.1489286Z ------------------------------------------
2019-07-17T05:00:47.1489359Z 
2019-07-17T05:00:47.1489624Z ------------------------------------------
2019-07-17T05:00:47.1489676Z stderr:
---
2019-07-17T05:00:47.2974790Z 
2019-07-17T05:00:47.2974831Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:47.2975097Z Actual stderr saved to /tmp/compiletest4X6PS3/enum-nullable-const-null-with-fields.stderr
2019-07-17T05:00:47.2975148Z To update references, run this command from build directory:
2019-07-17T05:00:47.2975432Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'enum-nullable-const-null-with-fields.rs'
2019-07-17T05:00:47.2975507Z error: 1 errors occurred comparing output.
2019-07-17T05:00:47.2975561Z status: exit code: 1
2019-07-17T05:00:47.2975561Z status: exit code: 1
2019-07-17T05:00:47.2976223Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/enum-nullable-const-null-with-fields.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/enum-nullable-const-null-with-fields.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/enum-nullable-const-null-with-fields.stage-id.aux" "-A" "unused"
2019-07-17T05:00:47.2976537Z ------------------------------------------
2019-07-17T05:00:47.2976567Z 
2019-07-17T05:00:47.2976801Z ------------------------------------------
2019-07-17T05:00:47.2976841Z stderr:
---
2019-07-17T05:00:47.4088186Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T05:00:47.4088265Z +
2019-07-17T05:00:47.4088293Z 
2019-07-17T05:00:47.4088336Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:47.4089817Z Actual stderr saved to /tmp/compiletest4X6PS3/dst-struct.stderr
2019-07-17T05:00:47.4089913Z To update references, run this command from build directory:
2019-07-17T05:00:47.4090205Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'dst-struct.rs'
2019-07-17T05:00:47.4090318Z error: 1 errors occurred comparing output.
2019-07-17T05:00:47.4090364Z status: exit code: 1
2019-07-17T05:00:47.4090364Z status: exit code: 1
2019-07-17T05:00:47.4091012Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/dst-struct.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/dst-struct.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/dst-struct.stage-id.aux" "-A" "unused"
2019-07-17T05:00:47.4091707Z ------------------------------------------
2019-07-17T05:00:47.4091752Z 
2019-07-17T05:00:47.4092499Z ------------------------------------------
2019-07-17T05:00:47.4102130Z stderr:
---
2019-07-17T05:00:47.4655935Z 
2019-07-17T05:00:47.4655989Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:47.4656067Z Actual stderr saved to /tmp/compiletest4X6PS3/enums.stderr
2019-07-17T05:00:47.4656124Z To update references, run this command from build directory:
2019-07-17T05:00:47.4656463Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'enums.rs'
2019-07-17T05:00:47.4656574Z error: 1 errors occurred comparing output.
2019-07-17T05:00:47.4656626Z status: exit code: 1
2019-07-17T05:00:47.4656626Z status: exit code: 1
2019-07-17T05:00:47.4657375Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/enums.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/enums.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/enums.stage-id.aux" "-A" "unused"
2019-07-17T05:00:47.4657743Z ------------------------------------------
2019-07-17T05:00:47.4657800Z 
2019-07-17T05:00:47.4658074Z ------------------------------------------
2019-07-17T05:00:47.4658125Z stderr:
---
2019-07-17T05:00:47.5898049Z 
2019-07-17T05:00:47.5898112Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:47.5898173Z Actual stderr saved to /tmp/compiletest4X6PS3/env.stderr
2019-07-17T05:00:47.5898224Z To update references, run this command from build directory:
2019-07-17T05:00:47.5898530Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'env.rs'
2019-07-17T05:00:47.5898620Z error: 1 errors occurred comparing output.
2019-07-17T05:00:47.5898668Z status: exit code: 1
2019-07-17T05:00:47.5898668Z status: exit code: 1
2019-07-17T05:00:47.5899704Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/env.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/env.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/env.stage-id.aux" "-A" "unused"
2019-07-17T05:00:47.5900044Z ------------------------------------------
2019-07-17T05:00:47.5900092Z 
2019-07-17T05:00:47.5900317Z ------------------------------------------
2019-07-17T05:00:47.5900377Z stderr:
---
2019-07-17T05:00:47.6231278Z 
2019-07-17T05:00:47.6231325Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:47.6231377Z Actual stderr saved to /tmp/compiletest4X6PS3/exit.stderr
2019-07-17T05:00:47.6231453Z To update references, run this command from build directory:
2019-07-17T05:00:47.6231740Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'exit.rs'
2019-07-17T05:00:47.6231821Z error: 1 errors occurred comparing output.
2019-07-17T05:00:47.6231884Z status: exit code: 1
2019-07-17T05:00:47.6231884Z status: exit code: 1
2019-07-17T05:00:47.6232524Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/exit.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/exit.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/exit.stage-id.aux" "-A" "unused"
2019-07-17T05:00:47.6232873Z ------------------------------------------
2019-07-17T05:00:47.6232907Z 
2019-07-17T05:00:47.6233277Z ------------------------------------------
2019-07-17T05:00:47.6233347Z stderr:
---
2019-07-17T05:00:47.7383459Z 
2019-07-17T05:00:47.7383522Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:47.7383571Z Actual stderr saved to /tmp/compiletest4X6PS3/extern_types.stderr
2019-07-17T05:00:47.7383631Z To update references, run this command from build directory:
2019-07-17T05:00:47.7383930Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'extern_types.rs'
2019-07-17T05:00:47.7384013Z error: 1 errors occurred comparing output.
2019-07-17T05:00:47.7384060Z status: exit code: 1
2019-07-17T05:00:47.7384060Z status: exit code: 1
2019-07-17T05:00:47.7384753Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/extern_types.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/extern_types.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/extern_types.stage-id.aux" "-A" "unused"
2019-07-17T05:00:47.7385105Z ------------------------------------------
2019-07-17T05:00:47.7385140Z 
2019-07-17T05:00:47.7385398Z ------------------------------------------
2019-07-17T05:00:47.7385462Z stderr:
---
2019-07-17T05:00:47.7860361Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T05:00:47.7860424Z +
2019-07-17T05:00:47.7860452Z 
2019-07-17T05:00:47.7860496Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:47.7860557Z Actual stderr saved to /tmp/compiletest4X6PS3/float_fast_math.stderr
2019-07-17T05:00:47.7860622Z To update references, run this command from build directory:
2019-07-17T05:00:47.7860896Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'float_fast_math.rs'
2019-07-17T05:00:47.7860996Z error: 1 errors occurred comparing output.
2019-07-17T05:00:47.7861041Z status: exit code: 1
2019-07-17T05:00:47.7861041Z status: exit code: 1
2019-07-17T05:00:47.7861687Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/float_fast_math.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/float_fast_math.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/float_fast_math.stage-id.aux" "-A" "unused"
2019-07-17T05:00:47.7862016Z ------------------------------------------
2019-07-17T05:00:47.7862049Z 
2019-07-17T05:00:47.7862295Z ------------------------------------------
2019-07-17T05:00:47.7862340Z stderr:
---
2019-07-17T05:00:47.9496491Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T05:00:47.9496554Z +
2019-07-17T05:00:47.9496581Z 
2019-07-17T05:00:47.9496645Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:47.9496925Z Actual stderr saved to /tmp/compiletest4X6PS3/foreign-fn-linkname.stderr
2019-07-17T05:00:47.9496981Z To update references, run this command from build directory:
2019-07-17T05:00:47.9497293Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'foreign-fn-linkname.rs'
2019-07-17T05:00:47.9497374Z error: 1 errors occurred comparing output.
2019-07-17T05:00:47.9497436Z status: exit code: 1
2019-07-17T05:00:47.9497436Z status: exit code: 1
2019-07-17T05:00:47.9498114Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/foreign-fn-linkname.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/foreign-fn-linkname.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/foreign-fn-linkname.stage-id.aux" "-A" "unused"
2019-07-17T05:00:47.9498467Z ------------------------------------------
2019-07-17T05:00:47.9498501Z 
2019-07-17T05:00:47.9498745Z ------------------------------------------
2019-07-17T05:00:47.9498807Z stderr:
---
2019-07-17T05:00:48.0094397Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T05:00:48.0094444Z +
2019-07-17T05:00:48.0094470Z 
2019-07-17T05:00:48.0094541Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:48.0094590Z Actual stderr saved to /tmp/compiletest4X6PS3/floats.stderr
2019-07-17T05:00:48.0094639Z To update references, run this command from build directory:
2019-07-17T05:00:48.0094941Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'floats.rs'
2019-07-17T05:00:48.0095017Z error: 1 errors occurred comparing output.
2019-07-17T05:00:48.0095060Z status: exit code: 1
2019-07-17T05:00:48.0095060Z status: exit code: 1
2019-07-17T05:00:48.0095684Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/floats.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/floats.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/floats.stage-id.aux" "-A" "unused"
2019-07-17T05:00:48.0096175Z ------------------------------------------
2019-07-17T05:00:48.0096223Z 
2019-07-17T05:00:48.0096502Z ------------------------------------------
2019-07-17T05:00:48.0096566Z stderr:
---
2019-07-17T05:00:48.1038807Z -
2019-07-17T05:00:48.1038867Z 
2019-07-17T05:00:48.1043932Z thread '[ui] run-pass/format.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
2019-07-17T05:00:48.1044014Z The actual stdout differed from the expected stdout.
2019-07-17T05:00:48.1044092Z Actual stdout saved to /tmp/compiletest4X6PS3/format.stdout
2019-07-17T05:00:48.1044216Z error[E0080]: Miri evaluation error: attempted to do invalid arithmetic on pointers that would leak base addresses, e.g., comparing pointers into different allocations
2019-07-17T05:00:48.1044889Z     --> /checkout/src/libcore/intrinsics.rs:1340:19
2019-07-17T05:00:48.1044955Z      |
2019-07-17T05:00:48.1045016Z 1340 |     let diff = if src_usize > dst_usize {
---
2019-07-17T05:00:48.1048896Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T05:00:48.1049058Z +
2019-07-17T05:00:48.1049099Z 
2019-07-17T05:00:48.1049140Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:48.1049186Z Actual stderr saved to /tmp/compiletest4X6PS3/format.stderr
2019-07-17T05:00:48.1049247Z To update references, run this command from build directory:
2019-07-17T05:00:48.1050081Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'format.rs'
2019-07-17T05:00:48.1050182Z error: 2 errors occurred comparing output.
2019-07-17T05:00:48.1050250Z status: exit code: 1
2019-07-17T05:00:48.1050250Z status: exit code: 1
2019-07-17T05:00:48.1050944Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/format.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/format.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/format.stage-id.aux" "-A" "unused"
2019-07-17T05:00:48.1051442Z ------------------------------------------
2019-07-17T05:00:48.1051479Z 
2019-07-17T05:00:48.1051739Z ------------------------------------------
2019-07-17T05:00:48.1051786Z stderr:
---
2019-07-17T05:00:48.1449094Z 
2019-07-17T05:00:48.1449777Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:48.1450125Z Actual stderr saved to /tmp/compiletest4X6PS3/from_utf8.stderr
2019-07-17T05:00:48.1450353Z To update references, run this command from build directory:
2019-07-17T05:00:48.1450969Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'from_utf8.rs'
2019-07-17T05:00:48.1451608Z error: 1 errors occurred comparing output.
2019-07-17T05:00:48.1451866Z status: exit code: 1
2019-07-17T05:00:48.1451866Z status: exit code: 1
2019-07-17T05:00:48.1452849Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/from_utf8.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/from_utf8.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/from_utf8.stage-id.aux" "-A" "unused"
2019-07-17T05:00:48.1453876Z ------------------------------------------
2019-07-17T05:00:48.1454159Z 
2019-07-17T05:00:48.1454647Z ------------------------------------------
2019-07-17T05:00:48.1454889Z stderr:
---
2019-07-17T05:00:48.3070607Z 
2019-07-17T05:00:48.3070790Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:48.3070954Z Actual stderr saved to /tmp/compiletest4X6PS3/function_pointers.stderr
2019-07-17T05:00:48.3071120Z To update references, run this command from build directory:
2019-07-17T05:00:48.3071688Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'function_pointers.rs'
2019-07-17T05:00:48.3072809Z error: 1 errors occurred comparing output.
2019-07-17T05:00:48.3073014Z status: exit code: 1
2019-07-17T05:00:48.3073014Z status: exit code: 1
2019-07-17T05:00:48.3074014Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/function_pointers.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/function_pointers.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/function_pointers.stage-id.aux" "-A" "unused"
2019-07-17T05:00:48.3116807Z ------------------------------------------
2019-07-17T05:00:48.3116855Z 
2019-07-17T05:00:48.3117161Z ------------------------------------------
2019-07-17T05:00:48.3117213Z stderr:
---
2019-07-17T05:00:48.3686874Z 
2019-07-17T05:00:48.3686915Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:48.3686959Z Actual stderr saved to /tmp/compiletest4X6PS3/generator.stderr
2019-07-17T05:00:48.3687019Z To update references, run this command from build directory:
2019-07-17T05:00:48.3687349Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'generator.rs'
2019-07-17T05:00:48.3687432Z error: 1 errors occurred comparing output.
2019-07-17T05:00:48.3687491Z status: exit code: 1
2019-07-17T05:00:48.3687491Z status: exit code: 1
2019-07-17T05:00:48.3688091Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/generator.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/generator.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/generator.stage-id.aux" "-A" "unused"
2019-07-17T05:00:48.3688516Z ------------------------------------------
2019-07-17T05:00:48.3688547Z 
2019-07-17T05:00:48.3688762Z ------------------------------------------
2019-07-17T05:00:48.3688820Z stderr:
---
2019-07-17T05:00:48.5011043Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T05:00:48.5018580Z +
2019-07-17T05:00:48.5025903Z 
2019-07-17T05:00:48.5026277Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:48.5026370Z Actual stderr saved to /tmp/compiletest4X6PS3/heap.stderr
2019-07-17T05:00:48.5026719Z To update references, run this command from build directory:
2019-07-17T05:00:48.5031058Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'heap.rs'
2019-07-17T05:00:48.5031186Z error: 1 errors occurred comparing output.
2019-07-17T05:00:48.5031232Z status: exit code: 1
2019-07-17T05:00:48.5031232Z status: exit code: 1
2019-07-17T05:00:48.5031891Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/heap.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/heap.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/heap.stage-id.aux" "-A" "unused"
2019-07-17T05:00:48.5032440Z ------------------------------------------
2019-07-17T05:00:48.5032475Z 
2019-07-17T05:00:48.5032717Z ------------------------------------------
2019-07-17T05:00:48.5032774Z stderr:
---
2019-07-17T05:00:48.5117504Z -Hello, world!
2019-07-17T05:00:48.5117685Z -
2019-07-17T05:00:48.5117723Z 
2019-07-17T05:00:48.5117765Z The actual stdout differed from the expected stdout.
2019-07-17T05:00:48.5117811Z Actual stdout saved to /tmp/compiletest4X6PS3/hello.stdout
2019-07-17T05:00:48.5117929Z error[E0080]: Miri evaluation error: attempted to do invalid arithmetic on pointers that would leak base addresses, e.g., comparing pointers into different allocations
2019-07-17T05:00:48.5118175Z     --> /checkout/src/libcore/intrinsics.rs:1340:19
2019-07-17T05:00:48.5118238Z      |
2019-07-17T05:00:48.5118280Z 1340 |     let diff = if src_usize > dst_usize {
---
2019-07-17T05:00:48.5122558Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T05:00:48.5122605Z +
2019-07-17T05:00:48.5122631Z 
2019-07-17T05:00:48.5122689Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:48.5122864Z Actual stderr saved to /tmp/compiletest4X6PS3/hello.stderr
2019-07-17T05:00:48.5122911Z To update references, run this command from build directory:
2019-07-17T05:00:48.5123170Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'hello.rs'
2019-07-17T05:00:48.5123332Z error: 2 errors occurred comparing output.
2019-07-17T05:00:48.5123386Z status: exit code: 1
2019-07-17T05:00:48.5123386Z status: exit code: 1
2019-07-17T05:00:48.5124121Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/hello.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/hello.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/hello.stage-id.aux" "-A" "unused"
2019-07-17T05:00:48.5124420Z ------------------------------------------
2019-07-17T05:00:48.5124451Z 
2019-07-17T05:00:48.5124660Z ------------------------------------------
2019-07-17T05:00:48.5124713Z stderr:
---
2019-07-17T05:00:49.1261961Z 
2019-07-17T05:00:49.1262025Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:49.1262299Z Actual stderr saved to /tmp/compiletest4X6PS3/integer-ops.stderr
2019-07-17T05:00:49.1262467Z To update references, run this command from build directory:
2019-07-17T05:00:49.1262795Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'integer-ops.rs'
2019-07-17T05:00:49.1262877Z error: 1 errors occurred comparing output.
2019-07-17T05:00:49.1262939Z status: exit code: 1
2019-07-17T05:00:49.1262939Z status: exit code: 1
2019-07-17T05:00:49.1263588Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/integer-ops.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/integer-ops.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/integer-ops.stage-id.aux" "-A" "unused"
2019-07-17T05:00:49.1263923Z ------------------------------------------
2019-07-17T05:00:49.1263967Z 
2019-07-17T05:00:49.1264193Z ------------------------------------------
2019-07-17T05:00:49.1264256Z stderr:
---
2019-07-17T05:00:49.6497976Z 
2019-07-17T05:00:49.6498020Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:49.6498468Z Actual stderr saved to /tmp/compiletest4X6PS3/intrinsics-math.stderr
2019-07-17T05:00:49.6498521Z To update references, run this command from build directory:
2019-07-17T05:00:49.6498799Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'intrinsics-math.rs'
2019-07-17T05:00:49.6498901Z error: 1 errors occurred comparing output.
2019-07-17T05:00:49.6498944Z status: exit code: 1
2019-07-17T05:00:49.6498944Z status: exit code: 1
2019-07-17T05:00:49.6500305Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/intrinsics-math.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/intrinsics-math.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/intrinsics-math.stage-id.aux" "-A" "unused"
2019-07-17T05:00:49.6500723Z ------------------------------------------
2019-07-17T05:00:49.6500773Z 
2019-07-17T05:00:49.6501002Z ------------------------------------------
2019-07-17T05:00:49.6501048Z stderr:
---
2019-07-17T05:00:49.8497801Z 
2019-07-17T05:00:49.8497843Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:49.8497889Z Actual stderr saved to /tmp/compiletest4X6PS3/intrinsics.stderr
2019-07-17T05:00:49.8497949Z To update references, run this command from build directory:
2019-07-17T05:00:49.8498266Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'intrinsics.rs'
2019-07-17T05:00:49.8498340Z error: 1 errors occurred comparing output.
2019-07-17T05:00:49.8498395Z status: exit code: 1
2019-07-17T05:00:49.8498395Z status: exit code: 1
2019-07-17T05:00:49.8498974Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/intrinsics.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/intrinsics.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/intrinsics.stage-id.aux" "-A" "unused"
2019-07-17T05:00:49.8499295Z ------------------------------------------
2019-07-17T05:00:49.8499326Z 
2019-07-17T05:00:49.8499671Z ------------------------------------------
2019-07-17T05:00:49.8499714Z stderr:
---
2019-07-17T05:00:50.0698638Z 
2019-07-17T05:00:50.0698680Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:50.0698735Z Actual stderr saved to /tmp/compiletest4X6PS3/ints.stderr
2019-07-17T05:00:50.0698798Z To update references, run this command from build directory:
2019-07-17T05:00:50.0699044Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'ints.rs'
2019-07-17T05:00:50.0699132Z error: 1 errors occurred comparing output.
2019-07-17T05:00:50.0699174Z status: exit code: 1
2019-07-17T05:00:50.0699174Z status: exit code: 1
2019-07-17T05:00:50.0699869Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/ints.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/ints.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/ints.stage-id.aux" "-A" "unused"
2019-07-17T05:00:50.0700728Z ------------------------------------------
2019-07-17T05:00:50.0700777Z 
2019-07-17T05:00:50.0701029Z ------------------------------------------
2019-07-17T05:00:50.0701073Z stderr:
---
2019-07-17T05:00:50.1986565Z 
2019-07-17T05:00:50.1986642Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:50.1987006Z Actual stderr saved to /tmp/compiletest4X6PS3/issue-15063.stderr
2019-07-17T05:00:50.1987089Z To update references, run this command from build directory:
2019-07-17T05:00:50.1987470Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'issue-15063.rs'
2019-07-17T05:00:50.1987570Z error: 1 errors occurred comparing output.
2019-07-17T05:00:50.1987621Z status: exit code: 1
2019-07-17T05:00:50.1987621Z status: exit code: 1
2019-07-17T05:00:50.1988411Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-15063.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/issue-15063.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/issue-15063.stage-id.aux" "-A" "unused"
2019-07-17T05:00:50.1988850Z ------------------------------------------
2019-07-17T05:00:50.1988895Z 
2019-07-17T05:00:50.1989199Z ------------------------------------------
2019-07-17T05:00:50.1989268Z stderr:
---
2019-07-17T05:00:50.3814276Z 
2019-07-17T05:00:50.3814316Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:50.3814574Z Actual stderr saved to /tmp/compiletest4X6PS3/issue-15080.stderr
2019-07-17T05:00:50.3814622Z To update references, run this command from build directory:
2019-07-17T05:00:50.3814858Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'issue-15080.rs'
2019-07-17T05:00:50.3814940Z error: 1 errors occurred comparing output.
2019-07-17T05:00:50.3814978Z status: exit code: 1
2019-07-17T05:00:50.3814978Z status: exit code: 1
2019-07-17T05:00:50.3815541Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-15080.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/issue-15080.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/issue-15080.stage-id.aux" "-A" "unused"
2019-07-17T05:00:50.3815832Z ------------------------------------------
2019-07-17T05:00:50.3815876Z 
2019-07-17T05:00:50.3816080Z ------------------------------------------
2019-07-17T05:00:50.3816120Z stderr:
---
2019-07-17T05:00:50.5737375Z 
2019-07-17T05:00:50.5737432Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:50.5737726Z Actual stderr saved to /tmp/compiletest4X6PS3/issue-15523-big.stderr
2019-07-17T05:00:50.5737799Z To update references, run this command from build directory:
2019-07-17T05:00:50.5738083Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'issue-15523-big.rs'
2019-07-17T05:00:50.5738162Z error: 1 errors occurred comparing output.
2019-07-17T05:00:50.5738225Z status: exit code: 1
2019-07-17T05:00:50.5738225Z status: exit code: 1
2019-07-17T05:00:50.5738869Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-15523-big.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/issue-15523-big.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/issue-15523-big.stage-id.aux" "-A" "unused"
2019-07-17T05:00:50.5739229Z ------------------------------------------
2019-07-17T05:00:50.5739263Z 
2019-07-17T05:00:50.5739519Z ------------------------------------------
2019-07-17T05:00:50.5739565Z stderr:
---
2019-07-17T05:00:50.7244246Z 
2019-07-17T05:00:50.7244286Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:50.7244523Z Actual stderr saved to /tmp/compiletest4X6PS3/issue-17877.stderr
2019-07-17T05:00:50.7244585Z To update references, run this command from build directory:
2019-07-17T05:00:50.7244817Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'issue-17877.rs'
2019-07-17T05:00:50.7244883Z error: 1 errors occurred comparing output.
2019-07-17T05:00:50.7244939Z status: exit code: 1
2019-07-17T05:00:50.7244939Z status: exit code: 1
2019-07-17T05:00:50.7245629Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-17877.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/issue-17877.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/issue-17877.stage-id.aux" "-A" "unused"
2019-07-17T05:00:50.7245943Z ------------------------------------------
2019-07-17T05:00:50.7245974Z 
2019-07-17T05:00:50.7246203Z ------------------------------------------
2019-07-17T05:00:50.7246243Z stderr:
---
2019-07-17T05:00:50.8802827Z 
2019-07-17T05:00:50.8803019Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:50.8803526Z Actual stderr saved to /tmp/compiletest4X6PS3/issue-20575.stderr
2019-07-17T05:00:50.8803805Z To update references, run this command from build directory:
2019-07-17T05:00:50.8804351Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'issue-20575.rs'
2019-07-17T05:00:50.8804794Z error: 1 errors occurred comparing output.
2019-07-17T05:00:50.8804977Z status: exit code: 1
2019-07-17T05:00:50.8804977Z status: exit code: 1
2019-07-17T05:00:50.8805956Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-20575.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/issue-20575.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/issue-20575.stage-id.aux" "-A" "unused"
2019-07-17T05:00:50.8806758Z ------------------------------------------
2019-07-17T05:00:50.8806981Z 
2019-07-17T05:00:50.8807461Z ------------------------------------------
2019-07-17T05:00:50.8807724Z stderr:
---
2019-07-17T05:00:51.0667268Z 
2019-07-17T05:00:51.0667321Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:51.0667691Z Actual stderr saved to /tmp/compiletest4X6PS3/issue-23261.stderr
2019-07-17T05:00:51.0667760Z To update references, run this command from build directory:
2019-07-17T05:00:51.0668112Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'issue-23261.rs'
2019-07-17T05:00:51.0668246Z error: 1 errors occurred comparing output.
2019-07-17T05:00:51.0668300Z status: exit code: 1
2019-07-17T05:00:51.0668300Z status: exit code: 1
2019-07-17T05:00:51.0669108Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-23261.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/issue-23261.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/issue-23261.stage-id.aux" "-A" "unused"
2019-07-17T05:00:51.0669537Z ------------------------------------------
2019-07-17T05:00:51.0669582Z 
2019-07-17T05:00:51.0669889Z ------------------------------------------
2019-07-17T05:00:51.0669947Z stderr:
---
2019-07-17T05:00:51.2628753Z 
2019-07-17T05:00:51.2628797Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:51.2629087Z Actual stderr saved to /tmp/compiletest4X6PS3/issue-26709.stderr
2019-07-17T05:00:51.2629140Z To update references, run this command from build directory:
2019-07-17T05:00:51.2629412Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'issue-26709.rs'
2019-07-17T05:00:51.2629516Z error: 1 errors occurred comparing output.
2019-07-17T05:00:51.2629558Z status: exit code: 1
2019-07-17T05:00:51.2629558Z status: exit code: 1
2019-07-17T05:00:51.2630799Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-26709.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/issue-26709.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/issue-26709.stage-id.aux" "-A" "unused"
2019-07-17T05:00:51.2631193Z ------------------------------------------
2019-07-17T05:00:51.2631245Z 
2019-07-17T05:00:51.2631472Z ------------------------------------------
2019-07-17T05:00:51.2631517Z stderr:
---
2019-07-17T05:00:51.4240714Z 
2019-07-17T05:00:51.4240759Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:51.4241097Z Actual stderr saved to /tmp/compiletest4X6PS3/issue-27901.stderr
2019-07-17T05:00:51.4241153Z To update references, run this command from build directory:
2019-07-17T05:00:51.4241432Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'issue-27901.rs'
2019-07-17T05:00:51.4241529Z error: 1 errors occurred comparing output.
2019-07-17T05:00:51.4241573Z status: exit code: 1
2019-07-17T05:00:51.4241573Z status: exit code: 1
2019-07-17T05:00:51.4242241Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-27901.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/issue-27901.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/issue-27901.stage-id.aux" "-A" "unused"
2019-07-17T05:00:51.4242564Z ------------------------------------------
2019-07-17T05:00:51.4242597Z 
2019-07-17T05:00:51.4242821Z ------------------------------------------
2019-07-17T05:00:51.4242875Z stderr:
---
2019-07-17T05:00:51.5147765Z 
2019-07-17T05:00:51.5147810Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:51.5148066Z Actual stderr saved to /tmp/compiletest4X6PS3/intrinsics-integer.stderr
2019-07-17T05:00:51.5148145Z To update references, run this command from build directory:
2019-07-17T05:00:51.5148415Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'intrinsics-integer.rs'
2019-07-17T05:00:51.5148494Z error: 1 errors occurred comparing output.
2019-07-17T05:00:51.5148555Z status: exit code: 1
2019-07-17T05:00:51.5148555Z status: exit code: 1
2019-07-17T05:00:51.5149209Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/intrinsics-integer.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/intrinsics-integer.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/intrinsics-integer.stage-id.aux" "-A" "unused"
2019-07-17T05:00:51.5149659Z ------------------------------------------
2019-07-17T05:00:51.5149699Z 
2019-07-17T05:00:51.5150146Z ------------------------------------------
2019-07-17T05:00:51.5150189Z stderr:
---
2019-07-17T05:00:51.6037261Z 
2019-07-17T05:00:51.6037316Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:51.6037559Z Actual stderr saved to /tmp/compiletest4X6PS3/issue-29746.stderr
2019-07-17T05:00:51.6037623Z To update references, run this command from build directory:
2019-07-17T05:00:51.6037856Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'issue-29746.rs'
2019-07-17T05:00:51.6037935Z error: 1 errors occurred comparing output.
2019-07-17T05:00:51.6037991Z status: exit code: 1
2019-07-17T05:00:51.6037991Z status: exit code: 1
2019-07-17T05:00:51.6038561Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-29746.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/issue-29746.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/issue-29746.stage-id.aux" "-A" "unused"
2019-07-17T05:00:51.6038861Z ------------------------------------------
2019-07-17T05:00:51.6038890Z 
2019-07-17T05:00:51.6039101Z ------------------------------------------
2019-07-17T05:00:51.6039141Z stderr:
---
2019-07-17T05:00:51.6494355Z 
2019-07-17T05:00:51.6494510Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:51.6494760Z Actual stderr saved to /tmp/compiletest4X6PS3/issue-30530.stderr
2019-07-17T05:00:51.6494805Z To update references, run this command from build directory:
2019-07-17T05:00:51.6495058Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'issue-30530.rs'
2019-07-17T05:00:51.6495126Z error: 1 errors occurred comparing output.
2019-07-17T05:00:51.6495180Z status: exit code: 1
2019-07-17T05:00:51.6495180Z status: exit code: 1
2019-07-17T05:00:51.6495753Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-30530.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/issue-30530.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/issue-30530.stage-id.aux" "-A" "unused"
2019-07-17T05:00:51.6496050Z ------------------------------------------
2019-07-17T05:00:51.6496079Z 
2019-07-17T05:00:51.6496274Z ------------------------------------------
2019-07-17T05:00:51.6496330Z stderr:
---
2019-07-17T05:00:51.7581651Z 
2019-07-17T05:00:51.7581704Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:51.7582072Z Actual stderr saved to /tmp/compiletest4X6PS3/issue-31267-additional.stderr
2019-07-17T05:00:51.7582172Z To update references, run this command from build directory:
2019-07-17T05:00:51.7582860Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'issue-31267-additional.rs'
2019-07-17T05:00:51.7585578Z error: 1 errors occurred comparing output.
2019-07-17T05:00:51.7585731Z status: exit code: 1
2019-07-17T05:00:51.7585731Z status: exit code: 1
2019-07-17T05:00:51.7586574Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-31267-additional.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/issue-31267-additional.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/issue-31267-additional.stage-id.aux" "-A" "unused"
2019-07-17T05:00:51.7586937Z ------------------------------------------
2019-07-17T05:00:51.7586972Z 
2019-07-17T05:00:51.7587223Z ------------------------------------------
2019-07-17T05:00:51.7587268Z stderr:
---
2019-07-17T05:00:51.7683647Z 
2019-07-17T05:00:51.7683694Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:51.7683996Z Actual stderr saved to /tmp/compiletest4X6PS3/issue-33387.stderr
2019-07-17T05:00:51.7684053Z To update references, run this command from build directory:
2019-07-17T05:00:51.7684339Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'issue-33387.rs'
2019-07-17T05:00:51.7684438Z error: 1 errors occurred comparing output.
2019-07-17T05:00:51.7684484Z status: exit code: 1
2019-07-17T05:00:51.7684484Z status: exit code: 1
2019-07-17T05:00:51.7685438Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-33387.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/issue-33387.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/issue-33387.stage-id.aux" "-A" "unused"
2019-07-17T05:00:51.7685724Z ------------------------------------------
2019-07-17T05:00:51.7685767Z 
2019-07-17T05:00:51.7685962Z ------------------------------------------
2019-07-17T05:00:51.7686000Z stderr:
---
2019-07-17T05:00:51.9002509Z 
2019-07-17T05:00:51.9002562Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:51.9002821Z Actual stderr saved to /tmp/compiletest4X6PS3/issue-34571.stderr
2019-07-17T05:00:51.9002874Z To update references, run this command from build directory:
2019-07-17T05:00:51.9003152Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'issue-34571.rs'
2019-07-17T05:00:51.9003230Z error: 1 errors occurred comparing output.
2019-07-17T05:00:51.9003288Z status: exit code: 1
2019-07-17T05:00:51.9003288Z status: exit code: 1
2019-07-17T05:00:51.9003999Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-34571.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/issue-34571.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/issue-34571.stage-id.aux" "-A" "unused"
2019-07-17T05:00:51.9004466Z ------------------------------------------
2019-07-17T05:00:51.9004498Z 
2019-07-17T05:00:51.9004819Z ------------------------------------------
2019-07-17T05:00:51.9004876Z stderr:
---
2019-07-17T05:00:51.9135383Z 
2019-07-17T05:00:51.9135426Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:51.9135703Z Actual stderr saved to /tmp/compiletest4X6PS3/issue-35815.stderr
2019-07-17T05:00:51.9135755Z To update references, run this command from build directory:
2019-07-17T05:00:51.9136018Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'issue-35815.rs'
2019-07-17T05:00:51.9136111Z error: 1 errors occurred comparing output.
2019-07-17T05:00:51.9136153Z status: exit code: 1
2019-07-17T05:00:51.9136153Z status: exit code: 1
2019-07-17T05:00:51.9136789Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-35815.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/issue-35815.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/issue-35815.stage-id.aux" "-A" "unused"
2019-07-17T05:00:51.9137115Z ------------------------------------------
2019-07-17T05:00:51.9137164Z 
2019-07-17T05:00:51.9137393Z ------------------------------------------
2019-07-17T05:00:51.9137436Z stderr:
---
2019-07-17T05:00:52.0618636Z 
2019-07-17T05:00:52.0618688Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:52.0619012Z Actual stderr saved to /tmp/compiletest4X6PS3/issue-36278-prefix-nesting.stderr
2019-07-17T05:00:52.0619092Z To update references, run this command from build directory:
2019-07-17T05:00:52.0619423Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'issue-36278-prefix-nesting.rs'
2019-07-17T05:00:52.0619542Z error: 1 errors occurred comparing output.
2019-07-17T05:00:52.0619594Z status: exit code: 1
2019-07-17T05:00:52.0619594Z status: exit code: 1
2019-07-17T05:00:52.0620393Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-36278-prefix-nesting.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/issue-36278-prefix-nesting.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/issue-36278-prefix-nesting.stage-id.aux" "-A" "unused"
2019-07-17T05:00:52.0621208Z ------------------------------------------
2019-07-17T05:00:52.0621253Z 
2019-07-17T05:00:52.0621533Z ------------------------------------------
2019-07-17T05:00:52.0621585Z stderr:
---
2019-07-17T05:00:52.0742006Z 
2019-07-17T05:00:52.0742059Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:52.0742387Z Actual stderr saved to /tmp/compiletest4X6PS3/issue-3794.stderr
2019-07-17T05:00:52.0742453Z To update references, run this command from build directory:
2019-07-17T05:00:52.0742770Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'issue-3794.rs'
2019-07-17T05:00:52.0742898Z error: 2 errors occurred comparing output.
2019-07-17T05:00:52.0742950Z status: exit code: 1
2019-07-17T05:00:52.0742950Z status: exit code: 1
2019-07-17T05:00:52.0743860Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-3794.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/issue-3794.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/issue-3794.stage-id.aux" "-A" "unused"
2019-07-17T05:00:52.0744402Z ------------------------------------------
2019-07-17T05:00:52.0744462Z 
2019-07-17T05:00:52.0744749Z ------------------------------------------
2019-07-17T05:00:52.0744806Z stderr:
---
2019-07-17T05:00:52.2021349Z 
2019-07-17T05:00:52.2021412Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:52.2021671Z Actual stderr saved to /tmp/compiletest4X6PS3/issue-53728.stderr
2019-07-17T05:00:52.2021723Z To update references, run this command from build directory:
2019-07-17T05:00:52.2021996Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'issue-53728.rs'
2019-07-17T05:00:52.2022074Z error: 1 errors occurred comparing output.
2019-07-17T05:00:52.2022119Z status: exit code: 1
2019-07-17T05:00:52.2022119Z status: exit code: 1
2019-07-17T05:00:52.2022930Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-53728.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/issue-53728.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/issue-53728.stage-id.aux" "-A" "unused"
2019-07-17T05:00:52.2023302Z ------------------------------------------
2019-07-17T05:00:52.2023337Z 
2019-07-17T05:00:52.2023560Z ------------------------------------------
2019-07-17T05:00:52.2023604Z stderr:
---
2019-07-17T05:00:52.2501548Z 
2019-07-17T05:00:52.2501594Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:52.2501869Z Actual stderr saved to /tmp/compiletest4X6PS3/issue-5917.stderr
2019-07-17T05:00:52.2501940Z To update references, run this command from build directory:
2019-07-17T05:00:52.2502220Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'issue-5917.rs'
2019-07-17T05:00:52.2502319Z error: 1 errors occurred comparing output.
2019-07-17T05:00:52.2502374Z status: exit code: 1
2019-07-17T05:00:52.2502374Z status: exit code: 1
2019-07-17T05:00:52.2503180Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-5917.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/issue-5917.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/issue-5917.stage-id.aux" "-A" "unused"
2019-07-17T05:00:52.2503579Z ------------------------------------------
2019-07-17T05:00:52.2503615Z 
2019-07-17T05:00:52.2503990Z ------------------------------------------
2019-07-17T05:00:52.2504035Z stderr:
---
2019-07-17T05:00:52.3382871Z 
2019-07-17T05:00:52.3382918Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:52.3383205Z Actual stderr saved to /tmp/compiletest4X6PS3/issue-miri-184.stderr
2019-07-17T05:00:52.3383275Z To update references, run this command from build directory:
2019-07-17T05:00:52.3383565Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'issue-miri-184.rs'
2019-07-17T05:00:52.3383657Z error: 1 errors occurred comparing output.
2019-07-17T05:00:52.3383718Z status: exit code: 1
2019-07-17T05:00:52.3383718Z status: exit code: 1
2019-07-17T05:00:52.3384785Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-miri-184.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/issue-miri-184.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/issue-miri-184.stage-id.aux" "-A" "unused"
2019-07-17T05:00:52.3385281Z ------------------------------------------
2019-07-17T05:00:52.3385314Z 
2019-07-17T05:00:52.3385679Z ------------------------------------------
2019-07-17T05:00:52.3385724Z stderr:
---
2019-07-17T05:00:52.4737844Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T05:00:52.4737893Z +
2019-07-17T05:00:52.4737918Z 
2019-07-17T05:00:52.4737962Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:52.4738024Z Actual stderr saved to /tmp/compiletest4X6PS3/iter.stderr
2019-07-17T05:00:52.4738072Z To update references, run this command from build directory:
2019-07-17T05:00:52.4738335Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'iter.rs'
2019-07-17T05:00:52.4738426Z error: 1 errors occurred comparing output.
2019-07-17T05:00:52.4738469Z status: exit code: 1
2019-07-17T05:00:52.4738469Z status: exit code: 1
2019-07-17T05:00:52.4739258Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/iter.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/iter.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/iter.stage-id.aux" "-A" "unused"
2019-07-17T05:00:52.4739610Z ------------------------------------------
2019-07-17T05:00:52.4739663Z 
2019-07-17T05:00:52.4739885Z ------------------------------------------
2019-07-17T05:00:52.4739930Z stderr:
---
2019-07-17T05:00:52.4959732Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T05:00:52.4959775Z +
2019-07-17T05:00:52.4959815Z 
2019-07-17T05:00:52.4959856Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:52.4960102Z Actual stderr saved to /tmp/compiletest4X6PS3/last-use-in-cap-clause.stderr
2019-07-17T05:00:52.4960166Z To update references, run this command from build directory:
2019-07-17T05:00:52.4960548Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'last-use-in-cap-clause.rs'
2019-07-17T05:00:52.4960623Z error: 1 errors occurred comparing output.
2019-07-17T05:00:52.4961075Z status: exit code: 1
2019-07-17T05:00:52.4961075Z status: exit code: 1
2019-07-17T05:00:52.4961930Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/last-use-in-cap-clause.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/last-use-in-cap-clause.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/last-use-in-cap-clause.stage-id.aux" "-A" "unused"
2019-07-17T05:00:52.4962310Z ------------------------------------------
2019-07-17T05:00:52.4962343Z 
2019-07-17T05:00:52.4962582Z ------------------------------------------
2019-07-17T05:00:52.4962720Z stderr:
---
2019-07-17T05:00:52.7050329Z 
2019-07-17T05:00:52.7050494Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:52.7050941Z Actual stderr saved to /tmp/compiletest4X6PS3/linked-list.stderr
2019-07-17T05:00:52.7050997Z To update references, run this command from build directory:
2019-07-17T05:00:52.7051353Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'linked-list.rs'
2019-07-17T05:00:52.7051463Z error: 1 errors occurred comparing output.
2019-07-17T05:00:52.7051506Z status: exit code: 1
2019-07-17T05:00:52.7051506Z status: exit code: 1
2019-07-17T05:00:52.7052200Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/linked-list.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/linked-list.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/linked-list.stage-id.aux" "-A" "unused"
2019-07-17T05:00:52.7052624Z ------------------------------------------
2019-07-17T05:00:52.7052674Z 
2019-07-17T05:00:52.7052896Z ------------------------------------------
2019-07-17T05:00:52.7052939Z stderr:
---
2019-07-17T05:00:52.7581220Z 
2019-07-17T05:00:52.7581271Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:52.7581559Z Actual stderr saved to /tmp/compiletest4X6PS3/loop-break-value.stderr
2019-07-17T05:00:52.7581763Z To update references, run this command from build directory:
2019-07-17T05:00:52.7582122Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'loop-break-value.rs'
2019-07-17T05:00:52.7582218Z error: 1 errors occurred comparing output.
2019-07-17T05:00:52.7582292Z status: exit code: 1
2019-07-17T05:00:52.7582292Z status: exit code: 1
2019-07-17T05:00:52.7583067Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/loop-break-value.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/loop-break-value.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/loop-break-value.stage-id.aux" "-A" "unused"
2019-07-17T05:00:52.7583641Z ------------------------------------------
2019-07-17T05:00:52.7583682Z 
2019-07-17T05:00:52.7583987Z ------------------------------------------
2019-07-17T05:00:52.7584042Z stderr:
---
2019-07-17T05:00:52.8778377Z 
2019-07-17T05:00:52.8778429Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:52.8778486Z Actual stderr saved to /tmp/compiletest4X6PS3/main_fn.stderr
2019-07-17T05:00:52.8778561Z To update references, run this command from build directory:
2019-07-17T05:00:52.8778993Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'main_fn.rs'
2019-07-17T05:00:52.8779116Z error: 1 errors occurred comparing output.
2019-07-17T05:00:52.8779168Z status: exit code: 1
2019-07-17T05:00:52.8779168Z status: exit code: 1
2019-07-17T05:00:52.8779937Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/main_fn.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/main_fn.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/main_fn.stage-id.aux" "-A" "unused"
2019-07-17T05:00:52.8780466Z ------------------------------------------
2019-07-17T05:00:52.8780529Z 
2019-07-17T05:00:52.8780866Z ------------------------------------------
2019-07-17T05:00:52.8780921Z stderr:
---
2019-07-17T05:00:52.9067816Z 
2019-07-17T05:00:52.9067858Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:52.9067902Z Actual stderr saved to /tmp/compiletest4X6PS3/loops.stderr
2019-07-17T05:00:52.9068092Z To update references, run this command from build directory:
2019-07-17T05:00:52.9068405Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'loops.rs'
2019-07-17T05:00:52.9068478Z error: 1 errors occurred comparing output.
2019-07-17T05:00:52.9068537Z status: exit code: 1
2019-07-17T05:00:52.9068537Z status: exit code: 1
2019-07-17T05:00:52.9069107Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/loops.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/loops.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/loops.stage-id.aux" "-A" "unused"
2019-07-17T05:00:52.9069540Z ------------------------------------------
2019-07-17T05:00:52.9069572Z 
2019-07-17T05:00:52.9069808Z ------------------------------------------
2019-07-17T05:00:52.9069857Z stderr:
---
2019-07-17T05:00:53.0148887Z 
2019-07-17T05:00:53.0148927Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:53.0149102Z Actual stderr saved to /tmp/compiletest4X6PS3/many_shr_bor.stderr
2019-07-17T05:00:53.0149169Z To update references, run this command from build directory:
2019-07-17T05:00:53.0149479Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'many_shr_bor.rs'
2019-07-17T05:00:53.0149551Z error: 1 errors occurred comparing output.
2019-07-17T05:00:53.0149600Z status: exit code: 1
2019-07-17T05:00:53.0149600Z status: exit code: 1
2019-07-17T05:00:53.0150181Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/many_shr_bor.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/many_shr_bor.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/many_shr_bor.stage-id.aux" "-A" "unused"
2019-07-17T05:00:53.0150742Z ------------------------------------------
2019-07-17T05:00:53.0150772Z 
2019-07-17T05:00:53.0151524Z ------------------------------------------
2019-07-17T05:00:53.0151578Z stderr:
---
2019-07-17T05:00:53.0236556Z 
2019-07-17T05:00:53.0236933Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:53.0237116Z Actual stderr saved to /tmp/compiletest4X6PS3/match_slice.stderr
2019-07-17T05:00:53.0237246Z To update references, run this command from build directory:
2019-07-17T05:00:53.0237646Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'match_slice.rs'
2019-07-17T05:00:53.0237942Z error: 1 errors occurred comparing output.
2019-07-17T05:00:53.0238182Z status: exit code: 1
2019-07-17T05:00:53.0238182Z status: exit code: 1
2019-07-17T05:00:53.0239030Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/match_slice.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/match_slice.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/match_slice.stage-id.aux" "-A" "unused"
2019-07-17T05:00:53.0239789Z ------------------------------------------
2019-07-17T05:00:53.0239949Z 
2019-07-17T05:00:53.0240290Z ------------------------------------------
2019-07-17T05:00:53.0240481Z stderr:
---
2019-07-17T05:00:53.2499938Z 
2019-07-17T05:00:53.2500002Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:53.2500071Z Actual stderr saved to /tmp/compiletest4X6PS3/mir_coercions.stderr
2019-07-17T05:00:53.2500123Z To update references, run this command from build directory:
2019-07-17T05:00:53.2500478Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'mir_coercions.rs'
2019-07-17T05:00:53.2500576Z error: 1 errors occurred comparing output.
2019-07-17T05:00:53.2500623Z status: exit code: 1
2019-07-17T05:00:53.2500623Z status: exit code: 1
2019-07-17T05:00:53.2501907Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/mir_coercions.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/mir_coercions.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/mir_coercions.stage-id.aux" "-A" "unused"
2019-07-17T05:00:53.2502452Z ------------------------------------------
2019-07-17T05:00:53.2502506Z 
2019-07-17T05:00:53.2502734Z ------------------------------------------
2019-07-17T05:00:53.2502780Z stderr:
---
2019-07-17T05:00:53.2538234Z 
2019-07-17T05:00:53.2538423Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:53.2538589Z Actual stderr saved to /tmp/compiletest4X6PS3/memchr.stderr
2019-07-17T05:00:53.2538752Z To update references, run this command from build directory:
2019-07-17T05:00:53.2539383Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'memchr.rs'
2019-07-17T05:00:53.2539749Z error: 1 errors occurred comparing output.
2019-07-17T05:00:53.2539936Z status: exit code: 1
2019-07-17T05:00:53.2539936Z status: exit code: 1
2019-07-17T05:00:53.2545912Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/memchr.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/memchr.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/memchr.stage-id.aux" "-A" "unused"
2019-07-17T05:00:53.2546789Z ------------------------------------------
2019-07-17T05:00:53.2546981Z 
2019-07-17T05:00:53.2551489Z ------------------------------------------
2019-07-17T05:00:53.2551775Z stderr:
---
2019-07-17T05:00:53.3901895Z 
2019-07-17T05:00:53.3901964Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:53.3902301Z Actual stderr saved to /tmp/compiletest4X6PS3/miri-issue-133.stderr
2019-07-17T05:00:53.3902361Z To update references, run this command from build directory:
2019-07-17T05:00:53.3902669Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'miri-issue-133.rs'
2019-07-17T05:00:53.3902764Z error: 1 errors occurred comparing output.
2019-07-17T05:00:53.3902921Z status: exit code: 1
2019-07-17T05:00:53.3902921Z status: exit code: 1
2019-07-17T05:00:53.3903740Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/miri-issue-133.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/miri-issue-133.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/miri-issue-133.stage-id.aux" "-A" "unused"
2019-07-17T05:00:53.3904130Z ------------------------------------------
2019-07-17T05:00:53.3904170Z 
2019-07-17T05:00:53.3904444Z ------------------------------------------
2019-07-17T05:00:53.3904497Z stderr:
---
2019-07-17T05:00:53.4499726Z 
2019-07-17T05:00:53.4499778Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:53.4499852Z Actual stderr saved to /tmp/compiletest4X6PS3/mir_fat_ptr.stderr
2019-07-17T05:00:53.4500009Z To update references, run this command from build directory:
2019-07-17T05:00:53.4500369Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'mir_fat_ptr.rs'
2019-07-17T05:00:53.4500480Z error: 1 errors occurred comparing output.
2019-07-17T05:00:53.4500531Z status: exit code: 1
2019-07-17T05:00:53.4500531Z status: exit code: 1
2019-07-17T05:00:53.4501749Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/mir_fat_ptr.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/mir_fat_ptr.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/mir_fat_ptr.stage-id.aux" "-A" "unused"
2019-07-17T05:00:53.4502141Z ------------------------------------------
2019-07-17T05:00:53.4502180Z 
2019-07-17T05:00:53.4502432Z ------------------------------------------
2019-07-17T05:00:53.4502494Z stderr:
---
2019-07-17T05:00:53.5337163Z 
2019-07-17T05:00:53.5337223Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:53.5337481Z Actual stderr saved to /tmp/compiletest4X6PS3/move-arg-2-unique.stderr
2019-07-17T05:00:53.5337532Z To update references, run this command from build directory:
2019-07-17T05:00:53.5337945Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'move-arg-2-unique.rs'
2019-07-17T05:00:53.5338022Z error: 1 errors occurred comparing output.
2019-07-17T05:00:53.5338064Z status: exit code: 1
2019-07-17T05:00:53.5338064Z status: exit code: 1
2019-07-17T05:00:53.5338708Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/move-arg-2-unique.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/move-arg-2-unique.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/move-arg-2-unique.stage-id.aux" "-A" "unused"
2019-07-17T05:00:53.5339027Z ------------------------------------------
2019-07-17T05:00:53.5339060Z 
2019-07-17T05:00:53.5339284Z ------------------------------------------
2019-07-17T05:00:53.5339345Z stderr:
---
2019-07-17T05:00:53.6224255Z 
2019-07-17T05:00:53.6224322Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:53.6224813Z Actual stderr saved to /tmp/compiletest4X6PS3/move-arg-3-unique.stderr
2019-07-17T05:00:53.6224970Z To update references, run this command from build directory:
2019-07-17T05:00:53.6225274Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'move-arg-3-unique.rs'
2019-07-17T05:00:53.6225348Z error: 1 errors occurred comparing output.
2019-07-17T05:00:53.6225389Z status: exit code: 1
2019-07-17T05:00:53.6225389Z status: exit code: 1
2019-07-17T05:00:53.6226021Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/move-arg-3-unique.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/move-arg-3-unique.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/move-arg-3-unique.stage-id.aux" "-A" "unused"
2019-07-17T05:00:53.6226323Z ------------------------------------------
2019-07-17T05:00:53.6226364Z 
2019-07-17T05:00:53.6226576Z ------------------------------------------
2019-07-17T05:00:53.6226618Z stderr:
---
2019-07-17T05:00:53.6540773Z 
2019-07-17T05:00:53.6540827Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:53.6546216Z Actual stderr saved to /tmp/compiletest4X6PS3/move-undef-primval.stderr
2019-07-17T05:00:53.6546320Z To update references, run this command from build directory:
2019-07-17T05:00:53.6546701Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'move-undef-primval.rs'
2019-07-17T05:00:53.6546800Z error: 1 errors occurred comparing output.
2019-07-17T05:00:53.6546871Z status: exit code: 1
2019-07-17T05:00:53.6546871Z status: exit code: 1
2019-07-17T05:00:53.6551592Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/move-undef-primval.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/move-undef-primval.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/move-undef-primval.stage-id.aux" "-A" "unused"
2019-07-17T05:00:53.6552107Z ------------------------------------------
2019-07-17T05:00:53.6552152Z 
2019-07-17T05:00:53.6552448Z ------------------------------------------
2019-07-17T05:00:53.6552529Z stderr:
---
2019-07-17T05:00:53.7980165Z 
2019-07-17T05:00:53.7980233Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:53.7980287Z Actual stderr saved to /tmp/compiletest4X6PS3/multi_arg_closure.stderr
2019-07-17T05:00:53.7980343Z To update references, run this command from build directory:
2019-07-17T05:00:53.7980709Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'multi_arg_closure.rs'
2019-07-17T05:00:53.7980798Z error: 1 errors occurred comparing output.
2019-07-17T05:00:53.7980864Z status: exit code: 1
2019-07-17T05:00:53.7980864Z status: exit code: 1
2019-07-17T05:00:53.7982049Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/multi_arg_closure.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/multi_arg_closure.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/multi_arg_closure.stage-id.aux" "-A" "unused"
2019-07-17T05:00:53.7982453Z ------------------------------------------
2019-07-17T05:00:53.7982489Z 
2019-07-17T05:00:53.7982740Z ------------------------------------------
2019-07-17T05:00:53.7982806Z stderr:
---
2019-07-17T05:00:53.8620208Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T05:00:53.8620258Z +
2019-07-17T05:00:53.8620305Z 
2019-07-17T05:00:53.8620352Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:53.8620402Z Actual stderr saved to /tmp/compiletest4X6PS3/mpsc.stderr
2019-07-17T05:00:53.8620477Z To update references, run this command from build directory:
2019-07-17T05:00:53.8620892Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'mpsc.rs'
2019-07-17T05:00:53.8620973Z error: 1 errors occurred comparing output.
2019-07-17T05:00:53.8621035Z status: exit code: 1
2019-07-17T05:00:53.8621035Z status: exit code: 1
2019-07-17T05:00:53.8621970Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/mpsc.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/mpsc.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/mpsc.stage-id.aux" "-A" "unused"
2019-07-17T05:00:53.8622328Z ------------------------------------------
2019-07-17T05:00:53.8622363Z 
2019-07-17T05:00:53.8622595Z ------------------------------------------
2019-07-17T05:00:53.8622660Z stderr:
---
2019-07-17T05:00:53.9339699Z 
2019-07-17T05:00:53.9339746Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:53.9339827Z Actual stderr saved to /tmp/compiletest4X6PS3/negative_discriminant.stderr
2019-07-17T05:00:53.9339881Z To update references, run this command from build directory:
2019-07-17T05:00:53.9340184Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'negative_discriminant.rs'
2019-07-17T05:00:53.9340284Z error: 1 errors occurred comparing output.
2019-07-17T05:00:53.9340329Z status: exit code: 1
2019-07-17T05:00:53.9340329Z status: exit code: 1
2019-07-17T05:00:53.9341041Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/negative_discriminant.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/negative_discriminant.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/negative_discriminant.stage-id.aux" "-A" "unused"
2019-07-17T05:00:53.9341696Z ------------------------------------------
2019-07-17T05:00:53.9341755Z 
2019-07-17T05:00:53.9341985Z ------------------------------------------
2019-07-17T05:00:53.9342030Z stderr:
---
2019-07-17T05:00:54.0461815Z 
2019-07-17T05:00:54.0461880Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:54.0461932Z Actual stderr saved to /tmp/compiletest4X6PS3/non_capture_closure_to_fn_ptr.stderr
2019-07-17T05:00:54.0461984Z To update references, run this command from build directory:
2019-07-17T05:00:54.0462283Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'non_capture_closure_to_fn_ptr.rs'
2019-07-17T05:00:54.0462373Z error: 1 errors occurred comparing output.
2019-07-17T05:00:54.0462433Z status: exit code: 1
2019-07-17T05:00:54.0462433Z status: exit code: 1
2019-07-17T05:00:54.0463118Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/non_capture_closure_to_fn_ptr.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/non_capture_closure_to_fn_ptr.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/non_capture_closure_to_fn_ptr.stage-id.aux" "-A" "unused"
2019-07-17T05:00:54.0463442Z ------------------------------------------
2019-07-17T05:00:54.0463475Z 
2019-07-17T05:00:54.0463695Z ------------------------------------------
2019-07-17T05:00:54.0463757Z stderr:
---
2019-07-17T05:00:54.0780907Z 
2019-07-17T05:00:54.0780969Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:54.0781020Z Actual stderr saved to /tmp/compiletest4X6PS3/observed_local_mut.stderr
2019-07-17T05:00:54.0781082Z To update references, run this command from build directory:
2019-07-17T05:00:54.0781766Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'observed_local_mut.rs'
2019-07-17T05:00:54.0781851Z error: 1 errors occurred comparing output.
2019-07-17T05:00:54.0781895Z status: exit code: 1
2019-07-17T05:00:54.0781895Z status: exit code: 1
2019-07-17T05:00:54.0782596Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/observed_local_mut.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/observed_local_mut.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-Zmiri-disable-validation" "-L" "/tmp/compiletest4X6PS3/observed_local_mut.stage-id.aux" "-A" "unused"
2019-07-17T05:00:54.0782924Z ------------------------------------------
2019-07-17T05:00:54.0782966Z 
2019-07-17T05:00:54.0783191Z ------------------------------------------
2019-07-17T05:00:54.0783252Z stderr:
---
2019-07-17T05:00:54.2064333Z 
2019-07-17T05:00:54.2064385Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:54.2064445Z Actual stderr saved to /tmp/compiletest4X6PS3/option_box_transmute_ptr.stderr
2019-07-17T05:00:54.2064520Z To update references, run this command from build directory:
2019-07-17T05:00:54.2064880Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'option_box_transmute_ptr.rs'
2019-07-17T05:00:54.2064991Z error: 1 errors occurred comparing output.
2019-07-17T05:00:54.2065042Z status: exit code: 1
2019-07-17T05:00:54.2065042Z status: exit code: 1
2019-07-17T05:00:54.2065821Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/option_box_transmute_ptr.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/option_box_transmute_ptr.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/option_box_transmute_ptr.stage-id.aux" "-A" "unused"
2019-07-17T05:00:54.2066213Z ------------------------------------------
2019-07-17T05:00:54.2066267Z 
2019-07-17T05:00:54.2066555Z ------------------------------------------
2019-07-17T05:00:54.2066609Z stderr:
---
2019-07-17T05:00:54.2379217Z 
2019-07-17T05:00:54.2379269Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:54.2379337Z Actual stderr saved to /tmp/compiletest4X6PS3/option_eq.stderr
2019-07-17T05:00:54.2379412Z To update references, run this command from build directory:
2019-07-17T05:00:54.2379733Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'option_eq.rs'
2019-07-17T05:00:54.2379835Z error: 1 errors occurred comparing output.
2019-07-17T05:00:54.2379905Z status: exit code: 1
2019-07-17T05:00:54.2379905Z status: exit code: 1
2019-07-17T05:00:54.2380639Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/option_eq.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/option_eq.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/option_eq.stage-id.aux" "-A" "unused"
2019-07-17T05:00:54.2381037Z ------------------------------------------
2019-07-17T05:00:54.2381075Z 
2019-07-17T05:00:54.2381715Z ------------------------------------------
2019-07-17T05:00:54.2381779Z stderr:
---
2019-07-17T05:00:54.3779439Z 
2019-07-17T05:00:54.3779502Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:54.3780093Z Actual stderr saved to /tmp/compiletest4X6PS3/overloaded-calls-simple.stderr
2019-07-17T05:00:54.3780152Z To update references, run this command from build directory:
2019-07-17T05:00:54.3780474Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'overloaded-calls-simple.rs'
2019-07-17T05:00:54.3780554Z error: 1 errors occurred comparing output.
2019-07-17T05:00:54.3780614Z status: exit code: 1
2019-07-17T05:00:54.3780614Z status: exit code: 1
2019-07-17T05:00:54.3781642Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/overloaded-calls-simple.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/overloaded-calls-simple.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/overloaded-calls-simple.stage-id.aux" "-A" "unused"
2019-07-17T05:00:54.3782048Z ------------------------------------------
2019-07-17T05:00:54.3782084Z 
2019-07-17T05:00:54.3782306Z ------------------------------------------
2019-07-17T05:00:54.3782367Z stderr:
---
2019-07-17T05:00:54.3903635Z 
2019-07-17T05:00:54.3903681Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:54.3903740Z Actual stderr saved to /tmp/compiletest4X6PS3/packed_static.stderr
2019-07-17T05:00:54.3903800Z To update references, run this command from build directory:
2019-07-17T05:00:54.3904113Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'packed_static.rs'
2019-07-17T05:00:54.3904195Z error: 1 errors occurred comparing output.
2019-07-17T05:00:54.3904257Z status: exit code: 1
2019-07-17T05:00:54.3904257Z status: exit code: 1
2019-07-17T05:00:54.3904909Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/packed_static.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/packed_static.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/packed_static.stage-id.aux" "-A" "unused"
2019-07-17T05:00:54.3905263Z ------------------------------------------
2019-07-17T05:00:54.3905298Z 
2019-07-17T05:00:54.3905547Z ------------------------------------------
2019-07-17T05:00:54.3905618Z stderr:
---
2019-07-17T05:00:54.5732667Z 
2019-07-17T05:00:54.5732735Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:54.5732803Z Actual stderr saved to /tmp/compiletest4X6PS3/packed_struct.stderr
2019-07-17T05:00:54.5732860Z To update references, run this command from build directory:
2019-07-17T05:00:54.5733183Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'packed_struct.rs'
2019-07-17T05:00:54.5733272Z error: 1 errors occurred comparing output.
2019-07-17T05:00:54.5733320Z status: exit code: 1
2019-07-17T05:00:54.5733320Z status: exit code: 1
2019-07-17T05:00:54.5734053Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/packed_struct.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/packed_struct.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/packed_struct.stage-id.aux" "-A" "unused"
2019-07-17T05:00:54.5734425Z ------------------------------------------
2019-07-17T05:00:54.5734470Z 
2019-07-17T05:00:54.5734724Z ------------------------------------------
2019-07-17T05:00:54.5734774Z stderr:
---
2019-07-17T05:00:54.6329009Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T05:00:54.6355984Z +
2019-07-17T05:00:54.6356206Z 
2019-07-17T05:00:54.6356423Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:54.6356600Z Actual stderr saved to /tmp/compiletest4X6PS3/pointers.stderr
2019-07-17T05:00:54.6356761Z To update references, run this command from build directory:
2019-07-17T05:00:54.6357394Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'pointers.rs'
2019-07-17T05:00:54.6357763Z error: 1 errors occurred comparing output.
2019-07-17T05:00:54.6357941Z status: exit code: 1
2019-07-17T05:00:54.6357941Z status: exit code: 1
2019-07-17T05:00:54.6358833Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/pointers.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/pointers.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/pointers.stage-id.aux" "-A" "unused"
2019-07-17T05:00:54.6359567Z ------------------------------------------
2019-07-17T05:00:54.6359757Z 
2019-07-17T05:00:54.6360201Z ------------------------------------------
2019-07-17T05:00:54.6360400Z stderr:
---
2019-07-17T05:00:54.7618285Z 
2019-07-17T05:00:54.7618348Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:54.7618399Z Actual stderr saved to /tmp/compiletest4X6PS3/products.stderr
2019-07-17T05:00:54.7618449Z To update references, run this command from build directory:
2019-07-17T05:00:54.7618752Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'products.rs'
2019-07-17T05:00:54.7618834Z error: 1 errors occurred comparing output.
2019-07-17T05:00:54.7618880Z status: exit code: 1
2019-07-17T05:00:54.7618880Z status: exit code: 1
2019-07-17T05:00:54.7619541Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/products.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/products.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/products.stage-id.aux" "-A" "unused"
2019-07-17T05:00:54.7619890Z ------------------------------------------
2019-07-17T05:00:54.7619925Z 
2019-07-17T05:00:54.7620165Z ------------------------------------------
2019-07-17T05:00:54.7620212Z stderr:
---
2019-07-17T05:00:54.7690266Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T05:00:54.7690335Z +
2019-07-17T05:00:54.7690363Z 
2019-07-17T05:00:54.7690409Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:54.7690460Z Actual stderr saved to /tmp/compiletest4X6PS3/ptr_arith_offset.stderr
2019-07-17T05:00:54.7690526Z To update references, run this command from build directory:
2019-07-17T05:00:54.7690819Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'ptr_arith_offset.rs'
2019-07-17T05:00:54.7690917Z error: 1 errors occurred comparing output.
2019-07-17T05:00:54.7690963Z status: exit code: 1
2019-07-17T05:00:54.7690963Z status: exit code: 1
2019-07-17T05:00:54.7692049Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/ptr_arith_offset.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/ptr_arith_offset.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/ptr_arith_offset.stage-id.aux" "-A" "unused"
2019-07-17T05:00:54.7692396Z ------------------------------------------
2019-07-17T05:00:54.7692430Z 
2019-07-17T05:00:54.7692669Z ------------------------------------------
2019-07-17T05:00:54.7692713Z stderr:
---
2019-07-17T05:00:54.9162322Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T05:00:54.9162368Z +
2019-07-17T05:00:54.9162395Z 
2019-07-17T05:00:54.9162457Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:54.9162510Z Actual stderr saved to /tmp/compiletest4X6PS3/ptr_arith_offset_overflow.stderr
2019-07-17T05:00:54.9162561Z To update references, run this command from build directory:
2019-07-17T05:00:54.9162855Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'ptr_arith_offset_overflow.rs'
2019-07-17T05:00:54.9162945Z error: 1 errors occurred comparing output.
2019-07-17T05:00:54.9162988Z status: exit code: 1
2019-07-17T05:00:54.9162988Z status: exit code: 1
2019-07-17T05:00:54.9163682Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/ptr_arith_offset_overflow.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/ptr_arith_offset_overflow.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/ptr_arith_offset_overflow.stage-id.aux" "-A" "unused"
2019-07-17T05:00:54.9164010Z ------------------------------------------
2019-07-17T05:00:54.9164043Z 
2019-07-17T05:00:54.9164264Z ------------------------------------------
2019-07-17T05:00:54.9164324Z stderr:
---
2019-07-17T05:00:54.9534668Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T05:00:54.9534715Z +
2019-07-17T05:00:54.9534741Z 
2019-07-17T05:00:54.9534801Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:54.9534851Z Actual stderr saved to /tmp/compiletest4X6PS3/ptr_int_casts.stderr
2019-07-17T05:00:54.9534900Z To update references, run this command from build directory:
2019-07-17T05:00:54.9535193Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'ptr_int_casts.rs'
2019-07-17T05:00:54.9535270Z error: 1 errors occurred comparing output.
2019-07-17T05:00:54.9535314Z status: exit code: 1
2019-07-17T05:00:54.9535314Z status: exit code: 1
2019-07-17T05:00:54.9535979Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/ptr_int_casts.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/ptr_int_casts.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/ptr_int_casts.stage-id.aux" "-A" "unused"
2019-07-17T05:00:54.9536299Z ------------------------------------------
2019-07-17T05:00:54.9536332Z 
2019-07-17T05:00:54.9536669Z ------------------------------------------
2019-07-17T05:00:54.9536735Z stderr:
---
2019-07-17T05:00:55.1065017Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T05:00:55.1065071Z +
2019-07-17T05:00:55.1065096Z 
2019-07-17T05:00:55.1065156Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:55.1065206Z Actual stderr saved to /tmp/compiletest4X6PS3/ptr_int_ops.stderr
2019-07-17T05:00:55.1065368Z To update references, run this command from build directory:
2019-07-17T05:00:55.1065662Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'ptr_int_ops.rs'
2019-07-17T05:00:55.1065741Z error: 1 errors occurred comparing output.
2019-07-17T05:00:55.1065785Z status: exit code: 1
2019-07-17T05:00:55.1065785Z status: exit code: 1
2019-07-17T05:00:55.1066441Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/ptr_int_ops.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/ptr_int_ops.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/ptr_int_ops.stage-id.aux" "-A" "unused"
2019-07-17T05:00:55.1066771Z ------------------------------------------
2019-07-17T05:00:55.1066804Z 
2019-07-17T05:00:55.1067025Z ------------------------------------------
2019-07-17T05:00:55.1067069Z stderr:
---
2019-07-17T05:00:55.1299324Z +
2019-07-17T05:00:55.1299687Z thread '[ui] run-pass/ptr_offset.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
2019-07-17T05:00:55.1299756Z 
2019-07-17T05:00:55.1299801Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:55.1299848Z Actual stderr saved to /tmp/compiletest4X6PS3/ptr_offset.stderr
2019-07-17T05:00:55.1299904Z To update references, run this command from build directory:
2019-07-17T05:00:55.1300192Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'ptr_offset.rs'
2019-07-17T05:00:55.1300267Z error: 1 errors occurred comparing output.
2019-07-17T05:00:55.1300327Z status: exit code: 1
2019-07-17T05:00:55.1300327Z status: exit code: 1
2019-07-17T05:00:55.1300937Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/ptr_offset.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/ptr_offset.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/ptr_offset.stage-id.aux" "-A" "unused"
2019-07-17T05:00:55.1301396Z ------------------------------------------
2019-07-17T05:00:55.1301428Z 
2019-07-17T05:00:55.1302190Z ------------------------------------------
2019-07-17T05:00:55.1302267Z stderr:
---
2019-07-17T05:00:55.2857556Z 
2019-07-17T05:00:55.2857598Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:55.2857661Z Actual stderr saved to /tmp/compiletest4X6PS3/raw.stderr
2019-07-17T05:00:55.2857717Z To update references, run this command from build directory:
2019-07-17T05:00:55.2857979Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'raw.rs'
2019-07-17T05:00:55.2858068Z error: 1 errors occurred comparing output.
2019-07-17T05:00:55.2858110Z status: exit code: 1
2019-07-17T05:00:55.2858110Z status: exit code: 1
2019-07-17T05:00:55.2858699Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/raw.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/raw.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/raw.stage-id.aux" "-A" "unused"
2019-07-17T05:00:55.2859026Z ------------------------------------------
2019-07-17T05:00:55.2859072Z 
2019-07-17T05:00:55.2859298Z ------------------------------------------
2019-07-17T05:00:55.2859429Z stderr:
---
2019-07-17T05:00:55.3860568Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T05:00:55.3860621Z +
2019-07-17T05:00:55.3860646Z 
2019-07-17T05:00:55.3860701Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:55.3860744Z Actual stderr saved to /tmp/compiletest4X6PS3/rc.stderr
2019-07-17T05:00:55.3860786Z To update references, run this command from build directory:
2019-07-17T05:00:55.3861045Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'rc.rs'
2019-07-17T05:00:55.3861119Z error: 1 errors occurred comparing output.
2019-07-17T05:00:55.3861160Z status: exit code: 1
2019-07-17T05:00:55.3861160Z status: exit code: 1
2019-07-17T05:00:55.3862339Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/rc.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/rc.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/rc.stage-id.aux" "-A" "unused"
2019-07-17T05:00:55.3862700Z ------------------------------------------
2019-07-17T05:00:55.3862734Z 
2019-07-17T05:00:55.3862957Z ------------------------------------------
2019-07-17T05:00:55.3863018Z stderr:
---
2019-07-17T05:00:55.4367148Z 
2019-07-17T05:00:55.4367189Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:55.4367240Z Actual stderr saved to /tmp/compiletest4X6PS3/recursive_static.stderr
2019-07-17T05:00:55.4367301Z To update references, run this command from build directory:
2019-07-17T05:00:55.4367556Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'recursive_static.rs'
2019-07-17T05:00:55.4367627Z error: 1 errors occurred comparing output.
2019-07-17T05:00:55.4367686Z status: exit code: 1
2019-07-17T05:00:55.4367686Z status: exit code: 1
2019-07-17T05:00:55.4368269Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/recursive_static.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/recursive_static.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/recursive_static.stage-id.aux" "-A" "unused"
2019-07-17T05:00:55.4368661Z ------------------------------------------
2019-07-17T05:00:55.4368700Z 
2019-07-17T05:00:55.4368947Z ------------------------------------------
2019-07-17T05:00:55.4369009Z stderr:
---
2019-07-17T05:00:55.5220664Z 
2019-07-17T05:00:55.5220718Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:55.5221068Z Actual stderr saved to /tmp/compiletest4X6PS3/ref-invalid-ptr.stderr
2019-07-17T05:00:55.5221143Z To update references, run this command from build directory:
2019-07-17T05:00:55.5221473Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'ref-invalid-ptr.rs'
2019-07-17T05:00:55.5222024Z error: 1 errors occurred comparing output.
2019-07-17T05:00:55.5222078Z status: exit code: 1
2019-07-17T05:00:55.5222078Z status: exit code: 1
2019-07-17T05:00:55.5222920Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/ref-invalid-ptr.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/ref-invalid-ptr.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-Zmiri-disable-validation" "-L" "/tmp/compiletest4X6PS3/ref-invalid-ptr.stage-id.aux" "-A" "unused"
2019-07-17T05:00:55.5223310Z ------------------------------------------
2019-07-17T05:00:55.5223366Z 
2019-07-17T05:00:55.5223788Z ------------------------------------------
2019-07-17T05:00:55.5223854Z stderr:
---
2019-07-17T05:00:55.5897740Z 
2019-07-17T05:00:55.5897792Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:55.5897849Z Actual stderr saved to /tmp/compiletest4X6PS3/refcell.stderr
2019-07-17T05:00:55.5897907Z To update references, run this command from build directory:
2019-07-17T05:00:55.5898249Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'refcell.rs'
2019-07-17T05:00:55.5898339Z error: 1 errors occurred comparing output.
2019-07-17T05:00:55.5898418Z status: exit code: 1
2019-07-17T05:00:55.5898418Z status: exit code: 1
2019-07-17T05:00:55.5899320Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/refcell.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/refcell.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/refcell.stage-id.aux" "-A" "unused"
2019-07-17T05:00:55.5899768Z ------------------------------------------
2019-07-17T05:00:55.5899808Z 
2019-07-17T05:00:55.5900084Z ------------------------------------------
2019-07-17T05:00:55.5900152Z stderr:
---
2019-07-17T05:00:55.6901028Z 
2019-07-17T05:00:55.6901091Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:55.6901446Z Actual stderr saved to /tmp/compiletest4X6PS3/regions-lifetime-nonfree-late-bound.stderr
2019-07-17T05:00:55.6901527Z To update references, run this command from build directory:
2019-07-17T05:00:55.6902260Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'regions-lifetime-nonfree-late-bound.rs'
2019-07-17T05:00:55.6902383Z error: 1 errors occurred comparing output.
2019-07-17T05:00:55.6902433Z status: exit code: 1
2019-07-17T05:00:55.6902433Z status: exit code: 1
2019-07-17T05:00:55.6903405Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/regions-lifetime-nonfree-late-bound.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/regions-lifetime-nonfree-late-bound.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/regions-lifetime-nonfree-late-bound.stage-id.aux" "-A" "unused"
2019-07-17T05:00:55.6903859Z ------------------------------------------
2019-07-17T05:00:55.6903898Z 
2019-07-17T05:00:55.6904172Z ------------------------------------------
2019-07-17T05:00:55.6904223Z stderr:
---
2019-07-17T05:00:55.7378640Z 
2019-07-17T05:00:55.7378709Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:55.7379016Z Actual stderr saved to /tmp/compiletest4X6PS3/regions-mock-trans.stderr
2019-07-17T05:00:55.7379089Z To update references, run this command from build directory:
2019-07-17T05:00:55.7379429Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'regions-mock-trans.rs'
2019-07-17T05:00:55.7379520Z error: 1 errors occurred comparing output.
2019-07-17T05:00:55.7379571Z status: exit code: 1
2019-07-17T05:00:55.7379571Z status: exit code: 1
2019-07-17T05:00:55.7380514Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/regions-mock-trans.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/regions-mock-trans.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/regions-mock-trans.stage-id.aux" "-A" "unused"
2019-07-17T05:00:55.7380961Z ------------------------------------------
2019-07-17T05:00:55.7381105Z 
2019-07-17T05:00:55.7381419Z ------------------------------------------
2019-07-17T05:00:55.7381493Z stderr:
---
2019-07-17T05:00:55.8657414Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T05:00:55.8657463Z +
2019-07-17T05:00:55.8657491Z 
2019-07-17T05:00:55.8657535Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:55.8657598Z Actual stderr saved to /tmp/compiletest4X6PS3/rfc1623.stderr
2019-07-17T05:00:55.8657645Z To update references, run this command from build directory:
2019-07-17T05:00:55.8657935Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'rfc1623.rs'
2019-07-17T05:00:55.8658026Z error: 1 errors occurred comparing output.
2019-07-17T05:00:55.8658068Z status: exit code: 1
2019-07-17T05:00:55.8658068Z status: exit code: 1
2019-07-17T05:00:55.8658840Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/rfc1623.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/rfc1623.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/rfc1623.stage-id.aux" "-A" "unused"
2019-07-17T05:00:55.8659229Z ------------------------------------------
2019-07-17T05:00:55.8659263Z 
2019-07-17T05:00:55.8659493Z ------------------------------------------
2019-07-17T05:00:55.8659537Z stderr:
---
2019-07-17T05:00:55.9177117Z 
2019-07-17T05:00:55.9177179Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:55.9177435Z Actual stderr saved to /tmp/compiletest4X6PS3/rust-lang-org.stderr
2019-07-17T05:00:55.9177488Z To update references, run this command from build directory:
2019-07-17T05:00:55.9177765Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'rust-lang-org.rs'
2019-07-17T05:00:55.9177942Z error: 1 errors occurred comparing output.
2019-07-17T05:00:55.9177986Z status: exit code: 1
2019-07-17T05:00:55.9177986Z status: exit code: 1
2019-07-17T05:00:55.9178688Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/rust-lang-org.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/rust-lang-org.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/rust-lang-org.stage-id.aux" "-A" "unused"
2019-07-17T05:00:55.9179129Z ------------------------------------------
2019-07-17T05:00:55.9179163Z 
2019-07-17T05:00:55.9179386Z ------------------------------------------
2019-07-17T05:00:55.9179450Z stderr:
---
2019-07-17T05:00:56.0381102Z 
2019-07-17T05:00:56.0381150Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:56.0381728Z Actual stderr saved to /tmp/compiletest4X6PS3/send-is-not-static-par-for.stderr
2019-07-17T05:00:56.0381800Z To update references, run this command from build directory:
2019-07-17T05:00:56.0382304Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'send-is-not-static-par-for.rs'
2019-07-17T05:00:56.0382426Z error: 1 errors occurred comparing output.
2019-07-17T05:00:56.0382470Z status: exit code: 1
2019-07-17T05:00:56.0382470Z status: exit code: 1
2019-07-17T05:00:56.0383214Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/send-is-not-static-par-for.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/send-is-not-static-par-for.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/send-is-not-static-par-for.stage-id.aux" "-A" "unused"
2019-07-17T05:00:56.0383684Z ------------------------------------------
2019-07-17T05:00:56.0383718Z 
2019-07-17T05:00:56.0383953Z ------------------------------------------
2019-07-17T05:00:56.0383998Z stderr:
---
2019-07-17T05:00:56.0484142Z 
2019-07-17T05:00:56.0484186Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:56.0484553Z Actual stderr saved to /tmp/compiletest4X6PS3/sendable-class.stderr
2019-07-17T05:00:56.0484639Z To update references, run this command from build directory:
2019-07-17T05:00:56.0484944Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'sendable-class.rs'
2019-07-17T05:00:56.0485040Z error: 1 errors occurred comparing output.
2019-07-17T05:00:56.0485087Z status: exit code: 1
2019-07-17T05:00:56.0485087Z status: exit code: 1
2019-07-17T05:00:56.0485836Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/sendable-class.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/sendable-class.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/sendable-class.stage-id.aux" "-A" "unused"
2019-07-17T05:00:56.0486266Z ------------------------------------------
2019-07-17T05:00:56.0486299Z 
2019-07-17T05:00:56.0486528Z ------------------------------------------
2019-07-17T05:00:56.0486568Z stderr:
---
2019-07-17T05:00:56.2214332Z 
2019-07-17T05:00:56.2214414Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:56.2214757Z Actual stderr saved to /tmp/compiletest4X6PS3/simd-intrinsic-generic-elements.stderr
2019-07-17T05:00:56.2214937Z To update references, run this command from build directory:
2019-07-17T05:00:56.2215334Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'simd-intrinsic-generic-elements.rs'
2019-07-17T05:00:56.2215408Z error: 1 errors occurred comparing output.
2019-07-17T05:00:56.2215449Z status: exit code: 1
2019-07-17T05:00:56.2215449Z status: exit code: 1
2019-07-17T05:00:56.2216183Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/simd-intrinsic-generic-elements.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/simd-intrinsic-generic-elements.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/simd-intrinsic-generic-elements.stage-id.aux" "-A" "unused"
2019-07-17T05:00:56.2216610Z ------------------------------------------
2019-07-17T05:00:56.2216641Z 
2019-07-17T05:00:56.2216832Z ------------------------------------------
2019-07-17T05:00:56.2216887Z stderr:
---
2019-07-17T05:00:56.3578514Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T05:00:56.3578690Z +
2019-07-17T05:00:56.3578725Z 
2019-07-17T05:00:56.3578786Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:56.3578833Z Actual stderr saved to /tmp/compiletest4X6PS3/small_enum_size_bug.stderr
2019-07-17T05:00:56.3578879Z To update references, run this command from build directory:
2019-07-17T05:00:56.3579209Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'small_enum_size_bug.rs'
2019-07-17T05:00:56.3579282Z error: 1 errors occurred comparing output.
2019-07-17T05:00:56.3579335Z status: exit code: 1
2019-07-17T05:00:56.3579335Z status: exit code: 1
2019-07-17T05:00:56.3579941Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/small_enum_size_bug.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/small_enum_size_bug.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/small_enum_size_bug.stage-id.aux" "-A" "unused"
2019-07-17T05:00:56.3580377Z ------------------------------------------
2019-07-17T05:00:56.3580409Z 
2019-07-17T05:00:56.3580620Z ------------------------------------------
2019-07-17T05:00:56.3580676Z stderr:
---
2019-07-17T05:00:56.4244651Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T05:00:56.4244703Z +
2019-07-17T05:00:56.4244733Z 
2019-07-17T05:00:56.4244796Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:56.4244850Z Actual stderr saved to /tmp/compiletest4X6PS3/slices.stderr
2019-07-17T05:00:56.4244901Z To update references, run this command from build directory:
2019-07-17T05:00:56.4245310Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'slices.rs'
2019-07-17T05:00:56.4245399Z error: 1 errors occurred comparing output.
2019-07-17T05:00:56.4245553Z status: exit code: 1
2019-07-17T05:00:56.4245553Z status: exit code: 1
2019-07-17T05:00:56.4246335Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/slices.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/slices.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/slices.stage-id.aux" "-A" "unused"
2019-07-17T05:00:56.4246653Z ------------------------------------------
2019-07-17T05:00:56.4246685Z 
2019-07-17T05:00:56.4246899Z ------------------------------------------
2019-07-17T05:00:56.4246942Z stderr:
---
2019-07-17T05:00:56.5427740Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T05:00:56.5427795Z +
2019-07-17T05:00:56.5427828Z 
2019-07-17T05:00:56.5427899Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:56.5427960Z Actual stderr saved to /tmp/compiletest4X6PS3/specialization.stderr
2019-07-17T05:00:56.5428144Z To update references, run this command from build directory:
2019-07-17T05:00:56.5428532Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'specialization.rs'
2019-07-17T05:00:56.5428626Z error: 1 errors occurred comparing output.
2019-07-17T05:00:56.5428676Z status: exit code: 1
2019-07-17T05:00:56.5428676Z status: exit code: 1
2019-07-17T05:00:56.5429496Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/specialization.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/specialization.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/specialization.stage-id.aux" "-A" "unused"
2019-07-17T05:00:56.5429882Z ------------------------------------------
2019-07-17T05:00:56.5429921Z 
2019-07-17T05:00:56.5430192Z ------------------------------------------
2019-07-17T05:00:56.5430273Z stderr:
---
2019-07-17T05:00:56.6096064Z 
2019-07-17T05:00:56.6096109Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:56.6096390Z Actual stderr saved to /tmp/compiletest4X6PS3/stacked-borrows/2phase.stderr
2019-07-17T05:00:56.6096461Z To update references, run this command from build directory:
2019-07-17T05:00:56.6096885Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'stacked-borrows/2phase.rs'
2019-07-17T05:00:56.6096984Z error: 1 errors occurred comparing output.
2019-07-17T05:00:56.6097030Z status: exit code: 1
2019-07-17T05:00:56.6097030Z status: exit code: 1
2019-07-17T05:00:56.6097717Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/stacked-borrows/2phase.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/stacked-borrows/2phase.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/stacked-borrows/2phase.stage-id.aux" "-A" "unused"
2019-07-17T05:00:56.6098057Z ------------------------------------------
2019-07-17T05:00:56.6098091Z 
2019-07-17T05:00:56.6098346Z ------------------------------------------
2019-07-17T05:00:56.6098403Z stderr:
---
2019-07-17T05:00:56.7441867Z 
2019-07-17T05:00:56.7441921Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:56.7442792Z Actual stderr saved to /tmp/compiletest4X6PS3/stacked-borrows/interior_mutability.stderr
2019-07-17T05:00:56.7443051Z To update references, run this command from build directory:
2019-07-17T05:00:56.7443454Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'stacked-borrows/interior_mutability.rs'
2019-07-17T05:00:56.7443555Z error: 1 errors occurred comparing output.
2019-07-17T05:00:56.7443639Z status: exit code: 1
2019-07-17T05:00:56.7443639Z status: exit code: 1
2019-07-17T05:00:56.7444460Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/stacked-borrows/interior_mutability.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/stacked-borrows/interior_mutability.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/stacked-borrows/interior_mutability.stage-id.aux" "-A" "unused"
2019-07-17T05:00:56.7444876Z ------------------------------------------
2019-07-17T05:00:56.7444919Z 
2019-07-17T05:00:56.7445286Z ------------------------------------------
2019-07-17T05:00:56.7445343Z stderr:
---
2019-07-17T05:00:56.8752794Z 
2019-07-17T05:00:56.8752848Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:56.8753122Z Actual stderr saved to /tmp/compiletest4X6PS3/stacked-borrows/stacked-borrows.stderr
2019-07-17T05:00:56.8753193Z To update references, run this command from build directory:
2019-07-17T05:00:56.8753467Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'stacked-borrows/stacked-borrows.rs'
2019-07-17T05:00:56.8753560Z error: 1 errors occurred comparing output.
2019-07-17T05:00:56.8753606Z status: exit code: 1
2019-07-17T05:00:56.8753606Z status: exit code: 1
2019-07-17T05:00:56.8754296Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/stacked-borrows/stacked-borrows.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/stacked-borrows/stacked-borrows.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/stacked-borrows/stacked-borrows.stage-id.aux" "-A" "unused"
2019-07-17T05:00:56.8754631Z ------------------------------------------
2019-07-17T05:00:56.8754665Z 
2019-07-17T05:00:56.8754903Z ------------------------------------------
2019-07-17T05:00:56.8754947Z stderr:
---
2019-07-17T05:00:56.9064646Z 
2019-07-17T05:00:56.9064702Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:56.9064773Z Actual stderr saved to /tmp/compiletest4X6PS3/static_memory_modification.stderr
2019-07-17T05:00:56.9064827Z To update references, run this command from build directory:
2019-07-17T05:00:56.9065130Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'static_memory_modification.rs'
2019-07-17T05:00:56.9065229Z error: 1 errors occurred comparing output.
2019-07-17T05:00:56.9065275Z status: exit code: 1
2019-07-17T05:00:56.9065275Z status: exit code: 1
2019-07-17T05:00:56.9065993Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/static_memory_modification.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/static_memory_modification.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/static_memory_modification.stage-id.aux" "-A" "unused"
2019-07-17T05:00:56.9066347Z ------------------------------------------
2019-07-17T05:00:56.9066382Z 
2019-07-17T05:00:56.9066623Z ------------------------------------------
2019-07-17T05:00:56.9066670Z stderr:
---
2019-07-17T05:00:57.0549152Z 
2019-07-17T05:00:57.0549204Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:57.0549281Z Actual stderr saved to /tmp/compiletest4X6PS3/static_mut.stderr
2019-07-17T05:00:57.0550275Z To update references, run this command from build directory:
2019-07-17T05:00:57.0554100Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'static_mut.rs'
2019-07-17T05:00:57.0554235Z error: 1 errors occurred comparing output.
2019-07-17T05:00:57.0554301Z status: exit code: 1
2019-07-17T05:00:57.0554301Z status: exit code: 1
2019-07-17T05:00:57.0555350Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/static_mut.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/static_mut.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/static_mut.stage-id.aux" "-A" "unused"
2019-07-17T05:00:57.0555767Z ------------------------------------------
2019-07-17T05:00:57.0555808Z 
2019-07-17T05:00:57.0556062Z ------------------------------------------
2019-07-17T05:00:57.0556111Z stderr:
---
2019-07-17T05:00:57.0707407Z 
2019-07-17T05:00:57.0707459Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:57.0707532Z Actual stderr saved to /tmp/compiletest4X6PS3/strings.stderr
2019-07-17T05:00:57.0707590Z To update references, run this command from build directory:
2019-07-17T05:00:57.0707916Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'strings.rs'
2019-07-17T05:00:57.0708036Z error: 1 errors occurred comparing output.
2019-07-17T05:00:57.0708088Z status: exit code: 1
2019-07-17T05:00:57.0708088Z status: exit code: 1
2019-07-17T05:00:57.0708827Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/strings.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/strings.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/strings.stage-id.aux" "-A" "unused"
2019-07-17T05:00:57.0709206Z ------------------------------------------
2019-07-17T05:00:57.0724218Z 
2019-07-17T05:00:57.0724672Z ------------------------------------------
2019-07-17T05:00:57.0724727Z stderr:
---
2019-07-17T05:00:57.2221566Z 
2019-07-17T05:00:57.2221613Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:57.2221681Z Actual stderr saved to /tmp/compiletest4X6PS3/subslice_array.stderr
2019-07-17T05:00:57.2221735Z To update references, run this command from build directory:
2019-07-17T05:00:57.2222395Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'subslice_array.rs'
2019-07-17T05:00:57.2222516Z error: 1 errors occurred comparing output.
2019-07-17T05:00:57.2222562Z status: exit code: 1
2019-07-17T05:00:57.2222562Z status: exit code: 1
2019-07-17T05:00:57.2223243Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/subslice_array.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/subslice_array.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/subslice_array.stage-id.aux" "-A" "unused"
2019-07-17T05:00:57.2223594Z ------------------------------------------
2019-07-17T05:00:57.2223645Z 
2019-07-17T05:00:57.2223891Z ------------------------------------------
2019-07-17T05:00:57.2223938Z stderr:
---
2019-07-17T05:00:57.3071330Z 
2019-07-17T05:00:57.3071377Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:57.3071435Z Actual stderr saved to /tmp/compiletest4X6PS3/sums.stderr
2019-07-17T05:00:57.3071503Z To update references, run this command from build directory:
2019-07-17T05:00:57.3071778Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'sums.rs'
2019-07-17T05:00:57.3071991Z error: 1 errors occurred comparing output.
2019-07-17T05:00:57.3072044Z status: exit code: 1
2019-07-17T05:00:57.3072044Z status: exit code: 1
2019-07-17T05:00:57.3073051Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/sums.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/sums.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/sums.stage-id.aux" "-A" "unused"
2019-07-17T05:00:57.3073390Z ------------------------------------------
2019-07-17T05:00:57.3073438Z 
2019-07-17T05:00:57.3073681Z ------------------------------------------
2019-07-17T05:00:57.3073724Z stderr:
---
2019-07-17T05:00:57.3920022Z 
2019-07-17T05:00:57.3920066Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:57.3920115Z Actual stderr saved to /tmp/compiletest4X6PS3/sync.stderr
2019-07-17T05:00:57.3920180Z To update references, run this command from build directory:
2019-07-17T05:00:57.3920488Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'sync.rs'
2019-07-17T05:00:57.3920580Z error: 1 errors occurred comparing output.
2019-07-17T05:00:57.3920624Z status: exit code: 1
2019-07-17T05:00:57.3920624Z status: exit code: 1
2019-07-17T05:00:57.3921251Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/sync.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/sync.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/sync.stage-id.aux" "-A" "unused"
2019-07-17T05:00:57.3921583Z ------------------------------------------
2019-07-17T05:00:57.3921617Z 
2019-07-17T05:00:57.3922347Z ------------------------------------------
2019-07-17T05:00:57.3922403Z stderr:
---
2019-07-17T05:00:57.4578137Z 
2019-07-17T05:00:57.4578178Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:57.4578429Z Actual stderr saved to /tmp/compiletest4X6PS3/tag-align-dyn-u64.stderr
2019-07-17T05:00:57.4578486Z To update references, run this command from build directory:
2019-07-17T05:00:57.4578751Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'tag-align-dyn-u64.rs'
2019-07-17T05:00:57.4578823Z error: 1 errors occurred comparing output.
2019-07-17T05:00:57.4578879Z status: exit code: 1
2019-07-17T05:00:57.4578879Z status: exit code: 1
2019-07-17T05:00:57.4579492Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/tag-align-dyn-u64.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/tag-align-dyn-u64.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/tag-align-dyn-u64.stage-id.aux" "-A" "unused"
2019-07-17T05:00:57.4579801Z ------------------------------------------
2019-07-17T05:00:57.4579832Z 
2019-07-17T05:00:57.4580051Z ------------------------------------------
2019-07-17T05:00:57.4580110Z stderr:
---
2019-07-17T05:00:57.5867505Z 
2019-07-17T05:00:57.5867557Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:57.5867905Z Actual stderr saved to /tmp/compiletest4X6PS3/too-large-primval-write-problem.stderr
2019-07-17T05:00:57.5867974Z To update references, run this command from build directory:
2019-07-17T05:00:57.5868335Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'too-large-primval-write-problem.rs'
2019-07-17T05:00:57.5868441Z error: 1 errors occurred comparing output.
2019-07-17T05:00:57.5868515Z status: exit code: 1
2019-07-17T05:00:57.5868515Z status: exit code: 1
2019-07-17T05:00:57.5869319Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/too-large-primval-write-problem.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/too-large-primval-write-problem.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/too-large-primval-write-problem.stage-id.aux" "-A" "unused"
2019-07-17T05:00:57.5869740Z ------------------------------------------
2019-07-17T05:00:57.5869783Z 
2019-07-17T05:00:57.5870080Z ------------------------------------------
2019-07-17T05:00:57.5870137Z stderr:
---
2019-07-17T05:00:57.5888408Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T05:00:57.5888469Z +
2019-07-17T05:00:57.5888500Z 
2019-07-17T05:00:57.5888551Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:57.5888878Z Actual stderr saved to /tmp/compiletest4X6PS3/thread-local.stderr
2019-07-17T05:00:57.5888956Z To update references, run this command from build directory:
2019-07-17T05:00:57.5889279Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'thread-local.rs'
2019-07-17T05:00:57.5889467Z error: 1 errors occurred comparing output.
2019-07-17T05:00:57.5889519Z status: exit code: 1
2019-07-17T05:00:57.5889519Z status: exit code: 1
2019-07-17T05:00:57.5890309Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/thread-local.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/thread-local.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/thread-local.stage-id.aux" "-A" "unused"
2019-07-17T05:00:57.5890733Z ------------------------------------------
2019-07-17T05:00:57.5890795Z 
2019-07-17T05:00:57.5891099Z ------------------------------------------
2019-07-17T05:00:57.5891167Z stderr:
---
2019-07-17T05:00:57.7445956Z 
2019-07-17T05:00:57.7446000Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:57.7446057Z Actual stderr saved to /tmp/compiletest4X6PS3/transmute_fat.stderr
2019-07-17T05:00:57.7446121Z To update references, run this command from build directory:
2019-07-17T05:00:57.7446396Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'transmute_fat.rs'
2019-07-17T05:00:57.7446470Z error: 1 errors occurred comparing output.
2019-07-17T05:00:57.7446527Z status: exit code: 1
2019-07-17T05:00:57.7446527Z status: exit code: 1
2019-07-17T05:00:57.7447216Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/transmute_fat.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/transmute_fat.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-Zmiri-disable-validation" "-L" "/tmp/compiletest4X6PS3/transmute_fat.stage-id.aux" "-A" "unused"
2019-07-17T05:00:57.7447546Z ------------------------------------------
2019-07-17T05:00:57.7455845Z 
2019-07-17T05:00:57.7456329Z ------------------------------------------
2019-07-17T05:00:57.7456382Z stderr:
---
2019-07-17T05:00:57.7946106Z 
2019-07-17T05:00:57.7946150Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:57.7946196Z Actual stderr saved to /tmp/compiletest4X6PS3/traits.stderr
2019-07-17T05:00:57.7946382Z To update references, run this command from build directory:
2019-07-17T05:00:57.7946722Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'traits.rs'
2019-07-17T05:00:57.7946804Z error: 1 errors occurred comparing output.
2019-07-17T05:00:57.7946850Z status: exit code: 1
2019-07-17T05:00:57.7946850Z status: exit code: 1
2019-07-17T05:00:57.7947404Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/traits.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/traits.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/traits.stage-id.aux" "-A" "unused"
2019-07-17T05:00:57.7947686Z ------------------------------------------
2019-07-17T05:00:57.7947715Z 
2019-07-17T05:00:57.7947922Z ------------------------------------------
2019-07-17T05:00:57.7947961Z stderr:
---
2019-07-17T05:00:57.8741600Z 
2019-07-17T05:00:57.8741659Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:57.8741709Z Actual stderr saved to /tmp/compiletest4X6PS3/trivial.stderr
2019-07-17T05:00:57.8741758Z To update references, run this command from build directory:
2019-07-17T05:00:57.8742161Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'trivial.rs'
2019-07-17T05:00:57.8742250Z error: 1 errors occurred comparing output.
2019-07-17T05:00:57.8742295Z status: exit code: 1
2019-07-17T05:00:57.8742295Z status: exit code: 1
2019-07-17T05:00:57.8743261Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/trivial.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/trivial.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/trivial.stage-id.aux" "-A" "unused"
2019-07-17T05:00:57.8743595Z ------------------------------------------
2019-07-17T05:00:57.8743629Z 
2019-07-17T05:00:57.8743850Z ------------------------------------------
2019-07-17T05:00:57.8743895Z stderr:
---
2019-07-17T05:00:57.9476207Z 
2019-07-17T05:00:57.9476247Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:57.9476488Z Actual stderr saved to /tmp/compiletest4X6PS3/try-operator-custom.stderr
2019-07-17T05:00:57.9476553Z To update references, run this command from build directory:
2019-07-17T05:00:57.9476800Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'try-operator-custom.rs'
2019-07-17T05:00:57.9476889Z error: 1 errors occurred comparing output.
2019-07-17T05:00:57.9476929Z status: exit code: 1
2019-07-17T05:00:57.9476929Z status: exit code: 1
2019-07-17T05:00:57.9477556Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/try-operator-custom.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/try-operator-custom.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/try-operator-custom.stage-id.aux" "-A" "unused"
2019-07-17T05:00:57.9477872Z ------------------------------------------
2019-07-17T05:00:57.9477903Z 
2019-07-17T05:00:57.9478126Z ------------------------------------------
2019-07-17T05:00:57.9478167Z stderr:
---
2019-07-17T05:00:58.0856917Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T05:00:58.0856968Z +
2019-07-17T05:00:58.0857012Z 
2019-07-17T05:00:58.0857059Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:58.0857113Z Actual stderr saved to /tmp/compiletest4X6PS3/tuple_like_enum_variant_constructor.stderr
2019-07-17T05:00:58.0857182Z To update references, run this command from build directory:
2019-07-17T05:00:58.0857490Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'tuple_like_enum_variant_constructor.rs'
2019-07-17T05:00:58.0857582Z error: 1 errors occurred comparing output.
2019-07-17T05:00:58.0857644Z status: exit code: 1
2019-07-17T05:00:58.0857644Z status: exit code: 1
2019-07-17T05:00:58.0858369Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/tuple_like_enum_variant_constructor.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/tuple_like_enum_variant_constructor.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/tuple_like_enum_variant_constructor.stage-id.aux" "-A" "unused"
2019-07-17T05:00:58.0858718Z ------------------------------------------
2019-07-17T05:00:58.0858752Z 
2019-07-17T05:00:58.0859009Z ------------------------------------------
2019-07-17T05:00:58.0859057Z stderr:
---
2019-07-17T05:00:58.1333990Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T05:00:58.1334036Z +
2019-07-17T05:00:58.1334063Z 
2019-07-17T05:00:58.1334122Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:58.1334176Z Actual stderr saved to /tmp/compiletest4X6PS3/tuple_like_enum_variant_constructor_pointer_opt.stderr
2019-07-17T05:00:58.1334237Z To update references, run this command from build directory:
2019-07-17T05:00:58.1334549Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'tuple_like_enum_variant_constructor_pointer_opt.rs'
2019-07-17T05:00:58.1334629Z error: 1 errors occurred comparing output.
2019-07-17T05:00:58.1334697Z status: exit code: 1
2019-07-17T05:00:58.1334697Z status: exit code: 1
2019-07-17T05:00:58.1335432Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/tuple_like_enum_variant_constructor_pointer_opt.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/tuple_like_enum_variant_constructor_pointer_opt.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/tuple_like_enum_variant_constructor_pointer_opt.stage-id.aux" "-A" "unused"
2019-07-17T05:00:58.1335897Z ------------------------------------------
2019-07-17T05:00:58.1336050Z 
2019-07-17T05:00:58.1336256Z ------------------------------------------
2019-07-17T05:00:58.1336417Z stderr:
---
2019-07-17T05:00:58.2942012Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T05:00:58.2942073Z +
2019-07-17T05:00:58.2942127Z 
2019-07-17T05:00:58.2942180Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:58.2942241Z Actual stderr saved to /tmp/compiletest4X6PS3/tuple_like_struct_constructor.stderr
2019-07-17T05:00:58.2942332Z To update references, run this command from build directory:
2019-07-17T05:00:58.2943190Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'tuple_like_struct_constructor.rs'
2019-07-17T05:00:58.2943301Z error: 1 errors occurred comparing output.
2019-07-17T05:00:58.2943380Z status: exit code: 1
2019-07-17T05:00:58.2943380Z status: exit code: 1
2019-07-17T05:00:58.2944189Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/tuple_like_struct_constructor.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/tuple_like_struct_constructor.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/tuple_like_struct_constructor.stage-id.aux" "-A" "unused"
2019-07-17T05:00:58.2944618Z ------------------------------------------
2019-07-17T05:00:58.2944796Z 
2019-07-17T05:00:58.2945167Z ------------------------------------------
2019-07-17T05:00:58.2945224Z stderr:
---
2019-07-17T05:00:58.3104700Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T05:00:58.3104763Z +
2019-07-17T05:00:58.3104798Z 
2019-07-17T05:00:58.3104854Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:58.3104945Z Actual stderr saved to /tmp/compiletest4X6PS3/tuple_like_enum_variant_constructor_struct_pointer_opt.stderr
2019-07-17T05:00:58.3105009Z To update references, run this command from build directory:
2019-07-17T05:00:58.3105410Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'tuple_like_enum_variant_constructor_struct_pointer_opt.rs'
2019-07-17T05:00:58.3105550Z error: 1 errors occurred comparing output.
2019-07-17T05:00:58.3105603Z status: exit code: 1
2019-07-17T05:00:58.3105603Z status: exit code: 1
2019-07-17T05:00:58.3106758Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/tuple_like_enum_variant_constructor_struct_pointer_opt.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/tuple_like_enum_variant_constructor_struct_pointer_opt.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/tuple_like_enum_variant_constructor_struct_pointer_opt.stage-id.aux" "-A" "unused"
2019-07-17T05:00:58.3107280Z ------------------------------------------
2019-07-17T05:00:58.3107326Z 
2019-07-17T05:00:58.3107632Z ------------------------------------------
2019-07-17T05:00:58.3107845Z stderr:
---
2019-07-17T05:00:58.4777934Z 
2019-07-17T05:00:58.4777992Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:58.4778240Z Actual stderr saved to /tmp/compiletest4X6PS3/union-overwrite.stderr
2019-07-17T05:00:58.4778291Z To update references, run this command from build directory:
2019-07-17T05:00:58.4778567Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'union-overwrite.rs'
2019-07-17T05:00:58.4778640Z error: 1 errors occurred comparing output.
2019-07-17T05:00:58.4778697Z status: exit code: 1
2019-07-17T05:00:58.4778697Z status: exit code: 1
2019-07-17T05:00:58.4779434Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/union-overwrite.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/union-overwrite.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/union-overwrite.stage-id.aux" "-A" "unused"
2019-07-17T05:00:58.4779808Z ------------------------------------------
2019-07-17T05:00:58.4779841Z 
2019-07-17T05:00:58.4780049Z ------------------------------------------
2019-07-17T05:00:58.4780205Z stderr:
---
2019-07-17T05:00:58.6607928Z 
2019-07-17T05:00:58.6607972Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:58.6608038Z Actual stderr saved to /tmp/compiletest4X6PS3/union.stderr
2019-07-17T05:00:58.6608095Z To update references, run this command from build directory:
2019-07-17T05:00:58.6608363Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'union.rs'
2019-07-17T05:00:58.6608455Z error: 1 errors occurred comparing output.
2019-07-17T05:00:58.6608498Z status: exit code: 1
2019-07-17T05:00:58.6608498Z status: exit code: 1
2019-07-17T05:00:58.6609248Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/union.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/union.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/union.stage-id.aux" "-A" "unused"
2019-07-17T05:00:58.6609619Z ------------------------------------------
2019-07-17T05:00:58.6609671Z 
2019-07-17T05:00:58.6610034Z ------------------------------------------
2019-07-17T05:00:58.6610080Z stderr:
---
2019-07-17T05:00:58.7217325Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T05:00:58.7217368Z +
2019-07-17T05:00:58.7217408Z 
2019-07-17T05:00:58.7217449Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:58.7217501Z Actual stderr saved to /tmp/compiletest4X6PS3/u128.stderr
2019-07-17T05:00:58.7217562Z To update references, run this command from build directory:
2019-07-17T05:00:58.7217810Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'u128.rs'
2019-07-17T05:00:58.7217879Z error: 1 errors occurred comparing output.
2019-07-17T05:00:58.7218020Z status: exit code: 1
2019-07-17T05:00:58.7218020Z status: exit code: 1
2019-07-17T05:00:58.7218625Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/u128.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/u128.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/u128.stage-id.aux" "-A" "unused"
2019-07-17T05:00:58.7218925Z ------------------------------------------
2019-07-17T05:00:58.7219039Z 
2019-07-17T05:00:58.7219286Z ------------------------------------------
2019-07-17T05:00:58.7219344Z stderr:
---
2019-07-17T05:00:58.8421676Z 
2019-07-17T05:00:58.8421731Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:58.8421806Z Actual stderr saved to /tmp/compiletest4X6PS3/unops.stderr
2019-07-17T05:00:58.8421883Z To update references, run this command from build directory:
2019-07-17T05:00:58.8422258Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'unops.rs'
2019-07-17T05:00:58.8422534Z error: 1 errors occurred comparing output.
2019-07-17T05:00:58.8423019Z status: exit code: 1
2019-07-17T05:00:58.8423019Z status: exit code: 1
2019-07-17T05:00:58.8423879Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/unops.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/unops.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/unops.stage-id.aux" "-A" "unused"
2019-07-17T05:00:58.8424294Z ------------------------------------------
2019-07-17T05:00:58.8424507Z 
2019-07-17T05:00:58.8424871Z ------------------------------------------
2019-07-17T05:00:58.8424933Z stderr:
---
2019-07-17T05:00:58.9015709Z 
2019-07-17T05:00:58.9015769Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:58.9016021Z Actual stderr saved to /tmp/compiletest4X6PS3/unsized-tuple-impls.stderr
2019-07-17T05:00:58.9016162Z To update references, run this command from build directory:
2019-07-17T05:00:58.9016491Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'unsized-tuple-impls.rs'
2019-07-17T05:00:58.9016569Z error: 1 errors occurred comparing output.
2019-07-17T05:00:58.9016613Z status: exit code: 1
2019-07-17T05:00:58.9016613Z status: exit code: 1
2019-07-17T05:00:58.9017283Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/unsized-tuple-impls.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/unsized-tuple-impls.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/unsized-tuple-impls.stage-id.aux" "-A" "unused"
2019-07-17T05:00:58.9017724Z ------------------------------------------
2019-07-17T05:00:58.9017757Z 
2019-07-17T05:00:58.9017987Z ------------------------------------------
2019-07-17T05:00:58.9018050Z stderr:
---
2019-07-17T05:00:59.0037715Z 
2019-07-17T05:00:59.0037778Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:59.0037833Z Actual stderr saved to /tmp/compiletest4X6PS3/validation_lifetime_resolution.stderr
2019-07-17T05:00:59.0038071Z To update references, run this command from build directory:
2019-07-17T05:00:59.0038466Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'validation_lifetime_resolution.rs'
2019-07-17T05:00:59.0038551Z error: 1 errors occurred comparing output.
2019-07-17T05:00:59.0038597Z status: exit code: 1
2019-07-17T05:00:59.0038597Z status: exit code: 1
2019-07-17T05:00:59.0039319Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/validation_lifetime_resolution.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/validation_lifetime_resolution.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/validation_lifetime_resolution.stage-id.aux" "-A" "unused"
2019-07-17T05:00:59.0039836Z ------------------------------------------
2019-07-17T05:00:59.0039872Z 
2019-07-17T05:00:59.0040237Z ------------------------------------------
2019-07-17T05:00:59.0040297Z stderr:
---
2019-07-17T05:00:59.1061674Z 
2019-07-17T05:00:59.1061821Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:59.1062139Z Actual stderr saved to /tmp/compiletest4X6PS3/vec-matching-fold.stderr
2019-07-17T05:00:59.1062194Z To update references, run this command from build directory:
2019-07-17T05:00:59.1062613Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'vec-matching-fold.rs'
2019-07-17T05:00:59.1062695Z error: 1 errors occurred comparing output.
2019-07-17T05:00:59.1063316Z status: exit code: 1
2019-07-17T05:00:59.1063316Z status: exit code: 1
2019-07-17T05:00:59.1064135Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/vec-matching-fold.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/vec-matching-fold.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/vec-matching-fold.stage-id.aux" "-A" "unused"
2019-07-17T05:00:59.1064665Z ------------------------------------------
2019-07-17T05:00:59.1064701Z 
2019-07-17T05:00:59.1064926Z ------------------------------------------
2019-07-17T05:00:59.1064988Z stderr:
---
2019-07-17T05:00:59.1941851Z 
2019-07-17T05:00:59.1941915Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:59.1941967Z Actual stderr saved to /tmp/compiletest4X6PS3/vecdeque.stderr
2019-07-17T05:00:59.1942030Z To update references, run this command from build directory:
2019-07-17T05:00:59.1942329Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'vecdeque.rs'
2019-07-17T05:00:59.1942409Z error: 2 errors occurred comparing output.
2019-07-17T05:00:59.1942481Z status: exit code: 1
2019-07-17T05:00:59.1942481Z status: exit code: 1
2019-07-17T05:00:59.1945073Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/vecdeque.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/vecdeque.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/vecdeque.stage-id.aux" "-A" "unused"
2019-07-17T05:00:59.1945462Z ------------------------------------------
2019-07-17T05:00:59.1945497Z 
2019-07-17T05:00:59.1945722Z ------------------------------------------
2019-07-17T05:00:59.1945784Z stderr:
---
2019-07-17T05:00:59.3622963Z 
2019-07-17T05:00:59.3623167Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:59.3623311Z Actual stderr saved to /tmp/compiletest4X6PS3/volatile.stderr
2019-07-17T05:00:59.3623470Z To update references, run this command from build directory:
2019-07-17T05:00:59.3624006Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'volatile.rs'
2019-07-17T05:00:59.3624343Z error: 1 errors occurred comparing output.
2019-07-17T05:00:59.3624485Z status: exit code: 1
2019-07-17T05:00:59.3624485Z status: exit code: 1
2019-07-17T05:00:59.3625335Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/volatile.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/volatile.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/volatile.stage-id.aux" "-A" "unused"
2019-07-17T05:00:59.3626111Z ------------------------------------------
2019-07-17T05:00:59.3626732Z 
2019-07-17T05:00:59.3627196Z ------------------------------------------
2019-07-17T05:00:59.3635428Z stderr:
---
2019-07-17T05:00:59.3983067Z 
2019-07-17T05:00:59.3983206Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:59.3983362Z Actual stderr saved to /tmp/compiletest4X6PS3/vecs.stderr
2019-07-17T05:00:59.3983506Z To update references, run this command from build directory:
2019-07-17T05:00:59.3983937Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'vecs.rs'
2019-07-17T05:00:59.3984389Z error: 1 errors occurred comparing output.
2019-07-17T05:00:59.3984543Z status: exit code: 1
2019-07-17T05:00:59.3984543Z status: exit code: 1
2019-07-17T05:00:59.3985365Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/vecs.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/vecs.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/vecs.stage-id.aux" "-A" "unused"
2019-07-17T05:00:59.3986071Z ------------------------------------------
2019-07-17T05:00:59.3986249Z 
2019-07-17T05:00:59.3986640Z ------------------------------------------
2019-07-17T05:00:59.3986851Z stderr:
---
2019-07-17T05:00:59.5807192Z 
2019-07-17T05:00:59.5807274Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:59.5807617Z Actual stderr saved to /tmp/compiletest4X6PS3/without-validation.stderr
2019-07-17T05:00:59.5807677Z To update references, run this command from build directory:
2019-07-17T05:00:59.5807988Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'without-validation.rs'
2019-07-17T05:00:59.5808076Z error: 1 errors occurred comparing output.
2019-07-17T05:00:59.5808125Z status: exit code: 1
2019-07-17T05:00:59.5808125Z status: exit code: 1
2019-07-17T05:00:59.5808902Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/without-validation.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/without-validation.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-Zmiri-disable-validation" "-L" "/tmp/compiletest4X6PS3/without-validation.stage-id.aux" "-A" "unused"
2019-07-17T05:00:59.5809293Z ------------------------------------------
2019-07-17T05:00:59.5809330Z 
2019-07-17T05:00:59.5809578Z ------------------------------------------
2019-07-17T05:00:59.5809644Z stderr:
---
2019-07-17T05:00:59.6181050Z 
2019-07-17T05:00:59.6181215Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:59.6181671Z Actual stderr saved to /tmp/compiletest4X6PS3/write-bytes.stderr
2019-07-17T05:00:59.6181873Z To update references, run this command from build directory:
2019-07-17T05:00:59.6182326Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'write-bytes.rs'
2019-07-17T05:00:59.6183001Z error: 1 errors occurred comparing output.
2019-07-17T05:00:59.6183167Z status: exit code: 1
2019-07-17T05:00:59.6183167Z status: exit code: 1
2019-07-17T05:00:59.6184043Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/write-bytes.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/write-bytes.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/write-bytes.stage-id.aux" "-A" "unused"
2019-07-17T05:00:59.6184678Z ------------------------------------------
2019-07-17T05:00:59.6184848Z 
2019-07-17T05:00:59.6185249Z ------------------------------------------
2019-07-17T05:00:59.6235513Z stderr:
---
2019-07-17T05:00:59.7856523Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T05:00:59.7856595Z +
2019-07-17T05:00:59.7856626Z 
2019-07-17T05:00:59.7856677Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:59.7856746Z Actual stderr saved to /tmp/compiletest4X6PS3/zst.stderr
2019-07-17T05:00:59.7856802Z To update references, run this command from build directory:
2019-07-17T05:00:59.7857101Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'zst.rs'
2019-07-17T05:00:59.7857203Z error: 1 errors occurred comparing output.
2019-07-17T05:00:59.7857253Z status: exit code: 1
2019-07-17T05:00:59.7857253Z status: exit code: 1
2019-07-17T05:00:59.7857948Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/zst.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/zst.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/zst.stage-id.aux" "-A" "unused"
2019-07-17T05:00:59.7858309Z ------------------------------------------
2019-07-17T05:00:59.7858347Z 
2019-07-17T05:00:59.7858612Z ------------------------------------------
2019-07-17T05:00:59.7858662Z stderr:
---
2019-07-17T05:00:59.7946648Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T05:00:59.7946702Z +
2019-07-17T05:00:59.7946732Z 
2019-07-17T05:00:59.7946782Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:59.7947114Z Actual stderr saved to /tmp/compiletest4X6PS3/zero-sized-binary-heap-push.stderr
2019-07-17T05:00:59.7947176Z To update references, run this command from build directory:
2019-07-17T05:00:59.7947502Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'zero-sized-binary-heap-push.rs'
2019-07-17T05:00:59.7947626Z error: 1 errors occurred comparing output.
2019-07-17T05:00:59.7947678Z status: exit code: 1
2019-07-17T05:00:59.7947678Z status: exit code: 1
2019-07-17T05:00:59.7948482Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/zero-sized-binary-heap-push.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/zero-sized-binary-heap-push.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/zero-sized-binary-heap-push.stage-id.aux" "-A" "unused"
2019-07-17T05:00:59.7948871Z ------------------------------------------
2019-07-17T05:00:59.7948910Z 
2019-07-17T05:00:59.7949180Z ------------------------------------------
2019-07-17T05:00:59.7949232Z stderr:
---
2019-07-17T05:00:59.9456960Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T05:00:59.9457013Z +
2019-07-17T05:00:59.9457043Z 
2019-07-17T05:00:59.9457107Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:59.9457160Z Actual stderr saved to /tmp/compiletest4X6PS3/zst_box.stderr
2019-07-17T05:00:59.9457211Z To update references, run this command from build directory:
2019-07-17T05:00:59.9457539Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'zst_box.rs'
2019-07-17T05:00:59.9457621Z error: 1 errors occurred comparing output.
2019-07-17T05:00:59.9457668Z status: exit code: 1
2019-07-17T05:00:59.9457668Z status: exit code: 1
2019-07-17T05:00:59.9458341Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/zst_box.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/zst_box.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/zst_box.stage-id.aux" "-A" "unused"
2019-07-17T05:00:59.9458684Z ------------------------------------------
2019-07-17T05:00:59.9458718Z 
2019-07-17T05:00:59.9458962Z ------------------------------------------
2019-07-17T05:00:59.9459009Z stderr:
---
2019-07-17T05:00:59.9509120Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T05:00:59.9509171Z +
2019-07-17T05:00:59.9509198Z 
2019-07-17T05:00:59.9509243Z The actual stderr differed from the expected stderr.
2019-07-17T05:00:59.9509312Z Actual stderr saved to /tmp/compiletest4X6PS3/zst_variant_drop.stderr
2019-07-17T05:00:59.9509374Z To update references, run this command from build directory:
2019-07-17T05:00:59.9509667Z tests/run-pass/update-references.sh '/tmp/compiletest4X6PS3' 'zst_variant_drop.rs'
2019-07-17T05:00:59.9509768Z error: 1 errors occurred comparing output.
2019-07-17T05:00:59.9509814Z status: exit code: 1
2019-07-17T05:00:59.9509814Z status: exit code: 1
2019-07-17T05:00:59.9510486Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/zst_variant_drop.rs" "-L" "/tmp/compiletest4X6PS3" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4X6PS3/zst_variant_drop.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest4X6PS3/zst_variant_drop.stage-id.aux" "-A" "unused"
2019-07-17T05:00:59.9510873Z ------------------------------------------
2019-07-17T05:00:59.9510929Z 
2019-07-17T05:00:59.9511171Z ------------------------------------------
2019-07-17T05:00:59.9511217Z stderr:
---
2019-07-17T05:00:59.9911093Z Verifying status of miri...
2019-07-17T05:00:59.9924881Z Verifying status of embedded-book...
2019-07-17T05:00:59.9937674Z This PR updated 'src/doc/embedded-book', verifying if status is 'test-pass'...
2019-07-17T05:00:59.9947696Z Verifying status of rustc-guide...
2019-07-17T05:01:00.0057952Z /tmp/checktools.sh: 38: /tmp/checktools.sh: TOOLSTATE_REPO: parameter not set
2019-07-17T05:01:00.7693014Z ##[error]Bash exited with code '2'.
2019-07-17T05:01:00.7726460Z ##[section]Starting: Checkout
2019-07-17T05:01:00.7728416Z ==============================================================================
2019-07-17T05:01:00.7728489Z Task         : Get sources
2019-07-17T05:01:00.7728531Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
