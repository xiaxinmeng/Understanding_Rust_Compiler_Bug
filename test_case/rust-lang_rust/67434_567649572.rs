plain
2019-12-19T18:55:32.3945467Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-19T18:55:33.3931148Z ##[command]git config gc.auto 0
2019-12-19T18:55:33.3936748Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-19T18:55:33.3939437Z ##[command]git config --get-all http.proxy
2019-12-19T18:55:33.3942744Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67434/merge:refs/remotes/pull/67434/merge
---
2019-12-19T19:04:12.5972067Z configure: build.locked-deps    := True
2019-12-19T19:04:12.5972113Z configure: llvm.ccache          := sccache
2019-12-19T19:04:12.5972313Z configure: build.cargo-native-static := True
2019-12-19T19:04:12.5972470Z configure: dist.missing-tools   := True
2019-12-19T19:04:12.5973250Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2019-12-19T19:04:12.5973377Z configure: writing `config.toml` in current directory
2019-12-19T19:04:12.5973419Z configure: 
2019-12-19T19:04:12.5973612Z configure: run `python /checkout/x.py --help`
2019-12-19T19:04:12.5973646Z configure: 
---
2019-12-19T19:10:44.5096983Z Build completed successfully in 0:01:17
2019-12-19T19:10:44.5103582Z + /scripts/validate-toolstate.sh
2019-12-19T19:10:44.5145748Z Cloning into 'rust-toolstate'...
2019-12-19T19:10:44.9590362Z Traceback (most recent call last):
2019-12-19T19:10:44.9591903Z   File "../../src/tools/publish_toolstate.py", line 303, in <module>
2019-12-19T19:10:44.9595428Z     cur_datetime
2019-12-19T19:10:44.9595504Z   File "../../src/tools/publish_toolstate.py", line 182, in update_latest
2019-12-19T19:10:44.9595981Z     for os in ['windows', 'linux']
2019-12-19T19:10:44.9596052Z   File "../../src/tools/publish_toolstate.py", line 182, in <dictcomp>
2019-12-19T19:10:44.9599287Z     for os in ['windows', 'linux']
2019-12-19T19:10:44.9599359Z   File "../../src/tools/publish_toolstate.py", line 111, in read_current_status
2019-12-19T19:10:44.9599564Z     (commit, status) = line.split('\t', 1)
2019-12-19T19:10:44.9599631Z ValueError: need more than 1 value to unpack
2019-12-19T19:10:44.9633310Z   local time: Thu Dec 19 19:10:44 UTC 2019
2019-12-19T19:10:45.2257675Z   network time: Thu, 19 Dec 2019 19:10:45 GMT
2019-12-19T19:10:45.2262113Z == end clock drift check ==
2019-12-19T19:10:47.0680659Z 
2019-12-19T19:10:47.0680659Z 
2019-12-19T19:10:47.0776370Z ##[error]Bash exited with code '1'.
2019-12-19T19:10:47.0801787Z ##[section]Starting: Checkout
2019-12-19T19:10:47.0803330Z ==============================================================================
2019-12-19T19:10:47.0803371Z Task         : Get sources
2019-12-19T19:10:47.0803425Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
