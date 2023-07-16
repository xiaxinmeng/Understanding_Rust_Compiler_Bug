plain
2019-07-10T11:25:59.2267542Z git config --global core.autocrlf false
2019-07-10T11:25:59.2289492Z ========================== Starting Command Output ===========================
2019-07-10T11:25:59.2311567Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/73462623-3d90-4c23-9d36-66ab5e383c89.sh
2019-07-10T11:25:59.4664671Z ##[section]Finishing: Disable git automatic line ending conversion
2019-07-10T11:25:59.4680565Z ##[section]Starting: git config pre checkout
2019-07-10T11:25:59.4683838Z Task         : Bash
2019-07-10T11:25:59.4683892Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-10T11:25:59.4683945Z Version      : 3.151.2
2019-07-10T11:25:59.4683978Z Author       : Microsoft Corporation
2019-07-10T11:25:59.4683978Z Author       : Microsoft Corporation
2019-07-10T11:25:59.4684012Z Help         : https://docs.microsoft.com/azure/devops/pipelines/tasks/utility/bash
2019-07-10T11:25:59.4684063Z ==============================================================================
2019-07-10T11:25:59.6015647Z Generating script.
2019-07-10T11:25:59.6047773Z ========================== Starting Command Output ===========================
2019-07-10T11:25:59.6067646Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/5e044218-5575-4a46-bd53-dbfc56c241e7.sh
2019-07-10T11:25:59.6145754Z file:/etc/gitconfig filter.lfs.clean=git-lfs clean -- %f
2019-07-10T11:25:59.6146649Z file:/etc/gitconfig filter.lfs.smudge=git-lfs smudge -- %f
2019-07-10T11:25:59.6147149Z file:/etc/gitconfig filter.lfs.process=git-lfs filter-process
2019-07-10T11:25:59.6147390Z file:/etc/gitconfig filter.lfs.required=true
2019-07-10T11:25:59.6147565Z file:/home/vsts/.gitconfig core.autocrlf=false
2019-07-10T11:25:59.6158195Z + git config --list --show-origin
2019-07-10T11:25:59.6213038Z ##[section]Finishing: git config pre checkout
2019-07-10T11:25:59.6220237Z ==============================================================================
2019-07-10T11:25:59.6220275Z Task         : Get sources
2019-07-10T11:25:59.6220341Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2019-07-10T11:25:59.6220371Z Version      : 1.0.0
---
2019-07-10T11:26:01.7269636Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-10T11:26:01.7445719Z ##[command]git config gc.auto 0
2019-07-10T11:26:01.7504670Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-10T11:26:01.7560238Z ##[command]git config --get-all http.proxy
2019-07-10T11:26:01.7701369Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62545/merge:refs/remotes/pull/62545/merge
---
2019-07-10T11:26:37.2353600Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-10T11:26:37.2353651Z 
2019-07-10T11:26:37.2354117Z   git checkout -b <new-branch-name>
2019-07-10T11:26:37.2354147Z 
2019-07-10T11:26:37.2354798Z HEAD is now at ea6878b3d Merge 2ac559a9e2b48b4e6a8b94953e4a9f97fd36a588 into 0324a2b309cd66cb7bd4a156bd0b84cb136e254f
2019-07-10T11:26:37.2512331Z ##[section]Finishing: Checkout
2019-07-10T11:26:37.2520618Z ##[section]Starting: git config post checkout
2019-07-10T11:26:37.2525277Z Task         : Bash
2019-07-10T11:26:37.2525340Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-10T11:26:37.2525385Z Version      : 3.151.2
2019-07-10T11:26:37.2525444Z Author       : Microsoft Corporation
2019-07-10T11:26:37.2525444Z Author       : Microsoft Corporation
2019-07-10T11:26:37.2525509Z Help         : https://docs.microsoft.com/azure/devops/pipelines/tasks/utility/bash
2019-07-10T11:26:37.2525559Z ==============================================================================
2019-07-10T11:26:37.3875351Z Generating script.
2019-07-10T11:26:37.3885505Z Script contents:
2019-07-10T11:26:37.3886024Z git config --list --show-origin
2019-07-10T11:26:37.3938286Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/19c548cc-1077-4102-9f91-d7ffe81f7e92.sh
2019-07-10T11:26:37.3938286Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/19c548cc-1077-4102-9f91-d7ffe81f7e92.sh
2019-07-10T11:26:37.4013243Z file:/etc/gitconfig filter.lfs.clean=git-lfs clean -- %f
2019-07-10T11:26:37.4013823Z file:/etc/gitconfig filter.lfs.smudge=git-lfs smudge -- %f
2019-07-10T11:26:37.4014042Z file:/etc/gitconfig filter.lfs.process=git-lfs filter-process
2019-07-10T11:26:37.4014136Z file:/etc/gitconfig filter.lfs.required=true
2019-07-10T11:26:37.4014212Z file:/home/vsts/.gitconfig core.autocrlf=false
2019-07-10T11:26:37.4014255Z file:.git/config core.repositoryformatversion=0
2019-07-10T11:26:37.4014299Z file:.git/config core.filemode=true
2019-07-10T11:26:37.4014377Z file:.git/config core.bare=false
2019-07-10T11:26:37.4014420Z file:.git/config core.logallrefupdates=true
2019-07-10T11:26:37.4014776Z file:.git/config remote.origin.url=***
2019-07-10T11:26:37.4014832Z file:.git/config remote.origin.fetch=+refs/heads/*:refs/remotes/origin/*
2019-07-10T11:26:37.4014876Z file:.git/config gc.auto=0
2019-07-10T11:26:37.4098205Z ##[section]Finishing: git config post checkout
2019-07-10T11:26:37.4107602Z ==============================================================================
2019-07-10T11:26:37.4107651Z Task         : Bash
2019-07-10T11:26:37.4107734Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-10T11:26:37.4107777Z Version      : 3.151.2
---
2019-07-10T11:27:13.7453882Z + return 0
2019-07-10T11:27:13.7454129Z + travis_time_finish
2019-07-10T11:27:13.7454422Z + return 0
2019-07-10T11:27:13.7572038Z ##[section]Finishing: Check out submodules (Unix)
2019-07-10T11:27:13.7584514Z ##[section]Starting: Verify line endings are LF
2019-07-10T11:27:13.7614960Z Task         : Bash
2019-07-10T11:27:13.7615040Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-10T11:27:13.7615098Z Version      : 3.151.2
2019-07-10T11:27:13.7615135Z Author       : Microsoft Corporation
2019-07-10T11:27:13.7615135Z Author       : Microsoft Corporation
2019-07-10T11:27:13.7615214Z Help         : https://docs.microsoft.com/azure/devops/pipelines/tasks/utility/bash
2019-07-10T11:27:13.7615260Z ==============================================================================
2019-07-10T11:27:13.9056013Z Generating script.
2019-07-10T11:27:13.9089814Z ========================== Starting Command Output ===========================
2019-07-10T11:27:13.9109703Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/824dfc51-11c6-4b9d-ada0-11c811909d87.sh
2019-07-10T11:27:13.9179447Z file:/etc/gitconfig filter.lfs.clean=git-lfs clean -- %f
2019-07-10T11:27:13.9180036Z file:/etc/gitconfig filter.lfs.smudge=git-lfs smudge -- %f
2019-07-10T11:27:13.9180489Z file:/etc/gitconfig filter.lfs.process=git-lfs filter-process
2019-07-10T11:27:13.9180679Z file:/etc/gitconfig filter.lfs.required=true
2019-07-10T11:27:13.9180832Z file:/home/vsts/.gitconfig core.autocrlf=false
2019-07-10T11:27:13.9180922Z file:.git/config core.repositoryformatversion=0
2019-07-10T11:27:13.9180971Z file:.git/config core.filemode=true
2019-07-10T11:27:13.9181065Z file:.git/config core.bare=false
2019-07-10T11:27:13.9181114Z file:.git/config core.logallrefupdates=true
2019-07-10T11:27:13.9181503Z file:.git/config remote.origin.url=***
2019-07-10T11:27:13.9181922Z file:.git/config remote.origin.fetch=+refs/heads/*:refs/remotes/origin/*
2019-07-10T11:27:13.9182041Z file:.git/config gc.auto=0
2019-07-10T11:27:13.9182316Z file:.git/config submodule.src/doc/edition-guide.active=true
2019-07-10T11:27:13.9182814Z file:.git/config submodule.src/doc/edition-guide.url=https://github.com/rust-lang-nursery/edition-guide.git
2019-07-10T11:27:13.9183217Z file:.git/config submodule.src/doc/embedded-book.active=true
2019-07-10T11:27:13.9183687Z file:.git/config submodule.src/doc/embedded-book.url=https://github.com/rust-embedded/book.git
2019-07-10T11:27:13.9183863Z file:.git/config submodule.src/doc/nomicon.active=true
2019-07-10T11:27:13.9184556Z file:.git/config submodule.src/doc/nomicon.url=https://github.com/rust-lang-nursery/nomicon.git
2019-07-10T11:27:13.9184738Z file:.git/config submodule.src/doc/reference.active=true
2019-07-10T11:27:13.9185065Z file:.git/config submodule.src/doc/reference.url=https://github.com/rust-lang-nursery/reference.git
2019-07-10T11:27:13.9185345Z file:.git/config submodule.src/doc/rustc-guide.active=true
2019-07-10T11:27:13.9185635Z file:.git/config submodule.src/doc/rustc-guide.url=***c-guide.git
2019-07-10T11:27:13.9185695Z file:.git/config submodule.src/stdsimd.active=true
2019-07-10T11:27:13.9186179Z file:.git/config submodule.src/stdsimd.url=https://github.com/rust-lang-nursery/stdsimd.git
2019-07-10T11:27:13.9186356Z file:.git/config submodule.src/tools/cargo.active=true
2019-07-10T11:27:13.9186705Z file:.git/config submodule.src/tools/cargo.url=https://github.com/rust-lang/cargo.git
2019-07-10T11:27:13.9186879Z file:.git/config submodule.src/tools/clippy.active=true
2019-07-10T11:27:13.9187408Z file:.git/config submodule.src/tools/clippy.url=https://github.com/rust-lang-nursery/rust-clippy.git
2019-07-10T11:27:13.9187689Z file:.git/config submodule.src/tools/miri.active=true
2019-07-10T11:27:13.9188442Z file:.git/config submodule.src/tools/miri.url=https://github.com/rust-lang/miri.git
2019-07-10T11:27:13.9188720Z file:.git/config submodule.src/tools/rls.active=true
2019-07-10T11:27:13.9189086Z file:.git/config submodule.src/tools/rls.url=https://github.com/rust-lang-nursery/rls.git
2019-07-10T11:27:13.9189362Z file:.git/config submodule.src/rust-installer.active=true
2019-07-10T11:27:13.9190080Z file:.git/config submodule.src/rust-installer.url=***-installer.git
2019-07-10T11:27:13.9190315Z file:.git/config submodule.src/tools/rustfmt.active=true
2019-07-10T11:27:13.9190788Z file:.git/config submodule.src/tools/rustfmt.url=https://github.com/rust-lang-nursery/rustfmt.git
2019-07-10T11:27:13.9192944Z + git config --list --show-origin
2019-07-10T11:27:13.9193383Z + bash -c 'egrep -q $"\r"\$ src/tools/rust-installer/install-template.sh'
2019-07-10T11:27:13.9220392Z + '[' 0 = 0 ']'
2019-07-10T11:27:13.9220949Z + exit 1
2019-07-10T11:27:13.9350455Z ##[error]Bash exited with code '1'.
2019-07-10T11:27:13.9363902Z ##[section]Finishing: Verify line endings are LF
2019-07-10T11:27:13.9411965Z ==============================================================================
2019-07-10T11:27:13.9412069Z Task         : Get sources
2019-07-10T11:27:13.9412117Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2019-07-10T11:27:13.9412162Z Version      : 1.0.0
