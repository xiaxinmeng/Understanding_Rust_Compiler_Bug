plain
2020-01-14T16:30:30.1302956Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-14T16:30:30.1383090Z ##[command]git config gc.auto 0
2020-01-14T16:30:30.1464795Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-14T16:30:30.1513208Z ##[command]git config --get-all http.proxy
2020-01-14T16:30:30.1665277Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68218/merge:refs/remotes/pull/68218/merge
---
2020-01-14T16:43:02.6439585Z configure: build.locked-deps    := True
2020-01-14T16:43:02.6439819Z configure: llvm.ccache          := sccache
2020-01-14T16:43:02.6440198Z configure: build.cargo-native-static := True
2020-01-14T16:43:02.6440567Z configure: dist.missing-tools   := True
2020-01-14T16:43:02.6441011Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-01-14T16:43:02.6441369Z configure: writing `config.toml` in current directory
2020-01-14T16:43:02.6441558Z configure: 
2020-01-14T16:43:02.6442096Z configure: run `python /checkout/x.py --help`
2020-01-14T16:43:02.6442237Z configure: 
---
2020-01-14T16:46:23.6541842Z     Checking rustc_data_structures v0.0.0 (/checkout/src/librustc_data_structures)
2020-01-14T16:46:24.3989587Z error[E0282]: type annotations needed
2020-01-14T16:46:24.3989959Z   --> src/librustc_data_structures/sync/future.rs:42:9
2020-01-14T16:46:24.3990186Z    |
2020-01-14T16:46:24.3990526Z 42 |         sync::spawn(move || run(&data));
2020-01-14T16:46:24.3991006Z    |         ^^^^^^^^^^^ cannot infer type for type parameter `F`
2020-01-14T16:46:24.5185488Z error: aborting due to previous error
2020-01-14T16:46:24.5189404Z 
2020-01-14T16:46:24.5197924Z For more information about this error, try `rustc --explain E0282`.
2020-01-14T16:46:24.5256669Z error: could not compile `rustc_data_structures`.
2020-01-14T16:46:24.5256669Z error: could not compile `rustc_data_structures`.
2020-01-14T16:46:24.5278931Z warning: build failed, waiting for other jobs to finish...
2020-01-14T16:46:31.2596168Z error: build failed
2020-01-14T16:46:31.2628467Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-01-14T16:46:31.2640918Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-01-14T16:46:31.2641347Z Build completed unsuccessfully in 0:01:57
2020-01-14T16:46:31.2689181Z == clock drift check ==
2020-01-14T16:46:31.2706973Z   local time: Tue Jan 14 16:46:31 UTC 2020
2020-01-14T16:46:31.2706973Z   local time: Tue Jan 14 16:46:31 UTC 2020
2020-01-14T16:46:31.3664927Z   network time: Tue, 14 Jan 2020 16:46:31 GMT
2020-01-14T16:46:31.3667613Z == end clock drift check ==
2020-01-14T16:46:31.9019990Z 
2020-01-14T16:46:31.9075853Z ##[error]Bash exited with code '1'.
2020-01-14T16:46:31.9116084Z ##[section]Starting: Checkout
2020-01-14T16:46:31.9118328Z ==============================================================================
2020-01-14T16:46:31.9118383Z Task         : Get sources
2020-01-14T16:46:31.9118451Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
