plain
2019-12-19T20:09:35.7461506Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-19T20:09:35.7475585Z ##[command]git config gc.auto 0
2019-12-19T20:09:35.7478447Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-19T20:09:35.7481112Z ##[command]git config --get-all http.proxy
2019-12-19T20:09:35.7485837Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67160/merge:refs/remotes/pull/67160/merge
---
2019-12-19T20:19:40.1114141Z configure: build.locked-deps    := True
2019-12-19T20:19:40.1114183Z configure: llvm.ccache          := sccache
2019-12-19T20:19:40.1114377Z configure: build.cargo-native-static := True
2019-12-19T20:19:40.1114583Z configure: dist.missing-tools   := True
2019-12-19T20:19:40.1114816Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2019-12-19T20:19:40.1114907Z configure: writing `config.toml` in current directory
2019-12-19T20:19:40.1114965Z configure: 
2019-12-19T20:19:40.1115193Z configure: run `python /checkout/x.py --help`
2019-12-19T20:19:40.1115246Z configure: 
---
2019-12-19T20:27:26.8917339Z Build completed successfully in 0:01:31
2019-12-19T20:27:26.8926495Z + /scripts/validate-toolstate.sh
2019-12-19T20:27:26.8983206Z Cloning into 'rust-toolstate'...
2019-12-19T20:27:27.4023856Z Traceback (most recent call last):
2019-12-19T20:27:27.4024640Z   File "../../src/tools/publish_toolstate.py", line 303, in <module>
2019-12-19T20:27:27.4024944Z     cur_datetime
2019-12-19T20:27:27.4025157Z   File "../../src/tools/publish_toolstate.py", line 182, in update_latest
2019-12-19T20:27:27.4026114Z     for os in ['windows', 'linux']
2019-12-19T20:27:27.4026460Z   File "../../src/tools/publish_toolstate.py", line 182, in <dictcomp>
2019-12-19T20:27:27.4026891Z     for os in ['windows', 'linux']
2019-12-19T20:27:27.4027154Z   File "../../src/tools/publish_toolstate.py", line 111, in read_current_status
2019-12-19T20:27:27.4027762Z     (commit, status) = line.split('\t', 1)
2019-12-19T20:27:27.4028052Z ValueError: need more than 1 value to unpack
2019-12-19T20:27:27.4064102Z   local time: Thu Dec 19 20:27:27 UTC 2019
2019-12-19T20:27:27.9273819Z   network time: Thu, 19 Dec 2019 20:27:27 GMT
2019-12-19T20:27:27.9279804Z == end clock drift check ==
2019-12-19T20:27:29.7928189Z 
2019-12-19T20:27:29.7928189Z 
2019-12-19T20:27:29.8012912Z ##[error]Bash exited with code '1'.
2019-12-19T20:27:29.8074879Z ##[section]Starting: Checkout
2019-12-19T20:27:29.8077860Z ==============================================================================
2019-12-19T20:27:29.8077939Z Task         : Get sources
2019-12-19T20:27:29.8077988Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
