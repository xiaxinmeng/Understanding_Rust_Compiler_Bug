plain
2020-02-26T22:47:07.0679038Z ========================== Starting Command Output ===========================
2020-02-26T22:47:07.0693332Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/9821ae43-4cb9-4c21-9bfa-ecf11b6f27a6.sh
2020-02-26T22:47:07.4046923Z 
2020-02-26T22:47:07.4120011Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-26T22:47:07.4147127Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69497/merge to s
2020-02-26T22:47:07.4157174Z Task         : Get sources
2020-02-26T22:47:07.4157549Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-26T22:47:07.4158242Z Version      : 1.0.0
2020-02-26T22:47:07.4158546Z Author       : Microsoft
---
2020-02-26T22:47:10.3387373Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-26T22:47:10.3497409Z ##[command]git config gc.auto 0
2020-02-26T22:47:10.3529243Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-26T22:47:10.3555238Z ##[command]git config --get-all http.proxy
2020-02-26T22:47:10.3658400Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69497/merge:refs/remotes/pull/69497/merge
---
2020-02-26T22:55:09.6255575Z     Checking rustc_expand v0.0.0 (/checkout/src/librustc_expand)
2020-02-26T22:55:09.8317969Z error: unused import: `FatalError`
2020-02-26T22:55:09.8318529Z   --> src/librustc_expand/expand.rs:11:35
2020-02-26T22:55:09.8318942Z    |
2020-02-26T22:55:09.8319978Z 11 | use rustc_errors::{Applicability, FatalError, PResult};
2020-02-26T22:55:09.8321013Z    |
2020-02-26T22:55:09.8321503Z    = note: `-D unused-imports` implied by `-D warnings`
2020-02-26T22:55:09.8321733Z 
2020-02-26T22:55:10.1637405Z error[E0061]: this function takes 2 parameters but 1 parameter was supplied
2020-02-26T22:55:10.1637405Z error[E0061]: this function takes 2 parameters but 1 parameter was supplied
2020-02-26T22:55:10.1638183Z    --> src/librustc_expand/expand.rs:642:20
2020-02-26T22:55:10.1638666Z     |
2020-02-26T22:55:10.1639163Z 208 |     fn dummy(self, span: Span) -> AstFragment {
2020-02-26T22:55:10.1640295Z ...
2020-02-26T22:55:10.1640295Z ...
2020-02-26T22:55:10.1640792Z 642 |             return AstFragmentKind::dummy(invoc.span());
2020-02-26T22:55:10.1645043Z 
2020-02-26T22:55:10.4209392Z error: aborting due to 2 previous errors
2020-02-26T22:55:10.4212811Z 
2020-02-26T22:55:10.4221313Z For more information about this error, try `rustc --explain E0061`.
2020-02-26T22:55:10.4221313Z For more information about this error, try `rustc --explain E0061`.
2020-02-26T22:55:10.4274193Z error: could not compile `rustc_expand`.
2020-02-26T22:55:10.4293309Z warning: build failed, waiting for other jobs to finish...
2020-02-26T22:55:31.3788991Z error: build failed
2020-02-26T22:55:31.3813467Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-02-26T22:55:31.3824929Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-02-26T22:55:31.3825239Z Build completed unsuccessfully in 0:04:51
2020-02-26T22:55:31.3872685Z == clock drift check ==
2020-02-26T22:55:31.3885474Z   local time: Wed Feb 26 22:55:31 UTC 2020
2020-02-26T22:55:31.3885474Z   local time: Wed Feb 26 22:55:31 UTC 2020
2020-02-26T22:55:31.6524363Z   network time: Wed, 26 Feb 2020 22:55:31 GMT
2020-02-26T22:55:31.6527777Z == end clock drift check ==
2020-02-26T22:55:32.4189161Z 
2020-02-26T22:55:32.4255585Z ##[error]Bash exited with code '1'.
2020-02-26T22:55:32.4265658Z ##[section]Finishing: Run build
2020-02-26T22:55:32.4302786Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69497/merge to s
2020-02-26T22:55:32.4308197Z Task         : Get sources
2020-02-26T22:55:32.4308497Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-26T22:55:32.4308738Z Version      : 1.0.0
2020-02-26T22:55:32.4308906Z Author       : Microsoft
2020-02-26T22:55:32.4308906Z Author       : Microsoft
2020-02-26T22:55:32.4309195Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-26T22:55:32.4309507Z ==============================================================================
2020-02-26T22:55:32.7117798Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-26T22:55:32.7158490Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69497/merge to s
2020-02-26T22:55:32.7229077Z Cleaning up task key
2020-02-26T22:55:32.7230085Z Start cleaning up orphan processes.
2020-02-26T22:55:32.7480218Z Terminate orphan process: pid (23776) (python)
2020-02-26T22:55:32.7519133Z ##[section]Finishing: Finalize Job
