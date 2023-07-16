plain
2019-08-24T18:03:01.9292879Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-24T18:03:01.9509219Z ##[command]git config gc.auto 0
2019-08-24T18:03:01.9554357Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-24T18:03:01.9613026Z ##[command]git config --get-all http.proxy
2019-08-24T18:03:01.9753327Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63860/merge:refs/remotes/pull/63860/merge
---
2019-08-24T18:03:36.5101381Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-24T18:03:36.5102290Z 
2019-08-24T18:03:36.5103450Z   git checkout -b <new-branch-name>
2019-08-24T18:03:36.5104314Z 
2019-08-24T18:03:36.5105064Z HEAD is now at 15593fcee Merge 24cf5ffa00e681c6cf873cd80c4dc8976b0e14ff into 5ade61a4f1515d4a18f38dacdbdb592bfd384a84
2019-08-24T18:03:36.5294276Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-24T18:03:36.5296878Z ==============================================================================
2019-08-24T18:03:36.5296948Z Task         : Bash
2019-08-24T18:03:36.5296990Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-24T18:12:08.4931222Z     Checking rustc_traits v0.0.0 (/checkout/src/librustc_traits)
2019-08-24T18:12:09.1175052Z error: unused variable: `index`
2019-08-24T18:12:09.1175687Z     --> src/librustc_mir/transform/qualify_consts.rs:1301:13
2019-08-24T18:12:09.1175952Z      |
2019-08-24T18:12:09.1176941Z 1301 |         let index = loop {
2019-08-24T18:12:09.1177289Z      |             ^^^^^ help: consider prefixing with an underscore: `_index`
2019-08-24T18:12:09.1177868Z      = note: `-D unused-variables` implied by `-D warnings`
2019-08-24T18:12:09.1183295Z 
2019-08-24T18:12:09.7418139Z     Checking rustc_codegen_utils v0.0.0 (/checkout/src/librustc_codegen_utils)
2019-08-24T18:12:10.3372820Z     Checking rustc_plugin_impl v0.0.0 (/checkout/src/librustc_plugin)
---
2019-08-24T18:12:18.1705613Z == clock drift check ==
2019-08-24T18:12:18.1722147Z   local time: Sat Aug 24 18:12:18 UTC 2019
2019-08-24T18:12:18.3047865Z   network time: Sat, 24 Aug 2019 18:12:18 GMT
2019-08-24T18:12:18.3050187Z == end clock drift check ==
2019-08-24T18:12:19.0330106Z ##[error]Bash exited with code '1'.
2019-08-24T18:12:19.0368272Z ##[section]Starting: Checkout
2019-08-24T18:12:19.0370103Z ==============================================================================
2019-08-24T18:12:19.0370172Z Task         : Get sources
2019-08-24T18:12:19.0370214Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
