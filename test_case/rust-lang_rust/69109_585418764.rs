plain
2020-02-12T20:57:23.2423374Z ========================== Starting Command Output ===========================
2020-02-12T20:57:23.2424857Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/7fb020f8-8782-4239-a02e-06c2b64815d7.sh
2020-02-12T20:57:23.2424887Z 
2020-02-12T20:57:23.2426962Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-12T20:57:23.2433621Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69109/merge to s
2020-02-12T20:57:23.2435275Z Task         : Get sources
2020-02-12T20:57:23.2435310Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-12T20:57:23.2435345Z Version      : 1.0.0
2020-02-12T20:57:23.2435381Z Author       : Microsoft
---
2020-02-12T20:57:24.0040222Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-12T20:57:24.0143290Z ##[command]git config gc.auto 0
2020-02-12T20:57:24.0282835Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-12T20:57:24.0328580Z ##[command]git config --get-all http.proxy
2020-02-12T20:57:24.0449528Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69109/merge:refs/remotes/pull/69109/merge
---
2020-02-12T21:03:55.8669979Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-02-12T21:03:58.0584442Z     Checking rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
2020-02-12T21:03:58.7283233Z     Checking rustc_expand v0.0.0 (/checkout/src/librustc_expand)
2020-02-12T21:04:00.0799650Z     Checking rustc_builtin_macros v0.0.0 (/checkout/src/librustc_builtin_macros)
2020-02-12T21:04:09.8361896Z error[E0609]: no field `cache_hits` on type `&mut ty::query::plumbing::QueryCache<'_, Q>`
2020-02-12T21:04:09.8363129Z    --> src/librustc/ty/query/plumbing.rs:396:36
2020-02-12T21:04:09.8363671Z     |
2020-02-12T21:04:09.8364561Z 396 |         let cache_hits = &mut lock.cache_hits;
2020-02-12T21:04:09.8365700Z     |
2020-02-12T21:04:09.8365700Z     |
2020-02-12T21:04:09.8366299Z     = note: available fields are: `results`, `active`, `jobs`
2020-02-12T21:04:13.2243154Z error: aborting due to previous error
2020-02-12T21:04:13.2244036Z 
2020-02-12T21:04:13.2244688Z For more information about this error, try `rustc --explain E0609`.
2020-02-12T21:04:13.2513835Z error: could not compile `rustc`.
2020-02-12T21:04:13.2513835Z error: could not compile `rustc`.
2020-02-12T21:04:13.2514556Z 
2020-02-12T21:04:13.2515034Z To learn more, run the command again with --verbose.
2020-02-12T21:04:13.2538193Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-02-12T21:04:13.2551005Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-02-12T21:04:13.2551223Z Build completed unsuccessfully in 0:04:34
2020-02-12T21:04:13.2603436Z == clock drift check ==
2020-02-12T21:04:13.2614454Z   local time: Wed Feb 12 21:04:13 UTC 2020
2020-02-12T21:04:13.2614454Z   local time: Wed Feb 12 21:04:13 UTC 2020
2020-02-12T21:04:13.4231251Z   network time: Wed, 12 Feb 2020 21:04:13 GMT
2020-02-12T21:04:13.4235903Z == end clock drift check ==
2020-02-12T21:04:13.7923013Z 
2020-02-12T21:04:13.8009133Z ##[error]Bash exited with code '1'.
2020-02-12T21:04:13.8020276Z ##[section]Finishing: Run build
2020-02-12T21:04:13.8034893Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69109/merge to s
2020-02-12T21:04:13.8036641Z Task         : Get sources
2020-02-12T21:04:13.8036691Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-12T21:04:13.8036759Z Version      : 1.0.0
2020-02-12T21:04:13.8036803Z Author       : Microsoft
2020-02-12T21:04:13.8036803Z Author       : Microsoft
2020-02-12T21:04:13.8036853Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-12T21:04:13.8036921Z ==============================================================================
2020-02-12T21:04:14.1993668Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-12T21:04:14.2039271Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69109/merge to s
2020-02-12T21:04:14.2159142Z Cleaning up task key
2020-02-12T21:04:14.2159931Z Start cleaning up orphan processes.
2020-02-12T21:04:14.2257170Z Terminate orphan process: pid (3896) (python)
2020-02-12T21:04:14.2437894Z ##[section]Finishing: Finalize Job
