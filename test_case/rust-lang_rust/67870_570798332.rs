plain
2020-01-04T16:05:59.2123058Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-04T16:05:59.2329330Z ##[command]git config gc.auto 0
2020-01-04T16:05:59.2403952Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-04T16:05:59.2464382Z ##[command]git config --get-all http.proxy
2020-01-04T16:05:59.2618164Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67870/merge:refs/remotes/pull/67870/merge
---
2020-01-04T16:18:16.6414613Z configure: build.locked-deps    := True
2020-01-04T16:18:16.6414675Z configure: llvm.ccache          := sccache
2020-01-04T16:18:16.6422006Z configure: build.cargo-native-static := True
2020-01-04T16:18:16.6422243Z configure: dist.missing-tools   := True
2020-01-04T16:18:16.6422480Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-01-04T16:18:16.6422600Z configure: writing `config.toml` in current directory
2020-01-04T16:18:16.6422660Z configure: 
2020-01-04T16:18:16.6422891Z configure: run `python /checkout/x.py --help`
2020-01-04T16:18:16.6422932Z configure: 
---
2020-01-04T16:22:00.1727267Z     |
2020-01-04T16:22:00.1730297Z 162 |             parallel!({
2020-01-04T16:22:00.1731043Z     |             ^^^^^^^^
2020-01-04T16:22:00.1734472Z     |
2020-01-04T16:22:00.1736445Z     = help: have you added the `#[macro_use]` on the module/import?
2020-01-04T16:22:01.0278136Z error: aborting due to previous error
2020-01-04T16:22:01.0278231Z 
2020-01-04T16:22:01.0348972Z error: could not compile `rustc_data_structures`.
2020-01-04T16:22:01.0395430Z warning: build failed, waiting for other jobs to finish...
2020-01-04T16:22:01.0395430Z warning: build failed, waiting for other jobs to finish...
2020-01-04T16:22:09.6055003Z error: build failed
2020-01-04T16:22:09.6085651Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-01-04T16:22:09.6102867Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-01-04T16:22:09.6102953Z Build completed unsuccessfully in 0:02:14
2020-01-04T16:22:09.6160836Z == clock drift check ==
2020-01-04T16:22:09.6177416Z   local time: Sat Jan  4 16:22:09 UTC 2020
2020-01-04T16:22:09.6177416Z   local time: Sat Jan  4 16:22:09 UTC 2020
2020-01-04T16:22:09.8960569Z   network time: Sat, 04 Jan 2020 16:22:09 GMT
2020-01-04T16:22:09.8960657Z == end clock drift check ==
2020-01-04T16:22:11.0196594Z 
2020-01-04T16:22:11.0308663Z ##[error]Bash exited with code '1'.
2020-01-04T16:22:11.0347628Z ##[section]Starting: Checkout
2020-01-04T16:22:11.0350300Z ==============================================================================
2020-01-04T16:22:11.0350506Z Task         : Get sources
2020-01-04T16:22:11.0350554Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
