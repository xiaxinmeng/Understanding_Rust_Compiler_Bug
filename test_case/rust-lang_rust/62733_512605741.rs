plain
2019-07-17T19:47:24.3331805Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-17T19:47:24.3520048Z ##[command]git config gc.auto 0
2019-07-17T19:47:24.3584749Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-17T19:47:24.3636435Z ##[command]git config --get-all http.proxy
2019-07-17T19:47:24.3785462Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62733/merge:refs/remotes/pull/62733/merge
---
2019-07-17T19:47:59.4145247Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-17T19:47:59.4146595Z 
2019-07-17T19:47:59.4148562Z   git checkout -b <new-branch-name>
2019-07-17T19:47:59.4150392Z 
2019-07-17T19:47:59.4151514Z HEAD is now at a051b3683 Merge 04538c680c7a18b9e9a8419456aa3c6f82e2403b into 2eb0bc5e3c52a34b6d62ab0527520c66e4c575bd
2019-07-17T19:47:59.4291188Z ##[section]Finishing: Checkout
2019-07-17T19:47:59.4299597Z ##[section]Starting: Decide whether to run this job
2019-07-17T19:47:59.4302817Z Task         : Bash
2019-07-17T19:47:59.4302866Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-17T19:47:59.4302918Z Version      : 3.151.2
2019-07-17T19:47:59.4302985Z Author       : Microsoft Corporation
2019-07-17T19:47:59.4302985Z Author       : Microsoft Corporation
2019-07-17T19:47:59.4303039Z Help         : https://docs.microsoft.com/azure/devops/pipelines/tasks/utility/bash
2019-07-17T19:47:59.4303095Z ==============================================================================
2019-07-17T19:47:59.5680265Z Generating script.
2019-07-17T19:47:59.5715718Z ========================== Starting Command Output ===========================
2019-07-17T19:47:59.5734761Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/1d476812-0647-4b38-888c-c65f7d0fe1bf.sh
2019-07-17T19:47:59.7742380Z Executing the job since submodules are updated
2019-07-17T19:47:59.7831545Z ##[section]Finishing: Decide whether to run this job
2019-07-17T19:47:59.7841265Z ==============================================================================
2019-07-17T19:47:59.7841358Z Task         : Bash
2019-07-17T19:47:59.7841397Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-17T19:47:59.7841435Z Version      : 3.151.2
---
2019-07-17T19:49:39.9339623Z Attempting with retry: curl -y 30 -Y 10 --connect-timeout 30 -f -L -C - -o /tmp/rustci_docker_cache https://.s3.amazonaws.com/docker/5237541d5b559bcea2f168e4ed3efe600aec9344996d80ee3a8ea6b1b5d4d12b46df9cf61ac5cec12b0346064f67d51e8664869834273098748b7174157de4e3
2019-07-17T19:49:39.9402662Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-17T19:49:39.9402982Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-17T19:49:39.9403021Z 
2019-07-17T19:49:39.9445743Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-17T19:49:40.9516941Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-17T19:49:40.9517288Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-17T19:49:40.9517813Z 
2019-07-17T19:49:40.9517813Z 
2019-07-17T19:49:40.9560453Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-17T19:49:42.9638695Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-17T19:49:42.9638781Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-17T19:49:42.9638850Z 
2019-07-17T19:49:42.9638850Z 
2019-07-17T19:49:42.9672238Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-17T19:49:45.9749848Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-17T19:49:45.9750033Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-17T19:49:45.9750075Z 
2019-07-17T19:49:45.9750075Z 
2019-07-17T19:49:45.9783053Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-17T19:49:49.9848220Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-17T19:49:49.9848672Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-17T19:49:49.9848901Z 
2019-07-17T19:49:49.9848901Z 
2019-07-17T19:49:49.9891390Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-17T19:49:49.9895470Z The command has failed after 5 attempts.
2019-07-17T19:49:50.0260023Z open /tmp/rustci_docker_cache: no such file or directory
2019-07-17T19:49:50.0294751Z Attempting with retry: docker build --rm -t rust-ci -f /home/vsts/work/1/s/src/ci/docker/x86_64-gnu-tools/Dockerfile /home/vsts/work/1/s/src/ci/docker
2019-07-17T19:49:50.2873527Z Sending build context to Docker daemon  521.2kB
2019-07-17T19:49:50.2874098Z 
2019-07-17T19:49:50.3109364Z Step 1/9 : FROM ubuntu:16.04
---
2019-07-17T19:50:21.7713291Z Reading package lists...
2019-07-17T19:50:22.7434415Z Reading package lists...
2019-07-17T19:50:22.9090594Z Building dependency tree...
2019-07-17T19:50:22.9091230Z Reading state information...
2019-07-17T19:50:23.0280576Z The following additional packages will be installed:
2019-07-17T19:50:23.0306731Z   binutils bzip2 cmake-data cpp cpp-5 dpkg-dev g++-5 gcc gcc-5 git-man
2019-07-17T19:50:23.0307093Z   libarchive13 libasan2 libasn1-8-heimdal libatomic1 libbz2-1.0 libc-dev-bin
2019-07-17T19:50:23.0340577Z   liberror-perl libexpat1 libffi6 libgcc-5-dev libgdbm3 libglib2.0-0 libgmp10
2019-07-17T19:50:23.0340864Z   libgnutls30 libgomp1 libgssapi-krb5-2 libgssapi3-heimdal libhcrypto4-heimdal
2019-07-17T19:50:23.0341543Z   libheimbase1-heimdal libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal
2019-07-17T19:50:23.0341607Z   libicu55 libidn11 libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1
---
2019-07-17T19:50:23.0343180Z   libssl1.0.0 libstdc++-5-dev libtasn1-6 libtsan0 libubsan0 libwind0-heimdal
2019-07-17T19:50:23.0343446Z   libxml2 linux-libc-dev mime-support openssl patch perl perl-modules-5.22
2019-07-17T19:50:23.0343661Z   python2.7-minimal zlib1g-dev
2019-07-17T19:50:23.0343853Z Suggested packages:
2019-07-17T19:50:23.0344126Z   binutils-doc bzip2-doc codeblocks eclipse ninja-build cpp-doc gcc-5-locales
2019-07-17T19:50:23.0344387Z   debian-keyring g++-multilib g++-5-multilib gcc-5-doc libstdc++6-5-dbg
2019-07-17T19:50:23.0344682Z   gcc-multilib manpages-dev autoconf automake libtool flex bison gdb gcc-doc
2019-07-17T19:50:23.0345371Z   libasan2-dbg liblsan0-dbg libtsan0-dbg libubsan0-dbg libcilkrts5-dbg
2019-07-17T19:50:23.0345656Z   libmpx0-dbg libquadmath0-dbg gettext-base git-daemon-run
2019-07-17T19:50:23.0345656Z   libmpx0-dbg libquadmath0-dbg gettext-base git-daemon-run
2019-07-17T19:50:23.0345951Z   | git-daemon-sysvinit git-doc git-el git-email git-gui gitk gitweb git-arch
2019-07-17T19:50:23.0346416Z   git-cvs git-mediawiki git-svn lrzip glibc-doc gnutls-bin krb5-doc krb5-user
2019-07-17T19:50:23.0346865Z   libstdc++-5-doc make-doc ed diffutils-doc perl-doc libterm-readline-gnu-perl
2019-07-17T19:50:23.0347097Z   | libterm-readline-perl-perl python2.7-doc binfmt-support
2019-07-17T19:50:23.0347145Z Recommended packages:
2019-07-17T19:50:23.0347394Z   build-essential fakeroot libalgorithm-merge-perl less rsync ssh-client
2019-07-17T19:50:23.0347638Z   manpages manpages-dev libfile-fcntllock-perl libglib2.0-data
2019-07-17T19:50:23.0347877Z   shared-mime-info xdg-user-dirs krb5-locales libsasl2-modules libssl-doc
2019-07-17T19:50:23.0348095Z   xml-core netbase rename
2019-07-17T19:50:23.0348153Z The following NEW packages will be installed:
2019-07-17T19:50:23.0348602Z   binutils bzip2 ca-certificates cmake cmake-data cpp cpp-5 curl dpkg-dev file
2019-07-17T19:50:23.0349266Z   g++ g++-5 gcc gcc-5 git git-man libarchive13 libasan2 libasn1-8-heimdal
2019-07-17T19:50:23.0349847Z   libcurl3-gnutls libdpkg-perl liberror-perl libexpat1 libffi6 libgcc-5-dev
2019-07-17T19:50:23.0350149Z   libgdbm3 libglib2.0-0 libgmp10 libgnutls30 libgomp1 libgssapi-krb5-2
2019-07-17T19:50:23.0350408Z   libgssapi3-heimdal libhcrypto4-heimdal libheimbase1-heimdal
2019-07-17T19:50:23.0350671Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-17T19:50:23.0350671Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-17T19:50:23.0350994Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-17T19:50:23.0351643Z   libkrb5-3 libkrb5support0 libldap-2.4-2 liblsan0 liblzo2-2 libmagic1 libmpc3
2019-07-17T19:50:23.0352095Z   libmpfr4 libmpx0 libnettle6 libp11-kit0 libperl5.22 libpython2.7-minimal
2019-07-17T19:50:23.0352609Z   libpython2.7-stdlib libquadmath0 libroken18-heimdal librtmp1 libsasl2-2
2019-07-17T19:50:23.0352878Z   libsasl2-modules-db libsqlite3-0 libssl-dev libssl1.0.0 libstdc++-5-dev
2019-07-17T19:50:23.0353134Z   libtasn1-6 libtsan0 libubsan0 libwind0-heimdal libxml2 linux-libc-dev make
2019-07-17T19:50:23.0353596Z   mime-support openssl patch perl perl-modules-5.22 pkg-config python2.7
2019-07-17T19:50:23.0353841Z   python2.7-minimal sudo xz-utils zlib1g-dev
2019-07-17T19:50:23.0353895Z The following packages will be upgraded:
2019-07-17T19:50:23.4190567Z 1 upgraded, 92 newly installed, 0 to remove and 5 not upgraded.
2019-07-17T19:50:23.4193832Z Need to get 71.6 MB of archives.
2019-07-17T19:50:23.4194767Z After this operation, 310 MB of additional disk space will be used.
2019-07-17T19:50:23.4195686Z Get:1 http://archive.ubuntu.com/ubuntu xenial/main amd64 libgdbm3 amd64 1.8.3-13.1 [16.9 kB]
---
2019-07-17T19:52:58.0982721Z Removing intermediate container 26b4aa7c33ac
2019-07-17T19:52:58.0983626Z  ---> a1dd479437d1
2019-07-17T19:52:58.1019125Z Successfully built a1dd479437d1
2019-07-17T19:52:58.2935115Z Successfully tagged rust-ci:latest
2019-07-17T19:52:58.3391367Z Built container sha256:a1dd479437d1c7bbc700fa61db1926d16be2e2172a3eae7e6f1b5b10c9217178
2019-07-17T19:52:58.3410590Z Uploading finished image to https://.s3.amazonaws.com/docker/5237541d5b559bcea2f168e4ed3efe600aec9344996d80ee3a8ea6b1b5d4d12b46df9cf61ac5cec12b0346064f67d51e8664869834273098748b7174157de4e3
2019-07-17T19:53:38.3187317Z upload failed: - to s3:///docker/5237541d5b559bcea2f168e4ed3efe600aec9344996d80ee3a8ea6b1b5d4d12b46df9cf61ac5cec12b0346064f67d51e8664869834273098748b7174157de4e3 Parameter validation failed:
2019-07-17T19:53:38.3189662Z Invalid bucket name "": Bucket name must match the regex "^[a-zA-Z0-9.\-_]{1,255}$"
2019-07-17T19:53:39.4416918Z [CI_JOB_NAME=LinuxTools]
2019-07-17T19:53:39.4462206Z Starting sccache server...
2019-07-17T19:53:39.4940500Z configure: processing command line
2019-07-17T19:53:39.4940597Z configure: 
---
2019-07-17T23:28:00.8309575Z -args
2019-07-17T23:28:00.8309943Z -
2019-07-17T23:28:00.8311568Z 
2019-07-17T23:28:00.8311741Z The actual stdout differed from the expected stdout.
2019-07-17T23:28:00.8311899Z Actual stdout saved to /tmp/compiletestNloeDC/args.stdout
2019-07-17T23:28:00.8312137Z error[E0080]: Miri evaluation error: attempted to do invalid arithmetic on pointers that would leak base addresses, e.g., comparing pointers into different allocations
2019-07-17T23:28:00.8312539Z     --> /checkout/src/libcore/intrinsics.rs:1340:19
2019-07-17T23:28:00.8312702Z      |
2019-07-17T23:28:00.8312824Z 1340 |     let diff = if src_usize > dst_usize {
---
2019-07-17T23:28:00.8320512Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:00.8320702Z +
2019-07-17T23:28:00.8320786Z 
2019-07-17T23:28:00.8320886Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:00.8321007Z Actual stderr saved to /tmp/compiletestNloeDC/args.stderr
2019-07-17T23:28:00.8321111Z To update references, run this command from build directory:
2019-07-17T23:28:00.8321433Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'args.rs'
2019-07-17T23:28:00.8321683Z error: 2 errors occurred comparing output.
2019-07-17T23:28:00.8321782Z status: exit code: 1
2019-07-17T23:28:00.8321782Z status: exit code: 1
2019-07-17T23:28:00.8322505Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/args.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/args.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/args.stage-id.aux" "-A" "unused"
2019-07-17T23:28:00.8323108Z ------------------------------------------
2019-07-17T23:28:00.8323239Z 
2019-07-17T23:28:00.8323514Z ------------------------------------------
2019-07-17T23:28:00.8323666Z stderr:
---
2019-07-17T23:28:00.8798875Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:00.8798941Z +
2019-07-17T23:28:00.8798969Z 
2019-07-17T23:28:00.8799014Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:00.8799082Z Actual stderr saved to /tmp/compiletestNloeDC/arrays.stderr
2019-07-17T23:28:00.8799132Z To update references, run this command from build directory:
2019-07-17T23:28:00.8799382Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'arrays.rs'
2019-07-17T23:28:00.8799474Z error: 1 errors occurred comparing output.
2019-07-17T23:28:00.8799597Z status: exit code: 1
2019-07-17T23:28:00.8799597Z status: exit code: 1
2019-07-17T23:28:00.8800605Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/arrays.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/arrays.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/arrays.stage-id.aux" "-A" "unused"
2019-07-17T23:28:00.8801200Z ------------------------------------------
2019-07-17T23:28:00.8801229Z 
2019-07-17T23:28:00.8801406Z ------------------------------------------
2019-07-17T23:28:00.8801445Z stderr:
---
2019-07-17T23:28:00.9882011Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:00.9882053Z +
2019-07-17T23:28:00.9882091Z 
2019-07-17T23:28:00.9882130Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:00.9882352Z Actual stderr saved to /tmp/compiletestNloeDC/associated-const.stderr
2019-07-17T23:28:00.9882397Z To update references, run this command from build directory:
2019-07-17T23:28:00.9882757Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'associated-const.rs'
2019-07-17T23:28:00.9882826Z error: 1 errors occurred comparing output.
2019-07-17T23:28:00.9882884Z status: exit code: 1
2019-07-17T23:28:00.9882884Z status: exit code: 1
2019-07-17T23:28:00.9883413Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/associated-const.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/associated-const.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/associated-const.stage-id.aux" "-A" "unused"
2019-07-17T23:28:00.9883686Z ------------------------------------------
2019-07-17T23:28:00.9883715Z 
2019-07-17T23:28:00.9883902Z ------------------------------------------
2019-07-17T23:28:00.9883963Z stderr:
---
2019-07-17T23:28:00.9972059Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:00.9972120Z +
2019-07-17T23:28:00.9972144Z 
2019-07-17T23:28:00.9972182Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:00.9972225Z Actual stderr saved to /tmp/compiletestNloeDC/assume_bug.stderr
2019-07-17T23:28:00.9972284Z To update references, run this command from build directory:
2019-07-17T23:28:00.9972620Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'assume_bug.rs'
2019-07-17T23:28:00.9972708Z error: 1 errors occurred comparing output.
2019-07-17T23:28:00.9972746Z status: exit code: 1
2019-07-17T23:28:00.9972746Z status: exit code: 1
2019-07-17T23:28:00.9973448Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/assume_bug.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/assume_bug.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/assume_bug.stage-id.aux" "-A" "unused"
2019-07-17T23:28:00.9973713Z ------------------------------------------
2019-07-17T23:28:00.9973742Z 
2019-07-17T23:28:00.9973931Z ------------------------------------------
2019-07-17T23:28:00.9973969Z stderr:
---
2019-07-17T23:28:01.1429916Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:01.1429956Z +
2019-07-17T23:28:01.1429978Z 
2019-07-17T23:28:01.1430022Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:01.1430255Z Actual stderr saved to /tmp/compiletestNloeDC/atomic-access-bool.stderr
2019-07-17T23:28:01.1430301Z To update references, run this command from build directory:
2019-07-17T23:28:01.1430519Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'atomic-access-bool.rs'
2019-07-17T23:28:01.1430599Z error: 1 errors occurred comparing output.
2019-07-17T23:28:01.1430636Z status: exit code: 1
2019-07-17T23:28:01.1430636Z status: exit code: 1
2019-07-17T23:28:01.1431180Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/atomic-access-bool.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/atomic-access-bool.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/atomic-access-bool.stage-id.aux" "-A" "unused"
2019-07-17T23:28:01.1431541Z ------------------------------------------
2019-07-17T23:28:01.1431591Z 
2019-07-17T23:28:01.1431780Z ------------------------------------------
2019-07-17T23:28:01.1431819Z stderr:
---
2019-07-17T23:28:01.1441673Z +
2019-07-17T23:28:01.1441953Z thread '[ui] run-pass/async-fn.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
2019-07-17T23:28:01.1442053Z 
2019-07-17T23:28:01.1442108Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:01.1442336Z Actual stderr saved to /tmp/compiletestNloeDC/async-fn.stderr
2019-07-17T23:28:01.1442382Z To update references, run this command from build directory:
2019-07-17T23:28:01.1442619Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'async-fn.rs'
2019-07-17T23:28:01.1442685Z error: 1 errors occurred comparing output.
2019-07-17T23:28:01.1442721Z status: exit code: 1
2019-07-17T23:28:01.1442721Z status: exit code: 1
2019-07-17T23:28:01.1443236Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/async-fn.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/async-fn.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/async-fn.stage-id.aux" "-A" "unused"
2019-07-17T23:28:01.1443516Z ------------------------------------------
2019-07-17T23:28:01.1443545Z 
2019-07-17T23:28:01.1443732Z ------------------------------------------
2019-07-17T23:28:01.1443787Z stderr:
---
2019-07-17T23:28:01.2674288Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:01.2674931Z +
2019-07-17T23:28:01.2674965Z 
2019-07-17T23:28:01.2675151Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:01.2675204Z Actual stderr saved to /tmp/compiletestNloeDC/bad_substs.stderr
2019-07-17T23:28:01.2675256Z To update references, run this command from build directory:
2019-07-17T23:28:01.2675642Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'bad_substs.rs'
2019-07-17T23:28:01.2675728Z error: 1 errors occurred comparing output.
2019-07-17T23:28:01.2675774Z status: exit code: 1
2019-07-17T23:28:01.2675774Z status: exit code: 1
2019-07-17T23:28:01.2676446Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/bad_substs.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/bad_substs.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/bad_substs.stage-id.aux" "-A" "unused"
2019-07-17T23:28:01.2676789Z ------------------------------------------
2019-07-17T23:28:01.2676825Z 
2019-07-17T23:28:01.2677051Z ------------------------------------------
2019-07-17T23:28:01.2677100Z stderr:
---
2019-07-17T23:28:01.3245227Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:01.3245397Z +
2019-07-17T23:28:01.3245424Z 
2019-07-17T23:28:01.3245470Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:01.3245778Z Actual stderr saved to /tmp/compiletestNloeDC/atomic-compare_exchange.stderr
2019-07-17T23:28:01.3245847Z To update references, run this command from build directory:
2019-07-17T23:28:01.3246122Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'atomic-compare_exchange.rs'
2019-07-17T23:28:01.3246223Z error: 1 errors occurred comparing output.
2019-07-17T23:28:01.3246269Z status: exit code: 1
2019-07-17T23:28:01.3246269Z status: exit code: 1
2019-07-17T23:28:01.3246974Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/atomic-compare_exchange.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/atomic-compare_exchange.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/atomic-compare_exchange.stage-id.aux" "-A" "unused"
2019-07-17T23:28:01.3247322Z ------------------------------------------
2019-07-17T23:28:01.3247358Z 
2019-07-17T23:28:01.3247590Z ------------------------------------------
2019-07-17T23:28:01.3247640Z stderr:
---
2019-07-17T23:28:01.4808111Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:01.4808168Z +
2019-07-17T23:28:01.4808191Z 
2019-07-17T23:28:01.4808226Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:01.4808290Z Actual stderr saved to /tmp/compiletestNloeDC/binops.stderr
2019-07-17T23:28:01.4808330Z To update references, run this command from build directory:
2019-07-17T23:28:01.4808541Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'binops.rs'
2019-07-17T23:28:01.4808619Z error: 1 errors occurred comparing output.
2019-07-17T23:28:01.4808656Z status: exit code: 1
2019-07-17T23:28:01.4808656Z status: exit code: 1
2019-07-17T23:28:01.4809150Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/binops.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/binops.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/binops.stage-id.aux" "-A" "unused"
2019-07-17T23:28:01.4809430Z ------------------------------------------
2019-07-17T23:28:01.4809474Z 
2019-07-17T23:28:01.4809653Z ------------------------------------------
2019-07-17T23:28:01.4809690Z stderr:
---
2019-07-17T23:28:01.4821391Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:01.4821434Z +
2019-07-17T23:28:01.4821458Z 
2019-07-17T23:28:01.4821495Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:01.4821552Z Actual stderr saved to /tmp/compiletestNloeDC/bools.stderr
2019-07-17T23:28:01.4821593Z To update references, run this command from build directory:
2019-07-17T23:28:01.4821815Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'bools.rs'
2019-07-17T23:28:01.4821895Z error: 1 errors occurred comparing output.
2019-07-17T23:28:01.4821933Z status: exit code: 1
2019-07-17T23:28:01.4821933Z status: exit code: 1
2019-07-17T23:28:01.4822449Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/bools.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/bools.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/bools.stage-id.aux" "-A" "unused"
2019-07-17T23:28:01.4822737Z ------------------------------------------
2019-07-17T23:28:01.4822766Z 
2019-07-17T23:28:01.4822957Z ------------------------------------------
2019-07-17T23:28:01.4822996Z stderr:
---
2019-07-17T23:28:01.6162155Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:01.6162202Z +
2019-07-17T23:28:01.6162251Z 
2019-07-17T23:28:01.6162290Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:01.6162334Z Actual stderr saved to /tmp/compiletestNloeDC/box_box_trait.stderr
2019-07-17T23:28:01.6162392Z To update references, run this command from build directory:
2019-07-17T23:28:01.6162644Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'box_box_trait.rs'
2019-07-17T23:28:01.6162714Z error: 1 errors occurred comparing output.
2019-07-17T23:28:01.6162768Z status: exit code: 1
2019-07-17T23:28:01.6162768Z status: exit code: 1
2019-07-17T23:28:01.6163309Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/box_box_trait.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/box_box_trait.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/box_box_trait.stage-id.aux" "-A" "unused"
2019-07-17T23:28:01.6163782Z ------------------------------------------
2019-07-17T23:28:01.6163813Z 
2019-07-17T23:28:01.6164001Z ------------------------------------------
2019-07-17T23:28:01.6164055Z stderr:
---
2019-07-17T23:28:01.6247638Z -foo #1 = Foo(1337)
2019-07-17T23:28:01.6248040Z -
2019-07-17T23:28:01.6248074Z 
2019-07-17T23:28:01.6248142Z The actual stdout differed from the expected stdout.
2019-07-17T23:28:01.6248716Z Actual stdout saved to /tmp/compiletestNloeDC/box-pair-to-vec.stdout
2019-07-17T23:28:01.6248840Z error[E0080]: Miri evaluation error: attempted to do invalid arithmetic on pointers that would leak base addresses, e.g., comparing pointers into different allocations
2019-07-17T23:28:01.6249064Z     --> /checkout/src/libcore/intrinsics.rs:1340:19
2019-07-17T23:28:01.6249111Z      |
2019-07-17T23:28:01.6249174Z 1340 |     let diff = if src_usize > dst_usize {
---
2019-07-17T23:28:01.6252443Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:01.6252503Z +
2019-07-17T23:28:01.6252527Z 
2019-07-17T23:28:01.6252573Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:01.6252795Z Actual stderr saved to /tmp/compiletestNloeDC/box-pair-to-vec.stderr
2019-07-17T23:28:01.6252862Z To update references, run this command from build directory:
2019-07-17T23:28:01.6253094Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'box-pair-to-vec.rs'
2019-07-17T23:28:01.6253183Z error: 2 errors occurred comparing output.
2019-07-17T23:28:01.6253222Z status: exit code: 1
2019-07-17T23:28:01.6253222Z status: exit code: 1
2019-07-17T23:28:01.6253750Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/box-pair-to-vec.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/box-pair-to-vec.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/box-pair-to-vec.stage-id.aux" "-A" "unused"
2019-07-17T23:28:01.6254107Z ------------------------------------------
2019-07-17T23:28:01.6254140Z 
2019-07-17T23:28:01.6254522Z ------------------------------------------
2019-07-17T23:28:01.6254751Z stderr:
---
2019-07-17T23:28:01.7882086Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:01.7882148Z +
2019-07-17T23:28:01.7882172Z 
2019-07-17T23:28:01.7882212Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:01.7882257Z Actual stderr saved to /tmp/compiletestNloeDC/c_enums.stderr
2019-07-17T23:28:01.7882320Z To update references, run this command from build directory:
2019-07-17T23:28:01.7882558Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'c_enums.rs'
2019-07-17T23:28:01.7882648Z error: 1 errors occurred comparing output.
2019-07-17T23:28:01.7882697Z status: exit code: 1
2019-07-17T23:28:01.7882697Z status: exit code: 1
2019-07-17T23:28:01.7883258Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/c_enums.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/c_enums.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/c_enums.stage-id.aux" "-A" "unused"
2019-07-17T23:28:01.7883560Z ------------------------------------------
2019-07-17T23:28:01.7883591Z 
2019-07-17T23:28:01.7883816Z ------------------------------------------
2019-07-17T23:28:01.7883858Z stderr:
---
2019-07-17T23:28:01.7928089Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:01.7928144Z +
2019-07-17T23:28:01.7928175Z 
2019-07-17T23:28:01.7928224Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:01.7928297Z Actual stderr saved to /tmp/compiletestNloeDC/btreemap.stderr
2019-07-17T23:28:01.7928515Z To update references, run this command from build directory:
2019-07-17T23:28:01.7928750Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'btreemap.rs'
2019-07-17T23:28:01.7928836Z error: 1 errors occurred comparing output.
2019-07-17T23:28:01.7928874Z status: exit code: 1
2019-07-17T23:28:01.7928874Z status: exit code: 1
2019-07-17T23:28:01.7929436Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/btreemap.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/btreemap.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/btreemap.stage-id.aux" "-A" "unused"
2019-07-17T23:28:01.7929713Z ------------------------------------------
2019-07-17T23:28:01.7929762Z 
2019-07-17T23:28:01.7929953Z ------------------------------------------
2019-07-17T23:28:01.7929994Z stderr:
---
2019-07-17T23:28:01.9208235Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:01.9208303Z +
2019-07-17T23:28:01.9208333Z 
2019-07-17T23:28:01.9208382Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:01.9208438Z Actual stderr saved to /tmp/compiletestNloeDC/call_drop_on_fat_ptr_array_elements.stderr
2019-07-17T23:28:01.9208680Z To update references, run this command from build directory:
2019-07-17T23:28:01.9208915Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'call_drop_on_fat_ptr_array_elements.rs'
2019-07-17T23:28:01.9208995Z error: 1 errors occurred comparing output.
2019-07-17T23:28:01.9209038Z status: exit code: 1
2019-07-17T23:28:01.9209038Z status: exit code: 1
2019-07-17T23:28:01.9209599Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/call_drop_on_fat_ptr_array_elements.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/call_drop_on_fat_ptr_array_elements.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/call_drop_on_fat_ptr_array_elements.stage-id.aux" "-A" "unused"
2019-07-17T23:28:01.9209863Z ------------------------------------------
2019-07-17T23:28:01.9209891Z 
2019-07-17T23:28:01.9210082Z ------------------------------------------
2019-07-17T23:28:01.9210119Z stderr:
---
2019-07-17T23:28:01.9355035Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:01.9355096Z +
2019-07-17T23:28:01.9355148Z 
2019-07-17T23:28:01.9355196Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:01.9355251Z Actual stderr saved to /tmp/compiletestNloeDC/call_drop_on_array_elements.stderr
2019-07-17T23:28:01.9355322Z To update references, run this command from build directory:
2019-07-17T23:28:01.9355619Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'call_drop_on_array_elements.rs'
2019-07-17T23:28:01.9355702Z error: 1 errors occurred comparing output.
2019-07-17T23:28:01.9355767Z status: exit code: 1
2019-07-17T23:28:01.9355767Z status: exit code: 1
2019-07-17T23:28:01.9356461Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/call_drop_on_array_elements.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/call_drop_on_array_elements.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/call_drop_on_array_elements.stage-id.aux" "-A" "unused"
2019-07-17T23:28:01.9356801Z ------------------------------------------
2019-07-17T23:28:01.9356837Z 
2019-07-17T23:28:01.9357077Z ------------------------------------------
2019-07-17T23:28:01.9357135Z stderr:
---
2019-07-17T23:28:02.0772456Z +     = note: inside call to `std::slice::hack::to_vec::<u8>` at /checkout/src/liballoc/slice.rs:379:9
2019-07-17T23:28:02.0772527Z +     = note: inside call to `std::slice::<impl [u8]>::to_vec` at /checkout/src/liballoc/slice.rs:654:9
2019-07-17T23:28:02.0772556Z 
2019-07-17T23:28:02.0772594Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:02.0772644Z Actual stderr saved to /tmp/compiletestNloeDC/call_drop_on_zst_array_elements.stderr
2019-07-17T23:28:02.0772704Z To update references, run this command from build directory:
2019-07-17T23:28:02.0772955Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'call_drop_on_zst_array_elements.rs'
2019-07-17T23:28:02.0773042Z error: 1 errors occurred comparing output.
2019-07-17T23:28:02.0773079Z status: exit code: 1
2019-07-17T23:28:02.0773079Z status: exit code: 1
2019-07-17T23:28:02.0773652Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/call_drop_on_zst_array_elements.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/call_drop_on_zst_array_elements.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/call_drop_on_zst_array_elements.stage-id.aux" "-A" "unused"
2019-07-17T23:28:02.0774065Z ------------------------------------------
2019-07-17T23:28:02.0774172Z 
2019-07-17T23:28:02.0774380Z ------------------------------------------
2019-07-17T23:28:02.0774422Z stderr:
---
2019-07-17T23:28:02.0783279Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:02.0783320Z +
2019-07-17T23:28:02.0783362Z 
2019-07-17T23:28:02.0783400Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:02.0783444Z Actual stderr saved to /tmp/compiletestNloeDC/call_drop_through_owned_slice.stderr
2019-07-17T23:28:02.0783512Z To update references, run this command from build directory:
2019-07-17T23:28:02.0783750Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'call_drop_through_owned_slice.rs'
2019-07-17T23:28:02.0783824Z error: 1 errors occurred comparing output.
2019-07-17T23:28:02.0783880Z status: exit code: 1
2019-07-17T23:28:02.0783880Z status: exit code: 1
2019-07-17T23:28:02.0784946Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/call_drop_through_owned_slice.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/call_drop_through_owned_slice.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/call_drop_through_owned_slice.stage-id.aux" "-A" "unused"
2019-07-17T23:28:02.0785431Z ------------------------------------------
2019-07-17T23:28:02.0785467Z 
2019-07-17T23:28:02.0785712Z ------------------------------------------
2019-07-17T23:28:02.0785764Z stderr:
---
2019-07-17T23:28:02.2052595Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:02.2052644Z +
2019-07-17T23:28:02.2052683Z 
2019-07-17T23:28:02.2052720Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:02.2052764Z Actual stderr saved to /tmp/compiletestNloeDC/call_drop_through_trait_object_rc.stderr
2019-07-17T23:28:02.2052990Z To update references, run this command from build directory:
2019-07-17T23:28:02.2053215Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'call_drop_through_trait_object_rc.rs'
2019-07-17T23:28:02.2053282Z error: 1 errors occurred comparing output.
2019-07-17T23:28:02.2053336Z status: exit code: 1
2019-07-17T23:28:02.2053336Z status: exit code: 1
2019-07-17T23:28:02.2053975Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/call_drop_through_trait_object_rc.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/call_drop_through_trait_object_rc.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/call_drop_through_trait_object_rc.stage-id.aux" "-A" "unused"
2019-07-17T23:28:02.2054777Z ------------------------------------------
2019-07-17T23:28:02.2055069Z 
2019-07-17T23:28:02.2055379Z ------------------------------------------
2019-07-17T23:28:02.2055429Z stderr:
---
2019-07-17T23:28:02.2069094Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:02.2069135Z +
2019-07-17T23:28:02.2069158Z 
2019-07-17T23:28:02.2069210Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:02.2069253Z Actual stderr saved to /tmp/compiletestNloeDC/call_drop_through_trait_object.stderr
2019-07-17T23:28:02.2069296Z To update references, run this command from build directory:
2019-07-17T23:28:02.2069542Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'call_drop_through_trait_object.rs'
2019-07-17T23:28:02.2069792Z error: 1 errors occurred comparing output.
2019-07-17T23:28:02.2069845Z status: exit code: 1
2019-07-17T23:28:02.2069845Z status: exit code: 1
2019-07-17T23:28:02.2070473Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/call_drop_through_trait_object.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/call_drop_through_trait_object.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/call_drop_through_trait_object.stage-id.aux" "-A" "unused"
2019-07-17T23:28:02.2070775Z ------------------------------------------
2019-07-17T23:28:02.2070804Z 
2019-07-17T23:28:02.2070980Z ------------------------------------------
2019-07-17T23:28:02.2071033Z stderr:
---
2019-07-17T23:28:02.3720805Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:02.3720965Z +
2019-07-17T23:28:02.3721055Z 
2019-07-17T23:28:02.3721162Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:02.3721290Z Actual stderr saved to /tmp/compiletestNloeDC/calloc.stderr
2019-07-17T23:28:02.3721561Z To update references, run this command from build directory:
2019-07-17T23:28:02.3721902Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'calloc.rs'
2019-07-17T23:28:02.3722133Z error: 1 errors occurred comparing output.
2019-07-17T23:28:02.3722237Z status: exit code: 1
2019-07-17T23:28:02.3722237Z status: exit code: 1
2019-07-17T23:28:02.3723076Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/calloc.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/calloc.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/calloc.stage-id.aux" "-A" "unused"
2019-07-17T23:28:02.3723617Z ------------------------------------------
2019-07-17T23:28:02.3723749Z 
2019-07-17T23:28:02.3724074Z ------------------------------------------
2019-07-17T23:28:02.3724323Z stderr:
---
2019-07-17T23:28:02.3806896Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:02.3806971Z +
2019-07-17T23:28:02.3807002Z 
2019-07-17T23:28:02.3807048Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:02.3807119Z Actual stderr saved to /tmp/compiletestNloeDC/calls.stderr
2019-07-17T23:28:02.3807180Z To update references, run this command from build directory:
2019-07-17T23:28:02.3807449Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'calls.rs'
2019-07-17T23:28:02.3807550Z error: 1 errors occurred comparing output.
2019-07-17T23:28:02.3807752Z status: exit code: 1
2019-07-17T23:28:02.3807752Z status: exit code: 1
2019-07-17T23:28:02.3808693Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/calls.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/calls.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/calls.stage-id.aux" "-A" "unused"
2019-07-17T23:28:02.3808985Z ------------------------------------------
2019-07-17T23:28:02.3809017Z 
2019-07-17T23:28:02.3809221Z ------------------------------------------
2019-07-17T23:28:02.3809358Z stderr:
---
2019-07-17T23:28:02.5086460Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:02.5086522Z +
2019-07-17T23:28:02.5086550Z 
2019-07-17T23:28:02.5086614Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:02.5086668Z Actual stderr saved to /tmp/compiletestNloeDC/cast_fn_ptr.stderr
2019-07-17T23:28:02.5086874Z To update references, run this command from build directory:
2019-07-17T23:28:02.5087214Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'cast_fn_ptr.rs'
2019-07-17T23:28:02.5087300Z error: 1 errors occurred comparing output.
2019-07-17T23:28:02.5087347Z status: exit code: 1
2019-07-17T23:28:02.5087347Z status: exit code: 1
2019-07-17T23:28:02.5088015Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/cast_fn_ptr.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/cast_fn_ptr.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/cast_fn_ptr.stage-id.aux" "-A" "unused"
2019-07-17T23:28:02.5088583Z ------------------------------------------
2019-07-17T23:28:02.5088614Z 
2019-07-17T23:28:02.5088796Z ------------------------------------------
2019-07-17T23:28:02.5088860Z stderr:
---
2019-07-17T23:28:02.5343809Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:02.5343851Z +
2019-07-17T23:28:02.5343874Z 
2019-07-17T23:28:02.5343918Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:02.5344164Z Actual stderr saved to /tmp/compiletestNloeDC/cast-rfc0401-vtable-kinds.stderr
2019-07-17T23:28:02.5344212Z To update references, run this command from build directory:
2019-07-17T23:28:02.5344521Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'cast-rfc0401-vtable-kinds.rs'
2019-07-17T23:28:02.5344995Z error: 1 errors occurred comparing output.
2019-07-17T23:28:02.5345057Z status: exit code: 1
2019-07-17T23:28:02.5345057Z status: exit code: 1
2019-07-17T23:28:02.5345856Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/cast-rfc0401-vtable-kinds.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/cast-rfc0401-vtable-kinds.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/cast-rfc0401-vtable-kinds.stage-id.aux" "-A" "unused"
2019-07-17T23:28:02.5346311Z ------------------------------------------
2019-07-17T23:28:02.5346365Z 
2019-07-17T23:28:02.5346594Z ------------------------------------------
2019-07-17T23:28:02.5346643Z stderr:
---
2019-07-17T23:28:02.6396385Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:02.6396442Z +
2019-07-17T23:28:02.6396492Z 
2019-07-17T23:28:02.6396541Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:02.6396754Z Actual stderr saved to /tmp/compiletestNloeDC/cast_fn_ptr_unsafe.stderr
2019-07-17T23:28:02.6396842Z To update references, run this command from build directory:
2019-07-17T23:28:02.6397204Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'cast_fn_ptr_unsafe.rs'
2019-07-17T23:28:02.6397294Z error: 1 errors occurred comparing output.
2019-07-17T23:28:02.6397363Z status: exit code: 1
2019-07-17T23:28:02.6397363Z status: exit code: 1
2019-07-17T23:28:02.6398295Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/cast_fn_ptr_unsafe.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/cast_fn_ptr_unsafe.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/cast_fn_ptr_unsafe.stage-id.aux" "-A" "unused"
2019-07-17T23:28:02.6398700Z ------------------------------------------
2019-07-17T23:28:02.6398739Z 
2019-07-17T23:28:02.6398951Z ------------------------------------------
2019-07-17T23:28:02.6398991Z stderr:
---
2019-07-17T23:28:02.6955643Z -1
2019-07-17T23:28:02.6955860Z -
2019-07-17T23:28:02.6955891Z 
2019-07-17T23:28:02.6955939Z The actual stdout differed from the expected stdout.
2019-07-17T23:28:02.6956009Z Actual stdout saved to /tmp/compiletestNloeDC/catch.stdout
2019-07-17T23:28:02.6956118Z error[E0080]: Miri evaluation error: attempted to do invalid arithmetic on pointers that would leak base addresses, e.g., comparing pointers into different allocations
2019-07-17T23:28:02.6956422Z     --> /checkout/src/libcore/intrinsics.rs:1340:19
2019-07-17T23:28:02.6956477Z      |
2019-07-17T23:28:02.6956528Z 1340 |     let diff = if src_usize > dst_usize {
---
2019-07-17T23:28:02.6963004Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:02.6963049Z +
2019-07-17T23:28:02.6963090Z 
2019-07-17T23:28:02.6963129Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:02.6963171Z Actual stderr saved to /tmp/compiletestNloeDC/catch.stderr
2019-07-17T23:28:02.6963333Z To update references, run this command from build directory:
2019-07-17T23:28:02.6963594Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'catch.rs'
2019-07-17T23:28:02.6963665Z error: 2 errors occurred comparing output.
2019-07-17T23:28:02.6963727Z status: exit code: 1
2019-07-17T23:28:02.6963727Z status: exit code: 1
2019-07-17T23:28:02.6965023Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/catch.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/catch.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/catch.stage-id.aux" "-A" "unused"
2019-07-17T23:28:02.6965401Z ------------------------------------------
2019-07-17T23:28:02.6965437Z 
2019-07-17T23:28:02.6965664Z ------------------------------------------
2019-07-17T23:28:02.6965741Z stderr:
---
2019-07-17T23:28:02.8042929Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:02.8043084Z +
2019-07-17T23:28:02.8043109Z 
2019-07-17T23:28:02.8043152Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:02.8043196Z Actual stderr saved to /tmp/compiletestNloeDC/char.stderr
2019-07-17T23:28:02.8043263Z To update references, run this command from build directory:
2019-07-17T23:28:02.8043539Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'char.rs'
2019-07-17T23:28:02.8043628Z error: 1 errors occurred comparing output.
2019-07-17T23:28:02.8043669Z status: exit code: 1
2019-07-17T23:28:02.8043669Z status: exit code: 1
2019-07-17T23:28:02.8044222Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/char.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/char.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/char.stage-id.aux" "-A" "unused"
2019-07-17T23:28:02.8044533Z ------------------------------------------
2019-07-17T23:28:02.8044565Z 
2019-07-17T23:28:02.8045307Z ------------------------------------------
2019-07-17T23:28:02.8045364Z stderr:
---
2019-07-17T23:28:02.8137899Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:02.8138468Z +
2019-07-17T23:28:02.8138492Z 
2019-07-17T23:28:02.8138551Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:02.8138988Z Actual stderr saved to /tmp/compiletestNloeDC/closure-drop.stderr
2019-07-17T23:28:02.8139042Z To update references, run this command from build directory:
2019-07-17T23:28:02.8139264Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'closure-drop.rs'
2019-07-17T23:28:02.8139349Z error: 1 errors occurred comparing output.
2019-07-17T23:28:02.8139387Z status: exit code: 1
2019-07-17T23:28:02.8139387Z status: exit code: 1
2019-07-17T23:28:02.8139943Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/closure-drop.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/closure-drop.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/closure-drop.stage-id.aux" "-A" "unused"
2019-07-17T23:28:02.8140227Z ------------------------------------------
2019-07-17T23:28:02.8140256Z 
2019-07-17T23:28:02.8140449Z ------------------------------------------
2019-07-17T23:28:02.8140487Z stderr:
---
2019-07-17T23:28:02.9642340Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:02.9642404Z +
2019-07-17T23:28:02.9642525Z 
2019-07-17T23:28:02.9642565Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:02.9642824Z Actual stderr saved to /tmp/compiletestNloeDC/closure-field-ty.stderr
2019-07-17T23:28:02.9642894Z To update references, run this command from build directory:
2019-07-17T23:28:02.9643136Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'closure-field-ty.rs'
2019-07-17T23:28:02.9643222Z error: 1 errors occurred comparing output.
2019-07-17T23:28:02.9643260Z status: exit code: 1
2019-07-17T23:28:02.9643260Z status: exit code: 1
2019-07-17T23:28:02.9643801Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/closure-field-ty.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/closure-field-ty.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/closure-field-ty.stage-id.aux" "-A" "unused"
2019-07-17T23:28:02.9644097Z ------------------------------------------
2019-07-17T23:28:02.9644126Z 
2019-07-17T23:28:02.9644342Z ------------------------------------------
2019-07-17T23:28:02.9644758Z stderr:
---
2019-07-17T23:28:03.0244084Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:03.0244147Z +
2019-07-17T23:28:03.0244172Z 
2019-07-17T23:28:03.0244211Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:03.0244255Z Actual stderr saved to /tmp/compiletestNloeDC/closures.stderr
2019-07-17T23:28:03.0244322Z To update references, run this command from build directory:
2019-07-17T23:28:03.0244574Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'closures.rs'
2019-07-17T23:28:03.0245007Z error: 1 errors occurred comparing output.
2019-07-17T23:28:03.0245122Z status: exit code: 1
2019-07-17T23:28:03.0245122Z status: exit code: 1
2019-07-17T23:28:03.0245886Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/closures.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/closures.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/closures.stage-id.aux" "-A" "unused"
2019-07-17T23:28:03.0246242Z ------------------------------------------
2019-07-17T23:28:03.0246280Z 
2019-07-17T23:28:03.0246534Z ------------------------------------------
2019-07-17T23:28:03.0246584Z stderr:
---
2019-07-17T23:28:03.0909041Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:03.0909082Z +
2019-07-17T23:28:03.0909105Z 
2019-07-17T23:28:03.0909141Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:03.0909372Z Actual stderr saved to /tmp/compiletestNloeDC/const-vec-of-fns.stderr
2019-07-17T23:28:03.0909418Z To update references, run this command from build directory:
2019-07-17T23:28:03.0909635Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'const-vec-of-fns.rs'
2019-07-17T23:28:03.0909715Z error: 1 errors occurred comparing output.
2019-07-17T23:28:03.0909751Z status: exit code: 1
2019-07-17T23:28:03.0909751Z status: exit code: 1
2019-07-17T23:28:03.0910300Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/const-vec-of-fns.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/const-vec-of-fns.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/const-vec-of-fns.stage-id.aux" "-A" "unused"
2019-07-17T23:28:03.0910572Z ------------------------------------------
2019-07-17T23:28:03.0910619Z 
2019-07-17T23:28:03.0910803Z ------------------------------------------
2019-07-17T23:28:03.0910841Z stderr:
---
2019-07-17T23:28:03.1763783Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:03.1763827Z +
2019-07-17T23:28:03.1763851Z 
2019-07-17T23:28:03.1763890Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:03.1763950Z Actual stderr saved to /tmp/compiletestNloeDC/constants.stderr
2019-07-17T23:28:03.1763994Z To update references, run this command from build directory:
2019-07-17T23:28:03.1764230Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'constants.rs'
2019-07-17T23:28:03.1764316Z error: 1 errors occurred comparing output.
2019-07-17T23:28:03.1764827Z status: exit code: 1
2019-07-17T23:28:03.1764827Z status: exit code: 1
2019-07-17T23:28:03.1766443Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/constants.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/constants.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/constants.stage-id.aux" "-A" "unused"
2019-07-17T23:28:03.1766845Z ------------------------------------------
2019-07-17T23:28:03.1766900Z 
2019-07-17T23:28:03.1767133Z ------------------------------------------
2019-07-17T23:28:03.1767183Z stderr:
---
2019-07-17T23:28:03.3000833Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:03.3000879Z +
2019-07-17T23:28:03.3000902Z 
2019-07-17T23:28:03.3000940Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:03.3000999Z Actual stderr saved to /tmp/compiletestNloeDC/drop_empty_slice.stderr
2019-07-17T23:28:03.3001217Z To update references, run this command from build directory:
2019-07-17T23:28:03.3001456Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'drop_empty_slice.rs'
2019-07-17T23:28:03.3001544Z error: 1 errors occurred comparing output.
2019-07-17T23:28:03.3001582Z status: exit code: 1
2019-07-17T23:28:03.3001582Z status: exit code: 1
2019-07-17T23:28:03.3002162Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/drop_empty_slice.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/drop_empty_slice.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/drop_empty_slice.stage-id.aux" "-A" "unused"
2019-07-17T23:28:03.3002435Z ------------------------------------------
2019-07-17T23:28:03.3002483Z 
2019-07-17T23:28:03.3002671Z ------------------------------------------
2019-07-17T23:28:03.3002710Z stderr:
---
2019-07-17T23:28:03.3759484Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:03.3759523Z +
2019-07-17T23:28:03.3759544Z 
2019-07-17T23:28:03.3759594Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:03.3759802Z Actual stderr saved to /tmp/compiletestNloeDC/deriving-associated-types.stderr
2019-07-17T23:28:03.3759846Z To update references, run this command from build directory:
2019-07-17T23:28:03.3760077Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'deriving-associated-types.rs'
2019-07-17T23:28:03.3760148Z error: 1 errors occurred comparing output.
2019-07-17T23:28:03.3760185Z status: exit code: 1
2019-07-17T23:28:03.3760185Z status: exit code: 1
2019-07-17T23:28:03.3760752Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/deriving-associated-types.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/deriving-associated-types.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/deriving-associated-types.stage-id.aux" "-A" "unused"
2019-07-17T23:28:03.3761017Z ------------------------------------------
2019-07-17T23:28:03.3761045Z 
2019-07-17T23:28:03.3761218Z ------------------------------------------
2019-07-17T23:28:03.3761269Z stderr:
---
2019-07-17T23:28:03.5067599Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:03.5067671Z +
2019-07-17T23:28:03.5067700Z 
2019-07-17T23:28:03.5067746Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:03.5068496Z Actual stderr saved to /tmp/compiletestNloeDC/dst-irrefutable-bind.stderr
2019-07-17T23:28:03.5068572Z To update references, run this command from build directory:
2019-07-17T23:28:03.5069100Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'dst-irrefutable-bind.rs'
2019-07-17T23:28:03.5069289Z error: 1 errors occurred comparing output.
2019-07-17T23:28:03.5069348Z status: exit code: 1
2019-07-17T23:28:03.5069348Z status: exit code: 1
2019-07-17T23:28:03.5070125Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/dst-irrefutable-bind.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/dst-irrefutable-bind.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/dst-irrefutable-bind.stage-id.aux" "-A" "unused"
2019-07-17T23:28:03.5070586Z ------------------------------------------
2019-07-17T23:28:03.5070622Z 
2019-07-17T23:28:03.5070826Z ------------------------------------------
2019-07-17T23:28:03.5070863Z stderr:
---
2019-07-17T23:28:03.5083404Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:03.5083446Z +
2019-07-17T23:28:03.5083470Z 
2019-07-17T23:28:03.5083525Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:03.5083737Z Actual stderr saved to /tmp/compiletestNloeDC/dst-field-align.stderr
2019-07-17T23:28:03.5083790Z To update references, run this command from build directory:
2019-07-17T23:28:03.5084030Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'dst-field-align.rs'
2019-07-17T23:28:03.5084097Z error: 1 errors occurred comparing output.
2019-07-17T23:28:03.5084157Z status: exit code: 1
2019-07-17T23:28:03.5084157Z status: exit code: 1
2019-07-17T23:28:03.5085026Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/dst-field-align.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/dst-field-align.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/dst-field-align.stage-id.aux" "-A" "unused"
2019-07-17T23:28:03.5085440Z ------------------------------------------
2019-07-17T23:28:03.5085477Z 
2019-07-17T23:28:03.5085705Z ------------------------------------------
2019-07-17T23:28:03.5085783Z stderr:
---
2019-07-17T23:28:03.7130407Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:03.7130450Z +
2019-07-17T23:28:03.7130473Z 
2019-07-17T23:28:03.7130528Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:03.7130759Z Actual stderr saved to /tmp/compiletestNloeDC/dst-struct-sole.stderr
2019-07-17T23:28:03.7130806Z To update references, run this command from build directory:
2019-07-17T23:28:03.7131055Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'dst-struct-sole.rs'
2019-07-17T23:28:03.7131132Z error: 1 errors occurred comparing output.
2019-07-17T23:28:03.7131187Z status: exit code: 1
2019-07-17T23:28:03.7131187Z status: exit code: 1
2019-07-17T23:28:03.7131730Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/dst-struct-sole.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/dst-struct-sole.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/dst-struct-sole.stage-id.aux" "-A" "unused"
2019-07-17T23:28:03.7132029Z ------------------------------------------
2019-07-17T23:28:03.7132060Z 
2019-07-17T23:28:03.7132258Z ------------------------------------------
2019-07-17T23:28:03.7132314Z stderr:
---
2019-07-17T23:28:03.7463606Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:03.7463661Z +
2019-07-17T23:28:03.7463866Z 
2019-07-17T23:28:03.7463908Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:03.7464238Z Actual stderr saved to /tmp/compiletestNloeDC/dst-raw.stderr
2019-07-17T23:28:03.7464286Z To update references, run this command from build directory:
2019-07-17T23:28:03.7465060Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'dst-raw.rs'
2019-07-17T23:28:03.7465178Z error: 1 errors occurred comparing output.
2019-07-17T23:28:03.7465224Z status: exit code: 1
2019-07-17T23:28:03.7465224Z status: exit code: 1
2019-07-17T23:28:03.7465894Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/dst-raw.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/dst-raw.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/dst-raw.stage-id.aux" "-A" "unused"
2019-07-17T23:28:03.7466236Z ------------------------------------------
2019-07-17T23:28:03.7466272Z 
2019-07-17T23:28:03.7466498Z ------------------------------------------
2019-07-17T23:28:03.7466545Z stderr:
---
2019-07-17T23:28:03.8677399Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:03.8677462Z +
2019-07-17T23:28:03.8677491Z 
2019-07-17T23:28:03.8677540Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:03.8677867Z Actual stderr saved to /tmp/compiletestNloeDC/enum-nullable-const-null-with-fields.stderr
2019-07-17T23:28:03.8677937Z To update references, run this command from build directory:
2019-07-17T23:28:03.8678700Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'enum-nullable-const-null-with-fields.rs'
2019-07-17T23:28:03.8678786Z error: 1 errors occurred comparing output.
2019-07-17T23:28:03.8678821Z status: exit code: 1
2019-07-17T23:28:03.8678821Z status: exit code: 1
2019-07-17T23:28:03.8679399Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/enum-nullable-const-null-with-fields.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/enum-nullable-const-null-with-fields.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/enum-nullable-const-null-with-fields.stage-id.aux" "-A" "unused"
2019-07-17T23:28:03.8679683Z ------------------------------------------
2019-07-17T23:28:03.8679712Z 
2019-07-17T23:28:03.8679890Z ------------------------------------------
2019-07-17T23:28:03.8679927Z stderr:
---
2019-07-17T23:28:03.9838877Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:03.9838925Z +
2019-07-17T23:28:03.9838948Z 
2019-07-17T23:28:03.9839004Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:03.9839218Z Actual stderr saved to /tmp/compiletestNloeDC/dst-struct.stderr
2019-07-17T23:28:03.9839265Z To update references, run this command from build directory:
2019-07-17T23:28:03.9839502Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'dst-struct.rs'
2019-07-17T23:28:03.9839568Z error: 1 errors occurred comparing output.
2019-07-17T23:28:03.9839604Z status: exit code: 1
2019-07-17T23:28:03.9839604Z status: exit code: 1
2019-07-17T23:28:03.9840138Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/dst-struct.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/dst-struct.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/dst-struct.stage-id.aux" "-A" "unused"
2019-07-17T23:28:03.9840423Z ------------------------------------------
2019-07-17T23:28:03.9840453Z 
2019-07-17T23:28:03.9840645Z ------------------------------------------
2019-07-17T23:28:03.9840701Z stderr:
---
2019-07-17T23:28:04.0147029Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:04.0147131Z +
2019-07-17T23:28:04.0147163Z 
2019-07-17T23:28:04.0147210Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:04.0147280Z Actual stderr saved to /tmp/compiletestNloeDC/enums.stderr
2019-07-17T23:28:04.0147331Z To update references, run this command from build directory:
2019-07-17T23:28:04.0147633Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'enums.rs'
2019-07-17T23:28:04.0147735Z error: 1 errors occurred comparing output.
2019-07-17T23:28:04.0147781Z status: exit code: 1
2019-07-17T23:28:04.0147781Z status: exit code: 1
2019-07-17T23:28:04.0148525Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/enums.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/enums.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/enums.stage-id.aux" "-A" "unused"
2019-07-17T23:28:04.0148990Z ------------------------------------------
2019-07-17T23:28:04.0149020Z 
2019-07-17T23:28:04.0149221Z ------------------------------------------
2019-07-17T23:28:04.0149262Z stderr:
---
2019-07-17T23:28:04.1322377Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:04.1322441Z +
2019-07-17T23:28:04.1322465Z 
2019-07-17T23:28:04.1322501Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:04.1322558Z Actual stderr saved to /tmp/compiletestNloeDC/exit.stderr
2019-07-17T23:28:04.1322597Z To update references, run this command from build directory:
2019-07-17T23:28:04.1322815Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'exit.rs'
2019-07-17T23:28:04.1322894Z error: 1 errors occurred comparing output.
2019-07-17T23:28:04.1322929Z status: exit code: 1
2019-07-17T23:28:04.1322929Z status: exit code: 1
2019-07-17T23:28:04.1323604Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/exit.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/exit.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/exit.stage-id.aux" "-A" "unused"
2019-07-17T23:28:04.1323884Z ------------------------------------------
2019-07-17T23:28:04.1323912Z 
2019-07-17T23:28:04.1324115Z ------------------------------------------
2019-07-17T23:28:04.1324153Z stderr:
---
2019-07-17T23:28:04.1348662Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:04.1348702Z +
2019-07-17T23:28:04.1348723Z 
2019-07-17T23:28:04.1348775Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:04.1348815Z Actual stderr saved to /tmp/compiletestNloeDC/env.stderr
2019-07-17T23:28:04.1348854Z To update references, run this command from build directory:
2019-07-17T23:28:04.1349068Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'env.rs'
2019-07-17T23:28:04.1349131Z error: 1 errors occurred comparing output.
2019-07-17T23:28:04.1349166Z status: exit code: 1
2019-07-17T23:28:04.1349166Z status: exit code: 1
2019-07-17T23:28:04.1349674Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/env.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/env.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/env.stage-id.aux" "-A" "unused"
2019-07-17T23:28:04.1349936Z ------------------------------------------
2019-07-17T23:28:04.1349962Z 
2019-07-17T23:28:04.1350133Z ------------------------------------------
2019-07-17T23:28:04.1350186Z stderr:
---
2019-07-17T23:28:04.2517401Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:04.2517472Z +
2019-07-17T23:28:04.2517500Z 
2019-07-17T23:28:04.2517546Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:04.2517598Z Actual stderr saved to /tmp/compiletestNloeDC/extern_types.stderr
2019-07-17T23:28:04.2517668Z To update references, run this command from build directory:
2019-07-17T23:28:04.2517941Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'extern_types.rs'
2019-07-17T23:28:04.2518039Z error: 1 errors occurred comparing output.
2019-07-17T23:28:04.2518093Z status: exit code: 1
2019-07-17T23:28:04.2518093Z status: exit code: 1
2019-07-17T23:28:04.2518863Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/extern_types.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/extern_types.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/extern_types.stage-id.aux" "-A" "unused"
2019-07-17T23:28:04.2519128Z ------------------------------------------
2019-07-17T23:28:04.2519156Z 
2019-07-17T23:28:04.2519528Z ------------------------------------------
2019-07-17T23:28:04.2519566Z stderr:
---
2019-07-17T23:28:04.2760693Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:04.2760733Z +
2019-07-17T23:28:04.2760756Z 
2019-07-17T23:28:04.2765042Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:04.2765113Z Actual stderr saved to /tmp/compiletestNloeDC/float_fast_math.stderr
2019-07-17T23:28:04.2769224Z To update references, run this command from build directory:
2019-07-17T23:28:04.2769624Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'float_fast_math.rs'
2019-07-17T23:28:04.2774374Z error: 1 errors occurred comparing output.
2019-07-17T23:28:04.2774796Z status: exit code: 1
2019-07-17T23:28:04.2774796Z status: exit code: 1
2019-07-17T23:28:04.2775636Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/float_fast_math.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/float_fast_math.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/float_fast_math.stage-id.aux" "-A" "unused"
2019-07-17T23:28:04.2775985Z ------------------------------------------
2019-07-17T23:28:04.2776022Z 
2019-07-17T23:28:04.2776252Z ------------------------------------------
2019-07-17T23:28:04.2776319Z stderr:
---
2019-07-17T23:28:04.4318011Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:04.4318061Z +
2019-07-17T23:28:04.4318090Z 
2019-07-17T23:28:04.4318319Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:04.4318715Z Actual stderr saved to /tmp/compiletestNloeDC/foreign-fn-linkname.stderr
2019-07-17T23:28:04.4318759Z To update references, run this command from build directory:
2019-07-17T23:28:04.4318986Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'foreign-fn-linkname.rs'
2019-07-17T23:28:04.4319059Z error: 1 errors occurred comparing output.
2019-07-17T23:28:04.4319111Z status: exit code: 1
2019-07-17T23:28:04.4319111Z status: exit code: 1
2019-07-17T23:28:04.4319641Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/foreign-fn-linkname.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/foreign-fn-linkname.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/foreign-fn-linkname.stage-id.aux" "-A" "unused"
2019-07-17T23:28:04.4319914Z ------------------------------------------
2019-07-17T23:28:04.4319942Z 
2019-07-17T23:28:04.4320115Z ------------------------------------------
2019-07-17T23:28:04.4320168Z stderr:
---
2019-07-17T23:28:04.4832301Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:04.4832360Z +
2019-07-17T23:28:04.4832383Z 
2019-07-17T23:28:04.4832420Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:04.4832461Z Actual stderr saved to /tmp/compiletestNloeDC/floats.stderr
2019-07-17T23:28:04.4832529Z To update references, run this command from build directory:
2019-07-17T23:28:04.4832750Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'floats.rs'
2019-07-17T23:28:04.4832822Z error: 1 errors occurred comparing output.
2019-07-17T23:28:04.4832965Z status: exit code: 1
2019-07-17T23:28:04.4832965Z status: exit code: 1
2019-07-17T23:28:04.4833474Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/floats.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/floats.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/floats.stage-id.aux" "-A" "unused"
2019-07-17T23:28:04.4833753Z ------------------------------------------
2019-07-17T23:28:04.4833782Z 
2019-07-17T23:28:04.4833986Z ------------------------------------------
2019-07-17T23:28:04.4834032Z stderr:
---
2019-07-17T23:28:04.5725270Z -hello00000
2019-07-17T23:28:04.5728169Z -
2019-07-17T23:28:04.5730613Z 
2019-07-17T23:28:04.5732711Z The actual stdout differed from the expected stdout.
2019-07-17T23:28:04.5735325Z Actual stdout saved to /tmp/compiletestNloeDC/format.stdout
2019-07-17T23:28:04.5741202Z error[E0080]: Miri evaluation error: attempted to do invalid arithmetic on pointers that would leak base addresses, e.g., comparing pointers into different allocations
2019-07-17T23:28:04.5741897Z     --> /checkout/src/libcore/intrinsics.rs:1340:19
2019-07-17T23:28:04.5741963Z      |
2019-07-17T23:28:04.5742008Z 1340 |     let diff = if src_usize > dst_usize {
---
2019-07-17T23:28:04.5761963Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:04.5762003Z +
2019-07-17T23:28:04.5762026Z 
2019-07-17T23:28:04.5762062Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:04.5762120Z Actual stderr saved to /tmp/compiletestNloeDC/format.stderr
2019-07-17T23:28:04.5762160Z To update references, run this command from build directory:
2019-07-17T23:28:04.5762374Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'format.rs'
2019-07-17T23:28:04.5762453Z error: 2 errors occurred comparing output.
2019-07-17T23:28:04.5762497Z status: exit code: 1
2019-07-17T23:28:04.5762497Z status: exit code: 1
2019-07-17T23:28:04.5763130Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/format.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/format.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/format.stage-id.aux" "-A" "unused"
2019-07-17T23:28:04.5763437Z ------------------------------------------
2019-07-17T23:28:04.5763481Z 
2019-07-17T23:28:04.5763659Z ------------------------------------------
2019-07-17T23:28:04.5763697Z stderr:
---
2019-07-17T23:28:04.6058882Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:04.6058941Z +
2019-07-17T23:28:04.6058970Z 
2019-07-17T23:28:04.6059016Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:04.6059086Z Actual stderr saved to /tmp/compiletestNloeDC/from_utf8.stderr
2019-07-17T23:28:04.6059137Z To update references, run this command from build directory:
2019-07-17T23:28:04.6059446Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'from_utf8.rs'
2019-07-17T23:28:04.6059558Z error: 1 errors occurred comparing output.
2019-07-17T23:28:04.6059604Z status: exit code: 1
2019-07-17T23:28:04.6059604Z status: exit code: 1
2019-07-17T23:28:04.6060358Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/from_utf8.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/from_utf8.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/from_utf8.stage-id.aux" "-A" "unused"
2019-07-17T23:28:04.6060724Z ------------------------------------------
2019-07-17T23:28:04.6060772Z 
2019-07-17T23:28:04.6060995Z ------------------------------------------
2019-07-17T23:28:04.6061043Z stderr:
---
2019-07-17T23:28:04.7723414Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:04.7723467Z +
2019-07-17T23:28:04.7723496Z 
2019-07-17T23:28:04.7723542Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:04.7723608Z Actual stderr saved to /tmp/compiletestNloeDC/function_pointers.stderr
2019-07-17T23:28:04.7723659Z To update references, run this command from build directory:
2019-07-17T23:28:04.7724093Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'function_pointers.rs'
2019-07-17T23:28:04.7724950Z error: 1 errors occurred comparing output.
2019-07-17T23:28:04.7724995Z status: exit code: 1
2019-07-17T23:28:04.7724995Z status: exit code: 1
2019-07-17T23:28:04.7725880Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/function_pointers.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/function_pointers.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/function_pointers.stage-id.aux" "-A" "unused"
2019-07-17T23:28:04.7726270Z ------------------------------------------
2019-07-17T23:28:04.7726309Z 
2019-07-17T23:28:04.7726537Z ------------------------------------------
2019-07-17T23:28:04.7726584Z stderr:
---
2019-07-17T23:28:04.8038800Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:04.8038854Z +
2019-07-17T23:28:04.8038876Z 
2019-07-17T23:28:04.8038912Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:04.8038954Z Actual stderr saved to /tmp/compiletestNloeDC/generator.stderr
2019-07-17T23:28:04.8039017Z To update references, run this command from build directory:
2019-07-17T23:28:04.8039225Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'generator.rs'
2019-07-17T23:28:04.8039289Z error: 1 errors occurred comparing output.
2019-07-17T23:28:04.8039415Z status: exit code: 1
2019-07-17T23:28:04.8039415Z status: exit code: 1
2019-07-17T23:28:04.8039952Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/generator.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/generator.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/generator.stage-id.aux" "-A" "unused"
2019-07-17T23:28:04.8040218Z ------------------------------------------
2019-07-17T23:28:04.8040247Z 
2019-07-17T23:28:04.8040440Z ------------------------------------------
2019-07-17T23:28:04.8040555Z stderr:
---
2019-07-17T23:28:04.9314719Z -Hello, world!
2019-07-17T23:28:04.9316350Z -
2019-07-17T23:28:04.9316393Z 
2019-07-17T23:28:04.9316459Z The actual stdout differed from the expected stdout.
2019-07-17T23:28:04.9316532Z Actual stdout saved to /tmp/compiletestNloeDC/hello.stdout
2019-07-17T23:28:04.9316644Z error[E0080]: Miri evaluation error: attempted to do invalid arithmetic on pointers that would leak base addresses, e.g., comparing pointers into different allocations
2019-07-17T23:28:04.9316934Z     --> /checkout/src/libcore/intrinsics.rs:1340:19
2019-07-17T23:28:04.9316988Z      |
2019-07-17T23:28:04.9317036Z 1340 |     let diff = if src_usize > dst_usize {
---
2019-07-17T23:28:04.9320713Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:04.9320771Z +
2019-07-17T23:28:04.9320795Z 
2019-07-17T23:28:04.9320834Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:04.9320879Z Actual stderr saved to /tmp/compiletestNloeDC/hello.stderr
2019-07-17T23:28:04.9320939Z To update references, run this command from build directory:
2019-07-17T23:28:04.9321163Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'hello.rs'
2019-07-17T23:28:04.9321247Z error: 2 errors occurred comparing output.
2019-07-17T23:28:04.9321286Z status: exit code: 1
2019-07-17T23:28:04.9321286Z status: exit code: 1
2019-07-17T23:28:04.9322013Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/hello.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/hello.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/hello.stage-id.aux" "-A" "unused"
2019-07-17T23:28:04.9322398Z ------------------------------------------
2019-07-17T23:28:04.9322430Z 
2019-07-17T23:28:04.9322635Z ------------------------------------------
2019-07-17T23:28:04.9322676Z stderr:
---
2019-07-17T23:28:04.9645368Z +
2019-07-17T23:28:04.9645397Z 
2019-07-17T23:28:04.9645775Z thread '[ui] run-pass/heap.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
2019-07-17T23:28:04.9645857Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:04.9645909Z Actual stderr saved to /tmp/compiletestNloeDC/heap.stderr
2019-07-17T23:28:04.9645960Z To update references, run this command from build directory:
2019-07-17T23:28:04.9646240Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'heap.rs'
2019-07-17T23:28:04.9646446Z error: 1 errors occurred comparing output.
2019-07-17T23:28:04.9646493Z status: exit code: 1
2019-07-17T23:28:04.9646493Z status: exit code: 1
2019-07-17T23:28:04.9647178Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/heap.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/heap.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/heap.stage-id.aux" "-A" "unused"
2019-07-17T23:28:04.9647515Z ------------------------------------------
2019-07-17T23:28:04.9647551Z 
2019-07-17T23:28:04.9647774Z ------------------------------------------
2019-07-17T23:28:04.9647840Z stderr:
---
2019-07-17T23:28:05.4917344Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:05.4917396Z +
2019-07-17T23:28:05.4917442Z 
2019-07-17T23:28:05.4917492Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:05.4917924Z Actual stderr saved to /tmp/compiletestNloeDC/integer-ops.stderr
2019-07-17T23:28:05.4917973Z To update references, run this command from build directory:
2019-07-17T23:28:05.4918223Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'integer-ops.rs'
2019-07-17T23:28:05.4918294Z error: 1 errors occurred comparing output.
2019-07-17T23:28:05.4918434Z status: exit code: 1
2019-07-17T23:28:05.4918434Z status: exit code: 1
2019-07-17T23:28:05.4919028Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/integer-ops.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/integer-ops.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/integer-ops.stage-id.aux" "-A" "unused"
2019-07-17T23:28:05.4919323Z ------------------------------------------
2019-07-17T23:28:05.4919356Z 
2019-07-17T23:28:05.4919549Z ------------------------------------------
2019-07-17T23:28:05.4919607Z stderr:
---
2019-07-17T23:28:05.9759203Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:05.9759259Z +
2019-07-17T23:28:05.9759282Z 
2019-07-17T23:28:05.9759319Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:05.9759533Z Actual stderr saved to /tmp/compiletestNloeDC/intrinsics-math.stderr
2019-07-17T23:28:05.9759593Z To update references, run this command from build directory:
2019-07-17T23:28:05.9759816Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'intrinsics-math.rs'
2019-07-17T23:28:05.9759971Z error: 1 errors occurred comparing output.
2019-07-17T23:28:05.9760008Z status: exit code: 1
2019-07-17T23:28:05.9760008Z status: exit code: 1
2019-07-17T23:28:05.9760560Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/intrinsics-math.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/intrinsics-math.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/intrinsics-math.stage-id.aux" "-A" "unused"
2019-07-17T23:28:05.9761013Z ------------------------------------------
2019-07-17T23:28:05.9761043Z 
2019-07-17T23:28:05.9761250Z ------------------------------------------
2019-07-17T23:28:05.9761290Z stderr:
---
2019-07-17T23:28:06.1480170Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:06.1480214Z +
2019-07-17T23:28:06.1480238Z 
2019-07-17T23:28:06.1480275Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:06.1480334Z Actual stderr saved to /tmp/compiletestNloeDC/intrinsics.stderr
2019-07-17T23:28:06.1480375Z To update references, run this command from build directory:
2019-07-17T23:28:06.1480735Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'intrinsics.rs'
2019-07-17T23:28:06.1480821Z error: 1 errors occurred comparing output.
2019-07-17T23:28:06.1480857Z status: exit code: 1
2019-07-17T23:28:06.1480857Z status: exit code: 1
2019-07-17T23:28:06.1481401Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/intrinsics.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/intrinsics.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/intrinsics.stage-id.aux" "-A" "unused"
2019-07-17T23:28:06.1481683Z ------------------------------------------
2019-07-17T23:28:06.1481714Z 
2019-07-17T23:28:06.1481906Z ------------------------------------------
2019-07-17T23:28:06.1481944Z stderr:
---
2019-07-17T23:28:06.3441794Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:06.3441838Z +
2019-07-17T23:28:06.3441863Z 
2019-07-17T23:28:06.3441899Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:06.3441957Z Actual stderr saved to /tmp/compiletestNloeDC/ints.stderr
2019-07-17T23:28:06.3441997Z To update references, run this command from build directory:
2019-07-17T23:28:06.3442341Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'ints.rs'
2019-07-17T23:28:06.3442425Z error: 1 errors occurred comparing output.
2019-07-17T23:28:06.3442462Z status: exit code: 1
2019-07-17T23:28:06.3442462Z status: exit code: 1
2019-07-17T23:28:06.3442995Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/ints.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/ints.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/ints.stage-id.aux" "-A" "unused"
2019-07-17T23:28:06.3443248Z ------------------------------------------
2019-07-17T23:28:06.3443295Z 
2019-07-17T23:28:06.3443476Z ------------------------------------------
2019-07-17T23:28:06.3443521Z stderr:
---
2019-07-17T23:28:06.4680805Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:06.4680848Z +
2019-07-17T23:28:06.4680958Z 
2019-07-17T23:28:06.4681017Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:06.4681260Z Actual stderr saved to /tmp/compiletestNloeDC/issue-15063.stderr
2019-07-17T23:28:06.4681307Z To update references, run this command from build directory:
2019-07-17T23:28:06.4681557Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'issue-15063.rs'
2019-07-17T23:28:06.4681626Z error: 1 errors occurred comparing output.
2019-07-17T23:28:06.4681663Z status: exit code: 1
2019-07-17T23:28:06.4681663Z status: exit code: 1
2019-07-17T23:28:06.4682201Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-15063.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/issue-15063.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/issue-15063.stage-id.aux" "-A" "unused"
2019-07-17T23:28:06.4682489Z ------------------------------------------
2019-07-17T23:28:06.4682518Z 
2019-07-17T23:28:06.4682707Z ------------------------------------------
2019-07-17T23:28:06.4682746Z stderr:
---
2019-07-17T23:28:06.6446599Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:06.6446654Z +
2019-07-17T23:28:06.6446684Z 
2019-07-17T23:28:06.6446752Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:06.6447163Z Actual stderr saved to /tmp/compiletestNloeDC/issue-15080.stderr
2019-07-17T23:28:06.6447224Z To update references, run this command from build directory:
2019-07-17T23:28:06.6447529Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'issue-15080.rs'
2019-07-17T23:28:06.6447627Z error: 1 errors occurred comparing output.
2019-07-17T23:28:06.6447675Z status: exit code: 1
2019-07-17T23:28:06.6447675Z status: exit code: 1
2019-07-17T23:28:06.6448366Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-15080.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/issue-15080.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/issue-15080.stage-id.aux" "-A" "unused"
2019-07-17T23:28:06.6448861Z ------------------------------------------
2019-07-17T23:28:06.6448900Z 
2019-07-17T23:28:06.6449087Z ------------------------------------------
2019-07-17T23:28:06.6449146Z stderr:
---
2019-07-17T23:28:06.8123465Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:06.8123507Z +
2019-07-17T23:28:06.8123549Z 
2019-07-17T23:28:06.8123588Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:06.8123798Z Actual stderr saved to /tmp/compiletestNloeDC/issue-15523-big.stderr
2019-07-17T23:28:06.8123860Z To update references, run this command from build directory:
2019-07-17T23:28:06.8124082Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'issue-15523-big.rs'
2019-07-17T23:28:06.8124149Z error: 1 errors occurred comparing output.
2019-07-17T23:28:06.8124201Z status: exit code: 1
2019-07-17T23:28:06.8124201Z status: exit code: 1
2019-07-17T23:28:06.8124727Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-15523-big.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/issue-15523-big.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/issue-15523-big.stage-id.aux" "-A" "unused"
2019-07-17T23:28:06.8125426Z ------------------------------------------
2019-07-17T23:28:06.8125573Z 
2019-07-17T23:28:06.8125879Z ------------------------------------------
2019-07-17T23:28:06.8125929Z stderr:
---
2019-07-17T23:28:06.9561773Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:06.9561814Z +
2019-07-17T23:28:06.9561837Z 
2019-07-17T23:28:06.9561896Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:06.9562097Z Actual stderr saved to /tmp/compiletestNloeDC/issue-17877.stderr
2019-07-17T23:28:06.9562142Z To update references, run this command from build directory:
2019-07-17T23:28:06.9562361Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'issue-17877.rs'
2019-07-17T23:28:06.9562425Z error: 1 errors occurred comparing output.
2019-07-17T23:28:06.9562476Z status: exit code: 1
2019-07-17T23:28:06.9562476Z status: exit code: 1
2019-07-17T23:28:06.9562978Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-17877.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/issue-17877.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/issue-17877.stage-id.aux" "-A" "unused"
2019-07-17T23:28:06.9563254Z ------------------------------------------
2019-07-17T23:28:06.9563281Z 
2019-07-17T23:28:06.9563456Z ------------------------------------------
2019-07-17T23:28:06.9563508Z stderr:
---
2019-07-17T23:28:07.0963413Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:07.0963457Z +
2019-07-17T23:28:07.0963481Z 
2019-07-17T23:28:07.0963536Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:07.0963760Z Actual stderr saved to /tmp/compiletestNloeDC/issue-20575.stderr
2019-07-17T23:28:07.0963806Z To update references, run this command from build directory:
2019-07-17T23:28:07.0964046Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'issue-20575.rs'
2019-07-17T23:28:07.0964116Z error: 1 errors occurred comparing output.
2019-07-17T23:28:07.0964169Z status: exit code: 1
2019-07-17T23:28:07.0964169Z status: exit code: 1
2019-07-17T23:28:07.0965463Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-20575.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/issue-20575.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/issue-20575.stage-id.aux" "-A" "unused"
2019-07-17T23:28:07.0965852Z ------------------------------------------
2019-07-17T23:28:07.0965889Z 
2019-07-17T23:28:07.0966114Z ------------------------------------------
2019-07-17T23:28:07.0966180Z stderr:
---
2019-07-17T23:28:07.2759534Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:07.2759579Z +
2019-07-17T23:28:07.2759619Z 
2019-07-17T23:28:07.2759665Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:07.2759901Z Actual stderr saved to /tmp/compiletestNloeDC/issue-23261.stderr
2019-07-17T23:28:07.2759963Z To update references, run this command from build directory:
2019-07-17T23:28:07.2760192Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'issue-23261.rs'
2019-07-17T23:28:07.2760259Z error: 1 errors occurred comparing output.
2019-07-17T23:28:07.2760311Z status: exit code: 1
2019-07-17T23:28:07.2760311Z status: exit code: 1
2019-07-17T23:28:07.2760845Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-23261.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/issue-23261.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/issue-23261.stage-id.aux" "-A" "unused"
2019-07-17T23:28:07.2761137Z ------------------------------------------
2019-07-17T23:28:07.2761167Z 
2019-07-17T23:28:07.2761359Z ------------------------------------------
2019-07-17T23:28:07.2761415Z stderr:
---
2019-07-17T23:28:07.4321705Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:07.4321767Z +
2019-07-17T23:28:07.4321792Z 
2019-07-17T23:28:07.4321831Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:07.4322058Z Actual stderr saved to /tmp/compiletestNloeDC/issue-26709.stderr
2019-07-17T23:28:07.4322123Z To update references, run this command from build directory:
2019-07-17T23:28:07.4322355Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'issue-26709.rs'
2019-07-17T23:28:07.4322424Z error: 1 errors occurred comparing output.
2019-07-17T23:28:07.4322487Z status: exit code: 1
2019-07-17T23:28:07.4322487Z status: exit code: 1
2019-07-17T23:28:07.4323043Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-26709.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/issue-26709.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/issue-26709.stage-id.aux" "-A" "unused"
2019-07-17T23:28:07.4323338Z ------------------------------------------
2019-07-17T23:28:07.4323368Z 
2019-07-17T23:28:07.4323580Z ------------------------------------------
2019-07-17T23:28:07.4323619Z stderr:
---
2019-07-17T23:28:07.5788555Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:07.5788806Z +
2019-07-17T23:28:07.5788948Z 
2019-07-17T23:28:07.5789101Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:07.5789512Z Actual stderr saved to /tmp/compiletestNloeDC/issue-27901.stderr
2019-07-17T23:28:07.5789727Z To update references, run this command from build directory:
2019-07-17T23:28:07.5790127Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'issue-27901.rs'
2019-07-17T23:28:07.5790484Z error: 1 errors occurred comparing output.
2019-07-17T23:28:07.5790637Z status: exit code: 1
2019-07-17T23:28:07.5790637Z status: exit code: 1
2019-07-17T23:28:07.5791346Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-27901.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/issue-27901.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/issue-27901.stage-id.aux" "-A" "unused"
2019-07-17T23:28:07.5791990Z ------------------------------------------
2019-07-17T23:28:07.5792185Z 
2019-07-17T23:28:07.5792559Z ------------------------------------------
2019-07-17T23:28:07.5792767Z stderr:
---
2019-07-17T23:28:07.7437370Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:07.7437442Z +
2019-07-17T23:28:07.7437472Z 
2019-07-17T23:28:07.7437521Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:07.7437802Z Actual stderr saved to /tmp/compiletestNloeDC/issue-29746.stderr
2019-07-17T23:28:07.7438048Z To update references, run this command from build directory:
2019-07-17T23:28:07.7438264Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'issue-29746.rs'
2019-07-17T23:28:07.7438346Z error: 1 errors occurred comparing output.
2019-07-17T23:28:07.7438389Z status: exit code: 1
2019-07-17T23:28:07.7438389Z status: exit code: 1
2019-07-17T23:28:07.7438909Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-29746.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/issue-29746.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/issue-29746.stage-id.aux" "-A" "unused"
2019-07-17T23:28:07.7439173Z ------------------------------------------
2019-07-17T23:28:07.7439201Z 
2019-07-17T23:28:07.7439391Z ------------------------------------------
2019-07-17T23:28:07.7439429Z stderr:
---
2019-07-17T23:28:07.7985330Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:07.7985381Z +
2019-07-17T23:28:07.7985411Z 
2019-07-17T23:28:07.7985456Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:07.7985734Z Actual stderr saved to /tmp/compiletestNloeDC/intrinsics-integer.stderr
2019-07-17T23:28:07.7985791Z To update references, run this command from build directory:
2019-07-17T23:28:07.7986062Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'intrinsics-integer.rs'
2019-07-17T23:28:07.7986171Z error: 1 errors occurred comparing output.
2019-07-17T23:28:07.7986216Z status: exit code: 1
2019-07-17T23:28:07.7986216Z status: exit code: 1
2019-07-17T23:28:07.7986915Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/intrinsics-integer.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/intrinsics-integer.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/intrinsics-integer.stage-id.aux" "-A" "unused"
2019-07-17T23:28:07.7987237Z ------------------------------------------
2019-07-17T23:28:07.7987292Z 
2019-07-17T23:28:07.7987526Z ------------------------------------------
2019-07-17T23:28:07.7987575Z stderr:
---
2019-07-17T23:28:07.8886827Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:07.8886882Z +
2019-07-17T23:28:07.8886913Z 
2019-07-17T23:28:07.8886985Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:07.8887285Z Actual stderr saved to /tmp/compiletestNloeDC/issue-30530.stderr
2019-07-17T23:28:07.8887344Z To update references, run this command from build directory:
2019-07-17T23:28:07.8887652Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'issue-30530.rs'
2019-07-17T23:28:07.8887747Z error: 1 errors occurred comparing output.
2019-07-17T23:28:07.8887795Z status: exit code: 1
2019-07-17T23:28:07.8887795Z status: exit code: 1
2019-07-17T23:28:07.8889251Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-30530.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/issue-30530.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/issue-30530.stage-id.aux" "-A" "unused"
2019-07-17T23:28:07.8889694Z ------------------------------------------
2019-07-17T23:28:07.8889732Z 
2019-07-17T23:28:07.8890073Z ------------------------------------------
2019-07-17T23:28:07.8890127Z stderr:
---
2019-07-17T23:28:07.9400218Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:07.9400260Z +
2019-07-17T23:28:07.9400299Z 
2019-07-17T23:28:07.9400337Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:07.9400562Z Actual stderr saved to /tmp/compiletestNloeDC/issue-31267-additional.stderr
2019-07-17T23:28:07.9400632Z To update references, run this command from build directory:
2019-07-17T23:28:07.9401078Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'issue-31267-additional.rs'
2019-07-17T23:28:07.9401146Z error: 1 errors occurred comparing output.
2019-07-17T23:28:07.9401204Z status: exit code: 1
2019-07-17T23:28:07.9401204Z status: exit code: 1
2019-07-17T23:28:07.9401771Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-31267-additional.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/issue-31267-additional.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/issue-31267-additional.stage-id.aux" "-A" "unused"
2019-07-17T23:28:07.9402051Z ------------------------------------------
2019-07-17T23:28:07.9402087Z 
2019-07-17T23:28:07.9402293Z ------------------------------------------
2019-07-17T23:28:07.9402332Z stderr:
---
2019-07-17T23:28:08.0202614Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:08.0202658Z +
2019-07-17T23:28:08.0202681Z 
2019-07-17T23:28:08.0202720Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:08.0202969Z Actual stderr saved to /tmp/compiletestNloeDC/issue-33387.stderr
2019-07-17T23:28:08.0203016Z To update references, run this command from build directory:
2019-07-17T23:28:08.0203245Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'issue-33387.rs'
2019-07-17T23:28:08.0203330Z error: 1 errors occurred comparing output.
2019-07-17T23:28:08.0203367Z status: exit code: 1
2019-07-17T23:28:08.0203367Z status: exit code: 1
2019-07-17T23:28:08.0204106Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-33387.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/issue-33387.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/issue-33387.stage-id.aux" "-A" "unused"
2019-07-17T23:28:08.0204746Z ------------------------------------------
2019-07-17T23:28:08.0204794Z 
2019-07-17T23:28:08.0205827Z ------------------------------------------
2019-07-17T23:28:08.0205885Z stderr:
---
2019-07-17T23:28:08.0587674Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:08.0587737Z +
2019-07-17T23:28:08.0587765Z 
2019-07-17T23:28:08.0587994Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:08.0588381Z Actual stderr saved to /tmp/compiletestNloeDC/issue-34571.stderr
2019-07-17T23:28:08.0588596Z To update references, run this command from build directory:
2019-07-17T23:28:08.0588828Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'issue-34571.rs'
2019-07-17T23:28:08.0588892Z error: 1 errors occurred comparing output.
2019-07-17T23:28:08.0588928Z status: exit code: 1
2019-07-17T23:28:08.0588928Z status: exit code: 1
2019-07-17T23:28:08.0589474Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-34571.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/issue-34571.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/issue-34571.stage-id.aux" "-A" "unused"
2019-07-17T23:28:08.0589755Z ------------------------------------------
2019-07-17T23:28:08.0589783Z 
2019-07-17T23:28:08.0589964Z ------------------------------------------
2019-07-17T23:28:08.0590019Z stderr:
---
2019-07-17T23:28:08.1642726Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:08.1642770Z +
2019-07-17T23:28:08.1642794Z 
2019-07-17T23:28:08.1642847Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:08.1643066Z Actual stderr saved to /tmp/compiletestNloeDC/issue-35815.stderr
2019-07-17T23:28:08.1643111Z To update references, run this command from build directory:
2019-07-17T23:28:08.1643327Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'issue-35815.rs'
2019-07-17T23:28:08.1643408Z error: 1 errors occurred comparing output.
2019-07-17T23:28:08.1643445Z status: exit code: 1
2019-07-17T23:28:08.1643445Z status: exit code: 1
2019-07-17T23:28:08.1643987Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-35815.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/issue-35815.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/issue-35815.stage-id.aux" "-A" "unused"
2019-07-17T23:28:08.1644268Z ------------------------------------------
2019-07-17T23:28:08.1644297Z 
2019-07-17T23:28:08.1644484Z ------------------------------------------
2019-07-17T23:28:08.1644523Z stderr:
---
2019-07-17T23:28:08.1996742Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:08.1996796Z +
2019-07-17T23:28:08.1996826Z 
2019-07-17T23:28:08.1996892Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:08.1997189Z Actual stderr saved to /tmp/compiletestNloeDC/issue-36278-prefix-nesting.stderr
2019-07-17T23:28:08.1997248Z To update references, run this command from build directory:
2019-07-17T23:28:08.1997566Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'issue-36278-prefix-nesting.rs'
2019-07-17T23:28:08.1997652Z error: 1 errors occurred comparing output.
2019-07-17T23:28:08.1997716Z status: exit code: 1
2019-07-17T23:28:08.1997716Z status: exit code: 1
2019-07-17T23:28:08.1998568Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-36278-prefix-nesting.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/issue-36278-prefix-nesting.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/issue-36278-prefix-nesting.stage-id.aux" "-A" "unused"
2019-07-17T23:28:08.1998852Z ------------------------------------------
2019-07-17T23:28:08.1998880Z 
2019-07-17T23:28:08.1999057Z ------------------------------------------
2019-07-17T23:28:08.1999111Z stderr:
---
2019-07-17T23:28:08.3169270Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:08.3169327Z +
2019-07-17T23:28:08.3169349Z 
2019-07-17T23:28:08.3169386Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:08.3169591Z Actual stderr saved to /tmp/compiletestNloeDC/issue-53728.stderr
2019-07-17T23:28:08.3169652Z To update references, run this command from build directory:
2019-07-17T23:28:08.3169866Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'issue-53728.rs'
2019-07-17T23:28:08.3169951Z error: 1 errors occurred comparing output.
2019-07-17T23:28:08.3169989Z status: exit code: 1
2019-07-17T23:28:08.3169989Z status: exit code: 1
2019-07-17T23:28:08.3170524Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-53728.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/issue-53728.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/issue-53728.stage-id.aux" "-A" "unused"
2019-07-17T23:28:08.3170799Z ------------------------------------------
2019-07-17T23:28:08.3170828Z 
2019-07-17T23:28:08.3171026Z ------------------------------------------
2019-07-17T23:28:08.3171065Z stderr:
---
2019-07-17T23:28:08.3181156Z -S { s: 5 }
2019-07-17T23:28:08.3181295Z -
2019-07-17T23:28:08.3181319Z 
2019-07-17T23:28:08.3181370Z The actual stdout differed from the expected stdout.
2019-07-17T23:28:08.3181567Z Actual stdout saved to /tmp/compiletestNloeDC/issue-3794.stdout
2019-07-17T23:28:08.3181675Z error[E0080]: Miri evaluation error: attempted to do invalid arithmetic on pointers that would leak base addresses, e.g., comparing pointers into different allocations
2019-07-17T23:28:08.3181868Z     --> /checkout/src/libcore/intrinsics.rs:1340:19
2019-07-17T23:28:08.3181908Z      |
2019-07-17T23:28:08.3181960Z 1340 |     let diff = if src_usize > dst_usize {
---
2019-07-17T23:28:08.3186531Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:08.3186603Z +
2019-07-17T23:28:08.3186633Z 
2019-07-17T23:28:08.3186682Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:08.3186979Z Actual stderr saved to /tmp/compiletestNloeDC/issue-3794.stderr
2019-07-17T23:28:08.3187039Z To update references, run this command from build directory:
2019-07-17T23:28:08.3187322Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'issue-3794.rs'
2019-07-17T23:28:08.3187423Z error: 2 errors occurred comparing output.
2019-07-17T23:28:08.3187472Z status: exit code: 1
2019-07-17T23:28:08.3187472Z status: exit code: 1
2019-07-17T23:28:08.3188135Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-3794.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/issue-3794.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/issue-3794.stage-id.aux" "-A" "unused"
2019-07-17T23:28:08.3188780Z ------------------------------------------
2019-07-17T23:28:08.3188823Z 
2019-07-17T23:28:08.3189088Z ------------------------------------------
2019-07-17T23:28:08.3189134Z stderr:
---
2019-07-17T23:28:08.4442247Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:08.4442310Z +
2019-07-17T23:28:08.4442334Z 
2019-07-17T23:28:08.4442381Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:08.4442639Z Actual stderr saved to /tmp/compiletestNloeDC/issue-miri-184.stderr
2019-07-17T23:28:08.4442687Z To update references, run this command from build directory:
2019-07-17T23:28:08.4442929Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'issue-miri-184.rs'
2019-07-17T23:28:08.4443014Z error: 1 errors occurred comparing output.
2019-07-17T23:28:08.4443052Z status: exit code: 1
2019-07-17T23:28:08.4443052Z status: exit code: 1
2019-07-17T23:28:08.4443592Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-miri-184.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/issue-miri-184.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/issue-miri-184.stage-id.aux" "-A" "unused"
2019-07-17T23:28:08.4444029Z ------------------------------------------
2019-07-17T23:28:08.4444090Z 
2019-07-17T23:28:08.4444999Z ------------------------------------------
2019-07-17T23:28:08.4445057Z stderr:
---
2019-07-17T23:28:08.4493654Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:08.4493699Z +
2019-07-17T23:28:08.4493736Z 
2019-07-17T23:28:08.4493772Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:08.4493973Z Actual stderr saved to /tmp/compiletestNloeDC/issue-5917.stderr
2019-07-17T23:28:08.4494017Z To update references, run this command from build directory:
2019-07-17T23:28:08.4494240Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'issue-5917.rs'
2019-07-17T23:28:08.4494304Z error: 1 errors occurred comparing output.
2019-07-17T23:28:08.4494355Z status: exit code: 1
2019-07-17T23:28:08.4494355Z status: exit code: 1
2019-07-17T23:28:08.4495865Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-5917.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/issue-5917.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/issue-5917.stage-id.aux" "-A" "unused"
2019-07-17T23:28:08.4496270Z ------------------------------------------
2019-07-17T23:28:08.4496308Z 
2019-07-17T23:28:08.4496534Z ------------------------------------------
2019-07-17T23:28:08.4496599Z stderr:
---
2019-07-17T23:28:08.5921666Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:08.5922235Z +
2019-07-17T23:28:08.5922267Z 
2019-07-17T23:28:08.5922305Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:08.5922672Z Actual stderr saved to /tmp/compiletestNloeDC/last-use-in-cap-clause.stderr
2019-07-17T23:28:08.5922724Z To update references, run this command from build directory:
2019-07-17T23:28:08.5926141Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'last-use-in-cap-clause.rs'
2019-07-17T23:28:08.5927329Z error: 1 errors occurred comparing output.
2019-07-17T23:28:08.5930478Z status: exit code: 1
2019-07-17T23:28:08.5930478Z status: exit code: 1
2019-07-17T23:28:08.5932730Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/last-use-in-cap-clause.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/last-use-in-cap-clause.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/last-use-in-cap-clause.stage-id.aux" "-A" "unused"
2019-07-17T23:28:08.5933452Z ------------------------------------------
2019-07-17T23:28:08.5933630Z 
2019-07-17T23:28:08.5933969Z ------------------------------------------
2019-07-17T23:28:08.5934117Z stderr:
---
2019-07-17T23:28:08.6556727Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:08.6556778Z +
2019-07-17T23:28:08.6556806Z 
2019-07-17T23:28:08.6556870Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:08.6556921Z Actual stderr saved to /tmp/compiletestNloeDC/iter.stderr
2019-07-17T23:28:08.6556972Z To update references, run this command from build directory:
2019-07-17T23:28:08.6557247Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'iter.rs'
2019-07-17T23:28:08.6557329Z error: 1 errors occurred comparing output.
2019-07-17T23:28:08.6557401Z status: exit code: 1
2019-07-17T23:28:08.6557401Z status: exit code: 1
2019-07-17T23:28:08.6558244Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/iter.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/iter.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/iter.stage-id.aux" "-A" "unused"
2019-07-17T23:28:08.6558545Z ------------------------------------------
2019-07-17T23:28:08.6558575Z 
2019-07-17T23:28:08.6558753Z ------------------------------------------
2019-07-17T23:28:08.6558807Z stderr:
---
2019-07-17T23:28:08.8280561Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:08.8280610Z +
2019-07-17T23:28:08.8280636Z 
2019-07-17T23:28:08.8280703Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:08.8280951Z Actual stderr saved to /tmp/compiletestNloeDC/linked-list.stderr
2019-07-17T23:28:08.8281000Z To update references, run this command from build directory:
2019-07-17T23:28:08.8281259Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'linked-list.rs'
2019-07-17T23:28:08.8281342Z error: 1 errors occurred comparing output.
2019-07-17T23:28:08.8281400Z status: exit code: 1
2019-07-17T23:28:08.8281400Z status: exit code: 1
2019-07-17T23:28:08.8282099Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/linked-list.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/linked-list.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/linked-list.stage-id.aux" "-A" "unused"
2019-07-17T23:28:08.8282446Z ------------------------------------------
2019-07-17T23:28:08.8282479Z 
2019-07-17T23:28:08.8282686Z ------------------------------------------
2019-07-17T23:28:08.8282747Z stderr:
---
2019-07-17T23:28:08.9075409Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:08.9075466Z +
2019-07-17T23:28:08.9075515Z 
2019-07-17T23:28:08.9075562Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:08.9075822Z Actual stderr saved to /tmp/compiletestNloeDC/loop-break-value.stderr
2019-07-17T23:28:08.9075878Z To update references, run this command from build directory:
2019-07-17T23:28:08.9076167Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'loop-break-value.rs'
2019-07-17T23:28:08.9076262Z error: 1 errors occurred comparing output.
2019-07-17T23:28:08.9076326Z status: exit code: 1
2019-07-17T23:28:08.9076326Z status: exit code: 1
2019-07-17T23:28:08.9077168Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/loop-break-value.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/loop-break-value.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/loop-break-value.stage-id.aux" "-A" "unused"
2019-07-17T23:28:08.9077543Z ------------------------------------------
2019-07-17T23:28:08.9077579Z 
2019-07-17T23:28:08.9077803Z ------------------------------------------
2019-07-17T23:28:08.9077869Z stderr:
---
2019-07-17T23:28:09.0127531Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:09.0127586Z +
2019-07-17T23:28:09.0127614Z 
2019-07-17T23:28:09.0127680Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:09.0127735Z Actual stderr saved to /tmp/compiletestNloeDC/loops.stderr
2019-07-17T23:28:09.0127786Z To update references, run this command from build directory:
2019-07-17T23:28:09.0128082Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'loops.rs'
2019-07-17T23:28:09.0128164Z error: 1 errors occurred comparing output.
2019-07-17T23:28:09.0128227Z status: exit code: 1
2019-07-17T23:28:09.0128227Z status: exit code: 1
2019-07-17T23:28:09.0129193Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/loops.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/loops.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/loops.stage-id.aux" "-A" "unused"
2019-07-17T23:28:09.0129509Z ------------------------------------------
2019-07-17T23:28:09.0129538Z 
2019-07-17T23:28:09.0129716Z ------------------------------------------
2019-07-17T23:28:09.0129850Z stderr:
---
2019-07-17T23:28:09.0355489Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:09.0355539Z +
2019-07-17T23:28:09.0355597Z 
2019-07-17T23:28:09.0355643Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:09.0355695Z Actual stderr saved to /tmp/compiletestNloeDC/main_fn.stderr
2019-07-17T23:28:09.0355765Z To update references, run this command from build directory:
2019-07-17T23:28:09.0356135Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'main_fn.rs'
2019-07-17T23:28:09.0356229Z error: 1 errors occurred comparing output.
2019-07-17T23:28:09.0356297Z status: exit code: 1
2019-07-17T23:28:09.0356297Z status: exit code: 1
2019-07-17T23:28:09.0356958Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/main_fn.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/main_fn.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/main_fn.stage-id.aux" "-A" "unused"
2019-07-17T23:28:09.0357397Z ------------------------------------------
2019-07-17T23:28:09.0357434Z 
2019-07-17T23:28:09.0357677Z ------------------------------------------
2019-07-17T23:28:09.0357726Z stderr:
---
2019-07-17T23:28:09.1401313Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:09.1401388Z +
2019-07-17T23:28:09.1401413Z 
2019-07-17T23:28:09.1401532Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:09.1401576Z Actual stderr saved to /tmp/compiletestNloeDC/many_shr_bor.stderr
2019-07-17T23:28:09.1401758Z To update references, run this command from build directory:
2019-07-17T23:28:09.1402065Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'many_shr_bor.rs'
2019-07-17T23:28:09.1402133Z error: 1 errors occurred comparing output.
2019-07-17T23:28:09.1402192Z status: exit code: 1
2019-07-17T23:28:09.1402192Z status: exit code: 1
2019-07-17T23:28:09.1402720Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/many_shr_bor.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/many_shr_bor.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/many_shr_bor.stage-id.aux" "-A" "unused"
2019-07-17T23:28:09.1403101Z ------------------------------------------
2019-07-17T23:28:09.1403132Z 
2019-07-17T23:28:09.1403311Z ------------------------------------------
2019-07-17T23:28:09.1403376Z stderr:
---
2019-07-17T23:28:09.1521029Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:09.1521819Z +
2019-07-17T23:28:09.1522483Z 
2019-07-17T23:28:09.1523135Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:09.1523527Z Actual stderr saved to /tmp/compiletestNloeDC/match_slice.stderr
2019-07-17T23:28:09.1523667Z To update references, run this command from build directory:
2019-07-17T23:28:09.1524262Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'match_slice.rs'
2019-07-17T23:28:09.1524575Z error: 1 errors occurred comparing output.
2019-07-17T23:28:09.1525172Z status: exit code: 1
2019-07-17T23:28:09.1525172Z status: exit code: 1
2019-07-17T23:28:09.1526045Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/match_slice.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/match_slice.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/match_slice.stage-id.aux" "-A" "unused"
2019-07-17T23:28:09.1526807Z ------------------------------------------
2019-07-17T23:28:09.1526963Z 
2019-07-17T23:28:09.1527329Z ------------------------------------------
2019-07-17T23:28:09.1527492Z stderr:
---
2019-07-17T23:28:09.3396678Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:09.3396755Z +
2019-07-17T23:28:09.3396784Z 
2019-07-17T23:28:09.3396832Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:09.3396902Z Actual stderr saved to /tmp/compiletestNloeDC/mir_coercions.stderr
2019-07-17T23:28:09.3397096Z To update references, run this command from build directory:
2019-07-17T23:28:09.3397439Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'mir_coercions.rs'
2019-07-17T23:28:09.3397543Z error: 1 errors occurred comparing output.
2019-07-17T23:28:09.3397591Z status: exit code: 1
2019-07-17T23:28:09.3397591Z status: exit code: 1
2019-07-17T23:28:09.3398284Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/mir_coercions.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/mir_coercions.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/mir_coercions.stage-id.aux" "-A" "unused"
2019-07-17T23:28:09.3398882Z ------------------------------------------
2019-07-17T23:28:09.3398927Z 
2019-07-17T23:28:09.3399116Z ------------------------------------------
2019-07-17T23:28:09.3399163Z stderr:
---
2019-07-17T23:28:09.3489524Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:09.3489565Z +
2019-07-17T23:28:09.3489652Z 
2019-07-17T23:28:09.3489692Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:09.3489828Z Actual stderr saved to /tmp/compiletestNloeDC/memchr.stderr
2019-07-17T23:28:09.3489878Z To update references, run this command from build directory:
2019-07-17T23:28:09.3490147Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'memchr.rs'
2019-07-17T23:28:09.3490211Z error: 1 errors occurred comparing output.
2019-07-17T23:28:09.3490264Z status: exit code: 1
2019-07-17T23:28:09.3490264Z status: exit code: 1
2019-07-17T23:28:09.3490776Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/memchr.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/memchr.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/memchr.stage-id.aux" "-A" "unused"
2019-07-17T23:28:09.3491145Z ------------------------------------------
2019-07-17T23:28:09.3491175Z 
2019-07-17T23:28:09.3491365Z ------------------------------------------
2019-07-17T23:28:09.3491422Z stderr:
---
2019-07-17T23:28:09.4880855Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:09.4880894Z +
2019-07-17T23:28:09.4880915Z 
2019-07-17T23:28:09.4880950Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:09.4881241Z Actual stderr saved to /tmp/compiletestNloeDC/miri-issue-133.stderr
2019-07-17T23:28:09.4881295Z To update references, run this command from build directory:
2019-07-17T23:28:09.4881532Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'miri-issue-133.rs'
2019-07-17T23:28:09.4881612Z error: 1 errors occurred comparing output.
2019-07-17T23:28:09.4881648Z status: exit code: 1
2019-07-17T23:28:09.4881648Z status: exit code: 1
2019-07-17T23:28:09.4882190Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/miri-issue-133.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/miri-issue-133.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/miri-issue-133.stage-id.aux" "-A" "unused"
2019-07-17T23:28:09.4882540Z ------------------------------------------
2019-07-17T23:28:09.4882586Z 
2019-07-17T23:28:09.4882766Z ------------------------------------------
2019-07-17T23:28:09.4882802Z stderr:
---
2019-07-17T23:28:09.4896605Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:09.4896680Z +
2019-07-17T23:28:09.4896706Z 
2019-07-17T23:28:09.4896752Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:09.4896822Z Actual stderr saved to /tmp/compiletestNloeDC/mir_fat_ptr.stderr
2019-07-17T23:28:09.4896960Z To update references, run this command from build directory:
2019-07-17T23:28:09.4897263Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'mir_fat_ptr.rs'
2019-07-17T23:28:09.4897362Z error: 1 errors occurred comparing output.
2019-07-17T23:28:09.4897408Z status: exit code: 1
2019-07-17T23:28:09.4897408Z status: exit code: 1
2019-07-17T23:28:09.4898730Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/mir_fat_ptr.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/mir_fat_ptr.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/mir_fat_ptr.stage-id.aux" "-A" "unused"
2019-07-17T23:28:09.4899407Z ------------------------------------------
2019-07-17T23:28:09.4899452Z 
2019-07-17T23:28:09.4899637Z ------------------------------------------
2019-07-17T23:28:09.4899673Z stderr:
---
2019-07-17T23:28:09.6322100Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:09.6322147Z +
2019-07-17T23:28:09.6322171Z 
2019-07-17T23:28:09.6322231Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:09.6322476Z Actual stderr saved to /tmp/compiletestNloeDC/move-arg-2-unique.stderr
2019-07-17T23:28:09.6322526Z To update references, run this command from build directory:
2019-07-17T23:28:09.6322782Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'move-arg-2-unique.rs'
2019-07-17T23:28:09.6322960Z error: 1 errors occurred comparing output.
2019-07-17T23:28:09.6323000Z status: exit code: 1
2019-07-17T23:28:09.6323000Z status: exit code: 1
2019-07-17T23:28:09.6323621Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/move-arg-2-unique.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/move-arg-2-unique.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/move-arg-2-unique.stage-id.aux" "-A" "unused"
2019-07-17T23:28:09.6323923Z ------------------------------------------
2019-07-17T23:28:09.6323955Z 
2019-07-17T23:28:09.6324158Z ------------------------------------------
2019-07-17T23:28:09.6324199Z stderr:
---
2019-07-17T23:28:09.6434744Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:09.6434806Z +
2019-07-17T23:28:09.6434830Z 
2019-07-17T23:28:09.6434869Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:09.6435532Z Actual stderr saved to /tmp/compiletestNloeDC/move-arg-3-unique.stderr
2019-07-17T23:28:09.6435618Z To update references, run this command from build directory:
2019-07-17T23:28:09.6436033Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'move-arg-3-unique.rs'
2019-07-17T23:28:09.6436139Z error: 1 errors occurred comparing output.
2019-07-17T23:28:09.6436186Z status: exit code: 1
2019-07-17T23:28:09.6436186Z status: exit code: 1
2019-07-17T23:28:09.6436860Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/move-arg-3-unique.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/move-arg-3-unique.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/move-arg-3-unique.stage-id.aux" "-A" "unused"
2019-07-17T23:28:09.6437201Z ------------------------------------------
2019-07-17T23:28:09.6437238Z 
2019-07-17T23:28:09.6437480Z ------------------------------------------
2019-07-17T23:28:09.6437528Z stderr:
---
2019-07-17T23:28:09.7604440Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:09.7604489Z +
2019-07-17T23:28:09.7604515Z 
2019-07-17T23:28:09.7605086Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:09.7605428Z Actual stderr saved to /tmp/compiletestNloeDC/move-undef-primval.stderr
2019-07-17T23:28:09.7605491Z To update references, run this command from build directory:
2019-07-17T23:28:09.7605783Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'move-undef-primval.rs'
2019-07-17T23:28:09.7606010Z error: 1 errors occurred comparing output.
2019-07-17T23:28:09.7606075Z status: exit code: 1
2019-07-17T23:28:09.7606075Z status: exit code: 1
2019-07-17T23:28:09.7606781Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/move-undef-primval.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/move-undef-primval.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/move-undef-primval.stage-id.aux" "-A" "unused"
2019-07-17T23:28:09.7607123Z ------------------------------------------
2019-07-17T23:28:09.7607159Z 
2019-07-17T23:28:09.7607382Z ------------------------------------------
2019-07-17T23:28:09.7607447Z stderr:
---
2019-07-17T23:28:09.8073781Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:09.8073839Z +
2019-07-17T23:28:09.8073863Z 
2019-07-17T23:28:09.8073901Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:09.8073961Z Actual stderr saved to /tmp/compiletestNloeDC/mpsc.stderr
2019-07-17T23:28:09.8074004Z To update references, run this command from build directory:
2019-07-17T23:28:09.8074324Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'mpsc.rs'
2019-07-17T23:28:09.8074413Z error: 1 errors occurred comparing output.
2019-07-17T23:28:09.8074451Z status: exit code: 1
2019-07-17T23:28:09.8074451Z status: exit code: 1
2019-07-17T23:28:09.8075388Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/mpsc.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/mpsc.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/mpsc.stage-id.aux" "-A" "unused"
2019-07-17T23:28:09.8075736Z ------------------------------------------
2019-07-17T23:28:09.8075790Z 
2019-07-17T23:28:09.8076018Z ------------------------------------------
2019-07-17T23:28:09.8076066Z stderr:
---
2019-07-17T23:28:09.9075878Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:09.9075934Z +
2019-07-17T23:28:09.9075964Z 
2019-07-17T23:28:09.9076028Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:09.9076083Z Actual stderr saved to /tmp/compiletestNloeDC/multi_arg_closure.stderr
2019-07-17T23:28:09.9076260Z To update references, run this command from build directory:
2019-07-17T23:28:09.9076578Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'multi_arg_closure.rs'
2019-07-17T23:28:09.9076923Z error: 1 errors occurred comparing output.
2019-07-17T23:28:09.9077014Z status: exit code: 1
2019-07-17T23:28:09.9077014Z status: exit code: 1
2019-07-17T23:28:09.9077812Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/multi_arg_closure.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/multi_arg_closure.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/multi_arg_closure.stage-id.aux" "-A" "unused"
2019-07-17T23:28:09.9078683Z ------------------------------------------
2019-07-17T23:28:09.9078713Z 
2019-07-17T23:28:09.9079055Z ------------------------------------------
2019-07-17T23:28:09.9079229Z stderr:
---
2019-07-17T23:28:09.9220118Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:09.9220246Z +
2019-07-17T23:28:09.9220270Z 
2019-07-17T23:28:09.9220307Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:09.9220366Z Actual stderr saved to /tmp/compiletestNloeDC/negative_discriminant.stderr
2019-07-17T23:28:09.9220415Z To update references, run this command from build directory:
2019-07-17T23:28:09.9220684Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'negative_discriminant.rs'
2019-07-17T23:28:09.9220770Z error: 1 errors occurred comparing output.
2019-07-17T23:28:09.9220807Z status: exit code: 1
2019-07-17T23:28:09.9220807Z status: exit code: 1
2019-07-17T23:28:09.9221373Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/negative_discriminant.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/negative_discriminant.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/negative_discriminant.stage-id.aux" "-A" "unused"
2019-07-17T23:28:09.9221665Z ------------------------------------------
2019-07-17T23:28:09.9221702Z 
2019-07-17T23:28:09.9221900Z ------------------------------------------
2019-07-17T23:28:09.9221939Z stderr:
---
2019-07-17T23:28:10.0609574Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:10.0609702Z +
2019-07-17T23:28:10.0609726Z 
2019-07-17T23:28:10.0609779Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:10.0609823Z Actual stderr saved to /tmp/compiletestNloeDC/observed_local_mut.stderr
2019-07-17T23:28:10.0609872Z To update references, run this command from build directory:
2019-07-17T23:28:10.0610158Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'observed_local_mut.rs'
2019-07-17T23:28:10.0610225Z error: 1 errors occurred comparing output.
2019-07-17T23:28:10.0610275Z status: exit code: 1
2019-07-17T23:28:10.0610275Z status: exit code: 1
2019-07-17T23:28:10.0610860Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/observed_local_mut.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/observed_local_mut.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-Zmiri-disable-validation" "-L" "/tmp/compiletestNloeDC/observed_local_mut.stage-id.aux" "-A" "unused"
2019-07-17T23:28:10.0611153Z ------------------------------------------
2019-07-17T23:28:10.0611184Z 
2019-07-17T23:28:10.0611383Z ------------------------------------------
2019-07-17T23:28:10.0611432Z stderr:
---
2019-07-17T23:28:10.0623431Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:10.0623488Z +
2019-07-17T23:28:10.0623511Z 
2019-07-17T23:28:10.0623557Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:10.0623601Z Actual stderr saved to /tmp/compiletestNloeDC/non_capture_closure_to_fn_ptr.stderr
2019-07-17T23:28:10.0623657Z To update references, run this command from build directory:
2019-07-17T23:28:10.0623897Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'non_capture_closure_to_fn_ptr.rs'
2019-07-17T23:28:10.0623980Z error: 1 errors occurred comparing output.
2019-07-17T23:28:10.0624017Z status: exit code: 1
2019-07-17T23:28:10.0624017Z status: exit code: 1
2019-07-17T23:28:10.0624780Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/non_capture_closure_to_fn_ptr.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/non_capture_closure_to_fn_ptr.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/non_capture_closure_to_fn_ptr.stage-id.aux" "-A" "unused"
2019-07-17T23:28:10.0625506Z ------------------------------------------
2019-07-17T23:28:10.0625544Z 
2019-07-17T23:28:10.0625793Z ------------------------------------------
2019-07-17T23:28:10.0625842Z stderr:
---
2019-07-17T23:28:10.2021924Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:10.2021984Z +
2019-07-17T23:28:10.2022007Z 
2019-07-17T23:28:10.2022044Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:10.2022103Z Actual stderr saved to /tmp/compiletestNloeDC/option_eq.stderr
2019-07-17T23:28:10.2022144Z To update references, run this command from build directory:
2019-07-17T23:28:10.2022374Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'option_eq.rs'
2019-07-17T23:28:10.2022460Z error: 1 errors occurred comparing output.
2019-07-17T23:28:10.2022498Z status: exit code: 1
2019-07-17T23:28:10.2022498Z status: exit code: 1
2019-07-17T23:28:10.2023035Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/option_eq.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/option_eq.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/option_eq.stage-id.aux" "-A" "unused"
2019-07-17T23:28:10.2023320Z ------------------------------------------
2019-07-17T23:28:10.2023351Z 
2019-07-17T23:28:10.2023564Z ------------------------------------------
2019-07-17T23:28:10.2023603Z stderr:
---
2019-07-17T23:28:10.2033204Z thread '[ui] run-pass/option_eq.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
2019-07-17T23:28:10.2033396Z test [ui] run-pass/option_eq.rs ... FAILED
2019-07-17T23:28:10.2033476Z 
2019-07-17T23:28:10.2033533Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:10.2033575Z Actual stderr saved to /tmp/compiletestNloeDC/option_box_transmute_ptr.stderr
2019-07-17T23:28:10.2033617Z To update references, run this command from build directory:
2019-07-17T23:28:10.2033862Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'option_box_transmute_ptr.rs'
2019-07-17T23:28:10.2033927Z error: 1 errors occurred comparing output.
2019-07-17T23:28:10.2033985Z status: exit code: 1
2019-07-17T23:28:10.2033985Z status: exit code: 1
2019-07-17T23:28:10.2034526Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/option_box_transmute_ptr.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/option_box_transmute_ptr.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/option_box_transmute_ptr.stage-id.aux" "-A" "unused"
2019-07-17T23:28:10.2035343Z ------------------------------------------
2019-07-17T23:28:10.2035379Z 
2019-07-17T23:28:10.2035607Z ------------------------------------------
2019-07-17T23:28:10.2035673Z stderr:
---
2019-07-17T23:28:10.3362897Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:10.3362940Z +
2019-07-17T23:28:10.3362977Z 
2019-07-17T23:28:10.3363016Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:10.3363258Z Actual stderr saved to /tmp/compiletestNloeDC/overloaded-calls-simple.stderr
2019-07-17T23:28:10.3363326Z To update references, run this command from build directory:
2019-07-17T23:28:10.3363570Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'overloaded-calls-simple.rs'
2019-07-17T23:28:10.3363648Z error: 1 errors occurred comparing output.
2019-07-17T23:28:10.3363701Z status: exit code: 1
2019-07-17T23:28:10.3363701Z status: exit code: 1
2019-07-17T23:28:10.3364274Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/overloaded-calls-simple.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/overloaded-calls-simple.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/overloaded-calls-simple.stage-id.aux" "-A" "unused"
2019-07-17T23:28:10.3364996Z ------------------------------------------
2019-07-17T23:28:10.3365041Z 
2019-07-17T23:28:10.3365293Z ------------------------------------------
2019-07-17T23:28:10.3365344Z stderr:
---
2019-07-17T23:28:10.3379233Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:10.3379275Z +
2019-07-17T23:28:10.3379298Z 
2019-07-17T23:28:10.3379335Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:10.3379394Z Actual stderr saved to /tmp/compiletestNloeDC/packed_static.stderr
2019-07-17T23:28:10.3379442Z To update references, run this command from build directory:
2019-07-17T23:28:10.3379668Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'packed_static.rs'
2019-07-17T23:28:10.3379756Z error: 1 errors occurred comparing output.
2019-07-17T23:28:10.3379794Z status: exit code: 1
2019-07-17T23:28:10.3379794Z status: exit code: 1
2019-07-17T23:28:10.3380330Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/packed_static.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/packed_static.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/packed_static.stage-id.aux" "-A" "unused"
2019-07-17T23:28:10.3380590Z ------------------------------------------
2019-07-17T23:28:10.3380634Z 
2019-07-17T23:28:10.3380834Z ------------------------------------------
2019-07-17T23:28:10.3380872Z stderr:
---
2019-07-17T23:28:10.5114637Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:10.5114681Z +
2019-07-17T23:28:10.5114813Z 
2019-07-17T23:28:10.5115596Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:10.5115719Z Actual stderr saved to /tmp/compiletestNloeDC/packed_struct.stderr
2019-07-17T23:28:10.5115899Z To update references, run this command from build directory:
2019-07-17T23:28:10.5116986Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'packed_struct.rs'
2019-07-17T23:28:10.5117259Z error: 1 errors occurred comparing output.
2019-07-17T23:28:10.5117345Z status: exit code: 1
2019-07-17T23:28:10.5117345Z status: exit code: 1
2019-07-17T23:28:10.5118088Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/packed_struct.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/packed_struct.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/packed_struct.stage-id.aux" "-A" "unused"
2019-07-17T23:28:10.5118600Z ------------------------------------------
2019-07-17T23:28:10.5118631Z 
2019-07-17T23:28:10.5118990Z ------------------------------------------
2019-07-17T23:28:10.5119028Z stderr:
---
2019-07-17T23:28:10.5286155Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:10.5286620Z +
2019-07-17T23:28:10.5286780Z 
2019-07-17T23:28:10.5286920Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:10.5287054Z Actual stderr saved to /tmp/compiletestNloeDC/pointers.stderr
2019-07-17T23:28:10.5287338Z To update references, run this command from build directory:
2019-07-17T23:28:10.5287852Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'pointers.rs'
2019-07-17T23:28:10.5288742Z error: 1 errors occurred comparing output.
2019-07-17T23:28:10.5289966Z status: exit code: 1
2019-07-17T23:28:10.5289966Z status: exit code: 1
2019-07-17T23:28:10.5291057Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/pointers.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/pointers.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/pointers.stage-id.aux" "-A" "unused"
2019-07-17T23:28:10.5291665Z ------------------------------------------
2019-07-17T23:28:10.5291980Z 
2019-07-17T23:28:10.5292316Z ------------------------------------------
2019-07-17T23:28:10.5292612Z stderr:
---
2019-07-17T23:28:10.6639818Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:10.6639866Z +
2019-07-17T23:28:10.6639888Z 
2019-07-17T23:28:10.6639941Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:10.6639984Z Actual stderr saved to /tmp/compiletestNloeDC/ptr_arith_offset.stderr
2019-07-17T23:28:10.6640031Z To update references, run this command from build directory:
2019-07-17T23:28:10.6640282Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'ptr_arith_offset.rs'
2019-07-17T23:28:10.6640349Z error: 1 errors occurred comparing output.
2019-07-17T23:28:10.6640386Z status: exit code: 1
2019-07-17T23:28:10.6640386Z status: exit code: 1
2019-07-17T23:28:10.6641329Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/ptr_arith_offset.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/ptr_arith_offset.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/ptr_arith_offset.stage-id.aux" "-A" "unused"
2019-07-17T23:28:10.6641703Z ------------------------------------------
2019-07-17T23:28:10.6641734Z 
2019-07-17T23:28:10.6641923Z ------------------------------------------
2019-07-17T23:28:10.6641981Z stderr:
---
2019-07-17T23:28:10.6760529Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:10.6760590Z +
2019-07-17T23:28:10.6760614Z 
2019-07-17T23:28:10.6760651Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:10.6760701Z Actual stderr saved to /tmp/compiletestNloeDC/products.stderr
2019-07-17T23:28:10.6760763Z To update references, run this command from build directory:
2019-07-17T23:28:10.6760993Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'products.rs'
2019-07-17T23:28:10.6761061Z error: 1 errors occurred comparing output.
2019-07-17T23:28:10.6761117Z status: exit code: 1
2019-07-17T23:28:10.6761117Z status: exit code: 1
2019-07-17T23:28:10.6761640Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/products.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/products.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/products.stage-id.aux" "-A" "unused"
2019-07-17T23:28:10.6762113Z ------------------------------------------
2019-07-17T23:28:10.6762144Z 
2019-07-17T23:28:10.6762383Z ------------------------------------------
2019-07-17T23:28:10.6762423Z stderr:
---
2019-07-17T23:28:10.8032743Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:10.8032789Z +
2019-07-17T23:28:10.8032814Z 
2019-07-17T23:28:10.8032861Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:10.8032928Z Actual stderr saved to /tmp/compiletestNloeDC/ptr_arith_offset_overflow.stderr
2019-07-17T23:28:10.8032976Z To update references, run this command from build directory:
2019-07-17T23:28:10.8033231Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'ptr_arith_offset_overflow.rs'
2019-07-17T23:28:10.8033325Z error: 1 errors occurred comparing output.
2019-07-17T23:28:10.8033367Z status: exit code: 1
2019-07-17T23:28:10.8033367Z status: exit code: 1
2019-07-17T23:28:10.8034188Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/ptr_arith_offset_overflow.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/ptr_arith_offset_overflow.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/ptr_arith_offset_overflow.stage-id.aux" "-A" "unused"
2019-07-17T23:28:10.8034517Z ------------------------------------------
2019-07-17T23:28:10.8034552Z 
2019-07-17T23:28:10.8034766Z ------------------------------------------
2019-07-17T23:28:10.8034810Z stderr:
---
2019-07-17T23:28:10.8566893Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:10.8566964Z +
2019-07-17T23:28:10.8566995Z 
2019-07-17T23:28:10.8567043Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:10.8567114Z Actual stderr saved to /tmp/compiletestNloeDC/ptr_int_casts.stderr
2019-07-17T23:28:10.8567168Z To update references, run this command from build directory:
2019-07-17T23:28:10.8567465Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'ptr_int_casts.rs'
2019-07-17T23:28:10.8567569Z error: 1 errors occurred comparing output.
2019-07-17T23:28:10.8567618Z status: exit code: 1
2019-07-17T23:28:10.8567618Z status: exit code: 1
2019-07-17T23:28:10.8568306Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/ptr_int_casts.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/ptr_int_casts.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/ptr_int_casts.stage-id.aux" "-A" "unused"
2019-07-17T23:28:10.8568830Z ------------------------------------------
2019-07-17T23:28:10.8569036Z 
2019-07-17T23:28:10.8569225Z ------------------------------------------
2019-07-17T23:28:10.8569264Z stderr:
---
2019-07-17T23:28:10.9709947Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:10.9709989Z +
2019-07-17T23:28:10.9710091Z 
2019-07-17T23:28:10.9710148Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:10.9710191Z Actual stderr saved to /tmp/compiletestNloeDC/ptr_int_ops.stderr
2019-07-17T23:28:10.9710234Z To update references, run this command from build directory:
2019-07-17T23:28:10.9710487Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'ptr_int_ops.rs'
2019-07-17T23:28:10.9710556Z error: 1 errors occurred comparing output.
2019-07-17T23:28:10.9710616Z status: exit code: 1
2019-07-17T23:28:10.9710616Z status: exit code: 1
2019-07-17T23:28:10.9711168Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/ptr_int_ops.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/ptr_int_ops.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/ptr_int_ops.stage-id.aux" "-A" "unused"
2019-07-17T23:28:10.9711450Z ------------------------------------------
2019-07-17T23:28:10.9711479Z 
2019-07-17T23:28:10.9711661Z ------------------------------------------
2019-07-17T23:28:10.9711716Z stderr:
---
2019-07-17T23:28:11.0156018Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:11.0156091Z +
2019-07-17T23:28:11.0156121Z 
2019-07-17T23:28:11.0156170Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:11.0156241Z Actual stderr saved to /tmp/compiletestNloeDC/ptr_offset.stderr
2019-07-17T23:28:11.0156295Z To update references, run this command from build directory:
2019-07-17T23:28:11.0156599Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'ptr_offset.rs'
2019-07-17T23:28:11.0156710Z error: 1 errors occurred comparing output.
2019-07-17T23:28:11.0156758Z status: exit code: 1
2019-07-17T23:28:11.0156758Z status: exit code: 1
2019-07-17T23:28:11.0157444Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/ptr_offset.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/ptr_offset.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/ptr_offset.stage-id.aux" "-A" "unused"
2019-07-17T23:28:11.0157804Z ------------------------------------------
2019-07-17T23:28:11.0157859Z 
2019-07-17T23:28:11.0158110Z ------------------------------------------
2019-07-17T23:28:11.0158161Z stderr:
---
2019-07-17T23:28:11.1423996Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:11.1424039Z +
2019-07-17T23:28:11.1424077Z 
2019-07-17T23:28:11.1424116Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:11.1424157Z Actual stderr saved to /tmp/compiletestNloeDC/raw.stderr
2019-07-17T23:28:11.1424216Z To update references, run this command from build directory:
2019-07-17T23:28:11.1424443Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'raw.rs'
2019-07-17T23:28:11.1424519Z error: 1 errors occurred comparing output.
2019-07-17T23:28:11.1424573Z status: exit code: 1
2019-07-17T23:28:11.1424573Z status: exit code: 1
2019-07-17T23:28:11.1425810Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/raw.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/raw.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/raw.stage-id.aux" "-A" "unused"
2019-07-17T23:28:11.1426173Z ------------------------------------------
2019-07-17T23:28:11.1426210Z 
2019-07-17T23:28:11.1426438Z ------------------------------------------
2019-07-17T23:28:11.1426505Z stderr:
---
2019-07-17T23:28:11.2606034Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:11.2606089Z +
2019-07-17T23:28:11.2606118Z 
2019-07-17T23:28:11.2606181Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:11.2606243Z Actual stderr saved to /tmp/compiletestNloeDC/rc.stderr
2019-07-17T23:28:11.2606296Z To update references, run this command from build directory:
2019-07-17T23:28:11.2606599Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'rc.rs'
2019-07-17T23:28:11.2606693Z error: 1 errors occurred comparing output.
2019-07-17T23:28:11.2606757Z status: exit code: 1
2019-07-17T23:28:11.2606757Z status: exit code: 1
2019-07-17T23:28:11.2607412Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/rc.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/rc.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/rc.stage-id.aux" "-A" "unused"
2019-07-17T23:28:11.2607762Z ------------------------------------------
2019-07-17T23:28:11.2607808Z 
2019-07-17T23:28:11.2608054Z ------------------------------------------
2019-07-17T23:28:11.2608120Z stderr:
---
2019-07-17T23:28:11.2846561Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:11.2846618Z +
2019-07-17T23:28:11.2846649Z 
2019-07-17T23:28:11.2846719Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:11.2846774Z Actual stderr saved to /tmp/compiletestNloeDC/recursive_static.stderr
2019-07-17T23:28:11.2846828Z To update references, run this command from build directory:
2019-07-17T23:28:11.2847172Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'recursive_static.rs'
2019-07-17T23:28:11.2847262Z error: 1 errors occurred comparing output.
2019-07-17T23:28:11.2847310Z status: exit code: 1
2019-07-17T23:28:11.2847310Z status: exit code: 1
2019-07-17T23:28:11.2848025Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/recursive_static.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/recursive_static.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/recursive_static.stage-id.aux" "-A" "unused"
2019-07-17T23:28:11.2848385Z ------------------------------------------
2019-07-17T23:28:11.2848424Z 
2019-07-17T23:28:11.2848821Z ------------------------------------------
2019-07-17T23:28:11.2848885Z stderr:
---
2019-07-17T23:28:11.3957609Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:11.3957680Z +
2019-07-17T23:28:11.3957710Z 
2019-07-17T23:28:11.3957759Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:11.3958054Z Actual stderr saved to /tmp/compiletestNloeDC/ref-invalid-ptr.stderr
2019-07-17T23:28:11.3958122Z To update references, run this command from build directory:
2019-07-17T23:28:11.3958410Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'ref-invalid-ptr.rs'
2019-07-17T23:28:11.3958524Z error: 1 errors occurred comparing output.
2019-07-17T23:28:11.3958573Z status: exit code: 1
2019-07-17T23:28:11.3958573Z status: exit code: 1
2019-07-17T23:28:11.3959356Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/ref-invalid-ptr.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/ref-invalid-ptr.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-Zmiri-disable-validation" "-L" "/tmp/compiletestNloeDC/ref-invalid-ptr.stage-id.aux" "-A" "unused"
2019-07-17T23:28:11.3959632Z ------------------------------------------
2019-07-17T23:28:11.3959686Z 
2019-07-17T23:28:11.3959868Z ------------------------------------------
2019-07-17T23:28:11.3959907Z stderr:
---
2019-07-17T23:28:11.4281956Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:11.4282021Z +
2019-07-17T23:28:11.4282054Z 
2019-07-17T23:28:11.4282092Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:11.4282134Z Actual stderr saved to /tmp/compiletestNloeDC/refcell.stderr
2019-07-17T23:28:11.4282372Z To update references, run this command from build directory:
2019-07-17T23:28:11.4282606Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'refcell.rs'
2019-07-17T23:28:11.4282693Z error: 1 errors occurred comparing output.
2019-07-17T23:28:11.4282733Z status: exit code: 1
2019-07-17T23:28:11.4282733Z status: exit code: 1
2019-07-17T23:28:11.4283267Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/refcell.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/refcell.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/refcell.stage-id.aux" "-A" "unused"
2019-07-17T23:28:11.4283553Z ------------------------------------------
2019-07-17T23:28:11.4283584Z 
2019-07-17T23:28:11.4283789Z ------------------------------------------
2019-07-17T23:28:11.4283830Z stderr:
---
2019-07-17T23:28:11.5281269Z +
2019-07-17T23:28:11.5281592Z thread '[ui] run-pass/regions-lifetime-nonfree-late-bound.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
2019-07-17T23:28:11.5281647Z 
2019-07-17T23:28:11.5281695Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:11.5281932Z Actual stderr saved to /tmp/compiletestNloeDC/regions-lifetime-nonfree-late-bound.stderr
2019-07-17T23:28:11.5281998Z To update references, run this command from build directory:
2019-07-17T23:28:11.5282243Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'regions-lifetime-nonfree-late-bound.rs'
2019-07-17T23:28:11.5282317Z error: 1 errors occurred comparing output.
2019-07-17T23:28:11.5282374Z status: exit code: 1
2019-07-17T23:28:11.5282374Z status: exit code: 1
2019-07-17T23:28:11.5283072Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/regions-lifetime-nonfree-late-bound.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/regions-lifetime-nonfree-late-bound.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/regions-lifetime-nonfree-late-bound.stage-id.aux" "-A" "unused"
2019-07-17T23:28:11.5283407Z ------------------------------------------
2019-07-17T23:28:11.5283439Z 
2019-07-17T23:28:11.5283651Z ------------------------------------------
2019-07-17T23:28:11.5283692Z stderr:
---
2019-07-17T23:28:11.5723363Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:11.5723416Z +
2019-07-17T23:28:11.5723453Z 
2019-07-17T23:28:11.5723515Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:11.5723792Z Actual stderr saved to /tmp/compiletestNloeDC/regions-mock-trans.stderr
2019-07-17T23:28:11.5723849Z To update references, run this command from build directory:
2019-07-17T23:28:11.5724138Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'regions-mock-trans.rs'
2019-07-17T23:28:11.5724219Z error: 1 errors occurred comparing output.
2019-07-17T23:28:11.5724280Z status: exit code: 1
2019-07-17T23:28:11.5724280Z status: exit code: 1
2019-07-17T23:28:11.5725438Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/regions-mock-trans.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/regions-mock-trans.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/regions-mock-trans.stage-id.aux" "-A" "unused"
2019-07-17T23:28:11.5725901Z ------------------------------------------
2019-07-17T23:28:11.5725940Z 
2019-07-17T23:28:11.5726165Z ------------------------------------------
2019-07-17T23:28:11.5726231Z stderr:
---
2019-07-17T23:28:11.6677502Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:11.6677554Z +
2019-07-17T23:28:11.6677583Z 
2019-07-17T23:28:11.6677629Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:11.6677697Z Actual stderr saved to /tmp/compiletestNloeDC/rfc1623.stderr
2019-07-17T23:28:11.6677748Z To update references, run this command from build directory:
2019-07-17T23:28:11.6678010Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'rfc1623.rs'
2019-07-17T23:28:11.6678107Z error: 1 errors occurred comparing output.
2019-07-17T23:28:11.6678153Z status: exit code: 1
2019-07-17T23:28:11.6678153Z status: exit code: 1
2019-07-17T23:28:11.6679011Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/rfc1623.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/rfc1623.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/rfc1623.stage-id.aux" "-A" "unused"
2019-07-17T23:28:11.6679304Z ------------------------------------------
2019-07-17T23:28:11.6679348Z 
2019-07-17T23:28:11.6679528Z ------------------------------------------
2019-07-17T23:28:11.6679566Z stderr:
---
2019-07-17T23:28:11.7442664Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:11.7442725Z +
2019-07-17T23:28:11.7442751Z 
2019-07-17T23:28:11.7442793Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:11.7443028Z Actual stderr saved to /tmp/compiletestNloeDC/rust-lang-org.stderr
2019-07-17T23:28:11.7443094Z To update references, run this command from build directory:
2019-07-17T23:28:11.7443332Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'rust-lang-org.rs'
2019-07-17T23:28:11.7443423Z error: 1 errors occurred comparing output.
2019-07-17T23:28:11.7443464Z status: exit code: 1
2019-07-17T23:28:11.7443464Z status: exit code: 1
2019-07-17T23:28:11.7444174Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/rust-lang-org.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/rust-lang-org.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/rust-lang-org.stage-id.aux" "-A" "unused"
2019-07-17T23:28:11.7444528Z ------------------------------------------
2019-07-17T23:28:11.7444560Z 
2019-07-17T23:28:11.7445180Z ------------------------------------------
2019-07-17T23:28:11.7445244Z stderr:
---
2019-07-17T23:28:11.8319576Z +
2019-07-17T23:28:11.8319888Z thread '[ui] run-pass/send-is-not-static-par-for.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
2019-07-17T23:28:11.8319926Z 
2019-07-17T23:28:11.8319965Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:11.8320180Z Actual stderr saved to /tmp/compiletestNloeDC/send-is-not-static-par-for.stderr
2019-07-17T23:28:11.8320253Z To update references, run this command from build directory:
2019-07-17T23:28:11.8320477Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'send-is-not-static-par-for.rs'
2019-07-17T23:28:11.8320562Z error: 1 errors occurred comparing output.
2019-07-17T23:28:11.8320679Z status: exit code: 1
2019-07-17T23:28:11.8320679Z status: exit code: 1
2019-07-17T23:28:11.8321280Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/send-is-not-static-par-for.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/send-is-not-static-par-for.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/send-is-not-static-par-for.stage-id.aux" "-A" "unused"
2019-07-17T23:28:11.8321563Z ------------------------------------------
2019-07-17T23:28:11.8321592Z 
2019-07-17T23:28:11.8322078Z ------------------------------------------
2019-07-17T23:28:11.8322120Z stderr:
---
2019-07-17T23:28:11.9076433Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:11.9076487Z +
2019-07-17T23:28:11.9076536Z 
2019-07-17T23:28:11.9076584Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:11.9076847Z Actual stderr saved to /tmp/compiletestNloeDC/sendable-class.stderr
2019-07-17T23:28:11.9076924Z To update references, run this command from build directory:
2019-07-17T23:28:11.9077202Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'sendable-class.rs'
2019-07-17T23:28:11.9077286Z error: 1 errors occurred comparing output.
2019-07-17T23:28:11.9077350Z status: exit code: 1
2019-07-17T23:28:11.9077350Z status: exit code: 1
2019-07-17T23:28:11.9078171Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/sendable-class.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/sendable-class.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/sendable-class.stage-id.aux" "-A" "unused"
2019-07-17T23:28:11.9078945Z ------------------------------------------
2019-07-17T23:28:11.9078977Z 
2019-07-17T23:28:11.9079154Z ------------------------------------------
2019-07-17T23:28:11.9079308Z stderr:
---
2019-07-17T23:28:11.9957834Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:11.9957903Z +
2019-07-17T23:28:11.9957930Z 
2019-07-17T23:28:11.9957975Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:11.9958240Z Actual stderr saved to /tmp/compiletestNloeDC/simd-intrinsic-generic-elements.stderr
2019-07-17T23:28:11.9958326Z To update references, run this command from build directory:
2019-07-17T23:28:11.9958722Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'simd-intrinsic-generic-elements.rs'
2019-07-17T23:28:11.9958885Z error: 1 errors occurred comparing output.
2019-07-17T23:28:11.9958921Z status: exit code: 1
2019-07-17T23:28:11.9958921Z status: exit code: 1
2019-07-17T23:28:11.9959487Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/simd-intrinsic-generic-elements.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/simd-intrinsic-generic-elements.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/simd-intrinsic-generic-elements.stage-id.aux" "-A" "unused"
2019-07-17T23:28:11.9959847Z ------------------------------------------
2019-07-17T23:28:11.9959877Z 
2019-07-17T23:28:11.9960070Z ------------------------------------------
2019-07-17T23:28:11.9960107Z stderr:
---
2019-07-17T23:28:12.1119051Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:12.1119209Z +
2019-07-17T23:28:12.1119307Z 
2019-07-17T23:28:12.1119364Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:12.1119425Z Actual stderr saved to /tmp/compiletestNloeDC/small_enum_size_bug.stderr
2019-07-17T23:28:12.1119594Z To update references, run this command from build directory:
2019-07-17T23:28:12.1119882Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'small_enum_size_bug.rs'
2019-07-17T23:28:12.1120104Z error: 1 errors occurred comparing output.
2019-07-17T23:28:12.1120167Z status: exit code: 1
2019-07-17T23:28:12.1120167Z status: exit code: 1
2019-07-17T23:28:12.1121115Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/small_enum_size_bug.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/small_enum_size_bug.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/small_enum_size_bug.stage-id.aux" "-A" "unused"
2019-07-17T23:28:12.1121704Z ------------------------------------------
2019-07-17T23:28:12.1121735Z 
2019-07-17T23:28:12.1122074Z ------------------------------------------
2019-07-17T23:28:12.1122249Z stderr:
---
2019-07-17T23:28:12.2316369Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:12.2316423Z +
2019-07-17T23:28:12.2316452Z 
2019-07-17T23:28:12.2316711Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:12.2316797Z Actual stderr saved to /tmp/compiletestNloeDC/slices.stderr
2019-07-17T23:28:12.2316852Z To update references, run this command from build directory:
2019-07-17T23:28:12.2317176Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'slices.rs'
2019-07-17T23:28:12.2317279Z error: 1 errors occurred comparing output.
2019-07-17T23:28:12.2317328Z status: exit code: 1
2019-07-17T23:28:12.2317328Z status: exit code: 1
2019-07-17T23:28:12.2318171Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/slices.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/slices.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/slices.stage-id.aux" "-A" "unused"
2019-07-17T23:28:12.2318589Z ------------------------------------------
2019-07-17T23:28:12.2318624Z 
2019-07-17T23:28:12.2318825Z ------------------------------------------
2019-07-17T23:28:12.2318867Z stderr:
---
2019-07-17T23:28:12.2650653Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:12.2651433Z +
2019-07-17T23:28:12.2652133Z 
2019-07-17T23:28:12.2653166Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:12.2653829Z Actual stderr saved to /tmp/compiletestNloeDC/specialization.stderr
2019-07-17T23:28:12.2654513Z To update references, run this command from build directory:
2019-07-17T23:28:12.2655983Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'specialization.rs'
2019-07-17T23:28:12.2657796Z error: 1 errors occurred comparing output.
2019-07-17T23:28:12.2659079Z status: exit code: 1
2019-07-17T23:28:12.2659079Z status: exit code: 1
2019-07-17T23:28:12.2660712Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/specialization.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/specialization.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/specialization.stage-id.aux" "-A" "unused"
2019-07-17T23:28:12.2662424Z ------------------------------------------
2019-07-17T23:28:12.2663150Z 
2019-07-17T23:28:12.2664014Z ------------------------------------------
2019-07-17T23:28:12.2664776Z stderr:
---
2019-07-17T23:28:12.3799789Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:12.3799841Z +
2019-07-17T23:28:12.3799871Z 
2019-07-17T23:28:12.3799939Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:12.3800222Z Actual stderr saved to /tmp/compiletestNloeDC/stacked-borrows/2phase.stderr
2019-07-17T23:28:12.3800282Z To update references, run this command from build directory:
2019-07-17T23:28:12.3800594Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'stacked-borrows/2phase.rs'
2019-07-17T23:28:12.3800767Z error: 1 errors occurred comparing output.
2019-07-17T23:28:12.3800817Z status: exit code: 1
2019-07-17T23:28:12.3800817Z status: exit code: 1
2019-07-17T23:28:12.3801965Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/stacked-borrows/2phase.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/stacked-borrows/2phase.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/stacked-borrows/2phase.stage-id.aux" "-A" "unused"
2019-07-17T23:28:12.3802583Z ------------------------------------------
2019-07-17T23:28:12.3802613Z 
2019-07-17T23:28:12.3802798Z ------------------------------------------
2019-07-17T23:28:12.3802855Z stderr:
---
2019-07-17T23:28:12.4449099Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:12.4449164Z +
2019-07-17T23:28:12.4449188Z 
2019-07-17T23:28:12.4449227Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:12.4449476Z Actual stderr saved to /tmp/compiletestNloeDC/stacked-borrows/interior_mutability.stderr
2019-07-17T23:28:12.4449524Z To update references, run this command from build directory:
2019-07-17T23:28:12.4449872Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'stacked-borrows/interior_mutability.rs'
2019-07-17T23:28:12.4449961Z error: 1 errors occurred comparing output.
2019-07-17T23:28:12.4450000Z status: exit code: 1
2019-07-17T23:28:12.4450000Z status: exit code: 1
2019-07-17T23:28:12.4450769Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/stacked-borrows/interior_mutability.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/stacked-borrows/interior_mutability.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/stacked-borrows/interior_mutability.stage-id.aux" "-A" "unused"
2019-07-17T23:28:12.4451061Z ------------------------------------------
2019-07-17T23:28:12.4451108Z 
2019-07-17T23:28:12.4451305Z ------------------------------------------
2019-07-17T23:28:12.4451347Z stderr:
---
2019-07-17T23:28:12.6011857Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:12.6011921Z +
2019-07-17T23:28:12.6011947Z 
2019-07-17T23:28:12.6012318Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:12.6012411Z Actual stderr saved to /tmp/compiletestNloeDC/static_memory_modification.stderr
2019-07-17T23:28:12.6012457Z To update references, run this command from build directory:
2019-07-17T23:28:12.6016429Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'static_memory_modification.rs'
2019-07-17T23:28:12.6016565Z error: 1 errors occurred comparing output.
2019-07-17T23:28:12.6016625Z status: exit code: 1
2019-07-17T23:28:12.6016625Z status: exit code: 1
2019-07-17T23:28:12.6017989Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/static_memory_modification.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/static_memory_modification.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/static_memory_modification.stage-id.aux" "-A" "unused"
2019-07-17T23:28:12.6018783Z ------------------------------------------
2019-07-17T23:28:12.6018834Z 
2019-07-17T23:28:12.6019037Z ------------------------------------------
2019-07-17T23:28:12.6019077Z stderr:
---
2019-07-17T23:28:12.6122174Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:12.6122234Z +
2019-07-17T23:28:12.6122259Z 
2019-07-17T23:28:12.6122299Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:12.6122636Z Actual stderr saved to /tmp/compiletestNloeDC/stacked-borrows/stacked-borrows.stderr
2019-07-17T23:28:12.6122704Z To update references, run this command from build directory:
2019-07-17T23:28:12.6122960Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'stacked-borrows/stacked-borrows.rs'
2019-07-17T23:28:12.6123048Z error: 1 errors occurred comparing output.
2019-07-17T23:28:12.6123087Z status: exit code: 1
2019-07-17T23:28:12.6123087Z status: exit code: 1
2019-07-17T23:28:12.6123675Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/stacked-borrows/stacked-borrows.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/stacked-borrows/stacked-borrows.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/stacked-borrows/stacked-borrows.stage-id.aux" "-A" "unused"
2019-07-17T23:28:12.6123977Z ------------------------------------------
2019-07-17T23:28:12.6124009Z 
2019-07-17T23:28:12.6124226Z ------------------------------------------
2019-07-17T23:28:12.6124273Z stderr:
---
2019-07-17T23:28:12.7391256Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:12.7391301Z +
2019-07-17T23:28:12.7391324Z 
2019-07-17T23:28:12.7391534Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:12.7391594Z Actual stderr saved to /tmp/compiletestNloeDC/static_mut.stderr
2019-07-17T23:28:12.7391865Z To update references, run this command from build directory:
2019-07-17T23:28:12.7392261Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'static_mut.rs'
2019-07-17T23:28:12.7392353Z error: 1 errors occurred comparing output.
2019-07-17T23:28:12.7392394Z status: exit code: 1
2019-07-17T23:28:12.7392394Z status: exit code: 1
2019-07-17T23:28:12.7393185Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/static_mut.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/static_mut.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/static_mut.stage-id.aux" "-A" "unused"
2019-07-17T23:28:12.7393495Z ------------------------------------------
2019-07-17T23:28:12.7393546Z 
2019-07-17T23:28:12.7393747Z ------------------------------------------
2019-07-17T23:28:12.7393792Z stderr:
---
2019-07-17T23:28:12.7792662Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:12.7792905Z +
2019-07-17T23:28:12.7792946Z 
2019-07-17T23:28:12.7792987Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:12.7793092Z Actual stderr saved to /tmp/compiletestNloeDC/strings.stderr
2019-07-17T23:28:12.7793135Z To update references, run this command from build directory:
2019-07-17T23:28:12.7793518Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'strings.rs'
2019-07-17T23:28:12.7793609Z error: 1 errors occurred comparing output.
2019-07-17T23:28:12.7793646Z status: exit code: 1
2019-07-17T23:28:12.7793646Z status: exit code: 1
2019-07-17T23:28:12.7794192Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/strings.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/strings.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/strings.stage-id.aux" "-A" "unused"
2019-07-17T23:28:12.7794457Z ------------------------------------------
2019-07-17T23:28:12.7794501Z 
2019-07-17T23:28:12.7794680Z ------------------------------------------
2019-07-17T23:28:12.7794719Z stderr:
---
2019-07-17T23:28:12.8918192Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:12.8918248Z +
2019-07-17T23:28:12.8918292Z 
2019-07-17T23:28:12.8918338Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:12.8918390Z Actual stderr saved to /tmp/compiletestNloeDC/subslice_array.stderr
2019-07-17T23:28:12.8918460Z To update references, run this command from build directory:
2019-07-17T23:28:12.8918730Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'subslice_array.rs'
2019-07-17T23:28:12.8918813Z error: 1 errors occurred comparing output.
2019-07-17T23:28:12.8918885Z status: exit code: 1
2019-07-17T23:28:12.8918885Z status: exit code: 1
2019-07-17T23:28:12.8919543Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/subslice_array.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/subslice_array.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/subslice_array.stage-id.aux" "-A" "unused"
2019-07-17T23:28:12.8919882Z ------------------------------------------
2019-07-17T23:28:12.8919917Z 
2019-07-17T23:28:12.8920157Z ------------------------------------------
2019-07-17T23:28:12.8920207Z stderr:
---
2019-07-17T23:28:12.9679312Z +
2019-07-17T23:28:12.9679614Z thread '[ui] run-pass/sums.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
2019-07-17T23:28:12.9679652Z 
2019-07-17T23:28:12.9679708Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:12.9679914Z Actual stderr saved to /tmp/compiletestNloeDC/sums.stderr
2019-07-17T23:28:12.9679956Z To update references, run this command from build directory:
2019-07-17T23:28:12.9680344Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'sums.rs'
2019-07-17T23:28:12.9680416Z error: 1 errors occurred comparing output.
2019-07-17T23:28:12.9680453Z status: exit code: 1
2019-07-17T23:28:12.9680453Z status: exit code: 1
2019-07-17T23:28:12.9680971Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/sums.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/sums.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/sums.stage-id.aux" "-A" "unused"
2019-07-17T23:28:12.9681245Z ------------------------------------------
2019-07-17T23:28:12.9681275Z 
2019-07-17T23:28:12.9681455Z ------------------------------------------
2019-07-17T23:28:12.9681510Z stderr:
---
2019-07-17T23:28:13.0199096Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:13.0199153Z +
2019-07-17T23:28:13.0199175Z 
2019-07-17T23:28:13.0199210Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:13.0199268Z Actual stderr saved to /tmp/compiletestNloeDC/sync.stderr
2019-07-17T23:28:13.0199308Z To update references, run this command from build directory:
2019-07-17T23:28:13.0199510Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'sync.rs'
2019-07-17T23:28:13.0199597Z error: 1 errors occurred comparing output.
2019-07-17T23:28:13.0199742Z status: exit code: 1
2019-07-17T23:28:13.0199742Z status: exit code: 1
2019-07-17T23:28:13.0200240Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/sync.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/sync.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/sync.stage-id.aux" "-A" "unused"
2019-07-17T23:28:13.0200499Z ------------------------------------------
2019-07-17T23:28:13.0200541Z 
2019-07-17T23:28:13.0200713Z ------------------------------------------
2019-07-17T23:28:13.0200750Z stderr:
---
2019-07-17T23:28:13.1077022Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:13.1077095Z +
2019-07-17T23:28:13.1077127Z 
2019-07-17T23:28:13.1077174Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:13.1077442Z Actual stderr saved to /tmp/compiletestNloeDC/tag-align-dyn-u64.stderr
2019-07-17T23:28:13.1077516Z To update references, run this command from build directory:
2019-07-17T23:28:13.1077786Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'tag-align-dyn-u64.rs'
2019-07-17T23:28:13.1077872Z error: 1 errors occurred comparing output.
2019-07-17T23:28:13.1077934Z status: exit code: 1
2019-07-17T23:28:13.1077934Z status: exit code: 1
2019-07-17T23:28:13.1078760Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/tag-align-dyn-u64.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/tag-align-dyn-u64.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/tag-align-dyn-u64.stage-id.aux" "-A" "unused"
2019-07-17T23:28:13.1079047Z ------------------------------------------
2019-07-17T23:28:13.1079078Z 
2019-07-17T23:28:13.1079285Z ------------------------------------------
2019-07-17T23:28:13.1079483Z stderr:
---
2019-07-17T23:28:13.1961983Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:13.1962025Z +
2019-07-17T23:28:13.1962048Z 
2019-07-17T23:28:13.1962086Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:13.1962321Z Actual stderr saved to /tmp/compiletestNloeDC/thread-local.stderr
2019-07-17T23:28:13.1962368Z To update references, run this command from build directory:
2019-07-17T23:28:13.1962588Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'thread-local.rs'
2019-07-17T23:28:13.1962682Z error: 1 errors occurred comparing output.
2019-07-17T23:28:13.1962719Z status: exit code: 1
2019-07-17T23:28:13.1962719Z status: exit code: 1
2019-07-17T23:28:13.1963271Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/thread-local.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/thread-local.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/thread-local.stage-id.aux" "-A" "unused"
2019-07-17T23:28:13.1963552Z ------------------------------------------
2019-07-17T23:28:13.1963582Z 
2019-07-17T23:28:13.1963777Z ------------------------------------------
2019-07-17T23:28:13.1963817Z stderr:
---
2019-07-17T23:28:13.2404462Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:13.2404505Z +
2019-07-17T23:28:13.2404529Z 
2019-07-17T23:28:13.2404585Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:13.2405314Z Actual stderr saved to /tmp/compiletestNloeDC/too-large-primval-write-problem.stderr
2019-07-17T23:28:13.2405390Z To update references, run this command from build directory:
2019-07-17T23:28:13.2405753Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'too-large-primval-write-problem.rs'
2019-07-17T23:28:13.2405843Z error: 1 errors occurred comparing output.
2019-07-17T23:28:13.2405890Z status: exit code: 1
2019-07-17T23:28:13.2405890Z status: exit code: 1
2019-07-17T23:28:13.2406619Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/too-large-primval-write-problem.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/too-large-primval-write-problem.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/too-large-primval-write-problem.stage-id.aux" "-A" "unused"
2019-07-17T23:28:13.2406965Z ------------------------------------------
2019-07-17T23:28:13.2407002Z 
2019-07-17T23:28:13.2407225Z ------------------------------------------
2019-07-17T23:28:13.2407291Z stderr:
---
2019-07-17T23:28:13.3671253Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:13.3671309Z +
2019-07-17T23:28:13.3671331Z 
2019-07-17T23:28:13.3671368Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:13.3671417Z Actual stderr saved to /tmp/compiletestNloeDC/traits.stderr
2019-07-17T23:28:13.3671474Z To update references, run this command from build directory:
2019-07-17T23:28:13.3671678Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'traits.rs'
2019-07-17T23:28:13.3671763Z error: 1 errors occurred comparing output.
2019-07-17T23:28:13.3671801Z status: exit code: 1
2019-07-17T23:28:13.3671801Z status: exit code: 1
2019-07-17T23:28:13.3672308Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/traits.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/traits.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/traits.stage-id.aux" "-A" "unused"
2019-07-17T23:28:13.3672571Z ------------------------------------------
2019-07-17T23:28:13.3672606Z 
2019-07-17T23:28:13.3672800Z ------------------------------------------
2019-07-17T23:28:13.3672839Z stderr:
---
2019-07-17T23:28:13.3811091Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:13.3811139Z +
2019-07-17T23:28:13.3811181Z 
2019-07-17T23:28:13.3811220Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:13.3811272Z Actual stderr saved to /tmp/compiletestNloeDC/transmute_fat.stderr
2019-07-17T23:28:13.3811330Z To update references, run this command from build directory:
2019-07-17T23:28:13.3811583Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'transmute_fat.rs'
2019-07-17T23:28:13.3811654Z error: 1 errors occurred comparing output.
2019-07-17T23:28:13.3811710Z status: exit code: 1
2019-07-17T23:28:13.3811710Z status: exit code: 1
2019-07-17T23:28:13.3812261Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/transmute_fat.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/transmute_fat.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-Zmiri-disable-validation" "-L" "/tmp/compiletestNloeDC/transmute_fat.stage-id.aux" "-A" "unused"
2019-07-17T23:28:13.3812556Z ------------------------------------------
2019-07-17T23:28:13.3812586Z 
2019-07-17T23:28:13.3812962Z ------------------------------------------
2019-07-17T23:28:13.3813001Z stderr:
---
2019-07-17T23:28:13.4876887Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:13.4876949Z +
2019-07-17T23:28:13.4876979Z 
2019-07-17T23:28:13.4877027Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:13.4877100Z Actual stderr saved to /tmp/compiletestNloeDC/trivial.stderr
2019-07-17T23:28:13.4877159Z To update references, run this command from build directory:
2019-07-17T23:28:13.4877442Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'trivial.rs'
2019-07-17T23:28:13.4877546Z error: 1 errors occurred comparing output.
2019-07-17T23:28:13.4877594Z status: exit code: 1
2019-07-17T23:28:13.4877594Z status: exit code: 1
2019-07-17T23:28:13.4878259Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/trivial.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/trivial.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/trivial.stage-id.aux" "-A" "unused"
2019-07-17T23:28:13.4878621Z ------------------------------------------
2019-07-17T23:28:13.4878659Z 
2019-07-17T23:28:13.4879026Z ------------------------------------------
2019-07-17T23:28:13.4879069Z stderr:
---
2019-07-17T23:28:13.5123886Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:13.5123931Z +
2019-07-17T23:28:13.5123955Z 
2019-07-17T23:28:13.5123994Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:13.5124245Z Actual stderr saved to /tmp/compiletestNloeDC/try-operator-custom.stderr
2019-07-17T23:28:13.5124292Z To update references, run this command from build directory:
2019-07-17T23:28:13.5124515Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'try-operator-custom.rs'
2019-07-17T23:28:13.5124598Z error: 1 errors occurred comparing output.
2019-07-17T23:28:13.5124634Z status: exit code: 1
2019-07-17T23:28:13.5124634Z status: exit code: 1
2019-07-17T23:28:13.5125592Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/try-operator-custom.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/try-operator-custom.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/try-operator-custom.stage-id.aux" "-A" "unused"
2019-07-17T23:28:13.5125975Z ------------------------------------------
2019-07-17T23:28:13.5126041Z 
2019-07-17T23:28:13.5126269Z ------------------------------------------
2019-07-17T23:28:13.5126319Z stderr:
---
2019-07-17T23:28:13.6396384Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:13.6396454Z +
2019-07-17T23:28:13.6396482Z 
2019-07-17T23:28:13.6396536Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:13.6396592Z Actual stderr saved to /tmp/compiletestNloeDC/tuple_like_enum_variant_constructor.stderr
2019-07-17T23:28:13.6396660Z To update references, run this command from build directory:
2019-07-17T23:28:13.6396947Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'tuple_like_enum_variant_constructor.rs'
2019-07-17T23:28:13.6397048Z error: 1 errors occurred comparing output.
2019-07-17T23:28:13.6397094Z status: exit code: 1
2019-07-17T23:28:13.6397094Z status: exit code: 1
2019-07-17T23:28:13.6397819Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/tuple_like_enum_variant_constructor.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/tuple_like_enum_variant_constructor.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/tuple_like_enum_variant_constructor.stage-id.aux" "-A" "unused"
2019-07-17T23:28:13.6398310Z ------------------------------------------
2019-07-17T23:28:13.6398339Z 
2019-07-17T23:28:13.6398543Z ------------------------------------------
2019-07-17T23:28:13.6398581Z stderr:
---
2019-07-17T23:28:13.6569761Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:13.6569803Z +
2019-07-17T23:28:13.6569826Z 
2019-07-17T23:28:13.6569878Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:13.6569923Z Actual stderr saved to /tmp/compiletestNloeDC/tuple_like_enum_variant_constructor_pointer_opt.stderr
2019-07-17T23:28:13.6569966Z To update references, run this command from build directory:
2019-07-17T23:28:13.6570232Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'tuple_like_enum_variant_constructor_pointer_opt.rs'
2019-07-17T23:28:13.6570307Z error: 1 errors occurred comparing output.
2019-07-17T23:28:13.6570345Z status: exit code: 1
2019-07-17T23:28:13.6570345Z status: exit code: 1
2019-07-17T23:28:13.6570946Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/tuple_like_enum_variant_constructor_pointer_opt.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/tuple_like_enum_variant_constructor_pointer_opt.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/tuple_like_enum_variant_constructor_pointer_opt.stage-id.aux" "-A" "unused"
2019-07-17T23:28:13.6571225Z ------------------------------------------
2019-07-17T23:28:13.6571253Z 
2019-07-17T23:28:13.6571885Z ------------------------------------------
2019-07-17T23:28:13.6571956Z stderr:
---
2019-07-17T23:28:13.8078608Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:13.8078837Z +
2019-07-17T23:28:13.8078875Z 
2019-07-17T23:28:13.8078911Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:13.8078953Z Actual stderr saved to /tmp/compiletestNloeDC/tuple_like_struct_constructor.stderr
2019-07-17T23:28:13.8079000Z To update references, run this command from build directory:
2019-07-17T23:28:13.8079239Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'tuple_like_struct_constructor.rs'
2019-07-17T23:28:13.8079310Z error: 1 errors occurred comparing output.
2019-07-17T23:28:13.8079363Z status: exit code: 1
2019-07-17T23:28:13.8079363Z status: exit code: 1
2019-07-17T23:28:13.8079900Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/tuple_like_struct_constructor.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/tuple_like_struct_constructor.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/tuple_like_struct_constructor.stage-id.aux" "-A" "unused"
2019-07-17T23:28:13.8080169Z ------------------------------------------
2019-07-17T23:28:13.8080197Z 
2019-07-17T23:28:13.8080368Z ------------------------------------------
2019-07-17T23:28:13.8080423Z stderr:
---
2019-07-17T23:28:13.8476264Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:13.8476345Z +
2019-07-17T23:28:13.8476375Z 
2019-07-17T23:28:13.8476422Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:13.8476480Z Actual stderr saved to /tmp/compiletestNloeDC/tuple_like_enum_variant_constructor_struct_pointer_opt.stderr
2019-07-17T23:28:13.8476563Z To update references, run this command from build directory:
2019-07-17T23:28:13.8476890Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'tuple_like_enum_variant_constructor_struct_pointer_opt.rs'
2019-07-17T23:28:13.8476993Z error: 1 errors occurred comparing output.
2019-07-17T23:28:13.8477042Z status: exit code: 1
2019-07-17T23:28:13.8477042Z status: exit code: 1
2019-07-17T23:28:13.8477809Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/tuple_like_enum_variant_constructor_struct_pointer_opt.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/tuple_like_enum_variant_constructor_struct_pointer_opt.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/tuple_like_enum_variant_constructor_struct_pointer_opt.stage-id.aux" "-A" "unused"
2019-07-17T23:28:13.8478360Z ------------------------------------------
2019-07-17T23:28:13.8478398Z 
2019-07-17T23:28:13.8479008Z ------------------------------------------
2019-07-17T23:28:13.8479046Z stderr:
---
2019-07-17T23:28:14.0003203Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:14.0003246Z +
2019-07-17T23:28:14.0003286Z 
2019-07-17T23:28:14.0003326Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:14.0003548Z Actual stderr saved to /tmp/compiletestNloeDC/union-overwrite.stderr
2019-07-17T23:28:14.0003612Z To update references, run this command from build directory:
2019-07-17T23:28:14.0003842Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'union-overwrite.rs'
2019-07-17T23:28:14.0003920Z error: 1 errors occurred comparing output.
2019-07-17T23:28:14.0003974Z status: exit code: 1
2019-07-17T23:28:14.0003974Z status: exit code: 1
2019-07-17T23:28:14.0004657Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/union-overwrite.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/union-overwrite.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/union-overwrite.stage-id.aux" "-A" "unused"
2019-07-17T23:28:14.0004995Z ------------------------------------------
2019-07-17T23:28:14.0005027Z 
2019-07-17T23:28:14.0005669Z ------------------------------------------
2019-07-17T23:28:14.0005729Z stderr:
---
2019-07-17T23:28:14.1609620Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:14.1609669Z +
2019-07-17T23:28:14.1609740Z 
2019-07-17T23:28:14.1609798Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:14.1609842Z Actual stderr saved to /tmp/compiletestNloeDC/union.stderr
2019-07-17T23:28:14.1609886Z To update references, run this command from build directory:
2019-07-17T23:28:14.1610120Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'union.rs'
2019-07-17T23:28:14.1610382Z error: 1 errors occurred comparing output.
2019-07-17T23:28:14.1610430Z status: exit code: 1
2019-07-17T23:28:14.1610430Z status: exit code: 1
2019-07-17T23:28:14.1611648Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/union.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/union.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/union.stage-id.aux" "-A" "unused"
2019-07-17T23:28:14.1612007Z ------------------------------------------
2019-07-17T23:28:14.1612042Z 
2019-07-17T23:28:14.1612251Z ------------------------------------------
2019-07-17T23:28:14.1612297Z stderr:
---
2019-07-17T23:28:14.1713971Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:14.1714017Z +
2019-07-17T23:28:14.1714042Z 
2019-07-17T23:28:14.1714082Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:14.1714145Z Actual stderr saved to /tmp/compiletestNloeDC/u128.stderr
2019-07-17T23:28:14.1714189Z To update references, run this command from build directory:
2019-07-17T23:28:14.1714429Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'u128.rs'
2019-07-17T23:28:14.1714522Z error: 1 errors occurred comparing output.
2019-07-17T23:28:14.1714565Z status: exit code: 1
2019-07-17T23:28:14.1714565Z status: exit code: 1
2019-07-17T23:28:14.1715654Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/u128.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/u128.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/u128.stage-id.aux" "-A" "unused"
2019-07-17T23:28:14.1716060Z ------------------------------------------
2019-07-17T23:28:14.1716117Z 
2019-07-17T23:28:14.1716342Z ------------------------------------------
2019-07-17T23:28:14.1716391Z stderr:
---
2019-07-17T23:28:14.3200911Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:14.3201094Z +
2019-07-17T23:28:14.3201186Z 
2019-07-17T23:28:14.3201292Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:14.3201424Z Actual stderr saved to /tmp/compiletestNloeDC/unops.stderr
2019-07-17T23:28:14.3201536Z To update references, run this command from build directory:
2019-07-17T23:28:14.3201873Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'unops.rs'
2019-07-17T23:28:14.3202145Z error: 1 errors occurred comparing output.
2019-07-17T23:28:14.3202252Z status: exit code: 1
2019-07-17T23:28:14.3202252Z status: exit code: 1
2019-07-17T23:28:14.3203054Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/unops.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/unops.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/unops.stage-id.aux" "-A" "unused"
2019-07-17T23:28:14.3203619Z ------------------------------------------
2019-07-17T23:28:14.3203753Z 
2019-07-17T23:28:14.3204049Z ------------------------------------------
2019-07-17T23:28:14.3204209Z stderr:
---
2019-07-17T23:28:14.3346403Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:14.3346458Z +
2019-07-17T23:28:14.3346489Z 
2019-07-17T23:28:14.3346556Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:14.3346850Z Actual stderr saved to /tmp/compiletestNloeDC/unsized-tuple-impls.stderr
2019-07-17T23:28:14.3346910Z To update references, run this command from build directory:
2019-07-17T23:28:14.3347203Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'unsized-tuple-impls.rs'
2019-07-17T23:28:14.3347318Z error: 1 errors occurred comparing output.
2019-07-17T23:28:14.3347366Z status: exit code: 1
2019-07-17T23:28:14.3347366Z status: exit code: 1
2019-07-17T23:28:14.3348228Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/unsized-tuple-impls.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/unsized-tuple-impls.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/unsized-tuple-impls.stage-id.aux" "-A" "unused"
2019-07-17T23:28:14.3348767Z ------------------------------------------
2019-07-17T23:28:14.3348798Z 
2019-07-17T23:28:14.3348987Z ------------------------------------------
2019-07-17T23:28:14.3349028Z stderr:
---
2019-07-17T23:28:14.4356923Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:14.4356994Z +
2019-07-17T23:28:14.4357022Z 
2019-07-17T23:28:14.4357070Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:14.4357144Z Actual stderr saved to /tmp/compiletestNloeDC/validation_lifetime_resolution.stderr
2019-07-17T23:28:14.4357209Z To update references, run this command from build directory:
2019-07-17T23:28:14.4357523Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'validation_lifetime_resolution.rs'
2019-07-17T23:28:14.4357628Z error: 1 errors occurred comparing output.
2019-07-17T23:28:14.4357787Z status: exit code: 1
2019-07-17T23:28:14.4357787Z status: exit code: 1
2019-07-17T23:28:14.4358547Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/validation_lifetime_resolution.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/validation_lifetime_resolution.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/validation_lifetime_resolution.stage-id.aux" "-A" "unused"
2019-07-17T23:28:14.4359012Z ------------------------------------------
2019-07-17T23:28:14.4359137Z 
2019-07-17T23:28:14.4359346Z ------------------------------------------
2019-07-17T23:28:14.4359387Z stderr:
---
2019-07-17T23:28:14.5201125Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:14.5201171Z +
2019-07-17T23:28:14.5201202Z 
2019-07-17T23:28:14.5201240Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:14.5201491Z Actual stderr saved to /tmp/compiletestNloeDC/vec-matching-fold.stderr
2019-07-17T23:28:14.5201536Z To update references, run this command from build directory:
2019-07-17T23:28:14.5201887Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'vec-matching-fold.rs'
2019-07-17T23:28:14.5201986Z error: 1 errors occurred comparing output.
2019-07-17T23:28:14.5202024Z status: exit code: 1
2019-07-17T23:28:14.5202024Z status: exit code: 1
2019-07-17T23:28:14.5202601Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/vec-matching-fold.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/vec-matching-fold.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/vec-matching-fold.stage-id.aux" "-A" "unused"
2019-07-17T23:28:14.5202974Z ------------------------------------------
2019-07-17T23:28:14.5203022Z 
2019-07-17T23:28:14.5203214Z ------------------------------------------
2019-07-17T23:28:14.5203256Z stderr:
---
2019-07-17T23:28:14.6156118Z -Iter([], [])
2019-07-17T23:28:14.6156309Z -
2019-07-17T23:28:14.6156341Z 
2019-07-17T23:28:14.6156390Z The actual stdout differed from the expected stdout.
2019-07-17T23:28:14.6156460Z Actual stdout saved to /tmp/compiletestNloeDC/vecdeque.stdout
2019-07-17T23:28:14.6156570Z error[E0080]: Miri evaluation error: attempted to do invalid arithmetic on pointers that would leak base addresses, e.g., comparing pointers into different allocations
2019-07-17T23:28:14.6156872Z     --> /checkout/src/libcore/intrinsics.rs:1340:19
2019-07-17T23:28:14.6156927Z      |
2019-07-17T23:28:14.6156978Z 1340 |     let diff = if src_usize > dst_usize {
---
2019-07-17T23:28:14.6161024Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:14.6161066Z +
2019-07-17T23:28:14.6161104Z 
2019-07-17T23:28:14.6161141Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:14.6161183Z Actual stderr saved to /tmp/compiletestNloeDC/vecdeque.stderr
2019-07-17T23:28:14.6161315Z To update references, run this command from build directory:
2019-07-17T23:28:14.6161555Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'vecdeque.rs'
2019-07-17T23:28:14.6161621Z error: 2 errors occurred comparing output.
2019-07-17T23:28:14.6161685Z status: exit code: 1
2019-07-17T23:28:14.6161685Z status: exit code: 1
2019-07-17T23:28:14.6162214Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/vecdeque.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/vecdeque.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/vecdeque.stage-id.aux" "-A" "unused"
2019-07-17T23:28:14.6162487Z ------------------------------------------
2019-07-17T23:28:14.6162517Z 
2019-07-17T23:28:14.6162715Z ------------------------------------------
2019-07-17T23:28:14.6162942Z stderr:
---
2019-07-17T23:28:14.7678819Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:14.7678939Z +
2019-07-17T23:28:14.7678982Z 
2019-07-17T23:28:14.7679021Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:14.7679065Z Actual stderr saved to /tmp/compiletestNloeDC/volatile.stderr
2019-07-17T23:28:14.7679115Z To update references, run this command from build directory:
2019-07-17T23:28:14.7679382Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'volatile.rs'
2019-07-17T23:28:14.7679452Z error: 1 errors occurred comparing output.
2019-07-17T23:28:14.7679508Z status: exit code: 1
2019-07-17T23:28:14.7679508Z status: exit code: 1
2019-07-17T23:28:14.7680049Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/volatile.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/volatile.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/volatile.stage-id.aux" "-A" "unused"
2019-07-17T23:28:14.7680337Z ------------------------------------------
2019-07-17T23:28:14.7680367Z 
2019-07-17T23:28:14.7680555Z ------------------------------------------
2019-07-17T23:28:14.7680613Z stderr:
---
2019-07-17T23:28:14.7825503Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:14.7825564Z +
2019-07-17T23:28:14.7825593Z 
2019-07-17T23:28:14.7825639Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:14.7825708Z Actual stderr saved to /tmp/compiletestNloeDC/vecs.stderr
2019-07-17T23:28:14.7825768Z To update references, run this command from build directory:
2019-07-17T23:28:14.7826037Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'vecs.rs'
2019-07-17T23:28:14.7826137Z error: 1 errors occurred comparing output.
2019-07-17T23:28:14.7826182Z status: exit code: 1
2019-07-17T23:28:14.7826182Z status: exit code: 1
2019-07-17T23:28:14.7826813Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/vecs.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/vecs.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/vecs.stage-id.aux" "-A" "unused"
2019-07-17T23:28:14.7827153Z ------------------------------------------
2019-07-17T23:28:14.7827206Z 
2019-07-17T23:28:14.7827431Z ------------------------------------------
2019-07-17T23:28:14.7827486Z stderr:
---
2019-07-17T23:28:14.9521577Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:14.9521624Z +
2019-07-17T23:28:14.9521669Z 
2019-07-17T23:28:14.9521712Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:14.9522120Z Actual stderr saved to /tmp/compiletestNloeDC/without-validation.stderr
2019-07-17T23:28:14.9522188Z To update references, run this command from build directory:
2019-07-17T23:28:14.9522434Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'without-validation.rs'
2019-07-17T23:28:14.9522687Z error: 1 errors occurred comparing output.
2019-07-17T23:28:14.9522749Z status: exit code: 1
2019-07-17T23:28:14.9522749Z status: exit code: 1
2019-07-17T23:28:14.9523551Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/without-validation.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/without-validation.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-Zmiri-disable-validation" "-L" "/tmp/compiletestNloeDC/without-validation.stage-id.aux" "-A" "unused"
2019-07-17T23:28:14.9523874Z ------------------------------------------
2019-07-17T23:28:14.9523907Z 
2019-07-17T23:28:14.9524124Z ------------------------------------------
2019-07-17T23:28:14.9524166Z stderr:
---
2019-07-17T23:28:14.9716749Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:14.9716800Z +
2019-07-17T23:28:14.9716838Z 
2019-07-17T23:28:14.9716902Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:14.9717153Z Actual stderr saved to /tmp/compiletestNloeDC/write-bytes.stderr
2019-07-17T23:28:14.9717210Z To update references, run this command from build directory:
2019-07-17T23:28:14.9717485Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'write-bytes.rs'
2019-07-17T23:28:14.9717569Z error: 1 errors occurred comparing output.
2019-07-17T23:28:14.9717633Z status: exit code: 1
2019-07-17T23:28:14.9717633Z status: exit code: 1
2019-07-17T23:28:14.9718593Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/write-bytes.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/write-bytes.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/write-bytes.stage-id.aux" "-A" "unused"
2019-07-17T23:28:14.9718885Z ------------------------------------------
2019-07-17T23:28:14.9718915Z 
2019-07-17T23:28:14.9719101Z ------------------------------------------
2019-07-17T23:28:14.9719158Z stderr:
---
2019-07-17T23:28:15.1159758Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:15.1159994Z +
2019-07-17T23:28:15.1160018Z 
2019-07-17T23:28:15.1160063Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:15.1160106Z Actual stderr saved to /tmp/compiletestNloeDC/zst.stderr
2019-07-17T23:28:15.1160165Z To update references, run this command from build directory:
2019-07-17T23:28:15.1160380Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'zst.rs'
2019-07-17T23:28:15.1160464Z error: 1 errors occurred comparing output.
2019-07-17T23:28:15.1160503Z status: exit code: 1
2019-07-17T23:28:15.1160503Z status: exit code: 1
2019-07-17T23:28:15.1161019Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/zst.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/zst.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/zst.stage-id.aux" "-A" "unused"
2019-07-17T23:28:15.1161305Z ------------------------------------------
2019-07-17T23:28:15.1161342Z 
2019-07-17T23:28:15.1161546Z ------------------------------------------
2019-07-17T23:28:15.1161587Z stderr:
---
2019-07-17T23:28:15.1209400Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:15.1240136Z +
2019-07-17T23:28:15.1240162Z 
2019-07-17T23:28:15.1240214Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:15.1240651Z Actual stderr saved to /tmp/compiletestNloeDC/zero-sized-binary-heap-push.stderr
2019-07-17T23:28:15.1240702Z To update references, run this command from build directory:
2019-07-17T23:28:15.1240938Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'zero-sized-binary-heap-push.rs'
2019-07-17T23:28:15.1241027Z error: 1 errors occurred comparing output.
2019-07-17T23:28:15.1241064Z status: exit code: 1
2019-07-17T23:28:15.1241064Z status: exit code: 1
2019-07-17T23:28:15.1241647Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/zero-sized-binary-heap-push.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/zero-sized-binary-heap-push.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/zero-sized-binary-heap-push.stage-id.aux" "-A" "unused"
2019-07-17T23:28:15.1241924Z ------------------------------------------
2019-07-17T23:28:15.1241971Z 
2019-07-17T23:28:15.1242155Z ------------------------------------------
2019-07-17T23:28:15.1242193Z stderr:
---
2019-07-17T23:28:15.2437455Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:15.2437508Z +
2019-07-17T23:28:15.2437537Z 
2019-07-17T23:28:15.2437602Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:15.2437656Z Actual stderr saved to /tmp/compiletestNloeDC/zst_box.stderr
2019-07-17T23:28:15.2437710Z To update references, run this command from build directory:
2019-07-17T23:28:15.2438014Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'zst_box.rs'
2019-07-17T23:28:15.2438109Z error: 1 errors occurred comparing output.
2019-07-17T23:28:15.2438158Z status: exit code: 1
2019-07-17T23:28:15.2438158Z status: exit code: 1
2019-07-17T23:28:15.2439141Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/zst_box.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/zst_box.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/zst_box.stage-id.aux" "-A" "unused"
2019-07-17T23:28:15.2439425Z ------------------------------------------
2019-07-17T23:28:15.2439454Z 
2019-07-17T23:28:15.2439640Z ------------------------------------------
2019-07-17T23:28:15.2439698Z stderr:
---
2019-07-17T23:28:15.2483340Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T23:28:15.2483611Z +
2019-07-17T23:28:15.2483793Z 
2019-07-17T23:28:15.2483964Z The actual stderr differed from the expected stderr.
2019-07-17T23:28:15.2484136Z Actual stderr saved to /tmp/compiletestNloeDC/zst_variant_drop.stderr
2019-07-17T23:28:15.2484391Z To update references, run this command from build directory:
2019-07-17T23:28:15.2484831Z tests/run-pass/update-references.sh '/tmp/compiletestNloeDC' 'zst_variant_drop.rs'
2019-07-17T23:28:15.2485779Z error: 1 errors occurred comparing output.
2019-07-17T23:28:15.2485978Z status: exit code: 1
2019-07-17T23:28:15.2485978Z status: exit code: 1
2019-07-17T23:28:15.2486933Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/zst_variant_drop.rs" "-L" "/tmp/compiletestNloeDC" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestNloeDC/zst_variant_drop.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestNloeDC/zst_variant_drop.stage-id.aux" "-A" "unused"
2019-07-17T23:28:15.2487717Z ------------------------------------------
2019-07-17T23:28:15.2487959Z 
2019-07-17T23:28:15.2488402Z ------------------------------------------
2019-07-17T23:28:15.2488844Z stderr:
---
2019-07-17T23:28:15.2793913Z Verifying status of miri...
2019-07-17T23:28:15.2803179Z Verifying status of embedded-book...
2019-07-17T23:28:15.2813087Z This PR updated 'src/doc/embedded-book', verifying if status is 'test-pass'...
2019-07-17T23:28:15.2823265Z Verifying status of rustc-guide...
2019-07-17T23:28:15.2924498Z /tmp/checktools.sh: 38: /tmp/checktools.sh: TOOLSTATE_REPO: parameter not set
2019-07-17T23:28:15.9164566Z ##[error]Bash exited with code '2'.
2019-07-17T23:28:15.9198306Z ##[section]Starting: Checkout
2019-07-17T23:28:15.9200171Z ==============================================================================
2019-07-17T23:28:15.9200214Z Task         : Get sources
2019-07-17T23:28:15.9200250Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
