plain
2020-01-07T21:28:13.0465822Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-07T21:28:13.0549482Z ##[command]git config gc.auto 0
2020-01-07T21:28:13.0627284Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-07T21:28:13.0686062Z ##[command]git config --get-all http.proxy
2020-01-07T21:28:13.0831017Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67953/merge:refs/remotes/pull/67953/merge
---
2020-01-07T21:37:48.4269745Z configure: build.locked-deps    := True
2020-01-07T21:37:48.4269792Z configure: llvm.ccache          := sccache
2020-01-07T21:37:48.4270006Z configure: build.cargo-native-static := True
2020-01-07T21:37:48.4270230Z configure: dist.missing-tools   := True
2020-01-07T21:37:48.4270496Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-01-07T21:37:48.4270607Z configure: writing `config.toml` in current directory
2020-01-07T21:37:48.4270650Z configure: 
2020-01-07T21:37:48.4270867Z configure: run `python /checkout/x.py --help`
2020-01-07T21:37:48.4271129Z configure: 
---
2020-01-07T21:42:26.8802246Z 
2020-01-07T21:42:26.8868390Z error: could not compile `rustc_infer`.
2020-01-07T21:42:26.8868804Z warning: build failed, waiting for other jobs to finish...
2020-01-07T21:42:28.1497172Z error: build failed
2020-01-07T21:42:28.1593299Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-01-07T21:42:28.1594301Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-01-07T21:42:28.1594392Z Build completed unsuccessfully in 0:03:08
2020-01-07T21:42:28.1632261Z == clock drift check ==
2020-01-07T21:42:28.1644394Z   local time: Tue Jan  7 21:42:28 UTC 2020
2020-01-07T21:42:28.1644394Z   local time: Tue Jan  7 21:42:28 UTC 2020
2020-01-07T21:42:28.3257950Z   network time: Tue, 07 Jan 2020 21:42:28 GMT
2020-01-07T21:42:28.3259041Z == end clock drift check ==
2020-01-07T21:42:29.0788166Z 
2020-01-07T21:42:29.0896282Z ##[error]Bash exited with code '1'.
2020-01-07T21:42:29.0930116Z ##[section]Starting: Checkout
2020-01-07T21:42:29.0932238Z ==============================================================================
2020-01-07T21:42:29.0932295Z Task         : Get sources
2020-01-07T21:42:29.0932362Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
