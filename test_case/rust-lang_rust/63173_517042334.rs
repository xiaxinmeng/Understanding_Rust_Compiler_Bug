plain
2019-07-31T20:06:56.5672458Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-31T20:06:57.4620774Z ##[command]git config gc.auto 0
2019-07-31T20:06:57.4624230Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-31T20:06:57.4626070Z ##[command]git config --get-all http.proxy
2019-07-31T20:06:57.4629729Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63173/merge:refs/remotes/pull/63173/merge
---
2019-07-31T20:07:31.6978927Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-31T20:07:31.6978953Z 
2019-07-31T20:07:31.6979122Z   git checkout -b <new-branch-name>
2019-07-31T20:07:31.6979145Z 
2019-07-31T20:07:31.6979183Z HEAD is now at 7cd479b37 Merge 5c8eba2e31c5cb14b977b080628dbb02c5009852 into e3976fff44e6ce14c2f92252e6a807800b9aa7c0
2019-07-31T20:07:31.7123970Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-31T20:07:31.7126483Z ==============================================================================
2019-07-31T20:07:31.7126529Z Task         : Bash
2019-07-31T20:07:31.7126579Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-31T20:16:46.9275607Z configure: build.locked-deps    := True
2019-07-31T20:16:46.9275676Z configure: llvm.ccache          := sccache
2019-07-31T20:16:46.9275894Z configure: build.cargo-native-static := True
2019-07-31T20:16:46.9276108Z configure: dist.missing-tools   := True
2019-07-31T20:16:46.9276556Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2019-07-31T20:16:46.9276704Z configure: writing `config.toml` in current directory
2019-07-31T20:16:46.9276753Z configure: 
2019-07-31T20:16:46.9277084Z configure: run `python /checkout/x.py --help`
2019-07-31T20:16:46.9277135Z configure: 
---
2019-07-31T20:18:36.0218719Z 87 | #![feature(const_generics)]
2019-07-31T20:18:36.0223457Z    |            ^^^^^^^^^^^^^^
2019-07-31T20:18:36.0226952Z 
2019-07-31T20:18:36.3576562Z     Checking panic_abort v0.0.0 (/checkout/src/libpanic_abort)
2019-07-31T20:18:36.4902365Z error[E0658]: kind="static-nobundle" is feature gated
2019-07-31T20:18:36.4909335Z   |
2019-07-31T20:18:36.4913723Z   = note: for more information, see ***/issues/37403
2019-07-31T20:18:36.4919604Z   = help: add #![feature(static_nobundle)] to the crate attributes to enable
2019-07-31T20:18:36.4981682Z error: aborting due to previous error
2019-07-31T20:18:36.4981759Z 
2019-07-31T20:18:36.4987006Z For more information about this error, try `rustc --explain E0658`.
2019-07-31T20:18:36.5022417Z error: Could not compile `unwind`.
2019-07-31T20:18:36.5022417Z error: Could not compile `unwind`.
2019-07-31T20:18:36.5034501Z warning: build failed, waiting for other jobs to finish...
2019-07-31T20:18:38.2352730Z error: build failed
2019-07-31T20:18:38.2370543Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
2019-07-31T20:18:38.2380657Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2019-07-31T20:18:38.2381070Z Build completed unsuccessfully in 0:00:26
2019-07-31T20:18:38.2381070Z Build completed unsuccessfully in 0:00:26
2019-07-31T20:18:44.5642891Z ##[error]Bash exited with code '1'.
2019-07-31T20:18:44.5674451Z ##[section]Starting: Checkout
2019-07-31T20:18:44.5676125Z ==============================================================================
2019-07-31T20:18:44.5676182Z Task         : Get sources
2019-07-31T20:18:44.5676250Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
