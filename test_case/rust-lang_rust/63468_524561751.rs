plain
2019-08-24T15:28:17.8878693Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-24T15:28:17.9066043Z ##[command]git config gc.auto 0
2019-08-24T15:28:17.9135271Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-24T15:28:17.9189166Z ##[command]git config --get-all http.proxy
2019-08-24T15:28:17.9318784Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63468/merge:refs/remotes/pull/63468/merge
---
2019-08-24T15:28:51.8642760Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-24T15:28:51.8642792Z 
2019-08-24T15:28:51.8643151Z   git checkout -b <new-branch-name>
2019-08-24T15:28:51.8643194Z 
2019-08-24T15:28:51.8643234Z HEAD is now at 590f8155e Merge 23d500a2f21a7db8e387e3d147eb6709c8d9cabe into 478464570e60523adc6d303577d1782229ca1f93
2019-08-24T15:28:51.8785407Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-24T15:28:51.8787853Z ==============================================================================
2019-08-24T15:28:51.8787899Z Task         : Bash
2019-08-24T15:28:51.8787950Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-24T16:00:19.6744840Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2019-08-24T16:00:36.4983748Z error: cannot find attribute macro `stable_hasher` in this scope
2019-08-24T16:00:36.4984810Z    --> src/librustc/hir/mod.rs:347:7
2019-08-24T16:00:36.4985367Z     |
2019-08-24T16:00:36.4985847Z 347 |     #[stable_hasher(project(name))]
2019-08-24T16:00:36.4988780Z 
2019-08-24T16:00:36.4993567Z error: cannot find attribute macro `stable_hasher` in this scope
2019-08-24T16:00:36.4994122Z    --> src/librustc/hir/mod.rs:858:7
2019-08-24T16:00:36.4994553Z     |
2019-08-24T16:00:36.4994553Z     |
2019-08-24T16:00:36.4995339Z 858 |     #[stable_hasher(ignore)]
2019-08-24T16:00:36.4998111Z 
2019-08-24T16:00:36.5001909Z error: cannot find attribute macro `stable_hasher` in this scope
2019-08-24T16:00:36.5002585Z    --> src/librustc/hir/mod.rs:871:7
2019-08-24T16:00:36.5003542Z     |
2019-08-24T16:00:36.5003542Z     |
2019-08-24T16:00:36.5004181Z 871 |     #[stable_hasher(ignore)]
2019-08-24T16:00:36.5008498Z 
2019-08-24T16:00:36.5013144Z error: cannot find attribute macro `stable_hasher` in this scope
2019-08-24T16:00:36.5075990Z    --> src/librustc/hir/mod.rs:935:7
2019-08-24T16:00:36.5076252Z     |
2019-08-24T16:00:36.5076252Z     |
2019-08-24T16:00:36.5076498Z 935 |     #[stable_hasher(ignore)]
2019-08-24T16:00:36.5076810Z 
2019-08-24T16:00:36.5077054Z error: cannot find attribute macro `stable_hasher` in this scope
2019-08-24T16:00:36.5077260Z    --> src/librustc/hir/mod.rs:938:7
2019-08-24T16:00:36.5077428Z     |
2019-08-24T16:00:36.5077428Z     |
2019-08-24T16:00:36.5077661Z 938 |     #[stable_hasher(project(name))]
2019-08-24T16:00:36.5077903Z 
2019-08-24T16:00:36.5078106Z error: cannot find attribute macro `stable_hasher` in this scope
2019-08-24T16:00:36.5078340Z     --> src/librustc/hir/mod.rs:1266:7
2019-08-24T16:00:36.5078509Z      |
2019-08-24T16:00:36.5078509Z      |
2019-08-24T16:00:36.5078889Z 1266 |     #[stable_hasher(ignore)]
2019-08-24T16:00:36.5079211Z 
2019-08-24T16:00:36.5079879Z error: cannot find attribute macro `stable_hasher` in this scope
2019-08-24T16:00:36.5080173Z     --> src/librustc/hir/mod.rs:1285:7
2019-08-24T16:00:36.5080398Z      |
2019-08-24T16:00:36.5080398Z      |
2019-08-24T16:00:36.5080672Z 1285 |     #[stable_hasher(ignore)]
2019-08-24T16:00:36.5081008Z 
2019-08-24T16:00:36.5081280Z error: cannot find attribute macro `stable_hasher` in this scope
2019-08-24T16:00:36.5081540Z     --> src/librustc/hir/mod.rs:1884:7
2019-08-24T16:00:36.5081790Z      |
2019-08-24T16:00:36.5081790Z      |
2019-08-24T16:00:36.5082075Z 1884 |     #[stable_hasher(project(name))]
2019-08-24T16:00:36.5082396Z 
2019-08-24T16:00:36.5085080Z error: cannot find attribute macro `stable_hasher` in this scope
2019-08-24T16:00:36.5085506Z     --> src/librustc/hir/mod.rs:2216:7
2019-08-24T16:00:36.5085885Z      |
2019-08-24T16:00:36.5085885Z      |
2019-08-24T16:00:36.5086249Z 2216 |     #[stable_hasher(project(name))]
2019-08-24T16:00:36.5086963Z 
2019-08-24T16:00:36.5087245Z error: cannot find attribute macro `stable_hasher` in this scope
2019-08-24T16:00:36.5087477Z     --> src/librustc/hir/mod.rs:2256:7
2019-08-24T16:00:36.5087805Z      |
2019-08-24T16:00:36.5087805Z      |
2019-08-24T16:00:36.5088155Z 2256 |     #[stable_hasher(ignore)]
2019-08-24T16:00:36.5088667Z 
2019-08-24T16:00:36.5135708Z error: cannot find attribute macro `stable_hasher` in this scope
2019-08-24T16:00:36.5136471Z     --> src/librustc/hir/mod.rs:2325:7
2019-08-24T16:00:36.5139699Z      |
2019-08-24T16:00:36.5139699Z      |
2019-08-24T16:00:36.5140293Z 2325 |     #[stable_hasher(project(name))]
2019-08-24T16:00:36.5140857Z 
2019-08-24T16:00:36.5141193Z error: cannot find attribute macro `stable_hasher` in this scope
2019-08-24T16:00:36.5141594Z     --> src/librustc/hir/mod.rs:2520:7
2019-08-24T16:00:36.5141903Z      |
2019-08-24T16:00:36.5141903Z      |
2019-08-24T16:00:36.5142253Z 2520 |     #[stable_hasher(project(name))]
2019-08-24T16:00:36.5142754Z 
2019-08-24T16:00:36.5143066Z error: cannot find attribute macro `stable_hasher` in this scope
2019-08-24T16:00:36.5143425Z     --> src/librustc/hir/mod.rs:2536:7
2019-08-24T16:00:36.5143744Z      |
2019-08-24T16:00:36.5143744Z      |
2019-08-24T16:00:36.5144111Z 2536 |     #[stable_hasher(project(name))]
2019-08-24T16:00:36.5144800Z 
2019-08-24T16:00:36.5145167Z error: cannot find attribute macro `stable_hasher` in this scope
2019-08-24T16:00:36.5145697Z     --> src/librustc/hir/mod.rs:2554:7
2019-08-24T16:00:36.5146006Z      |
2019-08-24T16:00:36.5146006Z      |
2019-08-24T16:00:36.5146468Z 2554 |     #[stable_hasher(project(name))]
2019-08-24T16:00:36.5176884Z 
2019-08-24T16:00:36.5177435Z error: cannot find attribute macro `stable_hasher` in this scope
2019-08-24T16:00:36.5177843Z   --> src/librustc/ty/trait_def.rs:19:7
2019-08-24T16:00:36.5178228Z    |
2019-08-24T16:00:36.5178228Z    |
2019-08-24T16:00:36.5178490Z 19 |     #[stable_hasher(ignore)]
2019-08-24T16:00:36.5178918Z 
2019-08-24T16:00:36.5179154Z error: cannot find attribute macro `stable_hasher` in this scope
2019-08-24T16:00:36.5179366Z    --> src/librustc/ty/mod.rs:172:7
2019-08-24T16:00:36.5180023Z     |
2019-08-24T16:00:36.5180023Z     |
2019-08-24T16:00:36.5180325Z 172 |     #[stable_hasher(project(name))]
2019-08-24T16:00:36.5180799Z 
2019-08-24T16:00:36.5181116Z error: cannot find attribute macro `stable_hasher` in this scope
2019-08-24T16:00:36.5181379Z    --> src/librustc/ty/mod.rs:901:7
2019-08-24T16:00:36.5181603Z     |
2019-08-24T16:00:36.5181603Z     |
2019-08-24T16:00:36.5181877Z 901 |     #[stable_hasher(ignore)]
2019-08-24T16:00:36.5182201Z 
2019-08-24T16:00:36.5182466Z error: cannot find attribute macro `stable_hasher` in this scope
2019-08-24T16:00:36.5182745Z     --> src/librustc/ty/mod.rs:1934:7
2019-08-24T16:00:36.5183106Z      |
2019-08-24T16:00:36.5183106Z      |
2019-08-24T16:00:36.5183659Z 1934 |     #[stable_hasher(project(name))]
2019-08-24T16:00:36.5183921Z 
2019-08-24T16:00:58.0943793Z error: aborting due to 18 previous errors
2019-08-24T16:00:58.0944070Z 
2019-08-24T16:00:58.3448333Z error: Could not compile `rustc`.
---
2019-08-24T16:01:54.4113921Z == clock drift check ==
2019-08-24T16:01:54.4132979Z   local time: Sat Aug 24 16:01:54 UTC 2019
2019-08-24T16:01:54.6939961Z   network time: Sat, 24 Aug 2019 16:01:54 GMT
2019-08-24T16:01:54.6940816Z == end clock drift check ==
2019-08-24T16:01:55.7329302Z ##[error]Bash exited with code '1'.
2019-08-24T16:01:55.7365508Z ##[section]Starting: Checkout
2019-08-24T16:01:55.7367259Z ==============================================================================
2019-08-24T16:01:55.7367314Z Task         : Get sources
2019-08-24T16:01:55.7367365Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
