plain
2020-01-23T02:03:15.7969304Z ========================== Starting Command Output ===========================
2020-01-23T02:03:15.7983481Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/7ea3ab8f-fc7a-4b7e-8ae4-577b7f0b6df3.sh
2020-01-23T02:03:16.0933415Z 
2020-01-23T02:03:16.0995188Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-23T02:03:16.1000691Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67398/merge to s
2020-01-23T02:03:16.1002329Z Task         : Get sources
2020-01-23T02:03:16.1002403Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-23T02:03:16.1002435Z Version      : 1.0.0
2020-01-23T02:03:16.1002467Z Author       : Microsoft
---
2020-01-23T02:03:19.1096725Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-23T02:03:19.1286847Z ##[command]git config gc.auto 0
2020-01-23T02:03:19.1361461Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-23T02:03:19.1409476Z ##[command]git config --get-all http.proxy
2020-01-23T02:03:19.1555730Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67398/merge:refs/remotes/pull/67398/merge
---
2020-01-23T02:08:52.3262307Z     Checking num_cpus v1.10.1
2020-01-23T02:08:52.4135281Z     Checking memmap v0.7.0
2020-01-23T02:08:52.5371866Z     Checking atty v0.2.11
2020-01-23T02:08:52.5916668Z     Checking term_size v0.3.1
2020-01-23T02:08:52.6569846Z     Checking jobserver v0.1.19 (https://github.com/alexcrichton/jobserver-rs#3fa404cf)
2020-01-23T02:08:53.2221684Z     Checking env_logger v0.7.1
2020-01-23T02:08:53.4670863Z     Checking rustc-hash v1.0.1
2020-01-23T02:08:53.5355552Z    Compiling memoffset v0.5.1
2020-01-23T02:08:53.6591344Z    Compiling parking_lot_core v0.6.2
2020-01-23T02:08:53.6591344Z    Compiling parking_lot_core v0.6.2
2020-01-23T02:08:53.9187979Z    Compiling parking_lot v0.9.0
2020-01-23T02:08:54.0076386Z     Checking rustc_apfloat v0.0.0 (/checkout/src/librustc_apfloat)
2020-01-23T02:08:54.2770613Z    Compiling quote v1.0.2
2020-01-23T02:08:54.6630374Z     Checking flate2 v1.0.12
2020-01-23T02:08:55.2032858Z     Checking rustc_jobserver v0.0.0 (/checkout/src/librustc_jobserver)
2020-01-23T02:08:55.3108478Z error[E0599]: no method named `drop_without_release` found for type `jobserver::Acquired` in the current scope
2020-01-23T02:08:55.3108851Z    --> src/librustc_jobserver/lib.rs:171:47
2020-01-23T02:08:55.3109156Z     |
2020-01-23T02:08:55.3109480Z 171 |                 token.expect("acquire token").drop_without_release();
2020-01-23T02:08:55.3109920Z     |                                               ^^^^^^^^^^^^^^^^^^^^ help: there is a method with a similar name: `drop_without_releasing`
2020-01-23T02:08:55.3284388Z error: aborting due to previous error
2020-01-23T02:08:55.3284576Z 
2020-01-23T02:08:55.3290102Z For more information about this error, try `rustc --explain E0599`.
2020-01-23T02:08:55.3323805Z error: could not compile `rustc_jobserver`.
2020-01-23T02:08:55.3323805Z error: could not compile `rustc_jobserver`.
2020-01-23T02:08:55.3330298Z warning: build failed, waiting for other jobs to finish...
2020-01-23T02:08:55.8460316Z error: build failed
2020-01-23T02:08:55.8484068Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-01-23T02:08:55.8495009Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-01-23T02:08:55.8495141Z Build completed unsuccessfully in 0:02:31
2020-01-23T02:08:55.8547492Z == clock drift check ==
2020-01-23T02:08:55.8556722Z   local time: Thu Jan 23 02:08:55 UTC 2020
2020-01-23T02:08:55.8556722Z   local time: Thu Jan 23 02:08:55 UTC 2020
2020-01-23T02:08:55.9983679Z   network time: Thu, 23 Jan 2020 02:08:55 GMT
2020-01-23T02:08:55.9983815Z == end clock drift check ==
2020-01-23T02:08:57.8964020Z 
2020-01-23T02:08:57.9060386Z ##[error]Bash exited with code '1'.
2020-01-23T02:08:57.9073036Z ##[section]Finishing: Run build
2020-01-23T02:08:57.9087609Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67398/merge to s
2020-01-23T02:08:57.9089409Z Task         : Get sources
2020-01-23T02:08:57.9089461Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-23T02:08:57.9089526Z Version      : 1.0.0
2020-01-23T02:08:57.9089572Z Author       : Microsoft
2020-01-23T02:08:57.9089572Z Author       : Microsoft
2020-01-23T02:08:57.9089625Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-23T02:08:57.9089693Z ==============================================================================
2020-01-23T02:08:58.2857785Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-23T02:08:58.2895828Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/67398/merge to s
2020-01-23T02:08:58.3009617Z Cleaning up task key
2020-01-23T02:08:58.3010364Z Start cleaning up orphan processes.
2020-01-23T02:08:58.3108530Z Terminate orphan process: pid (3620) (python)
2020-01-23T02:08:58.3549471Z ##[section]Finishing: Finalize Job
