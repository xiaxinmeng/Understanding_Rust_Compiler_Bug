plain
2020-02-15T01:50:20.9293974Z ========================== Starting Command Output ===========================
2020-02-15T01:50:20.9297596Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/bdce23c8-2642-4c07-83f9-951091324e0c.sh
2020-02-15T01:50:20.9297787Z 
2020-02-15T01:50:20.9302171Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-15T01:50:20.9307128Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69175/merge to s
2020-02-15T01:50:20.9308741Z Task         : Get sources
2020-02-15T01:50:20.9308769Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-15T01:50:20.9308799Z Version      : 1.0.0
2020-02-15T01:50:20.9308826Z Author       : Microsoft
---
2020-02-15T01:50:21.7354108Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-15T01:50:21.7443151Z ##[command]git config gc.auto 0
2020-02-15T01:50:21.7512110Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-15T01:50:21.7569937Z ##[command]git config --get-all http.proxy
2020-02-15T01:50:21.7720567Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69175/merge:refs/remotes/pull/69175/merge
---
2020-02-15T01:56:13.6799623Z Done!
2020-02-15T01:56:13.6799659Z some tidy checks failed
2020-02-15T01:56:13.6803808Z 
2020-02-15T01:56:13.6803896Z 
2020-02-15T01:56:13.6804709Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2020-02-15T01:56:13.6804824Z 
2020-02-15T01:56:13.6804848Z 
2020-02-15T01:56:13.6816757Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-02-15T01:56:13.6816848Z Build completed unsuccessfully in 0:01:33
2020-02-15T01:56:13.6816848Z Build completed unsuccessfully in 0:01:33
2020-02-15T01:56:13.6866728Z == clock drift check ==
2020-02-15T01:56:13.6876539Z   local time: Sat Feb 15 01:56:13 UTC 2020
2020-02-15T01:56:13.8454607Z   network time: Sat, 15 Feb 2020 01:56:13 GMT
2020-02-15T01:56:13.8458502Z == end clock drift check ==
2020-02-15T01:56:14.5585385Z 
2020-02-15T01:56:14.5671275Z ##[error]Bash exited with code '1'.
2020-02-15T01:56:14.5681835Z ##[section]Finishing: Run build
2020-02-15T01:56:14.5693609Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69175/merge to s
2020-02-15T01:56:14.5695241Z Task         : Get sources
2020-02-15T01:56:14.5695297Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-15T01:56:14.5695333Z Version      : 1.0.0
2020-02-15T01:56:14.5695364Z Author       : Microsoft
2020-02-15T01:56:14.5695364Z Author       : Microsoft
2020-02-15T01:56:14.5695417Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-15T01:56:14.5695588Z ==============================================================================
2020-02-15T01:56:14.9234394Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-15T01:56:14.9270033Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69175/merge to s
2020-02-15T01:56:14.9374214Z Cleaning up task key
2020-02-15T01:56:14.9375154Z Start cleaning up orphan processes.
2020-02-15T01:56:14.9475495Z Terminate orphan process: pid (3699) (python)
2020-02-15T01:56:14.9655344Z ##[section]Finishing: Finalize Job
