plain
2019-12-19T20:29:45.3003049Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-19T20:29:45.3193389Z ##[command]git config gc.auto 0
2019-12-19T20:29:45.3283906Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-19T20:29:45.3331753Z ##[command]git config --get-all http.proxy
2019-12-19T20:29:45.3492794Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67314/merge:refs/remotes/pull/67314/merge
---
2019-12-19T20:40:45.3904767Z configure: build.locked-deps    := True
2019-12-19T20:40:45.3905162Z configure: llvm.ccache          := sccache
2019-12-19T20:40:45.3905508Z configure: build.cargo-native-static := True
2019-12-19T20:40:45.3905750Z configure: dist.missing-tools   := True
2019-12-19T20:40:45.3906113Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2019-12-19T20:40:45.3906245Z configure: writing `config.toml` in current directory
2019-12-19T20:40:45.3908081Z configure: 
2019-12-19T20:40:45.3909002Z configure: run `python /checkout/x.py --help`
2019-12-19T20:40:45.3909089Z configure: 
---
2019-12-19T20:49:47.2063537Z Build completed successfully in 0:01:47
2019-12-19T20:49:47.2071894Z + /scripts/validate-toolstate.sh
2019-12-19T20:49:47.2140185Z Cloning into 'rust-toolstate'...
2019-12-19T20:49:47.8113797Z Traceback (most recent call last):
2019-12-19T20:49:47.8113917Z   File "../../src/tools/publish_toolstate.py", line 303, in <module>
2019-12-19T20:49:47.8114017Z     cur_datetime
2019-12-19T20:49:47.8114062Z   File "../../src/tools/publish_toolstate.py", line 182, in update_latest
2019-12-19T20:49:47.8114743Z     for os in ['windows', 'linux']
2019-12-19T20:49:47.8114816Z   File "../../src/tools/publish_toolstate.py", line 182, in <dictcomp>
2019-12-19T20:49:47.8115025Z     for os in ['windows', 'linux']
2019-12-19T20:49:47.8115073Z   File "../../src/tools/publish_toolstate.py", line 111, in read_current_status
2019-12-19T20:49:47.8115343Z     (commit, status) = line.split('\t', 1)
2019-12-19T20:49:47.8115388Z ValueError: need more than 1 value to unpack
2019-12-19T20:49:47.8166511Z   local time: Thu Dec 19 20:49:47 UTC 2019
2019-12-19T20:49:48.0930246Z   network time: Thu, 19 Dec 2019 20:49:48 GMT
2019-12-19T20:49:48.0934595Z == end clock drift check ==
2019-12-19T20:49:49.6699769Z 
2019-12-19T20:49:49.6699769Z 
2019-12-19T20:49:49.6806165Z ##[error]Bash exited with code '1'.
2019-12-19T20:49:49.6839627Z ##[section]Starting: Checkout
2019-12-19T20:49:49.6841210Z ==============================================================================
2019-12-19T20:49:49.6841261Z Task         : Get sources
2019-12-19T20:49:49.6841324Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
