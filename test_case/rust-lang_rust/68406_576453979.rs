plain
2020-01-20T22:40:59.6692169Z ========================== Starting Command Output ===========================
2020-01-20T22:40:59.6693998Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/945868a9-d5be-43f2-b714-0dcc3a51f4fa.sh
2020-01-20T22:40:59.6694039Z 
2020-01-20T22:40:59.6696958Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-20T22:40:59.6702846Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68406/merge to s
2020-01-20T22:40:59.6704582Z Task         : Get sources
2020-01-20T22:40:59.6704618Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-20T22:40:59.6704660Z Version      : 1.0.0
2020-01-20T22:40:59.6704694Z Author       : Microsoft
---
2020-01-20T22:41:00.5264651Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-20T22:41:00.5365114Z ##[command]git config gc.auto 0
2020-01-20T22:41:00.5449403Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-20T22:41:00.5499496Z ##[command]git config --get-all http.proxy
2020-01-20T22:41:00.5644214Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68406/merge:refs/remotes/pull/68406/merge
---
2020-01-20T22:52:04.8063216Z configure: build.locked-deps    := True
2020-01-20T22:52:04.8063271Z configure: llvm.ccache          := sccache
2020-01-20T22:52:04.8063523Z configure: build.cargo-native-static := True
2020-01-20T22:52:04.8063792Z configure: dist.missing-tools   := True
2020-01-20T22:52:04.8064259Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-01-20T22:52:04.8064545Z configure: writing `config.toml` in current directory
2020-01-20T22:52:04.8064583Z configure: 
2020-01-20T22:52:04.8064792Z configure: run `python /checkout/x.py --help`
2020-01-20T22:52:04.8064850Z configure: 
---
2020-01-20T22:55:39.6725305Z     Checking rustc_data_structures v0.0.0 (/checkout/src/librustc_data_structures)
2020-01-20T22:55:39.8384705Z error[E0432]: unresolved import `libc`
2020-01-20T22:55:39.8385810Z   --> src/librustc_data_structures/profiling.rs:98:5
2020-01-20T22:55:39.8386277Z    |
2020-01-20T22:55:39.8386712Z 98 | use libc::{c_char, c_void};
2020-01-20T22:55:39.8392751Z 
2020-01-20T22:55:40.7064145Z error: aborting due to previous error
2020-01-20T22:55:40.7065739Z 
2020-01-20T22:55:40.7076008Z For more information about this error, try `rustc --explain E0432`.
2020-01-20T22:55:40.7076008Z For more information about this error, try `rustc --explain E0432`.
2020-01-20T22:55:40.7136116Z error: could not compile `rustc_data_structures`.
2020-01-20T22:55:40.7156318Z warning: build failed, waiting for other jobs to finish...
2020-01-20T22:55:48.1607445Z error: build failed
2020-01-20T22:55:48.1626046Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-01-20T22:55:48.1642165Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-01-20T22:55:48.1642298Z Build completed unsuccessfully in 0:02:09
2020-01-20T22:55:48.1694305Z == clock drift check ==
2020-01-20T22:55:48.1712239Z   local time: Mon Jan 20 22:55:48 UTC 2020
2020-01-20T22:55:48.1712239Z   local time: Mon Jan 20 22:55:48 UTC 2020
2020-01-20T22:55:48.3343313Z   network time: Mon, 20 Jan 2020 22:55:48 GMT
2020-01-20T22:55:48.3343406Z == end clock drift check ==
2020-01-20T22:55:48.8685024Z 
2020-01-20T22:55:48.8795771Z ##[error]Bash exited with code '1'.
2020-01-20T22:55:48.8807564Z ##[section]Finishing: Run build
2020-01-20T22:55:48.8822573Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68406/merge to s
2020-01-20T22:55:48.8824365Z Task         : Get sources
2020-01-20T22:55:48.8824403Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-20T22:55:48.8824454Z Version      : 1.0.0
2020-01-20T22:55:48.8824487Z Author       : Microsoft
2020-01-20T22:55:48.8824487Z Author       : Microsoft
2020-01-20T22:55:48.8824522Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-20T22:55:48.8824583Z ==============================================================================
2020-01-20T22:55:49.2962744Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-20T22:55:49.3004171Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68406/merge to s
2020-01-20T22:55:49.3117148Z Cleaning up task key
2020-01-20T22:55:49.3118264Z Start cleaning up orphan processes.
2020-01-20T22:55:49.3224053Z Terminate orphan process: pid (3828) (python)
2020-01-20T22:55:49.3430099Z ##[section]Finishing: Finalize Job
