plain
2020-03-23T23:42:56.2541955Z ========================== Starting Command Output ===========================
2020-03-23T23:42:56.2544593Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/a78bfe7c-d650-475f-83c0-856f635db2b1.sh
2020-03-23T23:42:56.2544836Z 
2020-03-23T23:42:56.2548909Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-23T23:42:56.2566874Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70342/merge to s
2020-03-23T23:42:56.2571485Z Task         : Get sources
2020-03-23T23:42:56.2571774Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-23T23:42:56.2572051Z Version      : 1.0.0
2020-03-23T23:42:56.2572252Z Author       : Microsoft
---
2020-03-23T23:42:57.2428455Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-23T23:42:57.2433961Z ##[command]git config gc.auto 0
2020-03-23T23:42:57.2437648Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-23T23:42:57.2441067Z ##[command]git config --get-all http.proxy
2020-03-23T23:42:57.2447161Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70342/merge:refs/remotes/pull/70342/merge
---
2020-03-23T23:51:28.7511131Z configure: build.locked-deps    := True
2020-03-23T23:51:28.7511424Z configure: llvm.ccache          := sccache
2020-03-23T23:51:28.7511918Z configure: build.cargo-native-static := True
2020-03-23T23:51:28.7512380Z configure: dist.missing-tools   := True
2020-03-23T23:51:28.7512967Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-03-23T23:51:28.7513823Z configure: writing `config.toml` in current directory
2020-03-23T23:51:28.7514060Z configure: 
2020-03-23T23:51:28.7514517Z configure: run `python /checkout/x.py --help`
2020-03-23T23:51:28.7514742Z configure: 
---
2020-03-23T23:59:35.8031946Z skip untracked path cpu-usage.csv during rustfmt invocations
2020-03-23T23:59:35.8032398Z skip untracked path src/doc/book/ during rustfmt invocations
2020-03-23T23:59:35.8037684Z skip untracked path src/doc/rust-by-example/ during rustfmt invocations
2020-03-23T23:59:35.8041468Z skip untracked path src/llvm-project/ during rustfmt invocations
2020-03-23T23:59:38.7035608Z Diff in /checkout/src/libstd/io/mod.rs at line 952:
2020-03-23T23:59:38.7036617Z  pub struct IoSliceMut<'a>(sys::io::IoSliceMut<'a>);
2020-03-23T23:59:38.7037035Z  
2020-03-23T23:59:38.7037493Z  #[stable(feature = "iovec-send-sync", since = "1.44.0")]
2020-03-23T23:59:38.7037988Z -unsafe impl<'a> Send for IoSliceMut<'a> { }
2020-03-23T23:59:38.7038459Z +unsafe impl<'a> Send for IoSliceMut<'a> {}
2020-03-23T23:59:38.7038654Z  
2020-03-23T23:59:38.7039066Z  #[stable(feature = "iovec-send-sync", since = "1.44.0")]
2020-03-23T23:59:38.7039557Z -unsafe impl<'a> Sync for IoSliceMut<'a> { }
2020-03-23T23:59:38.7040002Z +unsafe impl<'a> Sync for IoSliceMut<'a> {}
2020-03-23T23:59:38.7040197Z  
2020-03-23T23:59:38.7040417Z  #[stable(feature = "iovec", since = "1.36.0")]
2020-03-23T23:59:38.7040981Z  impl<'a> fmt::Debug for IoSliceMut<'a> {
2020-03-23T23:59:38.7041263Z Diff in /checkout/src/libstd/io/mod.rs at line 1061:
2020-03-23T23:59:38.7041746Z  pub struct IoSlice<'a>(sys::io::IoSlice<'a>);
2020-03-23T23:59:38.7041955Z  
2020-03-23T23:59:38.7042351Z  #[stable(feature = "iovec-send-sync", since = "1.44.0")]
2020-03-23T23:59:38.7042824Z -unsafe impl<'a> Send for IoSlice<'a> { }
2020-03-23T23:59:38.7043247Z +unsafe impl<'a> Send for IoSlice<'a> {}
2020-03-23T23:59:38.7043437Z  
2020-03-23T23:59:38.7043831Z  #[stable(feature = "iovec-send-sync", since = "1.44.0")]
2020-03-23T23:59:38.7044288Z -unsafe impl<'a> Sync for IoSlice<'a> { }
2020-03-23T23:59:38.7044705Z +unsafe impl<'a> Sync for IoSlice<'a> {}
2020-03-23T23:59:38.7044888Z  
2020-03-23T23:59:38.7045098Z  #[stable(feature = "iovec", since = "1.36.0")]
2020-03-23T23:59:38.7045528Z  impl<'a> fmt::Debug for IoSlice<'a> {
2020-03-23T23:59:38.7049410Z Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/libstd/io/mod.rs"` failed.
2020-03-23T23:59:38.7050415Z If you're running `tidy`, try again with `--bless` flag. Or, you just want to format code, run `./x.py fmt` instead.
2020-03-23T23:59:38.7075866Z Build completed unsuccessfully in 0:00:41
2020-03-23T23:59:38.7125660Z == clock drift check ==
2020-03-23T23:59:38.7138717Z   local time: Mon Mar 23 23:59:38 UTC 2020
2020-03-23T23:59:39.0031428Z   network time: Mon, 23 Mar 2020 23:59:38 GMT
2020-03-23T23:59:39.0031428Z   network time: Mon, 23 Mar 2020 23:59:38 GMT
2020-03-23T23:59:39.0035185Z == end clock drift check ==
2020-03-23T23:59:40.5267665Z 
2020-03-23T23:59:40.5341988Z ##[error]Bash exited with code '1'.
2020-03-23T23:59:40.5354715Z ##[section]Finishing: Run build
2020-03-23T23:59:40.5400257Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70342/merge to s
2020-03-23T23:59:40.5404715Z Task         : Get sources
2020-03-23T23:59:40.5405032Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-23T23:59:40.5405328Z Version      : 1.0.0
2020-03-23T23:59:40.5405530Z Author       : Microsoft
2020-03-23T23:59:40.5405530Z Author       : Microsoft
2020-03-23T23:59:40.5405838Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-23T23:59:40.5406224Z ==============================================================================
2020-03-23T23:59:40.8548671Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-23T23:59:40.8604280Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70342/merge to s
2020-03-23T23:59:40.8696795Z Cleaning up task key
2020-03-23T23:59:40.8697971Z Start cleaning up orphan processes.
2020-03-23T23:59:40.8888688Z Terminate orphan process: pid (4195) (python)
2020-03-23T23:59:40.9125283Z ##[section]Finishing: Finalize Job
