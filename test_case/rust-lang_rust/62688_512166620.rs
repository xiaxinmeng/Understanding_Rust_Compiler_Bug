plain
2019-07-17T04:56:36.1488756Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-17T04:56:36.1689347Z ##[command]git config gc.auto 0
2019-07-17T04:56:36.1725510Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-17T04:56:36.1778257Z ##[command]git config --get-all http.proxy
2019-07-17T04:56:36.1914901Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62688/merge:refs/remotes/pull/62688/merge
---
2019-07-17T04:57:09.8799295Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-17T04:57:09.8799380Z 
2019-07-17T04:57:09.8799590Z   git checkout -b <new-branch-name>
2019-07-17T04:57:09.8799640Z 
2019-07-17T04:57:09.8799687Z HEAD is now at 133efb6c1 Merge d8ffe7ddbfa4ba65a885b493278d39f93f3c6bd1 into 38798c6d68394874686dfa3d03e56e12a3ff3d54
2019-07-17T04:57:09.8942380Z ##[section]Finishing: Checkout
2019-07-17T04:57:09.8950120Z ##[section]Starting: Decide whether to run this job
2019-07-17T04:57:09.8952744Z Task         : Bash
2019-07-17T04:57:09.8952805Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-17T04:57:09.8952851Z Version      : 3.151.2
2019-07-17T04:57:09.8952895Z Author       : Microsoft Corporation
2019-07-17T04:57:09.8952895Z Author       : Microsoft Corporation
2019-07-17T04:57:09.8952944Z Help         : https://docs.microsoft.com/azure/devops/pipelines/tasks/utility/bash
2019-07-17T04:57:09.8953014Z ==============================================================================
2019-07-17T04:57:10.0380358Z Generating script.
2019-07-17T04:57:10.0412451Z ========================== Starting Command Output ===========================
2019-07-17T04:57:10.0432599Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/7a785c24-1838-4217-b71a-241d11defb9d.sh
2019-07-17T04:57:10.3910682Z Executing the job since submodules are updated
2019-07-17T04:57:10.3999792Z ##[section]Finishing: Decide whether to run this job
2019-07-17T04:57:10.4010173Z ==============================================================================
2019-07-17T04:57:10.4010227Z Task         : Bash
2019-07-17T04:57:10.4010314Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-17T04:57:10.4010358Z Version      : 3.151.2
---
2019-07-17T04:59:03.8741397Z Attempting with retry: curl -y 30 -Y 10 --connect-timeout 30 -f -L -C - -o /tmp/rustci_docker_cache https://.s3.amazonaws.com/docker/5237541d5b559bcea2f168e4ed3efe600aec9344996d80ee3a8ea6b1b5d4d12b46df9cf61ac5cec12b0346064f67d51e8664869834273098748b7174157de4e3
2019-07-17T04:59:03.8810952Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-17T04:59:03.8811275Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-17T04:59:03.8811431Z 
2019-07-17T04:59:03.8812154Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-17T04:59:04.8882830Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-17T04:59:04.8883113Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-17T04:59:04.8883240Z 
2019-07-17T04:59:04.8883240Z 
2019-07-17T04:59:04.8925559Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-17T04:59:06.9008578Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-17T04:59:06.9008666Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-17T04:59:06.9008734Z 
2019-07-17T04:59:06.9008734Z 
2019-07-17T04:59:06.9043469Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-17T04:59:09.9117984Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-17T04:59:09.9118152Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-17T04:59:09.9118188Z 
2019-07-17T04:59:09.9118188Z 
2019-07-17T04:59:09.9166989Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-17T04:59:13.9268271Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-17T04:59:13.9268427Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-17T04:59:13.9269154Z 
2019-07-17T04:59:13.9269154Z 
2019-07-17T04:59:13.9285558Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-17T04:59:13.9285670Z The command has failed after 5 attempts.
2019-07-17T04:59:13.9837916Z open /tmp/rustci_docker_cache: no such file or directory
2019-07-17T04:59:13.9864501Z Attempting with retry: docker build --rm -t rust-ci -f /home/vsts/work/1/s/src/ci/docker/x86_64-gnu-tools/Dockerfile /home/vsts/work/1/s/src/ci/docker
2019-07-17T04:59:14.2280781Z Sending build context to Docker daemon  521.2kB
2019-07-17T04:59:14.2281501Z 
2019-07-17T04:59:14.2598989Z Step 1/9 : FROM ubuntu:16.04
---
2019-07-17T04:59:30.3412187Z Reading package lists...
2019-07-17T04:59:31.3664895Z Reading package lists...
2019-07-17T04:59:31.5675487Z Building dependency tree...
2019-07-17T04:59:31.5675570Z Reading state information...
2019-07-17T04:59:31.6823121Z The following additional packages will be installed:
2019-07-17T04:59:31.6824957Z   binutils bzip2 cmake-data cpp cpp-5 dpkg-dev g++-5 gcc gcc-5 git-man
2019-07-17T04:59:31.6825238Z   libarchive13 libasan2 libasn1-8-heimdal libatomic1 libbz2-1.0 libc-dev-bin
2019-07-17T04:59:31.6825672Z   liberror-perl libexpat1 libffi6 libgcc-5-dev libgdbm3 libglib2.0-0 libgmp10
2019-07-17T04:59:31.6825912Z   libgnutls30 libgomp1 libgssapi-krb5-2 libgssapi3-heimdal libhcrypto4-heimdal
2019-07-17T04:59:31.6826129Z   libheimbase1-heimdal libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal
2019-07-17T04:59:31.6826177Z   libicu55 libidn11 libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1
---
2019-07-17T04:59:31.6828363Z   libssl1.0.0 libstdc++-5-dev libtasn1-6 libtsan0 libubsan0 libwind0-heimdal
2019-07-17T04:59:31.6828614Z   libxml2 linux-libc-dev mime-support openssl patch perl perl-modules-5.22
2019-07-17T04:59:31.6828843Z   python2.7-minimal zlib1g-dev
2019-07-17T04:59:31.6828895Z Suggested packages:
2019-07-17T04:59:31.6829150Z   binutils-doc bzip2-doc codeblocks eclipse ninja-build cpp-doc gcc-5-locales
2019-07-17T04:59:31.6829420Z   debian-keyring g++-multilib g++-5-multilib gcc-5-doc libstdc++6-5-dbg
2019-07-17T04:59:31.6829676Z   gcc-multilib manpages-dev autoconf automake libtool flex bison gdb gcc-doc
2019-07-17T04:59:31.6830218Z   libasan2-dbg liblsan0-dbg libtsan0-dbg libubsan0-dbg libcilkrts5-dbg
2019-07-17T04:59:31.6830462Z   libmpx0-dbg libquadmath0-dbg gettext-base git-daemon-run
2019-07-17T04:59:31.6830462Z   libmpx0-dbg libquadmath0-dbg gettext-base git-daemon-run
2019-07-17T04:59:31.6830791Z   | git-daemon-sysvinit git-doc git-el git-email git-gui gitk gitweb git-arch
2019-07-17T04:59:31.6831093Z   git-cvs git-mediawiki git-svn lrzip glibc-doc gnutls-bin krb5-doc krb5-user
2019-07-17T04:59:31.6831378Z   libstdc++-5-doc make-doc ed diffutils-doc perl-doc libterm-readline-gnu-perl
2019-07-17T04:59:31.6831638Z   | libterm-readline-perl-perl python2.7-doc binfmt-support
2019-07-17T04:59:31.6831690Z Recommended packages:
2019-07-17T04:59:31.6831981Z   build-essential fakeroot libalgorithm-merge-perl less rsync ssh-client
2019-07-17T04:59:31.6832243Z   manpages manpages-dev libfile-fcntllock-perl libglib2.0-data
2019-07-17T04:59:31.6832513Z   shared-mime-info xdg-user-dirs krb5-locales libsasl2-modules libssl-doc
2019-07-17T04:59:31.6832769Z   xml-core netbase rename
2019-07-17T04:59:31.6832824Z The following NEW packages will be installed:
2019-07-17T04:59:31.6833105Z   binutils bzip2 ca-certificates cmake cmake-data cpp cpp-5 curl dpkg-dev file
2019-07-17T04:59:31.6833705Z   g++ g++-5 gcc gcc-5 git git-man libarchive13 libasan2 libasn1-8-heimdal
2019-07-17T04:59:31.6834458Z   libcurl3-gnutls libdpkg-perl liberror-perl libexpat1 libffi6 libgcc-5-dev
2019-07-17T04:59:31.6834668Z   libgdbm3 libglib2.0-0 libgmp10 libgnutls30 libgomp1 libgssapi-krb5-2
2019-07-17T04:59:31.6834852Z   libgssapi3-heimdal libhcrypto4-heimdal libheimbase1-heimdal
2019-07-17T04:59:31.6835046Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-17T04:59:31.6835046Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-17T04:59:31.6835261Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-17T04:59:31.6835587Z   libkrb5-3 libkrb5support0 libldap-2.4-2 liblsan0 liblzo2-2 libmagic1 libmpc3
2019-07-17T04:59:31.6835833Z   libmpfr4 libmpx0 libnettle6 libp11-kit0 libperl5.22 libpython2.7-minimal
2019-07-17T04:59:31.6836057Z   libpython2.7-stdlib libquadmath0 libroken18-heimdal librtmp1 libsasl2-2
2019-07-17T04:59:31.6836251Z   libsasl2-modules-db libsqlite3-0 libssl-dev libssl1.0.0 libstdc++-5-dev
2019-07-17T04:59:31.6836446Z   libtasn1-6 libtsan0 libubsan0 libwind0-heimdal libxml2 linux-libc-dev make
2019-07-17T04:59:31.6836656Z   mime-support openssl patch perl perl-modules-5.22 pkg-config python2.7
2019-07-17T04:59:31.6836830Z   python2.7-minimal sudo xz-utils zlib1g-dev
2019-07-17T04:59:31.6836871Z The following packages will be upgraded:
2019-07-17T04:59:32.0517095Z 1 upgraded, 92 newly installed, 0 to remove and 5 not upgraded.
2019-07-17T04:59:32.0517480Z Need to get 71.6 MB of archives.
2019-07-17T04:59:32.0517743Z After this operation, 310 MB of additional disk space will be used.
2019-07-17T04:59:32.0518798Z Get:1 http://archive.ubuntu.com/ubuntu xenial/main amd64 libgdbm3 amd64 1.8.3-13.1 [16.9 kB]
---
2019-07-17T05:00:36.0995426Z  ---> cd69413d7684
2019-07-17T05:00:36.1035196Z Successfully built cd69413d7684
2019-07-17T05:00:36.2976358Z Successfully tagged rust-ci:latest
2019-07-17T05:00:36.3717134Z Built container sha256:cd69413d76847141766aa1d4bcfa1e99b3ed0d414761b2ed5dd159f09a132921
2019-07-17T05:00:36.3732109Z Uploading finished image to https://.s3.amazonaws.com/docker/5237541d5b559bcea2f168e4ed3efe600aec9344996d80ee3a8ea6b1b5d4d12b46df9cf61ac5cec12b0346064f67d51e8664869834273098748b7174157de4e3
2019-07-17T05:01:19.7085434Z upload failed: - to s3:///docker/5237541d5b559bcea2f168e4ed3efe600aec9344996d80ee3a8ea6b1b5d4d12b46df9cf61ac5cec12b0346064f67d51e8664869834273098748b7174157de4e3 Parameter validation failed:
2019-07-17T05:01:19.7089921Z Invalid bucket name "": Bucket name must match the regex "^[a-zA-Z0-9.\-_]{1,255}$"
2019-07-17T05:01:20.5479629Z [CI_JOB_NAME=LinuxTools]
2019-07-17T05:01:20.7410265Z [CI_JOB_NAME=LinuxTools]
2019-07-17T05:01:20.8005171Z configure: processing command line
2019-07-17T05:01:20.8006002Z configure: 
2019-07-17T05:01:20.8007001Z configure: rust.dist-src        := False
2019-07-17T05:01:20.8007608Z configure: rust.save-toolstates := /tmp/toolstates.json
---
2019-07-17T08:53:03.3104263Z test config::test::test_config_used_to_toml ... ok
2019-07-17T08:53:03.3105041Z test config::test::test_dump_default_config ... ok
2019-07-17T08:53:03.3107659Z test config::test::test_print_docs_exclude_unstable ... ok
2019-07-17T08:53:03.3110416Z test config::test::test_print_docs_include_unstable ... ok
2019-07-17T08:53:03.3115051Z test emitter::checkstyle::xml::tests::other_characters_are_not_escaped ... ok
2019-07-17T08:53:03.3115269Z test config::test::test_was_set ... ok
2019-07-17T08:53:03.3117249Z test emitter::checkstyle::xml::tests::special_characters_are_escaped_in_string_with_other_characters ... ok
2019-07-17T08:53:03.3120355Z test emitter::checkstyle::xml::tests::special_characters_are_escaped ... ok
2019-07-17T08:53:03.3134101Z test formatting::newline_style::tests::applies_unix_newlines ... ok
2019-07-17T08:53:03.3134874Z test formatting::newline_style::tests::applies_unix_newlines_to_string_with_unix_and_windows_newlines ... ok
2019-07-17T08:53:03.3135125Z test formatting::newline_style::tests::applying_unix_newlines_changes_nothing_for_unix_newlines ... ok
2019-07-17T08:53:03.3135795Z test formatting::newline_style::tests::applying_windows_newlines_changes_nothing_for_windows_newlines ... ok
---
2019-07-17T08:56:53.1610010Z -args
2019-07-17T08:56:53.1610219Z -
2019-07-17T08:56:53.1610249Z 
2019-07-17T08:56:53.1610295Z The actual stdout differed from the expected stdout.
2019-07-17T08:56:53.1610512Z Actual stdout saved to /tmp/compiletestz0Rj1T/args.stdout
2019-07-17T08:56:53.1610633Z error[E0080]: Miri evaluation error: attempted to do invalid arithmetic on pointers that would leak base addresses, e.g., comparing pointers into different allocations
2019-07-17T08:56:53.1610901Z     --> /checkout/src/libcore/intrinsics.rs:1340:19
2019-07-17T08:56:53.1610951Z      |
2019-07-17T08:56:53.1611000Z 1340 |     let diff = if src_usize > dst_usize {
---
2019-07-17T08:56:53.1615225Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:56:53.1615274Z +
2019-07-17T08:56:53.1615309Z 
2019-07-17T08:56:53.1615414Z The actual stderr differed from the expected stderr.
2019-07-17T08:56:53.1615465Z Actual stderr saved to /tmp/compiletestz0Rj1T/args.stderr
2019-07-17T08:56:53.1615534Z To update references, run this command from build directory:
2019-07-17T08:56:53.1615799Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'args.rs'
2019-07-17T08:56:53.1615882Z error: 2 errors occurred comparing output.
2019-07-17T08:56:53.1615947Z status: exit code: 1
2019-07-17T08:56:53.1615947Z status: exit code: 1
2019-07-17T08:56:53.1616691Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/args.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/args.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/args.stage-id.aux" "-A" "unused"
2019-07-17T08:56:53.1617081Z ------------------------------------------
2019-07-17T08:56:53.1617117Z 
2019-07-17T08:56:53.1617349Z ------------------------------------------
2019-07-17T08:56:53.1617416Z stderr:
---
2019-07-17T08:56:53.2369603Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:56:53.2369828Z +
2019-07-17T08:56:53.2369857Z 
2019-07-17T08:56:53.2369939Z The actual stderr differed from the expected stderr.
2019-07-17T08:56:53.2369991Z Actual stderr saved to /tmp/compiletestz0Rj1T/arrays.stderr
2019-07-17T08:56:53.2370051Z To update references, run this command from build directory:
2019-07-17T08:56:53.2370502Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'arrays.rs'
2019-07-17T08:56:53.2370578Z error: 1 errors occurred comparing output.
2019-07-17T08:56:53.2370623Z status: exit code: 1
2019-07-17T08:56:53.2370623Z status: exit code: 1
2019-07-17T08:56:53.2371245Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/arrays.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/arrays.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/arrays.stage-id.aux" "-A" "unused"
2019-07-17T08:56:53.2371698Z ------------------------------------------
2019-07-17T08:56:53.2371744Z 
2019-07-17T08:56:53.2371997Z ------------------------------------------
2019-07-17T08:56:53.2372061Z stderr:
---
2019-07-17T08:56:53.3435934Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:56:53.3487508Z +
2019-07-17T08:56:53.3487581Z 
2019-07-17T08:56:53.3489313Z The actual stderr differed from the expected stderr.
2019-07-17T08:56:53.3490272Z Actual stderr saved to /tmp/compiletestz0Rj1T/associated-const.stderr
2019-07-17T08:56:53.3490336Z To update references, run this command from build directory:
2019-07-17T08:56:53.3490602Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'associated-const.rs'
2019-07-17T08:56:53.3490710Z error: 1 errors occurred comparing output.
2019-07-17T08:56:53.3490756Z status: exit code: 1
2019-07-17T08:56:53.3490756Z status: exit code: 1
2019-07-17T08:56:53.3492418Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/associated-const.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/associated-const.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/associated-const.stage-id.aux" "-A" "unused"
2019-07-17T08:56:53.3493769Z ------------------------------------------
2019-07-17T08:56:53.3493812Z 
2019-07-17T08:56:53.3494047Z ------------------------------------------
2019-07-17T08:56:53.3494094Z stderr:
---
2019-07-17T08:56:53.3952917Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:56:53.3952990Z +
2019-07-17T08:56:53.3953040Z 
2019-07-17T08:56:53.3953086Z The actual stderr differed from the expected stderr.
2019-07-17T08:56:53.3953138Z Actual stderr saved to /tmp/compiletestz0Rj1T/assume_bug.stderr
2019-07-17T08:56:53.3953190Z To update references, run this command from build directory:
2019-07-17T08:56:53.3953562Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'assume_bug.rs'
2019-07-17T08:56:53.3953646Z error: 1 errors occurred comparing output.
2019-07-17T08:56:53.3953711Z status: exit code: 1
2019-07-17T08:56:53.3953711Z status: exit code: 1
2019-07-17T08:56:53.3954599Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/assume_bug.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/assume_bug.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/assume_bug.stage-id.aux" "-A" "unused"
2019-07-17T08:56:53.3955448Z ------------------------------------------
2019-07-17T08:56:53.3955485Z 
2019-07-17T08:56:53.3955699Z ------------------------------------------
2019-07-17T08:56:53.3955765Z stderr:
---
2019-07-17T08:56:53.5127817Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:56:53.5127889Z +
2019-07-17T08:56:53.5127916Z 
2019-07-17T08:56:53.5127961Z The actual stderr differed from the expected stderr.
2019-07-17T08:56:53.5128371Z Actual stderr saved to /tmp/compiletestz0Rj1T/async-fn.stderr
2019-07-17T08:56:53.5128439Z To update references, run this command from build directory:
2019-07-17T08:56:53.5128677Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'async-fn.rs'
2019-07-17T08:56:53.5128773Z error: 1 errors occurred comparing output.
2019-07-17T08:56:53.5128818Z status: exit code: 1
2019-07-17T08:56:53.5128818Z status: exit code: 1
2019-07-17T08:56:53.5129497Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/async-fn.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/async-fn.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/async-fn.stage-id.aux" "-A" "unused"
2019-07-17T08:56:53.5129845Z ------------------------------------------
2019-07-17T08:56:53.5129876Z 
2019-07-17T08:56:53.5130370Z ------------------------------------------
2019-07-17T08:56:53.5130415Z stderr:
---
2019-07-17T08:56:53.5807582Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:56:53.5807659Z +
2019-07-17T08:56:53.5807688Z 
2019-07-17T08:56:53.5807736Z The actual stderr differed from the expected stderr.
2019-07-17T08:56:53.5808043Z Actual stderr saved to /tmp/compiletestz0Rj1T/atomic-access-bool.stderr
2019-07-17T08:56:53.5808102Z To update references, run this command from build directory:
2019-07-17T08:56:53.5808569Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'atomic-access-bool.rs'
2019-07-17T08:56:53.5808662Z error: 1 errors occurred comparing output.
2019-07-17T08:56:53.5808704Z status: exit code: 1
2019-07-17T08:56:53.5808704Z status: exit code: 1
2019-07-17T08:56:53.5809462Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/atomic-access-bool.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/atomic-access-bool.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/atomic-access-bool.stage-id.aux" "-A" "unused"
2019-07-17T08:56:53.5809820Z ------------------------------------------
2019-07-17T08:56:53.5809868Z 
2019-07-17T08:56:53.5810322Z ------------------------------------------
2019-07-17T08:56:53.5810368Z stderr:
---
2019-07-17T08:56:53.7249872Z +
2019-07-17T08:56:53.7250205Z thread '[ui] run-pass/bad_substs.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
2019-07-17T08:56:53.7250367Z 
2019-07-17T08:56:53.7250433Z The actual stderr differed from the expected stderr.
2019-07-17T08:56:53.7250484Z Actual stderr saved to /tmp/compiletestz0Rj1T/bad_substs.stderr
2019-07-17T08:56:53.7250535Z To update references, run this command from build directory:
2019-07-17T08:56:53.7250802Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'bad_substs.rs'
2019-07-17T08:56:53.7250980Z error: 1 errors occurred comparing output.
2019-07-17T08:56:53.7251035Z status: exit code: 1
2019-07-17T08:56:53.7251035Z status: exit code: 1
2019-07-17T08:56:53.7251722Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/bad_substs.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/bad_substs.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/bad_substs.stage-id.aux" "-A" "unused"
2019-07-17T08:56:53.7252036Z ------------------------------------------
2019-07-17T08:56:53.7252069Z 
2019-07-17T08:56:53.7252277Z ------------------------------------------
2019-07-17T08:56:53.7252345Z stderr:
---
2019-07-17T08:56:53.7288067Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:56:53.7288117Z +
2019-07-17T08:56:53.7288144Z 
2019-07-17T08:56:53.7288190Z The actual stderr differed from the expected stderr.
2019-07-17T08:56:53.7288476Z Actual stderr saved to /tmp/compiletestz0Rj1T/atomic-compare_exchange.stderr
2019-07-17T08:56:53.7288533Z To update references, run this command from build directory:
2019-07-17T08:56:53.7288892Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'atomic-compare_exchange.rs'
2019-07-17T08:56:53.7289013Z error: 1 errors occurred comparing output.
2019-07-17T08:56:53.7289060Z status: exit code: 1
2019-07-17T08:56:53.7289060Z status: exit code: 1
2019-07-17T08:56:53.7289798Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/atomic-compare_exchange.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/atomic-compare_exchange.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/atomic-compare_exchange.stage-id.aux" "-A" "unused"
2019-07-17T08:56:53.7290244Z ------------------------------------------
2019-07-17T08:56:53.7290298Z 
2019-07-17T08:56:53.7290526Z ------------------------------------------
2019-07-17T08:56:53.7290573Z stderr:
---
2019-07-17T08:56:53.9052300Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:56:53.9052351Z +
2019-07-17T08:56:53.9052400Z 
2019-07-17T08:56:53.9052452Z The actual stderr differed from the expected stderr.
2019-07-17T08:56:53.9052505Z Actual stderr saved to /tmp/compiletestz0Rj1T/bools.stderr
2019-07-17T08:56:53.9052558Z To update references, run this command from build directory:
2019-07-17T08:56:53.9052850Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'bools.rs'
2019-07-17T08:56:53.9053098Z error: 1 errors occurred comparing output.
2019-07-17T08:56:53.9053173Z status: exit code: 1
2019-07-17T08:56:53.9053173Z status: exit code: 1
2019-07-17T08:56:53.9054001Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/bools.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/bools.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/bools.stage-id.aux" "-A" "unused"
2019-07-17T08:56:53.9054316Z ------------------------------------------
2019-07-17T08:56:53.9054349Z 
2019-07-17T08:56:53.9055318Z ------------------------------------------
2019-07-17T08:56:53.9055392Z stderr:
---
2019-07-17T08:56:53.9607945Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:56:53.9608011Z +
2019-07-17T08:56:53.9608038Z 
2019-07-17T08:56:53.9608083Z The actual stderr differed from the expected stderr.
2019-07-17T08:56:53.9608208Z Actual stderr saved to /tmp/compiletestz0Rj1T/binops.stderr
2019-07-17T08:56:53.9608284Z To update references, run this command from build directory:
2019-07-17T08:56:53.9608715Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'binops.rs'
2019-07-17T08:56:53.9608804Z error: 1 errors occurred comparing output.
2019-07-17T08:56:53.9608848Z status: exit code: 1
2019-07-17T08:56:53.9608848Z status: exit code: 1
2019-07-17T08:56:53.9609427Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/binops.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/binops.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/binops.stage-id.aux" "-A" "unused"
2019-07-17T08:56:53.9609720Z ------------------------------------------
2019-07-17T08:56:53.9609750Z 
2019-07-17T08:56:53.9609968Z ------------------------------------------
2019-07-17T08:56:53.9610010Z stderr:
---
2019-07-17T08:56:54.0851361Z -
2019-07-17T08:56:54.0851393Z 
2019-07-17T08:56:54.0851745Z thread '[ui] run-pass/box-pair-to-vec.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
2019-07-17T08:56:54.0851990Z The actual stdout differed from the expected stdout.
2019-07-17T08:56:54.0852242Z Actual stdout saved to /tmp/compiletestz0Rj1T/box-pair-to-vec.stdout
2019-07-17T08:56:54.0852369Z error[E0080]: Miri evaluation error: attempted to do invalid arithmetic on pointers that would leak base addresses, e.g., comparing pointers into different allocations
2019-07-17T08:56:54.0852781Z     --> /checkout/src/libcore/intrinsics.rs:1340:19
2019-07-17T08:56:54.0852871Z      |
2019-07-17T08:56:54.0852931Z 1340 |     let diff = if src_usize > dst_usize {
---
2019-07-17T08:56:54.0857306Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:56:54.0857356Z +
2019-07-17T08:56:54.0857385Z 
2019-07-17T08:56:54.0857449Z The actual stderr differed from the expected stderr.
2019-07-17T08:56:54.0857716Z Actual stderr saved to /tmp/compiletestz0Rj1T/box-pair-to-vec.stderr
2019-07-17T08:56:54.0857780Z To update references, run this command from build directory:
2019-07-17T08:56:54.0858358Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'box-pair-to-vec.rs'
2019-07-17T08:56:54.0858516Z error: 2 errors occurred comparing output.
2019-07-17T08:56:54.0858561Z status: exit code: 1
2019-07-17T08:56:54.0858561Z status: exit code: 1
2019-07-17T08:56:54.0859234Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/box-pair-to-vec.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/box-pair-to-vec.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/box-pair-to-vec.stage-id.aux" "-A" "unused"
2019-07-17T08:56:54.0859535Z ------------------------------------------
2019-07-17T08:56:54.0859567Z 
2019-07-17T08:56:54.0859769Z ------------------------------------------
2019-07-17T08:56:54.0860010Z stderr:
---
2019-07-17T08:56:54.0958869Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:56:54.0958932Z +
2019-07-17T08:56:54.0958958Z 
2019-07-17T08:56:54.0959002Z The actual stderr differed from the expected stderr.
2019-07-17T08:56:54.0959076Z Actual stderr saved to /tmp/compiletestz0Rj1T/box_box_trait.stderr
2019-07-17T08:56:54.0959127Z To update references, run this command from build directory:
2019-07-17T08:56:54.0959501Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'box_box_trait.rs'
2019-07-17T08:56:54.0959595Z error: 1 errors occurred comparing output.
2019-07-17T08:56:54.0959641Z status: exit code: 1
2019-07-17T08:56:54.0959641Z status: exit code: 1
2019-07-17T08:56:54.0960479Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/box_box_trait.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/box_box_trait.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/box_box_trait.stage-id.aux" "-A" "unused"
2019-07-17T08:56:54.0960793Z ------------------------------------------
2019-07-17T08:56:54.0960850Z 
2019-07-17T08:56:54.0961065Z ------------------------------------------
2019-07-17T08:56:54.0961117Z stderr:
---
2019-07-17T08:56:54.2706221Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:56:54.2706314Z +
2019-07-17T08:56:54.2706368Z 
2019-07-17T08:56:54.2706418Z The actual stderr differed from the expected stderr.
2019-07-17T08:56:54.2706470Z Actual stderr saved to /tmp/compiletestz0Rj1T/c_enums.stderr
2019-07-17T08:56:54.2706533Z To update references, run this command from build directory:
2019-07-17T08:56:54.2706840Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'c_enums.rs'
2019-07-17T08:56:54.2707087Z error: 1 errors occurred comparing output.
2019-07-17T08:56:54.2707152Z status: exit code: 1
2019-07-17T08:56:54.2707152Z status: exit code: 1
2019-07-17T08:56:54.2707819Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/c_enums.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/c_enums.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/c_enums.stage-id.aux" "-A" "unused"
2019-07-17T08:56:54.2708136Z ------------------------------------------
2019-07-17T08:56:54.2708169Z 
2019-07-17T08:56:54.2708379Z ------------------------------------------
2019-07-17T08:56:54.2708452Z stderr:
---
2019-07-17T08:56:54.2808356Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:56:54.2808405Z +
2019-07-17T08:56:54.2808444Z 
2019-07-17T08:56:54.2808511Z The actual stderr differed from the expected stderr.
2019-07-17T08:56:54.2808673Z Actual stderr saved to /tmp/compiletestz0Rj1T/btreemap.stderr
2019-07-17T08:56:54.2808724Z To update references, run this command from build directory:
2019-07-17T08:56:54.2809036Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'btreemap.rs'
2019-07-17T08:56:54.2809120Z error: 1 errors occurred comparing output.
2019-07-17T08:56:54.2809184Z status: exit code: 1
2019-07-17T08:56:54.2809184Z status: exit code: 1
2019-07-17T08:56:54.2809827Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/btreemap.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/btreemap.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/btreemap.stage-id.aux" "-A" "unused"
2019-07-17T08:56:54.2810145Z ------------------------------------------
2019-07-17T08:56:54.2810187Z 
2019-07-17T08:56:54.2810399Z ------------------------------------------
2019-07-17T08:56:54.2810464Z stderr:
---
2019-07-17T08:56:54.4402950Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:56:54.4403000Z +
2019-07-17T08:56:54.4403029Z 
2019-07-17T08:56:54.4403077Z The actual stderr differed from the expected stderr.
2019-07-17T08:56:54.4403158Z Actual stderr saved to /tmp/compiletestz0Rj1T/call_drop_on_array_elements.stderr
2019-07-17T08:56:54.4403300Z To update references, run this command from build directory:
2019-07-17T08:56:54.4403619Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'call_drop_on_array_elements.rs'
2019-07-17T08:56:54.4403722Z error: 1 errors occurred comparing output.
2019-07-17T08:56:54.4403769Z status: exit code: 1
2019-07-17T08:56:54.4403769Z status: exit code: 1
2019-07-17T08:56:54.4405226Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/call_drop_on_array_elements.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/call_drop_on_array_elements.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/call_drop_on_array_elements.stage-id.aux" "-A" "unused"
2019-07-17T08:56:54.4405567Z ------------------------------------------
2019-07-17T08:56:54.4405630Z 
2019-07-17T08:56:54.4405850Z ------------------------------------------
2019-07-17T08:56:54.4405895Z stderr:
---
2019-07-17T08:56:54.4428482Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:56:54.4428524Z +
2019-07-17T08:56:54.4428619Z 
2019-07-17T08:56:54.4428677Z The actual stderr differed from the expected stderr.
2019-07-17T08:56:54.4428725Z Actual stderr saved to /tmp/compiletestz0Rj1T/call_drop_on_fat_ptr_array_elements.stderr
2019-07-17T08:56:54.4428771Z To update references, run this command from build directory:
2019-07-17T08:56:54.4429054Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'call_drop_on_fat_ptr_array_elements.rs'
2019-07-17T08:56:54.4429126Z error: 1 errors occurred comparing output.
2019-07-17T08:56:54.4429165Z status: exit code: 1
2019-07-17T08:56:54.4429165Z status: exit code: 1
2019-07-17T08:56:54.4430009Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/call_drop_on_fat_ptr_array_elements.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/call_drop_on_fat_ptr_array_elements.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/call_drop_on_fat_ptr_array_elements.stage-id.aux" "-A" "unused"
2019-07-17T08:56:54.4430423Z ------------------------------------------
2019-07-17T08:56:54.4430456Z 
2019-07-17T08:56:54.4430664Z ------------------------------------------
2019-07-17T08:56:54.4430783Z stderr:
---
2019-07-17T08:56:54.6051868Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:56:54.6052200Z +
2019-07-17T08:56:54.6052394Z 
2019-07-17T08:56:54.6053847Z The actual stderr differed from the expected stderr.
2019-07-17T08:56:54.6056109Z Actual stderr saved to /tmp/compiletestz0Rj1T/call_drop_on_zst_array_elements.stderr
2019-07-17T08:56:54.6057621Z To update references, run this command from build directory:
2019-07-17T08:56:54.6060092Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'call_drop_on_zst_array_elements.rs'
2019-07-17T08:56:54.6062251Z error: 1 errors occurred comparing output.
2019-07-17T08:56:54.6065120Z status: exit code: 1
2019-07-17T08:56:54.6065120Z status: exit code: 1
2019-07-17T08:56:54.6071467Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/call_drop_on_zst_array_elements.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/call_drop_on_zst_array_elements.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/call_drop_on_zst_array_elements.stage-id.aux" "-A" "unused"
2019-07-17T08:56:54.6072486Z ------------------------------------------
2019-07-17T08:56:54.6072536Z 
2019-07-17T08:56:54.6073154Z ------------------------------------------
2019-07-17T08:56:54.6073209Z stderr:
---
2019-07-17T08:56:54.6096908Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:56:54.6096972Z +
2019-07-17T08:56:54.6097000Z 
2019-07-17T08:56:54.6097046Z The actual stderr differed from the expected stderr.
2019-07-17T08:56:54.6097098Z Actual stderr saved to /tmp/compiletestz0Rj1T/call_drop_through_owned_slice.stderr
2019-07-17T08:56:54.6097177Z To update references, run this command from build directory:
2019-07-17T08:56:54.6097460Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'call_drop_through_owned_slice.rs'
2019-07-17T08:56:54.6097567Z error: 1 errors occurred comparing output.
2019-07-17T08:56:54.6097613Z status: exit code: 1
2019-07-17T08:56:54.6097613Z status: exit code: 1
2019-07-17T08:56:54.6098305Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/call_drop_through_owned_slice.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/call_drop_through_owned_slice.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/call_drop_through_owned_slice.stage-id.aux" "-A" "unused"
2019-07-17T08:56:54.6098623Z ------------------------------------------
2019-07-17T08:56:54.6098655Z 
2019-07-17T08:56:54.6098890Z ------------------------------------------
2019-07-17T08:56:54.6098937Z stderr:
---
2019-07-17T08:56:54.7436994Z +
2019-07-17T08:56:54.7437444Z thread '[ui] run-pass/call_drop_through_trait_object.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
2019-07-17T08:56:54.7437508Z 
2019-07-17T08:56:54.7437567Z The actual stderr differed from the expected stderr.
2019-07-17T08:56:54.7437623Z Actual stderr saved to /tmp/compiletestz0Rj1T/call_drop_through_trait_object.stderr
2019-07-17T08:56:54.7437703Z To update references, run this command from build directory:
2019-07-17T08:56:54.7438007Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'call_drop_through_trait_object.rs'
2019-07-17T08:56:54.7438092Z error: 1 errors occurred comparing output.
2019-07-17T08:56:54.7438156Z status: exit code: 1
2019-07-17T08:56:54.7438156Z status: exit code: 1
2019-07-17T08:56:54.7439002Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/call_drop_through_trait_object.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/call_drop_through_trait_object.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/call_drop_through_trait_object.stage-id.aux" "-A" "unused"
2019-07-17T08:56:54.7439423Z ------------------------------------------
2019-07-17T08:56:54.7439460Z 
2019-07-17T08:56:54.7439679Z ------------------------------------------
2019-07-17T08:56:54.7439724Z stderr:
---
2019-07-17T08:56:54.7508940Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:56:54.7509045Z +
2019-07-17T08:56:54.7509088Z 
2019-07-17T08:56:54.7509147Z The actual stderr differed from the expected stderr.
2019-07-17T08:56:54.7509197Z Actual stderr saved to /tmp/compiletestz0Rj1T/call_drop_through_trait_object_rc.stderr
2019-07-17T08:56:54.7509255Z To update references, run this command from build directory:
2019-07-17T08:56:54.7509564Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'call_drop_through_trait_object_rc.rs'
2019-07-17T08:56:54.7509639Z error: 1 errors occurred comparing output.
2019-07-17T08:56:54.7509688Z status: exit code: 1
2019-07-17T08:56:54.7509688Z status: exit code: 1
2019-07-17T08:56:54.7511581Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/call_drop_through_trait_object_rc.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/call_drop_through_trait_object_rc.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/call_drop_through_trait_object_rc.stage-id.aux" "-A" "unused"
2019-07-17T08:56:54.7511915Z ------------------------------------------
2019-07-17T08:56:54.7511948Z 
2019-07-17T08:56:54.7512163Z ------------------------------------------
2019-07-17T08:56:54.7512209Z stderr:
---
2019-07-17T08:56:54.9323303Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:56:54.9323639Z +
2019-07-17T08:56:54.9323840Z 
2019-07-17T08:56:54.9324063Z The actual stderr differed from the expected stderr.
2019-07-17T08:56:54.9324302Z Actual stderr saved to /tmp/compiletestz0Rj1T/calloc.stderr
2019-07-17T08:56:54.9325053Z To update references, run this command from build directory:
2019-07-17T08:56:54.9325600Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'calloc.rs'
2019-07-17T08:56:54.9326138Z error: 1 errors occurred comparing output.
2019-07-17T08:56:54.9326358Z status: exit code: 1
2019-07-17T08:56:54.9326358Z status: exit code: 1
2019-07-17T08:56:54.9327401Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/calloc.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/calloc.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/calloc.stage-id.aux" "-A" "unused"
2019-07-17T08:56:54.9328391Z ------------------------------------------
2019-07-17T08:56:54.9328667Z 
2019-07-17T08:56:54.9329112Z ------------------------------------------
2019-07-17T08:56:54.9329413Z stderr:
---
2019-07-17T08:56:54.9600734Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:56:54.9600782Z +
2019-07-17T08:56:54.9600825Z 
2019-07-17T08:56:54.9600872Z The actual stderr differed from the expected stderr.
2019-07-17T08:56:54.9600922Z Actual stderr saved to /tmp/compiletestz0Rj1T/calls.stderr
2019-07-17T08:56:54.9600973Z To update references, run this command from build directory:
2019-07-17T08:56:54.9601241Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'calls.rs'
2019-07-17T08:56:54.9601319Z error: 1 errors occurred comparing output.
2019-07-17T08:56:54.9601384Z status: exit code: 1
2019-07-17T08:56:54.9601384Z status: exit code: 1
2019-07-17T08:56:54.9602009Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/calls.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/calls.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/calls.stage-id.aux" "-A" "unused"
2019-07-17T08:56:54.9603243Z ------------------------------------------
2019-07-17T08:56:54.9603290Z 
2019-07-17T08:56:54.9603662Z ------------------------------------------
2019-07-17T08:56:54.9603727Z stderr:
---
2019-07-17T08:56:55.0888374Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:56:55.0888422Z +
2019-07-17T08:56:55.0888449Z 
2019-07-17T08:56:55.0888512Z The actual stderr differed from the expected stderr.
2019-07-17T08:56:55.0888562Z Actual stderr saved to /tmp/compiletestz0Rj1T/cast_fn_ptr.stderr
2019-07-17T08:56:55.0888611Z To update references, run this command from build directory:
2019-07-17T08:56:55.0888884Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'cast_fn_ptr.rs'
2019-07-17T08:56:55.0888961Z error: 1 errors occurred comparing output.
2019-07-17T08:56:55.0889005Z status: exit code: 1
2019-07-17T08:56:55.0889005Z status: exit code: 1
2019-07-17T08:56:55.0889639Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/cast_fn_ptr.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/cast_fn_ptr.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/cast_fn_ptr.stage-id.aux" "-A" "unused"
2019-07-17T08:56:55.0889960Z ------------------------------------------
2019-07-17T08:56:55.0889992Z 
2019-07-17T08:56:55.0890378Z ------------------------------------------
2019-07-17T08:56:55.0890503Z stderr:
---
2019-07-17T08:56:55.1286120Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:56:55.1286168Z +
2019-07-17T08:56:55.1286218Z 
2019-07-17T08:56:55.1286265Z The actual stderr differed from the expected stderr.
2019-07-17T08:56:55.1286523Z Actual stderr saved to /tmp/compiletestz0Rj1T/cast-rfc0401-vtable-kinds.stderr
2019-07-17T08:56:55.1286597Z To update references, run this command from build directory:
2019-07-17T08:56:55.1286862Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'cast-rfc0401-vtable-kinds.rs'
2019-07-17T08:56:55.1286951Z error: 1 errors occurred comparing output.
2019-07-17T08:56:55.1287026Z status: exit code: 1
2019-07-17T08:56:55.1287026Z status: exit code: 1
2019-07-17T08:56:55.1287862Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/cast-rfc0401-vtable-kinds.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/cast-rfc0401-vtable-kinds.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/cast-rfc0401-vtable-kinds.stage-id.aux" "-A" "unused"
2019-07-17T08:56:55.1288133Z ------------------------------------------
2019-07-17T08:56:55.1288162Z 
2019-07-17T08:56:55.1288356Z ------------------------------------------
2019-07-17T08:56:55.1288393Z stderr:
---
2019-07-17T08:56:55.2372536Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:56:55.2372609Z +
2019-07-17T08:56:55.2372639Z 
2019-07-17T08:56:55.2372689Z The actual stderr differed from the expected stderr.
2019-07-17T08:56:55.2372742Z Actual stderr saved to /tmp/compiletestz0Rj1T/cast_fn_ptr_unsafe.stderr
2019-07-17T08:56:55.2372822Z To update references, run this command from build directory:
2019-07-17T08:56:55.2373116Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'cast_fn_ptr_unsafe.rs'
2019-07-17T08:56:55.2373228Z error: 1 errors occurred comparing output.
2019-07-17T08:56:55.2373278Z status: exit code: 1
2019-07-17T08:56:55.2373278Z status: exit code: 1
2019-07-17T08:56:55.2374031Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/cast_fn_ptr_unsafe.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/cast_fn_ptr_unsafe.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/cast_fn_ptr_unsafe.stage-id.aux" "-A" "unused"
2019-07-17T08:56:55.2374372Z ------------------------------------------
2019-07-17T08:56:55.2374409Z 
2019-07-17T08:56:55.2374939Z ------------------------------------------
2019-07-17T08:56:55.2375111Z stderr:
---
2019-07-17T08:56:55.2760878Z -1
2019-07-17T08:56:55.2761095Z -
2019-07-17T08:56:55.2761144Z 
2019-07-17T08:56:55.2761198Z The actual stdout differed from the expected stdout.
2019-07-17T08:56:55.2761265Z Actual stdout saved to /tmp/compiletestz0Rj1T/catch.stdout
2019-07-17T08:56:55.2761400Z error[E0080]: Miri evaluation error: attempted to do invalid arithmetic on pointers that would leak base addresses, e.g., comparing pointers into different allocations
2019-07-17T08:56:55.2761661Z     --> /checkout/src/libcore/intrinsics.rs:1340:19
2019-07-17T08:56:55.2761727Z      |
2019-07-17T08:56:55.2761774Z 1340 |     let diff = if src_usize > dst_usize {
---
2019-07-17T08:56:55.2766107Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:56:55.2766159Z +
2019-07-17T08:56:55.2766187Z 
2019-07-17T08:56:55.2766250Z The actual stderr differed from the expected stderr.
2019-07-17T08:56:55.2766302Z Actual stderr saved to /tmp/compiletestz0Rj1T/catch.stderr
2019-07-17T08:56:55.2766353Z To update references, run this command from build directory:
2019-07-17T08:56:55.2766646Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'catch.rs'
2019-07-17T08:56:55.2766729Z error: 2 errors occurred comparing output.
2019-07-17T08:56:55.2766792Z status: exit code: 1
2019-07-17T08:56:55.2766792Z status: exit code: 1
2019-07-17T08:56:55.2767539Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/catch.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/catch.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/catch.stage-id.aux" "-A" "unused"
2019-07-17T08:56:55.2767933Z ------------------------------------------
2019-07-17T08:56:55.2767969Z 
2019-07-17T08:56:55.2768348Z ------------------------------------------
2019-07-17T08:56:55.2768408Z stderr:
---
2019-07-17T08:56:55.4116198Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:56:55.4116249Z +
2019-07-17T08:56:55.4116277Z 
2019-07-17T08:56:55.4116325Z The actual stderr differed from the expected stderr.
2019-07-17T08:56:55.4116398Z Actual stderr saved to /tmp/compiletestz0Rj1T/char.stderr
2019-07-17T08:56:55.4116449Z To update references, run this command from build directory:
2019-07-17T08:56:55.4116698Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'char.rs'
2019-07-17T08:56:55.4116795Z error: 1 errors occurred comparing output.
2019-07-17T08:56:55.4116841Z status: exit code: 1
2019-07-17T08:56:55.4116841Z status: exit code: 1
2019-07-17T08:56:55.4117701Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/char.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/char.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/char.stage-id.aux" "-A" "unused"
2019-07-17T08:56:55.4118084Z ------------------------------------------
2019-07-17T08:56:55.4118136Z 
2019-07-17T08:56:55.4118509Z ------------------------------------------
2019-07-17T08:56:55.4118552Z stderr:
---
2019-07-17T08:56:55.4151141Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:56:55.4151189Z +
2019-07-17T08:56:55.4151216Z 
2019-07-17T08:56:55.4151278Z The actual stderr differed from the expected stderr.
2019-07-17T08:56:55.4151522Z Actual stderr saved to /tmp/compiletestz0Rj1T/closure-drop.stderr
2019-07-17T08:56:55.4151576Z To update references, run this command from build directory:
2019-07-17T08:56:55.4151843Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'closure-drop.rs'
2019-07-17T08:56:55.4151929Z error: 1 errors occurred comparing output.
2019-07-17T08:56:55.4151976Z status: exit code: 1
2019-07-17T08:56:55.4151976Z status: exit code: 1
2019-07-17T08:56:55.4152731Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/closure-drop.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/closure-drop.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/closure-drop.stage-id.aux" "-A" "unused"
2019-07-17T08:56:55.4153115Z ------------------------------------------
2019-07-17T08:56:55.4153152Z 
2019-07-17T08:56:55.4153388Z ------------------------------------------
2019-07-17T08:56:55.4153454Z stderr:
---
2019-07-17T08:56:55.5927011Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:56:55.5927077Z +
2019-07-17T08:56:55.5927104Z 
2019-07-17T08:56:55.5927148Z The actual stderr differed from the expected stderr.
2019-07-17T08:56:55.5927390Z Actual stderr saved to /tmp/compiletestz0Rj1T/closure-field-ty.stderr
2019-07-17T08:56:55.5927462Z To update references, run this command from build directory:
2019-07-17T08:56:55.5927717Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'closure-field-ty.rs'
2019-07-17T08:56:55.5927897Z error: 1 errors occurred comparing output.
2019-07-17T08:56:55.5927972Z status: exit code: 1
2019-07-17T08:56:55.5927972Z status: exit code: 1
2019-07-17T08:56:55.5928852Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/closure-field-ty.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/closure-field-ty.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/closure-field-ty.stage-id.aux" "-A" "unused"
2019-07-17T08:56:55.5929155Z ------------------------------------------
2019-07-17T08:56:55.5929185Z 
2019-07-17T08:56:55.5929403Z ------------------------------------------
2019-07-17T08:56:55.5929444Z stderr:
---
2019-07-17T08:56:55.6573167Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:56:55.6573214Z +
2019-07-17T08:56:55.6573262Z 
2019-07-17T08:56:55.6573308Z The actual stderr differed from the expected stderr.
2019-07-17T08:56:55.6573360Z Actual stderr saved to /tmp/compiletestz0Rj1T/closures.stderr
2019-07-17T08:56:55.6573412Z To update references, run this command from build directory:
2019-07-17T08:56:55.6573845Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'closures.rs'
2019-07-17T08:56:55.6573950Z error: 1 errors occurred comparing output.
2019-07-17T08:56:55.6574179Z status: exit code: 1
2019-07-17T08:56:55.6574179Z status: exit code: 1
2019-07-17T08:56:55.6574983Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/closures.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/closures.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/closures.stage-id.aux" "-A" "unused"
2019-07-17T08:56:55.6575607Z ------------------------------------------
2019-07-17T08:56:55.6575641Z 
2019-07-17T08:56:55.6575853Z ------------------------------------------
2019-07-17T08:56:55.6575920Z stderr:
---
2019-07-17T08:56:55.7356315Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:56:55.7356366Z +
2019-07-17T08:56:55.7356394Z 
2019-07-17T08:56:55.7356441Z The actual stderr differed from the expected stderr.
2019-07-17T08:56:55.7356724Z Actual stderr saved to /tmp/compiletestz0Rj1T/const-vec-of-fns.stderr
2019-07-17T08:56:55.7356894Z To update references, run this command from build directory:
2019-07-17T08:56:55.7357207Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'const-vec-of-fns.rs'
2019-07-17T08:56:55.7357321Z error: 1 errors occurred comparing output.
2019-07-17T08:56:55.7357368Z status: exit code: 1
2019-07-17T08:56:55.7357368Z status: exit code: 1
2019-07-17T08:56:55.7358049Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/const-vec-of-fns.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/const-vec-of-fns.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/const-vec-of-fns.stage-id.aux" "-A" "unused"
2019-07-17T08:56:55.7358522Z ------------------------------------------
2019-07-17T08:56:55.7358576Z 
2019-07-17T08:56:55.7358794Z ------------------------------------------
2019-07-17T08:56:55.7358838Z stderr:
---
2019-07-17T08:56:55.8254121Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:56:55.8254176Z +
2019-07-17T08:56:55.8254228Z 
2019-07-17T08:56:55.8254279Z The actual stderr differed from the expected stderr.
2019-07-17T08:56:55.8254332Z Actual stderr saved to /tmp/compiletestz0Rj1T/constants.stderr
2019-07-17T08:56:55.8254616Z To update references, run this command from build directory:
2019-07-17T08:56:55.8255276Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'constants.rs'
2019-07-17T08:56:55.8255382Z error: 1 errors occurred comparing output.
2019-07-17T08:56:55.8255451Z status: exit code: 1
2019-07-17T08:56:55.8255451Z status: exit code: 1
2019-07-17T08:56:55.8256130Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/constants.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/constants.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/constants.stage-id.aux" "-A" "unused"
2019-07-17T08:56:55.8256456Z ------------------------------------------
2019-07-17T08:56:55.8256489Z 
2019-07-17T08:56:55.8256700Z ------------------------------------------
2019-07-17T08:56:55.8256765Z stderr:
---
2019-07-17T08:56:55.9610352Z +
2019-07-17T08:56:55.9610720Z thread '[ui] run-pass/drop_empty_slice.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
2019-07-17T08:56:55.9610785Z 
2019-07-17T08:56:55.9610996Z The actual stderr differed from the expected stderr.
2019-07-17T08:56:55.9611063Z Actual stderr saved to /tmp/compiletestz0Rj1T/drop_empty_slice.stderr
2019-07-17T08:56:55.9611143Z To update references, run this command from build directory:
2019-07-17T08:56:55.9611466Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'drop_empty_slice.rs'
2019-07-17T08:56:55.9611550Z error: 1 errors occurred comparing output.
2019-07-17T08:56:55.9611612Z status: exit code: 1
2019-07-17T08:56:55.9611612Z status: exit code: 1
2019-07-17T08:56:55.9612260Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/drop_empty_slice.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/drop_empty_slice.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/drop_empty_slice.stage-id.aux" "-A" "unused"
2019-07-17T08:56:55.9612597Z ------------------------------------------
2019-07-17T08:56:55.9612727Z 
2019-07-17T08:56:55.9613004Z ------------------------------------------
2019-07-17T08:56:55.9613052Z stderr:
---
2019-07-17T08:56:56.0525691Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:56:56.0525753Z +
2019-07-17T08:56:56.0525788Z 
2019-07-17T08:56:56.0525853Z The actual stderr differed from the expected stderr.
2019-07-17T08:56:56.0526141Z Actual stderr saved to /tmp/compiletestz0Rj1T/deriving-associated-types.stderr
2019-07-17T08:56:56.0526197Z To update references, run this command from build directory:
2019-07-17T08:56:56.0526471Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'deriving-associated-types.rs'
2019-07-17T08:56:56.0526551Z error: 1 errors occurred comparing output.
2019-07-17T08:56:56.0526598Z status: exit code: 1
2019-07-17T08:56:56.0526598Z status: exit code: 1
2019-07-17T08:56:56.0527292Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/deriving-associated-types.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/deriving-associated-types.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/deriving-associated-types.stage-id.aux" "-A" "unused"
2019-07-17T08:56:56.0527717Z ------------------------------------------
2019-07-17T08:56:56.0527752Z 
2019-07-17T08:56:56.0528069Z ------------------------------------------
2019-07-17T08:56:56.0528130Z stderr:
---
2019-07-17T08:56:56.1925783Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:56:56.1925863Z +
2019-07-17T08:56:56.1925891Z 
2019-07-17T08:56:56.1925944Z The actual stderr differed from the expected stderr.
2019-07-17T08:56:56.1926230Z Actual stderr saved to /tmp/compiletestz0Rj1T/dst-irrefutable-bind.stderr
2019-07-17T08:56:56.1926307Z To update references, run this command from build directory:
2019-07-17T08:56:56.1926569Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'dst-irrefutable-bind.rs'
2019-07-17T08:56:56.1926669Z error: 1 errors occurred comparing output.
2019-07-17T08:56:56.1926716Z status: exit code: 1
2019-07-17T08:56:56.1926716Z status: exit code: 1
2019-07-17T08:56:56.1927391Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/dst-irrefutable-bind.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/dst-irrefutable-bind.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/dst-irrefutable-bind.stage-id.aux" "-A" "unused"
2019-07-17T08:56:56.1927823Z ------------------------------------------
2019-07-17T08:56:56.1927858Z 
2019-07-17T08:56:56.1928093Z ------------------------------------------
2019-07-17T08:56:56.1928139Z stderr:
---
2019-07-17T08:56:56.2202273Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:56:56.2202322Z +
2019-07-17T08:56:56.2202350Z 
2019-07-17T08:56:56.2202397Z The actual stderr differed from the expected stderr.
2019-07-17T08:56:56.2202664Z Actual stderr saved to /tmp/compiletestz0Rj1T/dst-field-align.stderr
2019-07-17T08:56:56.2202719Z To update references, run this command from build directory:
2019-07-17T08:56:56.2202970Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'dst-field-align.rs'
2019-07-17T08:56:56.2203068Z error: 1 errors occurred comparing output.
2019-07-17T08:56:56.2203114Z status: exit code: 1
2019-07-17T08:56:56.2203114Z status: exit code: 1
2019-07-17T08:56:56.2204118Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/dst-field-align.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/dst-field-align.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/dst-field-align.stage-id.aux" "-A" "unused"
2019-07-17T08:56:56.2204534Z ------------------------------------------
2019-07-17T08:56:56.2204585Z 
2019-07-17T08:56:56.2205422Z ------------------------------------------
2019-07-17T08:56:56.2205476Z stderr:
---
2019-07-17T08:56:56.4453753Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:56:56.4453808Z +
2019-07-17T08:56:56.4453863Z 
2019-07-17T08:56:56.4453912Z The actual stderr differed from the expected stderr.
2019-07-17T08:56:56.4454177Z Actual stderr saved to /tmp/compiletestz0Rj1T/dst-struct-sole.stderr
2019-07-17T08:56:56.4454248Z To update references, run this command from build directory:
2019-07-17T08:56:56.4454510Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'dst-struct-sole.rs'
2019-07-17T08:56:56.4454589Z error: 1 errors occurred comparing output.
2019-07-17T08:56:56.4454651Z status: exit code: 1
2019-07-17T08:56:56.4454651Z status: exit code: 1
2019-07-17T08:56:56.4455841Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/dst-struct-sole.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/dst-struct-sole.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/dst-struct-sole.stage-id.aux" "-A" "unused"
2019-07-17T08:56:56.4456343Z ------------------------------------------
2019-07-17T08:56:56.4456378Z 
2019-07-17T08:56:56.4456609Z ------------------------------------------
2019-07-17T08:56:56.4456655Z stderr:
---
2019-07-17T08:56:56.4550556Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:56:56.4550604Z +
2019-07-17T08:56:56.4550632Z 
2019-07-17T08:56:56.4550679Z The actual stderr differed from the expected stderr.
2019-07-17T08:56:56.4550944Z Actual stderr saved to /tmp/compiletestz0Rj1T/dst-raw.stderr
2019-07-17T08:56:56.4550997Z To update references, run this command from build directory:
2019-07-17T08:56:56.4551242Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'dst-raw.rs'
2019-07-17T08:56:56.4551343Z error: 1 errors occurred comparing output.
2019-07-17T08:56:56.4551388Z status: exit code: 1
2019-07-17T08:56:56.4551388Z status: exit code: 1
2019-07-17T08:56:56.4552039Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/dst-raw.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/dst-raw.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/dst-raw.stage-id.aux" "-A" "unused"
2019-07-17T08:56:56.4552469Z ------------------------------------------
2019-07-17T08:56:56.4552504Z 
2019-07-17T08:56:56.4552719Z ------------------------------------------
2019-07-17T08:56:56.4552764Z stderr:
---
2019-07-17T08:56:56.5854325Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:56:56.5854621Z +
2019-07-17T08:56:56.5855246Z 
2019-07-17T08:56:56.5855482Z The actual stderr differed from the expected stderr.
2019-07-17T08:56:56.5856024Z Actual stderr saved to /tmp/compiletestz0Rj1T/enum-nullable-const-null-with-fields.stderr
2019-07-17T08:56:56.5856355Z To update references, run this command from build directory:
2019-07-17T08:56:56.5856901Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'enum-nullable-const-null-with-fields.rs'
2019-07-17T08:56:56.5857579Z error: 1 errors occurred comparing output.
2019-07-17T08:56:56.5857793Z status: exit code: 1
2019-07-17T08:56:56.5857793Z status: exit code: 1
2019-07-17T08:56:56.5858889Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/enum-nullable-const-null-with-fields.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/enum-nullable-const-null-with-fields.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/enum-nullable-const-null-with-fields.stage-id.aux" "-A" "unused"
2019-07-17T08:56:56.5859721Z ------------------------------------------
2019-07-17T08:56:56.5859988Z 
2019-07-17T08:56:56.5860469Z ------------------------------------------
2019-07-17T08:56:56.5860774Z stderr:
---
2019-07-17T08:56:56.7411659Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:56:56.7411728Z +
2019-07-17T08:56:56.7411754Z 
2019-07-17T08:56:56.7411799Z The actual stderr differed from the expected stderr.
2019-07-17T08:56:56.7412062Z Actual stderr saved to /tmp/compiletestz0Rj1T/dst-struct.stderr
2019-07-17T08:56:56.7412124Z To update references, run this command from build directory:
2019-07-17T08:56:56.7412419Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'dst-struct.rs'
2019-07-17T08:56:56.7412600Z error: 1 errors occurred comparing output.
2019-07-17T08:56:56.7412645Z status: exit code: 1
2019-07-17T08:56:56.7412645Z status: exit code: 1
2019-07-17T08:56:56.7413307Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/dst-struct.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/dst-struct.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/dst-struct.stage-id.aux" "-A" "unused"
2019-07-17T08:56:56.7413623Z ------------------------------------------
2019-07-17T08:56:56.7413673Z 
2019-07-17T08:56:56.7413883Z ------------------------------------------
2019-07-17T08:56:56.7413929Z stderr:
---
2019-07-17T08:56:56.7572140Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:56:56.7572188Z +
2019-07-17T08:56:56.7572214Z 
2019-07-17T08:56:56.7572278Z The actual stderr differed from the expected stderr.
2019-07-17T08:56:56.7572330Z Actual stderr saved to /tmp/compiletestz0Rj1T/enums.stderr
2019-07-17T08:56:56.7572389Z To update references, run this command from build directory:
2019-07-17T08:56:56.7572778Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'enums.rs'
2019-07-17T08:56:56.7572861Z error: 1 errors occurred comparing output.
2019-07-17T08:56:56.7572906Z status: exit code: 1
2019-07-17T08:56:56.7572906Z status: exit code: 1
2019-07-17T08:56:56.7573731Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/enums.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/enums.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/enums.stage-id.aux" "-A" "unused"
2019-07-17T08:56:56.7574217Z ------------------------------------------
2019-07-17T08:56:56.7574250Z 
2019-07-17T08:56:56.7574462Z ------------------------------------------
2019-07-17T08:56:56.7574507Z stderr:
---
2019-07-17T08:56:56.8932753Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:56:56.8932823Z +
2019-07-17T08:56:56.8932853Z 
2019-07-17T08:56:56.8932900Z The actual stderr differed from the expected stderr.
2019-07-17T08:56:56.8932960Z Actual stderr saved to /tmp/compiletestz0Rj1T/exit.stderr
2019-07-17T08:56:56.8933032Z To update references, run this command from build directory:
2019-07-17T08:56:56.8933424Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'exit.rs'
2019-07-17T08:56:56.8933730Z error: 1 errors occurred comparing output.
2019-07-17T08:56:56.8933795Z status: exit code: 1
2019-07-17T08:56:56.8933795Z status: exit code: 1
2019-07-17T08:56:56.8934403Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/exit.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/exit.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/exit.stage-id.aux" "-A" "unused"
2019-07-17T08:56:56.8935258Z ------------------------------------------
2019-07-17T08:56:56.8935298Z 
2019-07-17T08:56:56.8935580Z ------------------------------------------
2019-07-17T08:56:56.8935628Z stderr:
---
2019-07-17T08:56:56.9118618Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:56:56.9118666Z +
2019-07-17T08:56:56.9118760Z 
2019-07-17T08:56:56.9118805Z The actual stderr differed from the expected stderr.
2019-07-17T08:56:56.9118929Z Actual stderr saved to /tmp/compiletestz0Rj1T/env.stderr
2019-07-17T08:56:56.9118977Z To update references, run this command from build directory:
2019-07-17T08:56:56.9119363Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'env.rs'
2019-07-17T08:56:56.9119463Z error: 1 errors occurred comparing output.
2019-07-17T08:56:56.9119506Z status: exit code: 1
2019-07-17T08:56:56.9119506Z status: exit code: 1
2019-07-17T08:56:56.9120126Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/env.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/env.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/env.stage-id.aux" "-A" "unused"
2019-07-17T08:56:56.9120430Z ------------------------------------------
2019-07-17T08:56:56.9120462Z 
2019-07-17T08:56:56.9120675Z ------------------------------------------
2019-07-17T08:56:56.9120728Z stderr:
---
2019-07-17T08:56:57.0312997Z +
2019-07-17T08:56:57.0323269Z 
2019-07-17T08:56:57.0327690Z thread '[ui] run-pass/extern_types.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
2019-07-17T08:56:57.0328225Z The actual stderr differed from the expected stderr.
2019-07-17T08:56:57.0328589Z Actual stderr saved to /tmp/compiletestz0Rj1T/extern_types.stderr
2019-07-17T08:56:57.0328736Z To update references, run this command from build directory:
2019-07-17T08:56:57.0329196Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'extern_types.rs'
2019-07-17T08:56:57.0329511Z error: 1 errors occurred comparing output.
2019-07-17T08:56:57.0329663Z status: exit code: 1
2019-07-17T08:56:57.0329663Z status: exit code: 1
2019-07-17T08:56:57.0330633Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/extern_types.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/extern_types.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/extern_types.stage-id.aux" "-A" "unused"
2019-07-17T08:56:57.0331413Z ------------------------------------------
2019-07-17T08:56:57.0331572Z 
2019-07-17T08:56:57.0331936Z ------------------------------------------
2019-07-17T08:56:57.0332114Z stderr:
---
2019-07-17T08:56:57.0958067Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:56:57.0958361Z +
2019-07-17T08:56:57.0958567Z 
2019-07-17T08:56:57.0958967Z The actual stderr differed from the expected stderr.
2019-07-17T08:56:57.0959327Z Actual stderr saved to /tmp/compiletestz0Rj1T/float_fast_math.stderr
2019-07-17T08:56:57.0959529Z To update references, run this command from build directory:
2019-07-17T08:56:57.0960242Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'float_fast_math.rs'
2019-07-17T08:56:57.0960739Z error: 1 errors occurred comparing output.
2019-07-17T08:56:57.0960970Z status: exit code: 1
2019-07-17T08:56:57.0960970Z status: exit code: 1
2019-07-17T08:56:57.0962006Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/float_fast_math.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/float_fast_math.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/float_fast_math.stage-id.aux" "-A" "unused"
2019-07-17T08:56:57.0962851Z ------------------------------------------
2019-07-17T08:56:57.0963133Z 
2019-07-17T08:56:57.0963752Z ------------------------------------------
2019-07-17T08:56:57.0964026Z stderr:
---
2019-07-17T08:56:57.2638443Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:56:57.2638494Z +
2019-07-17T08:56:57.2638538Z 
2019-07-17T08:56:57.2638587Z The actual stderr differed from the expected stderr.
2019-07-17T08:56:57.2638859Z Actual stderr saved to /tmp/compiletestz0Rj1T/foreign-fn-linkname.stderr
2019-07-17T08:56:57.2638915Z To update references, run this command from build directory:
2019-07-17T08:56:57.2639216Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'foreign-fn-linkname.rs'
2019-07-17T08:56:57.2639300Z error: 1 errors occurred comparing output.
2019-07-17T08:56:57.2639364Z status: exit code: 1
2019-07-17T08:56:57.2639364Z status: exit code: 1
2019-07-17T08:56:57.2640047Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/foreign-fn-linkname.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/foreign-fn-linkname.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/foreign-fn-linkname.stage-id.aux" "-A" "unused"
2019-07-17T08:56:57.2640397Z ------------------------------------------
2019-07-17T08:56:57.2640432Z 
2019-07-17T08:56:57.2640788Z ------------------------------------------
2019-07-17T08:56:57.2640853Z stderr:
---
2019-07-17T08:56:57.3198573Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:56:57.3198646Z +
2019-07-17T08:56:57.3198676Z 
2019-07-17T08:56:57.3198722Z The actual stderr differed from the expected stderr.
2019-07-17T08:56:57.3198773Z Actual stderr saved to /tmp/compiletestz0Rj1T/floats.stderr
2019-07-17T08:56:57.3198843Z To update references, run this command from build directory:
2019-07-17T08:56:57.3199104Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'floats.rs'
2019-07-17T08:56:57.3199183Z error: 1 errors occurred comparing output.
2019-07-17T08:56:57.3199247Z status: exit code: 1
2019-07-17T08:56:57.3199247Z status: exit code: 1
2019-07-17T08:56:57.3199878Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/floats.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/floats.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/floats.stage-id.aux" "-A" "unused"
2019-07-17T08:56:57.3200203Z ------------------------------------------
2019-07-17T08:56:57.3200236Z 
2019-07-17T08:56:57.3200468Z ------------------------------------------
2019-07-17T08:56:57.3200513Z stderr:
---
2019-07-17T08:56:57.4174909Z -hello00000
2019-07-17T08:56:57.4178624Z -
2019-07-17T08:56:57.4185673Z 
2019-07-17T08:56:57.4185772Z The actual stdout differed from the expected stdout.
2019-07-17T08:56:57.4190865Z Actual stdout saved to /tmp/compiletestz0Rj1T/format.stdout
2019-07-17T08:56:57.4214135Z error[E0080]: Miri evaluation error: attempted to do invalid arithmetic on pointers that would leak base addresses, e.g., comparing pointers into different allocations
2019-07-17T08:56:57.4215540Z     --> /checkout/src/libcore/intrinsics.rs:1340:19
2019-07-17T08:56:57.4215608Z      |
2019-07-17T08:56:57.4215661Z 1340 |     let diff = if src_usize > dst_usize {
---
2019-07-17T08:56:57.4219641Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:56:57.4219690Z +
2019-07-17T08:56:57.4219735Z 
2019-07-17T08:56:57.4219790Z The actual stderr differed from the expected stderr.
2019-07-17T08:56:57.4219842Z Actual stderr saved to /tmp/compiletestz0Rj1T/format.stderr
2019-07-17T08:56:57.4219899Z To update references, run this command from build directory:
2019-07-17T08:56:57.4220184Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'format.rs'
2019-07-17T08:56:57.4220264Z error: 2 errors occurred comparing output.
2019-07-17T08:56:57.4220328Z status: exit code: 1
2019-07-17T08:56:57.4220328Z status: exit code: 1
2019-07-17T08:56:57.4221142Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/format.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/format.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/format.stage-id.aux" "-A" "unused"
2019-07-17T08:56:57.4221470Z ------------------------------------------
2019-07-17T08:56:57.4221504Z 
2019-07-17T08:56:57.4221724Z ------------------------------------------
2019-07-17T08:56:57.4221787Z stderr:
---
2019-07-17T08:56:57.4686849Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:56:57.4686905Z +
2019-07-17T08:56:57.4686946Z 
2019-07-17T08:56:57.4686995Z The actual stderr differed from the expected stderr.
2019-07-17T08:56:57.4687079Z Actual stderr saved to /tmp/compiletestz0Rj1T/from_utf8.stderr
2019-07-17T08:56:57.4687131Z To update references, run this command from build directory:
2019-07-17T08:56:57.4687427Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'from_utf8.rs'
2019-07-17T08:56:57.4687532Z error: 1 errors occurred comparing output.
2019-07-17T08:56:57.4687580Z status: exit code: 1
2019-07-17T08:56:57.4687580Z status: exit code: 1
2019-07-17T08:56:57.4689610Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/from_utf8.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/from_utf8.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/from_utf8.stage-id.aux" "-A" "unused"
2019-07-17T08:56:57.4690139Z ------------------------------------------
2019-07-17T08:56:57.4690184Z 
2019-07-17T08:56:57.4690403Z ------------------------------------------
2019-07-17T08:56:57.4690448Z stderr:
---
2019-07-17T08:56:57.6425978Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:56:57.6426030Z +
2019-07-17T08:56:57.6426059Z 
2019-07-17T08:56:57.6426125Z The actual stderr differed from the expected stderr.
2019-07-17T08:56:57.6426178Z Actual stderr saved to /tmp/compiletestz0Rj1T/function_pointers.stderr
2019-07-17T08:56:57.6426231Z To update references, run this command from build directory:
2019-07-17T08:56:57.6426521Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'function_pointers.rs'
2019-07-17T08:56:57.6426604Z error: 1 errors occurred comparing output.
2019-07-17T08:56:57.6426649Z status: exit code: 1
2019-07-17T08:56:57.6426649Z status: exit code: 1
2019-07-17T08:56:57.6427320Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/function_pointers.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/function_pointers.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/function_pointers.stage-id.aux" "-A" "unused"
2019-07-17T08:56:57.6427647Z ------------------------------------------
2019-07-17T08:56:57.6427681Z 
2019-07-17T08:56:57.6427897Z ------------------------------------------
2019-07-17T08:56:57.6427952Z stderr:
---
2019-07-17T08:56:57.7007340Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:56:57.7007403Z +
2019-07-17T08:56:57.7007432Z 
2019-07-17T08:56:57.7007478Z The actual stderr differed from the expected stderr.
2019-07-17T08:56:57.7007550Z Actual stderr saved to /tmp/compiletestz0Rj1T/generator.stderr
2019-07-17T08:56:57.7007602Z To update references, run this command from build directory:
2019-07-17T08:56:57.7007867Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'generator.rs'
2019-07-17T08:56:57.7008131Z error: 1 errors occurred comparing output.
2019-07-17T08:56:57.7008175Z status: exit code: 1
2019-07-17T08:56:57.7008175Z status: exit code: 1
2019-07-17T08:56:57.7008812Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/generator.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/generator.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/generator.stage-id.aux" "-A" "unused"
2019-07-17T08:56:57.7009128Z ------------------------------------------
2019-07-17T08:56:57.7009162Z 
2019-07-17T08:56:57.7009365Z ------------------------------------------
2019-07-17T08:56:57.7009408Z stderr:
---
2019-07-17T08:56:57.8281326Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:56:57.8281395Z +
2019-07-17T08:56:57.8281425Z 
2019-07-17T08:56:57.8281472Z The actual stderr differed from the expected stderr.
2019-07-17T08:56:57.8281523Z Actual stderr saved to /tmp/compiletestz0Rj1T/heap.stderr
2019-07-17T08:56:57.8281595Z To update references, run this command from build directory:
2019-07-17T08:56:57.8281865Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'heap.rs'
2019-07-17T08:56:57.8281948Z error: 1 errors occurred comparing output.
2019-07-17T08:56:57.8282014Z status: exit code: 1
2019-07-17T08:56:57.8282014Z status: exit code: 1
2019-07-17T08:56:57.8282719Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/heap.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/heap.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/heap.stage-id.aux" "-A" "unused"
2019-07-17T08:56:57.8283065Z ------------------------------------------
2019-07-17T08:56:57.8283100Z 
2019-07-17T08:56:57.8283350Z ------------------------------------------
2019-07-17T08:56:57.8283398Z stderr:
---
2019-07-17T08:56:57.8380572Z -Hello, world!
2019-07-17T08:56:57.8380922Z -
2019-07-17T08:56:57.8380952Z 
2019-07-17T08:56:57.8381000Z The actual stdout differed from the expected stdout.
2019-07-17T08:56:57.8381068Z Actual stdout saved to /tmp/compiletestz0Rj1T/hello.stdout
2019-07-17T08:56:57.8381212Z error[E0080]: Miri evaluation error: attempted to do invalid arithmetic on pointers that would leak base addresses, e.g., comparing pointers into different allocations
2019-07-17T08:56:57.8381509Z     --> /checkout/src/libcore/intrinsics.rs:1340:19
2019-07-17T08:56:57.8381562Z      |
2019-07-17T08:56:57.8381608Z 1340 |     let diff = if src_usize > dst_usize {
---
2019-07-17T08:56:57.8386753Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:56:57.8386819Z +
2019-07-17T08:56:57.8386847Z 
2019-07-17T08:56:57.8386892Z The actual stderr differed from the expected stderr.
2019-07-17T08:56:57.8386952Z Actual stderr saved to /tmp/compiletestz0Rj1T/hello.stderr
2019-07-17T08:56:57.8387029Z To update references, run this command from build directory:
2019-07-17T08:56:57.8387281Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'hello.rs'
2019-07-17T08:56:57.8387375Z error: 2 errors occurred comparing output.
2019-07-17T08:56:57.8387421Z status: exit code: 1
2019-07-17T08:56:57.8387421Z status: exit code: 1
2019-07-17T08:56:57.8388045Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/hello.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/hello.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/hello.stage-id.aux" "-A" "unused"
2019-07-17T08:56:57.8388360Z ------------------------------------------
2019-07-17T08:56:57.8388502Z 
2019-07-17T08:56:57.8388772Z ------------------------------------------
2019-07-17T08:56:57.8388828Z stderr:
---
2019-07-17T08:56:58.5045700Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:56:58.5045749Z +
2019-07-17T08:56:58.5045775Z 
2019-07-17T08:56:58.5045831Z The actual stderr differed from the expected stderr.
2019-07-17T08:56:58.5046089Z Actual stderr saved to /tmp/compiletestz0Rj1T/integer-ops.stderr
2019-07-17T08:56:58.5046154Z To update references, run this command from build directory:
2019-07-17T08:56:58.5046403Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'integer-ops.rs'
2019-07-17T08:56:58.5046499Z error: 1 errors occurred comparing output.
2019-07-17T08:56:58.5046544Z status: exit code: 1
2019-07-17T08:56:58.5046544Z status: exit code: 1
2019-07-17T08:56:58.5047202Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/integer-ops.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/integer-ops.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/integer-ops.stage-id.aux" "-A" "unused"
2019-07-17T08:56:58.5047605Z ------------------------------------------
2019-07-17T08:56:58.5047671Z 
2019-07-17T08:56:58.5047925Z ------------------------------------------
2019-07-17T08:56:58.5047971Z stderr:
---
2019-07-17T08:56:59.0488891Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:56:59.0488953Z +
2019-07-17T08:56:59.0488980Z 
2019-07-17T08:56:59.0489025Z The actual stderr differed from the expected stderr.
2019-07-17T08:56:59.0489298Z Actual stderr saved to /tmp/compiletestz0Rj1T/intrinsics-math.stderr
2019-07-17T08:56:59.0489352Z To update references, run this command from build directory:
2019-07-17T08:56:59.0489596Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'intrinsics-math.rs'
2019-07-17T08:56:59.0489690Z error: 1 errors occurred comparing output.
2019-07-17T08:56:59.0489734Z status: exit code: 1
2019-07-17T08:56:59.0489734Z status: exit code: 1
2019-07-17T08:56:59.0490487Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/intrinsics-math.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/intrinsics-math.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/intrinsics-math.stage-id.aux" "-A" "unused"
2019-07-17T08:56:59.0490831Z ------------------------------------------
2019-07-17T08:56:59.0490881Z 
2019-07-17T08:56:59.0491086Z ------------------------------------------
2019-07-17T08:56:59.0491130Z stderr:
---
2019-07-17T08:56:59.2491546Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:56:59.2491631Z +
2019-07-17T08:56:59.2491659Z 
2019-07-17T08:56:59.2491705Z The actual stderr differed from the expected stderr.
2019-07-17T08:56:59.2491773Z Actual stderr saved to /tmp/compiletestz0Rj1T/intrinsics.stderr
2019-07-17T08:56:59.2491824Z To update references, run this command from build directory:
2019-07-17T08:56:59.2492093Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'intrinsics.rs'
2019-07-17T08:56:59.2492191Z error: 1 errors occurred comparing output.
2019-07-17T08:56:59.2492238Z status: exit code: 1
2019-07-17T08:56:59.2492238Z status: exit code: 1
2019-07-17T08:56:59.2492974Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/intrinsics.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/intrinsics.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/intrinsics.stage-id.aux" "-A" "unused"
2019-07-17T08:56:59.2493337Z ------------------------------------------
2019-07-17T08:56:59.2493390Z 
2019-07-17T08:56:59.2493603Z ------------------------------------------
2019-07-17T08:56:59.2493649Z stderr:
---
2019-07-17T08:56:59.4854328Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:56:59.4854525Z +
2019-07-17T08:56:59.4854653Z 
2019-07-17T08:56:59.4854826Z The actual stderr differed from the expected stderr.
2019-07-17T08:56:59.4855517Z Actual stderr saved to /tmp/compiletestz0Rj1T/ints.stderr
2019-07-17T08:56:59.4855691Z To update references, run this command from build directory:
2019-07-17T08:56:59.4856136Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'ints.rs'
2019-07-17T08:56:59.4856469Z error: 1 errors occurred comparing output.
2019-07-17T08:56:59.4856613Z status: exit code: 1
2019-07-17T08:56:59.4856613Z status: exit code: 1
2019-07-17T08:56:59.4857503Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/ints.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/ints.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/ints.stage-id.aux" "-A" "unused"
2019-07-17T08:56:59.4858166Z ------------------------------------------
2019-07-17T08:56:59.4858348Z 
2019-07-17T08:56:59.4858869Z ------------------------------------------
2019-07-17T08:56:59.4859052Z stderr:
---
2019-07-17T08:56:59.6212823Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:56:59.6212901Z +
2019-07-17T08:56:59.6212931Z 
2019-07-17T08:56:59.6212979Z The actual stderr differed from the expected stderr.
2019-07-17T08:56:59.6213260Z Actual stderr saved to /tmp/compiletestz0Rj1T/issue-15063.stderr
2019-07-17T08:56:59.6213332Z To update references, run this command from build directory:
2019-07-17T08:56:59.6213604Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'issue-15063.rs'
2019-07-17T08:56:59.6213688Z error: 1 errors occurred comparing output.
2019-07-17T08:56:59.6213755Z status: exit code: 1
2019-07-17T08:56:59.6213755Z status: exit code: 1
2019-07-17T08:56:59.6214719Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-15063.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/issue-15063.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/issue-15063.stage-id.aux" "-A" "unused"
2019-07-17T08:56:59.6215605Z ------------------------------------------
2019-07-17T08:56:59.6215647Z 
2019-07-17T08:56:59.6215904Z ------------------------------------------
2019-07-17T08:56:59.6215952Z stderr:
---
2019-07-17T08:56:59.8170897Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:56:59.8171093Z +
2019-07-17T08:56:59.8171217Z 
2019-07-17T08:56:59.8171356Z The actual stderr differed from the expected stderr.
2019-07-17T08:56:59.8171790Z Actual stderr saved to /tmp/compiletestz0Rj1T/issue-15080.stderr
2019-07-17T08:56:59.8171978Z To update references, run this command from build directory:
2019-07-17T08:56:59.8172364Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'issue-15080.rs'
2019-07-17T08:56:59.8172731Z error: 1 errors occurred comparing output.
2019-07-17T08:56:59.8173052Z status: exit code: 1
2019-07-17T08:56:59.8173052Z status: exit code: 1
2019-07-17T08:56:59.8173885Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-15080.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/issue-15080.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/issue-15080.stage-id.aux" "-A" "unused"
2019-07-17T08:56:59.8174549Z ------------------------------------------
2019-07-17T08:56:59.8178415Z 
2019-07-17T08:56:59.8179172Z ------------------------------------------
2019-07-17T08:56:59.8179373Z stderr:
---
2019-07-17T08:57:00.0071869Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:00.0071934Z +
2019-07-17T08:57:00.0071964Z 
2019-07-17T08:57:00.0072012Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:00.0072275Z Actual stderr saved to /tmp/compiletestz0Rj1T/issue-15523-big.stderr
2019-07-17T08:57:00.0072348Z To update references, run this command from build directory:
2019-07-17T08:57:00.0072625Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'issue-15523-big.rs'
2019-07-17T08:57:00.0072815Z error: 1 errors occurred comparing output.
2019-07-17T08:57:00.0072869Z status: exit code: 1
2019-07-17T08:57:00.0072869Z status: exit code: 1
2019-07-17T08:57:00.0073722Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-15523-big.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/issue-15523-big.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/issue-15523-big.stage-id.aux" "-A" "unused"
2019-07-17T08:57:00.0074039Z ------------------------------------------
2019-07-17T08:57:00.0074071Z 
2019-07-17T08:57:00.0074294Z ------------------------------------------
2019-07-17T08:57:00.0074339Z stderr:
---
2019-07-17T08:57:00.1764748Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:00.1764912Z +
2019-07-17T08:57:00.1764942Z 
2019-07-17T08:57:00.1765278Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:00.1765607Z Actual stderr saved to /tmp/compiletestz0Rj1T/issue-17877.stderr
2019-07-17T08:57:00.1765680Z To update references, run this command from build directory:
2019-07-17T08:57:00.1765936Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'issue-17877.rs'
2019-07-17T08:57:00.1766035Z error: 1 errors occurred comparing output.
2019-07-17T08:57:00.1766243Z status: exit code: 1
2019-07-17T08:57:00.1766243Z status: exit code: 1
2019-07-17T08:57:00.1766942Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-17877.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/issue-17877.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/issue-17877.stage-id.aux" "-A" "unused"
2019-07-17T08:57:00.1767276Z ------------------------------------------
2019-07-17T08:57:00.1767308Z 
2019-07-17T08:57:00.1767541Z ------------------------------------------
2019-07-17T08:57:00.1767587Z stderr:
---
2019-07-17T08:57:00.3407307Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:00.3407355Z +
2019-07-17T08:57:00.3407406Z 
2019-07-17T08:57:00.3407453Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:00.3407696Z Actual stderr saved to /tmp/compiletestz0Rj1T/issue-20575.stderr
2019-07-17T08:57:00.3407769Z To update references, run this command from build directory:
2019-07-17T08:57:00.3408024Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'issue-20575.rs'
2019-07-17T08:57:00.3408409Z error: 1 errors occurred comparing output.
2019-07-17T08:57:00.3408496Z status: exit code: 1
2019-07-17T08:57:00.3408496Z status: exit code: 1
2019-07-17T08:57:00.3409157Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-20575.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/issue-20575.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/issue-20575.stage-id.aux" "-A" "unused"
2019-07-17T08:57:00.3409467Z ------------------------------------------
2019-07-17T08:57:00.3409499Z 
2019-07-17T08:57:00.3409717Z ------------------------------------------
2019-07-17T08:57:00.3409761Z stderr:
---
2019-07-17T08:57:00.5296548Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:00.5296767Z +
2019-07-17T08:57:00.5296894Z 
2019-07-17T08:57:00.5297039Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:00.5297654Z Actual stderr saved to /tmp/compiletestz0Rj1T/issue-23261.stderr
2019-07-17T08:57:00.5297911Z To update references, run this command from build directory:
2019-07-17T08:57:00.5298394Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'issue-23261.rs'
2019-07-17T08:57:00.5298870Z error: 1 errors occurred comparing output.
2019-07-17T08:57:00.5299009Z status: exit code: 1
2019-07-17T08:57:00.5299009Z status: exit code: 1
2019-07-17T08:57:00.5299820Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-23261.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/issue-23261.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/issue-23261.stage-id.aux" "-A" "unused"
2019-07-17T08:57:00.5300725Z ------------------------------------------
2019-07-17T08:57:00.5301025Z 
2019-07-17T08:57:00.5301390Z ------------------------------------------
2019-07-17T08:57:00.5301737Z stderr:
---
2019-07-17T08:57:00.7166515Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:00.7166565Z +
2019-07-17T08:57:00.7166595Z 
2019-07-17T08:57:00.7166662Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:00.7166934Z Actual stderr saved to /tmp/compiletestz0Rj1T/issue-26709.stderr
2019-07-17T08:57:00.7167143Z To update references, run this command from build directory:
2019-07-17T08:57:00.7167489Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'issue-26709.rs'
2019-07-17T08:57:00.7167585Z error: 1 errors occurred comparing output.
2019-07-17T08:57:00.7167650Z status: exit code: 1
2019-07-17T08:57:00.7167650Z status: exit code: 1
2019-07-17T08:57:00.7168461Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-26709.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/issue-26709.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/issue-26709.stage-id.aux" "-A" "unused"
2019-07-17T08:57:00.7168764Z ------------------------------------------
2019-07-17T08:57:00.7230830Z 
2019-07-17T08:57:00.7231349Z ------------------------------------------
2019-07-17T08:57:00.7231420Z stderr:
---
2019-07-17T08:57:00.8987635Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:00.8987703Z +
2019-07-17T08:57:00.8987733Z 
2019-07-17T08:57:00.8987780Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:00.8988183Z Actual stderr saved to /tmp/compiletestz0Rj1T/issue-27901.stderr
2019-07-17T08:57:00.8988274Z To update references, run this command from build directory:
2019-07-17T08:57:00.8988752Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'issue-27901.rs'
2019-07-17T08:57:00.8988832Z error: 1 errors occurred comparing output.
2019-07-17T08:57:00.8988894Z status: exit code: 1
2019-07-17T08:57:00.8988894Z status: exit code: 1
2019-07-17T08:57:00.8989525Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-27901.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/issue-27901.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/issue-27901.stage-id.aux" "-A" "unused"
2019-07-17T08:57:00.8994638Z ------------------------------------------
2019-07-17T08:57:00.8994695Z 
2019-07-17T08:57:00.8995454Z ------------------------------------------
2019-07-17T08:57:00.8995512Z stderr:
---
2019-07-17T08:57:01.0245731Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:01.0245778Z +
2019-07-17T08:57:01.0245806Z 
2019-07-17T08:57:01.0245975Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:01.0246284Z Actual stderr saved to /tmp/compiletestz0Rj1T/intrinsics-integer.stderr
2019-07-17T08:57:01.0246348Z To update references, run this command from build directory:
2019-07-17T08:57:01.0246602Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'intrinsics-integer.rs'
2019-07-17T08:57:01.0246710Z error: 1 errors occurred comparing output.
2019-07-17T08:57:01.0246754Z status: exit code: 1
2019-07-17T08:57:01.0246754Z status: exit code: 1
2019-07-17T08:57:01.0247436Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/intrinsics-integer.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/intrinsics-integer.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/intrinsics-integer.stage-id.aux" "-A" "unused"
2019-07-17T08:57:01.0247757Z ------------------------------------------
2019-07-17T08:57:01.0247872Z 
2019-07-17T08:57:01.0248110Z ------------------------------------------
2019-07-17T08:57:01.0248154Z stderr:
---
2019-07-17T08:57:01.1101923Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:01.1101974Z +
2019-07-17T08:57:01.1102021Z 
2019-07-17T08:57:01.1102070Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:01.1102341Z Actual stderr saved to /tmp/compiletestz0Rj1T/issue-29746.stderr
2019-07-17T08:57:01.1102414Z To update references, run this command from build directory:
2019-07-17T08:57:01.1102690Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'issue-29746.rs'
2019-07-17T08:57:01.1102774Z error: 1 errors occurred comparing output.
2019-07-17T08:57:01.1102840Z status: exit code: 1
2019-07-17T08:57:01.1102840Z status: exit code: 1
2019-07-17T08:57:01.1103723Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-29746.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/issue-29746.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/issue-29746.stage-id.aux" "-A" "unused"
2019-07-17T08:57:01.1104336Z ------------------------------------------
2019-07-17T08:57:01.1104535Z 
2019-07-17T08:57:01.1105187Z ------------------------------------------
2019-07-17T08:57:01.1105241Z stderr:
---
2019-07-17T08:57:01.1645593Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:01.1645679Z +
2019-07-17T08:57:01.1645707Z 
2019-07-17T08:57:01.1645752Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:01.1646025Z Actual stderr saved to /tmp/compiletestz0Rj1T/issue-30530.stderr
2019-07-17T08:57:01.1646098Z To update references, run this command from build directory:
2019-07-17T08:57:01.1646351Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'issue-30530.rs'
2019-07-17T08:57:01.1646430Z error: 1 errors occurred comparing output.
2019-07-17T08:57:01.1646494Z status: exit code: 1
2019-07-17T08:57:01.1646494Z status: exit code: 1
2019-07-17T08:57:01.1647147Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-30530.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/issue-30530.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/issue-30530.stage-id.aux" "-A" "unused"
2019-07-17T08:57:01.1647577Z ------------------------------------------
2019-07-17T08:57:01.1647611Z 
2019-07-17T08:57:01.1647842Z ------------------------------------------
2019-07-17T08:57:01.1647888Z stderr:
---
2019-07-17T08:57:01.2850928Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:01.2850982Z +
2019-07-17T08:57:01.2851029Z 
2019-07-17T08:57:01.2851076Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:01.2851339Z Actual stderr saved to /tmp/compiletestz0Rj1T/issue-31267-additional.stderr
2019-07-17T08:57:01.2851396Z To update references, run this command from build directory:
2019-07-17T08:57:01.2851681Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'issue-31267-additional.rs'
2019-07-17T08:57:01.2851762Z error: 1 errors occurred comparing output.
2019-07-17T08:57:01.2851825Z status: exit code: 1
2019-07-17T08:57:01.2851825Z status: exit code: 1
2019-07-17T08:57:01.2852513Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-31267-additional.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/issue-31267-additional.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/issue-31267-additional.stage-id.aux" "-A" "unused"
2019-07-17T08:57:01.2852973Z ------------------------------------------
2019-07-17T08:57:01.2853007Z 
2019-07-17T08:57:01.2853218Z ------------------------------------------
2019-07-17T08:57:01.2853283Z stderr:
---
2019-07-17T08:57:01.2963258Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:01.2963307Z +
2019-07-17T08:57:01.2963342Z 
2019-07-17T08:57:01.2963409Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:01.2963676Z Actual stderr saved to /tmp/compiletestz0Rj1T/issue-33387.stderr
2019-07-17T08:57:01.2963732Z To update references, run this command from build directory:
2019-07-17T08:57:01.2964025Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'issue-33387.rs'
2019-07-17T08:57:01.2964117Z error: 1 errors occurred comparing output.
2019-07-17T08:57:01.2964164Z status: exit code: 1
2019-07-17T08:57:01.2964164Z status: exit code: 1
2019-07-17T08:57:01.2965798Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-33387.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/issue-33387.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/issue-33387.stage-id.aux" "-A" "unused"
2019-07-17T08:57:01.2966132Z ------------------------------------------
2019-07-17T08:57:01.2966165Z 
2019-07-17T08:57:01.2966378Z ------------------------------------------
2019-07-17T08:57:01.2966442Z stderr:
---
2019-07-17T08:57:01.4211429Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:01.4211750Z +
2019-07-17T08:57:01.4211985Z 
2019-07-17T08:57:01.4212219Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:01.4212736Z Actual stderr saved to /tmp/compiletestz0Rj1T/issue-34571.stderr
2019-07-17T08:57:01.4213059Z To update references, run this command from build directory:
2019-07-17T08:57:01.4213673Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'issue-34571.rs'
2019-07-17T08:57:01.4217650Z error: 1 errors occurred comparing output.
2019-07-17T08:57:01.4217873Z status: exit code: 1
2019-07-17T08:57:01.4217873Z status: exit code: 1
2019-07-17T08:57:01.4219024Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-34571.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/issue-34571.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/issue-34571.stage-id.aux" "-A" "unused"
2019-07-17T08:57:01.4221849Z ------------------------------------------
2019-07-17T08:57:01.4222886Z 
2019-07-17T08:57:01.4264560Z ------------------------------------------
2019-07-17T08:57:01.4264907Z stderr:
---
2019-07-17T08:57:01.4569532Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:01.4569728Z +
2019-07-17T08:57:01.4569877Z 
2019-07-17T08:57:01.4570030Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:01.4570540Z Actual stderr saved to /tmp/compiletestz0Rj1T/issue-35815.stderr
2019-07-17T08:57:01.4570869Z To update references, run this command from build directory:
2019-07-17T08:57:01.4571293Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'issue-35815.rs'
2019-07-17T08:57:01.4571648Z error: 1 errors occurred comparing output.
2019-07-17T08:57:01.4571797Z status: exit code: 1
2019-07-17T08:57:01.4571797Z status: exit code: 1
2019-07-17T08:57:01.4572612Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-35815.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/issue-35815.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/issue-35815.stage-id.aux" "-A" "unused"
2019-07-17T08:57:01.4573414Z ------------------------------------------
2019-07-17T08:57:01.4575288Z 
2019-07-17T08:57:01.4575922Z ------------------------------------------
2019-07-17T08:57:01.4576793Z stderr:
---
2019-07-17T08:57:01.5970286Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:01.5970362Z +
2019-07-17T08:57:01.5970392Z 
2019-07-17T08:57:01.5970437Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:01.5970701Z Actual stderr saved to /tmp/compiletestz0Rj1T/issue-36278-prefix-nesting.stderr
2019-07-17T08:57:01.5970775Z To update references, run this command from build directory:
2019-07-17T08:57:01.5971225Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'issue-36278-prefix-nesting.rs'
2019-07-17T08:57:01.5971326Z error: 1 errors occurred comparing output.
2019-07-17T08:57:01.5971500Z status: exit code: 1
2019-07-17T08:57:01.5971500Z status: exit code: 1
2019-07-17T08:57:01.5972219Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-36278-prefix-nesting.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/issue-36278-prefix-nesting.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/issue-36278-prefix-nesting.stage-id.aux" "-A" "unused"
2019-07-17T08:57:01.5972543Z ------------------------------------------
2019-07-17T08:57:01.5972576Z 
2019-07-17T08:57:01.5972803Z ------------------------------------------
2019-07-17T08:57:01.5972848Z stderr:
---
2019-07-17T08:57:01.6189168Z -S { s: 5 }
2019-07-17T08:57:01.6193106Z -
2019-07-17T08:57:01.6216751Z 
2019-07-17T08:57:01.6216881Z The actual stdout differed from the expected stdout.
2019-07-17T08:57:01.6217348Z Actual stdout saved to /tmp/compiletestz0Rj1T/issue-3794.stdout
2019-07-17T08:57:01.6217484Z error[E0080]: Miri evaluation error: attempted to do invalid arithmetic on pointers that would leak base addresses, e.g., comparing pointers into different allocations
2019-07-17T08:57:01.6217741Z     --> /checkout/src/libcore/intrinsics.rs:1340:19
2019-07-17T08:57:01.6217789Z      |
2019-07-17T08:57:01.6217834Z 1340 |     let diff = if src_usize > dst_usize {
---
2019-07-17T08:57:01.6222257Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:01.6222324Z +
2019-07-17T08:57:01.6222352Z 
2019-07-17T08:57:01.6222400Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:01.6222678Z Actual stderr saved to /tmp/compiletestz0Rj1T/issue-3794.stderr
2019-07-17T08:57:01.6222737Z To update references, run this command from build directory:
2019-07-17T08:57:01.6223010Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'issue-3794.rs'
2019-07-17T08:57:01.6223109Z error: 2 errors occurred comparing output.
2019-07-17T08:57:01.6223156Z status: exit code: 1
2019-07-17T08:57:01.6223156Z status: exit code: 1
2019-07-17T08:57:01.6224638Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-3794.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/issue-3794.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/issue-3794.stage-id.aux" "-A" "unused"
2019-07-17T08:57:01.6225343Z ------------------------------------------
2019-07-17T08:57:01.6225401Z 
2019-07-17T08:57:01.6226183Z ------------------------------------------
2019-07-17T08:57:01.6226239Z stderr:
---
2019-07-17T08:57:01.7302013Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:01.7302062Z +
2019-07-17T08:57:01.7302089Z 
2019-07-17T08:57:01.7302151Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:01.7302395Z Actual stderr saved to /tmp/compiletestz0Rj1T/issue-53728.stderr
2019-07-17T08:57:01.7302449Z To update references, run this command from build directory:
2019-07-17T08:57:01.7302714Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'issue-53728.rs'
2019-07-17T08:57:01.7302802Z error: 1 errors occurred comparing output.
2019-07-17T08:57:01.7302848Z status: exit code: 1
2019-07-17T08:57:01.7302848Z status: exit code: 1
2019-07-17T08:57:01.7303675Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-53728.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/issue-53728.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/issue-53728.stage-id.aux" "-A" "unused"
2019-07-17T08:57:01.7304032Z ------------------------------------------
2019-07-17T08:57:01.7304068Z 
2019-07-17T08:57:01.7304434Z ------------------------------------------
2019-07-17T08:57:01.7304493Z stderr:
---
2019-07-17T08:57:01.7845755Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:01.7845811Z +
2019-07-17T08:57:01.7845838Z 
2019-07-17T08:57:01.7845904Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:01.7846154Z Actual stderr saved to /tmp/compiletestz0Rj1T/issue-5917.stderr
2019-07-17T08:57:01.7846219Z To update references, run this command from build directory:
2019-07-17T08:57:01.7846476Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'issue-5917.rs'
2019-07-17T08:57:01.7846578Z error: 1 errors occurred comparing output.
2019-07-17T08:57:01.7846624Z status: exit code: 1
2019-07-17T08:57:01.7846624Z status: exit code: 1
2019-07-17T08:57:01.7847281Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-5917.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/issue-5917.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/issue-5917.stage-id.aux" "-A" "unused"
2019-07-17T08:57:01.7847597Z ------------------------------------------
2019-07-17T08:57:01.7847630Z 
2019-07-17T08:57:01.7847848Z ------------------------------------------
2019-07-17T08:57:01.7847893Z stderr:
---
2019-07-17T08:57:01.8648243Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:01.8648314Z +
2019-07-17T08:57:01.8648343Z 
2019-07-17T08:57:01.8648391Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:01.8648669Z Actual stderr saved to /tmp/compiletestz0Rj1T/issue-miri-184.stderr
2019-07-17T08:57:01.8648864Z To update references, run this command from build directory:
2019-07-17T08:57:01.8649142Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'issue-miri-184.rs'
2019-07-17T08:57:01.8649247Z error: 1 errors occurred comparing output.
2019-07-17T08:57:01.8649294Z status: exit code: 1
2019-07-17T08:57:01.8649294Z status: exit code: 1
2019-07-17T08:57:01.8650240Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-miri-184.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/issue-miri-184.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/issue-miri-184.stage-id.aux" "-A" "unused"
2019-07-17T08:57:01.8650912Z ------------------------------------------
2019-07-17T08:57:01.8650947Z 
2019-07-17T08:57:01.8651177Z ------------------------------------------
2019-07-17T08:57:01.8651231Z stderr:
---
2019-07-17T08:57:02.0277262Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:02.0277311Z +
2019-07-17T08:57:02.0277347Z 
2019-07-17T08:57:02.0277412Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:02.0277674Z Actual stderr saved to /tmp/compiletestz0Rj1T/last-use-in-cap-clause.stderr
2019-07-17T08:57:02.0277729Z To update references, run this command from build directory:
2019-07-17T08:57:02.0278007Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'last-use-in-cap-clause.rs'
2019-07-17T08:57:02.0278088Z error: 1 errors occurred comparing output.
2019-07-17T08:57:02.0278151Z status: exit code: 1
2019-07-17T08:57:02.0278151Z status: exit code: 1
2019-07-17T08:57:02.0281121Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/last-use-in-cap-clause.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/last-use-in-cap-clause.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/last-use-in-cap-clause.stage-id.aux" "-A" "unused"
2019-07-17T08:57:02.0281495Z ------------------------------------------
2019-07-17T08:57:02.0281529Z 
2019-07-17T08:57:02.0281736Z ------------------------------------------
2019-07-17T08:57:02.0281797Z stderr:
---
2019-07-17T08:57:02.0295636Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:02.0295691Z +
2019-07-17T08:57:02.0295835Z 
2019-07-17T08:57:02.0295897Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:02.0295947Z Actual stderr saved to /tmp/compiletestz0Rj1T/iter.stderr
2019-07-17T08:57:02.0296022Z To update references, run this command from build directory:
2019-07-17T08:57:02.0296310Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'iter.rs'
2019-07-17T08:57:02.0296389Z error: 1 errors occurred comparing output.
2019-07-17T08:57:02.0296451Z status: exit code: 1
2019-07-17T08:57:02.0296451Z status: exit code: 1
2019-07-17T08:57:02.0297066Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/iter.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/iter.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/iter.stage-id.aux" "-A" "unused"
2019-07-17T08:57:02.0297386Z ------------------------------------------
2019-07-17T08:57:02.0297420Z 
2019-07-17T08:57:02.0297643Z ------------------------------------------
2019-07-17T08:57:02.0297704Z stderr:
---
2019-07-17T08:57:02.2557004Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:02.2557237Z +
2019-07-17T08:57:02.2557383Z 
2019-07-17T08:57:02.2557560Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:02.2558041Z Actual stderr saved to /tmp/compiletestz0Rj1T/linked-list.stderr
2019-07-17T08:57:02.2558814Z To update references, run this command from build directory:
2019-07-17T08:57:02.2559232Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'linked-list.rs'
2019-07-17T08:57:02.2584250Z error: 1 errors occurred comparing output.
2019-07-17T08:57:02.2584548Z status: exit code: 1
2019-07-17T08:57:02.2584548Z status: exit code: 1
2019-07-17T08:57:02.2585974Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/linked-list.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/linked-list.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/linked-list.stage-id.aux" "-A" "unused"
2019-07-17T08:57:02.2607220Z ------------------------------------------
2019-07-17T08:57:02.2607418Z 
2019-07-17T08:57:02.2608814Z ------------------------------------------
2019-07-17T08:57:02.2609071Z stderr:
---
2019-07-17T08:57:02.3110451Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:02.3110499Z +
2019-07-17T08:57:02.3110545Z 
2019-07-17T08:57:02.3110598Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:02.3111009Z Actual stderr saved to /tmp/compiletestz0Rj1T/loop-break-value.stderr
2019-07-17T08:57:02.3111066Z To update references, run this command from build directory:
2019-07-17T08:57:02.3111342Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'loop-break-value.rs'
2019-07-17T08:57:02.3111422Z error: 1 errors occurred comparing output.
2019-07-17T08:57:02.3111486Z status: exit code: 1
2019-07-17T08:57:02.3111486Z status: exit code: 1
2019-07-17T08:57:02.3112153Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/loop-break-value.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/loop-break-value.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/loop-break-value.stage-id.aux" "-A" "unused"
2019-07-17T08:57:02.3112484Z ------------------------------------------
2019-07-17T08:57:02.3112517Z 
2019-07-17T08:57:02.3112728Z ------------------------------------------
2019-07-17T08:57:02.3112793Z stderr:
---
2019-07-17T08:57:02.4408006Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:02.4408056Z +
2019-07-17T08:57:02.4408086Z 
2019-07-17T08:57:02.4408151Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:02.4408203Z Actual stderr saved to /tmp/compiletestz0Rj1T/main_fn.stderr
2019-07-17T08:57:02.4408256Z To update references, run this command from build directory:
2019-07-17T08:57:02.4408543Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'main_fn.rs'
2019-07-17T08:57:02.4408628Z error: 1 errors occurred comparing output.
2019-07-17T08:57:02.4408692Z status: exit code: 1
2019-07-17T08:57:02.4408692Z status: exit code: 1
2019-07-17T08:57:02.4409525Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/main_fn.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/main_fn.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/main_fn.stage-id.aux" "-A" "unused"
2019-07-17T08:57:02.4409840Z ------------------------------------------
2019-07-17T08:57:02.4409871Z 
2019-07-17T08:57:02.4410183Z ------------------------------------------
2019-07-17T08:57:02.4410243Z stderr:
---
2019-07-17T08:57:02.4573709Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:02.4573780Z +
2019-07-17T08:57:02.4573812Z 
2019-07-17T08:57:02.4573860Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:02.4573913Z Actual stderr saved to /tmp/compiletestz0Rj1T/loops.stderr
2019-07-17T08:57:02.4573985Z To update references, run this command from build directory:
2019-07-17T08:57:02.4574261Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'loops.rs'
2019-07-17T08:57:02.4574345Z error: 1 errors occurred comparing output.
2019-07-17T08:57:02.4574621Z status: exit code: 1
2019-07-17T08:57:02.4574621Z status: exit code: 1
2019-07-17T08:57:02.4575613Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/loops.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/loops.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/loops.stage-id.aux" "-A" "unused"
2019-07-17T08:57:02.4575991Z ------------------------------------------
2019-07-17T08:57:02.4576025Z 
2019-07-17T08:57:02.4576256Z ------------------------------------------
2019-07-17T08:57:02.4576302Z stderr:
---
2019-07-17T08:57:02.5927045Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:02.5927127Z +
2019-07-17T08:57:02.5927156Z 
2019-07-17T08:57:02.5932631Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:02.5932702Z Actual stderr saved to /tmp/compiletestz0Rj1T/match_slice.stderr
2019-07-17T08:57:02.5932755Z To update references, run this command from build directory:
2019-07-17T08:57:02.5933290Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'match_slice.rs'
2019-07-17T08:57:02.5933404Z error: 1 errors occurred comparing output.
2019-07-17T08:57:02.5933450Z status: exit code: 1
2019-07-17T08:57:02.5933450Z status: exit code: 1
2019-07-17T08:57:02.5934153Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/match_slice.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/match_slice.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/match_slice.stage-id.aux" "-A" "unused"
2019-07-17T08:57:02.5934497Z ------------------------------------------
2019-07-17T08:57:02.5952396Z thread '[ui] run-pass/match_slice.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
2019-07-17T08:57:02.5952828Z thread '[ui] run-pass/many_shr_bor.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
2019-07-17T08:57:02.5952883Z 
---
2019-07-17T08:57:02.5966588Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:02.5966656Z +
2019-07-17T08:57:02.5966683Z 
2019-07-17T08:57:02.5966729Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:02.5966798Z Actual stderr saved to /tmp/compiletestz0Rj1T/many_shr_bor.stderr
2019-07-17T08:57:02.5966849Z To update references, run this command from build directory:
2019-07-17T08:57:02.5967109Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'many_shr_bor.rs'
2019-07-17T08:57:02.5967212Z error: 1 errors occurred comparing output.
2019-07-17T08:57:02.5967269Z status: exit code: 1
2019-07-17T08:57:02.5967269Z status: exit code: 1
2019-07-17T08:57:02.5967938Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/many_shr_bor.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/many_shr_bor.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/many_shr_bor.stage-id.aux" "-A" "unused"
2019-07-17T08:57:02.5968283Z ------------------------------------------
2019-07-17T08:57:02.5968335Z 
2019-07-17T08:57:02.5968567Z ------------------------------------------
2019-07-17T08:57:02.5968614Z stderr:
---
2019-07-17T08:57:02.8180996Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:02.8181226Z +
2019-07-17T08:57:02.8181349Z 
2019-07-17T08:57:02.8181967Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:02.8224143Z Actual stderr saved to /tmp/compiletestz0Rj1T/mir_coercions.stderr
2019-07-17T08:57:02.8224529Z To update references, run this command from build directory:
2019-07-17T08:57:02.8226141Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'mir_coercions.rs'
2019-07-17T08:57:02.8226590Z error: 1 errors occurred comparing output.
2019-07-17T08:57:02.8226769Z status: exit code: 1
2019-07-17T08:57:02.8226769Z status: exit code: 1
2019-07-17T08:57:02.8227829Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/mir_coercions.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/mir_coercions.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/mir_coercions.stage-id.aux" "-A" "unused"
2019-07-17T08:57:02.8228469Z ------------------------------------------
2019-07-17T08:57:02.8228629Z 
2019-07-17T08:57:02.8229125Z ------------------------------------------
2019-07-17T08:57:02.8229319Z stderr:
---
2019-07-17T08:57:02.8473498Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:02.8473915Z +
2019-07-17T08:57:02.8474131Z 
2019-07-17T08:57:02.8474178Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:02.8474230Z Actual stderr saved to /tmp/compiletestz0Rj1T/memchr.stderr
2019-07-17T08:57:02.8474299Z To update references, run this command from build directory:
2019-07-17T08:57:02.8474639Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'memchr.rs'
2019-07-17T08:57:02.8475524Z error: 1 errors occurred comparing output.
2019-07-17T08:57:02.8475624Z status: exit code: 1
2019-07-17T08:57:02.8475624Z status: exit code: 1
2019-07-17T08:57:02.8476363Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/memchr.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/memchr.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/memchr.stage-id.aux" "-A" "unused"
2019-07-17T08:57:02.8476687Z ------------------------------------------
2019-07-17T08:57:02.8476721Z 
2019-07-17T08:57:02.8476956Z ------------------------------------------
2019-07-17T08:57:02.8477002Z stderr:
---
2019-07-17T08:57:03.0110487Z 
2019-07-17T08:57:03.0110834Z thread '[ui] run-pass/miri-issue-133.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
2019-07-17T08:57:03.0111454Z thread '[ui] run-pass/mir_fat_ptr.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
2019-07-17T08:57:03.0111547Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:03.0111830Z Actual stderr saved to /tmp/compiletestz0Rj1T/miri-issue-133.stderr
2019-07-17T08:57:03.0111885Z To update references, run this command from build directory:
2019-07-17T08:57:03.0112156Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'miri-issue-133.rs'
2019-07-17T08:57:03.0112237Z error: 1 errors occurred comparing output.
2019-07-17T08:57:03.0112283Z status: exit code: 1
2019-07-17T08:57:03.0112283Z status: exit code: 1
2019-07-17T08:57:03.0112977Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/miri-issue-133.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/miri-issue-133.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/miri-issue-133.stage-id.aux" "-A" "unused"
2019-07-17T08:57:03.0113472Z ------------------------------------------
2019-07-17T08:57:03.0113508Z 
2019-07-17T08:57:03.0113722Z ------------------------------------------
2019-07-17T08:57:03.0113787Z stderr:
---
2019-07-17T08:57:03.0122929Z ------------------------------------------
2019-07-17T08:57:03.0122962Z 
2019-07-17T08:57:03.0122991Z 
2019-07-17T08:57:03.0123038Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:03.0123116Z Actual stderr saved to /tmp/compiletestz0Rj1T/mir_fat_ptr.stderr
2019-07-17T08:57:03.0123170Z To update references, run this command from build directory:
2019-07-17T08:57:03.0123439Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'mir_fat_ptr.rs'
2019-07-17T08:57:03.0123539Z error: 1 errors occurred comparing output.
2019-07-17T08:57:03.0123583Z status: exit code: 1
2019-07-17T08:57:03.0123583Z status: exit code: 1
2019-07-17T08:57:03.0124240Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/mir_fat_ptr.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/mir_fat_ptr.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/mir_fat_ptr.stage-id.aux" "-A" "unused"
2019-07-17T08:57:03.0124554Z ------------------------------------------
2019-07-17T08:57:03.0124589Z 
2019-07-17T08:57:03.0124875Z ------------------------------------------
2019-07-17T08:57:03.0124931Z stderr:
---
2019-07-17T08:57:03.1648379Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:03.1648430Z +
2019-07-17T08:57:03.1648459Z 
2019-07-17T08:57:03.1648534Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:03.1649037Z Actual stderr saved to /tmp/compiletestz0Rj1T/move-arg-2-unique.stderr
2019-07-17T08:57:03.1649093Z To update references, run this command from build directory:
2019-07-17T08:57:03.1649361Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'move-arg-2-unique.rs'
2019-07-17T08:57:03.1649440Z error: 1 errors occurred comparing output.
2019-07-17T08:57:03.1649485Z status: exit code: 1
2019-07-17T08:57:03.1649485Z status: exit code: 1
2019-07-17T08:57:03.1650255Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/move-arg-2-unique.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/move-arg-2-unique.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/move-arg-2-unique.stage-id.aux" "-A" "unused"
2019-07-17T08:57:03.1650943Z ------------------------------------------
2019-07-17T08:57:03.1650989Z 
2019-07-17T08:57:03.1651201Z ------------------------------------------
2019-07-17T08:57:03.1651265Z stderr:
---
2019-07-17T08:57:03.1883753Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:03.1883825Z +
2019-07-17T08:57:03.1883856Z 
2019-07-17T08:57:03.1883913Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:03.1884189Z Actual stderr saved to /tmp/compiletestz0Rj1T/move-arg-3-unique.stderr
2019-07-17T08:57:03.1884275Z To update references, run this command from build directory:
2019-07-17T08:57:03.1884556Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'move-arg-3-unique.rs'
2019-07-17T08:57:03.1884640Z error: 1 errors occurred comparing output.
2019-07-17T08:57:03.1884873Z status: exit code: 1
2019-07-17T08:57:03.1884873Z status: exit code: 1
2019-07-17T08:57:03.1886145Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/move-arg-3-unique.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/move-arg-3-unique.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/move-arg-3-unique.stage-id.aux" "-A" "unused"
2019-07-17T08:57:03.1886663Z ------------------------------------------
2019-07-17T08:57:03.1886710Z 
2019-07-17T08:57:03.1886988Z ------------------------------------------
2019-07-17T08:57:03.1887035Z stderr:
---
2019-07-17T08:57:03.3089780Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:03.3089844Z +
2019-07-17T08:57:03.3089870Z 
2019-07-17T08:57:03.3089935Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:03.3090206Z Actual stderr saved to /tmp/compiletestz0Rj1T/move-undef-primval.stderr
2019-07-17T08:57:03.3091100Z To update references, run this command from build directory:
2019-07-17T08:57:03.3091444Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'move-undef-primval.rs'
2019-07-17T08:57:03.3091528Z error: 1 errors occurred comparing output.
2019-07-17T08:57:03.3091574Z status: exit code: 1
2019-07-17T08:57:03.3091574Z status: exit code: 1
2019-07-17T08:57:03.3092451Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/move-undef-primval.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/move-undef-primval.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/move-undef-primval.stage-id.aux" "-A" "unused"
2019-07-17T08:57:03.3092852Z ------------------------------------------
2019-07-17T08:57:03.3092888Z 
2019-07-17T08:57:03.3093102Z ------------------------------------------
2019-07-17T08:57:03.3093167Z stderr:
---
2019-07-17T08:57:03.3891146Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:03.3891201Z +
2019-07-17T08:57:03.3891226Z 
2019-07-17T08:57:03.3891270Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:03.3891337Z Actual stderr saved to /tmp/compiletestz0Rj1T/mpsc.stderr
2019-07-17T08:57:03.3891385Z To update references, run this command from build directory:
2019-07-17T08:57:03.3891626Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'mpsc.rs'
2019-07-17T08:57:03.3891723Z error: 1 errors occurred comparing output.
2019-07-17T08:57:03.3891768Z status: exit code: 1
2019-07-17T08:57:03.3891768Z status: exit code: 1
2019-07-17T08:57:03.3892487Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/mpsc.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/mpsc.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/mpsc.stage-id.aux" "-A" "unused"
2019-07-17T08:57:03.3892841Z ------------------------------------------
2019-07-17T08:57:03.3892895Z 
2019-07-17T08:57:03.3893123Z ------------------------------------------
2019-07-17T08:57:03.3893168Z stderr:
---
2019-07-17T08:57:03.4803083Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:03.4803143Z +
2019-07-17T08:57:03.4810869Z 
2019-07-17T08:57:03.4810996Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:03.4811051Z Actual stderr saved to /tmp/compiletestz0Rj1T/multi_arg_closure.stderr
2019-07-17T08:57:03.4811105Z To update references, run this command from build directory:
2019-07-17T08:57:03.4811575Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'multi_arg_closure.rs'
2019-07-17T08:57:03.4811664Z error: 1 errors occurred comparing output.
2019-07-17T08:57:03.4811728Z status: exit code: 1
2019-07-17T08:57:03.4811728Z status: exit code: 1
2019-07-17T08:57:03.4820760Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/multi_arg_closure.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/multi_arg_closure.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/multi_arg_closure.stage-id.aux" "-A" "unused"
2019-07-17T08:57:03.4821186Z ------------------------------------------
2019-07-17T08:57:03.4821223Z 
2019-07-17T08:57:03.4821436Z ------------------------------------------
2019-07-17T08:57:03.4821627Z stderr:
---
2019-07-17T08:57:03.5323142Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:03.5323199Z +
2019-07-17T08:57:03.5330131Z 
2019-07-17T08:57:03.5330249Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:03.5330305Z Actual stderr saved to /tmp/compiletestz0Rj1T/negative_discriminant.stderr
2019-07-17T08:57:03.5330357Z To update references, run this command from build directory:
2019-07-17T08:57:03.5330938Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'negative_discriminant.rs'
2019-07-17T08:57:03.5331025Z error: 1 errors occurred comparing output.
2019-07-17T08:57:03.5331069Z status: exit code: 1
2019-07-17T08:57:03.5331069Z status: exit code: 1
2019-07-17T08:57:03.5331873Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/negative_discriminant.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/negative_discriminant.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/negative_discriminant.stage-id.aux" "-A" "unused"
2019-07-17T08:57:03.5332241Z ------------------------------------------
2019-07-17T08:57:03.5332274Z 
2019-07-17T08:57:03.5332478Z ------------------------------------------
2019-07-17T08:57:03.5332522Z stderr:
---
2019-07-17T08:57:03.6676245Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:03.6676298Z +
2019-07-17T08:57:03.6676328Z 
2019-07-17T08:57:03.6676392Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:03.6676448Z Actual stderr saved to /tmp/compiletestz0Rj1T/non_capture_closure_to_fn_ptr.stderr
2019-07-17T08:57:03.6676504Z To update references, run this command from build directory:
2019-07-17T08:57:03.6676894Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'non_capture_closure_to_fn_ptr.rs'
2019-07-17T08:57:03.6676982Z error: 1 errors occurred comparing output.
2019-07-17T08:57:03.6677147Z status: exit code: 1
2019-07-17T08:57:03.6677147Z status: exit code: 1
2019-07-17T08:57:03.6677912Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/non_capture_closure_to_fn_ptr.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/non_capture_closure_to_fn_ptr.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/non_capture_closure_to_fn_ptr.stage-id.aux" "-A" "unused"
2019-07-17T08:57:03.6678255Z ------------------------------------------
2019-07-17T08:57:03.6678290Z 
2019-07-17T08:57:03.6678720Z ------------------------------------------
2019-07-17T08:57:03.6678770Z stderr:
---
2019-07-17T08:57:03.6898270Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:03.6902699Z +
2019-07-17T08:57:03.6909074Z 
2019-07-17T08:57:03.6912820Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:03.6930207Z Actual stderr saved to /tmp/compiletestz0Rj1T/observed_local_mut.stderr
2019-07-17T08:57:03.6931212Z To update references, run this command from build directory:
2019-07-17T08:57:03.6931212Z To update references, run this command from build directory:
2019-07-17T08:57:03.6931535Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'observed_local_mut.rs'
2019-07-17T08:57:03.6931642Z error: 1 errors occurred comparing output.
2019-07-17T08:57:03.6931691Z status: exit code: 1
2019-07-17T08:57:03.6931691Z status: exit code: 1
2019-07-17T08:57:03.6932380Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/observed_local_mut.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/observed_local_mut.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-Zmiri-disable-validation" "-L" "/tmp/compiletestz0Rj1T/observed_local_mut.stage-id.aux" "-A" "unused"
2019-07-17T08:57:03.6932710Z ------------------------------------------
2019-07-17T08:57:03.6932747Z 
2019-07-17T08:57:03.6933107Z ------------------------------------------
2019-07-17T08:57:03.6933154Z stderr:
---
2019-07-17T08:57:03.8211586Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:03.8211711Z +
2019-07-17T08:57:03.8211744Z 
2019-07-17T08:57:03.8211969Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:03.8212030Z Actual stderr saved to /tmp/compiletestz0Rj1T/option_box_transmute_ptr.stderr
2019-07-17T08:57:03.8212080Z To update references, run this command from build directory:
2019-07-17T08:57:03.8212388Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'option_box_transmute_ptr.rs'
2019-07-17T08:57:03.8212467Z error: 1 errors occurred comparing output.
2019-07-17T08:57:03.8212527Z status: exit code: 1
2019-07-17T08:57:03.8212527Z status: exit code: 1
2019-07-17T08:57:03.8213186Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/option_box_transmute_ptr.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/option_box_transmute_ptr.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/option_box_transmute_ptr.stage-id.aux" "-A" "unused"
2019-07-17T08:57:03.8213600Z ------------------------------------------
2019-07-17T08:57:03.8213632Z 
2019-07-17T08:57:03.8213833Z ------------------------------------------
2019-07-17T08:57:03.8213893Z stderr:
---
2019-07-17T08:57:03.8514706Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:03.8514758Z +
2019-07-17T08:57:03.8515241Z 
2019-07-17T08:57:03.8515300Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:03.8515360Z Actual stderr saved to /tmp/compiletestz0Rj1T/option_eq.stderr
2019-07-17T08:57:03.8515430Z To update references, run this command from build directory:
2019-07-17T08:57:03.8515769Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'option_eq.rs'
2019-07-17T08:57:03.8515867Z error: 1 errors occurred comparing output.
2019-07-17T08:57:03.8515914Z status: exit code: 1
2019-07-17T08:57:03.8515914Z status: exit code: 1
2019-07-17T08:57:03.8516563Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/option_eq.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/option_eq.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/option_eq.stage-id.aux" "-A" "unused"
2019-07-17T08:57:03.8516882Z ------------------------------------------
2019-07-17T08:57:03.8517001Z 
2019-07-17T08:57:03.8517260Z ------------------------------------------
2019-07-17T08:57:03.8517307Z stderr:
---
2019-07-17T08:57:04.0081437Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:04.0081513Z +
2019-07-17T08:57:04.0081540Z 
2019-07-17T08:57:04.0081593Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:04.0081645Z Actual stderr saved to /tmp/compiletestz0Rj1T/packed_static.stderr
2019-07-17T08:57:04.0081715Z To update references, run this command from build directory:
2019-07-17T08:57:04.0081998Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'packed_static.rs'
2019-07-17T08:57:04.0082093Z error: 1 errors occurred comparing output.
2019-07-17T08:57:04.0082140Z status: exit code: 1
2019-07-17T08:57:04.0082140Z status: exit code: 1
2019-07-17T08:57:04.0082795Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/packed_static.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/packed_static.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/packed_static.stage-id.aux" "-A" "unused"
2019-07-17T08:57:04.0083211Z ------------------------------------------
2019-07-17T08:57:04.0083244Z 
2019-07-17T08:57:04.0083469Z ------------------------------------------
2019-07-17T08:57:04.0083514Z stderr:
---
2019-07-17T08:57:04.0192155Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:04.0192373Z +
2019-07-17T08:57:04.0192496Z 
2019-07-17T08:57:04.0192639Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:04.0193052Z Actual stderr saved to /tmp/compiletestz0Rj1T/overloaded-calls-simple.stderr
2019-07-17T08:57:04.0193246Z To update references, run this command from build directory:
2019-07-17T08:57:04.0193670Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'overloaded-calls-simple.rs'
2019-07-17T08:57:04.0194163Z error: 1 errors occurred comparing output.
2019-07-17T08:57:04.0194313Z status: exit code: 1
2019-07-17T08:57:04.0194313Z status: exit code: 1
2019-07-17T08:57:04.0201635Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/overloaded-calls-simple.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/overloaded-calls-simple.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/overloaded-calls-simple.stage-id.aux" "-A" "unused"
2019-07-17T08:57:04.0202849Z ------------------------------------------
2019-07-17T08:57:04.0203041Z 
2019-07-17T08:57:04.0203444Z ------------------------------------------
2019-07-17T08:57:04.0203629Z stderr:
---
2019-07-17T08:57:04.2023232Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:04.2023281Z +
2019-07-17T08:57:04.2023308Z 
2019-07-17T08:57:04.2023376Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:04.2023430Z Actual stderr saved to /tmp/compiletestz0Rj1T/packed_struct.stderr
2019-07-17T08:57:04.2023705Z To update references, run this command from build directory:
2019-07-17T08:57:04.2024076Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'packed_struct.rs'
2019-07-17T08:57:04.2024173Z error: 1 errors occurred comparing output.
2019-07-17T08:57:04.2024237Z status: exit code: 1
2019-07-17T08:57:04.2024237Z status: exit code: 1
2019-07-17T08:57:04.2025043Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/packed_struct.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/packed_struct.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/packed_struct.stage-id.aux" "-A" "unused"
2019-07-17T08:57:04.2025960Z ------------------------------------------
2019-07-17T08:57:04.2025996Z 
2019-07-17T08:57:04.2026207Z ------------------------------------------
2019-07-17T08:57:04.2026272Z stderr:
---
2019-07-17T08:57:04.2286900Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:04.2286970Z +
2019-07-17T08:57:04.2286999Z 
2019-07-17T08:57:04.2287046Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:04.2287118Z Actual stderr saved to /tmp/compiletestz0Rj1T/pointers.stderr
2019-07-17T08:57:04.2287172Z To update references, run this command from build directory:
2019-07-17T08:57:04.2287452Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'pointers.rs'
2019-07-17T08:57:04.2287563Z error: 1 errors occurred comparing output.
2019-07-17T08:57:04.2287701Z status: exit code: 1
2019-07-17T08:57:04.2287701Z status: exit code: 1
2019-07-17T08:57:04.2288384Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/pointers.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/pointers.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/pointers.stage-id.aux" "-A" "unused"
2019-07-17T08:57:04.2289096Z ------------------------------------------
2019-07-17T08:57:04.2289149Z 
2019-07-17T08:57:04.2289363Z ------------------------------------------
2019-07-17T08:57:04.2289409Z stderr:
---
2019-07-17T08:57:04.3805858Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:04.3805906Z +
2019-07-17T08:57:04.3805933Z 
2019-07-17T08:57:04.3805979Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:04.3806050Z Actual stderr saved to /tmp/compiletestz0Rj1T/ptr_arith_offset.stderr
2019-07-17T08:57:04.3806102Z To update references, run this command from build directory:
2019-07-17T08:57:04.3806367Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'ptr_arith_offset.rs'
2019-07-17T08:57:04.3806467Z error: 1 errors occurred comparing output.
2019-07-17T08:57:04.3806520Z status: exit code: 1
2019-07-17T08:57:04.3806520Z status: exit code: 1
2019-07-17T08:57:04.3807198Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/ptr_arith_offset.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/ptr_arith_offset.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/ptr_arith_offset.stage-id.aux" "-A" "unused"
2019-07-17T08:57:04.3807719Z ------------------------------------------
2019-07-17T08:57:04.3807754Z 
2019-07-17T08:57:04.3807967Z ------------------------------------------
2019-07-17T08:57:04.3808012Z stderr:
---
2019-07-17T08:57:04.4047291Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:04.4047346Z +
2019-07-17T08:57:04.4047376Z 
2019-07-17T08:57:04.4047425Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:04.4047497Z Actual stderr saved to /tmp/compiletestz0Rj1T/products.stderr
2019-07-17T08:57:04.4047550Z To update references, run this command from build directory:
2019-07-17T08:57:04.4047843Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'products.rs'
2019-07-17T08:57:04.4047958Z error: 1 errors occurred comparing output.
2019-07-17T08:57:04.4048118Z status: exit code: 1
2019-07-17T08:57:04.4048118Z status: exit code: 1
2019-07-17T08:57:04.4048965Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/products.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/products.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/products.stage-id.aux" "-A" "unused"
2019-07-17T08:57:04.4049286Z ------------------------------------------
2019-07-17T08:57:04.4049319Z 
2019-07-17T08:57:04.4049524Z ------------------------------------------
2019-07-17T08:57:04.4049568Z stderr:
---
2019-07-17T08:57:04.5369087Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:04.5369152Z +
2019-07-17T08:57:04.5369179Z 
2019-07-17T08:57:04.5369224Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:04.5369340Z Actual stderr saved to /tmp/compiletestz0Rj1T/ptr_arith_offset_overflow.stderr
2019-07-17T08:57:04.5369408Z To update references, run this command from build directory:
2019-07-17T08:57:04.5369684Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'ptr_arith_offset_overflow.rs'
2019-07-17T08:57:04.5369889Z error: 1 errors occurred comparing output.
2019-07-17T08:57:04.5369933Z status: exit code: 1
2019-07-17T08:57:04.5369933Z status: exit code: 1
2019-07-17T08:57:04.5370620Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/ptr_arith_offset_overflow.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/ptr_arith_offset_overflow.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/ptr_arith_offset_overflow.stage-id.aux" "-A" "unused"
2019-07-17T08:57:04.5371147Z ------------------------------------------
2019-07-17T08:57:04.5371180Z 
2019-07-17T08:57:04.5371456Z ------------------------------------------
2019-07-17T08:57:04.5371502Z stderr:
---
2019-07-17T08:57:04.5862266Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:04.5862334Z +
2019-07-17T08:57:04.5862362Z 
2019-07-17T08:57:04.5862410Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:04.5862473Z Actual stderr saved to /tmp/compiletestz0Rj1T/ptr_int_casts.stderr
2019-07-17T08:57:04.5862546Z To update references, run this command from build directory:
2019-07-17T08:57:04.5862953Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'ptr_int_casts.rs'
2019-07-17T08:57:04.5863058Z error: 1 errors occurred comparing output.
2019-07-17T08:57:04.5863106Z status: exit code: 1
2019-07-17T08:57:04.5863106Z status: exit code: 1
2019-07-17T08:57:04.5864020Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/ptr_int_casts.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/ptr_int_casts.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/ptr_int_casts.stage-id.aux" "-A" "unused"
2019-07-17T08:57:04.5864396Z ------------------------------------------
2019-07-17T08:57:04.5864442Z 
2019-07-17T08:57:04.5864701Z ------------------------------------------
2019-07-17T08:57:04.5864758Z stderr:
---
2019-07-17T08:57:04.7249428Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:04.7249481Z +
2019-07-17T08:57:04.7249506Z 
2019-07-17T08:57:04.7249558Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:04.7249625Z Actual stderr saved to /tmp/compiletestz0Rj1T/ptr_int_ops.stderr
2019-07-17T08:57:04.7249769Z To update references, run this command from build directory:
2019-07-17T08:57:04.7250043Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'ptr_int_ops.rs'
2019-07-17T08:57:04.7250139Z error: 1 errors occurred comparing output.
2019-07-17T08:57:04.7250182Z status: exit code: 1
2019-07-17T08:57:04.7250182Z status: exit code: 1
2019-07-17T08:57:04.7250802Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/ptr_int_ops.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/ptr_int_ops.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/ptr_int_ops.stage-id.aux" "-A" "unused"
2019-07-17T08:57:04.7251272Z ------------------------------------------
2019-07-17T08:57:04.7251324Z 
2019-07-17T08:57:04.7251542Z ------------------------------------------
2019-07-17T08:57:04.7251586Z stderr:
---
2019-07-17T08:57:04.7564436Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:04.7564511Z +
2019-07-17T08:57:04.7564541Z 
2019-07-17T08:57:04.7564587Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:04.7564726Z Actual stderr saved to /tmp/compiletestz0Rj1T/ptr_offset.stderr
2019-07-17T08:57:04.7564796Z To update references, run this command from build directory:
2019-07-17T08:57:04.7565083Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'ptr_offset.rs'
2019-07-17T08:57:04.7565448Z error: 1 errors occurred comparing output.
2019-07-17T08:57:04.7565494Z status: exit code: 1
2019-07-17T08:57:04.7565494Z status: exit code: 1
2019-07-17T08:57:04.7566182Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/ptr_offset.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/ptr_offset.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/ptr_offset.stage-id.aux" "-A" "unused"
2019-07-17T08:57:04.7566514Z ------------------------------------------
2019-07-17T08:57:04.7566547Z 
2019-07-17T08:57:04.7566777Z ------------------------------------------
2019-07-17T08:57:04.7566822Z stderr:
---
2019-07-17T08:57:04.9184394Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:04.9184443Z +
2019-07-17T08:57:04.9184570Z 
2019-07-17T08:57:04.9184635Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:04.9184685Z Actual stderr saved to /tmp/compiletestz0Rj1T/raw.stderr
2019-07-17T08:57:04.9184733Z To update references, run this command from build directory:
2019-07-17T08:57:04.9185034Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'raw.rs'
2019-07-17T08:57:04.9185113Z error: 1 errors occurred comparing output.
2019-07-17T08:57:04.9185655Z status: exit code: 1
2019-07-17T08:57:04.9185655Z status: exit code: 1
2019-07-17T08:57:04.9186371Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/raw.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/raw.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/raw.stage-id.aux" "-A" "unused"
2019-07-17T08:57:04.9186700Z ------------------------------------------
2019-07-17T08:57:04.9186734Z 
2019-07-17T08:57:04.9186947Z ------------------------------------------
2019-07-17T08:57:04.9187013Z stderr:
---
2019-07-17T08:57:05.0291188Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:05.0291238Z +
2019-07-17T08:57:05.0291285Z 
2019-07-17T08:57:05.0291332Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:05.0291382Z Actual stderr saved to /tmp/compiletestz0Rj1T/rc.stderr
2019-07-17T08:57:05.0291432Z To update references, run this command from build directory:
2019-07-17T08:57:05.0291703Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'rc.rs'
2019-07-17T08:57:05.0291786Z error: 1 errors occurred comparing output.
2019-07-17T08:57:05.0291852Z status: exit code: 1
2019-07-17T08:57:05.0291852Z status: exit code: 1
2019-07-17T08:57:05.0292509Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/rc.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/rc.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/rc.stage-id.aux" "-A" "unused"
2019-07-17T08:57:05.0292858Z ------------------------------------------
2019-07-17T08:57:05.0292894Z 
2019-07-17T08:57:05.0293123Z ------------------------------------------
2019-07-17T08:57:05.0293187Z stderr:
---
2019-07-17T08:57:05.0692940Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:05.0693135Z +
2019-07-17T08:57:05.0693259Z 
2019-07-17T08:57:05.0693424Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:05.0693577Z Actual stderr saved to /tmp/compiletestz0Rj1T/recursive_static.stderr
2019-07-17T08:57:05.0693748Z To update references, run this command from build directory:
2019-07-17T08:57:05.0694286Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'recursive_static.rs'
2019-07-17T08:57:05.0694642Z error: 1 errors occurred comparing output.
2019-07-17T08:57:05.0694942Z status: exit code: 1
2019-07-17T08:57:05.0694942Z status: exit code: 1
2019-07-17T08:57:05.0696408Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/recursive_static.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/recursive_static.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/recursive_static.stage-id.aux" "-A" "unused"
2019-07-17T08:57:05.0697067Z ------------------------------------------
2019-07-17T08:57:05.0697243Z 
2019-07-17T08:57:05.0744096Z ------------------------------------------
2019-07-17T08:57:05.0744303Z stderr:
---
2019-07-17T08:57:05.1797579Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:05.1797777Z +
2019-07-17T08:57:05.1797926Z 
2019-07-17T08:57:05.1798067Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:05.1798470Z Actual stderr saved to /tmp/compiletestz0Rj1T/ref-invalid-ptr.stderr
2019-07-17T08:57:05.1798687Z To update references, run this command from build directory:
2019-07-17T08:57:05.1799120Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'ref-invalid-ptr.rs'
2019-07-17T08:57:05.1799441Z error: 1 errors occurred comparing output.
2019-07-17T08:57:05.1799581Z status: exit code: 1
2019-07-17T08:57:05.1799581Z status: exit code: 1
2019-07-17T08:57:05.1800444Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/ref-invalid-ptr.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/ref-invalid-ptr.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-Zmiri-disable-validation" "-L" "/tmp/compiletestz0Rj1T/ref-invalid-ptr.stage-id.aux" "-A" "unused"
2019-07-17T08:57:05.1801245Z ------------------------------------------
2019-07-17T08:57:05.1801404Z 
2019-07-17T08:57:05.1801772Z ------------------------------------------
2019-07-17T08:57:05.1801951Z stderr:
---
2019-07-17T08:57:05.2166896Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:05.2166946Z +
2019-07-17T08:57:05.2166974Z 
2019-07-17T08:57:05.2167039Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:05.2167094Z Actual stderr saved to /tmp/compiletestz0Rj1T/refcell.stderr
2019-07-17T08:57:05.2167146Z To update references, run this command from build directory:
2019-07-17T08:57:05.2167430Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'refcell.rs'
2019-07-17T08:57:05.2167514Z error: 1 errors occurred comparing output.
2019-07-17T08:57:05.2167561Z status: exit code: 1
2019-07-17T08:57:05.2167561Z status: exit code: 1
2019-07-17T08:57:05.2168228Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/refcell.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/refcell.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/refcell.stage-id.aux" "-A" "unused"
2019-07-17T08:57:05.2169069Z ------------------------------------------
2019-07-17T08:57:05.2169102Z 
2019-07-17T08:57:05.2169297Z ------------------------------------------
2019-07-17T08:57:05.2169340Z stderr:
---
2019-07-17T08:57:05.3415704Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:05.3415760Z +
2019-07-17T08:57:05.3415789Z 
2019-07-17T08:57:05.3415855Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:05.3416132Z Actual stderr saved to /tmp/compiletestz0Rj1T/regions-lifetime-nonfree-late-bound.stderr
2019-07-17T08:57:05.3416189Z To update references, run this command from build directory:
2019-07-17T08:57:05.3416495Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'regions-lifetime-nonfree-late-bound.rs'
2019-07-17T08:57:05.3416587Z error: 1 errors occurred comparing output.
2019-07-17T08:57:05.3416633Z status: exit code: 1
2019-07-17T08:57:05.3416633Z status: exit code: 1
2019-07-17T08:57:05.3417367Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/regions-lifetime-nonfree-late-bound.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/regions-lifetime-nonfree-late-bound.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/regions-lifetime-nonfree-late-bound.stage-id.aux" "-A" "unused"
2019-07-17T08:57:05.3417688Z ------------------------------------------
2019-07-17T08:57:05.3417721Z 
2019-07-17T08:57:05.3417940Z ------------------------------------------
2019-07-17T08:57:05.3418004Z stderr:
---
2019-07-17T08:57:05.3649044Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:05.3649091Z +
2019-07-17T08:57:05.3649117Z 
2019-07-17T08:57:05.3649181Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:05.3649430Z Actual stderr saved to /tmp/compiletestz0Rj1T/regions-mock-trans.stderr
2019-07-17T08:57:05.3649490Z To update references, run this command from build directory:
2019-07-17T08:57:05.3649756Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'regions-mock-trans.rs'
2019-07-17T08:57:05.3649834Z error: 1 errors occurred comparing output.
2019-07-17T08:57:05.3649898Z status: exit code: 1
2019-07-17T08:57:05.3649898Z status: exit code: 1
2019-07-17T08:57:05.3650523Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/regions-mock-trans.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/regions-mock-trans.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/regions-mock-trans.stage-id.aux" "-A" "unused"
2019-07-17T08:57:05.3651143Z ------------------------------------------
2019-07-17T08:57:05.3651177Z 
2019-07-17T08:57:05.3651396Z ------------------------------------------
2019-07-17T08:57:05.3651461Z stderr:
---
2019-07-17T08:57:05.5092153Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:05.5092203Z +
2019-07-17T08:57:05.5092231Z 
2019-07-17T08:57:05.5092277Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:05.5092354Z Actual stderr saved to /tmp/compiletestz0Rj1T/rfc1623.stderr
2019-07-17T08:57:05.5092406Z To update references, run this command from build directory:
2019-07-17T08:57:05.5092666Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'rfc1623.rs'
2019-07-17T08:57:05.5092763Z error: 1 errors occurred comparing output.
2019-07-17T08:57:05.5092808Z status: exit code: 1
2019-07-17T08:57:05.5092808Z status: exit code: 1
2019-07-17T08:57:05.5093453Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/rfc1623.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/rfc1623.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/rfc1623.stage-id.aux" "-A" "unused"
2019-07-17T08:57:05.5093745Z ------------------------------------------
2019-07-17T08:57:05.5093795Z 
2019-07-17T08:57:05.5094014Z ------------------------------------------
2019-07-17T08:57:05.5094060Z stderr:
---
2019-07-17T08:57:05.5493545Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:05.5493747Z +
2019-07-17T08:57:05.5493877Z 
2019-07-17T08:57:05.5494058Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:05.5494620Z Actual stderr saved to /tmp/compiletestz0Rj1T/rust-lang-org.stderr
2019-07-17T08:57:05.5494809Z To update references, run this command from build directory:
2019-07-17T08:57:05.5495724Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'rust-lang-org.rs'
2019-07-17T08:57:05.5496196Z error: 1 errors occurred comparing output.
2019-07-17T08:57:05.5496366Z status: exit code: 1
2019-07-17T08:57:05.5496366Z status: exit code: 1
2019-07-17T08:57:05.5497223Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/rust-lang-org.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/rust-lang-org.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/rust-lang-org.stage-id.aux" "-A" "unused"
2019-07-17T08:57:05.5497858Z ------------------------------------------
2019-07-17T08:57:05.5498040Z 
2019-07-17T08:57:05.5498392Z ------------------------------------------
2019-07-17T08:57:05.5498572Z stderr:
---
2019-07-17T08:57:05.6953083Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:05.6953151Z +
2019-07-17T08:57:05.6953179Z 
2019-07-17T08:57:05.6953233Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:05.6953483Z Actual stderr saved to /tmp/compiletestz0Rj1T/sendable-class.stderr
2019-07-17T08:57:05.6953554Z To update references, run this command from build directory:
2019-07-17T08:57:05.6953806Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'sendable-class.rs'
2019-07-17T08:57:05.6953904Z error: 1 errors occurred comparing output.
2019-07-17T08:57:05.6953951Z status: exit code: 1
2019-07-17T08:57:05.6953951Z status: exit code: 1
2019-07-17T08:57:05.6954670Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/sendable-class.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/sendable-class.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/sendable-class.stage-id.aux" "-A" "unused"
2019-07-17T08:57:05.6956082Z ------------------------------------------
2019-07-17T08:57:05.6956138Z 
2019-07-17T08:57:05.6956417Z ------------------------------------------
2019-07-17T08:57:05.6956464Z stderr:
---
2019-07-17T08:57:05.6966037Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:05.6966084Z +
2019-07-17T08:57:05.6966110Z 
2019-07-17T08:57:05.6966175Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:05.6966423Z Actual stderr saved to /tmp/compiletestz0Rj1T/send-is-not-static-par-for.stderr
2019-07-17T08:57:05.6966478Z To update references, run this command from build directory:
2019-07-17T08:57:05.6966755Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'send-is-not-static-par-for.rs'
2019-07-17T08:57:05.6966836Z error: 1 errors occurred comparing output.
2019-07-17T08:57:05.6966898Z status: exit code: 1
2019-07-17T08:57:05.6966898Z status: exit code: 1
2019-07-17T08:57:05.6967579Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/send-is-not-static-par-for.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/send-is-not-static-par-for.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/send-is-not-static-par-for.stage-id.aux" "-A" "unused"
2019-07-17T08:57:05.6967905Z ------------------------------------------
2019-07-17T08:57:05.6967937Z 
2019-07-17T08:57:05.6968143Z ------------------------------------------
2019-07-17T08:57:05.6968206Z stderr:
---
2019-07-17T08:57:05.8886065Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:05.8886111Z +
2019-07-17T08:57:05.8886139Z 
2019-07-17T08:57:05.8886201Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:05.8886462Z Actual stderr saved to /tmp/compiletestz0Rj1T/simd-intrinsic-generic-elements.stderr
2019-07-17T08:57:05.8886517Z To update references, run this command from build directory:
2019-07-17T08:57:05.8886819Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'simd-intrinsic-generic-elements.rs'
2019-07-17T08:57:05.8886900Z error: 1 errors occurred comparing output.
2019-07-17T08:57:05.8886962Z status: exit code: 1
2019-07-17T08:57:05.8886962Z status: exit code: 1
2019-07-17T08:57:05.8887664Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/simd-intrinsic-generic-elements.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/simd-intrinsic-generic-elements.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/simd-intrinsic-generic-elements.stage-id.aux" "-A" "unused"
2019-07-17T08:57:05.8887995Z ------------------------------------------
2019-07-17T08:57:05.8888028Z 
2019-07-17T08:57:05.8888241Z ------------------------------------------
2019-07-17T08:57:05.8888303Z stderr:
---
2019-07-17T08:57:06.0284029Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:06.0284076Z +
2019-07-17T08:57:06.0284101Z 
2019-07-17T08:57:06.0284162Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:06.0284214Z Actual stderr saved to /tmp/compiletestz0Rj1T/small_enum_size_bug.stderr
2019-07-17T08:57:06.0284263Z To update references, run this command from build directory:
2019-07-17T08:57:06.0284621Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'small_enum_size_bug.rs'
2019-07-17T08:57:06.0284698Z error: 1 errors occurred comparing output.
2019-07-17T08:57:06.0284748Z status: exit code: 1
2019-07-17T08:57:06.0284748Z status: exit code: 1
2019-07-17T08:57:06.0285691Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/small_enum_size_bug.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/small_enum_size_bug.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/small_enum_size_bug.stage-id.aux" "-A" "unused"
2019-07-17T08:57:06.0286031Z ------------------------------------------
2019-07-17T08:57:06.0286064Z 
2019-07-17T08:57:06.0286272Z ------------------------------------------
2019-07-17T08:57:06.0286316Z stderr:
---
2019-07-17T08:57:06.0770699Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:06.0770988Z +
2019-07-17T08:57:06.0771379Z 
2019-07-17T08:57:06.0771597Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:06.0771813Z Actual stderr saved to /tmp/compiletestz0Rj1T/slices.stderr
2019-07-17T08:57:06.0772053Z To update references, run this command from build directory:
2019-07-17T08:57:06.0772555Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'slices.rs'
2019-07-17T08:57:06.0773084Z error: 1 errors occurred comparing output.
2019-07-17T08:57:06.0773310Z status: exit code: 1
2019-07-17T08:57:06.0773310Z status: exit code: 1
2019-07-17T08:57:06.0774184Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/slices.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/slices.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/slices.stage-id.aux" "-A" "unused"
2019-07-17T08:57:06.0777224Z ------------------------------------------
2019-07-17T08:57:06.0777417Z 
2019-07-17T08:57:06.0777784Z ------------------------------------------
2019-07-17T08:57:06.0777981Z stderr:
---
2019-07-17T08:57:06.1809329Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:06.1809501Z +
2019-07-17T08:57:06.1809631Z 
2019-07-17T08:57:06.1809763Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:06.1809894Z Actual stderr saved to /tmp/compiletestz0Rj1T/specialization.stderr
2019-07-17T08:57:06.1810049Z To update references, run this command from build directory:
2019-07-17T08:57:06.1810430Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'specialization.rs'
2019-07-17T08:57:06.1810751Z error: 1 errors occurred comparing output.
2019-07-17T08:57:06.1810876Z status: exit code: 1
2019-07-17T08:57:06.1810876Z status: exit code: 1
2019-07-17T08:57:06.1812106Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/specialization.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/specialization.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/specialization.stage-id.aux" "-A" "unused"
2019-07-17T08:57:06.1812729Z ------------------------------------------
2019-07-17T08:57:06.1812881Z 
2019-07-17T08:57:06.1813223Z ------------------------------------------
2019-07-17T08:57:06.1813435Z stderr:
---
2019-07-17T08:57:06.2490931Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:06.2491007Z +
2019-07-17T08:57:06.2491038Z 
2019-07-17T08:57:06.2491273Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:06.2491571Z Actual stderr saved to /tmp/compiletestz0Rj1T/stacked-borrows/2phase.stderr
2019-07-17T08:57:06.2491639Z To update references, run this command from build directory:
2019-07-17T08:57:06.2491906Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'stacked-borrows/2phase.rs'
2019-07-17T08:57:06.2492015Z error: 1 errors occurred comparing output.
2019-07-17T08:57:06.2492061Z status: exit code: 1
2019-07-17T08:57:06.2492061Z status: exit code: 1
2019-07-17T08:57:06.2492776Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/stacked-borrows/2phase.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/stacked-borrows/2phase.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/stacked-borrows/2phase.stage-id.aux" "-A" "unused"
2019-07-17T08:57:06.2493100Z ------------------------------------------
2019-07-17T08:57:06.2493151Z 
2019-07-17T08:57:06.2493473Z ------------------------------------------
2019-07-17T08:57:06.2493528Z stderr:
---
2019-07-17T08:57:06.3725956Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:06.3726012Z +
2019-07-17T08:57:06.3726039Z 
2019-07-17T08:57:06.3726114Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:06.3726389Z Actual stderr saved to /tmp/compiletestz0Rj1T/stacked-borrows/interior_mutability.stderr
2019-07-17T08:57:06.3726453Z To update references, run this command from build directory:
2019-07-17T08:57:06.3726746Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'stacked-borrows/interior_mutability.rs'
2019-07-17T08:57:06.3726828Z error: 1 errors occurred comparing output.
2019-07-17T08:57:06.3726873Z status: exit code: 1
2019-07-17T08:57:06.3726873Z status: exit code: 1
2019-07-17T08:57:06.3727714Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/stacked-borrows/interior_mutability.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/stacked-borrows/interior_mutability.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/stacked-borrows/interior_mutability.stage-id.aux" "-A" "unused"
2019-07-17T08:57:06.3728098Z ------------------------------------------
2019-07-17T08:57:06.3728134Z 
2019-07-17T08:57:06.3728345Z ------------------------------------------
2019-07-17T08:57:06.3728407Z stderr:
---
2019-07-17T08:57:06.5175265Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:06.5175337Z +
2019-07-17T08:57:06.5175367Z 
2019-07-17T08:57:06.5175414Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:06.5175750Z Actual stderr saved to /tmp/compiletestz0Rj1T/stacked-borrows/stacked-borrows.stderr
2019-07-17T08:57:06.5175809Z To update references, run this command from build directory:
2019-07-17T08:57:06.5176081Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'stacked-borrows/stacked-borrows.rs'
2019-07-17T08:57:06.5176184Z error: 1 errors occurred comparing output.
2019-07-17T08:57:06.5176230Z status: exit code: 1
2019-07-17T08:57:06.5176230Z status: exit code: 1
2019-07-17T08:57:06.5177074Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/stacked-borrows/stacked-borrows.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/stacked-borrows/stacked-borrows.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/stacked-borrows/stacked-borrows.stage-id.aux" "-A" "unused"
2019-07-17T08:57:06.5177460Z ------------------------------------------
2019-07-17T08:57:06.5177496Z 
2019-07-17T08:57:06.5177712Z ------------------------------------------
2019-07-17T08:57:06.5177758Z stderr:
---
2019-07-17T08:57:06.5289301Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:06.5289368Z +
2019-07-17T08:57:06.5289394Z 
2019-07-17T08:57:06.5289436Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:06.5289486Z Actual stderr saved to /tmp/compiletestz0Rj1T/static_memory_modification.stderr
2019-07-17T08:57:06.5289556Z To update references, run this command from build directory:
2019-07-17T08:57:06.5289811Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'static_memory_modification.rs'
2019-07-17T08:57:06.5289907Z error: 1 errors occurred comparing output.
2019-07-17T08:57:06.5289949Z status: exit code: 1
2019-07-17T08:57:06.5289949Z status: exit code: 1
2019-07-17T08:57:06.5290866Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/static_memory_modification.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/static_memory_modification.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/static_memory_modification.stage-id.aux" "-A" "unused"
2019-07-17T08:57:06.5291240Z ------------------------------------------
2019-07-17T08:57:06.5291293Z 
2019-07-17T08:57:06.5291500Z ------------------------------------------
2019-07-17T08:57:06.5291543Z stderr:
---
2019-07-17T08:57:06.6846538Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:06.6846610Z +
2019-07-17T08:57:06.6846639Z 
2019-07-17T08:57:06.6846685Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:06.6846757Z Actual stderr saved to /tmp/compiletestz0Rj1T/static_mut.stderr
2019-07-17T08:57:06.6846808Z To update references, run this command from build directory:
2019-07-17T08:57:06.6847074Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'static_mut.rs'
2019-07-17T08:57:06.6847348Z error: 1 errors occurred comparing output.
2019-07-17T08:57:06.6847411Z status: exit code: 1
2019-07-17T08:57:06.6847411Z status: exit code: 1
2019-07-17T08:57:06.6848092Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/static_mut.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/static_mut.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/static_mut.stage-id.aux" "-A" "unused"
2019-07-17T08:57:06.6848415Z ------------------------------------------
2019-07-17T08:57:06.6848467Z 
2019-07-17T08:57:06.6848681Z ------------------------------------------
2019-07-17T08:57:06.6848726Z stderr:
---
2019-07-17T08:57:06.7127702Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:06.7127751Z +
2019-07-17T08:57:06.7127780Z 
2019-07-17T08:57:06.7127844Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:06.7128043Z Actual stderr saved to /tmp/compiletestz0Rj1T/strings.stderr
2019-07-17T08:57:06.7128108Z To update references, run this command from build directory:
2019-07-17T08:57:06.7128430Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'strings.rs'
2019-07-17T08:57:06.7128510Z error: 1 errors occurred comparing output.
2019-07-17T08:57:06.7128555Z status: exit code: 1
2019-07-17T08:57:06.7128555Z status: exit code: 1
2019-07-17T08:57:06.7129353Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/strings.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/strings.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/strings.stage-id.aux" "-A" "unused"
2019-07-17T08:57:06.7129655Z ------------------------------------------
2019-07-17T08:57:06.7129688Z 
2019-07-17T08:57:06.7129899Z ------------------------------------------
2019-07-17T08:57:06.7129943Z stderr:
---
2019-07-17T08:57:06.8613614Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:06.8613919Z +
2019-07-17T08:57:06.8614112Z 
2019-07-17T08:57:06.8614339Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:06.8614552Z Actual stderr saved to /tmp/compiletestz0Rj1T/subslice_array.stderr
2019-07-17T08:57:06.8614762Z To update references, run this command from build directory:
2019-07-17T08:57:06.8616657Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'subslice_array.rs'
2019-07-17T08:57:06.8617266Z error: 1 errors occurred comparing output.
2019-07-17T08:57:06.8621055Z status: exit code: 1
2019-07-17T08:57:06.8621055Z status: exit code: 1
2019-07-17T08:57:06.8622301Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/subslice_array.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/subslice_array.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/subslice_array.stage-id.aux" "-A" "unused"
2019-07-17T08:57:06.8622970Z ------------------------------------------
2019-07-17T08:57:06.8623135Z 
2019-07-17T08:57:06.8623655Z ------------------------------------------
2019-07-17T08:57:06.8623901Z stderr:
---
2019-07-17T08:57:06.9269890Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:06.9269940Z +
2019-07-17T08:57:06.9269990Z 
2019-07-17T08:57:06.9270039Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:06.9270090Z Actual stderr saved to /tmp/compiletestz0Rj1T/sums.stderr
2019-07-17T08:57:06.9270486Z To update references, run this command from build directory:
2019-07-17T08:57:06.9270974Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'sums.rs'
2019-07-17T08:57:06.9271055Z error: 1 errors occurred comparing output.
2019-07-17T08:57:06.9271241Z status: exit code: 1
2019-07-17T08:57:06.9271241Z status: exit code: 1
2019-07-17T08:57:06.9271861Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/sums.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/sums.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/sums.stage-id.aux" "-A" "unused"
2019-07-17T08:57:06.9272279Z ------------------------------------------
2019-07-17T08:57:06.9272311Z 
2019-07-17T08:57:06.9272528Z ------------------------------------------
2019-07-17T08:57:06.9272595Z stderr:
---
2019-07-17T08:57:07.0210163Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:07.0210364Z +
2019-07-17T08:57:07.0210405Z 
2019-07-17T08:57:07.0210453Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:07.0210535Z Actual stderr saved to /tmp/compiletestz0Rj1T/sync.stderr
2019-07-17T08:57:07.0210586Z To update references, run this command from build directory:
2019-07-17T08:57:07.0210871Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'sync.rs'
2019-07-17T08:57:07.0210972Z error: 1 errors occurred comparing output.
2019-07-17T08:57:07.0211019Z status: exit code: 1
2019-07-17T08:57:07.0211019Z status: exit code: 1
2019-07-17T08:57:07.0211662Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/sync.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/sync.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/sync.stage-id.aux" "-A" "unused"
2019-07-17T08:57:07.0211965Z ------------------------------------------
2019-07-17T08:57:07.0212113Z 
2019-07-17T08:57:07.0212357Z ------------------------------------------
2019-07-17T08:57:07.0212404Z stderr:
---
2019-07-17T08:57:07.0797025Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:07.0797081Z +
2019-07-17T08:57:07.0797124Z 
2019-07-17T08:57:07.0797176Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:07.0797445Z Actual stderr saved to /tmp/compiletestz0Rj1T/tag-align-dyn-u64.stderr
2019-07-17T08:57:07.0797515Z To update references, run this command from build directory:
2019-07-17T08:57:07.0797761Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'tag-align-dyn-u64.rs'
2019-07-17T08:57:07.0797837Z error: 1 errors occurred comparing output.
2019-07-17T08:57:07.0797898Z status: exit code: 1
2019-07-17T08:57:07.0797898Z status: exit code: 1
2019-07-17T08:57:07.0798532Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/tag-align-dyn-u64.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/tag-align-dyn-u64.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/tag-align-dyn-u64.stage-id.aux" "-A" "unused"
2019-07-17T08:57:07.0798935Z ------------------------------------------
2019-07-17T08:57:07.0798967Z 
2019-07-17T08:57:07.0799186Z ------------------------------------------
2019-07-17T08:57:07.0799230Z stderr:
---
2019-07-17T08:57:07.2129849Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:07.2129920Z +
2019-07-17T08:57:07.2129947Z 
2019-07-17T08:57:07.2130014Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:07.2130331Z Actual stderr saved to /tmp/compiletestz0Rj1T/too-large-primval-write-problem.stderr
2019-07-17T08:57:07.2130388Z To update references, run this command from build directory:
2019-07-17T08:57:07.2130675Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'too-large-primval-write-problem.rs'
2019-07-17T08:57:07.2130757Z error: 1 errors occurred comparing output.
2019-07-17T08:57:07.2130902Z status: exit code: 1
2019-07-17T08:57:07.2130902Z status: exit code: 1
2019-07-17T08:57:07.2132991Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/too-large-primval-write-problem.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/too-large-primval-write-problem.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/too-large-primval-write-problem.stage-id.aux" "-A" "unused"
2019-07-17T08:57:07.2133550Z ------------------------------------------
2019-07-17T08:57:07.2133585Z 
2019-07-17T08:57:07.2133801Z ------------------------------------------
2019-07-17T08:57:07.2133866Z stderr:
---
2019-07-17T08:57:07.2179862Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:07.2179931Z +
2019-07-17T08:57:07.2179959Z 
2019-07-17T08:57:07.2180005Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:07.2180267Z Actual stderr saved to /tmp/compiletestz0Rj1T/thread-local.stderr
2019-07-17T08:57:07.2180339Z To update references, run this command from build directory:
2019-07-17T08:57:07.2180778Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'thread-local.rs'
2019-07-17T08:57:07.2180880Z error: 1 errors occurred comparing output.
2019-07-17T08:57:07.2180927Z status: exit code: 1
2019-07-17T08:57:07.2180927Z status: exit code: 1
2019-07-17T08:57:07.2181592Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/thread-local.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/thread-local.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/thread-local.stage-id.aux" "-A" "unused"
2019-07-17T08:57:07.2182060Z ------------------------------------------
2019-07-17T08:57:07.2182096Z 
2019-07-17T08:57:07.2182347Z ------------------------------------------
2019-07-17T08:57:07.2182394Z stderr:
---
2019-07-17T08:57:07.3773398Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:07.3773446Z +
2019-07-17T08:57:07.3773497Z 
2019-07-17T08:57:07.3773544Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:07.3773597Z Actual stderr saved to /tmp/compiletestz0Rj1T/transmute_fat.stderr
2019-07-17T08:57:07.3773668Z To update references, run this command from build directory:
2019-07-17T08:57:07.3773927Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'transmute_fat.rs'
2019-07-17T08:57:07.3774006Z error: 1 errors occurred comparing output.
2019-07-17T08:57:07.3774071Z status: exit code: 1
2019-07-17T08:57:07.3774071Z status: exit code: 1
2019-07-17T08:57:07.3774908Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/transmute_fat.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/transmute_fat.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-Zmiri-disable-validation" "-L" "/tmp/compiletestz0Rj1T/transmute_fat.stage-id.aux" "-A" "unused"
2019-07-17T08:57:07.3775911Z ------------------------------------------
2019-07-17T08:57:07.3775952Z 
2019-07-17T08:57:07.3776204Z ------------------------------------------
2019-07-17T08:57:07.3776251Z stderr:
---
2019-07-17T08:57:07.4082889Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:07.4082939Z +
2019-07-17T08:57:07.4082967Z 
2019-07-17T08:57:07.4083014Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:07.4083084Z Actual stderr saved to /tmp/compiletestz0Rj1T/traits.stderr
2019-07-17T08:57:07.4083137Z To update references, run this command from build directory:
2019-07-17T08:57:07.4083408Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'traits.rs'
2019-07-17T08:57:07.4083510Z error: 1 errors occurred comparing output.
2019-07-17T08:57:07.4083557Z status: exit code: 1
2019-07-17T08:57:07.4083557Z status: exit code: 1
2019-07-17T08:57:07.4084220Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/traits.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/traits.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/traits.stage-id.aux" "-A" "unused"
2019-07-17T08:57:07.4084810Z ------------------------------------------
2019-07-17T08:57:07.4084865Z 
2019-07-17T08:57:07.4085076Z ------------------------------------------
2019-07-17T08:57:07.4085121Z stderr:
---
2019-07-17T08:57:07.5290347Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:07.5290565Z +
2019-07-17T08:57:07.5290687Z 
2019-07-17T08:57:07.5290825Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:07.5291280Z Actual stderr saved to /tmp/compiletestz0Rj1T/trivial.stderr
2019-07-17T08:57:07.5291453Z To update references, run this command from build directory:
2019-07-17T08:57:07.5291864Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'trivial.rs'
2019-07-17T08:57:07.5292199Z error: 1 errors occurred comparing output.
2019-07-17T08:57:07.5292349Z status: exit code: 1
2019-07-17T08:57:07.5292349Z status: exit code: 1
2019-07-17T08:57:07.5293153Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/trivial.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/trivial.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/trivial.stage-id.aux" "-A" "unused"
2019-07-17T08:57:07.5293947Z ------------------------------------------
2019-07-17T08:57:07.5294119Z 
2019-07-17T08:57:07.5294477Z ------------------------------------------
2019-07-17T08:57:07.5294836Z stderr:
---
2019-07-17T08:57:07.5577317Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:07.5577384Z +
2019-07-17T08:57:07.5577411Z 
2019-07-17T08:57:07.5577457Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:07.5577729Z Actual stderr saved to /tmp/compiletestz0Rj1T/try-operator-custom.stderr
2019-07-17T08:57:07.5577792Z To update references, run this command from build directory:
2019-07-17T08:57:07.5578050Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'try-operator-custom.rs'
2019-07-17T08:57:07.5578233Z error: 1 errors occurred comparing output.
2019-07-17T08:57:07.5578278Z status: exit code: 1
2019-07-17T08:57:07.5578278Z status: exit code: 1
2019-07-17T08:57:07.5578964Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/try-operator-custom.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/try-operator-custom.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/try-operator-custom.stage-id.aux" "-A" "unused"
2019-07-17T08:57:07.5579281Z ------------------------------------------
2019-07-17T08:57:07.5579330Z 
2019-07-17T08:57:07.5579542Z ------------------------------------------
2019-07-17T08:57:07.5579594Z stderr:
---
2019-07-17T08:57:07.6910293Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:07.6910357Z +
2019-07-17T08:57:07.6910384Z 
2019-07-17T08:57:07.6910427Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:07.6910487Z Actual stderr saved to /tmp/compiletestz0Rj1T/tuple_like_enum_variant_constructor.stderr
2019-07-17T08:57:07.6910627Z To update references, run this command from build directory:
2019-07-17T08:57:07.6910928Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'tuple_like_enum_variant_constructor.rs'
2019-07-17T08:57:07.6911027Z error: 1 errors occurred comparing output.
2019-07-17T08:57:07.6911074Z status: exit code: 1
2019-07-17T08:57:07.6911074Z status: exit code: 1
2019-07-17T08:57:07.6912006Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/tuple_like_enum_variant_constructor.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/tuple_like_enum_variant_constructor.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/tuple_like_enum_variant_constructor.stage-id.aux" "-A" "unused"
2019-07-17T08:57:07.6912334Z ------------------------------------------
2019-07-17T08:57:07.6912377Z 
2019-07-17T08:57:07.6912609Z ------------------------------------------
2019-07-17T08:57:07.6912655Z stderr:
---
2019-07-17T08:57:07.7444902Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:07.7444952Z +
2019-07-17T08:57:07.7444997Z 
2019-07-17T08:57:07.7445053Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:07.7445110Z Actual stderr saved to /tmp/compiletestz0Rj1T/tuple_like_enum_variant_constructor_pointer_opt.stderr
2019-07-17T08:57:07.7445265Z To update references, run this command from build directory:
2019-07-17T08:57:07.7446083Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'tuple_like_enum_variant_constructor_pointer_opt.rs'
2019-07-17T08:57:07.7446226Z error: 1 errors occurred comparing output.
2019-07-17T08:57:07.7446290Z status: exit code: 1
2019-07-17T08:57:07.7446290Z status: exit code: 1
2019-07-17T08:57:07.7447082Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/tuple_like_enum_variant_constructor_pointer_opt.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/tuple_like_enum_variant_constructor_pointer_opt.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/tuple_like_enum_variant_constructor_pointer_opt.stage-id.aux" "-A" "unused"
2019-07-17T08:57:07.7447426Z ------------------------------------------
2019-07-17T08:57:07.7447460Z 
2019-07-17T08:57:07.7447686Z ------------------------------------------
2019-07-17T08:57:07.7447732Z stderr:
---
2019-07-17T08:57:07.9177999Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:07.9178065Z +
2019-07-17T08:57:07.9178092Z 
2019-07-17T08:57:07.9178140Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:07.9178196Z Actual stderr saved to /tmp/compiletestz0Rj1T/tuple_like_enum_variant_constructor_struct_pointer_opt.stderr
2019-07-17T08:57:07.9178268Z To update references, run this command from build directory:
2019-07-17T08:57:07.9178578Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'tuple_like_enum_variant_constructor_struct_pointer_opt.rs'
2019-07-17T08:57:07.9178678Z error: 1 errors occurred comparing output.
2019-07-17T08:57:07.9178725Z status: exit code: 1
2019-07-17T08:57:07.9178725Z status: exit code: 1
2019-07-17T08:57:07.9179480Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/tuple_like_enum_variant_constructor_struct_pointer_opt.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/tuple_like_enum_variant_constructor_struct_pointer_opt.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/tuple_like_enum_variant_constructor_struct_pointer_opt.stage-id.aux" "-A" "unused"
2019-07-17T08:57:07.9179824Z ------------------------------------------
2019-07-17T08:57:07.9179872Z 
2019-07-17T08:57:07.9180096Z ------------------------------------------
2019-07-17T08:57:07.9180143Z stderr:
---
2019-07-17T08:57:07.9190646Z ------------------------------------------
2019-07-17T08:57:07.9190679Z 
2019-07-17T08:57:07.9191162Z 
2019-07-17T08:57:07.9191210Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:07.9191264Z Actual stderr saved to /tmp/compiletestz0Rj1T/tuple_like_struct_constructor.stderr
2019-07-17T08:57:07.9191333Z To update references, run this command from build directory:
2019-07-17T08:57:07.9191614Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'tuple_like_struct_constructor.rs'
2019-07-17T08:57:07.9191694Z error: 1 errors occurred comparing output.
2019-07-17T08:57:07.9191756Z status: exit code: 1
2019-07-17T08:57:07.9191756Z status: exit code: 1
2019-07-17T08:57:07.9192454Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/tuple_like_struct_constructor.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/tuple_like_struct_constructor.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/tuple_like_struct_constructor.stage-id.aux" "-A" "unused"
2019-07-17T08:57:07.9192782Z ------------------------------------------
2019-07-17T08:57:07.9192814Z 
2019-07-17T08:57:07.9193036Z ------------------------------------------
2019-07-17T08:57:07.9193082Z stderr:
---
2019-07-17T08:57:08.0952093Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:08.0952162Z +
2019-07-17T08:57:08.0952192Z 
2019-07-17T08:57:08.0952240Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:08.0952515Z Actual stderr saved to /tmp/compiletestz0Rj1T/union-overwrite.stderr
2019-07-17T08:57:08.0952589Z To update references, run this command from build directory:
2019-07-17T08:57:08.0952871Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'union-overwrite.rs'
2019-07-17T08:57:08.0952973Z error: 1 errors occurred comparing output.
2019-07-17T08:57:08.0953021Z status: exit code: 1
2019-07-17T08:57:08.0953021Z status: exit code: 1
2019-07-17T08:57:08.0953680Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/union-overwrite.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/union-overwrite.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/union-overwrite.stage-id.aux" "-A" "unused"
2019-07-17T08:57:08.0954022Z ------------------------------------------
2019-07-17T08:57:08.0954066Z 
2019-07-17T08:57:08.0954318Z ------------------------------------------
2019-07-17T08:57:08.0954373Z stderr:
---
2019-07-17T08:57:08.2693162Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:08.2693224Z +
2019-07-17T08:57:08.2693274Z 
2019-07-17T08:57:08.2693324Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:08.2693376Z Actual stderr saved to /tmp/compiletestz0Rj1T/union.stderr
2019-07-17T08:57:08.2693449Z To update references, run this command from build directory:
2019-07-17T08:57:08.2693732Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'union.rs'
2019-07-17T08:57:08.2693816Z error: 1 errors occurred comparing output.
2019-07-17T08:57:08.2693881Z status: exit code: 1
2019-07-17T08:57:08.2693881Z status: exit code: 1
2019-07-17T08:57:08.2694686Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/union.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/union.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/union.stage-id.aux" "-A" "unused"
2019-07-17T08:57:08.2695015Z ------------------------------------------
2019-07-17T08:57:08.2695048Z 
2019-07-17T08:57:08.2695256Z ------------------------------------------
2019-07-17T08:57:08.2695326Z stderr:
---
2019-07-17T08:57:08.3326773Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:08.3326834Z +
2019-07-17T08:57:08.3326862Z 
2019-07-17T08:57:08.3326910Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:08.3326982Z Actual stderr saved to /tmp/compiletestz0Rj1T/u128.stderr
2019-07-17T08:57:08.3327036Z To update references, run this command from build directory:
2019-07-17T08:57:08.3327303Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'u128.rs'
2019-07-17T08:57:08.3327404Z error: 1 errors occurred comparing output.
2019-07-17T08:57:08.3327453Z status: exit code: 1
2019-07-17T08:57:08.3327453Z status: exit code: 1
2019-07-17T08:57:08.3328111Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/u128.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/u128.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/u128.stage-id.aux" "-A" "unused"
2019-07-17T08:57:08.3328440Z ------------------------------------------
2019-07-17T08:57:08.3328494Z 
2019-07-17T08:57:08.3328726Z ------------------------------------------
2019-07-17T08:57:08.3328774Z stderr:
---
2019-07-17T08:57:08.4451154Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:08.4451230Z +
2019-07-17T08:57:08.4451258Z 
2019-07-17T08:57:08.4451408Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:08.4451459Z Actual stderr saved to /tmp/compiletestz0Rj1T/unops.stderr
2019-07-17T08:57:08.4451528Z To update references, run this command from build directory:
2019-07-17T08:57:08.4451780Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'unops.rs'
2019-07-17T08:57:08.4451876Z error: 1 errors occurred comparing output.
2019-07-17T08:57:08.4451923Z status: exit code: 1
2019-07-17T08:57:08.4451923Z status: exit code: 1
2019-07-17T08:57:08.4452547Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/unops.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/unops.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/unops.stage-id.aux" "-A" "unused"
2019-07-17T08:57:08.4452874Z ------------------------------------------
2019-07-17T08:57:08.4452907Z 
2019-07-17T08:57:08.4453131Z ------------------------------------------
2019-07-17T08:57:08.4453177Z stderr:
---
2019-07-17T08:57:08.5167793Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:08.5167843Z +
2019-07-17T08:57:08.5167871Z 
2019-07-17T08:57:08.5167935Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:08.5168204Z Actual stderr saved to /tmp/compiletestz0Rj1T/unsized-tuple-impls.stderr
2019-07-17T08:57:08.5168262Z To update references, run this command from build directory:
2019-07-17T08:57:08.5168555Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'unsized-tuple-impls.rs'
2019-07-17T08:57:08.5168639Z error: 1 errors occurred comparing output.
2019-07-17T08:57:08.5168732Z status: exit code: 1
2019-07-17T08:57:08.5168732Z status: exit code: 1
2019-07-17T08:57:08.5170452Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/unsized-tuple-impls.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/unsized-tuple-impls.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/unsized-tuple-impls.stage-id.aux" "-A" "unused"
2019-07-17T08:57:08.5170797Z ------------------------------------------
2019-07-17T08:57:08.5170830Z 
2019-07-17T08:57:08.5171222Z ------------------------------------------
2019-07-17T08:57:08.5171287Z stderr:
---
2019-07-17T08:57:08.5929586Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:08.5929631Z +
2019-07-17T08:57:08.5929659Z 
2019-07-17T08:57:08.5929721Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:08.5929773Z Actual stderr saved to /tmp/compiletestz0Rj1T/validation_lifetime_resolution.stderr
2019-07-17T08:57:08.5929823Z To update references, run this command from build directory:
2019-07-17T08:57:08.5930103Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'validation_lifetime_resolution.rs'
2019-07-17T08:57:08.5930182Z error: 1 errors occurred comparing output.
2019-07-17T08:57:08.5930242Z status: exit code: 1
2019-07-17T08:57:08.5930242Z status: exit code: 1
2019-07-17T08:57:08.5931094Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/validation_lifetime_resolution.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/validation_lifetime_resolution.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/validation_lifetime_resolution.stage-id.aux" "-A" "unused"
2019-07-17T08:57:08.5931425Z ------------------------------------------
2019-07-17T08:57:08.5931458Z 
2019-07-17T08:57:08.5931664Z ------------------------------------------
2019-07-17T08:57:08.5931727Z stderr:
---
2019-07-17T08:57:08.7126394Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:08.7126464Z +
2019-07-17T08:57:08.7126492Z 
2019-07-17T08:57:08.7126539Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:08.7126832Z Actual stderr saved to /tmp/compiletestz0Rj1T/vec-matching-fold.stderr
2019-07-17T08:57:08.7126892Z To update references, run this command from build directory:
2019-07-17T08:57:08.7127171Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'vec-matching-fold.rs'
2019-07-17T08:57:08.7127278Z error: 1 errors occurred comparing output.
2019-07-17T08:57:08.7127334Z status: exit code: 1
2019-07-17T08:57:08.7127334Z status: exit code: 1
2019-07-17T08:57:08.7128010Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/vec-matching-fold.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/vec-matching-fold.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/vec-matching-fold.stage-id.aux" "-A" "unused"
2019-07-17T08:57:08.7128352Z ------------------------------------------
2019-07-17T08:57:08.7128440Z 
2019-07-17T08:57:08.7128677Z ------------------------------------------
2019-07-17T08:57:08.7128725Z stderr:
---
2019-07-17T08:57:08.8013137Z -Iter([], [])
2019-07-17T08:57:08.8013340Z -
2019-07-17T08:57:08.8019483Z 
2019-07-17T08:57:08.8019622Z The actual stdout differed from the expected stdout.
2019-07-17T08:57:08.8024963Z Actual stdout saved to /tmp/compiletestz0Rj1T/vecdeque.stdout
2019-07-17T08:57:08.8048103Z error[E0080]: Miri evaluation error: attempted to do invalid arithmetic on pointers that would leak base addresses, e.g., comparing pointers into different allocations
2019-07-17T08:57:08.8048609Z     --> /checkout/src/libcore/intrinsics.rs:1340:19
2019-07-17T08:57:08.8048671Z      |
2019-07-17T08:57:08.8048719Z 1340 |     let diff = if src_usize > dst_usize {
---
2019-07-17T08:57:08.8052910Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:08.8052959Z +
2019-07-17T08:57:08.8053006Z 
2019-07-17T08:57:08.8053054Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:08.8053107Z Actual stderr saved to /tmp/compiletestz0Rj1T/vecdeque.stderr
2019-07-17T08:57:08.8053159Z To update references, run this command from build directory:
2019-07-17T08:57:08.8053454Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'vecdeque.rs'
2019-07-17T08:57:08.8053535Z error: 2 errors occurred comparing output.
2019-07-17T08:57:08.8053602Z status: exit code: 1
2019-07-17T08:57:08.8053602Z status: exit code: 1
2019-07-17T08:57:08.8054326Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/vecdeque.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/vecdeque.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/vecdeque.stage-id.aux" "-A" "unused"
2019-07-17T08:57:08.8054709Z ------------------------------------------
2019-07-17T08:57:08.8054745Z 
2019-07-17T08:57:08.8055657Z ------------------------------------------
2019-07-17T08:57:08.8055730Z stderr:
---
2019-07-17T08:57:08.9732106Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:08.9732162Z +
2019-07-17T08:57:08.9732190Z 
2019-07-17T08:57:08.9732238Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:08.9732311Z Actual stderr saved to /tmp/compiletestz0Rj1T/volatile.stderr
2019-07-17T08:57:08.9732366Z To update references, run this command from build directory:
2019-07-17T08:57:08.9732665Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'volatile.rs'
2019-07-17T08:57:08.9732771Z error: 1 errors occurred comparing output.
2019-07-17T08:57:08.9732818Z status: exit code: 1
2019-07-17T08:57:08.9732818Z status: exit code: 1
2019-07-17T08:57:08.9733634Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/volatile.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/volatile.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/volatile.stage-id.aux" "-A" "unused"
2019-07-17T08:57:08.9734153Z ------------------------------------------
2019-07-17T08:57:08.9734209Z 
2019-07-17T08:57:08.9734447Z ------------------------------------------
2019-07-17T08:57:08.9734494Z stderr:
---
2019-07-17T08:57:09.0046404Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:09.0046451Z +
2019-07-17T08:57:09.0046496Z 
2019-07-17T08:57:09.0046542Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:09.0046593Z Actual stderr saved to /tmp/compiletestz0Rj1T/vecs.stderr
2019-07-17T08:57:09.0046643Z To update references, run this command from build directory:
2019-07-17T08:57:09.0046915Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'vecs.rs'
2019-07-17T08:57:09.0046993Z error: 1 errors occurred comparing output.
2019-07-17T08:57:09.0047055Z status: exit code: 1
2019-07-17T08:57:09.0047055Z status: exit code: 1
2019-07-17T08:57:09.0047769Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/vecs.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/vecs.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/vecs.stage-id.aux" "-A" "unused"
2019-07-17T08:57:09.0048130Z ------------------------------------------
2019-07-17T08:57:09.0048164Z 
2019-07-17T08:57:09.0048373Z ------------------------------------------
2019-07-17T08:57:09.0048437Z stderr:
---
2019-07-17T08:57:09.1447700Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:09.1447756Z +
2019-07-17T08:57:09.1447784Z 
2019-07-17T08:57:09.1447855Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:09.1448150Z Actual stderr saved to /tmp/compiletestz0Rj1T/without-validation.stderr
2019-07-17T08:57:09.1448205Z To update references, run this command from build directory:
2019-07-17T08:57:09.1448512Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'without-validation.rs'
2019-07-17T08:57:09.1448598Z error: 1 errors occurred comparing output.
2019-07-17T08:57:09.1448645Z status: exit code: 1
2019-07-17T08:57:09.1448645Z status: exit code: 1
2019-07-17T08:57:09.1449639Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/without-validation.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/without-validation.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-Zmiri-disable-validation" "-L" "/tmp/compiletestz0Rj1T/without-validation.stage-id.aux" "-A" "unused"
2019-07-17T08:57:09.1450009Z ------------------------------------------
2019-07-17T08:57:09.1450042Z 
2019-07-17T08:57:09.1450241Z ------------------------------------------
2019-07-17T08:57:09.1450304Z stderr:
---
2019-07-17T08:57:09.2163561Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:09.2163630Z +
2019-07-17T08:57:09.2163658Z 
2019-07-17T08:57:09.2163706Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:09.2163988Z Actual stderr saved to /tmp/compiletestz0Rj1T/write-bytes.stderr
2019-07-17T08:57:09.2164045Z To update references, run this command from build directory:
2019-07-17T08:57:09.2164322Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'write-bytes.rs'
2019-07-17T08:57:09.2164503Z error: 1 errors occurred comparing output.
2019-07-17T08:57:09.2164559Z status: exit code: 1
2019-07-17T08:57:09.2164559Z status: exit code: 1
2019-07-17T08:57:09.2168213Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/write-bytes.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/write-bytes.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/write-bytes.stage-id.aux" "-A" "unused"
2019-07-17T08:57:09.2168572Z ------------------------------------------
2019-07-17T08:57:09.2168606Z 
2019-07-17T08:57:09.2168952Z ------------------------------------------
2019-07-17T08:57:09.2168998Z stderr:
---
2019-07-17T08:57:09.3454175Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:09.3454239Z +
2019-07-17T08:57:09.3454266Z 
2019-07-17T08:57:09.3454311Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:09.3454721Z Actual stderr saved to /tmp/compiletestz0Rj1T/zero-sized-binary-heap-push.stderr
2019-07-17T08:57:09.3454967Z To update references, run this command from build directory:
2019-07-17T08:57:09.3455266Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'zero-sized-binary-heap-push.rs'
2019-07-17T08:57:09.3455359Z error: 1 errors occurred comparing output.
2019-07-17T08:57:09.3455405Z status: exit code: 1
2019-07-17T08:57:09.3455405Z status: exit code: 1
2019-07-17T08:57:09.3456513Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/zero-sized-binary-heap-push.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/zero-sized-binary-heap-push.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/zero-sized-binary-heap-push.stage-id.aux" "-A" "unused"
2019-07-17T08:57:09.3456860Z ------------------------------------------
2019-07-17T08:57:09.3456894Z 
2019-07-17T08:57:09.3457127Z ------------------------------------------
2019-07-17T08:57:09.3457296Z stderr:
---
2019-07-17T08:57:09.3884936Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:09.3884983Z +
2019-07-17T08:57:09.3885008Z 
2019-07-17T08:57:09.3885049Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:09.3885114Z Actual stderr saved to /tmp/compiletestz0Rj1T/zst.stderr
2019-07-17T08:57:09.3885235Z To update references, run this command from build directory:
2019-07-17T08:57:09.3885914Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'zst.rs'
2019-07-17T08:57:09.3886022Z error: 1 errors occurred comparing output.
2019-07-17T08:57:09.3886067Z status: exit code: 1
2019-07-17T08:57:09.3886067Z status: exit code: 1
2019-07-17T08:57:09.3886707Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/zst.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/zst.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/zst.stage-id.aux" "-A" "unused"
2019-07-17T08:57:09.3887007Z ------------------------------------------
2019-07-17T08:57:09.3887059Z 
2019-07-17T08:57:09.3887271Z ------------------------------------------
2019-07-17T08:57:09.3887325Z stderr:
---
2019-07-17T08:57:09.4886398Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:09.4886478Z +
2019-07-17T08:57:09.4886508Z 
2019-07-17T08:57:09.4886554Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:09.4886753Z Actual stderr saved to /tmp/compiletestz0Rj1T/zst_box.stderr
2019-07-17T08:57:09.4886836Z To update references, run this command from build directory:
2019-07-17T08:57:09.4887140Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'zst_box.rs'
2019-07-17T08:57:09.4887240Z error: 1 errors occurred comparing output.
2019-07-17T08:57:09.4887287Z status: exit code: 1
2019-07-17T08:57:09.4887287Z status: exit code: 1
2019-07-17T08:57:09.4887917Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/zst_box.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/zst_box.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/zst_box.stage-id.aux" "-A" "unused"
2019-07-17T08:57:09.4888236Z ------------------------------------------
2019-07-17T08:57:09.4888270Z 
2019-07-17T08:57:09.4888509Z ------------------------------------------
2019-07-17T08:57:09.4888555Z stderr:
---
2019-07-17T08:57:09.5271387Z +For more information about this error, try `rustc --explain E0080`.
2019-07-17T08:57:09.5271433Z +
2019-07-17T08:57:09.5271458Z 
2019-07-17T08:57:09.5271502Z The actual stderr differed from the expected stderr.
2019-07-17T08:57:09.5271569Z Actual stderr saved to /tmp/compiletestz0Rj1T/zst_variant_drop.stderr
2019-07-17T08:57:09.5271619Z To update references, run this command from build directory:
2019-07-17T08:57:09.5271868Z tests/run-pass/update-references.sh '/tmp/compiletestz0Rj1T' 'zst_variant_drop.rs'
2019-07-17T08:57:09.5271964Z error: 1 errors occurred comparing output.
2019-07-17T08:57:09.5272007Z status: exit code: 1
2019-07-17T08:57:09.5272007Z status: exit code: 1
2019-07-17T08:57:09.5272659Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/zst_variant_drop.rs" "-L" "/tmp/compiletestz0Rj1T" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestz0Rj1T/zst_variant_drop.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestz0Rj1T/zst_variant_drop.stage-id.aux" "-A" "unused"
2019-07-17T08:57:09.5273066Z ------------------------------------------
2019-07-17T08:57:09.5273101Z 
2019-07-17T08:57:09.5273305Z ------------------------------------------
2019-07-17T08:57:09.5273347Z stderr:
---
2019-07-17T08:57:09.5559080Z Verifying status of clippy-driver...
2019-07-17T08:57:09.5571421Z Verifying status of miri...
2019-07-17T08:57:09.5584251Z Verifying status of embedded-book...
2019-07-17T08:57:09.5598294Z Verifying status of rustc-guide...
2019-07-17T08:57:09.5690247Z /tmp/checktools.sh: 38: /tmp/checktools.sh: TOOLSTATE_REPO: parameter not set
2019-07-17T08:57:10.1694030Z ##[error]Bash exited with code '2'.
2019-07-17T08:57:10.1730406Z ##[section]Starting: Checkout
2019-07-17T08:57:10.1732522Z ==============================================================================
2019-07-17T08:57:10.1732577Z Task         : Get sources
2019-07-17T08:57:10.1732623Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
