plain
2020-02-25T17:40:36.2556915Z ========================== Starting Command Output ===========================
2020-02-25T17:40:36.2559377Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/0d459a57-569a-4995-8a32-15f98c3f08e7.sh
2020-02-25T17:40:36.2559657Z 
2020-02-25T17:40:36.2562927Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-25T17:40:36.2580993Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69464/merge to s
2020-02-25T17:40:36.2584009Z Task         : Get sources
2020-02-25T17:40:36.2584300Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-25T17:40:36.2584589Z Version      : 1.0.0
2020-02-25T17:40:36.2584794Z Author       : Microsoft
---
2020-02-25T17:40:37.2397094Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-25T17:40:37.2405179Z ##[command]git config gc.auto 0
2020-02-25T17:40:37.2408566Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-25T17:40:37.2411601Z ##[command]git config --get-all http.proxy
2020-02-25T17:40:37.2416992Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69464/merge:refs/remotes/pull/69464/merge
---
2020-02-25T17:43:20.4820524Z 
2020-02-25T17:43:20.5207578Z ###########                                                               16.5%
2020-02-25T17:43:20.5209059Z ######################################################################## 100.0%
2020-02-25T17:43:20.7890262Z extracting /checkout/obj/build/cache/2020-01-31/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.gz
2020-02-25T17:43:20.8567733Z     Updating git repository `https://github.com/Marwes/ena`
---
2020-02-25T17:46:17.8315837Z     Checking atty v0.2.11
2020-02-25T17:46:17.9278391Z    Compiling memoffset v0.5.1
2020-02-25T17:46:18.0422197Z    Compiling parking_lot_core v0.6.2
2020-02-25T17:46:19.3472068Z    Compiling parking_lot v0.9.0
2020-02-25T17:46:19.4122589Z     Checking ena v0.13.1 (https://github.com/Marwes/ena?branch=detach_undo_log#9c416dc2)
2020-02-25T17:46:19.8960300Z     Checking rustc_apfloat v0.0.0 (/checkout/src/librustc_apfloat)
2020-02-25T17:46:20.4993175Z     Checking rustc-hash v1.0.1
2020-02-25T17:46:20.5653150Z    Compiling quote v1.0.2
2020-02-25T17:46:20.6999987Z     Checking flate2 v1.0.12
---
2020-02-25T17:49:20.9913280Z configure: build.locked-deps    := True
2020-02-25T17:49:20.9913555Z configure: llvm.ccache          := sccache
2020-02-25T17:49:20.9913999Z configure: build.cargo-native-static := True
2020-02-25T17:49:20.9914448Z configure: dist.missing-tools   := True
2020-02-25T17:49:20.9914998Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-02-25T17:49:20.9915526Z configure: writing `config.toml` in current directory
2020-02-25T17:49:20.9915752Z configure: 
2020-02-25T17:49:20.9916122Z configure: run `python /checkout/x.py --help`
2020-02-25T17:49:20.9916349Z configure: 
---
2020-02-25T17:52:15.3830628Z    Compiling rustc_version v0.2.3
2020-02-25T17:52:16.3055560Z     Checking crossbeam-queue v0.1.2
2020-02-25T17:52:17.6917816Z     Checking rustc_index v0.0.0 (/checkout/src/librustc_index)
2020-02-25T17:52:23.6593035Z     Checking num_cpus v1.10.1
2020-02-25T17:52:23.7218237Z     Checking ena v0.13.1 (https://github.com/Marwes/ena?branch=detach_undo_log#9c416dc2)
2020-02-25T17:52:24.2075610Z     Checking rustc-hash v1.0.1
2020-02-25T17:52:24.2743063Z    Compiling quote v1.0.2
2020-02-25T17:52:27.9500925Z    Compiling memoffset v0.5.1
2020-02-25T17:52:27.9504452Z    Compiling parking_lot_core v0.6.2
---
2020-02-25T17:53:59.9732196Z     Checking rustc_infer v0.0.0 (/checkout/src/librustc_infer)
2020-02-25T17:53:59.9733423Z error[E0433]: failed to resolve: maybe a missing crate `rustc_data_structures`?
2020-02-25T17:53:59.9810676Z   --> src/librustc_infer/infer/outlives/obligations.rs:67:12
2020-02-25T17:53:59.9811212Z    |
2020-02-25T17:53:59.9812135Z 67 | use crate::rustc_data_structures::undo_log::UndoLogs;
2020-02-25T17:53:59.9813035Z    |            ^^^^^^^^^^^^^^^^^^^^^ maybe a missing crate `rustc_data_structures`?
2020-02-25T17:53:59.9813439Z 
2020-02-25T17:54:01.4709189Z error[E0599]: no method named `push` found for struct `infer::undo_log::InferCtxtUndoLogs<'tcx>` in the current scope
2020-02-25T17:54:01.4710051Z    --> src/librustc_infer/infer/outlives/obligations.rs:91:24
2020-02-25T17:54:01.4710554Z     |
2020-02-25T17:54:01.4711227Z 91  |         inner.undo_log.push(UndoLog::PushRegionObligation);
2020-02-25T17:54:01.4712616Z     |                        ^^^^ method not found in `infer::undo_log::InferCtxtUndoLogs<'tcx>`
2020-02-25T17:54:01.4713351Z     | 
2020-02-25T17:54:01.4713887Z    ::: src/librustc_infer/infer/undo_log.rs:128:1
2020-02-25T17:54:01.4714369Z     |
2020-02-25T17:54:01.4714955Z 128 | pub(crate) struct InferCtxtUndoLogs<'tcx> {
2020-02-25T17:54:01.4715768Z     | ----------------------------------------- method `push` not found for this
2020-02-25T17:54:01.4716973Z     = help: items from traits can only be used if the trait is in scope
2020-02-25T17:54:01.4717834Z     = note: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-02-25T17:54:01.4717834Z     = note: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-02-25T17:54:01.4718521Z             `use ena::undo_log::UndoLogs;`
2020-02-25T17:54:02.3540739Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
2020-02-25T17:54:02.7542526Z error: aborting due to 2 previous errors
2020-02-25T17:54:02.7542860Z 
2020-02-25T17:54:02.7548374Z Some errors have detailed explanations: E0433, E0599.
2020-02-25T17:54:02.7548374Z Some errors have detailed explanations: E0433, E0599.
2020-02-25T17:54:02.7554724Z For more information about an error, try `rustc --explain E0433`.
2020-02-25T17:54:02.7609738Z error: could not compile `rustc_infer`.
2020-02-25T17:54:02.7610362Z warning: build failed, waiting for other jobs to finish...
2020-02-25T17:54:03.4587925Z error: build failed
2020-02-25T17:54:03.4607247Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-02-25T17:54:03.4617829Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-02-25T17:54:03.4618320Z Build completed unsuccessfully in 0:02:53
2020-02-25T17:54:03.4673480Z == clock drift check ==
2020-02-25T17:54:03.4687472Z   local time: Tue Feb 25 17:54:03 UTC 2020
2020-02-25T17:54:03.4687472Z   local time: Tue Feb 25 17:54:03 UTC 2020
2020-02-25T17:54:03.7592069Z   network time: Tue, 25 Feb 2020 17:54:03 GMT
2020-02-25T17:54:03.7600667Z == end clock drift check ==
2020-02-25T17:54:04.3928048Z 
2020-02-25T17:54:04.3993522Z ##[error]Bash exited with code '1'.
2020-02-25T17:54:04.4029187Z ##[section]Finishing: Run build
2020-02-25T17:54:04.4069209Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69464/merge to s
2020-02-25T17:54:04.4073938Z Task         : Get sources
2020-02-25T17:54:04.4074277Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-25T17:54:04.4074577Z Version      : 1.0.0
2020-02-25T17:54:04.4074787Z Author       : Microsoft
2020-02-25T17:54:04.4074787Z Author       : Microsoft
2020-02-25T17:54:04.4075130Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-25T17:54:04.4075513Z ==============================================================================
2020-02-25T17:54:04.7362762Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-25T17:54:04.7411011Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69464/merge to s
2020-02-25T17:54:04.7509728Z Cleaning up task key
2020-02-25T17:54:04.7511020Z Start cleaning up orphan processes.
2020-02-25T17:54:04.7685006Z Terminate orphan process: pid (4911) (python)
2020-02-25T17:54:04.7896581Z ##[section]Finishing: Finalize Job
