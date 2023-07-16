plain
2019-12-19T20:28:37.3282672Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-19T20:28:37.3526796Z ##[command]git config gc.auto 0
2019-12-19T20:28:37.3589344Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-19T20:28:37.3653706Z ##[command]git config --get-all http.proxy
2019-12-19T20:28:37.3805209Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67437/merge:refs/remotes/pull/67437/merge
---
2019-12-19T20:38:44.8874423Z configure: build.locked-deps    := True
2019-12-19T20:38:44.8874470Z configure: llvm.ccache          := sccache
2019-12-19T20:38:44.8874692Z configure: build.cargo-native-static := True
2019-12-19T20:38:44.8874892Z configure: dist.missing-tools   := True
2019-12-19T20:38:44.8875138Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2019-12-19T20:38:44.8875296Z configure: writing `config.toml` in current directory
2019-12-19T20:38:44.8875341Z configure: 
2019-12-19T20:38:44.8875614Z configure: run `python /checkout/x.py --help`
2019-12-19T20:38:44.8875661Z configure: 
---
2019-12-19T20:46:56.7915137Z Build completed successfully in 0:01:36
2019-12-19T20:46:56.7924923Z + /scripts/validate-toolstate.sh
2019-12-19T20:46:56.7994761Z Cloning into 'rust-toolstate'...
2019-12-19T20:46:57.3263505Z Traceback (most recent call last):
2019-12-19T20:46:57.3264416Z   File "../../src/tools/publish_toolstate.py", line 303, in <module>
2019-12-19T20:46:57.3264706Z     cur_datetime
2019-12-19T20:46:57.3264889Z   File "../../src/tools/publish_toolstate.py", line 182, in update_latest
2019-12-19T20:46:57.3268162Z     for os in ['windows', 'linux']
2019-12-19T20:46:57.3268564Z   File "../../src/tools/publish_toolstate.py", line 182, in <dictcomp>
2019-12-19T20:46:57.3269024Z     for os in ['windows', 'linux']
2019-12-19T20:46:57.3269279Z   File "../../src/tools/publish_toolstate.py", line 111, in read_current_status
2019-12-19T20:46:57.3270213Z     (commit, status) = line.split('\t', 1)
2019-12-19T20:46:57.3270672Z ValueError: need more than 1 value to unpack
2019-12-19T20:46:57.3320862Z   local time: Thu Dec 19 20:46:57 UTC 2019
2019-12-19T20:46:57.5959915Z   network time: Thu, 19 Dec 2019 20:46:57 GMT
2019-12-19T20:46:57.5963776Z == end clock drift check ==
2019-12-19T20:46:59.2820469Z 
2019-12-19T20:46:59.2820469Z 
2019-12-19T20:46:59.2941954Z ##[error]Bash exited with code '1'.
2019-12-19T20:46:59.2989246Z ##[section]Starting: Checkout
2019-12-19T20:46:59.2991286Z ==============================================================================
2019-12-19T20:46:59.2991374Z Task         : Get sources
2019-12-19T20:46:59.2991428Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
