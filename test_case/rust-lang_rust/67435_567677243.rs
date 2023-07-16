plain
2019-12-19T19:21:55.5269654Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-19T19:21:55.5452255Z ##[command]git config gc.auto 0
2019-12-19T19:21:55.5502771Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-19T19:21:55.5545484Z ##[command]git config --get-all http.proxy
2019-12-19T19:21:55.5650960Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67435/merge:refs/remotes/pull/67435/merge
---
2019-12-19T19:30:48.2091236Z configure: build.locked-deps    := True
2019-12-19T19:30:48.2091289Z configure: llvm.ccache          := sccache
2019-12-19T19:30:48.2091460Z configure: build.cargo-native-static := True
2019-12-19T19:30:48.2091619Z configure: dist.missing-tools   := True
2019-12-19T19:30:48.2091831Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2019-12-19T19:30:48.2091912Z configure: writing `config.toml` in current directory
2019-12-19T19:30:48.2091946Z configure: 
2019-12-19T19:30:48.2092159Z configure: run `python /checkout/x.py --help`
2019-12-19T19:30:48.2092197Z configure: 
---
2019-12-19T19:37:32.1837987Z Build completed successfully in 0:01:18
2019-12-19T19:37:32.1845409Z + /scripts/validate-toolstate.sh
2019-12-19T19:37:32.1891308Z Cloning into 'rust-toolstate'...
2019-12-19T19:37:32.7163696Z Traceback (most recent call last):
2019-12-19T19:37:32.7163834Z   File "../../src/tools/publish_toolstate.py", line 303, in <module>
2019-12-19T19:37:32.7164142Z     cur_datetime
2019-12-19T19:37:32.7164190Z   File "../../src/tools/publish_toolstate.py", line 182, in update_latest
2019-12-19T19:37:32.7164728Z     for os in ['windows', 'linux']
2019-12-19T19:37:32.7164802Z   File "../../src/tools/publish_toolstate.py", line 182, in <dictcomp>
2019-12-19T19:37:32.7165027Z     for os in ['windows', 'linux']
2019-12-19T19:37:32.7165073Z   File "../../src/tools/publish_toolstate.py", line 111, in read_current_status
2019-12-19T19:37:32.7165331Z     (commit, status) = line.split('\t', 1)
2019-12-19T19:37:32.7165378Z ValueError: need more than 1 value to unpack
2019-12-19T19:37:32.7202782Z   local time: Thu Dec 19 19:37:32 UTC 2019
2019-12-19T19:37:32.9824830Z   network time: Thu, 19 Dec 2019 19:37:32 GMT
2019-12-19T19:37:32.9830549Z == end clock drift check ==
2019-12-19T19:37:34.7695923Z 
2019-12-19T19:37:34.7695923Z 
2019-12-19T19:37:34.7743975Z ##[error]Bash exited with code '1'.
2019-12-19T19:37:34.7778934Z ##[section]Starting: Checkout
2019-12-19T19:37:34.7780332Z ==============================================================================
2019-12-19T19:37:34.7780391Z Task         : Get sources
2019-12-19T19:37:34.7780427Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
