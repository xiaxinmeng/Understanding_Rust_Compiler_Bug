plain
2019-07-05T03:45:19.7980462Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-05T03:45:19.8155924Z ##[command]git config gc.auto 0
2019-07-05T03:45:19.8230852Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-05T03:45:19.8282742Z ##[command]git config --get-all http.proxy
2019-07-05T03:45:19.8416230Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62306/merge:refs/remotes/pull/62306/merge
2019-07-05T03:45:23.1897443Z remote: Counting objects:   0% (1/138657)           
2019-07-05T03:45:23.1904845Z remote: Counting objects:   0% (92/138657)           
2019-07-05T03:45:23.1922493Z remote: Counting objects:   1% (1387/138657)           
2019-07-05T03:45:23.1937389Z remote: Counting objects:   2% (2774/138657)           
---
2019-07-05T03:45:55.2706821Z  * [new branch]          fix-loop-break-mir-generation -> origin/fix-loop-break-mir-generation
2019-07-05T03:45:55.2713801Z  * [new branch]          master                -> origin/master
2019-07-05T03:45:55.2720127Z  * [new branch]          stable                -> origin/stable
2019-07-05T03:45:55.2728197Z  * [new branch]          try                   -> origin/try
2019-07-05T03:45:55.2735575Z  * [new ref]             refs/pull/62306/merge -> pull/62306/merge
2019-07-05T03:45:55.2765157Z  * [new tag]             0.10                  -> 0.10
2019-07-05T03:45:55.2765409Z  * [new tag]             0.11.0                -> 0.11.0
2019-07-05T03:45:55.2770188Z  * [new tag]             0.12.0                -> 0.12.0
2019-07-05T03:45:55.2774026Z  * [new tag]             0.2                   -> 0.2
---
2019-07-05T03:45:55.3070927Z  * [new tag]             release-0.4           -> release-0.4
2019-07-05T03:45:55.3076522Z  * [new tag]             release-0.5           -> release-0.5
2019-07-05T03:45:55.3081498Z  * [new tag]             release-0.6           -> release-0.6
2019-07-05T03:45:55.3085846Z  * [new tag]             release-0.7           -> release-0.7
2019-07-05T03:45:56.2772636Z ##[command]git checkout --progress --force refs/remotes/pull/62306/merge
2019-07-05T03:45:56.4006618Z Checking out files:  99% (18408/18593)
2019-07-05T03:45:56.4009036Z Checking out files: 100% (18593/18593)
2019-07-05T03:45:56.4009097Z Checking out files: 100% (18593/18593), done.
2019-07-05T03:45:56.4009097Z Checking out files: 100% (18593/18593), done.
2019-07-05T03:45:56.4344357Z Note: checking out 'refs/remotes/pull/62306/merge'.
2019-07-05T03:45:56.4355410Z You are in 'detached HEAD' state. You can look around, make experimental
2019-07-05T03:45:56.4355530Z changes and commit them, and you can discard any commits you make in this
2019-07-05T03:45:56.4355576Z state without impacting any branches by performing another checkout.
2019-07-05T03:45:56.4355624Z 
2019-07-05T03:45:56.4355624Z 
2019-07-05T03:45:56.4355664Z If you want to create a new branch to retain commits you create, you may
2019-07-05T03:45:56.4355931Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-05T03:45:56.4355960Z 
2019-07-05T03:45:56.4356326Z   git checkout -b <new-branch-name>
2019-07-05T03:45:56.4356367Z 
2019-07-05T03:45:56.4356411Z HEAD is now at 5f89e1f16 Merge ac3ae07e7a82a689453aa693e475d9bed76b55ca into 4ca7a349da6869286d94fd8159eb225629ffba5f
2019-07-05T03:45:56.4480959Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-05T03:45:56.4483979Z ==============================================================================
2019-07-05T03:45:56.4484031Z Task         : Bash
2019-07-05T03:45:56.4484073Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-05T03:45:56.7627991Z BUILD_REPOSITORY_URI=***
2019-07-05T03:45:56.7628282Z BUILD_REQUESTEDFOREMAIL=
2019-07-05T03:45:56.7628485Z BUILD_REQUESTEDFOR=GitHub
2019-07-05T03:45:56.7629005Z BUILD_REQUESTEDFORID=5e5f2016-b710-434c-83b9-9adad28a8d3a
2019-07-05T03:45:56.7629271Z BUILD_SOURCEBRANCHNAME=merge
2019-07-05T03:45:56.7629622Z BUILD_SOURCEBRANCH=refs/pull/62306/merge
2019-07-05T03:45:56.7630148Z BUILD_SOURCEVERSION=5f89e1f1629c180c0133b1799f3960460dfe1f15
2019-07-05T03:45:56.7630148Z BUILD_SOURCEVERSION=5f89e1f1629c180c0133b1799f3960460dfe1f15
2019-07-05T03:45:56.7630391Z BUILD_SOURCEVERSIONAUTHOR=Chris Gregory
2019-07-05T03:45:56.7630609Z BUILD_SOURCEVERSIONMESSAGE=Merge ac3ae07e7a82a689453aa693e475d9bed76b55ca into 4ca7a349da6869286d94fd8159eb225629ffba5f
2019-07-05T03:45:56.7631198Z CARGO_HOME=/usr/local/cargo
2019-07-05T03:45:56.7631628Z CHROME_BIN=/usr/bin/google-chrome
2019-07-05T03:45:56.7631879Z COMMON_TESTRESULTSDIRECTORY=/home/vsts/work/1/TestResults
2019-07-05T03:45:56.7632144Z CONDA=/usr/share/miniconda
---
2019-07-05T03:45:56.7647880Z SYSTEM_PHASEDISPLAYNAME=Linux
2019-07-05T03:45:56.7648307Z SYSTEM_PHASEID=c041ccde-92b7-5163-1d93-b1c06077f4b4
2019-07-05T03:45:56.7648534Z SYSTEM_PHASENAME=Linux
2019-07-05T03:45:56.7648909Z SYSTEM_PIPELINESTARTTIME=2019-07-05 03:45:13+00:00
2019-07-05T03:45:56.7649409Z SYSTEM_PLANID=e04469a1-de67-4b2f-8de2-ee78aa6c6aa9
2019-07-05T03:45:56.7649741Z SYSTEM_PULLREQUEST_ISFORK=True
2019-07-05T03:45:56.7649967Z SYSTEM_PULLREQUEST_MERGEDAT=
2019-07-05T03:45:56.7650105Z SYSTEM_PULLREQUEST_PULLREQUESTID=293753840
2019-07-05T03:45:56.7650240Z SYSTEM_PULLREQUEST_PULLREQUESTNUMBER=62306
2019-07-05T03:45:56.7650689Z SYSTEM_PULLREQUEST_SOURCEBRANCH=use-mem-take-in-compiletest
2019-07-05T03:45:56.7651038Z SYSTEM_PULLREQUEST_SOURCECOMMITID=ac3ae07e7a82a689453aa693e475d9bed76b55ca
2019-07-05T03:45:56.7651485Z SYSTEM_PULLREQUEST_SOURCEREPOSITORYURI=***.git
2019-07-05T03:45:56.7651661Z SYSTEM_PULLREQUEST_TARGETBRANCH=master
2019-07-05T03:45:56.7651794Z SYSTEM_RESTRICTSECRETS=True
2019-07-05T03:45:56.7652071Z SYSTEM_STAGEATTEMPT=1
2019-07-05T03:45:56.7652186Z SYSTEM_STAGEDISPLAYNAME=__default
2019-07-05T03:45:56.7652569Z SYSTEM_STAGEID=96ac2280-8cb4-5df5-99de-dd2da759617d
2019-07-05T03:45:56.7652737Z SYSTEM_STAGENAME=__default
---
2019-07-05T03:46:12.5612401Z + modules=($modules)
2019-07-05T03:46:12.5612436Z + use_git=
2019-07-05T03:46:12.5617332Z ++ git config --file .gitmodules --get-regexp '\.url$'
2019-07-05T03:46:12.5618423Z ++ cut '-d ' -f2
2019-07-05T03:46:12.5636705Z + urls='***-installer.git
2019-07-05T03:46:12.5638865Z https://github.com/rust-lang/cargo.git
2019-07-05T03:46:12.5639481Z https://github.com/rust-lang-nursery/reference.git
2019-07-05T03:46:12.5639919Z https://github.com/rust-lang/book.git
2019-07-05T03:46:12.5640328Z https://github.com/rust-lang-nursery/rls.git
---
2019-07-05T03:46:12.8180205Z ++ awk '{print $3}'
2019-07-05T03:46:12.8202640Z + commit=62b3ff2cb44dd8b648c3ef2a9347c3706d148014
2019-07-05T03:46:12.8203326Z + git rm src/doc/rust-by-example
2019-07-05T03:46:12.8409950Z rm 'src/doc/rust-by-example'
2019-07-05T03:46:12.8412459Z + url=***-by-example.git
2019-07-05T03:46:12.8414785Z + url=***-by-example
2019-07-05T03:46:12.8415470Z + for i in '${!modules[@]}'
2019-07-05T03:46:12.8415797Z + module=src/llvm-emscripten
2019-07-05T03:46:12.8416677Z + [[  src/llvm-project src/llvm-emscripten src/doc/book src/doc/rust-by-example  = *\ \s\r\c\/\l\l\v\m\-\e\m\s\c\r\i\p\t\e\n\ * ]]
2019-07-05T03:46:12.8416677Z + [[  src/llvm-project src/llvm-emscripten src/doc/book src/doc/rust-by-example  = *\ \s\r\c\/\l\l\v\m\-\e\m\s\c\r\i\p\t\e\n\ * ]]
2019-07-05T03:46:12.8417447Z + fetch_github_commit_archive src/doc/rust-by-example ***-by-example/archive/62b3ff2cb44dd8b648c3ef2a9347c3706d148014.tar.gz
2019-07-05T03:46:12.8419318Z + local cached=download-src-doc-rust-by-example.tar.gz
2019-07-05T03:46:12.8419318Z + local cached=download-src-doc-rust-by-example.tar.gz
2019-07-05T03:46:12.8420059Z + retry sh -c 'rm -f download-src-doc-rust-by-example.tar.gz &&         curl -f -sSL -o download-src-doc-rust-by-example.tar.gz ***-by-example/archive/62b3ff2cb44dd8b648c3ef2a9347c3706d148014.tar.gz'
2019-07-05T03:46:12.8421678Z + echo 'Attempting with retry:' sh -c 'rm -f download-src-doc-rust-by-example.tar.gz &&         curl -f -sSL -o download-src-doc-rust-by-example.tar.gz ***-by-example/archive/62b3ff2cb44dd8b648c3ef2a9347c3706d148014.tar.gz'
2019-07-05T03:46:12.8431464Z Attempting with retry: sh -c rm -f download-src-doc-rust-by-example.tar.gz &&         curl -f -sSL -o download-src-doc-rust-by-example.tar.gz ***-by-example/archive/62b3ff2cb44dd8b648c3ef2a9347c3706d148014.tar.gz
2019-07-05T03:46:12.8437802Z + local n=1
2019-07-05T03:46:12.8440538Z + local max=5
2019-07-05T03:46:12.8458293Z + true
2019-07-05T03:46:12.8459097Z ++ awk '{print $3}'
2019-07-05T03:46:12.8459097Z ++ awk '{print $3}'
2019-07-05T03:46:12.8459994Z + sh -c 'rm -f download-src-doc-rust-by-example.tar.gz &&         curl -f -sSL -o download-src-doc-rust-by-example.tar.gz ***-by-example/archive/62b3ff2cb44dd8b648c3ef2a9347c3706d148014.tar.gz'
2019-07-05T03:46:12.8488918Z + git rm src/llvm-emscripten
2019-07-05T03:46:12.8705162Z rm 'src/llvm-emscripten'
2019-07-05T03:46:12.8713122Z + url=https://github.com/rust-lang/llvm.git
2019-07-05T03:46:12.8713689Z + url=https://github.com/rust-lang/llvm
---
2019-07-05T03:46:13.1181895Z Submodule 'src/doc/edition-guide' (https://github.com/rust-lang-nursery/edition-guide.git) registered for path 'src/doc/edition-guide'
2019-07-05T03:46:13.1185325Z Submodule 'src/doc/embedded-book' (https://github.com/rust-embedded/book.git) registered for path 'src/doc/embedded-book'
2019-07-05T03:46:13.1189454Z Submodule 'src/doc/nomicon' (https://github.com/rust-lang-nursery/nomicon.git) registered for path 'src/doc/nomicon'
2019-07-05T03:46:13.1193848Z Submodule 'src/doc/reference' (https://github.com/rust-lang-nursery/reference.git) registered for path 'src/doc/reference'
2019-07-05T03:46:13.1197501Z Submodule 'src/doc/rustc-guide' (***c-guide.git) registered for path 'src/doc/rustc-guide'
2019-07-05T03:46:13.1205499Z Submodule 'src/tools/cargo' (https://github.com/rust-lang/cargo.git) registered for path 'src/tools/cargo'
2019-07-05T03:46:13.1210739Z Submodule 'src/tools/clippy' (https://github.com/rust-lang-nursery/rust-clippy.git) registered for path 'src/tools/clippy'
2019-07-05T03:46:13.1218022Z Submodule 'src/tools/miri' (https://github.com/rust-lang/miri.git) registered for path 'src/tools/miri'
2019-07-05T03:46:13.1221715Z Submodule 'src/tools/rls' (https://github.com/rust-lang-nursery/rls.git) registered for path 'src/tools/rls'
2019-07-05T03:46:13.1221715Z Submodule 'src/tools/rls' (https://github.com/rust-lang-nursery/rls.git) registered for path 'src/tools/rls'
2019-07-05T03:46:13.1225902Z Submodule 'src/rust-installer' (***-installer.git) registered for path 'src/tools/rust-installer'
2019-07-05T03:46:13.1430901Z Cloning into '/home/vsts/work/1/s/src/doc/edition-guide'...
2019-07-05T03:46:13.9306255Z + break
2019-07-05T03:46:13.9308439Z + mkdir src/doc/rust-by-example
2019-07-05T03:46:13.9325424Z + touch src/doc/rust-by-example/.git
---
2019-07-05T03:47:22.2640738Z   Downloading https://files.pythonhosted.org/packages/73/fb/00a976f728d0d1fecfe898238ce23f502a721c0ac0ecfedb80e0d88c64e9/six-1.12.0-py2.py3-none-any.whl
2019-07-05T03:47:22.2687909Z Building wheels for collected packages: PyYAML
2019-07-05T03:47:22.2688331Z   Running setup.py bdist_wheel for PyYAML: started
2019-07-05T03:47:22.4622442Z   Running setup.py bdist_wheel for PyYAML: finished with status 'error'
2019-07-05T03:47:22.4623128Z   Complete output from command /usr/bin/python3 -u -c "import setuptools, tokenize;__file__='/tmp/pip-build-n81t4_l4/PyYAML/setup.py';exec(compile(getattr(tokenize, 'open', open)(__file__).read().replace('\r\n', '\n'), __file__, 'exec'))" bdist_wheel -d /tmp/tmpd2seii_0pip-wheel- --python-tag cp35:
2019-07-05T03:47:22.4624070Z     warnings.warn(msg)
2019-07-05T03:47:22.4624361Z   usage: -c [global_opts] cmd1 [cmd1_opts] [cmd2 [cmd2_opts] ...]
2019-07-05T03:47:22.4624589Z      or: -c --help [cmd1 cmd2 ...]
2019-07-05T03:47:22.4624853Z      or: -c --help-commands
---
2019-07-05T03:47:32.9451403Z Attempting with retry: curl -y 30 -Y 10 --connect-timeout 30 -f -L -C - -o /tmp/rustci_docker_cache https://.s3.amazonaws.com/docker/a4940e6914a5e1f6360ebc241d0d850aff083be5c43d268fd716ce48f63cb24e84238becb662f27f8c5d7740413ca17da24bb9cdc003ef32d7b03d2c9052b94d
2019-07-05T03:47:32.9520676Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-05T03:47:32.9521074Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-05T03:47:32.9521359Z 
2019-07-05T03:47:32.9523091Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-05T03:47:33.9582904Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-05T03:47:33.9583215Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-05T03:47:33.9583439Z 
2019-07-05T03:47:33.9583439Z 
2019-07-05T03:47:33.9625882Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-05T03:47:35.9694864Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-05T03:47:35.9694953Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-05T03:47:35.9695058Z 
2019-07-05T03:47:35.9695058Z 
2019-07-05T03:47:35.9738266Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-05T03:47:38.9810278Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-05T03:47:38.9814770Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-05T03:47:38.9815069Z 
2019-07-05T03:47:38.9815069Z 
2019-07-05T03:47:38.9863387Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-05T03:47:42.9922612Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-05T03:47:42.9922700Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-05T03:47:42.9922738Z 
2019-07-05T03:47:42.9922738Z 
2019-07-05T03:47:42.9966006Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-05T03:47:42.9969390Z The command has failed after 5 attempts.
2019-07-05T03:47:43.0420861Z open /tmp/rustci_docker_cache: no such file or directory
2019-07-05T03:47:43.0450333Z Attempting with retry: docker build --rm -t rust-ci -f /home/vsts/work/1/s/src/ci/docker/mingw-check/Dockerfile /home/vsts/work/1/s/src/ci/docker
2019-07-05T03:47:43.2710005Z Sending build context to Docker daemon  521.7kB
2019-07-05T03:47:43.2710134Z 
2019-07-05T03:47:43.2987863Z Step 1/6 : FROM ubuntu:16.04
---
2019-07-05T03:47:59.2256683Z Reading package lists...
2019-07-05T03:48:00.1638741Z Reading package lists...
2019-07-05T03:48:00.3558140Z Building dependency tree...
2019-07-05T03:48:00.3558772Z Reading state information...
2019-07-05T03:48:00.4755665Z The following additional packages will be installed:
2019-07-05T03:48:00.4757765Z   binutils binutils-mingw-w64-i686 binutils-mingw-w64-x86-64 bzip2 cmake-data
2019-07-05T03:48:00.4762643Z   cpp cpp-5 dpkg-dev g++-5 g++-mingw-w64 g++-mingw-w64-i686
2019-07-05T03:48:00.4763106Z   g++-mingw-w64-x86-64 gcc gcc-5 gcc-mingw-w64 gcc-mingw-w64-base
2019-07-05T03:48:00.4779853Z   gcc-mingw-w64-i686 gcc-mingw-w64-x86-64 git-man libarchive13 libasan2
2019-07-05T03:48:00.4780633Z   libasn1-8-heimdal libatomic1 libbabeltrace-ctf1 libbabeltrace1 libbz2-1.0
2019-07-05T03:48:00.4784336Z   libdpkg-perl liberror-perl libexpat1 libffi6 libgcc-5-dev libgdbm3
2019-07-05T03:48:00.4784934Z   libglib2.0-0 libgmp10 libgnutls30 libgomp1 libgssapi-krb5-2
2019-07-05T03:48:00.4786582Z   libgssapi3-heimdal libhcrypto4-heimdal libheimbase1-heimdal
2019-07-05T03:48:00.4787046Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-05T03:48:00.4787046Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-05T03:48:00.4787491Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-05T03:48:00.4787881Z   libkrb5-3 libkrb5support0 libldap-2.4-2 liblsan0 liblzo2-2 libmagic1 libmpc3
2019-07-05T03:48:00.4788257Z   libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0 libperl5.22
2019-07-05T03:48:00.4788639Z   libpython2.7-minimal libpython2.7-stdlib libpython3.5 libpython3.5-minimal
2019-07-05T03:48:00.4789127Z   libpython3.5-stdlib libquadmath0 libroken18-heimdal librtmp1 libsasl2-2
2019-07-05T03:48:00.4789518Z   libsasl2-modules-db libsqlite3-0 libssl1.0.0 libstdc++-5-dev libtasn1-6
2019-07-05T03:48:00.4789897Z   libtsan0 libubsan0 libwind0-heimdal libxml2 linux-libc-dev mime-support
2019-07-05T03:48:00.4790290Z   mingw-w64-common mingw-w64-i686-dev mingw-w64-x86-64-dev openssl patch perl
2019-07-05T03:48:00.4790667Z   perl-modules-5.22 python2.7-minimal zlib1g-dev
2019-07-05T03:48:00.4790881Z Suggested packages:
2019-07-05T03:48:00.4791227Z   binutils-doc bzip2-doc codeblocks eclipse ninja-build cpp-doc gcc-5-locales
2019-07-05T03:48:00.4791836Z   debian-keyring g++-multilib g++-5-multilib gcc-5-doc libstdc++6-5-dbg
2019-07-05T03:48:00.4792440Z   gcc-multilib manpages-dev autoconf automake libtool flex bison gcc-doc
2019-07-05T03:48:00.4794074Z   libasan2-dbg liblsan0-dbg libtsan0-dbg libubsan0-dbg libcilkrts5-dbg
2019-07-05T03:48:00.4794545Z   libmpx0-dbg libquadmath0-dbg gdb-doc gettext-base git-daemon-run
2019-07-05T03:48:00.4794545Z   libmpx0-dbg libquadmath0-dbg gdb-doc gettext-base git-daemon-run
2019-07-05T03:48:00.4794976Z   | git-daemon-sysvinit git-doc git-el git-email git-gui gitk gitweb git-arch
2019-07-05T03:48:00.4795413Z   git-cvs git-mediawiki git-svn lrzip glibc-doc gnutls-bin krb5-doc krb5-user
2019-07-05T03:48:00.4795852Z   libstdc++-5-doc make-doc wine wine64 ed diffutils-doc perl-doc
2019-07-05T03:48:00.4796489Z   libterm-readline-gnu-perl | libterm-readline-perl-perl python2.7-doc
2019-07-05T03:48:00.4796831Z   binfmt-support
2019-07-05T03:48:00.4797037Z Recommended packages:
2019-07-05T03:48:00.4797367Z   build-essential fakeroot libalgorithm-merge-perl gfortran-mingw-w64
2019-07-05T03:48:00.4797735Z   gnat-mingw-w64 libc-dbg gdbserver less rsync ssh-client manpages
2019-07-05T03:48:00.4798150Z   manpages-dev libfile-fcntllock-perl libglib2.0-data shared-mime-info
2019-07-05T03:48:00.4798531Z   xdg-user-dirs krb5-locales libsasl2-modules libssl-doc xml-core netbase
2019-07-05T03:48:00.4798757Z   rename
2019-07-05T03:48:00.4798948Z The following NEW packages will be installed:
2019-07-05T03:48:00.4799352Z   binutils binutils-mingw-w64-i686 binutils-mingw-w64-x86-64 bzip2
2019-07-05T03:48:00.4799768Z   ca-certificates cmake cmake-data cpp cpp-5 curl dpkg-dev file g++ g++-5
2019-07-05T03:48:00.4800483Z   g++-mingw-w64 g++-mingw-w64-i686 g++-mingw-w64-x86-64 gcc gcc-5
2019-07-05T03:48:00.4801078Z   gcc-mingw-w64 gcc-mingw-w64-base gcc-mingw-w64-i686 gcc-mingw-w64-x86-64 gdb
2019-07-05T03:48:00.4801513Z   git git-man libarchive13 libasan2 libasn1-8-heimdal libatomic1
2019-07-05T03:48:00.4801884Z   libbabeltrace-ctf1 libbabeltrace1 libc-dev-bin libc6-dev libcc1-0
2019-07-05T03:48:00.4803312Z   libffi6 libgcc-5-dev libgdbm3 libglib2.0-0 libgmp10 libgnutls30 libgomp1
2019-07-05T03:48:00.4803819Z   libgssapi-krb5-2 libgssapi3-heimdal libhcrypto4-heimdal libheimbase1-heimdal
2019-07-05T03:48:00.4804310Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-05T03:48:00.4804758Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-05T03:48:00.4804758Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-05T03:48:00.4805199Z   libkrb5-3 libkrb5support0 libldap-2.4-2 liblsan0 liblzo2-2 libmagic1 libmpc3
2019-07-05T03:48:00.4805681Z   libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0 libperl5.22
2019-07-05T03:48:00.4806295Z   libpython2.7-minimal libpython2.7-stdlib libpython3.5 libpython3.5-minimal
2019-07-05T03:48:00.4806895Z   libpython3.5-stdlib libquadmath0 libroken18-heimdal librtmp1 libsasl2-2
2019-07-05T03:48:00.4807259Z   libsasl2-modules-db libsqlite3-0 libssl-dev libssl1.0.0 libstdc++-5-dev
2019-07-05T03:48:00.4807639Z   libtasn1-6 libtsan0 libubsan0 libwind0-heimdal libxml2 linux-libc-dev make
2019-07-05T03:48:00.4808056Z   mime-support mingw-w64 mingw-w64-common mingw-w64-i686-dev
2019-07-05T03:48:00.4808431Z   mingw-w64-x86-64-dev openssl patch perl perl-modules-5.22 pkg-config
2019-07-05T03:48:00.4808789Z   python2.7 python2.7-minimal sudo xz-utils zlib1g-dev
2019-07-05T03:48:00.4808994Z The following packages will be upgraded:
2019-07-05T03:48:00.9314921Z 1 upgraded, 112 newly installed, 0 to remove and 4 not upgraded.
2019-07-05T03:48:00.9315049Z Need to get 187 MB of archives.
2019-07-05T03:48:00.9315506Z After this operation, 968 MB of additional disk space will be used.
2019-07-05T03:48:01.1612489Z Get:1 http://archive.ubuntu.com/ubuntu xenial/main amd64 libgdbm3 amd64 1.8.3-13.1 [16.9 kB]
---
2019-07-05T03:48:05.1817940Z Get:97 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 zlib1g-dev amd64 1:1.2.8.dfsg-2ubuntu4.1 [168 kB]
2019-07-05T03:48:05.1838875Z Get:98 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 libssl-dev amd64 1.0.2g-1ubuntu4.15 [1344 kB]
2019-07-05T03:48:05.2408821Z Get:99 http://archive.ubuntu.com/ubuntu xenial/main amd64 pkg-config amd64 0.29.1-0ubuntu1 [45.0 kB]
2019-07-05T03:48:05.2416866Z Get:100 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 python2.7 amd64 2.7.12-1ubuntu0~16.04.4 [224 kB]
2019-07-05T03:48:05.2770543Z Get:101 http://archive.ubuntu.com/ubuntu xenial/universe amd64 binutils-mingw-w64-i686 amd64 2.26-3ubuntu1+6.6 [1782 kB]
2019-07-05T03:48:05.5238675Z Get:102 http://archive.ubuntu.com/ubuntu xenial/universe amd64 binutils-mingw-w64-x86-64 amd64 2.26-3ubuntu1+6.6 [2029 kB]
2019-07-05T03:48:06.4436370Z Get:103 http://archive.ubuntu.com/ubuntu xenial/universe amd64 mingw-w64-common all 4.0.4-2 [4787 kB]
2019-07-05T03:48:06.6558831Z Get:104 http://archive.ubuntu.com/ubuntu xenial/universe amd64 mingw-w64-i686-dev all 4.0.4-2 [2059 kB]
2019-07-05T03:48:06.7325141Z Get:105 http://archive.ubuntu.com/ubuntu xenial/universe amd64 gcc-mingw-w64-base amd64 5.3.1-8ubuntu3+17 [11.2 kB]
2019-07-05T03:48:06.7328994Z Get:106 http://archive.ubuntu.com/ubuntu xenial/universe amd64 gcc-mingw-w64-i686 amd64 5.3.1-8ubuntu3+17 [27.3 MB]
2019-07-05T03:48:07.7398631Z Get:107 http://archive.ubuntu.com/ubuntu xenial/universe amd64 g++-mingw-w64-i686 amd64 5.3.1-8ubuntu3+17 [19.8 MB]
2019-07-05T03:48:08.4691934Z Get:108 http://archive.ubuntu.com/ubuntu xenial/universe amd64 mingw-w64-x86-64-dev all 4.0.4-2 [3238 kB]
2019-07-05T03:48:08.5830732Z Get:109 http://archive.ubuntu.com/ubuntu xenial/universe amd64 gcc-mingw-w64-x86-64 amd64 5.3.1-8ubuntu3+17 [27.4 MB]
2019-07-05T03:48:09.5847002Z Get:110 http://archive.ubuntu.com/ubuntu xenial/universe amd64 g++-mingw-w64-x86-64 amd64 5.3.1-8ubuntu3+17 [20.4 MB]
2019-07-05T03:48:10.3305735Z Get:111 http://archive.ubuntu.com/ubuntu xenial/universe amd64 g++-mingw-w64 all 5.3.1-8ubuntu3+17 [10.7 kB]
2019-07-05T03:48:10.3310233Z Get:112 http://archive.ubuntu.com/ubuntu xenial/universe amd64 gcc-mingw-w64 all 5.3.1-8ubuntu3+17 [10.7 kB]
2019-07-05T03:48:10.3317158Z Get:113 http://archive.ubuntu.com/ubuntu xenial/universe amd64 mingw-w64 all 4.0.4-2 [9274 B]
2019-07-05T03:48:12.5722056Z debconf: delaying package configuration, since apt-utils is not installed
2019-07-05T03:48:12.5917170Z Fetched 187 MB in 9s (19.4 MB/s)
2019-07-05T03:48:12.6538184Z (Reading database ... 
2019-07-05T03:48:12.6539343Z (Reading database ... 5%
2019-07-05T03:48:12.6539778Z (Reading database ... 10%
2019-07-05T03:48:12.6540096Z (Reading database ... 15%
---
2019-07-05T03:48:38.1588921Z Unpacking git (1:2.7.4-0ubuntu1.6) ...
2019-07-05T03:48:38.8419237Z Selecting previously unselected package libpython2.7-stdlib:amd64.
2019-07-05T03:48:38.8438964Z Preparing to unpack .../libpython2.7-stdlib_2.7.12-1ubuntu0~16.04.4_amd64.deb ...
2019-07-05T03:48:38.8588527Z Unpacking libpython2.7-stdlib:amd64 (2.7.12-1ubuntu0~16.04.4) ...
2019-07-05T03:48:39.2365794Z Selecting previously unselected package zlib1g-dev:amd64.
2019-07-05T03:48:39.2388671Z Preparing to unpack .../zlib1g-dev_1%3a1.2.8.dfsg-2ubuntu4.1_amd64.deb ...
2019-07-05T03:48:39.2510095Z Unpacking zlib1g-dev:amd64 (1:1.2.8.dfsg-2ubuntu4.1) ...
2019-07-05T03:48:39.3529373Z Preparing to unpack .../libssl-dev_1.0.2g-1ubuntu4.15_amd64.deb ...
2019-07-05T03:48:39.3659526Z Unpacking libssl-dev:amd64 (1.0.2g-1ubuntu4.15) ...
2019-07-05T03:48:39.6903613Z Selecting previously unselected package pkg-config.
2019-07-05T03:48:39.6921407Z Preparing to unpack .../pkg-config_0.29.1-0ubuntu1_amd64.deb ...
2019-07-05T03:48:39.6921407Z Preparing to unpack .../pkg-config_0.29.1-0ubuntu1_amd64.deb ...
2019-07-05T03:48:39.7063784Z Unpacking pkg-config (0.29.1-0ubuntu1) ...
2019-07-05T03:48:39.8018915Z Selecting previously unselected package python2.7.
2019-07-05T03:48:39.8035109Z Preparing to unpack .../python2.7_2.7.12-1ubuntu0~16.04.4_amd64.deb ...
2019-07-05T03:48:39.8211572Z Unpacking python2.7 (2.7.12-1ubuntu0~16.04.4) ...
2019-07-05T03:48:39.9089262Z Selecting previously unselected package binutils-mingw-w64-i686.
2019-07-05T03:48:39.9108425Z Preparing to unpack .../binutils-mingw-w64-i686_2.26-3ubuntu1+6.6_amd64.deb ...
2019-07-05T03:48:39.9228915Z Unpacking binutils-mingw-w64-i686 (2.26-3ubuntu1+6.6) ...
2019-07-05T03:48:40.4287458Z Selecting previously unselected package binutils-mingw-w64-x86-64.
2019-07-05T03:48:40.4307140Z Preparing to unpack .../binutils-mingw-w64-x86-64_2.26-3ubuntu1+6.6_amd64.deb ...
2019-07-05T03:48:40.4450122Z Unpacking binutils-mingw-w64-x86-64 (2.26-3ubuntu1+6.6) ...
2019-07-05T03:48:41.0353572Z Selecting previously unselected package mingw-w64-common.
2019-07-05T03:48:41.0369525Z Preparing to unpack .../mingw-w64-common_4.0.4-2_all.deb ...
2019-07-05T03:48:41.0487270Z Unpacking mingw-w64-common (4.0.4-2) ...
2019-07-05T03:48:42.4810558Z Selecting previously unselected package mingw-w64-i686-dev.
2019-07-05T03:48:42.4831091Z Preparing to unpack .../mingw-w64-i686-dev_4.0.4-2_all.deb ...
2019-07-05T03:48:42.4953874Z Unpacking mingw-w64-i686-dev (4.0.4-2) ...
2019-07-05T03:48:43.6243708Z Selecting previously unselected package gcc-mingw-w64-base.
2019-07-05T03:48:43.6273097Z Preparing to unpack .../gcc-mingw-w64-base_5.3.1-8ubuntu3+17_amd64.deb ...
2019-07-05T03:48:43.6409504Z Unpacking gcc-mingw-w64-base (5.3.1-8ubuntu3+17) ...
2019-07-05T03:48:43.7415045Z Selecting previously unselected package gcc-mingw-w64-i686.
2019-07-05T03:48:43.7433233Z Preparing to unpack .../gcc-mingw-w64-i686_5.3.1-8ubuntu3+17_amd64.deb ...
2019-07-05T03:48:43.7561168Z Unpacking gcc-mingw-w64-i686 (5.3.1-8ubuntu3+17) ...
2019-07-05T03:48:48.0996229Z Selecting previously unselected package g++-mingw-w64-i686.
2019-07-05T03:48:48.1018074Z Preparing to unpack .../g++-mingw-w64-i686_5.3.1-8ubuntu3+17_amd64.deb ...
2019-07-05T03:48:48.1213014Z Unpacking g++-mingw-w64-i686 (5.3.1-8ubuntu3+17) ...
2019-07-05T03:48:51.9660147Z Selecting previously unselected package mingw-w64-x86-64-dev.
2019-07-05T03:48:51.9691090Z Preparing to unpack .../mingw-w64-x86-64-dev_4.0.4-2_all.deb ...
2019-07-05T03:48:51.9830414Z Unpacking mingw-w64-x86-64-dev (4.0.4-2) ...
2019-07-05T03:48:54.0229187Z Selecting previously unselected package gcc-mingw-w64-x86-64.
2019-07-05T03:48:54.0263478Z Preparing to unpack .../gcc-mingw-w64-x86-64_5.3.1-8ubuntu3+17_amd64.deb ...
2019-07-05T03:48:54.0405357Z Unpacking gcc-mingw-w64-x86-64 (5.3.1-8ubuntu3+17) ...
2019-07-05T03:48:58.4920398Z Selecting previously unselected package g++-mingw-w64-x86-64.
2019-07-05T03:48:58.4943512Z Preparing to unpack .../g++-mingw-w64-x86-64_5.3.1-8ubuntu3+17_amd64.deb ...
2019-07-05T03:48:58.5092464Z Unpacking g++-mingw-w64-x86-64 (5.3.1-8ubuntu3+17) ...
2019-07-05T03:49:02.5917279Z Selecting previously unselected package g++-mingw-w64.
2019-07-05T03:49:02.5946177Z Preparing to unpack .../g++-mingw-w64_5.3.1-8ubuntu3+17_all.deb ...
2019-07-05T03:49:02.6099158Z Unpacking g++-mingw-w64 (5.3.1-8ubuntu3+17) ...
2019-07-05T03:49:02.7195582Z Selecting previously unselected package gcc-mingw-w64.
2019-07-05T03:49:02.7219884Z Preparing to unpack .../gcc-mingw-w64_5.3.1-8ubuntu3+17_all.deb ...
2019-07-05T03:49:02.7396483Z Unpacking gcc-mingw-w64 (5.3.1-8ubuntu3+17) ...
2019-07-05T03:49:02.8276289Z Selecting previously unselected package mingw-w64.
2019-07-05T03:49:02.8289036Z Preparing to unpack .../mingw-w64_4.0.4-2_all.deb ...
2019-07-05T03:49:02.8450531Z Unpacking mingw-w64 (4.0.4-2) ...
2019-07-05T03:49:03.2551008Z Setting up libgdbm3:amd64 (1.8.3-13.1) ...
2019-07-05T03:49:03.3232695Z Setting up libffi6:amd64 (3.2.1-4) ...
2019-07-05T03:49:03.3634272Z Setting up libglib2.0-0:amd64 (2.48.2-0ubuntu4.2) ...
2019-07-05T03:49:03.3916020Z No schema files found: doing nothing.
---
2019-07-05T03:49:08.5806736Z Setting up zlib1g-dev:amd64 (1:1.2.8.dfsg-2ubuntu4.1) ...
2019-07-05T03:49:08.6206512Z Setting up libssl-dev:amd64 (1.0.2g-1ubuntu4.15) ...
2019-07-05T03:49:08.6599654Z Setting up pkg-config (0.29.1-0ubuntu1) ...
2019-07-05T03:49:08.7438249Z Setting up python2.7 (2.7.12-1ubuntu0~16.04.4) ...
2019-07-05T03:49:09.5065100Z Setting up binutils-mingw-w64-i686 (2.26-3ubuntu1+6.6) ...
2019-07-05T03:49:09.5477397Z Setting up binutils-mingw-w64-x86-64 (2.26-3ubuntu1+6.6) ...
2019-07-05T03:49:09.5882741Z Setting up mingw-w64-common (4.0.4-2) ...
2019-07-05T03:49:09.6292749Z Setting up mingw-w64-i686-dev (4.0.4-2) ...
2019-07-05T03:49:09.6683598Z Setting up gcc-mingw-w64-base (5.3.1-8ubuntu3+17) ...
2019-07-05T03:49:09.7074387Z Setting up gcc-mingw-w64-i686 (5.3.1-8ubuntu3+17) ...
2019-07-05T03:49:09.7380387Z update-alternatives: using /usr/bin/i686-w64-mingw32-gcc-posix to provide /usr/bin/i686-w64-mingw32-gcc (i686-w64-mingw32-gcc) in auto mode
2019-07-05T03:49:09.7380886Z update-alternatives: warning: skip creation of /usr/bin/i686-w64-mingw32-gcc-5 because associated file /usr/bin/i686-w64-mingw32-gcc-5-posix (of link group i686-w64-mingw32-gcc) doesn't exist
2019-07-05T03:49:09.7480299Z update-alternatives: using /usr/bin/i686-w64-mingw32-gcc-win32 to provide /usr/bin/i686-w64-mingw32-gcc (i686-w64-mingw32-gcc) in auto mode
2019-07-05T03:49:09.7481076Z update-alternatives: warning: skip creation of /usr/bin/i686-w64-mingw32-gcc-5 because associated file /usr/bin/i686-w64-mingw32-gcc-5-win32 (of link group i686-w64-mingw32-gcc) doesn't exist
2019-07-05T03:49:09.7744002Z Setting up g++-mingw-w64-i686 (5.3.1-8ubuntu3+17) ...
2019-07-05T03:49:09.8027409Z update-alternatives: using /usr/bin/i686-w64-mingw32-g++-posix to provide /usr/bin/i686-w64-mingw32-g++ (i686-w64-mingw32-g++) in auto mode
2019-07-05T03:49:09.8118660Z update-alternatives: using /usr/bin/i686-w64-mingw32-g++-win32 to provide /usr/bin/i686-w64-mingw32-g++ (i686-w64-mingw32-g++) in auto mode
2019-07-05T03:49:09.8356014Z Setting up mingw-w64-x86-64-dev (4.0.4-2) ...
2019-07-05T03:49:09.8763349Z Setting up gcc-mingw-w64-x86-64 (5.3.1-8ubuntu3+17) ...
2019-07-05T03:49:09.9046558Z update-alternatives: using /usr/bin/x86_64-w64-mingw32-gcc-posix to provide /usr/bin/x86_64-w64-mingw32-gcc (x86_64-w64-mingw32-gcc) in auto mode
2019-07-05T03:49:09.9052430Z update-alternatives: warning: skip creation of /usr/bin/x86_64-w64-mingw32-gcc-5 because associated file /usr/bin/x86_64-w64-mingw32-gcc-5-posix (of link group x86_64-w64-mingw32-gcc) doesn't exist
2019-07-05T03:49:09.9147944Z update-alternatives: using /usr/bin/x86_64-w64-mingw32-gcc-win32 to provide /usr/bin/x86_64-w64-mingw32-gcc (x86_64-w64-mingw32-gcc) in auto mode
2019-07-05T03:49:09.9149464Z update-alternatives: warning: skip creation of /usr/bin/x86_64-w64-mingw32-gcc-5 because associated file /usr/bin/x86_64-w64-mingw32-gcc-5-win32 (of link group x86_64-w64-mingw32-gcc) doesn't exist
2019-07-05T03:49:09.9389467Z Setting up g++-mingw-w64-x86-64 (5.3.1-8ubuntu3+17) ...
2019-07-05T03:49:09.9670250Z update-alternatives: using /usr/bin/x86_64-w64-mingw32-g++-posix to provide /usr/bin/x86_64-w64-mingw32-g++ (x86_64-w64-mingw32-g++) in auto mode
2019-07-05T03:49:09.9759010Z update-alternatives: using /usr/bin/x86_64-w64-mingw32-g++-win32 to provide /usr/bin/x86_64-w64-mingw32-g++ (x86_64-w64-mingw32-g++) in auto mode
2019-07-05T03:49:10.0008747Z Setting up g++-mingw-w64 (5.3.1-8ubuntu3+17) ...
2019-07-05T03:49:10.0429562Z Setting up gcc-mingw-w64 (5.3.1-8ubuntu3+17) ...
2019-07-05T03:49:10.0903460Z Setting up mingw-w64 (4.0.4-2) ...
2019-07-05T03:49:10.1766598Z Processing triggers for ca-certificates (20170717~16.04.2) ...
2019-07-05T03:49:10.1932366Z Updating certificates in /etc/ssl/certs...
2019-07-05T03:49:11.6311114Z 148 added, 0 removed; done.
2019-07-05T03:49:11.6312431Z Running hooks in /etc/ca-certificates/update.d...
---
2019-07-05T03:49:50.8214634Z  ---> 5a411c557a11
2019-07-05T03:49:50.8255438Z Successfully built 5a411c557a11
2019-07-05T03:49:51.0166812Z Successfully tagged rust-ci:latest
2019-07-05T03:49:51.0948286Z Built container sha256:5a411c557a115bece3dc2201d29f49087a43d9fbde21dc0fcaee491134de46f2
2019-07-05T03:49:51.0965473Z Uploading finished image to https://.s3.amazonaws.com/docker/a4940e6914a5e1f6360ebc241d0d850aff083be5c43d268fd716ce48f63cb24e84238becb662f27f8c5d7740413ca17da24bb9cdc003ef32d7b03d2c9052b94d
2019-07-05T03:51:13.0236320Z upload failed: - to s3:///docker/a4940e6914a5e1f6360ebc241d0d850aff083be5c43d268fd716ce48f63cb24e84238becb662f27f8c5d7740413ca17da24bb9cdc003ef32d7b03d2c9052b94d Parameter validation failed:
2019-07-05T03:51:13.0240809Z Invalid bucket name "": Bucket name must match the regex "^[a-zA-Z0-9.\-_]{1,255}$"
2019-07-05T03:51:14.0117228Z [CI_JOB_NAME=Job2]
2019-07-05T03:51:14.0163578Z Starting sccache server...
2019-07-05T03:51:14.0764163Z configure: processing command line
2019-07-05T03:51:14.0765626Z configure: 
---
2019-07-05T03:58:05.0648816Z configure: build.locked-deps    := True
2019-07-05T03:58:05.0648851Z configure: llvm.ccache          := sccache
2019-07-05T03:58:05.0649012Z configure: build.cargo-native-static := True
2019-07-05T03:58:05.0649190Z configure: dist.missing-tools   := True
2019-07-05T03:58:05.0649386Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2019-07-05T03:58:05.0649479Z configure: writing `config.toml` in current directory
2019-07-05T03:58:05.0649775Z configure: 
2019-07-05T03:58:05.0650015Z configure: run `python /checkout/x.py --help`
2019-07-05T03:58:05.0650069Z configure: 
---
2019-07-05T04:06:32.6388525Z    Compiling compiletest v0.0.0 (/checkout/src/tools/compiletest)
2019-07-05T04:06:32.8204047Z error[E0432]: unresolved import `std::mem::take`
2019-07-05T04:06:32.8214058Z     --> src/tools/compiletest/src/runtest.rs:3611:9
2019-07-05T04:06:32.8219438Z      |
2019-07-05T04:06:32.8225293Z 3611 |     use std::mem::take;
2019-07-05T04:06:32.8229831Z      |         ^^^^^^^^^^^^^^ no `take` in `mem`
2019-07-05T04:06:33.7356801Z error: aborting due to previous error
2019-07-05T04:06:33.7357066Z 
2019-07-05T04:06:33.7357535Z For more information about this error, try `rustc --explain E0432`.
2019-07-05T04:06:33.7564397Z error: Could not compile `compiletest`.
---
2019-07-05T04:06:33.7587607Z 
2019-07-05T04:06:33.7587692Z 
2019-07-05T04:06:33.7617506Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 0 src/tools/compiletest
2019-07-05T04:06:33.7617824Z Build completed unsuccessfully in 0:01:48
2019-07-05T04:06:34.9465106Z ##[error]Bash exited with code '1'.
2019-07-05T04:06:34.9493605Z ##[section]Starting: Checkout
2019-07-05T04:06:34.9495652Z ==============================================================================
2019-07-05T04:06:34.9495711Z Task         : Get sources
2019-07-05T04:06:34.9495745Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
