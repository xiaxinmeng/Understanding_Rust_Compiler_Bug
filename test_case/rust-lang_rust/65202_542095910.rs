plain
2019-10-15T08:07:22.9794048Z do so (now or later) by using -b with the checkout command again. Example:
2019-10-15T08:07:22.9794622Z 
2019-10-15T08:07:22.9795219Z   git checkout -b <new-branch-name>
2019-10-15T08:07:22.9796061Z 
2019-10-15T08:07:22.9796589Z HEAD is now at 72946a714 Auto merge of #65202 - pietroalbini:scriptify-ci-config, r=<try>
2019-10-15T08:07:23.0159885Z ##[section]Finishing: Checkout
2019-10-15T08:07:23.0311600Z ##[section]Starting: Copy python.exe to python3.exe
2019-10-15T08:07:23.0430741Z Task         : Bash
2019-10-15T08:07:23.0430840Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-10-15T08:07:23.0430920Z Version      : 3.151.3
2019-10-15T08:07:23.0431009Z Author       : Microsoft Corporation
2019-10-15T08:07:23.0431009Z Author       : Microsoft Corporation
2019-10-15T08:07:23.0431113Z Help         : https://docs.microsoft.com/azure/devops/pipelines/tasks/utility/bash
2019-10-15T08:07:23.0431210Z ==============================================================================
2019-10-15T08:07:23.3603428Z Generating script.
2019-10-15T08:07:23.3827287Z Script contents:
2019-10-15T08:07:23.3827612Z cp $(which python) $(dirname $(which python))/python3.exe
2019-10-15T08:07:23.4526667Z /d/a/_temp
2019-10-15T08:07:23.4609045Z ========================== Starting Command Output ===========================
2019-10-15T08:07:23.4628087Z [command]"C:\Program Files\Git\bin\bash.exe" --noprofile --norc /d/a/_temp/42c87780-e3dc-45e0-99c2-9ea6af294fde.sh
2019-10-15T08:07:23.4628087Z [command]"C:\Program Files\Git\bin\bash.exe" --noprofile --norc /d/a/_temp/42c87780-e3dc-45e0-99c2-9ea6af294fde.sh
2019-10-15T08:07:24.0129694Z ##[section]Finishing: Copy python.exe to python3.exe
2019-10-15T08:07:24.0360907Z ==============================================================================
2019-10-15T08:07:24.0361005Z Task         : Bash
2019-10-15T08:07:24.0361091Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-10-15T08:07:24.0361176Z Version      : 3.151.3
---
2019-10-15T08:07:26.9331775Z 
2019-10-15T08:07:26.9331812Z 
2019-10-15T08:07:26.9331852Z 
2019-10-15T08:07:26.9331912Z 
2019-10-15T08:07:26.9335592Z * Being able to prepare the build environment locally by just running `src/ci/prepare.py` simplifies a lot setting up a local VM similar to CI (software pre-installed in the CI images won't be prepared, but it's a start anyway).
2019-10-15T08:07:26.9370658Z ##[warning]'* Extracted CI-specific behavior (like issuing `##vso[` commands and detecting the host platform) into `shared.sh` and included it in most of the scripts. This way a switch to another CI provider will be less painful.' contains logging command keyword '##vso', but it's not a legal command. Please see the list of accepted commands: https://go.microsoft.com/fwlink/?LinkId=817296
2019-10-15T08:07:26.9384139Z * Extracted CI-specific behavior (like issuing `##vso[` commands and detecting the host platform) into `shared.sh` and included it in most of the scripts. This way a switch to another CI provider will be less painful.
2019-10-15T08:07:26.9384474Z * Mirrored all the remaining external URLs we download (except chocolatey) to the `rust-lang-ci-mirrors` bucket, to increase reliability and reduce the chance of supply chain attacks. I didn't audit and mirror the CI scripts outside this PR though.
2019-10-15T08:07:26.9384706Z * When we'll switch to GitHub Actions we'll need to either duplicate code in multiple workflows or write a preprocessor. Having all the prepare steps in a single one is going to simplify the implementation of both options.
2019-10-15T08:07:26.9384966Z AGENT_BUILDDIRECTORY=D:\a\1
2019-10-15T08:07:26.9385431Z AGENT_DISABLELOGPLUGIN_TESTFILEPUBLISHERPLUGIN=true
2019-10-15T08:07:26.9385526Z AGENT_DISABLELOGPLUGIN_TESTRESULTLOGPLUGIN=true
2019-10-15T08:07:26.9386158Z AGENT_HOMEDIRECTORY=C:\agents\2.158.0
---
2019-10-15T08:07:26.9392110Z BUILD_SOURCEBRANCHNAME=try
2019-10-15T08:07:26.9392186Z BUILD_SOURCESDIRECTORY=D:\a\1\s
2019-10-15T08:07:26.9392296Z BUILD_SOURCEVERSION=72946a714eef362e01a744557b2804703e2a4134
2019-10-15T08:07:26.9392379Z BUILD_SOURCEVERSIONAUTHOR=bors
2019-10-15T08:07:26.9392497Z BUILD_SOURCEVERSIONMESSAGE=Auto merge of #65202 - pietroalbini:scriptify-ci-config, r=<try>
2019-10-15T08:07:26.9392690Z CHOCOLATEYINSTALL=C:\ProgramData\chocolatey
2019-10-15T08:07:26.9392803Z CHROMEWEBDRIVER=C:\SeleniumWebDrivers\ChromeDriver
2019-10-15T08:07:26.9392885Z COBERTURA_HOME=C:\cobertura-2.1.1
2019-10-15T08:07:26.9393120Z COMMONPROGRAMFILES(X86)=C:\Program Files (x86)\Common Files
---
2019-10-15T08:07:47.8667996Z SYSTEM_STAGEDISPLAYNAME=__default
2019-10-15T08:07:47.8668119Z SYSTEM_STAGEID=96ac2280-8cb4-5df5-99de-dd2da759617d
2019-10-15T08:07:47.8668201Z SYSTEM_STAGENAME=__default
2019-10-15T08:07:47.8668322Z SYSTEM_TASKDEFINITIONSURI=https://dev.azure.com/rust-lang/
2019-10-15T08:07:47.8668415Z SYSTEM_TASKDISPLAYNAME=Prepare the CI environment
2019-10-15T08:07:47.8668913Z SYSTEM_TASKINSTANCENAME=Bash4
2019-10-15T08:07:47.8669022Z SYSTEM_TEAMFOUNDATIONCOLLECTIONURI=https://dev.azure.com/rust-lang/
2019-10-15T08:07:47.8669162Z SYSTEM_TEAMFOUNDATIONSERVERURI=https://dev.azure.com/rust-lang/
2019-10-15T08:07:47.8669314Z SYSTEM_TEAMPROJECT=rust
2019-10-15T08:07:47.8669314Z SYSTEM_TEAMPROJECT=rust
2019-10-15T08:07:47.8669403Z SYSTEM_TEAMPROJECTID=e71b0ddf-dd27-435a-873c-e30f86eea377
2019-10-15T08:07:47.8669525Z SYSTEM_TIMELINEID=b5f1e1f4-283a-46e1-83d2-1a1ebf4fdb2e
2019-10-15T08:07:47.8669867Z SYSTEM_TOTALJOBSINPHASE=1
2019-10-15T08:07:47.8669981Z SYSTEM_WORKFOLDER=D:\a
2019-10-15T08:07:47.8670061Z TASK_DISPLAYNAME=Prepare the CI environment
2019-10-15T08:07:47.8670260Z TERM=cygwin
2019-10-15T08:07:47.8670329Z TF_BUILD=True
2019-10-15T08:07:47.8670573Z TMP=/tmp
2019-10-15T08:07:47.8670638Z TOOLSTATE_ISSUES_API_URL=https://api.github.com/repos/rust-lang/rust/issues
2019-10-15T08:07:47.8670638Z TOOLSTATE_ISSUES_API_URL=https://api.github.com/repos/rust-lang/rust/issues
2019-10-15T08:07:47.8670733Z TOOLSTATE_PUBLISH=1
2019-10-15T08:07:47.8670813Z TOOLSTATE_REPO=https://github.com/rust-lang-nursery/rust-toolstate
2019-10-15T08:07:47.8670914Z There are two reasons why we'd want to do this:
2019-10-15T08:07:47.8671512Z ##[warning]'This PR moves most of the configuration from the CI yamls into bash scripts, driven by a small Python script (which understands and emulates the two `##vso[` commands we use).' contains logging command keyword '##vso', but it's not a legal command. Please see the list of accepted commands: https://go.microsoft.com/fwlink/?LinkId=817296
2019-10-15T08:07:47.8672024Z This PR moves most of the configuration from the CI yamls into bash scripts, driven by a small Python script (which understands and emulates the two `##vso[` commands we use).
2019-10-15T08:07:47.8672215Z USERDOMAIN=fv-az433
2019-10-15T08:07:47.8672271Z USERDOMAIN_ROAMINGPROFILE=fv-az433
2019-10-15T08:07:47.8672355Z USERNAME=VssAdministrator
2019-10-15T08:07:47.8672438Z USERPROFILE=C:\Users\VssAdministrator
---
2019-10-15T08:07:47.8707781Z r? @alexcrichton
2019-10-15T08:07:47.8707843Z 
2019-10-15T08:07:47.8707908Z disk usage:
2019-10-15T08:07:47.8708071Z Filesystem            Size  Used Avail Use% Mounted on
2019-10-15T08:07:47.8708172Z C:/Program Files/Git  256G  139G  117G  55% /
2019-10-15T08:07:47.8708268Z D:                     14G  2.0G   13G  14% /d
2019-10-15T08:07:47.8708436Z biggest files in the working dir:
2019-10-15T08:07:47.8708543Z 154398 .
2019-10-15T08:07:47.8708885Z 77799 ./src
2019-10-15T08:07:47.8708952Z 75949 ./.git
---
2019-10-15T08:07:47.8716442Z 257 ./src/libstd/io
2019-10-15T08:07:47.8716506Z 256 ./src/libstd/net
2019-10-15T08:07:47.8716564Z 256 ./src/librustc_mir/hair
2019-10-15T08:07:47.8716617Z 
2019-10-15T08:07:47.8716675Z ==== success: Show the current environment ====
2019-10-15T08:07:47.8716788Z ==== Install sccache ====
2019-10-15T08:07:47.8716873Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-10-15T08:07:47.8717313Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-10-15T08:07:47.8717401Z 
2019-10-15T08:07:47.8717401Z 
2019-10-15T08:07:47.8717497Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
2019-10-15T08:07:47.8717616Z   0 5495k    0 33346    0     0  36563      0  0:02:33 --:--:--  0:02:33 36523
2019-10-15T08:07:47.8717734Z 100 5495k  100 5495k    0     0  4286k      0  0:00:01  0:00:01 --:--:-- 4286k
2019-10-15T08:07:47.8720590Z ==== success: Install sccache ====
2019-10-15T08:07:47.8720876Z ==== Install clang ====
2019-10-15T08:07:47.8720960Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-10-15T08:07:47.8721046Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-10-15T08:07:47.8721100Z 
---
2019-10-15T08:07:47.8723308Z  91  331M   91  301M    0     0  22.4M      0  0:00:14  0:00:13  0:00:01 25.4M
2019-10-15T08:07:47.8723605Z  94  331M   94  311M    0     0  21.6M      0  0:00:15  0:00:14  0:00:01 22.5M
2019-10-15T08:07:47.8723794Z 100  331M  100  331M    0     0  21.8M      0  0:00:15  0:00:15 --:--:-- 22.6M
2019-10-15T08:07:47.8743810Z  --set llvm.clang-cl=/d/a/1/s/citools/clang-rust/bin/clang-cl.exe
2019-10-15T08:07:47.8743924Z ==== success: Install clang ====
2019-10-15T08:07:47.8743991Z 
2019-10-15T08:07:47.8744054Z ==== Switch to Xcode 9.3 ====
2019-10-15T08:07:47.8744148Z ==== success: Switch to Xcode 9.3 ====
2019-10-15T08:07:47.8744197Z 
2019-10-15T08:07:47.8744270Z ==== Install WIX ====
2019-10-15T08:07:47.8744663Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-10-15T08:07:52.6354949Z 
2019-10-15T08:07:52.6355901Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
2019-10-15T08:07:52.6356300Z  17 32.7M   17 5732k    0     0  6791k      0  0:00:04 --:--:--  0:00:04 6783k
---
2019-10-15T08:07:52.6359535Z 
2019-10-15T08:07:52.6359696Z Files: 220
2019-10-15T08:07:52.6359861Z Size:       103746900
2019-10-15T08:07:52.6360016Z Compressed: 34358269
2019-10-15T08:07:52.6361247Z ==== success: Install WIX ====
2019-10-15T08:07:52.6361370Z 
2019-10-15T08:07:52.6361521Z ==== Install InnoSetup ====
2019-10-15T08:07:52.6362140Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-10-15T08:07:52.6362271Z 
2019-10-15T08:07:52.6362448Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
2019-10-15T08:07:52.6362626Z 100 1935k  100 1935k    0     0  2833k      0 --:--:-- --:--:-- --:--:-- 2833k
2019-10-15T08:07:52.6362626Z 100 1935k  100 1935k    0     0  2833k      0 --:--:-- --:--:-- --:--:-- 2833k
2019-10-15T08:07:52.6362827Z src/ci/scripts/install-innosetup.sh: line 15: is-install.exe: command not found
2019-10-15T08:07:52.6363012Z ==== failure: Install InnoSetup ====
2019-10-15T08:07:52.6363168Z exit code: 127
2019-10-15T08:07:52.6538065Z ##[error]Bash exited with code '127'.
2019-10-15T08:07:52.6571289Z ##[section]Finishing: Prepare the CI environment
2019-10-15T08:07:52.6748210Z ==============================================================================
2019-10-15T08:07:52.6748316Z Task         : Bash
2019-10-15T08:07:52.6748425Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-10-15T08:07:52.6748505Z Version      : 3.151.3
---
2019-10-15T08:07:52.9558465Z [command]"C:\Program Files\Git\bin\bash.exe" --noprofile --norc -c pwd
2019-10-15T08:07:52.9944475Z /d/a/_temp
2019-10-15T08:07:53.0013316Z ========================== Starting Command Output ===========================
2019-10-15T08:07:53.0031925Z [command]"C:\Program Files\Git\bin\bash.exe" --noprofile --norc /d/a/_temp/2b8c79f6-ab8c-4f0e-b67b-04e1f3adb015.sh
2019-10-15T08:07:53.0516577Z /d/a/_temp/2b8c79f6-ab8c-4f0e-b67b-04e1f3adb015.sh: line 1: aws: command not found
2019-10-15T08:07:53.0581780Z ##[error]Bash exited with code '127'.
2019-10-15T08:07:53.0661495Z ##[section]Starting: Checkout
2019-10-15T08:07:53.0772986Z ==============================================================================
2019-10-15T08:07:53.0773099Z Task         : Get sources
2019-10-15T08:07:53.0773186Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
