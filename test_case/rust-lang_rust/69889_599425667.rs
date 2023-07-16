plain
2020-03-16T08:49:15.2185797Z ========================== Starting Command Output ===========================
2020-03-16T08:49:15.2188176Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/44443eb7-ef63-4d80-a650-aab76c1d6b80.sh
2020-03-16T08:49:15.2188463Z 
2020-03-16T08:49:15.2192056Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-16T08:49:15.2213744Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69889/merge to s
2020-03-16T08:49:15.2217111Z Task         : Get sources
2020-03-16T08:49:15.2217435Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-16T08:49:15.2217733Z Version      : 1.0.0
2020-03-16T08:49:15.2217933Z Author       : Microsoft
---
2020-03-16T08:49:16.2183748Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-16T08:49:16.2191037Z ##[command]git config gc.auto 0
2020-03-16T08:49:16.2196074Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-16T08:49:16.2200786Z ##[command]git config --get-all http.proxy
2020-03-16T08:49:16.2208987Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69889/merge:refs/remotes/pull/69889/merge
---
2020-03-16T09:14:04.2412232Z Building stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-16T09:14:04.6190954Z error: failed to run `rustc` to learn about target-specific information
2020-03-16T09:14:04.6191426Z 
2020-03-16T09:14:04.6191617Z Caused by:
2020-03-16T09:14:04.6192983Z   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc - --crate-name ___ --print=file-names -Zmacro-backtrace '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Wrust_2018_idioms -Wunused_lifetimes -Dwarnings -Cprefer-dynamic -Cllvm-args=-import-instr-limit=10 --target x86_64-unknown-linux-gnu --crate-type bin --crate-type rlib --crate-type dylib --crate-type cdylib --crate-type staticlib --crate-type proc-macro --print=sysroot --print=cfg` (signal: 4, SIGILL: illegal instruction)
2020-03-16T09:14:04.6206886Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-03-16T09:14:04.6214887Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-03-16T09:14:04.6215316Z Build completed unsuccessfully in 0:19:36
2020-03-16T09:14:04.6280425Z == clock drift check ==
2020-03-16T09:14:04.6295833Z   local time: Mon Mar 16 09:14:04 UTC 2020
2020-03-16T09:14:04.6295833Z   local time: Mon Mar 16 09:14:04 UTC 2020
2020-03-16T09:14:04.7968636Z   network time: Mon, 16 Mar 2020 09:14:04 GMT
2020-03-16T09:14:04.7971603Z == end clock drift check ==
2020-03-16T09:14:06.2262689Z 
2020-03-16T09:14:06.2338310Z ##[error]Bash exited with code '1'.
2020-03-16T09:14:06.2350701Z ##[section]Finishing: Run build
2020-03-16T09:14:06.2397099Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69889/merge to s
2020-03-16T09:14:06.2401702Z Task         : Get sources
2020-03-16T09:14:06.2401985Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-16T09:14:06.2402452Z Version      : 1.0.0
2020-03-16T09:14:06.2402667Z Author       : Microsoft
2020-03-16T09:14:06.2402667Z Author       : Microsoft
2020-03-16T09:14:06.2402998Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-16T09:14:06.2403396Z ==============================================================================
2020-03-16T09:14:06.5165388Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-16T09:14:06.5225394Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69889/merge to s
2020-03-16T09:14:06.5337404Z Cleaning up task key
2020-03-16T09:14:06.5338486Z Start cleaning up orphan processes.
2020-03-16T09:14:06.5493960Z Terminate orphan process: pid (3920) (python)
2020-03-16T09:14:06.5635494Z ##[section]Finishing: Finalize Job
