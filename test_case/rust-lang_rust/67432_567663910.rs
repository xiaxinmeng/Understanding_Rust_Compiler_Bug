plain
2019-12-19T18:50:19.7531310Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-19T18:50:19.7543506Z ##[command]git config gc.auto 0
2019-12-19T18:50:19.7545371Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-19T18:50:19.7546886Z ##[command]git config --get-all http.proxy
2019-12-19T18:50:19.7614500Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67432/merge:refs/remotes/pull/67432/merge
---
2019-12-19T19:00:32.2664211Z configure: build.locked-deps    := True
2019-12-19T19:00:32.2664272Z configure: llvm.ccache          := sccache
2019-12-19T19:00:32.2664466Z configure: build.cargo-native-static := True
2019-12-19T19:00:32.2664652Z configure: dist.missing-tools   := True
2019-12-19T19:00:32.2664902Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2019-12-19T19:00:32.2664996Z configure: writing `config.toml` in current directory
2019-12-19T19:00:32.2665053Z configure: 
2019-12-19T19:00:32.2665280Z configure: run `python /checkout/x.py --help`
2019-12-19T19:00:32.2665324Z configure: 
---
2019-12-19T19:09:07.9230770Z Build completed successfully in 0:01:43
2019-12-19T19:09:07.9239930Z + /scripts/validate-toolstate.sh
2019-12-19T19:09:07.9301747Z Cloning into 'rust-toolstate'...
2019-12-19T19:09:08.4800738Z Traceback (most recent call last):
2019-12-19T19:09:08.4801464Z   File "../../src/tools/publish_toolstate.py", line 303, in <module>
2019-12-19T19:09:08.4801894Z     cur_datetime
2019-12-19T19:09:08.4801993Z   File "../../src/tools/publish_toolstate.py", line 182, in update_latest
2019-12-19T19:09:08.4802657Z     for os in ['windows', 'linux']
2019-12-19T19:09:08.4802791Z   File "../../src/tools/publish_toolstate.py", line 182, in <dictcomp>
2019-12-19T19:09:08.4803003Z     for os in ['windows', 'linux']
2019-12-19T19:09:08.4803053Z   File "../../src/tools/publish_toolstate.py", line 111, in read_current_status
2019-12-19T19:09:08.4803287Z     (commit, status) = line.split('\t', 1)
2019-12-19T19:09:08.4803340Z ValueError: need more than 1 value to unpack
2019-12-19T19:09:08.4850113Z   local time: Thu Dec 19 19:09:08 UTC 2019
2019-12-19T19:09:08.7477272Z   network time: Thu, 19 Dec 2019 19:09:08 GMT
2019-12-19T19:09:08.7481638Z == end clock drift check ==
2019-12-19T19:09:10.7489649Z 
2019-12-19T19:09:10.7489649Z 
2019-12-19T19:09:10.7645878Z ##[error]Bash exited with code '1'.
2019-12-19T19:09:10.7681477Z ##[section]Starting: Checkout
2019-12-19T19:09:10.7683449Z ==============================================================================
2019-12-19T19:09:10.7683525Z Task         : Get sources
2019-12-19T19:09:10.7683573Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
