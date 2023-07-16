plain
2020-03-22T02:30:47.0888657Z ========================== Starting Command Output ===========================
2020-03-22T02:30:47.0890962Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/534a5675-cc8c-4b89-8330-2029162e4e93.sh
2020-03-22T02:30:47.0891206Z 
2020-03-22T02:30:47.0894355Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-22T02:30:47.0915047Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69251/merge to s
2020-03-22T02:30:47.0917934Z Task         : Get sources
2020-03-22T02:30:47.0918200Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-22T02:30:47.0918443Z Version      : 1.0.0
2020-03-22T02:30:47.0918606Z Author       : Microsoft
---
2020-03-22T02:30:48.4245502Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-22T02:30:48.4251525Z ##[command]git config gc.auto 0
2020-03-22T02:30:48.4256312Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-22T02:30:48.4260460Z ##[command]git config --get-all http.proxy
2020-03-22T02:30:48.4269159Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69251/merge:refs/remotes/pull/69251/merge
---
2020-03-22T02:39:15.3185076Z configure: build.locked-deps    := True
2020-03-22T02:39:15.3185347Z configure: llvm.ccache          := sccache
2020-03-22T02:39:15.3185784Z configure: build.cargo-native-static := True
2020-03-22T02:39:15.3186230Z configure: dist.missing-tools   := True
2020-03-22T02:39:15.3186769Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-03-22T02:39:15.3187298Z configure: writing `config.toml` in current directory
2020-03-22T02:39:15.3187517Z configure: 
2020-03-22T02:39:15.3187887Z configure: run `python /checkout/x.py --help`
2020-03-22T02:39:15.3188112Z configure: 
---
2020-03-22T02:45:31.6435114Z    Compiling cargo_metadata v0.9.1
2020-03-22T02:45:34.7122755Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2020-03-22T02:45:44.1463196Z     Finished release [optimized] target(s) in 21.29s
2020-03-22T02:45:44.1570921Z tidy check
2020-03-22T02:45:50.3996818Z tidy error: /checkout/src/librustc_error_codes/error_codes/E0738.md:1: line longer than 80 chars
2020-03-22T02:45:53.0198940Z Found 489 error codes
2020-03-22T02:45:53.0201190Z Found 0 error codes with no tests
2020-03-22T02:45:53.0201767Z Done!
2020-03-22T02:45:53.0202115Z some tidy checks failed
2020-03-22T02:45:53.0202115Z some tidy checks failed
2020-03-22T02:45:53.0205388Z 
2020-03-22T02:45:53.0205640Z 
2020-03-22T02:45:53.0206848Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo"
2020-03-22T02:45:53.0207557Z 
2020-03-22T02:45:53.0207649Z 
2020-03-22T02:45:53.0220978Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-03-22T02:45:53.0221520Z Build completed unsuccessfully in 0:00:31
2020-03-22T02:45:53.0221520Z Build completed unsuccessfully in 0:00:31
2020-03-22T02:45:53.0272492Z == clock drift check ==
2020-03-22T02:45:53.0291483Z   local time: Sun Mar 22 02:45:53 UTC 2020
2020-03-22T02:45:53.1221396Z   network time: Sun, 22 Mar 2020 02:45:53 GMT
2020-03-22T02:45:53.1225436Z == end clock drift check ==
2020-03-22T02:45:54.8262940Z 
2020-03-22T02:45:54.8302501Z ##[error]Bash exited with code '1'.
2020-03-22T02:45:54.8313606Z ##[section]Finishing: Run build
2020-03-22T02:45:54.8364106Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69251/merge to s
2020-03-22T02:45:54.8368812Z Task         : Get sources
2020-03-22T02:45:54.8369149Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-22T02:45:54.8369467Z Version      : 1.0.0
2020-03-22T02:45:54.8369676Z Author       : Microsoft
2020-03-22T02:45:54.8369676Z Author       : Microsoft
2020-03-22T02:45:54.8370117Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-22T02:45:54.8370451Z ==============================================================================
2020-03-22T02:45:55.1202093Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-22T02:45:55.1261606Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69251/merge to s
2020-03-22T02:45:55.1340723Z Cleaning up task key
2020-03-22T02:45:55.1341904Z Start cleaning up orphan processes.
2020-03-22T02:45:55.1609782Z Terminate orphan process: pid (3891) (python)
2020-03-22T02:45:55.1650456Z ##[section]Finishing: Finalize Job
