plain
2019-11-26T19:42:01.3916613Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-26T19:42:01.3928276Z ##[command]git config gc.auto 0
2019-11-26T19:42:01.3931835Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-26T19:42:01.3935034Z ##[command]git config --get-all http.proxy
2019-11-26T19:42:01.3938432Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66498/merge:refs/remotes/pull/66498/merge
---
2019-11-26T19:49:47.1005428Z configure: build.locked-deps    := True
2019-11-26T19:49:47.1005467Z configure: llvm.ccache          := sccache
2019-11-26T19:49:47.1005655Z configure: build.cargo-native-static := True
2019-11-26T19:49:47.1005821Z configure: dist.missing-tools   := True
2019-11-26T19:49:47.1006025Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2019-11-26T19:49:47.1006127Z configure: writing `config.toml` in current directory
2019-11-26T19:49:47.1006173Z configure: 
2019-11-26T19:49:47.1006367Z configure: run `python /checkout/x.py --help`
2019-11-26T19:49:47.1006405Z configure: 
---
2019-11-26T19:51:23.4148474Z     Checking panic_abort v0.0.0 (/checkout/src/libpanic_abort)
2019-11-26T19:51:23.5214587Z     Checking backtrace v0.3.40
2019-11-26T19:51:25.2371475Z     Checking rustc-std-workspace-alloc v1.99.0 (/checkout/src/tools/rustc-std-workspace-alloc)
2019-11-26T19:51:25.2394786Z     Checking panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
2019-11-26T19:51:26.8504545Z error[E0658]: use of unstable library feature 'slice_internals': exposed from core to be reused in std; use the memchr crate
2019-11-26T19:51:26.8505651Z   --> src/libstd/memchr.rs:25:5
2019-11-26T19:51:26.8506118Z    |
2019-11-26T19:51:26.8506525Z 25 |     crate::sys::memchr::memchr(needle, haystack)
2019-11-26T19:51:26.8507283Z    |
2019-11-26T19:51:26.8507745Z    = help: add `#![feature(slice_internals)]` to the crate attributes to enable
2019-11-26T19:51:26.8507929Z 
2019-11-26T19:51:26.8507929Z 
2019-11-26T19:51:26.8508382Z error[E0658]: use of unstable library feature 'slice_internals': exposed from core to be reused in std; use the memchr crate
2019-11-26T19:51:26.8508776Z   --> src/libstd/memchr.rs:45:5
2019-11-26T19:51:26.8509117Z    |
2019-11-26T19:51:26.8509538Z 45 |     crate::sys::memchr::memrchr(needle, haystack)
2019-11-26T19:51:26.8510314Z    |
2019-11-26T19:51:26.8510729Z    = help: add `#![feature(slice_internals)]` to the crate attributes to enable
2019-11-26T19:51:26.8510885Z 
2019-11-26T19:51:26.8510885Z 
2019-11-26T19:51:26.8522781Z error[E0658]: use of unstable library feature 'slice_internals': exposed from core to be reused in std; use the memchr crate
2019-11-26T19:51:26.8523363Z  --> src/libstd/sys/windows/memchr.rs:5:31
2019-11-26T19:51:26.8523770Z   |
2019-11-26T19:51:26.8524190Z 5 | pub use core::slice::memchr::{memchr, memrchr};
2019-11-26T19:51:26.8524941Z   |
2019-11-26T19:51:26.8525392Z   = help: add `#![feature(slice_internals)]` to the crate attributes to enable
2019-11-26T19:51:26.8525557Z 
2019-11-26T19:51:26.8525557Z 
2019-11-26T19:51:26.8533091Z error[E0658]: use of unstable library feature 'slice_internals': exposed from core to be reused in std; use the memchr crate
2019-11-26T19:51:26.8533609Z  --> src/libstd/sys/windows/memchr.rs:5:39
2019-11-26T19:51:26.8534190Z   |
2019-11-26T19:51:26.8534633Z 5 | pub use core::slice::memchr::{memchr, memrchr};
2019-11-26T19:51:26.8535748Z   |
2019-11-26T19:51:26.8536171Z   = help: add `#![feature(slice_internals)]` to the crate attributes to enable
2019-11-26T19:51:26.8539345Z 
2019-11-26T19:51:28.3879172Z error: aborting due to 4 previous errors
---
2019-11-26T19:51:28.4279039Z   local time: Tue Nov 26 19:51:28 UTC 2019
2019-11-26T19:51:28.4968040Z   network time: Tue, 26 Nov 2019 19:51:28 GMT
2019-11-26T19:51:28.4968103Z == end clock drift check ==
2019-11-26T19:51:37.7173704Z 
2019-11-26T19:51:37.7263187Z ##[error]Bash exited with code '1'.
2019-11-26T19:51:37.7296306Z ##[section]Starting: Checkout
2019-11-26T19:51:37.7297864Z ==============================================================================
2019-11-26T19:51:37.7297906Z Task         : Get sources
2019-11-26T19:51:37.7297943Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
