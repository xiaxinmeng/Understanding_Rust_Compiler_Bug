plain
2020-02-25T17:06:24.8872696Z ========================== Starting Command Output ===========================
2020-02-25T17:06:24.8875077Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/ec393ed9-a0db-4c4d-8940-82fd4287a9bd.sh
2020-02-25T17:06:24.8875322Z 
2020-02-25T17:06:24.8879808Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-25T17:06:24.8897461Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69464/merge to s
2020-02-25T17:06:24.8900949Z Task         : Get sources
2020-02-25T17:06:24.8901376Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-25T17:06:24.8901629Z Version      : 1.0.0
2020-02-25T17:06:24.8901795Z Author       : Microsoft
---
2020-02-25T17:06:26.0469435Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-25T17:06:26.0480701Z ##[command]git config gc.auto 0
2020-02-25T17:06:26.0487795Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-25T17:06:26.0494668Z ##[command]git config --get-all http.proxy
2020-02-25T17:06:26.0504649Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69464/merge:refs/remotes/pull/69464/merge
---
2020-02-25T17:09:12.0911306Z 
2020-02-25T17:09:12.0915582Z ########################################################                  78.9%
2020-02-25T17:09:12.0916014Z ######################################################################## 100.0%
2020-02-25T17:09:12.2488550Z extracting /checkout/obj/build/cache/2020-01-31/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.gz
2020-02-25T17:09:12.3276911Z     Updating git repository `https://github.com/Marwes/ena`
2020-02-25T17:09:12.7543016Z error: failed to resolve patches for `https://github.com/rust-lang/crates.io-index`
2020-02-25T17:09:12.7543576Z Caused by:
2020-02-25T17:09:12.7543926Z   failed to load source for a dependency on `ena`
2020-02-25T17:09:12.7544135Z 
2020-02-25T17:09:12.7544269Z Caused by:
2020-02-25T17:09:12.7544269Z Caused by:
2020-02-25T17:09:12.7544557Z   Unable to update https://github.com/Marwes/ena?branch=detach_undo_log#da974876
2020-02-25T17:09:12.7544964Z Caused by:
2020-02-25T17:09:12.7544964Z Caused by:
2020-02-25T17:09:12.7545871Z   revspec 'da974876317c95a0fb797e45530cacf383fa873b' not found; class=Reference (4); code=NotFound (-3)
2020-02-25T17:09:12.7552194Z Build completed unsuccessfully in 0:00:13
2020-02-25T17:09:12.7625584Z == clock drift check ==
2020-02-25T17:09:12.7637507Z   local time: Tue Feb 25 17:09:12 UTC 2020
2020-02-25T17:09:13.3037844Z   network time: Tue, 25 Feb 2020 17:09:13 GMT
2020-02-25T17:09:13.3037844Z   network time: Tue, 25 Feb 2020 17:09:13 GMT
2020-02-25T17:09:13.3038180Z == end clock drift check ==
2020-02-25T17:09:20.8677402Z 
2020-02-25T17:09:20.8771239Z ##[error]Bash exited with code '1'.
2020-02-25T17:09:20.8787798Z ##[section]Finishing: Run build
2020-02-25T17:09:20.8834123Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69464/merge to s
2020-02-25T17:09:20.8840048Z Task         : Get sources
2020-02-25T17:09:20.8840501Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-25T17:09:20.8840777Z Version      : 1.0.0
2020-02-25T17:09:20.8840974Z Author       : Microsoft
2020-02-25T17:09:20.8840974Z Author       : Microsoft
2020-02-25T17:09:20.8841307Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-25T17:09:20.8841749Z ==============================================================================
2020-02-25T17:09:21.2242588Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-25T17:09:21.2293401Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69464/merge to s
2020-02-25T17:09:21.2384011Z Cleaning up task key
2020-02-25T17:09:21.2385121Z Start cleaning up orphan processes.
2020-02-25T17:09:21.2650691Z Terminate orphan process: pid (3787) (python)
2020-02-25T17:09:21.2680434Z ##[section]Finishing: Finalize Job
