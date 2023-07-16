plain
2019-07-09T13:11:01.4034644Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-09T13:11:01.4200515Z ##[command]git config gc.auto 0
2019-07-09T13:11:01.4285333Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-09T13:11:01.4340933Z ##[command]git config --get-all http.proxy
2019-07-09T13:11:01.4485695Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/61946/merge:refs/remotes/pull/61946/merge
2019-07-09T13:11:06.4932633Z remote: Counting objects:   0% (1/139650)           
2019-07-09T13:11:06.4947633Z remote: Counting objects:   0% (29/139650)           
2019-07-09T13:11:06.4965136Z remote: Counting objects:   1% (1397/139650)           
2019-07-09T13:11:06.4989070Z remote: Counting objects:   2% (2793/139650)           
---
2019-07-09T13:11:38.4064040Z  * [new branch]          master                -> origin/master
2019-07-09T13:11:38.4073933Z  * [new branch]          mutable-overloaded-operators -> origin/mutable-overloaded-operators
2019-07-09T13:11:38.4079283Z  * [new branch]          stable                -> origin/stable
2019-07-09T13:11:38.4081742Z  * [new branch]          try                   -> origin/try
2019-07-09T13:11:38.4086037Z  * [new ref]             refs/pull/61946/merge -> pull/61946/merge
2019-07-09T13:11:38.4117656Z  * [new tag]             0.10                  -> 0.10
2019-07-09T13:11:38.4118647Z  * [new tag]             0.11.0                -> 0.11.0
2019-07-09T13:11:38.4120410Z  * [new tag]             0.12.0                -> 0.12.0
2019-07-09T13:11:38.4122285Z  * [new tag]             0.2                   -> 0.2
---
2019-07-09T13:11:38.4206619Z  * [new tag]             release-0.4           -> release-0.4
2019-07-09T13:11:38.4207106Z  * [new tag]             release-0.5           -> release-0.5
2019-07-09T13:11:38.4207614Z  * [new tag]             release-0.6           -> release-0.6
2019-07-09T13:11:38.4208117Z  * [new tag]             release-0.7           -> release-0.7
2019-07-09T13:11:38.5446101Z ##[command]git checkout --progress --force refs/remotes/pull/61946/merge
2019-07-09T13:11:39.5185592Z Checking out files:  75% (14017/18689)
2019-07-09T13:11:39.5259955Z Checking out files:  76% (14204/18689)
2019-07-09T13:11:39.5343247Z Checking out files:  77% (14391/18689)
2019-07-09T13:11:39.5527773Z Checking out files:  78% (14578/18689)
---
2019-07-09T13:11:39.6887088Z Checking out files:  98% (18316/18689)
2019-07-09T13:11:39.6993718Z Checking out files:  99% (18503/18689)
2019-07-09T13:11:39.6993798Z Checking out files: 100% (18689/18689)
2019-07-09T13:11:39.6996763Z Checking out files: 100% (18689/18689), done.
2019-07-09T13:11:39.7320115Z Note: checking out 'refs/remotes/pull/61946/merge'.
2019-07-09T13:11:39.7320539Z You are in 'detached HEAD' state. You can look around, make experimental
2019-07-09T13:11:39.7320600Z changes and commit them, and you can discard any commits you make in this
2019-07-09T13:11:39.7320668Z state without impacting any branches by performing another checkout.
2019-07-09T13:11:39.7320700Z 
2019-07-09T13:11:39.7320700Z 
2019-07-09T13:11:39.7320748Z If you want to create a new branch to retain commits you create, you may
2019-07-09T13:11:39.7321075Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-09T13:11:39.7321115Z 
2019-07-09T13:11:39.7321400Z   git checkout -b <new-branch-name>
2019-07-09T13:11:39.7321435Z 
2019-07-09T13:11:39.7321489Z HEAD is now at 7317baf8f Merge 781bddf9d43a5228f0e0b3c0b184e08a44023e84 into 5d8ffb5b8bdb3c7d1a59c3f8a7746735b73977be
2019-07-09T13:11:39.7454505Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-09T13:11:39.7457231Z ==============================================================================
2019-07-09T13:11:39.7457292Z Task         : Bash
2019-07-09T13:11:39.7457356Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-09T13:11:40.0742684Z BUILD_REPOSITORY_URI=***
2019-07-09T13:11:40.0743038Z BUILD_REQUESTEDFOREMAIL=
2019-07-09T13:11:40.0743356Z BUILD_REQUESTEDFOR=GitHub
2019-07-09T13:11:40.0744393Z BUILD_REQUESTEDFORID=5e5f2016-b710-434c-83b9-9adad28a8d3a
2019-07-09T13:11:40.0744806Z BUILD_SOURCEBRANCHNAME=merge
2019-07-09T13:11:40.0745324Z BUILD_SOURCEBRANCH=refs/pull/61946/merge
2019-07-09T13:11:40.0745908Z BUILD_SOURCEVERSION=621c781adf66b3ce09081ead97e882f5941fe3fc
2019-07-09T13:11:40.0745908Z BUILD_SOURCEVERSION=621c781adf66b3ce09081ead97e882f5941fe3fc
2019-07-09T13:11:40.0746225Z BUILD_SOURCEVERSIONAUTHOR=Baoshan
2019-07-09T13:11:40.0746481Z BUILD_SOURCEVERSIONMESSAGE=Merge 781bddf9d43a5228f0e0b3c0b184e08a44023e84 into 5d8ffb5b8bdb3c7d1a59c3f8a7746735b73977be
2019-07-09T13:11:40.0747049Z CARGO_HOME=/usr/local/cargo
2019-07-09T13:11:40.0747665Z CHROME_BIN=/usr/bin/google-chrome
2019-07-09T13:11:40.0748069Z COMMON_TESTRESULTSDIRECTORY=/home/vsts/work/1/TestResults
2019-07-09T13:11:40.0748320Z CONDA=/usr/share/miniconda
---
2019-07-09T13:11:40.0773383Z SYSTEM_PHASEDISPLAYNAME=Linux
2019-07-09T13:11:40.0774251Z SYSTEM_PHASEID=c041ccde-92b7-5163-1d93-b1c06077f4b4
2019-07-09T13:11:40.0774539Z SYSTEM_PHASENAME=Linux
2019-07-09T13:11:40.0775074Z SYSTEM_PIPELINESTARTTIME=2019-07-09 13:10:52+00:00
2019-07-09T13:11:40.0775718Z SYSTEM_PLANID=5fd0158e-6831-4db5-b407-4b8c055f3128
2019-07-09T13:11:40.0776052Z SYSTEM_PULLREQUEST_ISFORK=True
2019-07-09T13:11:40.0776338Z SYSTEM_PULLREQUEST_MERGEDAT=
2019-07-09T13:11:40.0776555Z SYSTEM_PULLREQUEST_PULLREQUESTID=289498677
2019-07-09T13:11:40.0776713Z SYSTEM_PULLREQUEST_PULLREQUESTNUMBER=61946
2019-07-09T13:11:40.0776869Z SYSTEM_PULLREQUEST_SOURCEBRANCH=vxworks
2019-07-09T13:11:40.0777086Z SYSTEM_PULLREQUEST_SOURCECOMMITID=781bddf9d43a5228f0e0b3c0b184e08a44023e84
2019-07-09T13:11:40.0777651Z SYSTEM_PULLREQUEST_SOURCEREPOSITORYURI=***.git
2019-07-09T13:11:40.0777953Z SYSTEM_PULLREQUEST_TARGETBRANCH=master
2019-07-09T13:11:40.0778178Z SYSTEM_RESTRICTSECRETS=True
2019-07-09T13:11:40.0778491Z SYSTEM_STAGEATTEMPT=1
2019-07-09T13:11:40.0780323Z SYSTEM_STAGEDISPLAYNAME=__default
2019-07-09T13:11:40.0785740Z SYSTEM_STAGEID=96ac2280-8cb4-5df5-99de-dd2da759617d
2019-07-09T13:11:40.0786176Z SYSTEM_STAGENAME=__default
---
2019-07-09T13:11:40.3575204Z 408 ./src/test/run-pass/macros
2019-07-09T13:11:40.3575457Z 400 ./src/liballoc/collections
2019-07-09T13:11:40.3575614Z 384 ./src/libstd/os
2019-07-09T13:11:40.3576122Z 380 ./src/test/run-pass/binding
2019-07-09T13:11:40.3576352Z 380 ./src/libstd/sys/vxworks
2019-07-09T13:11:40.3576782Z 364 ./src/librustc_apfloat
2019-07-09T13:11:40.3576940Z 360 ./.git/refs
2019-07-09T13:11:40.3577375Z 352 ./src/test/ui/nll/user-annotations
2019-07-09T13:11:40.3577941Z 352 ./src/test/run-pass/issues/auxiliary
---
2019-07-09T13:11:55.2508881Z + modules=($modules)
2019-07-09T13:11:55.2509266Z + use_git=
2019-07-09T13:11:55.2509816Z ++ cut '-d ' -f2
2019-07-09T13:11:55.2514127Z ++ git config --file .gitmodules --get-regexp '\.url$'
2019-07-09T13:11:55.2605705Z + urls='***-installer.git
2019-07-09T13:11:55.2611269Z https://github.com/rust-lang/cargo.git
2019-07-09T13:11:55.2612018Z https://github.com/rust-lang-nursery/reference.git
2019-07-09T13:11:55.2612649Z https://github.com/rust-lang/book.git
2019-07-09T13:11:55.2613353Z https://github.com/rust-lang-nursery/rls.git
---
2019-07-09T13:11:55.6548130Z + local max=5
2019-07-09T13:11:55.6548408Z + true
2019-07-09T13:11:55.6549237Z + sh -c 'rm -f download-src-doc-book.tar.gz &&         curl -f -sSL -o download-src-doc-book.tar.gz https://github.com/rust-lang/book/archive/6c0d83499c8e77e06a71d28c5e1adccec278d4f3.tar.gz'
2019-07-09T13:11:55.6822696Z rm 'src/doc/rust-by-example'
2019-07-09T13:11:55.6826530Z + url=***-by-example.git
2019-07-09T13:11:55.6827646Z + url=***-by-example
2019-07-09T13:11:55.6831345Z + for i in '${!modules[@]}'
2019-07-09T13:11:55.6832076Z + module=src/llvm-emscripten
2019-07-09T13:11:55.6832534Z + [[  src/llvm-project src/llvm-emscripten src/doc/book src/doc/rust-by-example  = *\ \s\r\c\/\l\l\v\m\-\e\m\s\c\r\i\p\t\e\n\ * ]]
2019-07-09T13:11:55.6832534Z + [[  src/llvm-project src/llvm-emscripten src/doc/book src/doc/rust-by-example  = *\ \s\r\c\/\l\l\v\m\-\e\m\s\c\r\i\p\t\e\n\ * ]]
2019-07-09T13:11:55.6833140Z + fetch_github_commit_archive src/doc/rust-by-example ***-by-example/archive/62b3ff2cb44dd8b648c3ef2a9347c3706d148014.tar.gz
2019-07-09T13:11:55.6838245Z + local cached=download-src-doc-rust-by-example.tar.gz
2019-07-09T13:11:55.6838245Z + local cached=download-src-doc-rust-by-example.tar.gz
2019-07-09T13:11:55.6839144Z + retry sh -c 'rm -f download-src-doc-rust-by-example.tar.gz &&         curl -f -sSL -o download-src-doc-rust-by-example.tar.gz ***-by-example/archive/62b3ff2cb44dd8b648c3ef2a9347c3706d148014.tar.gz'
2019-07-09T13:11:55.6843195Z + echo 'Attempting with retry:' sh -c 'rm -f download-src-doc-rust-by-example.tar.gz &&         curl -f -sSL -o download-src-doc-rust-by-example.tar.gz ***-by-example/archive/62b3ff2cb44dd8b648c3ef2a9347c3706d148014.tar.gz'
2019-07-09T13:11:55.6843930Z + local max=5
2019-07-09T13:11:55.6844091Z + true
2019-07-09T13:11:55.6844091Z + true
2019-07-09T13:11:55.6844918Z + sh -c 'rm -f download-src-doc-rust-by-example.tar.gz &&         curl -f -sSL -o download-src-doc-rust-by-example.tar.gz ***-by-example/archive/62b3ff2cb44dd8b648c3ef2a9347c3706d148014.tar.gz'
2019-07-09T13:11:55.6845767Z Attempting with retry: sh -c rm -f download-src-doc-rust-by-example.tar.gz &&         curl -f -sSL -o download-src-doc-rust-by-example.tar.gz ***-by-example/archive/62b3ff2cb44dd8b648c3ef2a9347c3706d148014.tar.gz
2019-07-09T13:11:55.6850800Z ++ git ls-tree HEAD src/llvm-emscripten
2019-07-09T13:11:55.6874138Z + commit=7f23313edff8beccb3fe44b815714269c5124c15
2019-07-09T13:11:55.6874936Z + git rm src/llvm-emscripten
2019-07-09T13:11:55.7472119Z rm 'src/llvm-emscripten'
---
2019-07-09T13:11:56.0563649Z Submodule 'src/doc/edition-guide' (https://github.com/rust-lang-nursery/edition-guide.git) registered for path 'src/doc/edition-guide'
2019-07-09T13:11:56.0570904Z Submodule 'src/doc/embedded-book' (https://github.com/rust-embedded/book.git) registered for path 'src/doc/embedded-book'
2019-07-09T13:11:56.0573524Z Submodule 'src/doc/nomicon' (https://github.com/rust-lang-nursery/nomicon.git) registered for path 'src/doc/nomicon'
2019-07-09T13:11:56.0579402Z Submodule 'src/doc/reference' (https://github.com/rust-lang-nursery/reference.git) registered for path 'src/doc/reference'
2019-07-09T13:11:56.0582956Z Submodule 'src/doc/rustc-guide' (***c-guide.git) registered for path 'src/doc/rustc-guide'
2019-07-09T13:11:56.0591423Z Submodule 'src/tools/cargo' (https://github.com/rust-lang/cargo.git) registered for path 'src/tools/cargo'
2019-07-09T13:11:56.0595495Z Submodule 'src/tools/clippy' (https://github.com/rust-lang-nursery/rust-clippy.git) registered for path 'src/tools/clippy'
2019-07-09T13:11:56.0600100Z Submodule 'src/tools/miri' (https://github.com/rust-lang/miri.git) registered for path 'src/tools/miri'
2019-07-09T13:11:56.0605346Z Submodule 'src/tools/rls' (https://github.com/rust-lang-nursery/rls.git) registered for path 'src/tools/rls'
2019-07-09T13:11:56.0605346Z Submodule 'src/tools/rls' (https://github.com/rust-lang-nursery/rls.git) registered for path 'src/tools/rls'
2019-07-09T13:11:56.0610508Z Submodule 'src/rust-installer' (***-installer.git) registered for path 'src/tools/rust-installer'
2019-07-09T13:11:56.0737545Z Cloning into '/home/vsts/work/1/s/src/doc/edition-guide'...
2019-07-09T13:11:57.0799947Z + break
2019-07-09T13:11:57.0801230Z + mkdir src/doc/rust-by-example
2019-07-09T13:11:57.0922407Z + touch src/doc/rust-by-example/.git
---
2019-07-09T13:13:14.1642976Z Unpacking python3-setuptools (20.7.0-1) ...
2019-07-09T13:13:14.3972261Z Setting up python3-setuptools (20.7.0-1) ...
2019-07-09T13:13:16.2163568Z Attempting with retry: pip3 install awscli --upgrade --user
2019-07-09T13:13:17.7465309Z Collecting awscli
2019-07-09T13:13:18.8191835Z   Downloading https://files.pythonhosted.org/packages/7f/3f/9ecdb63eba199e04a5b0dcad0e409700269ef4860349604705c6fcffbd76/awscli-1.16.194-py2.py3-none-any.whl (1.7MB)
2019-07-09T13:13:19.5714912Z   Downloading https://files.pythonhosted.org/packages/e1/ae/baedc9cb175552e95f3395c43055a6a5e125ae4d48a1d7a924baca83e92e/rsa-3.4.2-py2.py3-none-any.whl (46kB)
2019-07-09T13:13:19.5839053Z Collecting colorama<=0.3.9,>=0.2.5 (from awscli)
2019-07-09T13:13:19.7185420Z   Downloading https://files.pythonhosted.org/packages/db/c8/7dcf9dbcb22429512708fe3a547f8b6101c0d02137acbd892505aee57adf/colorama-0.3.9-py2.py3-none-any.whl
2019-07-09T13:13:19.7245745Z Collecting docutils>=0.10 (from awscli)
2019-07-09T13:13:19.7245745Z Collecting docutils>=0.10 (from awscli)
2019-07-09T13:13:19.8438822Z   Downloading https://files.pythonhosted.org/packages/36/fa/08e9e6e0e3cbd1d362c3bbee8d01d0aedb2155c4ac112b19ef3cae8eed8d/docutils-0.14-py3-none-any.whl (543kB)
2019-07-09T13:13:19.9472995Z Collecting botocore==1.12.184 (from awscli)
2019-07-09T13:13:20.5146598Z   Downloading https://files.pythonhosted.org/packages/5a/7a/d40fabadbdc48fc4c15c087e69ab3f18e776b88662d5d118b03e09847673/botocore-1.12.184-py2.py3-none-any.whl (5.6MB)
2019-07-09T13:13:21.5368406Z   Downloading https://files.pythonhosted.org/packages/9f/2c/9417b5c774792634834e730932745bc09a7d36754ca00acf1ccd1ac2594d/PyYAML-5.1.tar.gz (274kB)
2019-07-09T13:13:22.1286158Z Collecting s3transfer<0.3.0,>=0.2.0 (from awscli)
2019-07-09T13:13:22.2556076Z   Downloading https://files.pythonhosted.org/packages/16/8a/1fc3dba0c4923c2a76e1ff0d52b305c44606da63f718d14d3231e21c51b0/s3transfer-0.2.1-py2.py3-none-any.whl (70kB)
2019-07-09T13:13:22.2728250Z Collecting pyasn1>=0.1.3 (from rsa<=3.5.0,>=3.1.2->awscli)
---
2019-07-09T13:13:23.1040589Z   Downloading https://files.pythonhosted.org/packages/73/fb/00a976f728d0d1fecfe898238ce23f502a721c0ac0ecfedb80e0d88c64e9/six-1.12.0-py2.py3-none-any.whl
2019-07-09T13:13:23.1080545Z Building wheels for collected packages: PyYAML
2019-07-09T13:13:23.1084878Z   Running setup.py bdist_wheel for PyYAML: started
2019-07-09T13:13:23.3446613Z   Running setup.py bdist_wheel for PyYAML: finished with status 'error'
2019-07-09T13:13:23.3448730Z   Complete output from command /usr/bin/python3 -u -c "import setuptools, tokenize;__file__='/tmp/pip-build-dt2a6rlw/PyYAML/setup.py';exec(compile(getattr(tokenize, 'open', open)(__file__).read().replace('\r\n', '\n'), __file__, 'exec'))" bdist_wheel -d /tmp/tmp1e8ha252pip-wheel- --python-tag cp35:
2019-07-09T13:13:23.3450981Z     warnings.warn(msg)
2019-07-09T13:13:23.3451780Z   usage: -c [global_opts] cmd1 [cmd1_opts] [cmd2 [cmd2_opts] ...]
2019-07-09T13:13:23.3452470Z      or: -c --help [cmd1 cmd2 ...]
2019-07-09T13:13:23.3453216Z      or: -c --help-commands
---
2019-07-09T13:13:32.8800102Z Attempting with retry: curl -y 30 -Y 10 --connect-timeout 30 -f -L -C - -o /tmp/rustci_docker_cache https://.s3.amazonaws.com/docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624
2019-07-09T13:13:32.8864897Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-09T13:13:32.8865737Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-09T13:13:32.8865904Z 
2019-07-09T13:13:32.8908471Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-09T13:13:33.9004946Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-09T13:13:33.9006839Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-09T13:13:33.9008044Z 
2019-07-09T13:13:33.9008044Z 
2019-07-09T13:13:33.9038624Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-09T13:13:35.9160321Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-09T13:13:35.9168328Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-09T13:13:35.9168742Z 
2019-07-09T13:13:35.9168742Z 
2019-07-09T13:13:35.9169782Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-09T13:13:38.9234021Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-09T13:13:38.9234847Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-09T13:13:38.9234895Z 
2019-07-09T13:13:38.9234895Z 
2019-07-09T13:13:38.9277441Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-09T13:13:42.9398440Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-07-09T13:13:42.9399190Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-07-09T13:13:42.9399577Z 
2019-07-09T13:13:42.9399577Z 
2019-07-09T13:13:42.9401274Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0curl: (6) Could not resolve host: .s3.amazonaws.com
2019-07-09T13:13:42.9407203Z The command has failed after 5 attempts.
2019-07-09T13:13:43.0002018Z open /tmp/rustci_docker_cache: no such file or directory
2019-07-09T13:13:43.0028447Z Attempting with retry: docker build --rm -t rust-ci -f /home/vsts/work/1/s/src/ci/docker/x86_64-gnu-llvm-6.0/Dockerfile /home/vsts/work/1/s/src/ci/docker
2019-07-09T13:13:43.1518014Z Sending build context to Docker daemon  521.7kB
2019-07-09T13:13:43.1518924Z 
2019-07-09T13:13:43.1708512Z Step 1/8 : FROM ubuntu:16.04
---
2019-07-09T13:14:00.4991789Z Reading package lists...
2019-07-09T13:14:01.5903147Z Reading package lists...
2019-07-09T13:14:01.7894868Z Building dependency tree...
2019-07-09T13:14:01.7895115Z Reading state information...
2019-07-09T13:14:01.9344552Z The following additional packages will be installed:
2019-07-09T13:14:01.9345460Z   binfmt-support binutils bzip2 cmake-data cpp cpp-5 dpkg-dev g++-5 gcc gcc-5
2019-07-09T13:14:01.9345828Z   git-man libarchive13 libasan2 libasn1-8-heimdal libatomic1
2019-07-09T13:14:01.9346111Z   libbabeltrace-ctf1 libbabeltrace1 libbsd-dev libbsd0 libbz2-1.0 libc-dev-bin
2019-07-09T13:14:01.9346713Z   libedit2 liberror-perl libexpat1 libffi-dev libffi6 libgcc-5-dev libgdbm3
2019-07-09T13:14:01.9346977Z   libglib2.0-0 libgmp10 libgnutls30 libgomp1 libgssapi-krb5-2
2019-07-09T13:14:01.9347236Z   libgssapi3-heimdal libhcrypto4-heimdal libheimbase1-heimdal
2019-07-09T13:14:01.9347584Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-09T13:14:01.9347584Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-09T13:14:01.9347863Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-09T13:14:01.9348136Z   libkrb5-3 libkrb5support0 libldap-2.4-2 libllvm6.0 liblsan0 liblzo2-2
2019-07-09T13:14:01.9348463Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-09T13:14:01.9348739Z   libperl5.22 libpipeline1 libpython-stdlib libpython2.7-minimal
2019-07-09T13:14:01.9349372Z   libpython2.7-stdlib libpython3.5 libpython3.5-minimal libpython3.5-stdlib
2019-07-09T13:14:01.9349761Z   libquadmath0 libroken18-heimdal librtmp1 libsasl2-2 libsasl2-modules-db
2019-07-09T13:14:01.9350040Z   libsqlite3-0 libssl1.0.0 libstdc++-5-dev libtasn1-6 libtinfo-dev libtsan0
2019-07-09T13:14:01.9350311Z   libubsan0 libwind0-heimdal libxml2 linux-libc-dev llvm-6.0 llvm-6.0-dev
2019-07-09T13:14:01.9350645Z   llvm-6.0-runtime mime-support openssl patch perl perl-modules-5.22 python
2019-07-09T13:14:01.9350897Z   python-minimal python2.7-minimal
2019-07-09T13:14:01.9350948Z Suggested packages:
2019-07-09T13:14:01.9351270Z   binutils-doc bzip2-doc codeblocks eclipse ninja-build cpp-doc gcc-5-locales
2019-07-09T13:14:01.9351547Z   debian-keyring g++-multilib g++-5-multilib gcc-5-doc libstdc++6-5-dbg
2019-07-09T13:14:01.9351814Z   gcc-multilib manpages-dev autoconf automake libtool flex bison gcc-doc
2019-07-09T13:14:01.9352423Z   libasan2-dbg liblsan0-dbg libtsan0-dbg libubsan0-dbg libcilkrts5-dbg
2019-07-09T13:14:01.9352686Z   libmpx0-dbg libquadmath0-dbg gdb-doc gettext-base git-daemon-run
2019-07-09T13:14:01.9352686Z   libmpx0-dbg libquadmath0-dbg gdb-doc gettext-base git-daemon-run
2019-07-09T13:14:01.9353582Z   | git-daemon-sysvinit git-doc git-el git-email git-gui gitk gitweb git-arch
2019-07-09T13:14:01.9353970Z   git-cvs git-mediawiki git-svn lrzip glibc-doc gnutls-bin krb5-doc krb5-user
2019-07-09T13:14:01.9354235Z   libstdc++-5-doc llvm-6.0-doc make-doc ed diffutils-doc perl-doc
2019-07-09T13:14:01.9354573Z   libterm-readline-gnu-perl | libterm-readline-perl-perl python-doc python-tk
2019-07-09T13:14:01.9354807Z   python2.7-doc
2019-07-09T13:14:01.9354856Z Recommended packages:
2019-07-09T13:14:01.9355172Z   build-essential fakeroot libalgorithm-merge-perl libc-dbg gdbserver less
2019-07-09T13:14:01.9355445Z   rsync ssh-client manpages manpages-dev libfile-fcntllock-perl
2019-07-09T13:14:01.9356585Z   libglib2.0-data shared-mime-info xdg-user-dirs krb5-locales libsasl2-modules
2019-07-09T13:14:01.9356892Z   libssl-doc xml-core netbase rename
2019-07-09T13:14:01.9356948Z The following NEW packages will be installed:
2019-07-09T13:14:01.9357226Z   binfmt-support binutils bzip2 ca-certificates cmake cmake-data cpp cpp-5
2019-07-09T13:14:01.9357504Z   curl dpkg-dev file g++ g++-5 gcc gcc-5 gdb git git-man libarchive13 libasan2
2019-07-09T13:14:01.9357838Z   libasn1-8-heimdal libatomic1 libbabeltrace-ctf1 libbabeltrace1 libbsd-dev
2019-07-09T13:14:01.9358235Z   libbsd0 libc-dev-bin libc6-dev libcc1-0 libcilkrts5 libcurl3 libcurl3-gnutls
2019-07-09T13:14:01.9358551Z   libdpkg-perl libedit-dev libedit2 liberror-perl libexpat1 libffi-dev libffi6
2019-07-09T13:14:01.9359101Z   libgcc-5-dev libgdbm3 libglib2.0-0 libgmp10 libgnutls30 libgomp1
2019-07-09T13:14:01.9359769Z   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu55 libidn11
2019-07-09T13:14:01.9360119Z   libisl15 libitm1 libjsoncpp1 libk5crypto3 libkeyutils1 libkrb5-26-heimdal
2019-07-09T13:14:01.9360394Z   libkrb5-3 libkrb5support0 libldap-2.4-2 libllvm6.0 liblsan0 liblzo2-2
2019-07-09T13:14:01.9360663Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-09T13:14:01.9360663Z   libmagic1 libmpc3 libmpdec2 libmpfr4 libmpx0 libnettle6 libp11-kit0
2019-07-09T13:14:01.9360987Z   libperl5.22 libpipeline1 libpython-stdlib libpython2.7-minimal
2019-07-09T13:14:01.9361263Z   libpython2.7-stdlib libpython3.5 libpython3.5-minimal libpython3.5-stdlib
2019-07-09T13:14:01.9361548Z   libquadmath0 libroken18-heimdal librtmp1 libsasl2-2 libsasl2-modules-db
2019-07-09T13:14:01.9361885Z   libsqlite3-0 libssl-dev libssl1.0.0 libstdc++-5-dev libtasn1-6 libtinfo-dev
2019-07-09T13:14:01.9362154Z   libtsan0 libubsan0 libwind0-heimdal libxml2 linux-libc-dev llvm-6.0
2019-07-09T13:14:01.9362429Z   llvm-6.0-dev llvm-6.0-runtime llvm-6.0-tools make mime-support openssl patch
2019-07-09T13:14:01.9362766Z   perl perl-modules-5.22 pkg-config python python-minimal python2.7
2019-07-09T13:14:01.9363015Z   python2.7-minimal sudo xz-utils zlib1g-dev
2019-07-09T13:14:01.9363067Z The following packages will be upgraded:
2019-07-09T13:14:02.2214548Z 1 upgraded, 115 newly installed, 0 to remove and 4 not upgraded.
2019-07-09T13:14:02.2214742Z Need to get 121 MB of archives.
2019-07-09T13:14:02.2215072Z After this operation, 592 MB of additional disk space will be used.
2019-07-09T13:14:02.2216162Z Get:1 http://archive.ubuntu.com/ubuntu xenial/main amd64 libgdbm3 amd64 1.8.3-13.1 [16.9 kB]
---
2019-07-09T13:14:03.4542783Z Get:61 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 sudo amd64 1.8.16-0ubuntu1.7 [390 kB]
2019-07-09T13:14:03.4624014Z Get:62 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 openssl amd64 1.0.2g-1ubuntu4.15 [492 kB]
2019-07-09T13:14:03.4722669Z Get:63 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 ca-certificates all 20170717~16.04.2 [167 kB]
2019-07-09T13:14:03.4751117Z Get:64 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 libcurl3-gnutls amd64 7.47.0-1ubuntu2.13 [184 kB]
2019-07-09T13:14:03.4787565Z Get:65 http://archive.ubuntu.com/ubuntu xenial/main amd64 libedit2 amd64 3.1-20150325-1ubuntu2 [76.5 kB]
2019-07-09T13:14:03.4808150Z Get:66 http://archive.ubuntu.com/ubuntu xenial/main amd64 libpipeline1 amd64 1.4.1-2 [24.6 kB]
2019-07-09T13:14:03.4808553Z Get:67 http://archive.ubuntu.com/ubuntu xenial/main amd64 binfmt-support amd64 2.1.6-1 [50.7 kB]
2019-07-09T13:14:03.5427043Z Get:69 http://archive.ubuntu.com/ubuntu xenial/main amd64 libisl15 amd64 0.16.1-1 [524 kB]
2019-07-09T13:14:03.5548373Z Get:70 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 cpp-5 amd64 5.4.0-6ubuntu1~16.04.11 [7660 kB]
2019-07-09T13:14:03.6900733Z Get:71 http://archive.ubuntu.com/ubuntu xenial/main amd64 cpp amd64 4:5.3.1-1ubuntu1 [27.7 kB]
2019-07-09T13:14:03.6901637Z Get:72 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 curl amd64 7.47.0-1ubuntu2.13 [139 kB]
---
2019-07-09T13:14:21.8996449Z Unpacking ca-certificates (20170717~16.04.2) ...
2019-07-09T13:14:22.0861450Z Selecting previously unselected package libcurl3-gnutls:amd64.
2019-07-09T13:14:22.0882897Z Preparing to unpack .../libcurl3-gnutls_7.47.0-1ubuntu2.13_amd64.deb ...
2019-07-09T13:14:22.1024394Z Unpacking libcurl3-gnutls:amd64 (7.47.0-1ubuntu2.13) ...
2019-07-09T13:14:22.2392334Z Selecting previously unselected package libedit2:amd64.
2019-07-09T13:14:22.2411781Z Preparing to unpack .../libedit2_3.1-20150325-1ubuntu2_amd64.deb ...
2019-07-09T13:14:22.2541059Z Unpacking libedit2:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-09T13:14:22.3722110Z Selecting previously unselected package libpipeline1:amd64.
2019-07-09T13:14:22.3741663Z Preparing to unpack .../libpipeline1_1.4.1-2_amd64.deb ...
2019-07-09T13:14:22.3874275Z Unpacking libpipeline1:amd64 (1.4.1-2) ...
2019-07-09T13:14:22.4989486Z Selecting previously unselected package binfmt-support.
2019-07-09T13:14:22.5006577Z Preparing to unpack .../binfmt-support_2.1.6-1_amd64.deb ...
2019-07-09T13:14:22.5137434Z Unpacking binfmt-support (2.1.6-1) ...
2019-07-09T13:14:22.6346409Z Preparing to unpack .../binutils_2.26.1-1ubuntu1~16.04.8_amd64.deb ...
2019-07-09T13:14:22.6479413Z Unpacking binutils (2.26.1-1ubuntu1~16.04.8) ...
2019-07-09T13:14:23.1737563Z Selecting previously unselected package libisl15:amd64.
2019-07-09T13:14:23.1755418Z Preparing to unpack .../libisl15_0.16.1-1_amd64.deb ...
---
2019-07-09T13:14:35.2028878Z Selecting previously unselected package libtinfo-dev:amd64.
2019-07-09T13:14:35.2052553Z Preparing to unpack .../libtinfo-dev_6.0+20160213-1ubuntu1_amd64.deb ...
2019-07-09T13:14:35.2180137Z Unpacking libtinfo-dev:amd64 (6.0+20160213-1ubuntu1) ...
2019-07-09T13:14:35.3313459Z Selecting previously unselected package libedit-dev:amd64.
2019-07-09T13:14:35.3335972Z Preparing to unpack .../libedit-dev_3.1-20150325-1ubuntu2_amd64.deb ...
2019-07-09T13:14:35.3500241Z Unpacking libedit-dev:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-09T13:14:35.4767618Z Selecting previously unselected package libllvm6.0:amd64.
2019-07-09T13:14:35.4790226Z Preparing to unpack .../libllvm6.0_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-09T13:14:35.4937155Z Unpacking libllvm6.0:amd64 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-09T13:14:38.5790411Z Selecting previously unselected package zlib1g-dev:amd64.
2019-07-09T13:14:38.5810048Z Preparing to unpack .../zlib1g-dev_1%3a1.2.8.dfsg-2ubuntu4.1_amd64.deb ...
2019-07-09T13:14:38.5944775Z Unpacking zlib1g-dev:amd64 (1:1.2.8.dfsg-2ubuntu4.1) ...
2019-07-09T13:14:38.7041175Z Preparing to unpack .../libssl-dev_1.0.2g-1ubuntu4.15_amd64.deb ...
2019-07-09T13:14:38.7169225Z Unpacking libssl-dev:amd64 (1.0.2g-1ubuntu4.15) ...
2019-07-09T13:14:39.0228932Z Selecting previously unselected package llvm-6.0-runtime.
2019-07-09T13:14:39.0228932Z Selecting previously unselected package llvm-6.0-runtime.
2019-07-09T13:14:39.0245185Z Preparing to unpack .../llvm-6.0-runtime_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-09T13:14:39.0383450Z Unpacking llvm-6.0-runtime (1:6.0-1ubuntu2~16.04.1) ...
2019-07-09T13:14:39.1682012Z Selecting previously unselected package llvm-6.0.
2019-07-09T13:14:39.1703937Z Preparing to unpack .../llvm-6.0_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-09T13:14:39.2039999Z Unpacking llvm-6.0 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-09T13:14:39.9095553Z Selecting previously unselected package libffi-dev:amd64.
2019-07-09T13:14:39.9116084Z Preparing to unpack .../libffi-dev_3.2.1-4_amd64.deb ...
2019-07-09T13:14:39.9268133Z Unpacking libffi-dev:amd64 (3.2.1-4) ...
2019-07-09T13:14:40.0578234Z Selecting previously unselected package llvm-6.0-dev.
2019-07-09T13:14:40.0596993Z Preparing to unpack .../llvm-6.0-dev_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-09T13:14:40.0755850Z Unpacking llvm-6.0-dev (1:6.0-1ubuntu2~16.04.1) ...
2019-07-09T13:14:44.8695095Z Selecting previously unselected package llvm-6.0-tools.
2019-07-09T13:14:44.8716219Z Preparing to unpack .../llvm-6.0-tools_1%3a6.0-1ubuntu2~16.04.1_amd64.deb ...
2019-07-09T13:14:44.8843978Z Unpacking llvm-6.0-tools (1:6.0-1ubuntu2~16.04.1) ...
2019-07-09T13:14:45.0491617Z Selecting previously unselected package pkg-config.
2019-07-09T13:14:45.0513025Z Preparing to unpack .../pkg-config_0.29.1-0ubuntu1_amd64.deb ...
2019-07-09T13:14:45.0647014Z Unpacking pkg-config (0.29.1-0ubuntu1) ...
2019-07-09T13:14:45.2231176Z Processing triggers for systemd (229-4ubuntu21.21) ...
2019-07-09T13:14:45.5987548Z Setting up libgdbm3:amd64 (1.8.3-13.1) ...
2019-07-09T13:14:45.6725902Z Setting up libffi6:amd64 (3.2.1-4) ...
2019-07-09T13:14:45.7156506Z Setting up libglib2.0-0:amd64 (2.48.2-0ubuntu4.3) ...
---
2019-07-09T13:14:49.8102070Z debconf: unable to initialize frontend: Dialog
2019-07-09T13:14:49.8102280Z debconf: (TERM is not set, so the dialog frontend is not usable.)
2019-07-09T13:14:49.8102341Z debconf: falling back to frontend: Readline
2019-07-09T13:14:50.3519977Z Setting up libcurl3-gnutls:amd64 (7.47.0-1ubuntu2.13) ...
2019-07-09T13:14:50.3933990Z Setting up libedit2:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-09T13:14:50.4322587Z Setting up libpipeline1:amd64 (1.4.1-2) ...
2019-07-09T13:14:50.4724619Z Setting up binfmt-support (2.1.6-1) ...
2019-07-09T13:14:50.5527388Z mount: permission denied
2019-07-09T13:14:50.5531539Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-09T13:14:50.5544167Z mount: permission denied
2019-07-09T13:14:50.5548316Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-09T13:14:50.7371056Z invoke-rc.d: could not determine current runlevel
2019-07-09T13:14:50.7407645Z invoke-rc.d: policy-rc.d denied execution of start.
2019-07-09T13:14:50.8011597Z Setting up libisl15:amd64 (0.16.1-1) ...
2019-07-09T13:14:50.8422355Z Setting up cpp-5 (5.4.0-6ubuntu1~16.04.11) ...
2019-07-09T13:14:50.8936091Z Setting up cpp (4:5.3.1-1ubuntu1) ...
2019-07-09T13:14:50.9447843Z Setting up curl (7.47.0-1ubuntu2.13) ...
---
2019-07-09T13:14:52.7540567Z Setting up libedit-dev:amd64 (3.1-20150325-1ubuntu2) ...
2019-07-09T13:14:52.7999237Z Setting up libllvm6.0:amd64 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-09T13:14:52.8506710Z Setting up zlib1g-dev:amd64 (1:1.2.8.dfsg-2ubuntu4.1) ...
2019-07-09T13:14:52.8951615Z Setting up libssl-dev:amd64 (1.0.2g-1ubuntu4.15) ...
2019-07-09T13:14:52.9347656Z Setting up llvm-6.0-runtime (1:6.0-1ubuntu2~16.04.1) ...
2019-07-09T13:14:52.9651731Z mount: permission denied
2019-07-09T13:14:52.9658040Z update-binfmts: warning: Couldn't mount the binfmt_misc filesystem on /proc/sys/fs/binfmt_misc.
2019-07-09T13:14:52.9804143Z Setting up llvm-6.0 (1:6.0-1ubuntu2~16.04.1) ...
2019-07-09T13:14:53.0226058Z Setting up libffi-dev:amd64 (3.2.1-4) ...
2019-07-09T13:14:53.0791054Z Setting up llvm-6.0-dev (1:6.0-1ubuntu2~16.04.1) ...
2019-07-09T13:14:53.1189508Z Setting up llvm-6.0-tools (1:6.0-1ubuntu2~16.04.1) ...
2019-07-09T13:14:53.1625758Z Setting up pkg-config (0.29.1-0ubuntu1) ...
2019-07-09T13:14:53.2977153Z Processing triggers for ca-certificates (20170717~16.04.2) ...
2019-07-09T13:14:53.3176676Z Updating certificates in /etc/ssl/certs...
2019-07-09T13:14:54.9954236Z 148 added, 0 removed; done.
2019-07-09T13:14:54.9955124Z Running hooks in /etc/ca-certificates/update.d...
---
2019-07-09T13:15:28.7770073Z  ---> a45f627caa5a
2019-07-09T13:15:28.7813031Z Successfully built a45f627caa5a
2019-07-09T13:15:28.9526157Z Successfully tagged rust-ci:latest
2019-07-09T13:15:29.0373647Z Built container sha256:a45f627caa5a60441ed28b141caf59b8309ffec66be17a954f12d0f45c15736e
2019-07-09T13:15:29.0391259Z Uploading finished image to https://.s3.amazonaws.com/docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624
2019-07-09T13:16:32.7264232Z upload failed: - to s3:///docker/c7688a42c3598c0b7dfe0f9f69838f24b25841ef6f7f87b4686f4da367d970f5a477b9c1277bdc58ebfc14a49c51c0e2ddb2b3366d867d7aae1de3d9233c8624 Parameter validation failed:
2019-07-09T13:16:32.7265409Z Invalid bucket name "": Bucket name must match the regex "^[a-zA-Z0-9.\-_]{1,255}$"
2019-07-09T13:16:33.7879678Z [CI_JOB_NAME=Job1]
2019-07-09T13:16:33.7940690Z Starting sccache server...
2019-07-09T13:16:33.8465788Z configure: processing command line
2019-07-09T13:16:33.8466223Z configure: 
---
2019-07-09T13:18:36.3235733Z   Downloaded strip-ansi-escapes v0.1.0
2019-07-09T13:18:36.3406178Z   Downloaded kernel32-sys v0.2.2
2019-07-09T13:18:36.3770335Z   Downloaded crossbeam-epoch v0.7.0
2019-07-09T13:18:36.3929864Z   Downloaded utf8-ranges v1.0.2
2019-07-09T13:18:36.4275603Z   Downloaded security-framework-sys v0.3.1
2019-07-09T13:18:36.4844059Z   Downloaded crc32fast v1.1.2
2019-07-09T13:18:36.4998227Z   Downloaded bitflags v0.9.1
2019-07-09T13:18:36.5319164Z   Downloaded open v1.2.1
2019-07-09T13:18:36.5518757Z   Downloaded mio-uds v0.6.7
---
2019-07-09T13:18:39.2892289Z   Downloaded chrono v0.4.6
2019-07-09T13:18:39.3165158Z   Downloaded strum_macros v0.11.0
2019-07-09T13:18:39.3398863Z   Downloaded http v0.1.16
2019-07-09T13:18:39.4143992Z   Downloaded globset v0.4.3
2019-07-09T13:18:39.4323469Z   Downloaded reqwest v0.9.11
2019-07-09T13:18:39.4866790Z   Downloaded sized-chunks v0.3.0
2019-07-09T13:18:39.5455922Z   Downloaded constant_time_eq v0.1.3
2019-07-09T13:18:39.5759443Z   Downloaded crypto-hash v0.3.1
2019-07-09T13:18:39.5848931Z   Downloaded fst v0.3.0
---
2019-07-09T13:18:40.6019428Z   Downloaded rayon-core v1.5.0
2019-07-09T13:18:40.6906925Z   Downloaded unicase v2.4.0
2019-07-09T13:18:40.7023538Z   Downloaded chalk-macros v0.1.0
2019-07-09T13:18:40.7037889Z   Downloaded html5ever v0.23.0
2019-07-09T13:18:40.7368679Z   Downloaded tokio-buf v0.1.1
2019-07-09T13:18:40.8207396Z   Downloaded glob v0.3.0
2019-07-09T13:18:40.8400592Z   Downloaded bufstream v0.1.4
2019-07-09T13:18:40.8884116Z   Downloaded directories v2.0.1
2019-07-09T13:18:40.9495916Z   Downloaded rustc-ap-graphviz v491.0.0
---
2019-07-09T13:18:41.5670760Z   Downloaded h2 v0.1.25
2019-07-09T13:18:41.5949654Z   Downloaded mdbook-linkcheck v0.3.0
2019-07-09T13:18:41.6108872Z   Downloaded crossbeam-utils v0.6.5
2019-07-09T13:18:41.6414217Z   Downloaded scoped-tls v1.0.0
2019-07-09T13:18:41.6583006Z   Downloaded futures-cpupool v0.1.8
2019-07-09T13:18:41.7090508Z   Downloaded either v1.5.0
2019-07-09T13:18:41.7395980Z   Downloaded darling_core v0.8.6
2019-07-09T13:18:41.7636752Z   Downloaded native-tls v0.2.3
2019-07-09T13:18:41.7887998Z   Downloaded utf-8 v0.7.2
2019-07-09T13:18:41.7887998Z   Downloaded utf-8 v0.7.2
2019-07-09T13:18:41.8153875Z   Downloaded tokio-timer v0.2.8
2019-07-09T13:18:41.8373736Z   Downloaded siphasher v0.2.2
2019-07-09T13:18:41.8646473Z   Downloaded try-lock v0.2.2
2019-07-09T13:18:41.9161376Z   Downloaded failure_derive v0.1.5
2019-07-09T13:18:41.9440649Z   Downloaded structopt-derive v0.2.16
2019-07-09T13:18:41.9671952Z   Downloaded rls-analysis v0.17.0
2019-07-09T13:18:41.9935722Z   Downloaded mac v0.1.1
---
2019-07-09T13:18:48.8062220Z   Downloaded iovec v0.1.2
2019-07-09T13:18:48.8429906Z   Downloaded semver-parser v0.7.0
2019-07-09T13:18:48.8593200Z   Downloaded bitflags v1.0.4
2019-07-09T13:18:48.8931312Z   Downloaded winapi-build v0.1.1
2019-07-09T13:18:48.9108411Z   Downloaded crossbeam-queue v0.1.2
2019-07-09T13:18:48.9768490Z   Downloaded quote v0.3.15
2019-07-09T13:18:49.0060250Z   Downloaded dirs v1.0.5
2019-07-09T13:18:49.0264706Z   Downloaded cloudabi v0.0.3
2019-07-09T13:18:49.8582880Z Build completed successfully in 0:02:15
---
2019-07-09T13:20:14.4605021Z    Compiling serde_json v1.0.33
2019-07-09T13:20:18.9230050Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-07-09T13:20:28.4166670Z     Finished release [optimized] target(s) in 1m 36s
2019-07-09T13:20:28.4234856Z tidy check
2019-07-09T13:20:29.3857496Z tidy error: /checkout/src/libstd/sys/vxworks/condvar.rs:32: trailing whitespace
2019-07-09T13:20:29.3857617Z tidy error: /checkout/src/libstd/sys/vxworks/mutex.rs:43: trailing whitespace
2019-07-09T13:20:29.3857702Z tidy error: /checkout/src/libstd/sys/vxworks/mutex.rs:96: trailing whitespace
2019-07-09T13:20:30.5201980Z some tidy checks failed
2019-07-09T13:20:30.5202698Z 
2019-07-09T13:20:30.5202698Z 
2019-07-09T13:20:30.5203798Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-07-09T13:20:30.5204895Z 
2019-07-09T13:20:30.5205123Z 
2019-07-09T13:20:30.5209415Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-07-09T13:20:30.5209788Z Build completed unsuccessfully in 0:01:40
2019-07-09T13:20:30.5209788Z Build completed unsuccessfully in 0:01:40
2019-07-09T13:20:31.7733054Z ##[error]Bash exited with code '1'.
2019-07-09T13:20:31.7778613Z ##[section]Starting: Checkout
2019-07-09T13:20:31.7780318Z ==============================================================================
2019-07-09T13:20:31.7780390Z Task         : Get sources
2019-07-09T13:20:31.7780437Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
