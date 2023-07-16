plain
2020-02-20T18:41:19.0882261Z ========================== Starting Command Output ===========================
2020-02-20T18:41:19.0886002Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/588fe109-9643-4280-ae1b-c64984ed0174.sh
2020-02-20T18:41:19.0886462Z 
2020-02-20T18:41:19.0890175Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-20T18:41:19.0910105Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69218/merge to s
2020-02-20T18:41:19.0913472Z Task         : Get sources
2020-02-20T18:41:19.0913827Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-20T18:41:19.0914143Z Version      : 1.0.0
2020-02-20T18:41:19.0914358Z Author       : Microsoft
---
2020-02-20T18:41:20.0811700Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-20T18:41:20.0817838Z ##[command]git config gc.auto 0
2020-02-20T18:41:20.0821556Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-20T18:41:20.0825026Z ##[command]git config --get-all http.proxy
2020-02-20T18:41:20.0831093Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69218/merge:refs/remotes/pull/69218/merge
---
2020-02-20T18:45:11.6385837Z 
2020-02-20T18:45:11.6625275Z ##################################################                        70.0%
2020-02-20T18:45:11.6625824Z ######################################################################## 100.0%
2020-02-20T18:45:11.9527970Z extracting /checkout/obj/build/cache/2020-01-31/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.gz
2020-02-20T18:45:11.9827310Z error: failed to read `/ena/Cargo.toml`
2020-02-20T18:45:11.9827951Z Caused by:
2020-02-20T18:45:11.9828381Z   No such file or directory (os error 2)
2020-02-20T18:45:11.9837646Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml
2020-02-20T18:45:11.9838169Z Build completed unsuccessfully in 0:00:13
2020-02-20T18:45:11.9838169Z Build completed unsuccessfully in 0:00:13
2020-02-20T18:45:11.9886425Z == clock drift check ==
2020-02-20T18:45:11.9897341Z   local time: Thu Feb 20 18:45:11 UTC 2020
2020-02-20T18:45:12.2742909Z   network time: Thu, 20 Feb 2020 18:45:12 GMT
2020-02-20T18:45:12.2747505Z == end clock drift check ==
2020-02-20T18:45:19.8183803Z 
2020-02-20T18:45:19.8252488Z ##[error]Bash exited with code '1'.
2020-02-20T18:45:19.8277279Z ##[section]Finishing: Run build
2020-02-20T18:45:19.8327638Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69218/merge to s
2020-02-20T18:45:19.8332827Z Task         : Get sources
2020-02-20T18:45:19.8333204Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-20T18:45:19.8333566Z Version      : 1.0.0
2020-02-20T18:45:19.8333834Z Author       : Microsoft
2020-02-20T18:45:19.8333834Z Author       : Microsoft
2020-02-20T18:45:19.8334239Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-20T18:45:19.8334710Z ==============================================================================
2020-02-20T18:45:20.1622063Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-20T18:45:20.1672168Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69218/merge to s
2020-02-20T18:45:20.1759979Z Cleaning up task key
2020-02-20T18:45:20.1761347Z Start cleaning up orphan processes.
2020-02-20T18:45:20.1930030Z Terminate orphan process: pid (4271) (python)
2020-02-20T18:45:20.2095921Z ##[section]Finishing: Finalize Job
