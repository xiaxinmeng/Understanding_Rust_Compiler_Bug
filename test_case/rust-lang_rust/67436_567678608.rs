plain
2019-12-19T19:47:02.0539728Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-19T19:47:02.0554552Z ##[command]git config gc.auto 0
2019-12-19T19:47:02.0560005Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-19T19:47:02.0575053Z ##[command]git config --get-all http.proxy
2019-12-19T19:47:02.0583438Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67436/merge:refs/remotes/pull/67436/merge
2019-12-19T19:47:02.9819078Z fatal: remote error: upload-pack: not our ref f5a30f554f9cadcd0286dbb6133a6d2434fd82e5
2019-12-19T19:47:02.9930323Z fatal: the remote end hung up unexpectedly
2019-12-19T19:47:03.1041661Z ##[warning]Git fetch failed with exit code 128, back off 7.741 seconds before retry.
2019-12-19T19:47:10.7646431Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67436/merge:refs/remotes/pull/67436/merge
---
2019-12-19T19:58:09.8440196Z configure: build.locked-deps    := True
2019-12-19T19:58:09.8440405Z configure: llvm.ccache          := sccache
2019-12-19T19:58:09.8440792Z configure: build.cargo-native-static := True
2019-12-19T19:58:09.8441212Z configure: dist.missing-tools   := True
2019-12-19T19:58:09.8441707Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2019-12-19T19:58:09.8442085Z configure: writing `config.toml` in current directory
2019-12-19T19:58:09.8442270Z configure: 
2019-12-19T19:58:09.8442702Z configure: run `python /checkout/x.py --help`
2019-12-19T19:58:09.8442921Z configure: 
---
2019-12-19T20:07:07.0046280Z Build completed successfully in 0:01:45
2019-12-19T20:07:07.0053158Z + /scripts/validate-toolstate.sh
2019-12-19T20:07:07.0117723Z Cloning into 'rust-toolstate'...
2019-12-19T20:07:07.6179234Z Traceback (most recent call last):
2019-12-19T20:07:07.6179360Z   File "../../src/tools/publish_toolstate.py", line 303, in <module>
2019-12-19T20:07:07.6179405Z     cur_datetime
2019-12-19T20:07:07.6179451Z   File "../../src/tools/publish_toolstate.py", line 182, in update_latest
2019-12-19T20:07:07.6180169Z     for os in ['windows', 'linux']
2019-12-19T20:07:07.6180222Z   File "../../src/tools/publish_toolstate.py", line 182, in <dictcomp>
2019-12-19T20:07:07.6180452Z     for os in ['windows', 'linux']
2019-12-19T20:07:07.6180523Z   File "../../src/tools/publish_toolstate.py", line 111, in read_current_status
2019-12-19T20:07:07.6180763Z     (commit, status) = line.split('\t', 1)
2019-12-19T20:07:07.6180812Z ValueError: need more than 1 value to unpack
2019-12-19T20:07:08.4326870Z   local time: Thu Dec 19 20:07:07 UTC 2019
2019-12-19T20:07:08.4326958Z   network time: Thu, 19 Dec 2019 20:07:07 GMT
2019-12-19T20:07:08.4327035Z == end clock drift check ==
2019-12-19T20:07:09.4069153Z 
2019-12-19T20:07:09.4069153Z 
2019-12-19T20:07:09.4073915Z ##[error]Bash exited with code '1'.
2019-12-19T20:07:09.4115091Z ##[section]Starting: Checkout
2019-12-19T20:07:09.4117003Z ==============================================================================
2019-12-19T20:07:09.4117055Z Task         : Get sources
2019-12-19T20:07:09.4117119Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
