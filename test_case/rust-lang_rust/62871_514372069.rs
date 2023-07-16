plain
2019-07-23T19:19:35.4158091Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-23T19:19:35.4364851Z ##[command]git config gc.auto 0
2019-07-23T19:19:35.4453038Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-23T19:19:35.4514395Z ##[command]git config --get-all http.proxy
2019-07-23T19:19:35.4671104Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62871/merge:refs/remotes/pull/62871/merge
---
2019-07-23T19:20:08.3093803Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-23T19:20:08.3111533Z 
2019-07-23T19:20:08.3112594Z   git checkout -b <new-branch-name>
2019-07-23T19:20:08.3113030Z 
2019-07-23T19:20:08.3113393Z HEAD is now at 76428ee52 Merge b7910c99e261f11321bb623013e56d720dd80b9e into 3ebca72a11869f946b31f900faffb75c2bb2473a
2019-07-23T19:20:08.3283399Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-23T19:20:08.3286517Z ==============================================================================
2019-07-23T19:20:08.3286583Z Task         : Bash
2019-07-23T19:20:08.3286636Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-23T19:23:09.8152121Z Reading package lists...
2019-07-23T19:23:09.9747833Z Building dependency tree...
2019-07-23T19:23:09.9748075Z Reading state information...
2019-07-23T19:23:10.0898815Z The following additional packages will be installed:
2019-07-23T19:23:10.0899827Z   binfmt-support binutils bzip2 cmake-data cpp cpp-5 dpkg-dev g++-5 gcc gcc-5
2019-07-23T19:23:10.0900429Z   libbabeltrace-ctf1 libbabeltrace1 libbsd-dev libbsd0 libc-dev-bin libc6-dev
2019-07-23T19:23:10.0901159Z   libcc1-0 libcilkrts5 libcurl3 libcurl3-gnutls libdpkg-perl libedit2
2019-07-23T19:23:10.0901527Z   liberror-perl libexpat1 libffi-dev libffi6 libgcc-5-dev libgdbm3
2019-07-23T19:23:10.0901811Z   libglib2.0-0 libgmp10 libgnutls30 libgomp1 libgssapi-krb5-2
---
2019-07-23T19:23:10.0903527Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-23T19:23:10.0903805Z   libperl5.22 libpipeline1 libpython-stdlib libpython2.7-minimal
2019-07-23T19:23:10.0904659Z   libpython2.7-stdlib libpython3.5 libpython3.5-minimal libpython3.5-stdlib
2019-07-23T19:23:10.0904976Z   libquadmath0 libroken18-heimdal librtmp1 libsasl2-2 libsasl2-modules-db
2019-07-23T19:23:10.0905273Z   libsqlite3-0 libssl1.0.0 libstdc++-5-dev libtasn1-6 libtinfo-dev libtsan0
2019-07-23T19:23:10.0905626Z   libubsan0 libwind0-heimdal libxml2 linux-libc-dev llvm-6.0 llvm-6.0-dev
2019-07-23T19:23:10.0905916Z   llvm-6.0-runtime mime-support openssl patch perl perl-modules-5.22 python
2019-07-23T19:23:10.0906462Z Suggested packages:
2019-07-23T19:23:10.0906462Z Suggested packages:
2019-07-23T19:23:10.0906803Z   binutils-doc bzip2-doc codeblocks eclipse ninja-build cpp-doc gcc-5-locales
2019-07-23T19:23:10.0907093Z   debian-keyring g++-multilib g++-5-multilib gcc-5-doc libstdc++6-5-dbg
2019-07-23T19:23:10.0907438Z   gcc-multilib manpages-dev autoconf automake libtool flex bison gcc-doc
2019-07-23T19:23:10.0908011Z   libasan2-dbg liblsan0-dbg libtsan0-dbg libubsan0-dbg libcilkrts5-dbg
2019-07-23T19:23:10.0908357Z   libmpx0-dbg libquadmath0-dbg gdb-doc gettext-base git-daemon-run
2019-07-23T19:23:10.0908652Z   | git-daemon-sysvinit git-doc git-el git-email git-gui gitk gitweb git-arch
2019-07-23T19:23:10.0908945Z   git-cvs git-mediawiki git-svn lrzip glibc-doc gnutls-bin krb5-doc krb5-user
2019-07-23T19:23:10.0908945Z   git-cvs git-mediawiki git-svn lrzip glibc-doc gnutls-bin krb5-doc krb5-user
2019-07-23T19:23:10.0909315Z   libstdc++-5-doc llvm-6.0-doc make-doc ed diffutils-doc perl-doc
2019-07-23T19:23:10.0909607Z   libterm-readline-gnu-perl | libterm-readline-perl-perl python-doc python-tk
2019-07-23T19:23:10.0909958Z Recommended packages:
2019-07-23T19:23:10.0909958Z Recommended packages:
2019-07-23T19:23:10.0910246Z   build-essential fakeroot libalgorithm-merge-perl libc-dbg gdbserver less
2019-07-23T19:23:10.0910524Z   rsync ssh-client manpages manpages-dev libfile-fcntllock-perl
2019-07-23T19:23:10.0910867Z   libglib2.0-data shared-mime-info xdg-user-dirs krb5-locales libsasl2-modules
2019-07-23T19:23:10.0911128Z   libssl-doc xml-core netbase rename
2019-07-23T19:23:10.0911560Z   binfmt-support binutils bzip2 ca-certificates cmake cmake-data cpp cpp-5
2019-07-23T19:23:10.0911560Z   binfmt-support binutils bzip2 ca-certificates cmake cmake-data cpp cpp-5
2019-07-23T19:23:10.0912180Z   curl dpkg-dev file g++ g++-5 gcc gcc-5 gdb git git-man libarchive13 libasan2
2019-07-23T19:23:10.0912526Z   libasn1-8-heimdal libatomic1 libbabeltrace-ctf1 libbabeltrace1 libbsd-dev
2019-07-23T19:23:10.0912887Z   libbsd0 libc-dev-bin libc6-dev libcc1-0 libcilkrts5 libcurl3 libcurl3-gnutls
2019-07-23T19:23:10.0913209Z   libdpkg-perl libedit-dev libedit2 liberror-perl libexpat1 libffi-dev libffi6
2019-07-23T19:23:10.0913496Z   libgcc-5-dev libgdbm3 libglib2.0-0 libgmp10 libgnutls30 libgomp1
2019-07-23T19:23:10.0914533Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-23T19:23:10.0914829Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-23T19:23:10.0915173Z   libkrb5-3 libkrb5support0 libldap-2.4-2 libllvm6.0 liblsan0 liblzo2-2
2019-07-23T19:23:10.0915639Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-23T19:23:10.0915639Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-23T19:23:10.0915975Z   libperl5.22 libpipeline1 libpython-stdlib libpython2.7-minimal
2019-07-23T19:23:10.0916333Z   libpython2.7-stdlib libpython3.5 libpython3.5-minimal libpython3.5-stdlib
2019-07-23T19:23:10.0916633Z   libquadmath0 libroken18-heimdal librtmp1 libsasl2-2 libsasl2-modules-db
2019-07-23T19:23:10.0916940Z   libsqlite3-0 libssl-dev libssl1.0.0 libstdc++-5-dev libtasn1-6 libtinfo-dev
2019-07-23T19:23:10.0917301Z   libtsan0 libubsan0 libwind0-heimdal libxml2 linux-libc-dev llvm-6.0
2019-07-23T19:23:10.0917595Z   llvm-6.0-dev llvm-6.0-runtime llvm-6.0-tools make mime-support openssl patch
2019-07-23T19:23:10.0917877Z   perl perl-modules-5.22 pkg-config python python-minimal python2.7
2019-07-23T19:23:10.0918191Z   python2.7-minimal sudo xz-utils zlib1g-dev
2019-07-23T19:23:10.4872836Z Need to get 121 MB of archives.
2019-07-23T19:23:10.4873007Z After this operation, 592 MB of additional disk space will be used.
2019-07-23T19:23:10.4874669Z Get:1 http://archive.ubuntu.com/ubuntu xenial/main amd64 libgdbm3 amd64 1.8.3-13.1 [16.9 kB]
2019-07-23T19:23:10.7135828Z Get:2 http://archive.ubuntu.com/ubuntu xenial/main amd64 libffi6 amd64 3.2.1-4 [17.8 kB]
---
2019-07-23T19:24:17.2705298Z Unpacking zlib1g-dev:amd64 (1:1.2.8.dfsg-2ubuntu4.1) ...
2019-07-23T19:24:17.3864403Z Selecting previously unselected package libssl-dev:amd64.
2019-07-23T19:24:17.3883347Z Preparing to unpack .../libssl-dev_1.0.2g-1ubuntu4.15_amd64.deb ...
2019-07-23T19:24:17.4008699Z Unpacking libssl-dev:amd64 (1.0.2g-1ubuntu4.15) ...
2019-07-23T19:24:17.7176880Z Selecting previously unselected package llvm-6.0-runtime.
2019-07-23T19:24:17.7200601Z Preparing to unpack .../llvm-6.0-runtime_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-23T19:24:17.7348121Z Unpacking llvm-6.0-runtime (1:6.0-1ubuntu2~16.04.1) ...
2019-07-23T19:24:17.8574188Z Selecting previously unselected package llvm-6.0.
2019-07-23T19:24:17.8593373Z Preparing to unpack .../llvm-6.0_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-23T19:24:17.8751117Z Unpacking llvm-6.0 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-23T19:24:18.9292702Z Selecting previously unselected package libffi-dev:amd64.
2019-07-23T19:24:18.9313640Z Preparing to unpack .../libffi-dev_3.2.1-4_amd64.deb ...
2019-07-23T19:24:19.0194684Z Unpacking libffi-dev:amd64 (3.2.1-4) ...
2019-07-23T19:24:19.4703347Z Selecting previously unselected package llvm-6.0-dev.
2019-07-23T19:24:19.4729350Z Preparing to unpack .../llvm-6.0-dev_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-23T19:24:19.5696117Z Unpacking llvm-6.0-dev (1:6.0-1ubuntu2~16.04.1) ...
2019-07-23T19:24:24.4045901Z Selecting previously unselected package llvm-6.0-tools.
2019-07-23T19:24:24.4068648Z Preparing to unpack .../llvm-6.0-tools_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-23T19:24:24.4192383Z Unpacking llvm-6.0-tools (1:6.0-1ubuntu2~16.04.1) ...
2019-07-23T19:24:24.5823337Z Selecting previously unselected package pkg-config.
2019-07-23T19:24:24.5839816Z Preparing to unpack .../pkg-config_0.29.1-0ubuntu1_amd64.deb ...
2019-07-23T19:24:24.6517896Z Processing triggers for libc-bin (2.23-0ubuntu11) ...
2019-07-23T19:24:24.7042480Z Processing triggers for systemd (229-4ubuntu21.22) ...
2019-07-23T19:24:25.1218498Z Setting up libgdbm3:amd64 (1.8.3-13.1) ...
2019-07-23T19:24:25.6387227Z Setting up libffi6:amd64 (3.2.1-4) ...
---
2019-07-23T19:24:34.2936723Z Setting up libcurl3-gnutls:amd64 (7.47.0-1ubuntu2.13) ...
2019-07-23T19:24:34.3323398Z Setting up libedit2:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-23T19:24:34.3699360Z Setting up libpipeline1:amd64 (1.4.1-2) ...
2019-07-23T19:24:34.4225630Z Setting up binfmt-support (2.1.6-1) ...
2019-07-23T19:24:34.4903708Z mount: permission denied
2019-07-23T19:24:34.4904948Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-23T19:24:34.4920370Z mount: permission denied
2019-07-23T19:24:34.4927812Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-23T19:24:34.6761808Z invoke-rc.d: could not determine current runlevel
2019-07-23T19:24:34.6803281Z invoke-rc.d: policy-rc.d denied execution of start.
2019-07-23T19:24:34.7401498Z Setting up libisl15:amd64 (0.16.1-1) ...
2019-07-23T19:24:34.7772368Z Setting up cpp-5 (5.4.0-6ubuntu1~16.04.11) ...
2019-07-23T19:24:34.8277574Z Setting up cpp (4:5.3.1-1ubuntu1) ...
2019-07-23T19:24:34.8916797Z Setting up curl (7.47.0-1ubuntu2.13) ...
---
2019-07-23T19:24:39.6503192Z Setting up libedit-dev:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-23T19:24:39.6913664Z Setting up libllvm6.0:amd64 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-23T19:24:39.7282294Z Setting up zlib1g-dev:amd64 (1:1.2.8.dfsg-2ubuntu4.1) ...
2019-07-23T19:24:39.7640576Z Setting up libssl-dev:amd64 (1.0.2g-1ubuntu4.15) ...
2019-07-23T19:24:39.8251129Z Setting up llvm-6.0-runtime (1:6.0-1ubuntu2~16.04.1) ...
2019-07-23T19:24:39.9917477Z mount: permission denied
2019-07-23T19:24:39.9918959Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-23T19:24:40.1387264Z Setting up llvm-6.0 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-23T19:24:40.4867127Z Setting up libffi-dev:amd64 (3.2.1-4) ...
2019-07-23T19:24:44.7824667Z Setting up llvm-6.0-dev (1:6.0-1ubuntu2~16.04.1) ...
2019-07-23T19:24:44.8196908Z Setting up llvm-6.0-tools (1:6.0-1ubuntu2~16.04.1) ...
2019-07-23T19:24:44.8578507Z Setting up pkg-config (0.29.1-0ubuntu1) ...
2019-07-23T19:24:44.9853601Z Processing triggers for ca-certificates (20170717~16.04.2) ...
2019-07-23T19:24:45.0025567Z Updating certificates in /etc/ssl/certs...
2019-07-23T19:24:46.7806073Z 148 added, 0 removed; done.
2019-07-23T19:24:46.7807070Z Running hooks in /etc/ca-certificates/update.d...
---
2019-07-23T19:25:27.2829047Z Successfully built 4ed41b34bbd6
2019-07-23T19:25:27.4768642Z Successfully tagged rust-ci:latest
2019-07-23T19:25:27.5337702Z Built container sha256:4ed41b34bbd63d35078dfbe8f87751d398523af361b1fc9681b183e3cc59d0c0
2019-07-23T19:25:27.5357995Z Uploading finished image to https://rust-lang-ci-sccache2.s3.amazonaws.com/docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624
2019-07-23T19:26:35.5733510Z upload failed: - to s3://rust-lang-ci-sccache2/docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624 Unable to locate credentials
2019-07-23T19:26:40.1260471Z [CI_JOB_NAME=x86_64-gnu-llvm-6.0]
2019-07-23T19:26:40.1311123Z Starting sccache server...
2019-07-23T19:26:40.2236171Z configure: processing command line
2019-07-23T19:26:40.2237974Z configure: 
---
2019-07-23T20:26:16.1050266Z .................................................................................................... 200/5841
2019-07-23T20:26:20.7172665Z F................................................................................................... 300/5841
2019-07-23T20:26:24.7832475Z .................................................................................................... 400/5841
2019-07-23T20:26:28.9595073Z .................................................................................................... 500/5841
2019-07-23T20:26:33.1499997Z ........................................................................i........................... 600/5841
2019-07-23T20:26:42.8392035Z .................................................................................................... 800/5841
2019-07-23T20:26:48.8512876Z .................................................................................................... 900/5841
2019-07-23T20:26:54.2913265Z ...................................................................................................i 1000/5841
2019-07-23T20:26:54.2913265Z ...................................................................................................i 1000/5841
2019-07-23T20:27:00.2856143Z ...........i........................................................................................ 1100/5841
2019-07-23T20:27:04.5434063Z .............................iiiii.................................................................. 1200/5841
2019-07-23T20:27:11.2937639Z .................................................................................................... 1400/5841
2019-07-23T20:27:14.3762558Z .................................................................................................... 1500/5841
2019-07-23T20:27:18.5464406Z .................................................................................................... 1600/5841
2019-07-23T20:27:21.4451566Z .................................................................................................... 1700/5841
2019-07-23T20:27:21.4451566Z .................................................................................................... 1700/5841
2019-07-23T20:27:25.2997495Z ...................................................................i................................ 1800/5841
2019-07-23T20:27:34.7027811Z .................................................................................................... 2000/5841
2019-07-23T20:27:39.4095327Z .................................................................................................... 2100/5841
2019-07-23T20:27:43.5676274Z .................................................................................................... 2200/5841
2019-07-23T20:27:43.5676274Z .................................................................................................... 2200/5841
2019-07-23T20:27:47.7811633Z ..................................................i................................................. 2300/5841
2019-07-23T20:27:58.4772643Z .................................................................................................... 2500/5841
2019-07-23T20:28:03.0300528Z .................................................................................................... 2600/5841
2019-07-23T20:28:08.6661140Z .................................................................................................... 2700/5841
2019-07-23T20:28:13.0331978Z .................................................................................................... 2800/5841
2019-07-23T20:28:13.0331978Z .................................................................................................... 2800/5841
2019-07-23T20:28:17.7495170Z .................................................................................................... 2900/5841
2019-07-23T20:28:23.4441806Z .................................................................................................... 3000/5841
2019-07-23T20:28:28.2968569Z .................................................................................................... 3100/5841
2019-07-23T20:28:34.0469615Z .................................................................................................... 3200/5841
2019-07-23T20:28:37.8884081Z .................................................................................................... 3300/5841
2019-07-23T20:28:41.9656421Z .................................................................................................... 3400/5841
2019-07-23T20:28:47.4853037Z .................................................................................................... 3500/5841
2019-07-23T20:28:51.5678210Z .................i.................................................................................. 3600/5841
2019-07-23T20:28:56.1113618Z ...........................................................................................ii...i..i 3700/5841
2019-07-23T20:29:00.5873952Z i................................................................................................... 3800/5841
2019-07-23T20:29:09.8201458Z .................................................................................................... 4000/5841
2019-07-23T20:29:09.8201458Z .................................................................................................... 4000/5841
2019-07-23T20:29:13.8776131Z .....ii............................................................................................. 4100/5841
2019-07-23T20:29:16.1695713Z ..........................i......................................................................... 4200/5841
2019-07-23T20:29:18.4332676Z ............................................................................................i....... 4300/5841
2019-07-23T20:29:26.1406755Z .................................................................................................... 4500/5841
2019-07-23T20:29:45.0081040Z .................................................................................................... 4600/5841
2019-07-23T20:29:48.8033683Z .................................................................................................... 4700/5841
2019-07-23T20:29:52.9759165Z .................................................................................................... 4800/5841
---
2019-07-23T20:30:29.3629101Z .................................................................................................... 5400/5841
2019-07-23T20:30:33.7505766Z .................................................................................................... 5500/5841
2019-07-23T20:30:38.3414283Z .................................................................................................... 5600/5841
2019-07-23T20:30:41.7963045Z .................................................................................................... 5700/5841
2019-07-23T20:30:45.0855501Z .................................................................................i.................. 5800/5841
2019-07-23T20:30:46.7620333Z failures:
2019-07-23T20:30:46.7671904Z 
2019-07-23T20:30:46.7672536Z ---- [ui] ui/async-await/recursive-async-impl-trait-type.rs stdout ----
2019-07-23T20:30:46.7672598Z diff of stderr:
2019-07-23T20:30:46.7672598Z diff of stderr:
2019-07-23T20:30:46.7672706Z 
2019-07-23T20:30:46.7672968Z - error[E0733]: recursion in an async fn requires boxing
2019-07-23T20:30:46.7673019Z + error[E0733]: recursion in an `async fn` requires boxing
2019-07-23T20:30:46.7673557Z 3    |
2019-07-23T20:30:46.7673557Z 3    |
2019-07-23T20:30:46.7673950Z 4 LL | async fn recursive_async_function() -> () {
2019-07-23T20:30:46.7674262Z -    |                                        ^^ async fn cannot invoke themselves directly
2019-07-23T20:30:46.7674262Z -    |                                        ^^ async fn cannot invoke themselves directly
2019-07-23T20:30:46.7674323Z +    |                                        ^^ an `async fn` cannot invoke themselves directly
2019-07-23T20:30:46.7674384Z 6    |
2019-07-23T20:30:46.7674655Z -    = note: to create a recursive async fn, it must be rewritten to return a
2019-07-23T20:30:46.7674712Z +    = note: to create a recursive `async fn`, it must be rewritten to return a
2019-07-23T20:30:46.7674759Z 8            boxed future.
2019-07-23T20:30:46.7675023Z 9            For more information, see https://rust-lang.github.io/async-book/
2019-07-23T20:30:46.7675097Z 
2019-07-23T20:30:46.7675123Z 
2019-07-23T20:30:46.7675193Z The actual stderr differed from the expected stderr.
2019-07-23T20:30:46.7675522Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/recursive-async-impl-trait-type/recursive-async-impl-trait-type.stderr
2019-07-23T20:30:46.7675522Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/recursive-async-impl-trait-type/recursive-async-impl-trait-type.stderr
2019-07-23T20:30:46.7675771Z To update references, rerun the tests and pass the `--bless` flag
2019-07-23T20:30:46.7676202Z To only update this specific test, also pass `--test-args async-await/recursive-async-impl-trait-type.rs`
2019-07-23T20:30:46.7676413Z error: 1 errors occurred comparing output.
2019-07-23T20:30:46.7676473Z status: exit code: 1
2019-07-23T20:30:46.7676473Z status: exit code: 1
2019-07-23T20:30:46.7677373Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/recursive-async-impl-trait-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/recursive-async-impl-trait-type" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/recursive-async-impl-trait-type/auxiliary" "-A" "unused"
2019-07-23T20:30:46.7678286Z ------------------------------------------
2019-07-23T20:30:46.7678323Z 
2019-07-23T20:30:46.7678591Z ------------------------------------------
2019-07-23T20:30:46.7678636Z stderr:
2019-07-23T20:30:46.7678636Z stderr:
2019-07-23T20:30:46.7678853Z ------------------------------------------
2019-07-23T20:30:46.7678918Z error[E0733]: recursion in an `async fn` requires boxing
2019-07-23T20:30:46.7679236Z    |
2019-07-23T20:30:46.7679236Z    |
2019-07-23T20:30:46.7679489Z LL | async fn recursive_async_function() -> () { //~ ERROR
2019-07-23T20:30:46.7679545Z    |                                        ^^ an `async fn` cannot invoke themselves directly
2019-07-23T20:30:46.7679601Z    |
2019-07-23T20:30:46.7679667Z    = note: to create a recursive `async fn`, it must be rewritten to return a
2019-07-23T20:30:46.7679715Z            boxed future.
2019-07-23T20:30:46.7679972Z            For more information, see https://rust-lang.github.io/async-book/
2019-07-23T20:30:46.7680067Z error: aborting due to previous error
2019-07-23T20:30:46.7680105Z 
2019-07-23T20:30:46.7680351Z For more information about this error, try `rustc --explain E0733`.
2019-07-23T20:30:46.7680384Z 
---
2019-07-23T20:30:46.7705292Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:535:22
2019-07-23T20:30:46.7705421Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-07-23T20:30:46.7732828Z 
2019-07-23T20:30:46.7733105Z 
2019-07-23T20:30:46.7737714Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-07-23T20:30:46.7738772Z 
2019-07-23T20:30:46.7738840Z 
2019-07-23T20:30:46.7750397Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-07-23T20:30:46.7750478Z Build completed unsuccessfully in 1:00:19
2019-07-23T20:30:46.7750478Z Build completed unsuccessfully in 1:00:19
2019-07-23T20:30:47.8569925Z ##[error]Bash exited with code '1'.
2019-07-23T20:30:47.8609882Z ##[section]Starting: Checkout
2019-07-23T20:30:47.8612084Z ==============================================================================
2019-07-23T20:30:47.8612251Z Task         : Get sources
2019-07-23T20:30:47.8612318Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
