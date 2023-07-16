plain
2020-02-23T21:15:46.3732532Z ========================== Starting Command Output ===========================
2020-02-23T21:15:46.3736603Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/ce026cbd-f617-4c9d-86a2-e55478d0dd9f.sh
2020-02-23T21:15:46.3736848Z 
2020-02-23T21:15:46.3740551Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-23T21:15:46.3762165Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69406/merge to s
2020-02-23T21:15:46.3765608Z Task         : Get sources
2020-02-23T21:15:46.3765859Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-23T21:15:46.3766102Z Version      : 1.0.0
2020-02-23T21:15:46.3766317Z Author       : Microsoft
---
2020-02-23T21:15:47.3831120Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-23T21:15:47.3837036Z ##[command]git config gc.auto 0
2020-02-23T21:15:47.3872044Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-23T21:15:47.3933674Z ##[command]git config --get-all http.proxy
2020-02-23T21:15:47.4002801Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69406/merge:refs/remotes/pull/69406/merge
---
2020-02-23T21:19:33.8586032Z 
2020-02-23T21:19:33.8905769Z ################                                                          23.5%
2020-02-23T21:19:33.8907199Z ######################################################################## 100.0%
2020-02-23T21:19:34.1945943Z extracting /checkout/obj/build/cache/2020-01-31/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.gz
2020-02-23T21:19:34.2286406Z error: failed to read `/chalk/chalk-ir/Cargo.toml`
2020-02-23T21:19:34.2293940Z Caused by:
2020-02-23T21:19:34.2300350Z   No such file or directory (os error 2)
2020-02-23T21:19:34.2301617Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml
2020-02-23T21:19:34.2302289Z Build completed unsuccessfully in 0:00:12
2020-02-23T21:19:34.2302289Z Build completed unsuccessfully in 0:00:12
2020-02-23T21:19:34.2322166Z == clock drift check ==
2020-02-23T21:19:34.2347157Z   local time: Sun Feb 23 21:19:34 UTC 2020
2020-02-23T21:19:34.3942904Z   network time: Sun, 23 Feb 2020 21:19:34 GMT
2020-02-23T21:19:34.3943317Z == end clock drift check ==
2020-02-23T21:19:41.9350079Z 
2020-02-23T21:19:41.9430068Z ##[error]Bash exited with code '1'.
2020-02-23T21:19:41.9444034Z ##[section]Finishing: Run build
2020-02-23T21:19:41.9488014Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69406/merge to s
2020-02-23T21:19:41.9492971Z Task         : Get sources
2020-02-23T21:19:41.9493483Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-23T21:19:41.9494172Z Version      : 1.0.0
2020-02-23T21:19:41.9494402Z Author       : Microsoft
2020-02-23T21:19:41.9494402Z Author       : Microsoft
2020-02-23T21:19:41.9494942Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-23T21:19:41.9495965Z ==============================================================================
2020-02-23T21:19:42.2880204Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-23T21:19:42.2927289Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69406/merge to s
2020-02-23T21:19:42.3018930Z Cleaning up task key
2020-02-23T21:19:42.3020274Z Start cleaning up orphan processes.
2020-02-23T21:19:42.3206064Z Terminate orphan process: pid (4919) (python)
2020-02-23T21:19:42.3351256Z ##[section]Finishing: Finalize Job
