plain
2019-12-19T20:03:03.1726349Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-19T20:03:03.1917838Z ##[command]git config gc.auto 0
2019-12-19T20:03:03.1989056Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-19T20:03:03.2047524Z ##[command]git config --get-all http.proxy
2019-12-19T20:03:03.2190353Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67268/merge:refs/remotes/pull/67268/merge
---
2019-12-19T20:13:14.2522460Z configure: build.locked-deps    := True
2019-12-19T20:13:14.2522508Z configure: llvm.ccache          := sccache
2019-12-19T20:13:14.2522719Z configure: build.cargo-native-static := True
2019-12-19T20:13:14.2522949Z configure: dist.missing-tools   := True
2019-12-19T20:13:14.2523205Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2019-12-19T20:13:14.2523321Z configure: writing `config.toml` in current directory
2019-12-19T20:13:14.2523366Z configure: 
2019-12-19T20:13:14.2523581Z configure: run `python /checkout/x.py --help`
2019-12-19T20:13:14.2523644Z configure: 
---
2019-12-19T20:21:37.4360396Z Build completed successfully in 0:01:39
2019-12-19T20:21:37.4369970Z + /scripts/validate-toolstate.sh
2019-12-19T20:21:37.4427946Z Cloning into 'rust-toolstate'...
2019-12-19T20:21:38.0197015Z Traceback (most recent call last):
2019-12-19T20:21:38.0197125Z   File "../../src/tools/publish_toolstate.py", line 303, in <module>
2019-12-19T20:21:38.0197178Z     cur_datetime
2019-12-19T20:21:38.0197250Z   File "../../src/tools/publish_toolstate.py", line 182, in update_latest
2019-12-19T20:21:38.0197944Z     for os in ['windows', 'linux']
2019-12-19T20:21:38.0198001Z   File "../../src/tools/publish_toolstate.py", line 182, in <dictcomp>
2019-12-19T20:21:38.0198228Z     for os in ['windows', 'linux']
2019-12-19T20:21:38.0198312Z   File "../../src/tools/publish_toolstate.py", line 111, in read_current_status
2019-12-19T20:21:38.0198554Z     (commit, status) = line.split('\t', 1)
2019-12-19T20:21:38.0198607Z ValueError: need more than 1 value to unpack
2019-12-19T20:21:38.0254097Z   local time: Thu Dec 19 20:21:38 UTC 2019
2019-12-19T20:21:38.1621321Z   network time: Thu, 19 Dec 2019 20:21:38 GMT
2019-12-19T20:21:38.1624204Z == end clock drift check ==
2019-12-19T20:21:39.9388126Z 
2019-12-19T20:21:39.9388126Z 
2019-12-19T20:21:39.9474396Z ##[error]Bash exited with code '1'.
2019-12-19T20:21:39.9505897Z ##[section]Starting: Checkout
2019-12-19T20:21:39.9507369Z ==============================================================================
2019-12-19T20:21:39.9507415Z Task         : Get sources
2019-12-19T20:21:39.9507453Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
