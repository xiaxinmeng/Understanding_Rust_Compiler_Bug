plain
2020-03-10T14:19:50.6483950Z ========================== Starting Command Output ===========================
2020-03-10T14:19:50.6486737Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/304d695e-1273-448f-9553-992ee628b7c6.sh
2020-03-10T14:19:50.6486983Z 
2020-03-10T14:19:50.6491339Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-10T14:19:50.6510430Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69747/merge to s
2020-03-10T14:19:50.6513659Z Task         : Get sources
2020-03-10T14:19:50.6513929Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-10T14:19:50.6514188Z Version      : 1.0.0
2020-03-10T14:19:50.6514429Z Author       : Microsoft
---
2020-03-10T14:19:51.8160851Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-10T14:19:51.8170160Z ##[command]git config gc.auto 0
2020-03-10T14:19:51.8178122Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-10T14:19:51.8184390Z ##[command]git config --get-all http.proxy
2020-03-10T14:19:51.8196533Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69747/merge:refs/remotes/pull/69747/merge
---
2020-03-10T14:28:28.2763082Z configure: build.locked-deps    := True
2020-03-10T14:28:28.2763379Z configure: llvm.ccache          := sccache
2020-03-10T14:28:28.2763848Z configure: build.cargo-native-static := True
2020-03-10T14:28:28.2764303Z configure: dist.missing-tools   := True
2020-03-10T14:28:28.2764894Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-03-10T14:28:28.2765416Z configure: writing `config.toml` in current directory
2020-03-10T14:28:28.2765658Z configure: 
2020-03-10T14:28:28.2766051Z configure: run `python /checkout/x.py --help`
2020-03-10T14:28:28.2766265Z configure: 
---
2020-03-10T14:36:00.3399103Z Build completed successfully in 0:00:43
2020-03-10T14:36:00.3409015Z + /scripts/validate-toolstate.sh
2020-03-10T14:36:00.3458977Z Cloning into 'rust-toolstate'...
2020-03-10T14:36:01.0332278Z Traceback (most recent call last):
2020-03-10T14:36:01.0332659Z   File "../../src/tools/publish_toolstate.py", line 307, in <module>
2020-03-10T14:36:01.0332967Z     cur_datetime
2020-03-10T14:36:01.0333232Z   File "../../src/tools/publish_toolstate.py", line 207, in update_latest
2020-03-10T14:36:01.0334493Z     maintainers = ' '.join('@'+name for name in MAINTAINERS[tool])
2020-03-10T14:36:01.0334976Z KeyError: u'rustc-guide'
2020-03-10T14:36:01.0384060Z   local time: Tue Mar 10 14:36:01 UTC 2020
2020-03-10T14:36:01.3265433Z   network time: Tue, 10 Mar 2020 14:36:01 GMT
2020-03-10T14:36:01.3265728Z == end clock drift check ==
2020-03-10T14:36:02.0944975Z 
2020-03-10T14:36:02.0944975Z 
2020-03-10T14:36:02.1026810Z ##[error]Bash exited with code '1'.
2020-03-10T14:36:02.1041706Z ##[section]Finishing: Run build
2020-03-10T14:36:02.1092484Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69747/merge to s
2020-03-10T14:36:02.1097275Z Task         : Get sources
2020-03-10T14:36:02.1097593Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-10T14:36:02.1097903Z Version      : 1.0.0
2020-03-10T14:36:02.1098107Z Author       : Microsoft
2020-03-10T14:36:02.1098107Z Author       : Microsoft
2020-03-10T14:36:02.1098437Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-10T14:36:02.1098830Z ==============================================================================
2020-03-10T14:36:02.4485157Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-10T14:36:02.4527596Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69747/merge to s
2020-03-10T14:36:02.4621988Z Cleaning up task key
2020-03-10T14:36:02.4623152Z Start cleaning up orphan processes.
2020-03-10T14:36:02.4808693Z Terminate orphan process: pid (3783) (python)
2020-03-10T14:36:02.5052810Z ##[section]Finishing: Finalize Job
