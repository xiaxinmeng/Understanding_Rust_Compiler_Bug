plain
2020-03-22T05:19:46.3790906Z ========================== Starting Command Output ===========================
2020-03-22T05:19:46.3793188Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/80507012-eae8-4eb3-90c8-cff147597ae3.sh
2020-03-22T05:19:46.3793472Z 
2020-03-22T05:19:46.3797415Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-22T05:19:46.3815379Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70261/merge to s
2020-03-22T05:19:46.3818344Z Task         : Get sources
2020-03-22T05:19:46.3818646Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-22T05:19:46.3818928Z Version      : 1.0.0
2020-03-22T05:19:46.3819115Z Author       : Microsoft
---
2020-03-22T05:19:47.3761110Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-22T05:19:47.3766535Z ##[command]git config gc.auto 0
2020-03-22T05:19:47.3770443Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-22T05:19:47.3773443Z ##[command]git config --get-all http.proxy
2020-03-22T05:19:47.3779494Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70261/merge:refs/remotes/pull/70261/merge
---
2020-03-22T05:28:10.4344923Z configure: build.locked-deps    := True
2020-03-22T05:28:10.4345377Z configure: llvm.ccache          := sccache
2020-03-22T05:28:10.4345947Z configure: build.cargo-native-static := True
2020-03-22T05:28:10.4346507Z configure: dist.missing-tools   := True
2020-03-22T05:28:10.4347396Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-03-22T05:28:10.4348131Z configure: writing `config.toml` in current directory
2020-03-22T05:28:10.4348472Z configure: 
2020-03-22T05:28:10.4348967Z configure: run `python /checkout/x.py --help`
2020-03-22T05:28:10.4349292Z configure: 
---
2020-03-22T05:34:26.6127456Z    Compiling cargo_metadata v0.9.1
2020-03-22T05:34:30.5387473Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2020-03-22T05:34:39.5293499Z     Finished release [optimized] target(s) in 21.38s
2020-03-22T05:34:39.5356595Z tidy check
2020-03-22T05:34:39.7168209Z tidy error: /checkout/src/test/ui/parser/recover-assoc-eq-missing-term.rs: too many trailing newlines (2)
2020-03-22T05:34:39.7753118Z tidy error: /checkout/src/test/ui/parser/recover-assoc-lifetime-constraint.rs: too many trailing newlines (2)
2020-03-22T05:34:40.6680940Z tidy error: /checkout/src/test/ui/suggestions/suggest-move-types.rs: ignoring line length unnecessarily
2020-03-22T05:34:46.8101900Z some tidy checks failed
2020-03-22T05:34:46.8102740Z Found 489 error codes
2020-03-22T05:34:46.8103463Z Found 0 error codes with no tests
2020-03-22T05:34:46.8103915Z Done!
2020-03-22T05:34:46.8103915Z Done!
2020-03-22T05:34:46.8109352Z 
2020-03-22T05:34:46.8109593Z 
2020-03-22T05:34:46.8111638Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo"
2020-03-22T05:34:46.8112997Z 
2020-03-22T05:34:46.8113194Z 
2020-03-22T05:34:46.8119169Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-03-22T05:34:46.8119528Z Build completed unsuccessfully in 0:00:29
2020-03-22T05:34:46.8119528Z Build completed unsuccessfully in 0:00:29
2020-03-22T05:34:46.8170680Z == clock drift check ==
2020-03-22T05:34:46.8185738Z   local time: Sun Mar 22 05:34:46 UTC 2020
2020-03-22T05:34:47.1080495Z   network time: Sun, 22 Mar 2020 05:34:47 GMT
2020-03-22T05:34:47.1087712Z == end clock drift check ==
2020-03-22T05:34:48.8331054Z 
2020-03-22T05:34:48.8398646Z ##[error]Bash exited with code '1'.
2020-03-22T05:34:48.8411225Z ##[section]Finishing: Run build
2020-03-22T05:34:48.8453587Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70261/merge to s
2020-03-22T05:34:48.8458162Z Task         : Get sources
2020-03-22T05:34:48.8458493Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-22T05:34:48.8458798Z Version      : 1.0.0
2020-03-22T05:34:48.8459028Z Author       : Microsoft
2020-03-22T05:34:48.8459028Z Author       : Microsoft
2020-03-22T05:34:48.8459362Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-22T05:34:48.8459752Z ==============================================================================
2020-03-22T05:34:49.1386810Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-22T05:34:49.1437113Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70261/merge to s
2020-03-22T05:34:49.1512005Z Cleaning up task key
2020-03-22T05:34:49.1513036Z Start cleaning up orphan processes.
2020-03-22T05:34:49.1666852Z Terminate orphan process: pid (4126) (python)
2020-03-22T05:34:49.1804249Z ##[section]Finishing: Finalize Job
