plain
2020-02-16T00:55:05.7206040Z ========================== Starting Command Output ===========================
2020-02-16T00:55:05.7208405Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/3e3264f4-5b2a-4b16-b72a-41c97274d7a1.sh
2020-02-16T00:55:05.7208510Z 
2020-02-16T00:55:05.7211829Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-16T00:55:05.7218936Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68985/merge to s
2020-02-16T00:55:05.7220880Z Task         : Get sources
2020-02-16T00:55:05.7220980Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-16T00:55:05.7221020Z Version      : 1.0.0
2020-02-16T00:55:05.7221057Z Author       : Microsoft
---
2020-02-16T00:55:06.6977367Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-16T00:55:06.6990143Z ##[command]git config gc.auto 0
2020-02-16T00:55:06.6993045Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-16T00:55:06.6995494Z ##[command]git config --get-all http.proxy
2020-02-16T00:55:06.7003585Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68985/merge:refs/remotes/pull/68985/merge
---
2020-02-16T01:01:46.9972698Z Found 0 error codes with no tests
2020-02-16T01:01:46.9972800Z Done!
2020-02-16T01:01:46.9972836Z 
2020-02-16T01:01:46.9972866Z 
2020-02-16T01:01:46.9973842Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2020-02-16T01:01:46.9974391Z 
2020-02-16T01:01:46.9974427Z 
2020-02-16T01:01:46.9985679Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-02-16T01:01:46.9986158Z Build completed unsuccessfully in 0:01:46
2020-02-16T01:01:46.9986158Z Build completed unsuccessfully in 0:01:46
2020-02-16T01:01:47.0041681Z == clock drift check ==
2020-02-16T01:01:47.0050599Z   local time: Sun Feb 16 01:01:47 UTC 2020
2020-02-16T01:01:47.1680929Z   network time: Sun, 16 Feb 2020 01:01:47 GMT
2020-02-16T01:01:47.1681062Z == end clock drift check ==
2020-02-16T01:01:47.9538823Z 
2020-02-16T01:01:47.9651008Z ##[error]Bash exited with code '1'.
2020-02-16T01:01:47.9664241Z ##[section]Finishing: Run build
2020-02-16T01:01:47.9681810Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68985/merge to s
2020-02-16T01:01:47.9683811Z Task         : Get sources
2020-02-16T01:01:47.9683864Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-16T01:01:47.9683933Z Version      : 1.0.0
2020-02-16T01:01:47.9683982Z Author       : Microsoft
2020-02-16T01:01:47.9683982Z Author       : Microsoft
2020-02-16T01:01:47.9684033Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-16T01:01:47.9684107Z ==============================================================================
2020-02-16T01:01:48.4117242Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-16T01:01:48.4159889Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68985/merge to s
2020-02-16T01:01:48.4282431Z Cleaning up task key
2020-02-16T01:01:48.4283288Z Start cleaning up orphan processes.
2020-02-16T01:01:48.4402257Z Terminate orphan process: pid (3556) (python)
2020-02-16T01:01:48.4617847Z ##[section]Finishing: Finalize Job
