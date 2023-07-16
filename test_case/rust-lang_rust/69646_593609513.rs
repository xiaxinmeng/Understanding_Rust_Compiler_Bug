plain
2020-03-02T20:18:31.3998461Z ========================== Starting Command Output ===========================
2020-03-02T20:18:31.4003991Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/ca8aab24-a49e-4ac5-9d9b-97239a75ec12.sh
2020-03-02T20:18:31.4004505Z 
2020-03-02T20:18:31.4009042Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-02T20:18:31.4030471Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69646/merge to s
2020-03-02T20:18:31.4034052Z Task         : Get sources
2020-03-02T20:18:31.4034379Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-02T20:18:31.4034711Z Version      : 1.0.0
2020-03-02T20:18:31.4034928Z Author       : Microsoft
---
2020-03-02T20:18:32.3946353Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-02T20:18:32.3952119Z ##[command]git config gc.auto 0
2020-03-02T20:18:32.3956085Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-02T20:18:32.3959754Z ##[command]git config --get-all http.proxy
2020-03-02T20:18:32.3966405Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69646/merge:refs/remotes/pull/69646/merge
---
2020-03-02T20:24:33.3420202Z    Compiling serde_json v1.0.40
2020-03-02T20:24:34.9632490Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2020-03-02T20:24:44.5314832Z     Finished release [optimized] target(s) in 1m 28s
2020-03-02T20:24:44.5410776Z tidy check
2020-03-02T20:24:44.7142457Z tidy error: /checkout/src/test/ui/consts/const-eval/ub-enum.rs:90: line longer than 100 chars
2020-03-02T20:24:44.7144120Z tidy error: /checkout/src/test/ui/consts/const-eval/ub-enum.rs:92: line longer than 100 chars
2020-03-02T20:24:47.1677537Z Found 489 error codes
2020-03-02T20:24:47.1678589Z Found 0 error codes with no tests
2020-03-02T20:24:47.1685318Z Done!
2020-03-02T20:24:47.1685873Z some tidy checks failed
2020-03-02T20:24:47.1685873Z some tidy checks failed
2020-03-02T20:24:47.1687097Z 
2020-03-02T20:24:47.1687442Z 
2020-03-02T20:24:47.1689280Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2020-03-02T20:24:47.1690515Z 
2020-03-02T20:24:47.1690826Z 
2020-03-02T20:24:47.1694842Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-03-02T20:24:47.1695438Z Build completed unsuccessfully in 0:01:39
2020-03-02T20:24:47.1695438Z Build completed unsuccessfully in 0:01:39
2020-03-02T20:24:47.1820762Z == clock drift check ==
2020-03-02T20:24:47.1821431Z   local time: Mon Mar  2 20:24:47 UTC 2020
2020-03-02T20:24:47.4704355Z   network time: Mon, 02 Mar 2020 20:24:47 GMT
2020-03-02T20:24:47.4704713Z == end clock drift check ==
2020-03-02T20:24:48.2406596Z 
2020-03-02T20:24:48.2487082Z ##[error]Bash exited with code '1'.
2020-03-02T20:24:48.2502742Z ##[section]Finishing: Run build
2020-03-02T20:24:48.2560975Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69646/merge to s
2020-03-02T20:24:48.2566501Z Task         : Get sources
2020-03-02T20:24:48.2566892Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-02T20:24:48.2567273Z Version      : 1.0.0
2020-03-02T20:24:48.2567527Z Author       : Microsoft
2020-03-02T20:24:48.2567527Z Author       : Microsoft
2020-03-02T20:24:48.2567927Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-02T20:24:48.2568409Z ==============================================================================
2020-03-02T20:24:48.6121631Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-02T20:24:48.6179999Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69646/merge to s
2020-03-02T20:24:48.6278738Z Cleaning up task key
2020-03-02T20:24:48.6280146Z Start cleaning up orphan processes.
2020-03-02T20:24:48.6480568Z Terminate orphan process: pid (3516) (python)
2020-03-02T20:24:48.6660713Z ##[section]Finishing: Finalize Job
