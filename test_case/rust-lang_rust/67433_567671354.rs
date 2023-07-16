plain
2019-12-19T18:52:26.3146504Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-19T18:52:26.3376373Z ##[command]git config gc.auto 0
2019-12-19T18:52:26.3460266Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-19T18:52:26.3524488Z ##[command]git config --get-all http.proxy
2019-12-19T18:52:26.3692630Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67433/merge:refs/remotes/pull/67433/merge
---
2019-12-19T19:03:03.1888752Z configure: build.locked-deps    := True
2019-12-19T19:03:03.1888817Z configure: llvm.ccache          := sccache
2019-12-19T19:03:03.1889100Z configure: build.cargo-native-static := True
2019-12-19T19:03:03.1889369Z configure: dist.missing-tools   := True
2019-12-19T19:03:03.1889673Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2019-12-19T19:03:03.1889814Z configure: writing `config.toml` in current directory
2019-12-19T19:03:03.1889867Z configure: 
2019-12-19T19:03:03.1890157Z configure: run `python /checkout/x.py --help`
2019-12-19T19:03:03.1890232Z configure: 
---
2019-12-19T19:11:45.8216455Z Build completed successfully in 0:01:42
2019-12-19T19:11:45.8226490Z + /scripts/validate-toolstate.sh
2019-12-19T19:11:45.8289752Z Cloning into 'rust-toolstate'...
2019-12-19T19:11:46.4361364Z Traceback (most recent call last):
2019-12-19T19:11:46.4361492Z   File "../../src/tools/publish_toolstate.py", line 303, in <module>
2019-12-19T19:11:46.4361542Z     cur_datetime
2019-12-19T19:11:46.4368152Z   File "../../src/tools/publish_toolstate.py", line 182, in update_latest
2019-12-19T19:11:46.4368867Z     for os in ['windows', 'linux']
2019-12-19T19:11:46.4368929Z   File "../../src/tools/publish_toolstate.py", line 182, in <dictcomp>
2019-12-19T19:11:46.4369191Z     for os in ['windows', 'linux']
2019-12-19T19:11:46.4369246Z   File "../../src/tools/publish_toolstate.py", line 111, in read_current_status
2019-12-19T19:11:46.4369486Z     (commit, status) = line.split('\t', 1)
2019-12-19T19:11:46.4369556Z ValueError: need more than 1 value to unpack
2019-12-19T19:11:46.4409737Z   local time: Thu Dec 19 19:11:46 UTC 2019
2019-12-19T19:11:46.7068442Z   network time: Thu, 19 Dec 2019 19:11:46 GMT
2019-12-19T19:11:46.7072524Z == end clock drift check ==
2019-12-19T19:11:48.2993996Z 
2019-12-19T19:11:48.2993996Z 
2019-12-19T19:11:48.3103607Z ##[error]Bash exited with code '1'.
2019-12-19T19:11:48.3144232Z ##[section]Starting: Checkout
2019-12-19T19:11:48.3146362Z ==============================================================================
2019-12-19T19:11:48.3146425Z Task         : Get sources
2019-12-19T19:11:48.3146479Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
