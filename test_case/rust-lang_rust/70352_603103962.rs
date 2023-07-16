plain
2020-03-24T07:40:04.2294866Z ========================== Starting Command Output ===========================
2020-03-24T07:40:04.2299061Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/86db3308-5b8d-4b79-9990-6350ab90be86.sh
2020-03-24T07:40:04.2299278Z 
2020-03-24T07:40:04.2302524Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-24T07:40:04.2318437Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70352/merge to s
2020-03-24T07:40:04.2322992Z Task         : Get sources
2020-03-24T07:40:04.2323420Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-24T07:40:04.2324056Z Version      : 1.0.0
2020-03-24T07:40:04.2324213Z Author       : Microsoft
---
2020-03-24T07:40:05.5379202Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-24T07:40:05.5383478Z ##[command]git config gc.auto 0
2020-03-24T07:40:05.5386412Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-24T07:40:05.5389007Z ##[command]git config --get-all http.proxy
2020-03-24T07:40:05.5394094Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70352/merge:refs/remotes/pull/70352/merge
---
2020-03-24T07:45:59.4623439Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-24T07:45:59.8599948Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-24T07:46:01.6739526Z     Checking rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
2020-03-24T07:46:02.2677471Z     Checking rustc_expand v0.0.0 (/checkout/src/librustc_expand)
2020-03-24T07:46:02.3726779Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-24T07:46:03.8439921Z     Checking rustc_builtin_macros v0.0.0 (/checkout/src/librustc_builtin_macros)
2020-03-24T07:46:22.0519880Z     Checking rustc_infer v0.0.0 (/checkout/src/librustc_infer)
2020-03-24T07:46:24.5971413Z     Checking rustc_save_analysis v0.0.0 (/checkout/src/librustc_save_analysis)
2020-03-24T07:46:25.2218592Z     Checking rustc_symbol_mangling v0.0.0 (/checkout/src/librustc_symbol_mangling)
---
2020-03-24T07:47:36.5174053Z configure: build.locked-deps    := True
2020-03-24T07:47:36.5174473Z configure: llvm.ccache          := sccache
2020-03-24T07:47:36.5174871Z configure: build.cargo-native-static := True
2020-03-24T07:47:36.5175244Z configure: dist.missing-tools   := True
2020-03-24T07:47:36.5175714Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-03-24T07:47:36.5176182Z configure: writing `config.toml` in current directory
2020-03-24T07:47:36.5176375Z configure: 
2020-03-24T07:47:36.5176710Z configure: run `python /checkout/x.py --help`
2020-03-24T07:47:36.5176894Z configure: 
---
2020-03-24T07:50:37.4373280Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-24T07:50:37.8383384Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-24T07:50:39.6080443Z     Checking rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
2020-03-24T07:50:40.2485280Z     Checking rustc_expand v0.0.0 (/checkout/src/librustc_expand)
2020-03-24T07:50:40.3803275Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-24T07:50:41.8833617Z     Checking rustc_builtin_macros v0.0.0 (/checkout/src/librustc_builtin_macros)
2020-03-24T07:50:59.6428321Z     Checking rustc_infer v0.0.0 (/checkout/src/librustc_infer)
2020-03-24T07:51:02.2072549Z     Checking rustc_save_analysis v0.0.0 (/checkout/src/librustc_save_analysis)
2020-03-24T07:51:02.2906129Z     Checking rustc_symbol_mangling v0.0.0 (/checkout/src/librustc_symbol_mangling)
---
2020-03-24T07:53:52.0305544Z    Compiling cargo_metadata v0.9.1
2020-03-24T07:53:55.2320969Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2020-03-24T07:54:04.5476483Z     Finished release [optimized] target(s) in 21.06s
2020-03-24T07:54:04.5539185Z tidy check
2020-03-24T07:54:10.4654515Z tidy error: /checkout/src/librustc_error_codes/error_codes/E0710.md:6: line longer than 80 chars
2020-03-24T07:54:10.4655316Z tidy error: /checkout/src/librustc_error_codes/error_codes/E0710.md: missing trailing newline
2020-03-24T07:54:14.4830686Z Found 489 error codes
2020-03-24T07:54:14.4830955Z Found 0 error codes with no tests
2020-03-24T07:54:14.4831094Z Done!
2020-03-24T07:54:14.4831219Z some tidy checks failed
2020-03-24T07:54:14.4831219Z some tidy checks failed
2020-03-24T07:54:14.4831326Z 
2020-03-24T07:54:14.4831420Z 
2020-03-24T07:54:14.4833078Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo"
2020-03-24T07:54:14.4833734Z 
2020-03-24T07:54:14.4833814Z 
2020-03-24T07:54:14.4845302Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-03-24T07:54:14.4845587Z Build completed unsuccessfully in 0:00:32
2020-03-24T07:54:14.4845587Z Build completed unsuccessfully in 0:00:32
2020-03-24T07:54:14.4887422Z == clock drift check ==
2020-03-24T07:54:14.4915332Z   local time: Tue Mar 24 07:54:14 UTC 2020
2020-03-24T07:54:14.6537094Z   network time: Tue, 24 Mar 2020 07:54:14 GMT
2020-03-24T07:54:14.6537397Z == end clock drift check ==
2020-03-24T07:54:16.3648870Z 
2020-03-24T07:54:16.3721171Z ##[error]Bash exited with code '1'.
2020-03-24T07:54:16.3733309Z ##[section]Finishing: Run build
2020-03-24T07:54:16.3779692Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70352/merge to s
2020-03-24T07:54:16.3783895Z Task         : Get sources
2020-03-24T07:54:16.3784162Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-24T07:54:16.3784601Z Version      : 1.0.0
2020-03-24T07:54:16.3784769Z Author       : Microsoft
2020-03-24T07:54:16.3784769Z Author       : Microsoft
2020-03-24T07:54:16.3785037Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-24T07:54:16.3785362Z ==============================================================================
2020-03-24T07:54:16.6682188Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-24T07:54:16.6724273Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70352/merge to s
2020-03-24T07:54:16.6791324Z Cleaning up task key
2020-03-24T07:54:16.6792317Z Start cleaning up orphan processes.
2020-03-24T07:54:16.7140778Z Terminate orphan process: pid (4527) (python)
2020-03-24T07:54:16.7178488Z ##[section]Finishing: Finalize Job
