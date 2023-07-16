plain
2020-01-24T22:01:33.9330209Z ========================== Starting Command Output ===========================
2020-01-24T22:01:33.9332118Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/7e0d1fea-1894-429e-ab17-80cbcb4e68f0.sh
2020-01-24T22:01:33.9332159Z 
2020-01-24T22:01:33.9335061Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-24T22:01:33.9341789Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68524/merge to s
2020-01-24T22:01:33.9343411Z Task         : Get sources
2020-01-24T22:01:33.9343441Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-24T22:01:33.9343471Z Version      : 1.0.0
2020-01-24T22:01:33.9343516Z Author       : Microsoft
---
2020-01-24T22:01:34.9093274Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-24T22:01:34.9104973Z ##[command]git config gc.auto 0
2020-01-24T22:01:34.9107091Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-24T22:01:34.9108835Z ##[command]git config --get-all http.proxy
2020-01-24T22:01:34.9115205Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68524/merge:refs/remotes/pull/68524/merge
---
2020-01-24T22:32:33.7371424Z    Compiling measureme v0.7.1
2020-01-24T22:32:40.6997121Z    Compiling rustc-rayon v0.3.0
2020-01-24T22:32:40.9884708Z    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
2020-01-24T22:32:46.3380500Z    Compiling rustc_data_structures v0.0.0 (/checkout/src/librustc_data_structures)
2020-01-24T22:32:46.6131958Z error[E0191]: the value of the associated type `Resume` (from trait `std::ops::Generator`) must be specified
2020-01-24T22:32:46.6132282Z   --> src/librustc_data_structures/box_region.rs:24:28
2020-01-24T22:32:46.6132530Z    |
2020-01-24T22:32:46.6132823Z 24 |     generator: Pin<Box<dyn Generator<Yield = YieldType<I, A>, Return = R>>>,
2020-01-24T22:32:46.6133213Z    |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: specify the associated type: `Generator<Yield = YieldType<I, A>, Return = R, Resume = Type>`
2020-01-24T22:32:46.6374797Z error: aborting due to previous error
2020-01-24T22:32:46.6378499Z 
2020-01-24T22:32:46.6388385Z For more information about this error, try `rustc --explain E0191`.
2020-01-24T22:32:46.6451330Z error: could not compile `rustc_data_structures`.
---
2020-01-24T22:32:55.8823146Z   local time: Fri Jan 24 22:32:55 UTC 2020
2020-01-24T22:32:56.1496931Z   network time: Fri, 24 Jan 2020 22:32:56 GMT
2020-01-24T22:32:56.1504464Z == end clock drift check ==
2020-01-24T22:32:56.9333947Z 
2020-01-24T22:32:56.9415769Z ##[error]Bash exited with code '1'.
2020-01-24T22:32:56.9426748Z ##[section]Finishing: Run build
2020-01-24T22:32:56.9449749Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68524/merge to s
2020-01-24T22:32:56.9453219Z Task         : Get sources
2020-01-24T22:32:56.9453283Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-24T22:32:56.9453326Z Version      : 1.0.0
2020-01-24T22:32:56.9453366Z Author       : Microsoft
2020-01-24T22:32:56.9453366Z Author       : Microsoft
2020-01-24T22:32:56.9453426Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-24T22:32:56.9453473Z ==============================================================================
2020-01-24T22:32:57.4425044Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-24T22:32:57.4465578Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68524/merge to s
2020-01-24T22:32:57.4582996Z Cleaning up task key
2020-01-24T22:32:57.4583791Z Start cleaning up orphan processes.
2020-01-24T22:32:57.4732435Z Terminate orphan process: pid (7106) (python)
2020-01-24T22:32:57.4985275Z ##[section]Finishing: Finalize Job
