plain
2020-02-19T03:55:58.4684641Z ========================== Starting Command Output ===========================
2020-02-19T03:55:58.4704362Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/14458366-0308-4e21-b817-5b4bbf92e1b3.sh
2020-02-19T03:55:58.4922906Z 
2020-02-19T03:55:58.5016922Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-19T03:55:58.5023931Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68362/merge to s
2020-02-19T03:55:58.5025883Z Task         : Get sources
2020-02-19T03:55:58.5025909Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-19T03:55:58.5025956Z Version      : 1.0.0
2020-02-19T03:55:58.5025981Z Author       : Microsoft
---
2020-02-19T03:55:59.3741783Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-19T03:55:59.3824454Z ##[command]git config gc.auto 0
2020-02-19T03:55:59.3896815Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-19T03:55:59.3960562Z ##[command]git config --get-all http.proxy
2020-02-19T03:55:59.4102979Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68362/merge:refs/remotes/pull/68362/merge
---
2020-02-19T03:58:44.9642065Z extracting /checkout/obj/build/cache/2020-01-31/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.gz
2020-02-19T03:58:45.6726660Z error: failed to resolve patches for `https://github.com/rust-lang/cargo`
2020-02-19T03:58:45.6728016Z 
2020-02-19T03:58:45.6728254Z Caused by:
2020-02-19T03:58:45.6729246Z   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
2020-02-19T03:58:45.6730558Z Build completed unsuccessfully in 0:00:13
2020-02-19T03:58:45.6730753Z == clock drift check ==
2020-02-19T03:58:45.6730887Z   local time: Wed Feb 19 03:58:45 UTC 2020
2020-02-19T03:58:45.6731062Z   network time: Wed, 19 Feb 2020 03:58:45 GMT
2020-02-19T03:58:45.6731062Z   network time: Wed, 19 Feb 2020 03:58:45 GMT
2020-02-19T03:58:45.6731243Z == end clock drift check ==
2020-02-19T03:58:52.8788389Z 
2020-02-19T03:58:52.8884355Z ##[error]Bash exited with code '1'.
2020-02-19T03:58:52.8920222Z ##[section]Finishing: Run build
2020-02-19T03:58:52.8939875Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68362/merge to s
2020-02-19T03:58:52.8941837Z Task         : Get sources
2020-02-19T03:58:52.8941878Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-19T03:58:52.8941914Z Version      : 1.0.0
2020-02-19T03:58:52.8941981Z Author       : Microsoft
2020-02-19T03:58:52.8941981Z Author       : Microsoft
2020-02-19T03:58:52.8942019Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-19T03:58:52.8942058Z ==============================================================================
2020-02-19T03:58:53.2891479Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-19T03:58:53.2933818Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68362/merge to s
2020-02-19T03:58:53.3046994Z Cleaning up task key
2020-02-19T03:58:53.3048571Z Start cleaning up orphan processes.
2020-02-19T03:58:53.3422022Z Terminate orphan process: pid (3774) (python)
2020-02-19T03:58:53.3444097Z ##[section]Finishing: Finalize Job
