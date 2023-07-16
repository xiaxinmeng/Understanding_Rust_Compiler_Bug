plain
2019-11-22T15:01:01.1333124Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-22T15:01:01.1568260Z ##[command]git config gc.auto 0
2019-11-22T15:01:01.1628188Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-22T15:01:01.1736551Z ##[command]git config --get-all http.proxy
2019-11-22T15:01:01.1898976Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66498/merge:refs/remotes/pull/66498/merge
---
2019-11-22T15:10:00.6195193Z configure: build.locked-deps    := True
2019-11-22T15:10:00.6195273Z configure: llvm.ccache          := sccache
2019-11-22T15:10:00.6195514Z configure: build.cargo-native-static := True
2019-11-22T15:10:00.6195741Z configure: dist.missing-tools   := True
2019-11-22T15:10:00.6196010Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2019-11-22T15:10:00.6196125Z configure: writing `config.toml` in current directory
2019-11-22T15:10:00.6196168Z configure: 
2019-11-22T15:10:00.6196384Z configure: run `python /checkout/x.py --help`
2019-11-22T15:10:00.6196443Z configure: 
---
2019-11-22T15:11:50.0627517Z     Checking cfg-if v0.1.8
2019-11-22T15:11:50.0668845Z     Checking alloc v0.0.0 (/checkout/src/liballoc)
2019-11-22T15:11:50.1506126Z     Checking rustc-demangle v0.1.16
2019-11-22T15:11:50.4748240Z     Checking panic_abort v0.0.0 (/checkout/src/libpanic_abort)
2019-11-22T15:11:50.5104576Z error[E0658]: use of unstable library feature 'core_intrinsics': intrinsics are unlikely to ever be stabilized, instead they should be used through stabilized interfaces in the rest of the standard library
2019-11-22T15:11:50.5104937Z   --> src/libpanic_abort/lib.rs:52:9
2019-11-22T15:11:50.5110491Z 52 |         core::intrinsics::abort();
2019-11-22T15:11:50.5111319Z    |         ^^^^^^^^^^^^^^^^^^^^^^^
2019-11-22T15:11:50.5111790Z    |
2019-11-22T15:11:50.5112291Z    = help: add `#![feature(core_intrinsics)]` to the crate attributes to enable
---
2019-11-22T15:11:52.6537310Z   local time: Fri Nov 22 15:11:52 UTC 2019
2019-11-22T15:11:52.8018607Z   network time: Fri, 22 Nov 2019 15:11:52 GMT
2019-11-22T15:11:52.8026085Z == end clock drift check ==
2019-11-22T15:12:01.8389106Z 
2019-11-22T15:12:01.8517284Z ##[error]Bash exited with code '1'.
2019-11-22T15:12:01.8550117Z ##[section]Starting: Checkout
2019-11-22T15:12:01.8552285Z ==============================================================================
2019-11-22T15:12:01.8552342Z Task         : Get sources
2019-11-22T15:12:01.8552404Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
