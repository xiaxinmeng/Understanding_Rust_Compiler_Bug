plain
2020-03-23T12:25:47.0472843Z ========================== Starting Command Output ===========================
2020-03-23T12:25:47.0477542Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/8dbc1315-e5e7-425a-8d40-4ce884e5136a.sh
2020-03-23T12:25:47.0478026Z 
2020-03-23T12:25:47.0482766Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-23T12:25:47.0509703Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69981/merge to s
2020-03-23T12:25:47.0512955Z Task         : Get sources
2020-03-23T12:25:47.0513267Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-23T12:25:47.0513546Z Version      : 1.0.0
2020-03-23T12:25:47.0513738Z Author       : Microsoft
---
2020-03-23T12:25:48.0452419Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-23T12:25:48.0458925Z ##[command]git config gc.auto 0
2020-03-23T12:25:48.0462905Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-23T12:25:48.0466747Z ##[command]git config --get-all http.proxy
2020-03-23T12:25:48.0473745Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69981/merge:refs/remotes/pull/69981/merge
---
2020-03-23T12:34:40.8139166Z configure: build.locked-deps    := True
2020-03-23T12:34:40.8139664Z configure: llvm.ccache          := sccache
2020-03-23T12:34:40.8140371Z configure: build.cargo-native-static := True
2020-03-23T12:34:40.8141129Z configure: dist.missing-tools   := True
2020-03-23T12:34:40.8141991Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-03-23T12:34:40.8144398Z configure: writing `config.toml` in current directory
2020-03-23T12:34:40.8144712Z configure: 
2020-03-23T12:34:40.8145378Z configure: run `python /checkout/x.py --help`
2020-03-23T12:34:40.8145620Z configure: 
---
2020-03-23T12:42:43.0483221Z    Compiling cargo_metadata v0.9.1
2020-03-23T12:42:48.0181761Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2020-03-23T12:42:59.4924909Z     Finished release [optimized] target(s) in 27.39s
2020-03-23T12:42:59.4963297Z tidy check
2020-03-23T12:43:04.8730457Z tidy error: /checkout/src/librustc/mir/mod.rs: too many lines (3007) (add `// ignore-tidy-filelength` to the file to suppress this error)
2020-03-23T12:43:08.7467185Z some tidy checks failed
2020-03-23T12:43:08.7467446Z Found 489 error codes
2020-03-23T12:43:08.7467664Z Found 0 error codes with no tests
2020-03-23T12:43:08.7467881Z Done!
2020-03-23T12:43:08.7467881Z Done!
2020-03-23T12:43:08.7468010Z 
2020-03-23T12:43:08.7468111Z 
2020-03-23T12:43:08.7469404Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo"
2020-03-23T12:43:08.7470167Z 
2020-03-23T12:43:08.7470268Z 
2020-03-23T12:43:08.7474823Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-03-23T12:43:08.7475418Z Build completed unsuccessfully in 0:00:38
2020-03-23T12:43:08.7475418Z Build completed unsuccessfully in 0:00:38
2020-03-23T12:43:08.7520922Z == clock drift check ==
2020-03-23T12:43:08.7547334Z   local time: Mon Mar 23 12:43:08 UTC 2020
2020-03-23T12:43:08.9071036Z   network time: Mon, 23 Mar 2020 12:43:08 GMT
2020-03-23T12:43:08.9074488Z == end clock drift check ==
2020-03-23T12:43:10.4868926Z 
2020-03-23T12:43:10.4932679Z ##[error]Bash exited with code '1'.
2020-03-23T12:43:10.4949276Z ##[section]Finishing: Run build
2020-03-23T12:43:10.5002645Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69981/merge to s
2020-03-23T12:43:10.5008098Z Task         : Get sources
2020-03-23T12:43:10.5008520Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-23T12:43:10.5008878Z Version      : 1.0.0
2020-03-23T12:43:10.5009139Z Author       : Microsoft
2020-03-23T12:43:10.5009139Z Author       : Microsoft
2020-03-23T12:43:10.5009568Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-23T12:43:10.5010036Z ==============================================================================
2020-03-23T12:43:10.8791916Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-23T12:43:10.8845437Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69981/merge to s
2020-03-23T12:43:10.8942936Z Cleaning up task key
2020-03-23T12:43:10.8944245Z Start cleaning up orphan processes.
2020-03-23T12:43:10.9148373Z Terminate orphan process: pid (3458) (python)
2020-03-23T12:43:10.9314236Z ##[section]Finishing: Finalize Job
