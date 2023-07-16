plain
2019-12-19T19:28:35.5980340Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-19T19:28:35.6180035Z ##[command]git config gc.auto 0
2019-12-19T19:28:35.6255084Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-19T19:28:35.6316872Z ##[command]git config --get-all http.proxy
2019-12-19T19:28:35.6458806Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67268/merge:refs/remotes/pull/67268/merge
---
2019-12-19T19:38:35.6759900Z configure: build.locked-deps    := True
2019-12-19T19:38:35.6761289Z configure: llvm.ccache          := sccache
2019-12-19T19:38:35.6761699Z configure: build.cargo-native-static := True
2019-12-19T19:38:35.6761923Z configure: dist.missing-tools   := True
2019-12-19T19:38:35.6762200Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2019-12-19T19:38:35.6762334Z configure: writing `config.toml` in current directory
2019-12-19T19:38:35.6762378Z configure: 
2019-12-19T19:38:35.6763328Z configure: run `python /checkout/x.py --help`
2019-12-19T19:38:35.6763550Z configure: 
---
2019-12-19T19:46:24.0389271Z Build completed successfully in 0:01:30
2019-12-19T19:46:24.0395560Z + /scripts/validate-toolstate.sh
2019-12-19T19:46:24.0444529Z Cloning into 'rust-toolstate'...
2019-12-19T19:46:24.5966355Z Traceback (most recent call last):
2019-12-19T19:46:24.5966460Z   File "../../src/tools/publish_toolstate.py", line 303, in <module>
2019-12-19T19:46:24.5966558Z     cur_datetime
2019-12-19T19:46:24.5966603Z   File "../../src/tools/publish_toolstate.py", line 182, in update_latest
2019-12-19T19:46:24.5967250Z     for os in ['windows', 'linux']
2019-12-19T19:46:24.5967321Z   File "../../src/tools/publish_toolstate.py", line 182, in <dictcomp>
2019-12-19T19:46:24.5967583Z     for os in ['windows', 'linux']
2019-12-19T19:46:24.5967628Z   File "../../src/tools/publish_toolstate.py", line 111, in read_current_status
2019-12-19T19:46:24.5967862Z     (commit, status) = line.split('\t', 1)
2019-12-19T19:46:24.5967922Z ValueError: need more than 1 value to unpack
2019-12-19T19:46:24.6012456Z   local time: Thu Dec 19 19:46:24 UTC 2019
2019-12-19T19:46:24.8642648Z   network time: Thu, 19 Dec 2019 19:46:24 GMT
2019-12-19T19:46:24.8650066Z == end clock drift check ==
2019-12-19T19:46:26.6759789Z 
2019-12-19T19:46:26.6759789Z 
2019-12-19T19:46:26.6873259Z ##[error]Bash exited with code '1'.
2019-12-19T19:46:26.6918417Z ##[section]Starting: Checkout
2019-12-19T19:46:26.6919827Z ==============================================================================
2019-12-19T19:46:26.6919868Z Task         : Get sources
2019-12-19T19:46:26.6919920Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
