plain
2019-07-09T14:43:20.1948078Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-09T14:43:20.2166998Z ##[command]git config gc.auto 0
2019-07-09T14:43:20.2242661Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-09T14:43:20.2312566Z ##[command]git config --get-all http.proxy
2019-07-09T14:43:20.2457059Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62532/merge:refs/remotes/pull/62532/merge
2019-07-09T14:43:23.7285263Z remote: Counting objects:   0% (1/139330)           
2019-07-09T14:43:23.7294899Z remote: Counting objects:   0% (19/139330)           
2019-07-09T14:43:23.7315813Z remote: Counting objects:   1% (1394/139330)           
2019-07-09T14:43:23.7336006Z remote: Counting objects:   2% (2787/139330)           
---
2019-07-09T14:43:54.7008415Z  * [new branch]          master                -> origin/master
2019-07-09T14:43:54.7010783Z  * [new branch]          mutable-overloaded-operators -> origin/mutable-overloaded-operators
2019-07-09T14:43:54.7014562Z  * [new branch]          stable                -> origin/stable
2019-07-09T14:43:54.7016950Z  * [new branch]          try                   -> origin/try
2019-07-09T14:43:54.7022725Z  * [new ref]             refs/pull/62532/merge -> pull/62532/merge
2019-07-09T14:43:54.7054178Z  * [new tag]             0.10                  -> 0.10
2019-07-09T14:43:54.7064953Z  * [new tag]             0.11.0                -> 0.11.0
2019-07-09T14:43:54.7068299Z  * [new tag]             0.12.0                -> 0.12.0
2019-07-09T14:43:54.7077298Z  * [new tag]             0.2                   -> 0.2
---
2019-07-09T14:43:54.7121233Z  * [new tag]             release-0.4           -> release-0.4
2019-07-09T14:43:54.7121596Z  * [new tag]             release-0.5           -> release-0.5
2019-07-09T14:43:54.7121890Z  * [new tag]             release-0.6           -> release-0.6
2019-07-09T14:43:54.7122291Z  * [new tag]             release-0.7           -> release-0.7
2019-07-09T14:43:54.8567723Z ##[command]git checkout --progress --force refs/remotes/pull/62532/merge
2019-07-09T14:43:55.8198437Z Checking out files:  75% (13974/18631)
2019-07-09T14:43:55.8272335Z Checking out files:  76% (14160/18631)
2019-07-09T14:43:55.8358514Z Checking out files:  77% (14346/18631)
2019-07-09T14:43:55.8435039Z Checking out files:  78% (14533/18631)
---
2019-07-09T14:43:55.9928691Z Checking out files:  98% (18259/18631)
2019-07-09T14:43:56.0040706Z Checking out files:  99% (18445/18631)
2019-07-09T14:43:56.0040891Z Checking out files: 100% (18631/18631)
2019-07-09T14:43:56.0040939Z Checking out files: 100% (18631/18631), done.
2019-07-09T14:43:56.0388124Z Note: checking out 'refs/remotes/pull/62532/merge'.
2019-07-09T14:43:56.0390833Z You are in 'detached HEAD' state. You can look around, make experimental
2019-07-09T14:43:56.0391763Z changes and commit them, and you can discard any commits you make in this
2019-07-09T14:43:56.0392681Z state without impacting any branches by performing another checkout.
2019-07-09T14:43:56.0393368Z 
2019-07-09T14:43:56.0393368Z 
2019-07-09T14:43:56.0394595Z If you want to create a new branch to retain commits you create, you may
2019-07-09T14:43:56.0396933Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-09T14:43:56.0398664Z 
2019-07-09T14:43:56.0400859Z   git checkout -b <new-branch-name>
2019-07-09T14:43:56.0402503Z 
2019-07-09T14:43:56.0404248Z HEAD is now at 439fa1fc8 Merge d7cb16758937a46c418c44df7baee4420729c4d2 into 5d8ffb5b8bdb3c7d1a59c3f8a7746735b73977be
2019-07-09T14:43:56.0547044Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-09T14:43:56.0549990Z ==============================================================================
2019-07-09T14:43:56.0550063Z Task         : Bash
2019-07-09T14:43:56.0550128Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-09T14:43:56.3823853Z BUILD_REPOSITORY_URI=***
2019-07-09T14:43:56.3823907Z BUILD_REQUESTEDFOREMAIL=
2019-07-09T14:43:56.3823952Z BUILD_REQUESTEDFOR=GitHub
2019-07-09T14:43:56.3824211Z BUILD_REQUESTEDFORID=5e5f2016-b710-434c-83b9-9adad28a8d3a
2019-07-09T14:43:56.3824313Z BUILD_SOURCEBRANCHNAME=merge
2019-07-09T14:43:56.3824563Z BUILD_SOURCEBRANCH=refs/pull/62532/merge
2019-07-09T14:43:56.3824621Z BUILD_SOURCESDIRECTORY=/home/vsts/work/1/s
2019-07-09T14:43:56.3824713Z BUILD_SOURCEVERSIONAUTHOR=Mark Rousskov
2019-07-09T14:43:56.3824764Z BUILD_SOURCEVERSION=c6ee0ac41cce5bd4899de4bbe76b1b19795174af
2019-07-09T14:43:56.3824824Z BUILD_SOURCEVERSIONMESSAGE=Merge d7cb16758937a46c418c44df7baee4420729c4d2 into 5d8ffb5b8bdb3c7d1a59c3f8a7746735b73977be
2019-07-09T14:43:56.3824976Z CARGO_HOME=/usr/local/cargo
2019-07-09T14:43:56.3825238Z CHROME_BIN=/usr/bin/google-chrome
2019-07-09T14:43:56.3825337Z COMMON_TESTRESULTSDIRECTORY=/home/vsts/work/1/TestResults
2019-07-09T14:43:56.3825610Z CONDA=/usr/share/miniconda
---
2019-07-09T14:43:56.3902379Z SYSTEM_PHASEID=c041ccde-92b7-5163-1d93-b1c06077f4b4
2019-07-09T14:43:56.3902431Z SYSTEM_PHASENAME=Linux
2019-07-09T14:43:56.3902737Z SYSTEM_PIPELINESTARTTIME=2019-07-09 14:43:12+00:00
2019-07-09T14:43:56.3903209Z SYSTEM_PLANID=120f93c5-0455-44d1-8d73-19f6eb647783
2019-07-09T14:43:56.3903274Z SYSTEM_PULLREQUEST_ISFORK=True
2019-07-09T14:43:56.3903320Z SYSTEM_PULLREQUEST_MERGEDAT=
2019-07-09T14:43:56.3903412Z SYSTEM_PULLREQUEST_PULLREQUESTID=295772302
2019-07-09T14:43:56.3903459Z SYSTEM_PULLREQUEST_PULLREQUESTNUMBER=62532
2019-07-09T14:43:56.3903747Z SYSTEM_PULLREQUEST_SOURCEBRANCH=syntax-print-cleanup
2019-07-09T14:43:56.3904008Z SYSTEM_PULLREQUEST_SOURCECOMMITID=d7cb16758937a46c418c44df7baee4420729c4d2
2019-07-09T14:43:56.3904375Z SYSTEM_PULLREQUEST_SOURCEREPOSITORYURI=***.git
2019-07-09T14:43:56.3904468Z SYSTEM_PULLREQUEST_TARGETBRANCH=master
2019-07-09T14:43:56.3904510Z SYSTEM_RESTRICTSECRETS=True
2019-07-09T14:43:56.3904588Z SYSTEM_STAGEATTEMPT=1
2019-07-09T14:43:56.3904665Z SYSTEM_STAGEDISPLAYNAME=__default
2019-07-09T14:43:56.3904907Z SYSTEM_STAGEID=96ac2280-8cb4-5df5-99de-dd2da759617d
2019-07-09T14:43:56.3904972Z SYSTEM_STAGENAME=__default
---
2019-07-09T14:44:10.8789271Z + modules=($modules)
2019-07-09T14:44:10.8789315Z + use_git=
2019-07-09T14:44:10.8793303Z ++ git config --file .gitmodules --get-regexp '\.url$'
2019-07-09T14:44:10.8795121Z ++ cut '-d ' -f2
2019-07-09T14:44:10.8812703Z + urls='***-installer.git
2019-07-09T14:44:10.8814138Z https://github.com/rust-lang/cargo.git
2019-07-09T14:44:10.8814674Z https://github.com/rust-lang-nursery/reference.git
2019-07-09T14:44:10.8821398Z https://github.com/rust-lang/book.git
2019-07-09T14:44:10.8822447Z https://github.com/rust-lang-nursery/rls.git
---
2019-07-09T14:44:10.9141760Z + commit=62b3ff2cb44dd8b648c3ef2a9347c3706d148014
2019-07-09T14:44:10.9142228Z + git rm src/doc/rust-by-example
2019-07-09T14:44:10.9142635Z + sh -c 'rm -f download-src-doc-book.tar.gz &&         curl -f -sSL -o download-src-doc-book.tar.gz https://github.com/rust-lang/book/archive/6c0d83499c8e77e06a71d28c5e1adccec278d4f3.tar.gz'
2019-07-09T14:44:10.9369620Z rm 'src/doc/rust-by-example'
2019-07-09T14:44:10.9370769Z + url=***-by-example.git
2019-07-09T14:44:10.9371399Z + url=***-by-example
2019-07-09T14:44:10.9372137Z + for i in '${!modules[@]}'
2019-07-09T14:44:10.9372644Z + module=src/llvm-emscripten
2019-07-09T14:44:10.9373242Z + [[  src/llvm-project src/llvm-emscripten src/doc/book src/doc/rust-by-example  = *\ \s\r\c\/\l\l\v\m\-\e\m\s\c\r\i\p\t\e\n\ * ]]
2019-07-09T14:44:10.9373242Z + [[  src/llvm-project src/llvm-emscripten src/doc/book src/doc/rust-by-example  = *\ \s\r\c\/\l\l\v\m\-\e\m\s\c\r\i\p\t\e\n\ * ]]
2019-07-09T14:44:10.9380478Z + fetch_github_commit_archive src/doc/rust-by-example ***-by-example/archive/62b3ff2cb44dd8b648c3ef2a9347c3706d148014.tar.gz
2019-07-09T14:44:10.9385281Z + local cached=download-src-doc-rust-by-example.tar.gz
2019-07-09T14:44:10.9385946Z ++ git ls-tree HEAD src/llvm-emscripten
2019-07-09T14:44:10.9388142Z ++ awk '{print $3}'
2019-07-09T14:44:10.9388142Z ++ awk '{print $3}'
2019-07-09T14:44:10.9389101Z + retry sh -c 'rm -f download-src-doc-rust-by-example.tar.gz &&         curl -f -sSL -o download-src-doc-rust-by-example.tar.gz ***-by-example/archive/62b3ff2cb44dd8b648c3ef2a9347c3706d148014.tar.gz'
2019-07-09T14:44:10.9395252Z + echo 'Attempting with retry:' sh -c 'rm -f download-src-doc-rust-by-example.tar.gz &&         curl -f -sSL -o download-src-doc-rust-by-example.tar.gz ***-by-example/archive/62b3ff2cb44dd8b648c3ef2a9347c3706d148014.tar.gz'
2019-07-09T14:44:10.9414104Z Attempting with retry: sh -c rm -f download-src-doc-rust-by-example.tar.gz &&         curl -f -sSL -o download-src-doc-rust-by-example.tar.gz ***-by-example/archive/62b3ff2cb44dd8b648c3ef2a9347c3706d148014.tar.gz
2019-07-09T14:44:10.9419984Z + local max=5
2019-07-09T14:44:10.9422649Z + commit=7f23313edff8beccb3fe44b815714269c5124c15
2019-07-09T14:44:10.9423440Z + git rm src/llvm-emscripten
2019-07-09T14:44:10.9424896Z + true
2019-07-09T14:44:10.9424896Z + true
2019-07-09T14:44:10.9428229Z + sh -c 'rm -f download-src-doc-rust-by-example.tar.gz &&         curl -f -sSL -o download-src-doc-rust-by-example.tar.gz ***-by-example/archive/62b3ff2cb44dd8b648c3ef2a9347c3706d148014.tar.gz'
2019-07-09T14:44:10.9674826Z + url=https://github.com/rust-lang/llvm.git
2019-07-09T14:44:10.9675128Z + url=https://github.com/rust-lang/llvm
2019-07-09T14:44:10.9675282Z + continue
2019-07-09T14:44:10.9675501Z + for i in '${!modules[@]}'
---
2019-07-09T14:44:11.2169632Z Submodule 'src/doc/edition-guide' (https://github.com/rust-lang-nursery/edition-guide.git) registered for path 'src/doc/edition-guide'
2019-07-09T14:44:11.2177541Z Submodule 'src/doc/embedded-book' (https://github.com/rust-embedded/book.git) registered for path 'src/doc/embedded-book'
2019-07-09T14:44:11.2185221Z Submodule 'src/doc/nomicon' (https://github.com/rust-lang-nursery/nomicon.git) registered for path 'src/doc/nomicon'
2019-07-09T14:44:11.2191491Z Submodule 'src/doc/reference' (https://github.com/rust-lang-nursery/reference.git) registered for path 'src/doc/reference'
2019-07-09T14:44:11.2195504Z Submodule 'src/doc/rustc-guide' (***c-guide.git) registered for path 'src/doc/rustc-guide'
2019-07-09T14:44:11.2211950Z Submodule 'src/tools/cargo' (https://github.com/rust-lang/cargo.git) registered for path 'src/tools/cargo'
2019-07-09T14:44:11.2221045Z Submodule 'src/tools/clippy' (https://github.com/rust-lang-nursery/rust-clippy.git) registered for path 'src/tools/clippy'
2019-07-09T14:44:11.2230467Z Submodule 'src/tools/miri' (https://github.com/rust-lang/miri.git) registered for path 'src/tools/miri'
2019-07-09T14:44:11.2237850Z Submodule 'src/tools/rls' (https://github.com/rust-lang-nursery/rls.git) registered for path 'src/tools/rls'
2019-07-09T14:44:11.2237850Z Submodule 'src/tools/rls' (https://github.com/rust-lang-nursery/rls.git) registered for path 'src/tools/rls'
2019-07-09T14:44:11.2245616Z Submodule 'src/rust-installer' (***-installer.git) registered for path 'src/tools/rust-installer'
2019-07-09T14:44:11.2371142Z Cloning into '/home/vsts/work/1/s/src/doc/edition-guide'...
2019-07-09T14:44:12.0964342Z + break
2019-07-09T14:44:12.0965799Z + mkdir src/doc/rust-by-example
2019-07-09T14:44:12.1033236Z + touch src/doc/rust-by-example/.git
---
2019-07-09T14:45:21.8526967Z Unpacking python3-setuptools (20.7.0-1) ...
2019-07-09T14:45:22.0543916Z Setting up python3-setuptools (20.7.0-1) ...
2019-07-09T14:45:23.7557330Z Attempting with retry: pip3 install awscli --upgrade --user
2019-07-09T14:45:25.3383547Z Collecting awscli
2019-07-09T14:45:26.9383851Z   Downloading https://files.pythonhosted.org/packages/7f/3f/9ecdb63eba199e04a5b0dcad0e409700269ef4860349604705c6fcffbd76/awscli-1.16.194-py2.py3-none-any.whl (1.7MB)
2019-07-09T14:45:26.9384086Z Collecting botocore==1.12.184 (from awscli)
2019-07-09T14:45:27.2983154Z   Downloading https://files.pythonhosted.org/packages/5a/7a/d40fabadbdc48fc4c15c087e69ab3f18e776b88662d5d118b03e09847673/botocore-1.12.184-py2.py3-none-any.whl (5.6MB)
2019-07-09T14:45:28.2714520Z   Downloading https://files.pythonhosted.org/packages/9f/2c/9417b5c774792634834e730932745bc09a7d36754ca00acf1ccd1ac2594d/PyYAML-5.1.tar.gz (274kB)
2019-07-09T14:45:28.8687994Z Collecting docutils>=0.10 (from awscli)
2019-07-09T14:45:28.9994120Z   Downloading https://files.pythonhosted.org/packages/36/fa/08e9e6e0e3cbd1d362c3bbee8d01d0aedb2155c4ac112b19ef3cae8eed8d/docutils-0.14-py3-none-any.whl (543kB)
2019-07-09T14:45:29.0940416Z Collecting colorama<=0.3.9,>=0.2.5 (from awscli)
---
2019-07-09T14:45:30.3526712Z   Downloading https://files.pythonhosted.org/packages/73/fb/00a976f728d0d1fecfe898238ce23f502a721c0ac0ecfedb80e0d88c64e9/six-1.12.0-py2.py3-none-any.whl
2019-07-09T14:45:30.3566738Z Building wheels for collected packages: PyYAML
2019-07-09T14:45:30.3570052Z   Running setup.py bdist_wheel for PyYAML: started
2019-07-09T14:45:30.5652523Z   Running setup.py bdist_wheel for PyYAML: finished with status 'error'
2019-07-09T14:45:30.5653089Z   Complete output from command /usr/bin/python3 -u -c "import setuptools, tokenize;__file__='/tmp/pip-build-ookjem1m/PyYAML/setup.py';exec(compile(getattr(tokenize, 'open', open)(__file__).read().replace('\r\n', '\n'), __file__, 'exec'))" bdist_wheel -d /tmp/tmpw42yn5fnpip-wheel- --python-tag cp35:
2019-07-09T14:45:30.5653773Z     warnings.warn(msg)
2019-07-09T14:45:30.5654130Z   usage: -c [global_opts] cmd1 [cmd1_opts] [cmd2 [cmd2_opts] ...]
2019-07-09T14:45:30.5654361Z      or: -c --help [cmd1 cmd2 ...]
2019-07-09T14:45:30.5654567Z      or: -c --help-commands
---
2019-07-09T14:45:41.3325560Z Attempting with retry: curl -y 30 -Y 10 --connect-timeout 30 -f -L -C - -o /tmp/rustci_docker_cache https://.s3.amazonaws.com/docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624
2019-07-09T14:45:41.3387754Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-09T14:45:41.3388175Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-09T14:45:41.3388332Z 
2019-07-09T14:45:41.3389402Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-09T14:45:42.3465429Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-09T14:45:42.3472281Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-09T14:45:42.3473923Z 
2019-07-09T14:45:42.3473923Z 
2019-07-09T14:45:42.3514282Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-09T14:45:44.3590470Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-09T14:45:44.3590675Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-09T14:45:44.3593064Z 
2019-07-09T14:45:44.3593064Z 
2019-07-09T14:45:44.3596313Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-09T14:45:47.3668551Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-09T14:45:47.3669044Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-09T14:45:47.3669300Z 
2019-07-09T14:45:47.3669300Z 
2019-07-09T14:45:47.3711947Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-09T14:45:51.3786678Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-09T14:45:51.3787451Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-09T14:45:51.3787701Z 
2019-07-09T14:45:51.3787701Z 
2019-07-09T14:45:51.3830501Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-09T14:45:51.3836583Z The command has failed after 5 attempts.
2019-07-09T14:45:51.4729964Z open /tmp/rustci_docker_cache: no such file or directory
2019-07-09T14:45:51.4760873Z Attempting with retry: docker build --rm -t rust-ci -f /home/vsts/work/1/s/src/ci/docker/x86_64-gnu-llvm-6.0/Dockerfile /home/vsts/work/1/s/src/ci/docker
2019-07-09T14:45:51.6247520Z Sending build context to Docker daemon  521.7kB
2019-07-09T14:45:51.6248535Z 
2019-07-09T14:45:51.6432643Z Step 1/8 : FROM ubuntu:16.04
---
2019-07-09T14:46:08.0043802Z Reading package lists...
2019-07-09T14:46:09.1234230Z Reading package lists...
2019-07-09T14:46:09.3621048Z Building dependency tree...
2019-07-09T14:46:09.3622268Z Reading state information...
2019-07-09T14:46:09.4978391Z The following additional packages will be installed:
2019-07-09T14:46:09.4979287Z   binfmt-support binutils bzip2 cmake-data cpp cpp-5 dpkg-dev g++-5 gcc gcc-5
2019-07-09T14:46:09.4984027Z   git-man libarchive13 libasan2 libasn1-8-heimdal libatomic1
2019-07-09T14:46:09.4984469Z   libbabeltrace-ctf1 libbabeltrace1 libbsd-dev libbsd0 libbz2-1.0 libc-dev-bin
2019-07-09T14:46:09.4984999Z   libedit2 liberror-perl libexpat1 libffi-dev libffi6 libgcc-5-dev libgdbm3
2019-07-09T14:46:09.4985302Z   libglib2.0-0 libgmp10 libgnutls30 libgomp1 libgssapi-krb5-2
2019-07-09T14:46:09.4985549Z   libgssapi3-heimdal libhcrypto4-heimdal libheimbase1-heimdal
2019-07-09T14:46:09.4985823Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-09T14:46:09.4985823Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-09T14:46:09.4986143Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-09T14:46:09.4986402Z   libkrb5-3 libkrb5support0 libldap-2.4-2 libllvm6.0 liblsan0 liblzo2-2
2019-07-09T14:46:09.4986658Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-09T14:46:09.4987113Z   libperl5.22 libpipeline1 libpython-stdlib libpython2.7-minimal
2019-07-09T14:46:09.4987380Z   libpython2.7-stdlib libpython3.5 libpython3.5-minimal libpython3.5-stdlib
2019-07-09T14:46:09.4987631Z   libquadmath0 libroken18-heimdal librtmp1 libsasl2-2 libsasl2-modules-db
2019-07-09T14:46:09.4987939Z   libsqlite3-0 libssl1.0.0 libstdc++-5-dev libtasn1-6 libtinfo-dev libtsan0
2019-07-09T14:46:09.4988190Z   libubsan0 libwind0-heimdal libxml2 linux-libc-dev llvm-6.0 llvm-6.0-dev
2019-07-09T14:46:09.4988440Z   llvm-6.0-runtime mime-support openssl patch perl perl-modules-5.22 python
2019-07-09T14:46:09.4988711Z   python-minimal python2.7-minimal
2019-07-09T14:46:09.4989861Z Suggested packages:
2019-07-09T14:46:09.4990277Z   binutils-doc bzip2-doc codeblocks eclipse ninja-build cpp-doc gcc-5-locales
2019-07-09T14:46:09.4990547Z   debian-keyring g++-multilib g++-5-multilib gcc-5-doc libstdc++6-5-dbg
2019-07-09T14:46:09.4990953Z   gcc-multilib manpages-dev autoconf automake libtool flex bison gcc-doc
2019-07-09T14:46:09.4991499Z   libasan2-dbg liblsan0-dbg libtsan0-dbg libubsan0-dbg libcilkrts5-dbg
2019-07-09T14:46:09.4991740Z   libmpx0-dbg libquadmath0-dbg gdb-doc gettext-base git-daemon-run
2019-07-09T14:46:09.4991740Z   libmpx0-dbg libquadmath0-dbg gdb-doc gettext-base git-daemon-run
2019-07-09T14:46:09.4992018Z   | git-daemon-sysvinit git-doc git-el git-email git-gui gitk gitweb git-arch
2019-07-09T14:46:09.4992274Z   git-cvs git-mediawiki git-svn lrzip glibc-doc gnutls-bin krb5-doc krb5-user
2019-07-09T14:46:09.4992514Z   libstdc++-5-doc llvm-6.0-doc make-doc ed diffutils-doc perl-doc
2019-07-09T14:46:09.4992805Z   libterm-readline-gnu-perl | libterm-readline-perl-perl python-doc python-tk
2019-07-09T14:46:09.4993012Z   python2.7-doc
2019-07-09T14:46:09.4993060Z Recommended packages:
2019-07-09T14:46:09.4993347Z   build-essential fakeroot libalgorithm-merge-perl libc-dbg gdbserver less
2019-07-09T14:46:09.4993813Z   rsync ssh-client manpages manpages-dev libfile-fcntllock-perl
2019-07-09T14:46:09.4994076Z   libglib2.0-data shared-mime-info xdg-user-dirs krb5-locales libsasl2-modules
2019-07-09T14:46:09.4994628Z   libssl-doc xml-core netbase rename
2019-07-09T14:46:09.5004377Z The following NEW packages will be installed:
2019-07-09T14:46:09.5008315Z   binfmt-support binutils bzip2 ca-certificates cmake cmake-data cpp cpp-5
2019-07-09T14:46:09.5009237Z   curl dpkg-dev file g++ g++-5 gcc gcc-5 gdb git git-man libarchive13 libasan2
2019-07-09T14:46:09.5011725Z   libasn1-8-heimdal libatomic1 libbabeltrace-ctf1 libbabeltrace1 libbsd-dev
2019-07-09T14:46:09.5012849Z   libbsd0 libc-dev-bin libc6-dev libcc1-0 libcilkrts5 libcurl3 libcurl3-gnutls
2019-07-09T14:46:09.5013821Z   libdpkg-perl libedit-dev libedit2 liberror-perl libexpat1 libffi-dev libffi6
2019-07-09T14:46:09.5014288Z   libgcc-5-dev libgdbm3 libglib2.0-0 libgmp10 libgnutls30 libgomp1
2019-07-09T14:46:09.5017250Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-09T14:46:09.5017786Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-09T14:46:09.5018303Z   libkrb5-3 libkrb5support0 libldap-2.4-2 libllvm6.0 liblsan0 liblzo2-2
2019-07-09T14:46:09.5018746Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-09T14:46:09.5018746Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-09T14:46:09.5019166Z   libperl5.22 libpipeline1 libpython-stdlib libpython2.7-minimal
2019-07-09T14:46:09.5019656Z   libpython2.7-stdlib libpython3.5 libpython3.5-minimal libpython3.5-stdlib
2019-07-09T14:46:09.5020089Z   libquadmath0 libroken18-heimdal librtmp1 libsasl2-2 libsasl2-modules-db
2019-07-09T14:46:09.5020567Z   libsqlite3-0 libssl-dev libssl1.0.0 libstdc++-5-dev libtasn1-6 libtinfo-dev
2019-07-09T14:46:09.5021004Z   libtsan0 libubsan0 libwind0-heimdal libxml2 linux-libc-dev llvm-6.0
2019-07-09T14:46:09.5021428Z   llvm-6.0-dev llvm-6.0-runtime llvm-6.0-tools make mime-support openssl patch
2019-07-09T14:46:09.5021891Z   perl perl-modules-5.22 pkg-config python python-minimal python2.7
2019-07-09T14:46:09.5022295Z   python2.7-minimal sudo xz-utils zlib1g-dev
2019-07-09T14:46:09.5022559Z The following packages will be upgraded:
2019-07-09T14:46:09.7820945Z 1 upgraded, 115 newly installed, 0 to remove and 4 not upgraded.
2019-07-09T14:46:09.7821152Z Need to get 121 MB of archives.
2019-07-09T14:46:09.7821251Z After this operation, 592 MB of additional disk space will be used.
2019-07-09T14:46:09.7822124Z Get:1 http://archive.ubuntu.com/ubuntu xenial/main amd64 libgdbm3 amd64 1.8.3-13.1 [16.9 kB]
---
2019-07-09T14:46:10.8692420Z Get:61 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 sudo amd64 1.8.16-0ubuntu1.7 [390 kB]
2019-07-09T14:46:10.8751354Z Get:62 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 openssl amd64 1.0.2g-1ubuntu4.15 [492 kB]
2019-07-09T14:46:10.8844284Z Get:63 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 ca-certificates all 20170717~16.04.2 [167 kB]
2019-07-09T14:46:10.8879464Z Get:64 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 libcurl3-gnutls amd64 7.47.0-1ubuntu2.13 [184 kB]
2019-07-09T14:46:10.8915082Z Get:65 http://archive.ubuntu.com/ubuntu xenial/main amd64 libedit2 amd64 3.1-20150325-1ubuntu2 [76.5 kB]
2019-07-09T14:46:10.8932423Z Get:66 http://archive.ubuntu.com/ubuntu xenial/main amd64 libpipeline1 amd64 1.4.1-2 [24.6 kB]
2019-07-09T14:46:10.8941005Z Get:67 http://archive.ubuntu.com/ubuntu xenial/main amd64 binfmt-support amd64 2.1.6-1 [50.7 kB]
2019-07-09T14:46:10.9510605Z Get:69 http://archive.ubuntu.com/ubuntu xenial/main amd64 libisl15 amd64 0.16.1-1 [524 kB]
2019-07-09T14:46:10.9592156Z Get:70 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 cpp-5 amd64 5.4.0-6ubuntu1~16.04.11 [7660 kB]
2019-07-09T14:46:11.0851567Z Get:71 http://archive.ubuntu.com/ubuntu xenial/main amd64 cpp amd64 4:5.3.1-1ubuntu1 [27.7 kB]
2019-07-09T14:46:11.0852308Z Get:72 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 curl amd64 7.47.0-1ubuntu2.13 [139 kB]
---
2019-07-09T14:46:29.5229159Z Unpacking ca-certificates (20170717~16.04.2) ...
2019-07-09T14:46:29.6821329Z Selecting previously unselected package libcurl3-gnutls:amd64.
2019-07-09T14:46:29.6838370Z Preparing to unpack .../libcurl3-gnutls_7.47.0-1ubuntu2.13_amd64.deb ...
2019-07-09T14:46:29.6978381Z Unpacking libcurl3-gnutls:amd64 (7.47.0-1ubuntu2.13) ...
2019-07-09T14:46:29.8338945Z Selecting previously unselected package libedit2:amd64.
2019-07-09T14:46:29.8357463Z Preparing to unpack .../libedit2_3.1-20150325-1ubuntu2_amd64.deb ...
2019-07-09T14:46:29.8491985Z Unpacking libedit2:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-09T14:46:29.9665967Z Selecting previously unselected package libpipeline1:amd64.
2019-07-09T14:46:29.9683141Z Preparing to unpack .../libpipeline1_1.4.1-2_amd64.deb ...
2019-07-09T14:46:29.9809216Z Unpacking libpipeline1:amd64 (1.4.1-2) ...
2019-07-09T14:46:30.0932687Z Selecting previously unselected package binfmt-support.
2019-07-09T14:46:30.0945997Z Preparing to unpack .../binfmt-support_2.1.6-1_amd64.deb ...
2019-07-09T14:46:30.1079764Z Unpacking binfmt-support (2.1.6-1) ...
2019-07-09T14:46:30.2186457Z Preparing to unpack .../binutils_2.26.1-1ubuntu1~16.04.8_amd64.deb ...
2019-07-09T14:46:30.2327178Z Unpacking binutils (2.26.1-1ubuntu1~16.04.8) ...
2019-07-09T14:46:30.7269440Z Selecting previously unselected package libisl15:amd64.
2019-07-09T14:46:30.7286393Z Preparing to unpack .../libisl15_0.16.1-1_amd64.deb ...
---
2019-07-09T14:46:42.1986135Z Selecting previously unselected package libtinfo-dev:amd64.
2019-07-09T14:46:42.2004794Z Preparing to unpack .../libtinfo-dev_6.0+20160213-1ubuntu1_amd64.deb ...
2019-07-09T14:46:42.2155099Z Unpacking libtinfo-dev:amd64 (6.0+20160213-1ubuntu1) ...
2019-07-09T14:46:42.3137441Z Selecting previously unselected package libedit-dev:amd64.
2019-07-09T14:46:42.3156641Z Preparing to unpack .../libedit-dev_3.1-20150325-1ubuntu2_amd64.deb ...
2019-07-09T14:46:42.3284110Z Unpacking libedit-dev:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-09T14:46:42.4462103Z Selecting previously unselected package libllvm6.0:amd64.
2019-07-09T14:46:42.4483340Z Preparing to unpack .../libllvm6.0_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-09T14:46:42.4622511Z Unpacking libllvm6.0:amd64 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-09T14:46:45.4720033Z Selecting previously unselected package zlib1g-dev:amd64.
2019-07-09T14:46:45.4738356Z Preparing to unpack .../zlib1g-dev_1%3a1.2.8.dfsg-2ubuntu4.1_amd64.deb ...
2019-07-09T14:46:45.4871031Z Unpacking zlib1g-dev:amd64 (1:1.2.8.dfsg-2ubuntu4.1) ...
2019-07-09T14:46:45.5932820Z Preparing to unpack .../libssl-dev_1.0.2g-1ubuntu4.15_amd64.deb ...
2019-07-09T14:46:45.6091689Z Unpacking libssl-dev:amd64 (1.0.2g-1ubuntu4.15) ...
2019-07-09T14:46:45.9324492Z Selecting previously unselected package llvm-6.0-runtime.
2019-07-09T14:46:45.9324492Z Selecting previously unselected package llvm-6.0-runtime.
2019-07-09T14:46:45.9348014Z Preparing to unpack .../llvm-6.0-runtime_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-09T14:46:45.9495682Z Unpacking llvm-6.0-runtime (1:6.0-1ubuntu2~16.04.1) ...
2019-07-09T14:46:46.0807184Z Selecting previously unselected package llvm-6.0.
2019-07-09T14:46:46.0826939Z Preparing to unpack .../llvm-6.0_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-09T14:46:46.0983439Z Unpacking llvm-6.0 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-09T14:46:46.7686189Z Selecting previously unselected package libffi-dev:amd64.
2019-07-09T14:46:46.7706659Z Preparing to unpack .../libffi-dev_3.2.1-4_amd64.deb ...
2019-07-09T14:46:46.7841840Z Unpacking libffi-dev:amd64 (3.2.1-4) ...
2019-07-09T14:46:46.9047995Z Selecting previously unselected package llvm-6.0-dev.
2019-07-09T14:46:46.9068233Z Preparing to unpack .../llvm-6.0-dev_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-09T14:46:46.9201645Z Unpacking llvm-6.0-dev (1:6.0-1ubuntu2~16.04.1) ...
2019-07-09T14:46:51.6128553Z Selecting previously unselected package llvm-6.0-tools.
2019-07-09T14:46:51.6158568Z Preparing to unpack .../llvm-6.0-tools_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-09T14:46:51.6297177Z Unpacking llvm-6.0-tools (1:6.0-1ubuntu2~16.04.1) ...
2019-07-09T14:46:51.8000906Z Selecting previously unselected package pkg-config.
2019-07-09T14:46:51.8026678Z Preparing to unpack .../pkg-config_0.29.1-0ubuntu1_amd64.deb ...
2019-07-09T14:46:51.8184265Z Unpacking pkg-config (0.29.1-0ubuntu1) ...
2019-07-09T14:46:51.9580845Z Processing triggers for systemd (229-4ubuntu21.21) ...
2019-07-09T14:46:52.3762426Z Setting up libgdbm3:amd64 (1.8.3-13.1) ...
2019-07-09T14:46:52.4504244Z Setting up libffi6:amd64 (3.2.1-4) ...
2019-07-09T14:46:52.4898382Z Setting up libglib2.0-0:amd64 (2.48.2-0ubuntu4.3) ...
---
2019-07-09T14:46:56.4366853Z debconf: unable to initialize frontend: Dialog
2019-07-09T14:46:56.4367037Z debconf: (TERM is not set, so the dialog frontend is not usable.)
2019-07-09T14:46:56.4367294Z debconf: falling back to frontend: Readline
2019-07-09T14:46:56.9407572Z Setting up libcurl3-gnutls:amd64 (7.47.0-1ubuntu2.13) ...
2019-07-09T14:46:56.9828447Z Setting up libedit2:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-09T14:46:57.0238118Z Setting up libpipeline1:amd64 (1.4.1-2) ...
2019-07-09T14:46:57.0650822Z Setting up binfmt-support (2.1.6-1) ...
2019-07-09T14:46:57.1454430Z mount: permission denied
2019-07-09T14:46:57.1455922Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-09T14:46:57.1475879Z mount: permission denied
2019-07-09T14:46:57.1478041Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-09T14:46:57.3263179Z invoke-rc.d: could not determine current runlevel
2019-07-09T14:46:57.3291724Z invoke-rc.d: policy-rc.d denied execution of start.
2019-07-09T14:46:57.3878484Z Setting up libisl15:amd64 (0.16.1-1) ...
2019-07-09T14:46:57.4310302Z Setting up cpp-5 (5.4.0-6ubuntu1~16.04.11) ...
2019-07-09T14:46:57.4703395Z Setting up cpp (4:5.3.1-1ubuntu1) ...
2019-07-09T14:46:57.5172557Z Setting up curl (7.47.0-1ubuntu2.13) ...
---
2019-07-09T14:46:59.3126575Z Setting up libedit-dev:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-09T14:46:59.3553699Z Setting up libllvm6.0:amd64 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-09T14:46:59.3981703Z Setting up zlib1g-dev:amd64 (1:1.2.8.dfsg-2ubuntu4.1) ...
2019-07-09T14:46:59.4401300Z Setting up libssl-dev:amd64 (1.0.2g-1ubuntu4.15) ...
2019-07-09T14:46:59.4813699Z Setting up llvm-6.0-runtime (1:6.0-1ubuntu2~16.04.1) ...
2019-07-09T14:46:59.5137590Z mount: permission denied
2019-07-09T14:46:59.5139406Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-09T14:46:59.5287787Z Setting up llvm-6.0 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-09T14:46:59.5706695Z Setting up libffi-dev:amd64 (3.2.1-4) ...
2019-07-09T14:46:59.6144331Z Setting up llvm-6.0-dev (1:6.0-1ubuntu2~16.04.1) ...
2019-07-09T14:46:59.6564953Z Setting up llvm-6.0-tools (1:6.0-1ubuntu2~16.04.1) ...
2019-07-09T14:46:59.7005178Z Setting up pkg-config (0.29.1-0ubuntu1) ...
2019-07-09T14:46:59.8366331Z Processing triggers for ca-certificates (20170717~16.04.2) ...
2019-07-09T14:46:59.8544730Z Updating certificates in /etc/ssl/certs...
2019-07-09T14:47:01.4955606Z 148 added, 0 removed; done.
2019-07-09T14:47:01.4957106Z Running hooks in /etc/ca-certificates/update.d...
---
2019-07-09T14:47:33.3526161Z  ---> 96e1f9557f8e
2019-07-09T14:47:33.3566150Z Successfully built 96e1f9557f8e
2019-07-09T14:47:33.5350664Z Successfully tagged rust-ci:latest
2019-07-09T14:47:33.6047442Z Built container sha256:96e1f9557f8e4ed694da9db553e0ff3c652e097bddbe0b860d86c6dbf96fcce1
2019-07-09T14:47:33.6128410Z Uploading finished image to https://.s3.amazonaws.com/docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624
2019-07-09T14:48:36.7370158Z upload failed: - to s3:///docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624 Parameter validation failed:
2019-07-09T14:48:36.7371928Z Invalid bucket name "": Bucket name must match the regex "^[a-zA-Z0-9.\-_]{1,255}$"
2019-07-09T14:48:37.8958991Z [CI_JOB_NAME=Job1]
2019-07-09T14:48:37.9011856Z Starting sccache server...
2019-07-09T14:48:37.9531308Z configure: processing command line
2019-07-09T14:48:37.9532212Z configure: 
---
2019-07-09T14:50:44.9382434Z   Downloaded termcolor v1.0.4
2019-07-09T14:50:44.9788657Z   Downloaded want v0.2.0
2019-07-09T14:50:45.0378659Z   Downloaded either v1.5.0
2019-07-09T14:50:45.0550344Z   Downloaded fuchsia-cprng v0.1.1
2019-07-09T14:50:45.0916620Z   Downloaded try-lock v0.2.2
2019-07-09T14:50:45.2479521Z   Downloaded arc-swap v0.3.7
2019-07-09T14:50:45.2830478Z   Downloaded rayon-core v1.5.0
2019-07-09T14:50:45.5428028Z   Downloaded rand_hc v0.1.0
2019-07-09T14:50:45.5619641Z   Downloaded quote v0.3.15
---
2019-07-09T14:50:46.0162802Z   Downloaded ucd-util v0.1.3
2019-07-09T14:50:46.0487446Z   Downloaded home v0.3.3
2019-07-09T14:50:46.0681307Z   Downloaded rustc-ap-rustc_cratesio_shim v491.0.0
2019-07-09T14:50:46.1227780Z   Downloaded elasticlunr-rs v2.3.4
2019-07-09T14:50:46.1382355Z   Downloaded crossbeam-queue v0.1.2
2019-07-09T14:50:46.1916253Z   Downloaded tokio-reactor v0.1.8
2019-07-09T14:50:46.2299370Z   Downloaded adler32 v1.0.3
2019-07-09T14:50:46.3185125Z   Downloaded curl-sys v0.4.18
2019-07-09T14:50:46.5664827Z   Downloaded crc32fast v1.1.2
---
2019-07-09T14:50:46.8090602Z   Downloaded parking_lot v0.7.1
2019-07-09T14:50:46.8546269Z   Downloaded libz-sys v1.0.25
2019-07-09T14:50:46.8869147Z   Downloaded jsonrpc-core v12.0.0
2019-07-09T14:50:46.9068300Z   Downloaded crossbeam-utils v0.2.2
2019-07-09T14:50:46.9237100Z   Downloaded security-framework-sys v0.3.1
2019-07-09T14:50:46.9848005Z   Downloaded rand_xorshift v0.1.0
2019-07-09T14:50:47.0250240Z   Downloaded native-tls v0.2.3
2019-07-09T14:50:47.0465794Z   Downloaded ammonia v1.2.0
2019-07-09T14:50:47.0828673Z   Downloaded pkg-config v0.3.14
---
2019-07-09T14:50:48.0288871Z   Downloaded xz2 v0.1.5
2019-07-09T14:50:48.0459962Z   Downloaded memoffset v0.2.1
2019-07-09T14:50:48.0934019Z   Downloaded slab v0.4.2
2019-07-09T14:50:48.1118137Z   Downloaded rls-span v0.5.1
2019-07-09T14:50:48.1742818Z   Downloaded tokio-buf v0.1.1
2019-07-09T14:50:48.2326993Z   Downloaded arrayref v0.3.5
2019-07-09T14:50:48.2498048Z   Downloaded foreign-types v0.3.2
2019-07-09T14:50:48.2976496Z   Downloaded dlmalloc v0.1.3
2019-07-09T14:50:48.3140920Z   Downloaded tokio-fs v0.1.5
---
2019-07-09T14:50:50.9051656Z   Downloaded yaml-rust v0.3.5
2019-07-09T14:50:50.9472443Z   Downloaded is-match v0.1.0
2019-07-09T14:50:50.9919860Z   Downloaded redox_termios v0.1.1
2019-07-09T14:50:51.0154226Z   Downloaded textwrap v0.10.0
2019-07-09T14:50:51.0385404Z   Downloaded reqwest v0.9.11
2019-07-09T14:50:51.0974462Z   Downloaded synstructure v0.10.2
2019-07-09T14:50:51.1243064Z   Downloaded pulldown-cmark v0.5.2
2019-07-09T14:50:51.1526186Z   Downloaded unicode_categories v0.1.1
2019-07-09T14:50:51.1703818Z   Downloaded openssl v0.10.16
---
2019-07-09T14:50:56.3327927Z   Downloaded miniz_oxide_c_api v0.2.0
2019-07-09T14:50:56.3634185Z   Downloaded encoding_rs v0.8.17
2019-07-09T14:50:56.3963217Z   Downloaded redox_users v0.3.0
2019-07-09T14:50:56.4265705Z   Downloaded html5ever v0.22.5
2019-07-09T14:50:56.4423410Z   Downloaded futures-cpupool v0.1.8
2019-07-09T14:50:56.5845248Z   Downloaded compiler_builtins v0.1.16
2019-07-09T14:50:56.6094551Z   Downloaded string_cache_codegen v0.4.2
2019-07-09T14:50:56.6359016Z   Downloaded rand_core v0.4.0
2019-07-09T14:50:56.6666213Z   Downloaded digest v0.7.6
---
2019-07-09T14:52:23.4341031Z    Compiling serde_json v1.0.33
2019-07-09T14:52:28.0049225Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-07-09T14:52:37.2398486Z     Finished release [optimized] target(s) in 1m 37s
2019-07-09T14:52:37.2475143Z tidy check
2019-07-09T14:52:37.4267873Z tidy error: /checkout/src/libsyntax/print/pprust.rs: ignoring file length unnecessarily
2019-07-09T14:52:39.1057473Z some tidy checks failed
2019-07-09T14:52:39.1065931Z 
2019-07-09T14:52:39.1065931Z 
2019-07-09T14:52:39.1068533Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-07-09T14:52:39.1068680Z 
2019-07-09T14:52:39.1068766Z 
2019-07-09T14:52:39.1073488Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-07-09T14:52:39.1074811Z Build completed unsuccessfully in 0:01:40
2019-07-09T14:52:39.1074811Z Build completed unsuccessfully in 0:01:40
2019-07-09T14:52:40.4793970Z ##[error]Bash exited with code '1'.
2019-07-09T14:52:40.4827990Z ##[section]Starting: Checkout
2019-07-09T14:52:40.4829764Z ==============================================================================
2019-07-09T14:52:40.4829820Z Task         : Get sources
2019-07-09T14:52:40.4829869Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
