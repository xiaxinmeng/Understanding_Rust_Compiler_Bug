plain
2020-03-17T19:36:59.3336432Z ========================== Starting Command Output ===========================
2020-03-17T19:36:59.3341688Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/5af0f539-bc08-4dd2-b661-fb6fcd4a22cc.sh
2020-03-17T19:36:59.3341994Z 
2020-03-17T19:36:59.3347220Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-17T19:36:59.3368770Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67343/merge to s
2020-03-17T19:36:59.3372545Z Task         : Get sources
2020-03-17T19:36:59.3372874Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-17T19:36:59.3373206Z Version      : 1.0.0
2020-03-17T19:36:59.3373423Z Author       : Microsoft
---
2020-03-17T19:37:00.3275818Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-17T19:37:00.3284997Z ##[command]git config gc.auto 0
2020-03-17T19:37:00.3291615Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-17T19:37:00.3295722Z ##[command]git config --get-all http.proxy
2020-03-17T19:37:00.3303656Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67343/merge:refs/remotes/pull/67343/merge
---
2020-03-17T19:45:54.5011844Z configure: build.locked-deps    := True
2020-03-17T19:45:54.5012172Z configure: llvm.ccache          := sccache
2020-03-17T19:45:54.5012698Z configure: build.cargo-native-static := True
2020-03-17T19:45:54.5013226Z configure: dist.missing-tools   := True
2020-03-17T19:45:54.5013876Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-03-17T19:45:54.5014503Z configure: writing `config.toml` in current directory
2020-03-17T19:45:54.5014768Z configure: 
2020-03-17T19:45:54.5015210Z configure: run `python /checkout/x.py --help`
2020-03-17T19:45:54.5015459Z configure: 
---
2020-03-17T19:53:11.7044524Z skip untracked path src/llvm-project/ during rustfmt invocations
2020-03-17T19:53:12.1433163Z Diff in /checkout/src/libcore/cell.rs at line 191:
2020-03-17T19:53:12.1437709Z  
2020-03-17T19:53:12.1448361Z  use crate::cmp::Ordering;
2020-03-17T19:53:12.1448703Z  use crate::fmt::{self, Debug, Display};
2020-03-17T19:53:12.1450039Z -use crate::marker::{Unsize, StructuralPartialEq, StructuralEq};
2020-03-17T19:53:12.1450454Z +use crate::marker::{StructuralEq, StructuralPartialEq, Unsize};
2020-03-17T19:53:12.1451057Z  use crate::ops::{CoerceUnsized, Deref, DerefMut};
2020-03-17T19:53:12.1451331Z  use crate::ptr;
2020-03-17T19:53:12.1451331Z  use crate::ptr;
2020-03-17T19:53:12.1461223Z Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/libcore/cell.rs"` failed.
2020-03-17T19:53:12.1462365Z If you're running `tidy`, try again with `--bless` flag. Or, you just want to format code, run `./x.py fmt` instead.
2020-03-17T19:53:12.1477449Z Build completed unsuccessfully in 0:00:04
2020-03-17T19:53:12.1531077Z == clock drift check ==
2020-03-17T19:53:12.1547656Z   local time: Tue Mar 17 19:53:12 UTC 2020
2020-03-17T19:53:12.6992411Z   network time: Tue, 17 Mar 2020 19:53:12 GMT
2020-03-17T19:53:12.6992411Z   network time: Tue, 17 Mar 2020 19:53:12 GMT
2020-03-17T19:53:12.6992874Z == end clock drift check ==
2020-03-17T19:53:13.3665897Z 
2020-03-17T19:53:13.3750319Z ##[error]Bash exited with code '1'.
2020-03-17T19:53:13.3765824Z ##[section]Finishing: Run build
2020-03-17T19:53:13.3844755Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67343/merge to s
2020-03-17T19:53:13.3850510Z Task         : Get sources
2020-03-17T19:53:13.3850895Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-17T19:53:13.3851268Z Version      : 1.0.0
2020-03-17T19:53:13.3851514Z Author       : Microsoft
2020-03-17T19:53:13.3851514Z Author       : Microsoft
2020-03-17T19:53:13.3851902Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-17T19:53:13.3852375Z ==============================================================================
2020-03-17T19:53:13.7659271Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-17T19:53:13.7702613Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/67343/merge to s
2020-03-17T19:53:13.7824787Z Cleaning up task key
2020-03-17T19:53:13.7826203Z Start cleaning up orphan processes.
2020-03-17T19:53:13.8032567Z Terminate orphan process: pid (3561) (python)
2020-03-17T19:53:13.8330403Z ##[section]Finishing: Finalize Job
