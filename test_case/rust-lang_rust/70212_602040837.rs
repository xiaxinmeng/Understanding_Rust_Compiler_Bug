plain
2020-03-21T09:54:20.6018737Z ========================== Starting Command Output ===========================
2020-03-21T09:54:20.6022165Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/176a3c32-c12c-4d8b-8f02-5b4759aaaba5.sh
2020-03-21T09:54:20.6022450Z 
2020-03-21T09:54:20.6027025Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-21T09:54:20.6046742Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70212/merge to s
2020-03-21T09:54:20.6050141Z Task         : Get sources
2020-03-21T09:54:20.6050460Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-21T09:54:20.6050767Z Version      : 1.0.0
2020-03-21T09:54:20.6050976Z Author       : Microsoft
---
2020-03-21T09:54:21.5960651Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-21T09:54:21.5966147Z ##[command]git config gc.auto 0
2020-03-21T09:54:21.5969768Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-21T09:54:21.5973084Z ##[command]git config --get-all http.proxy
2020-03-21T09:54:21.5978804Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70212/merge:refs/remotes/pull/70212/merge
---
2020-03-21T09:57:29.3341873Z configure: build.locked-deps    := True
2020-03-21T09:57:29.3342171Z configure: llvm.ccache          := sccache
2020-03-21T09:57:29.3342666Z configure: build.cargo-native-static := True
2020-03-21T09:57:29.3343139Z configure: dist.missing-tools   := True
2020-03-21T09:57:29.3343731Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-03-21T09:57:29.3344308Z configure: writing `config.toml` in current directory
2020-03-21T09:57:29.3344548Z configure: 
2020-03-21T09:57:29.3344954Z configure: run `python /checkout/x.py --help`
2020-03-21T09:57:29.3345196Z configure: 
---
2020-03-21T11:20:32.7037006Z    Compiling backtrace v0.3.44
2020-03-21T11:20:33.0920680Z    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
2020-03-21T11:20:34.1493718Z warning: dropping unsupported crate type `dylib` for target `wasm32-unknown-emscripten`
2020-03-21T11:20:34.1497666Z 
2020-03-21T11:20:40.3005566Z error: internal compiler error: src/librustc_codegen_llvm/intrinsic.rs:1110: eh_catch_typeinfo not defined, but needed for emcc unwinding
2020-03-21T11:20:40.3007395Z thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:880:9
2020-03-21T11:20:40.3008047Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-21T11:20:40.3008465Z 
2020-03-21T11:20:40.3008889Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-21T11:20:40.3008889Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-21T11:20:40.3009263Z 
2020-03-21T11:20:40.3010264Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-21T11:20:40.3011495Z note: rustc 1.44.0-nightly (0080865a2 2020-03-21) running on x86_64-unknown-linux-gnu
2020-03-21T11:20:40.3011965Z 
2020-03-21T11:20:40.3011965Z 
2020-03-21T11:20:40.3013398Z note: compiler flags: -Z macro-backtrace -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C prefer-dynamic -C opt-level=3 -C codegen-units=1 -C debuginfo=0 -C prefer-dynamic -C debug-assertions=y --crate-type dylib --crate-type rlib
2020-03-21T11:20:40.3014635Z note: some of the compiler flags provided by cargo are hidden
2020-03-21T11:20:40.3015018Z 
2020-03-21T11:20:40.3730561Z error: aborting due to previous error
2020-03-21T11:20:41.1975695Z 
---
2020-03-21T11:20:41.1986669Z   local time: Sat Mar 21 11:20:40 UTC 2020
2020-03-21T11:20:41.1987528Z   network time: Sat, 21 Mar 2020 11:20:40 GMT
2020-03-21T11:20:41.1988065Z == end clock drift check ==
2020-03-21T11:20:42.0580718Z 
2020-03-21T11:20:42.0642439Z ##[error]Bash exited with code '1'.
2020-03-21T11:20:42.0658608Z ##[section]Finishing: Run build
2020-03-21T11:20:42.0729762Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70212/merge to s
2020-03-21T11:20:42.0734839Z Task         : Get sources
2020-03-21T11:20:42.0735167Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-21T11:20:42.0735480Z Version      : 1.0.0
2020-03-21T11:20:42.0735691Z Author       : Microsoft
2020-03-21T11:20:42.0735691Z Author       : Microsoft
2020-03-21T11:20:42.0736034Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-21T11:20:42.0736437Z ==============================================================================
2020-03-21T11:20:42.3890526Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-21T11:20:42.3933905Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70212/merge to s
2020-03-21T11:20:42.4023662Z Cleaning up task key
2020-03-21T11:20:42.4024890Z Start cleaning up orphan processes.
2020-03-21T11:20:42.4199915Z Terminate orphan process: pid (3932) (python)
2020-03-21T11:20:42.4414069Z ##[section]Finishing: Finalize Job
