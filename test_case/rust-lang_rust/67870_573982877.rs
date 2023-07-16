plain
2020-01-14T03:14:00.4336300Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-14T03:14:00.4428214Z ##[command]git config gc.auto 0
2020-01-14T03:14:00.4491956Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-14T03:14:00.4546459Z ##[command]git config --get-all http.proxy
2020-01-14T03:14:00.4677836Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67870/merge:refs/remotes/pull/67870/merge
---
2020-01-14T03:25:02.8191957Z configure: build.locked-deps    := True
2020-01-14T03:25:02.8192120Z configure: llvm.ccache          := sccache
2020-01-14T03:25:02.8192404Z configure: build.cargo-native-static := True
2020-01-14T03:25:02.8192728Z configure: dist.missing-tools   := True
2020-01-14T03:25:02.8193080Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-01-14T03:25:02.8193845Z configure: writing `config.toml` in current directory
2020-01-14T03:25:02.8194002Z configure: 
2020-01-14T03:25:02.8194417Z configure: run `python /checkout/x.py --help`
2020-01-14T03:25:02.8194608Z configure: 
---
2020-01-14T03:28:28.4886223Z     |
2020-01-14T03:28:28.4886451Z 180 |             parallel!({
2020-01-14T03:28:28.4886666Z     |             ^^^^^^^^
2020-01-14T03:28:28.4886835Z     |
2020-01-14T03:28:28.4887085Z     = help: have you added the `#[macro_use]` on the module/import?
2020-01-14T03:28:29.3633428Z error: aborting due to previous error
2020-01-14T03:28:29.3634717Z 
2020-01-14T03:28:29.3711086Z error: could not compile `rustc_data_structures`.
2020-01-14T03:28:29.3729000Z warning: build failed, waiting for other jobs to finish...
2020-01-14T03:28:29.3729000Z warning: build failed, waiting for other jobs to finish...
2020-01-14T03:28:35.9445901Z error: build failed
2020-01-14T03:28:35.9472501Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-01-14T03:28:35.9486506Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-01-14T03:28:35.9486604Z Build completed unsuccessfully in 0:02:04
2020-01-14T03:28:35.9539826Z == clock drift check ==
2020-01-14T03:28:35.9555971Z   local time: Tue Jan 14 03:28:35 UTC 2020
2020-01-14T03:28:35.9555971Z   local time: Tue Jan 14 03:28:35 UTC 2020
2020-01-14T03:28:36.0400192Z   network time: Tue, 14 Jan 2020 03:28:36 GMT
2020-01-14T03:28:36.0404005Z == end clock drift check ==
2020-01-14T03:28:36.5478306Z 
2020-01-14T03:28:36.5565165Z ##[error]Bash exited with code '1'.
2020-01-14T03:28:36.5747110Z ##[section]Starting: Checkout
2020-01-14T03:28:36.5748599Z ==============================================================================
2020-01-14T03:28:36.5748645Z Task         : Get sources
2020-01-14T03:28:36.5748699Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
