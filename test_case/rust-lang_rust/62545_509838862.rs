plain
2019-07-09T22:48:44.8214854Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-09T22:48:44.8402581Z ##[command]git config gc.auto 0
2019-07-09T22:48:44.8475866Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-09T22:48:44.8529824Z ##[command]git config --get-all http.proxy
2019-07-09T22:48:44.8677082Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62545/merge:refs/remotes/pull/62545/merge
---
2019-07-09T22:49:19.7007313Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-09T22:49:19.7007351Z 
2019-07-09T22:49:19.7007642Z   git checkout -b <new-branch-name>
2019-07-09T22:49:19.7007695Z 
2019-07-09T22:49:19.7007751Z HEAD is now at c9cb4b938 Merge 74a3e2d6d68ceecf2d06851609b910030c3cc86a into 0b680cfce544ff9a59d720020e397c4bf3346650
2019-07-09T22:49:19.7161800Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-09T22:49:19.7165298Z ==============================================================================
2019-07-09T22:49:19.7165366Z Task         : Bash
2019-07-09T22:49:19.7165435Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-09T22:50:13.8890987Z + return 0
2019-07-09T22:50:13.8891321Z + travis_time_finish
2019-07-09T22:50:13.8891549Z + return 0
2019-07-09T22:50:13.8974821Z ##[section]Finishing: Check out submodules (Unix)
2019-07-09T22:50:13.8991142Z ##[section]Starting: Verify line endings are LF
2019-07-09T22:50:13.8993998Z Task         : Bash
2019-07-09T22:50:13.8994044Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-09T22:50:13.8994127Z Version      : 3.151.2
2019-07-09T22:50:13.8994167Z Author       : Microsoft Corporation
2019-07-09T22:50:13.8994167Z Author       : Microsoft Corporation
2019-07-09T22:50:13.8994211Z Help         : https://docs.microsoft.com/azure/devops/pipelines/tasks/utility/bash
2019-07-09T22:50:13.8994299Z ==============================================================================
2019-07-09T22:50:14.0432443Z Generating script.
2019-07-09T22:50:14.0460610Z ========================== Starting Command Output ===========================
2019-07-09T22:50:14.0479775Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/0ea7f6da-7d37-4a2e-b3ca-8ef436179f89.sh
2019-07-09T22:50:14.0540543Z filter.lfs.clean=git-lfs clean -- %f
2019-07-09T22:50:14.0541583Z filter.lfs.smudge=git-lfs smudge -- %f
2019-07-09T22:50:14.0542129Z filter.lfs.process=git-lfs filter-process
2019-07-09T22:50:14.0542459Z filter.lfs.required=true
2019-07-09T22:50:14.0542670Z core.autocrlf=false
2019-07-09T22:50:14.0542862Z core.repositoryformatversion=0
2019-07-09T22:50:14.0543094Z core.filemode=true
2019-07-09T22:50:14.0543290Z core.bare=false
2019-07-09T22:50:14.0543477Z core.logallrefupdates=true
2019-07-09T22:50:14.0544181Z remote.origin.url=***
2019-07-09T22:50:14.0544535Z remote.origin.fetch=+refs/heads/*:refs/remotes/origin/*
2019-07-09T22:50:14.0544630Z gc.auto=0
2019-07-09T22:50:14.0545214Z submodule.src/doc/edition-guide.active=true
2019-07-09T22:50:14.0546212Z submodule.src/doc/edition-guide.url=https://github.com/rust-lang-nursery/edition-guide.git
2019-07-09T22:50:14.0546810Z submodule.src/doc/embedded-book.active=true
2019-07-09T22:50:14.0547805Z submodule.src/doc/embedded-book.url=https://github.com/rust-embedded/book.git
2019-07-09T22:50:14.0548059Z submodule.src/doc/nomicon.active=true
2019-07-09T22:50:14.0548515Z submodule.src/doc/nomicon.url=https://github.com/rust-lang-nursery/nomicon.git
2019-07-09T22:50:14.0548731Z submodule.src/doc/reference.active=true
2019-07-09T22:50:14.0553411Z submodule.src/doc/reference.url=https://github.com/rust-lang-nursery/reference.git
2019-07-09T22:50:14.0553979Z submodule.src/doc/rustc-guide.active=true
2019-07-09T22:50:14.0554471Z submodule.src/doc/rustc-guide.url=***c-guide.git
2019-07-09T22:50:14.0554731Z submodule.src/stdsimd.active=true
2019-07-09T22:50:14.0555071Z submodule.src/stdsimd.url=https://github.com/rust-lang-nursery/stdsimd.git
2019-07-09T22:50:14.0555122Z submodule.src/tools/cargo.active=true
2019-07-09T22:50:14.0556253Z submodule.src/tools/cargo.url=https://github.com/rust-lang/cargo.git
2019-07-09T22:50:14.0556513Z submodule.src/tools/clippy.active=true
2019-07-09T22:50:14.0556906Z submodule.src/tools/clippy.url=https://github.com/rust-lang-nursery/rust-clippy.git
2019-07-09T22:50:14.0557194Z submodule.src/tools/miri.active=true
2019-07-09T22:50:14.0557549Z submodule.src/tools/miri.url=https://github.com/rust-lang/miri.git
2019-07-09T22:50:14.0557659Z submodule.src/tools/rls.active=true
2019-07-09T22:50:14.0558133Z submodule.src/tools/rls.url=https://github.com/rust-lang-nursery/rls.git
2019-07-09T22:50:14.0558612Z submodule.src/rust-installer.active=true
2019-07-09T22:50:14.0559191Z submodule.src/rust-installer.url=***-installer.git
2019-07-09T22:50:14.0591156Z submodule.src/tools/rustfmt.active=true
2019-07-09T22:50:14.0591587Z submodule.src/tools/rustfmt.url=https://github.com/rust-lang-nursery/rustfmt.git
2019-07-09T22:50:14.0683469Z ##[error]Bash exited with code '1'.
2019-07-09T22:50:14.0694902Z ##[section]Finishing: Verify line endings are LF
2019-07-09T22:50:14.0740000Z ==============================================================================
2019-07-09T22:50:14.0740061Z Task         : Get sources
2019-07-09T22:50:14.0740141Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2019-07-09T22:50:14.0740181Z Version      : 1.0.0
