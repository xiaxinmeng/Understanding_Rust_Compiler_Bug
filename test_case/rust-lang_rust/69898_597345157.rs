plain
2020-03-10T20:26:45.4409534Z ========================== Starting Command Output ===========================
2020-03-10T20:26:45.4411722Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/7e77987c-e8e4-4a9e-a86c-ba5b08c60581.sh
2020-03-10T20:26:45.4412029Z 
2020-03-10T20:26:45.4416584Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-10T20:26:45.4436840Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69898/merge to s
2020-03-10T20:26:45.4440198Z Task         : Get sources
2020-03-10T20:26:45.4440434Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-10T20:26:45.4440659Z Version      : 1.0.0
2020-03-10T20:26:45.4440811Z Author       : Microsoft
---
2020-03-10T20:26:46.4515060Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-10T20:26:46.4519464Z ##[command]git config gc.auto 0
2020-03-10T20:26:46.4522443Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-10T20:26:46.4525169Z ##[command]git config --get-all http.proxy
2020-03-10T20:26:46.4529938Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69898/merge:refs/remotes/pull/69898/merge
---
2020-03-10T20:36:11.1094307Z configure: build.locked-deps    := True
2020-03-10T20:36:11.1094553Z configure: llvm.ccache          := sccache
2020-03-10T20:36:11.1094940Z configure: build.cargo-native-static := True
2020-03-10T20:36:11.1095352Z configure: dist.missing-tools   := True
2020-03-10T20:36:11.1095829Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-03-10T20:36:11.1096297Z configure: writing `config.toml` in current directory
2020-03-10T20:36:11.1096488Z configure: 
2020-03-10T20:36:11.1096816Z configure: run `python /checkout/x.py --help`
2020-03-10T20:36:11.1097019Z configure: 
---
2020-03-10T20:43:34.4418510Z Build completed successfully in 0:00:41
2020-03-10T20:43:34.4427940Z + /scripts/validate-toolstate.sh
2020-03-10T20:43:34.4462779Z Cloning into 'rust-toolstate'...
2020-03-10T20:43:35.1420522Z Traceback (most recent call last):
2020-03-10T20:43:35.1420885Z   File "../../src/tools/publish_toolstate.py", line 307, in <module>
2020-03-10T20:43:35.1421117Z     cur_datetime
2020-03-10T20:43:35.1421577Z   File "../../src/tools/publish_toolstate.py", line 207, in update_latest
2020-03-10T20:43:35.1422795Z     maintainers = ' '.join('@'+name for name in MAINTAINERS[tool])
2020-03-10T20:43:35.1423385Z KeyError: u'rustc-guide'
2020-03-10T20:43:35.1467619Z   local time: Tue Mar 10 20:43:35 UTC 2020
2020-03-10T20:43:35.3064314Z   network time: Tue, 10 Mar 2020 20:43:35 GMT
2020-03-10T20:43:35.3067104Z == end clock drift check ==
2020-03-10T20:43:36.2357644Z 
2020-03-10T20:43:36.2357644Z 
2020-03-10T20:43:36.2393809Z ##[error]Bash exited with code '1'.
2020-03-10T20:43:36.2407172Z ##[section]Finishing: Run build
2020-03-10T20:43:36.2451274Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69898/merge to s
2020-03-10T20:43:36.2455957Z Task         : Get sources
2020-03-10T20:43:36.2456253Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-10T20:43:36.2456537Z Version      : 1.0.0
2020-03-10T20:43:36.2456723Z Author       : Microsoft
2020-03-10T20:43:36.2456723Z Author       : Microsoft
2020-03-10T20:43:36.2457023Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-10T20:43:36.2457383Z ==============================================================================
2020-03-10T20:43:36.5584247Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-10T20:43:36.5628826Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69898/merge to s
2020-03-10T20:43:36.5722245Z Cleaning up task key
2020-03-10T20:43:36.5723376Z Start cleaning up orphan processes.
2020-03-10T20:43:36.5889650Z Terminate orphan process: pid (4153) (python)
2020-03-10T20:43:36.6038601Z ##[section]Finishing: Finalize Job
