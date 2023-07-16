plain
2019-07-10T01:14:50.1939355Z git config --global core.autocrlf false
2019-07-10T01:14:50.1939535Z ========================== Starting Command Output ===========================
2019-07-10T01:14:50.1941906Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/76cf7343-26ef-4f05-9d0a-5156bc33bf57.sh
2019-07-10T01:14:50.3327611Z ##[section]Finishing: Disable git automatic line ending conversion
2019-07-10T01:14:50.3343766Z ##[section]Starting: git config pre checkout
2019-07-10T01:14:50.3346062Z Task         : Bash
2019-07-10T01:14:50.3346088Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-10T01:14:50.3346127Z Version      : 3.151.2
2019-07-10T01:14:50.3346151Z Author       : Microsoft Corporation
2019-07-10T01:14:50.3346151Z Author       : Microsoft Corporation
2019-07-10T01:14:50.3346194Z Help         : https://docs.microsoft.com/azure/devops/pipelines/tasks/utility/bash
2019-07-10T01:14:50.3346218Z ==============================================================================
2019-07-10T01:14:50.4659647Z Generating script.
2019-07-10T01:14:50.4662135Z ========================== Starting Command Output ===========================
2019-07-10T01:14:50.4665303Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/8cd9a4c7-7e86-4f55-b700-27a2d2316b7d.sh
2019-07-10T01:14:50.4735459Z file:/etc/gitconfig filter.lfs.clean=git-lfs clean -- %f
2019-07-10T01:14:50.4736038Z file:/etc/gitconfig filter.lfs.smudge=git-lfs smudge -- %f
2019-07-10T01:14:50.4736461Z file:/etc/gitconfig filter.lfs.process=git-lfs filter-process
2019-07-10T01:14:50.4736700Z file:/etc/gitconfig filter.lfs.required=true
2019-07-10T01:14:50.4736895Z file:/home/vsts/.gitconfig core.autocrlf=false
2019-07-10T01:14:50.4746610Z + git config --list --show-origin
2019-07-10T01:14:50.4795082Z ##[section]Finishing: git config pre checkout
2019-07-10T01:14:50.4802473Z ==============================================================================
2019-07-10T01:14:50.4802505Z Task         : Get sources
2019-07-10T01:14:50.4802534Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2019-07-10T01:14:50.4802560Z Version      : 1.0.0
---
2019-07-10T01:14:52.6204234Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-10T01:14:52.6368918Z ##[command]git config gc.auto 0
2019-07-10T01:14:52.6436056Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-10T01:14:52.6483813Z ##[command]git config --get-all http.proxy
2019-07-10T01:14:52.6605418Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62545/merge:refs/remotes/pull/62545/merge
---
2019-07-10T01:15:27.3786250Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-10T01:15:27.3787562Z 
2019-07-10T01:15:27.3789059Z   git checkout -b <new-branch-name>
2019-07-10T01:15:27.3790530Z 
2019-07-10T01:15:27.3791596Z HEAD is now at f77a7f566 Merge 6c073417be076ee9821e87de67958d7fdc475c4a into 0b680cfce544ff9a59d720020e397c4bf3346650
2019-07-10T01:15:27.3922691Z ##[section]Finishing: Checkout
2019-07-10T01:15:27.3929041Z ##[section]Starting: git config post checkout
2019-07-10T01:15:27.3931534Z Task         : Bash
2019-07-10T01:15:27.3931570Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-10T01:15:27.3931607Z Version      : 3.151.2
2019-07-10T01:15:27.3931641Z Author       : Microsoft Corporation
2019-07-10T01:15:27.3931641Z Author       : Microsoft Corporation
2019-07-10T01:15:27.3931698Z Help         : https://docs.microsoft.com/azure/devops/pipelines/tasks/utility/bash
2019-07-10T01:15:27.3931738Z ==============================================================================
2019-07-10T01:15:27.5101460Z Generating script.
2019-07-10T01:15:27.5112925Z Script contents:
2019-07-10T01:15:27.5113157Z git config --list --show-origin
2019-07-10T01:15:27.5163951Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/6b9f709c-5354-45d3-b756-b7d59ad6a5ad.sh
2019-07-10T01:15:27.5163951Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/6b9f709c-5354-45d3-b756-b7d59ad6a5ad.sh
2019-07-10T01:15:27.5235253Z file:/etc/gitconfig filter.lfs.clean=git-lfs clean -- %f
2019-07-10T01:15:27.5236538Z file:/etc/gitconfig filter.lfs.smudge=git-lfs smudge -- %f
2019-07-10T01:15:27.5237409Z file:/etc/gitconfig filter.lfs.process=git-lfs filter-process
2019-07-10T01:15:27.5237977Z file:/etc/gitconfig filter.lfs.required=true
2019-07-10T01:15:27.5238348Z file:/home/vsts/.gitconfig core.autocrlf=false
2019-07-10T01:15:27.5238576Z file:.git/config core.repositoryformatversion=0
2019-07-10T01:15:27.5238791Z file:.git/config core.filemode=true
2019-07-10T01:15:27.5239063Z file:.git/config core.bare=false
2019-07-10T01:15:27.5239419Z file:.git/config core.logallrefupdates=true
2019-07-10T01:15:27.5240386Z file:.git/config remote.origin.url=***
2019-07-10T01:15:27.5240828Z file:.git/config remote.origin.fetch=+refs/heads/*:refs/remotes/origin/*
2019-07-10T01:15:27.5241046Z file:.git/config gc.auto=0
2019-07-10T01:15:27.5288914Z ##[section]Finishing: git config post checkout
2019-07-10T01:15:27.5296857Z ==============================================================================
2019-07-10T01:15:27.5296959Z Task         : Bash
2019-07-10T01:15:27.5297008Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-10T01:15:27.5297055Z Version      : 3.151.2
---
2019-07-10T01:16:02.4887538Z + return 0
2019-07-10T01:16:02.4887757Z + travis_time_finish
2019-07-10T01:16:02.4888021Z + return 0
2019-07-10T01:16:02.4967442Z ##[section]Finishing: Check out submodules (Unix)
2019-07-10T01:16:02.4978463Z ##[section]Starting: Verify line endings are LF
2019-07-10T01:16:02.4980782Z Task         : Bash
2019-07-10T01:16:02.4980820Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-10T01:16:02.4980888Z Version      : 3.151.2
2019-07-10T01:16:02.4980921Z Author       : Microsoft Corporation
2019-07-10T01:16:02.4980921Z Author       : Microsoft Corporation
2019-07-10T01:16:02.4980958Z Help         : https://docs.microsoft.com/azure/devops/pipelines/tasks/utility/bash
2019-07-10T01:16:02.4981030Z ==============================================================================
2019-07-10T01:16:02.8857294Z Generating script.
2019-07-10T01:16:02.8889054Z ========================== Starting Command Output ===========================
2019-07-10T01:16:02.8908774Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/695e513b-096e-4572-ab34-00607a9b48a5.sh
2019-07-10T01:16:02.8969663Z + git config --list
2019-07-10T01:16:02.8985440Z filter.lfs.clean=git-lfs clean -- %f
2019-07-10T01:16:02.8986022Z filter.lfs.smudge=git-lfs smudge -- %f
2019-07-10T01:16:02.8986503Z filter.lfs.process=git-lfs filter-process
2019-07-10T01:16:02.8986737Z filter.lfs.required=true
2019-07-10T01:16:02.8986899Z core.autocrlf=false
2019-07-10T01:16:02.8987118Z core.repositoryformatversion=0
2019-07-10T01:16:02.8987278Z core.filemode=true
2019-07-10T01:16:02.8987461Z core.bare=false
2019-07-10T01:16:02.8987627Z core.logallrefupdates=true
2019-07-10T01:16:02.8988072Z remote.origin.url=***
2019-07-10T01:16:02.8988357Z remote.origin.fetch=+refs/heads/*:refs/remotes/origin/*
2019-07-10T01:16:02.8988533Z gc.auto=0
2019-07-10T01:16:02.8988895Z submodule.src/doc/edition-guide.active=true
2019-07-10T01:16:02.8989375Z submodule.src/doc/edition-guide.url=https://github.com/rust-lang-nursery/edition-guide.git
2019-07-10T01:16:02.8989788Z submodule.src/doc/embedded-book.active=true
2019-07-10T01:16:02.8990215Z submodule.src/doc/embedded-book.url=https://github.com/rust-embedded/book.git
2019-07-10T01:16:02.8990533Z submodule.src/doc/nomicon.active=true
2019-07-10T01:16:02.8991096Z submodule.src/doc/nomicon.url=https://github.com/rust-lang-nursery/nomicon.git
2019-07-10T01:16:02.8991855Z submodule.src/doc/reference.active=true
2019-07-10T01:16:02.8992432Z submodule.src/doc/reference.url=https://github.com/rust-lang-nursery/reference.git
2019-07-10T01:16:02.8995274Z submodule.src/doc/rustc-guide.active=true
2019-07-10T01:16:02.8996066Z submodule.src/doc/rustc-guide.url=***c-guide.git
2019-07-10T01:16:02.8996111Z submodule.src/stdsimd.active=true
2019-07-10T01:16:02.8996354Z submodule.src/stdsimd.url=https://github.com/rust-lang-nursery/stdsimd.git
2019-07-10T01:16:02.8996395Z submodule.src/tools/cargo.active=true
2019-07-10T01:16:02.8996582Z submodule.src/tools/cargo.url=https://github.com/rust-lang/cargo.git
2019-07-10T01:16:02.8996784Z submodule.src/tools/clippy.active=true
2019-07-10T01:16:02.8997045Z submodule.src/tools/clippy.url=https://github.com/rust-lang-nursery/rust-clippy.git
2019-07-10T01:16:02.8997087Z submodule.src/tools/miri.active=true
2019-07-10T01:16:02.8997270Z submodule.src/tools/miri.url=https://github.com/rust-lang/miri.git
2019-07-10T01:16:02.8997352Z submodule.src/tools/rls.active=true
2019-07-10T01:16:02.8997538Z submodule.src/tools/rls.url=https://github.com/rust-lang-nursery/rls.git
2019-07-10T01:16:02.8997733Z submodule.src/rust-installer.active=true
2019-07-10T01:16:02.8997986Z submodule.src/rust-installer.url=***-installer.git
2019-07-10T01:16:02.8998027Z submodule.src/tools/rustfmt.active=true
2019-07-10T01:16:02.8998226Z submodule.src/tools/rustfmt.url=https://github.com/rust-lang-nursery/rustfmt.git
2019-07-10T01:16:02.8998445Z + bash -c 'egrep -q $"\r"\$ src/tools/rust-installer/install-template.sh'
2019-07-10T01:16:02.9011977Z + '[' 0 = 0 ']'
2019-07-10T01:16:02.9033709Z + exit 1
2019-07-10T01:16:02.9120420Z ##[error]Bash exited with code '1'.
2019-07-10T01:16:02.9132873Z ##[section]Finishing: Verify line endings are LF
2019-07-10T01:16:02.9178212Z ==============================================================================
2019-07-10T01:16:02.9178254Z Task         : Get sources
2019-07-10T01:16:02.9178325Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2019-07-10T01:16:02.9178361Z Version      : 1.0.0
