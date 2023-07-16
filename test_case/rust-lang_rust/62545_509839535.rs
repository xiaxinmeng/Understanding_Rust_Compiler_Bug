plain
2019-07-09T22:49:33.7386512Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-09T22:49:33.7386571Z 
2019-07-09T22:49:33.7386741Z   git checkout -b <new-branch-name>
2019-07-09T22:49:33.7386775Z 
2019-07-09T22:49:33.7387010Z HEAD is now at 1a2877061 Auto merge of #62545 - Mark-Simulacrum:azure-line-endings, r=<try>
2019-07-09T22:49:33.7528463Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-09T22:49:33.7533721Z ==============================================================================
2019-07-09T22:49:33.7533831Z Task         : Bash
2019-07-09T22:49:33.7533902Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-09T22:50:14.9702886Z + return 0
2019-07-09T22:50:14.9703034Z + travis_time_finish
2019-07-09T22:50:14.9703099Z + return 0
2019-07-09T22:50:14.9797677Z ##[section]Finishing: Check out submodules (Unix)
2019-07-09T22:50:14.9815856Z ##[section]Starting: Verify line endings are LF
2019-07-09T22:50:14.9818670Z Task         : Bash
2019-07-09T22:50:14.9818729Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-09T22:50:14.9818822Z Version      : 3.151.2
2019-07-09T22:50:14.9818874Z Author       : Microsoft Corporation
2019-07-09T22:50:14.9818874Z Author       : Microsoft Corporation
2019-07-09T22:50:14.9819100Z Help         : https://docs.microsoft.com/azure/devops/pipelines/tasks/utility/bash
2019-07-09T22:50:14.9819186Z ==============================================================================
2019-07-09T22:50:15.8369777Z Generating script.
2019-07-09T22:50:15.8407709Z ========================== Starting Command Output ===========================
2019-07-09T22:50:15.8438952Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/405f8163-f8cc-473d-85a0-cb9bb732cd07.sh
2019-07-09T22:50:15.8523723Z filter.lfs.clean=git-lfs clean -- %f
2019-07-09T22:50:15.8524529Z filter.lfs.smudge=git-lfs smudge -- %f
2019-07-09T22:50:15.8525275Z filter.lfs.process=git-lfs filter-process
2019-07-09T22:50:15.8525923Z filter.lfs.required=true
2019-07-09T22:50:15.8526011Z core.autocrlf=false
2019-07-09T22:50:15.8526107Z core.repositoryformatversion=0
2019-07-09T22:50:15.8526163Z core.filemode=true
2019-07-09T22:50:15.8526302Z core.bare=false
2019-07-09T22:50:15.8526356Z core.logallrefupdates=true
2019-07-09T22:50:15.8526979Z remote.origin.url=https://github.com/rust-lang/rust
2019-07-09T22:50:15.8527081Z remote.origin.fetch=+refs/heads/*:refs/remotes/origin/*
2019-07-09T22:50:15.8527177Z gc.auto=0
2019-07-09T22:50:15.8527434Z submodule.src/doc/edition-guide.active=true
2019-07-09T22:50:15.8527777Z submodule.src/doc/edition-guide.url=https://github.com/rust-lang-nursery/edition-guide.git
2019-07-09T22:50:15.8528009Z submodule.src/doc/embedded-book.active=true
2019-07-09T22:50:15.8528295Z submodule.src/doc/embedded-book.url=https://github.com/rust-embedded/book.git
2019-07-09T22:50:15.8528370Z submodule.src/doc/nomicon.active=true
2019-07-09T22:50:15.8528657Z submodule.src/doc/nomicon.url=https://github.com/rust-lang-nursery/nomicon.git
2019-07-09T22:50:15.8559951Z submodule.src/doc/reference.active=true
2019-07-09T22:50:15.8560446Z submodule.src/doc/reference.url=https://github.com/rust-lang-nursery/reference.git
2019-07-09T22:50:15.8560715Z submodule.src/doc/rustc-guide.active=true
2019-07-09T22:50:15.8560978Z submodule.src/doc/rustc-guide.url=https://github.com/rust-lang/rustc-guide.git
2019-07-09T22:50:15.8561259Z submodule.src/stdsimd.active=true
2019-07-09T22:50:15.8561717Z submodule.src/stdsimd.url=https://github.com/rust-lang-nursery/stdsimd.git
2019-07-09T22:50:15.8562364Z submodule.src/tools/cargo.active=true
2019-07-09T22:50:15.8562721Z submodule.src/tools/cargo.url=https://github.com/rust-lang/cargo.git
2019-07-09T22:50:15.8562835Z submodule.src/tools/clippy.active=true
2019-07-09T22:50:15.8563147Z submodule.src/tools/clippy.url=https://github.com/rust-lang-nursery/rust-clippy.git
2019-07-09T22:50:15.8563264Z submodule.src/tools/miri.active=true
2019-07-09T22:50:15.8563541Z submodule.src/tools/miri.url=https://github.com/rust-lang/miri.git
2019-07-09T22:50:15.8563642Z submodule.src/tools/rls.active=true
2019-07-09T22:50:15.8563914Z submodule.src/tools/rls.url=https://github.com/rust-lang-nursery/rls.git
2019-07-09T22:50:15.8564203Z submodule.src/rust-installer.active=true
2019-07-09T22:50:15.8564523Z submodule.src/rust-installer.url=https://github.com/rust-lang/rust-installer.git
2019-07-09T22:50:15.8564619Z submodule.src/tools/rustfmt.active=true
2019-07-09T22:50:15.8564941Z submodule.src/tools/rustfmt.url=https://github.com/rust-lang-nursery/rustfmt.git
2019-07-09T22:50:15.8664537Z ##[error]Bash exited with code '1'.
2019-07-09T22:50:15.8678835Z ##[section]Finishing: Verify line endings are LF
2019-07-09T22:50:15.8740501Z ==============================================================================
2019-07-09T22:50:15.8740574Z Task         : Bash
2019-07-09T22:50:15.8740662Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-09T22:50:15.8740722Z Version      : 3.151.2
---
2019-07-09T22:50:16.0285308Z Script contents:
2019-07-09T22:50:16.0287447Z aws s3 cp --acl public-read cpu-usage.csv s3://$DEPLOY_BUCKET/rustc-builds/$BUILD_SOURCEVERSION/cpu-$SYSTEM_JOBNAME.csv
2019-07-09T22:50:16.0303714Z ========================== Starting Command Output ===========================
2019-07-09T22:50:16.0322588Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/c0c80c39-1588-4cd4-9054-2e826b1176f4.sh
2019-07-09T22:50:16.0412677Z /home/vsts/work/_temp/c0c80c39-1588-4cd4-9054-2e826b1176f4.sh: line 1: aws: command not found
2019-07-09T22:50:16.0445911Z ##[error]Bash exited with code '127'.
2019-07-09T22:50:16.0475008Z ##[section]Starting: Checkout
2019-07-09T22:50:16.0476943Z ==============================================================================
2019-07-09T22:50:16.0477063Z Task         : Get sources
2019-07-09T22:50:16.0477160Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
