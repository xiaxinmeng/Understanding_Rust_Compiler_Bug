plain
2019-12-19T17:31:28.5436426Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-19T17:31:28.5447455Z ##[command]git config gc.auto 0
2019-12-19T17:31:28.5449746Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-19T17:31:29.5430864Z ##[command]git config --get-all http.proxy
2019-12-19T17:31:29.5433813Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67192/merge:refs/remotes/pull/67192/merge
---
2019-12-19T17:42:25.6556850Z configure: build.locked-deps    := True
2019-12-19T17:42:25.6556919Z configure: llvm.ccache          := sccache
2019-12-19T17:42:25.6557139Z configure: build.cargo-native-static := True
2019-12-19T17:42:25.6557354Z configure: dist.missing-tools   := True
2019-12-19T17:42:25.6557643Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2019-12-19T17:42:25.6557744Z configure: writing `config.toml` in current directory
2019-12-19T17:42:25.6557817Z configure: 
2019-12-19T17:42:25.6558044Z configure: run `python /checkout/x.py --help`
2019-12-19T17:42:25.6558090Z configure: 
---
2019-12-19T17:51:20.1465323Z Build completed successfully in 0:01:45
2019-12-19T17:51:20.1473351Z + /scripts/validate-toolstate.sh
2019-12-19T17:51:20.1523545Z Cloning into 'rust-toolstate'...
2019-12-19T17:51:20.7179712Z Traceback (most recent call last):
2019-12-19T17:51:20.7180098Z   File "../../src/tools/publish_toolstate.py", line 303, in <module>
2019-12-19T17:51:20.7180362Z     cur_datetime
2019-12-19T17:51:20.7180429Z   File "../../src/tools/publish_toolstate.py", line 182, in update_latest
2019-12-19T17:51:20.7181274Z     for os in ['windows', 'linux']
2019-12-19T17:51:20.7181347Z   File "../../src/tools/publish_toolstate.py", line 182, in <dictcomp>
2019-12-19T17:51:20.7181561Z     for os in ['windows', 'linux']
2019-12-19T17:51:20.7181612Z   File "../../src/tools/publish_toolstate.py", line 111, in read_current_status
2019-12-19T17:51:20.7181830Z     (commit, status) = line.split('\t', 1)
2019-12-19T17:51:20.7181892Z ValueError: need more than 1 value to unpack
2019-12-19T17:51:20.7233673Z   local time: Thu Dec 19 17:51:20 UTC 2019
2019-12-19T17:51:20.9888935Z   network time: Thu, 19 Dec 2019 17:51:20 GMT
2019-12-19T17:51:20.9889086Z == end clock drift check ==
2019-12-19T17:51:22.6779282Z 
2019-12-19T17:51:22.6779282Z 
2019-12-19T17:51:22.6872373Z ##[error]Bash exited with code '1'.
2019-12-19T17:51:22.6902460Z ##[section]Starting: Checkout
2019-12-19T17:51:22.6904560Z ==============================================================================
2019-12-19T17:51:22.6904614Z Task         : Get sources
2019-12-19T17:51:22.6904659Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
