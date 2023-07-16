plain
2020-01-08T08:32:03.4336844Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-08T08:32:03.4439409Z ##[command]git config gc.auto 0
2020-01-08T08:32:03.4501733Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-08T08:32:03.4559485Z ##[command]git config --get-all http.proxy
2020-01-08T08:32:03.4686107Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67953/merge:refs/remotes/pull/67953/merge
---
2020-01-08T08:40:56.8844388Z configure: build.locked-deps    := True
2020-01-08T08:40:56.8844452Z configure: llvm.ccache          := sccache
2020-01-08T08:40:56.8844650Z configure: build.cargo-native-static := True
2020-01-08T08:40:56.8844841Z configure: dist.missing-tools   := True
2020-01-08T08:40:56.8845092Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-01-08T08:40:56.8845187Z configure: writing `config.toml` in current directory
2020-01-08T08:40:56.8845245Z configure: 
2020-01-08T08:40:56.8845468Z configure: run `python /checkout/x.py --help`
2020-01-08T08:40:56.8845513Z configure: 
---
2020-01-08T08:45:10.2078966Z 
2020-01-08T08:45:10.2134805Z error: could not compile `rustc_infer`.
2020-01-08T08:45:10.2135110Z warning: build failed, waiting for other jobs to finish...
2020-01-08T08:45:11.4125358Z error: build failed
2020-01-08T08:45:11.4148725Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-01-08T08:45:11.4191284Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-01-08T08:45:11.4191431Z Build completed unsuccessfully in 0:02:47
2020-01-08T08:45:11.4242627Z == clock drift check ==
2020-01-08T08:45:11.4242707Z   local time: Wed Jan  8 08:45:11 UTC 2020
2020-01-08T08:45:11.4242707Z   local time: Wed Jan  8 08:45:11 UTC 2020
2020-01-08T08:45:11.5723667Z   network time: Wed, 08 Jan 2020 08:45:11 GMT
2020-01-08T08:45:11.5729432Z == end clock drift check ==
2020-01-08T08:45:12.2410426Z 
2020-01-08T08:45:12.2507829Z ##[error]Bash exited with code '1'.
2020-01-08T08:45:12.2677653Z ##[section]Starting: Checkout
2020-01-08T08:45:12.2680288Z ==============================================================================
2020-01-08T08:45:12.2680567Z Task         : Get sources
2020-01-08T08:45:12.2680613Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
