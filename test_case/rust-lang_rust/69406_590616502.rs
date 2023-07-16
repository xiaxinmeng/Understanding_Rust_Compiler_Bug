plain
2020-02-25T00:17:23.8508291Z ========================== Starting Command Output ===========================
2020-02-25T00:17:23.8512853Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/9c54b0f1-dbd3-40c7-9de8-2122b8de898d.sh
2020-02-25T00:17:23.8513270Z 
2020-02-25T00:17:23.8517791Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-25T00:17:23.8535010Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69406/merge to s
2020-02-25T00:17:23.8538096Z Task         : Get sources
2020-02-25T00:17:23.8538356Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-25T00:17:23.8538608Z Version      : 1.0.0
2020-02-25T00:17:23.8538780Z Author       : Microsoft
---
2020-02-25T00:17:25.0615365Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-25T00:17:25.0625091Z ##[command]git config gc.auto 0
2020-02-25T00:17:25.0632957Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-25T00:17:25.0643070Z ##[command]git config --get-all http.proxy
2020-02-25T00:17:25.0658422Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69406/merge:refs/remotes/pull/69406/merge
---
2020-02-25T00:22:01.3711816Z 
2020-02-25T00:22:01.3844890Z #########################################################                 80.4%
2020-02-25T00:22:01.3851832Z ######################################################################## 100.0%
2020-02-25T00:22:01.6832888Z extracting /checkout/obj/build/cache/2020-01-31/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.gz
2020-02-25T00:22:01.7122516Z error: failed to read `/chalk/chalk-ir/Cargo.toml`
2020-02-25T00:22:01.7123128Z Caused by:
2020-02-25T00:22:01.7123364Z   No such file or directory (os error 2)
2020-02-25T00:22:01.7129828Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml
2020-02-25T00:22:01.7130545Z Build completed unsuccessfully in 0:00:19
2020-02-25T00:22:01.7130545Z Build completed unsuccessfully in 0:00:19
2020-02-25T00:22:01.7175016Z == clock drift check ==
2020-02-25T00:22:01.7183134Z   local time: Tue Feb 25 00:22:01 UTC 2020
2020-02-25T00:22:01.7458899Z   network time: Tue, 25 Feb 2020 00:22:01 GMT
2020-02-25T00:22:01.7461143Z == end clock drift check ==
2020-02-25T00:22:09.2741166Z 
2020-02-25T00:22:09.2777255Z ##[error]Bash exited with code '1'.
2020-02-25T00:22:09.2792912Z ##[section]Finishing: Run build
2020-02-25T00:22:09.2840347Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69406/merge to s
2020-02-25T00:22:09.2844886Z Task         : Get sources
2020-02-25T00:22:09.2845376Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-25T00:22:09.2845675Z Version      : 1.0.0
2020-02-25T00:22:09.2845902Z Author       : Microsoft
2020-02-25T00:22:09.2845902Z Author       : Microsoft
2020-02-25T00:22:09.2846233Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-25T00:22:09.2846618Z ==============================================================================
2020-02-25T00:22:09.6281317Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-25T00:22:09.6336054Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69406/merge to s
2020-02-25T00:22:09.6423540Z Cleaning up task key
2020-02-25T00:22:09.6424797Z Start cleaning up orphan processes.
2020-02-25T00:22:09.6601084Z Terminate orphan process: pid (15055) (python)
2020-02-25T00:22:09.6766794Z ##[section]Finishing: Finalize Job
