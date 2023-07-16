plain
2019-07-10T01:15:02.5439359Z git config --global core.autocrlf false
2019-07-10T01:15:02.5439644Z ========================== Starting Command Output ===========================
2019-07-10T01:15:02.5441789Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/4700304f-b589-4239-80bf-48101194bb23.sh
2019-07-10T01:15:02.6327804Z ##[section]Finishing: Disable git automatic line ending conversion
2019-07-10T01:15:02.6350826Z ##[section]Starting: git config pre checkout
2019-07-10T01:15:02.6354776Z Task         : Bash
2019-07-10T01:15:02.6354881Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-10T01:15:02.6354977Z Version      : 3.151.2
2019-07-10T01:15:02.6355073Z Author       : Microsoft Corporation
2019-07-10T01:15:02.6355073Z Author       : Microsoft Corporation
2019-07-10T01:15:02.6355146Z Help         : https://docs.microsoft.com/azure/devops/pipelines/tasks/utility/bash
2019-07-10T01:15:02.6355266Z ==============================================================================
2019-07-10T01:15:02.7714320Z Generating script.
2019-07-10T01:15:02.7747768Z ========================== Starting Command Output ===========================
2019-07-10T01:15:02.7769622Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/dc973b22-45ba-4bcf-928f-aebd88a3772d.sh
2019-07-10T01:15:02.7855260Z + git config --list --show-origin
2019-07-10T01:15:02.7882083Z file:/etc/gitconfig filter.lfs.clean=git-lfs clean -- %f
2019-07-10T01:15:02.7882554Z file:/etc/gitconfig filter.lfs.smudge=git-lfs smudge -- %f
2019-07-10T01:15:02.7882834Z file:/etc/gitconfig filter.lfs.process=git-lfs filter-process
2019-07-10T01:15:02.7882957Z file:/etc/gitconfig filter.lfs.required=true
2019-07-10T01:15:02.7883061Z file:/home/vsts/.gitconfig core.autocrlf=false
2019-07-10T01:15:02.7955403Z ##[section]Finishing: git config pre checkout
2019-07-10T01:15:02.7966756Z ==============================================================================
2019-07-10T01:15:02.7966874Z Task         : Get sources
2019-07-10T01:15:02.7966948Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2019-07-10T01:15:02.7967061Z Version      : 1.0.0
---
2019-07-10T01:15:40.3739161Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-10T01:15:40.3740541Z 
2019-07-10T01:15:40.3742421Z   git checkout -b <new-branch-name>
2019-07-10T01:15:40.3744229Z 
2019-07-10T01:15:40.3746116Z HEAD is now at 2c05e837f Auto merge of #62545 - Mark-Simulacrum:azure-line-endings, r=<try>
2019-07-10T01:15:40.3938034Z ##[section]Finishing: Checkout
2019-07-10T01:15:40.3946757Z ##[section]Starting: git config post checkout
2019-07-10T01:15:40.3950011Z Task         : Bash
2019-07-10T01:15:40.3950080Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-10T01:15:40.3950168Z Version      : 3.151.2
2019-07-10T01:15:40.3950231Z Author       : Microsoft Corporation
2019-07-10T01:15:40.3950231Z Author       : Microsoft Corporation
2019-07-10T01:15:40.3950324Z Help         : https://docs.microsoft.com/azure/devops/pipelines/tasks/utility/bash
2019-07-10T01:15:40.3950413Z ==============================================================================
2019-07-10T01:15:40.5326479Z Generating script.
2019-07-10T01:15:40.5340345Z Script contents:
2019-07-10T01:15:40.5341662Z git config --list --show-origin
2019-07-10T01:15:40.5385880Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/efa22ef2-123f-4e7f-9696-f9271b6858b1.sh
2019-07-10T01:15:40.5385880Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/efa22ef2-123f-4e7f-9696-f9271b6858b1.sh
2019-07-10T01:15:40.5456882Z file:/etc/gitconfig filter.lfs.clean=git-lfs clean -- %f
2019-07-10T01:15:40.5457265Z file:/etc/gitconfig filter.lfs.smudge=git-lfs smudge -- %f
2019-07-10T01:15:40.5457550Z file:/etc/gitconfig filter.lfs.process=git-lfs filter-process
2019-07-10T01:15:40.5457703Z file:/etc/gitconfig filter.lfs.required=true
2019-07-10T01:15:40.5457823Z file:/home/vsts/.gitconfig core.autocrlf=false
2019-07-10T01:15:40.5457900Z file:.git/config core.repositoryformatversion=0
2019-07-10T01:15:40.5458010Z file:.git/config core.filemode=true
2019-07-10T01:15:40.5458116Z file:.git/config core.bare=false
2019-07-10T01:15:40.5458225Z file:.git/config core.logallrefupdates=true
2019-07-10T01:15:40.5458522Z file:.git/config remote.origin.url=https://github.com/rust-lang/rust
2019-07-10T01:15:40.5458657Z file:.git/config remote.origin.fetch=+refs/heads/*:refs/remotes/origin/*
2019-07-10T01:15:40.5458736Z file:.git/config gc.auto=0
2019-07-10T01:15:40.5533272Z ##[section]Finishing: git config post checkout
2019-07-10T01:15:40.5547360Z ==============================================================================
2019-07-10T01:15:40.5547455Z Task         : Bash
2019-07-10T01:15:40.5547572Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-10T01:15:40.5547682Z Version      : 3.151.2
---
2019-07-10T01:16:16.8261915Z + return 0
2019-07-10T01:16:16.8262064Z + travis_time_finish
2019-07-10T01:16:16.8262249Z + return 0
2019-07-10T01:16:16.8349120Z ##[section]Finishing: Check out submodules (Unix)
2019-07-10T01:16:16.8366270Z ##[section]Starting: Verify line endings are LF
2019-07-10T01:16:16.8370275Z Task         : Bash
2019-07-10T01:16:16.8370370Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-10T01:16:16.8370435Z Version      : 3.151.2
2019-07-10T01:16:16.8370523Z Author       : Microsoft Corporation
2019-07-10T01:16:16.8370523Z Author       : Microsoft Corporation
2019-07-10T01:16:16.8370595Z Help         : https://docs.microsoft.com/azure/devops/pipelines/tasks/utility/bash
2019-07-10T01:16:16.8370716Z ==============================================================================
2019-07-10T01:16:17.0856784Z Generating script.
2019-07-10T01:16:17.0891111Z ========================== Starting Command Output ===========================
2019-07-10T01:16:17.0912379Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/92b4de00-0425-476c-b573-84cb889b79fd.sh
2019-07-10T01:16:17.0985801Z filter.lfs.clean=git-lfs clean -- %f
2019-07-10T01:16:17.0986790Z filter.lfs.smudge=git-lfs smudge -- %f
2019-07-10T01:16:17.0987275Z filter.lfs.process=git-lfs filter-process
2019-07-10T01:16:17.0987539Z filter.lfs.required=true
2019-07-10T01:16:17.0987707Z core.autocrlf=false
2019-07-10T01:16:17.0987912Z core.repositoryformatversion=0
2019-07-10T01:16:17.0988074Z core.filemode=true
2019-07-10T01:16:17.0988279Z core.bare=false
2019-07-10T01:16:17.0988474Z core.logallrefupdates=true
2019-07-10T01:16:17.0988871Z remote.origin.url=https://github.com/rust-lang/rust
2019-07-10T01:16:17.0989118Z remote.origin.fetch=+refs/heads/*:refs/remotes/origin/*
2019-07-10T01:16:17.0989309Z gc.auto=0
2019-07-10T01:16:17.0989912Z submodule.src/doc/edition-guide.active=true
2019-07-10T01:16:17.0990460Z submodule.src/doc/edition-guide.url=https://github.com/rust-lang-nursery/edition-guide.git
2019-07-10T01:16:17.0990876Z submodule.src/doc/embedded-book.active=true
2019-07-10T01:16:17.0991561Z submodule.src/doc/embedded-book.url=https://github.com/rust-embedded/book.git
2019-07-10T01:16:17.0991766Z submodule.src/doc/nomicon.active=true
2019-07-10T01:16:17.0992173Z submodule.src/doc/nomicon.url=https://github.com/rust-lang-nursery/nomicon.git
2019-07-10T01:16:17.0992361Z submodule.src/doc/reference.active=true
2019-07-10T01:16:17.0992904Z submodule.src/doc/reference.url=https://github.com/rust-lang-nursery/reference.git
2019-07-10T01:16:17.1004197Z submodule.src/doc/rustc-guide.active=true
2019-07-10T01:16:17.1004690Z submodule.src/doc/rustc-guide.url=https://github.com/rust-lang/rustc-guide.git
2019-07-10T01:16:17.1004916Z submodule.src/stdsimd.active=true
2019-07-10T01:16:17.1005660Z submodule.src/stdsimd.url=https://github.com/rust-lang-nursery/stdsimd.git
2019-07-10T01:16:17.1005913Z submodule.src/tools/cargo.active=true
2019-07-10T01:16:17.1006565Z submodule.src/tools/cargo.url=https://github.com/rust-lang/cargo.git
2019-07-10T01:16:17.1006865Z submodule.src/tools/clippy.active=true
2019-07-10T01:16:17.1007350Z submodule.src/tools/clippy.url=https://github.com/rust-lang-nursery/rust-clippy.git
2019-07-10T01:16:17.1007618Z submodule.src/tools/miri.active=true
2019-07-10T01:16:17.1008064Z submodule.src/tools/miri.url=https://github.com/rust-lang/miri.git
2019-07-10T01:16:17.1008270Z submodule.src/tools/rls.active=true
2019-07-10T01:16:17.1008711Z submodule.src/tools/rls.url=https://github.com/rust-lang-nursery/rls.git
2019-07-10T01:16:17.1009115Z submodule.src/rust-installer.active=true
2019-07-10T01:16:17.1009648Z submodule.src/rust-installer.url=https://github.com/rust-lang/rust-installer.git
2019-07-10T01:16:17.1009853Z submodule.src/tools/rustfmt.active=true
2019-07-10T01:16:17.1010311Z submodule.src/tools/rustfmt.url=https://github.com/rust-lang-nursery/rustfmt.git
2019-07-10T01:16:17.1010815Z + git config --list
2019-07-10T01:16:17.1011248Z + bash -c 'egrep -q $"\r"\$ src/tools/rust-installer/install-template.sh'
2019-07-10T01:16:17.1011665Z + '[' 0 = 0 ']'
2019-07-10T01:16:17.1011858Z + exit 1
2019-07-10T01:16:17.1167566Z ##[error]Bash exited with code '1'.
2019-07-10T01:16:17.1188326Z ##[section]Finishing: Verify line endings are LF
2019-07-10T01:16:17.1245910Z ==============================================================================
2019-07-10T01:16:17.1246025Z Task         : Bash
2019-07-10T01:16:17.1246086Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-10T01:16:17.1246185Z Version      : 3.151.2
2019-07-10T01:16:17.1246185Z Version      : 3.151.2
2019-07-10T01:16:17.1246279Z Author       : Microsoft Corporation
2019-07-10T01:16:17.1246347Z Help         : https://docs.microsoft.com/azure/devops/pipelines/tasks/utility/bash
2019-07-10T01:16:17.1246832Z ==============================================================================
2019-07-10T01:16:17.2858002Z Generating script.
2019-07-10T01:16:17.2870941Z Script contents:
2019-07-10T01:16:17.2871631Z aws s3 cp --acl public-read cpu-usage.csv s3://$DEPLOY_BUCKET/rustc-builds/$BUILD_SOURCEVERSION/cpu-$SYSTEM_JOBNAME.csv
2019-07-10T01:16:17.2893131Z ========================== Starting Command Output ===========================
2019-07-10T01:16:17.2915408Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/436b3814-c53f-4250-8990-12af43b9ab4a.sh
2019-07-10T01:16:17.3017220Z /home/vsts/work/_temp/436b3814-c53f-4250-8990-12af43b9ab4a.sh: line 1: aws: command not found
2019-07-10T01:16:17.3055101Z ##[error]Bash exited with code '127'.
2019-07-10T01:16:17.3097848Z ##[section]Starting: Checkout
2019-07-10T01:16:17.3099848Z ==============================================================================
2019-07-10T01:16:17.3099938Z Task         : Get sources
2019-07-10T01:16:17.3100044Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
