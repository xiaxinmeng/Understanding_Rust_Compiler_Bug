plain
2019-12-19T17:37:47.7650466Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-19T17:37:47.7827423Z ##[command]git config gc.auto 0
2019-12-19T17:37:47.7902419Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-19T17:37:47.7955998Z ##[command]git config --get-all http.proxy
2019-12-19T17:37:47.8093855Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67000/merge:refs/remotes/pull/67000/merge
---
2019-12-19T17:47:44.4603145Z configure: build.locked-deps    := True
2019-12-19T17:47:44.4603207Z configure: llvm.ccache          := sccache
2019-12-19T17:47:44.4603380Z configure: build.cargo-native-static := True
2019-12-19T17:47:44.4603545Z configure: dist.missing-tools   := True
2019-12-19T17:47:44.4604328Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2019-12-19T17:47:44.4605063Z configure: writing `config.toml` in current directory
2019-12-19T17:47:44.4605150Z configure: 
2019-12-19T17:47:44.4605419Z configure: run `python /checkout/x.py --help`
2019-12-19T17:47:44.4605461Z configure: 
---
2019-12-19T17:55:38.2003065Z Build completed successfully in 0:01:33
2019-12-19T17:55:38.2013506Z + /scripts/validate-toolstate.sh
2019-12-19T17:55:38.2095521Z Cloning into 'rust-toolstate'...
2019-12-19T17:55:38.8620509Z Traceback (most recent call last):
2019-12-19T17:55:38.8620953Z   File "../../src/tools/publish_toolstate.py", line 303, in <module>
2019-12-19T17:55:38.8622119Z     cur_datetime
2019-12-19T17:55:38.8622385Z   File "../../src/tools/publish_toolstate.py", line 182, in update_latest
2019-12-19T17:55:38.8623277Z     for os in ['windows', 'linux']
2019-12-19T17:55:38.8623569Z   File "../../src/tools/publish_toolstate.py", line 182, in <dictcomp>
2019-12-19T17:55:38.8624058Z     for os in ['windows', 'linux']
2019-12-19T17:55:38.8624334Z   File "../../src/tools/publish_toolstate.py", line 111, in read_current_status
2019-12-19T17:55:38.8625019Z     (commit, status) = line.split('\t', 1)
2019-12-19T17:55:38.8625749Z ValueError: need more than 1 value to unpack
2019-12-19T17:55:38.8677119Z   local time: Thu Dec 19 17:55:38 UTC 2019
2019-12-19T17:55:39.1384868Z   network time: Thu, 19 Dec 2019 17:55:39 GMT
2019-12-19T17:55:39.1392608Z == end clock drift check ==
2019-12-19T17:55:41.0432741Z 
2019-12-19T17:55:41.0432741Z 
2019-12-19T17:55:41.0518164Z ##[error]Bash exited with code '1'.
2019-12-19T17:55:41.0575231Z ##[section]Starting: Checkout
2019-12-19T17:55:41.0576989Z ==============================================================================
2019-12-19T17:55:41.0577032Z Task         : Get sources
2019-12-19T17:55:41.0577190Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
