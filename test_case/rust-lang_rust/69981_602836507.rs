plain
2020-03-23T18:36:05.6348098Z ========================== Starting Command Output ===========================
2020-03-23T18:36:05.6350559Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/821aa7af-7a9a-4b09-8b1d-e46c0fa21602.sh
2020-03-23T18:36:05.6350854Z 
2020-03-23T18:36:05.6354805Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-23T18:36:05.6374712Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69981/merge to s
2020-03-23T18:36:05.6378364Z Task         : Get sources
2020-03-23T18:36:05.6378694Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-23T18:36:05.6378995Z Version      : 1.0.0
2020-03-23T18:36:05.6379199Z Author       : Microsoft
---
2020-03-23T18:36:06.6287399Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-23T18:36:06.6293692Z ##[command]git config gc.auto 0
2020-03-23T18:36:06.6297337Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-23T18:36:06.6300682Z ##[command]git config --get-all http.proxy
2020-03-23T18:36:06.6306984Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69981/merge:refs/remotes/pull/69981/merge
---
2020-03-23T18:44:17.9235379Z configure: build.locked-deps    := True
2020-03-23T18:44:17.9235795Z configure: llvm.ccache          := sccache
2020-03-23T18:44:17.9237254Z configure: build.cargo-native-static := True
2020-03-23T18:44:17.9238037Z configure: dist.missing-tools   := True
2020-03-23T18:44:17.9238882Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-03-23T18:44:17.9239679Z configure: writing `config.toml` in current directory
2020-03-23T18:44:17.9240045Z configure: 
2020-03-23T18:44:17.9240667Z configure: run `python /checkout/x.py --help`
2020-03-23T18:44:17.9240990Z configure: 
---
2020-03-23T18:51:20.8027964Z    Compiling cargo_metadata v0.9.1
2020-03-23T18:51:25.0830655Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2020-03-23T18:51:35.2533919Z     Finished release [optimized] target(s) in 23.94s
2020-03-23T18:51:35.2623800Z tidy check
2020-03-23T18:51:40.4428009Z tidy error: /checkout/src/librustc/mir/mod.rs: too many lines (3007) (add `// ignore-tidy-filelength` to the file to suppress this error)
2020-03-23T18:51:43.6162303Z some tidy checks failed
2020-03-23T18:51:43.6169393Z Found 489 error codes
2020-03-23T18:51:43.6170220Z Found 0 error codes with no tests
2020-03-23T18:51:43.6170944Z Done!
2020-03-23T18:51:43.6170944Z Done!
2020-03-23T18:51:43.6171452Z 
2020-03-23T18:51:43.6171923Z 
2020-03-23T18:51:43.6180698Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo"
2020-03-23T18:51:43.6185131Z 
2020-03-23T18:51:43.6185382Z 
2020-03-23T18:51:43.6185893Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-03-23T18:51:43.6186363Z Build completed unsuccessfully in 0:00:33
2020-03-23T18:51:43.6186363Z Build completed unsuccessfully in 0:00:33
2020-03-23T18:51:43.6257885Z == clock drift check ==
2020-03-23T18:51:43.6272820Z   local time: Mon Mar 23 18:51:43 UTC 2020
2020-03-23T18:51:43.7904364Z   network time: Mon, 23 Mar 2020 18:51:43 GMT
2020-03-23T18:51:43.7904794Z == end clock drift check ==
2020-03-23T18:51:45.3407128Z 
2020-03-23T18:51:45.3476470Z ##[error]Bash exited with code '1'.
2020-03-23T18:51:45.3509354Z ##[section]Finishing: Run build
2020-03-23T18:51:45.3555874Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69981/merge to s
2020-03-23T18:51:45.3560766Z Task         : Get sources
2020-03-23T18:51:45.3561119Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-23T18:51:45.3561443Z Version      : 1.0.0
2020-03-23T18:51:45.3561683Z Author       : Microsoft
2020-03-23T18:51:45.3561683Z Author       : Microsoft
2020-03-23T18:51:45.3562034Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-23T18:51:45.3562447Z ==============================================================================
2020-03-23T18:51:45.7030263Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-23T18:51:45.7075816Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69981/merge to s
2020-03-23T18:51:45.7163986Z Cleaning up task key
2020-03-23T18:51:45.7165350Z Start cleaning up orphan processes.
2020-03-23T18:51:45.7343453Z Terminate orphan process: pid (3496) (python)
2020-03-23T18:51:45.7582093Z ##[section]Finishing: Finalize Job
