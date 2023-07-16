plain
2020-04-17T00:10:15.4055296Z ========================== Starting Command Output ===========================
2020-04-17T00:10:15.4059410Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/1500072e-41eb-4245-bcb8-42e46ae4819e.sh
2020-04-17T00:10:15.4059785Z 
2020-04-17T00:10:15.4063789Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-17T00:10:15.4079656Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71228/merge to s
2020-04-17T00:10:15.4082517Z Task         : Get sources
2020-04-17T00:10:15.4082753Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-17T00:10:15.4082982Z Version      : 1.0.0
2020-04-17T00:10:15.4083157Z Author       : Microsoft
---
2020-04-17T00:10:16.6228258Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-17T00:10:16.6739995Z ##[command]git config gc.auto 0
2020-04-17T00:10:16.6750179Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-17T00:10:16.6756426Z ##[command]git config --get-all http.proxy
2020-04-17T00:10:16.6767387Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71228/merge:refs/remotes/pull/71228/merge
---
2020-04-17T00:12:36.7231756Z  ---> f58a2bb1e753
2020-04-17T00:12:36.7232602Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-7       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-17T00:12:36.7233255Z  ---> Using cache
2020-04-17T00:12:36.7233642Z  ---> d079cc6b6db8
2020-04-17T00:12:36.7234673Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-17T00:12:36.7235833Z  ---> 4183ca46ee56
2020-04-17T00:12:36.7236062Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-17T00:12:36.7236454Z  ---> Using cache
2020-04-17T00:12:36.7237239Z  ---> 69e7f8a2a2fb
---
2020-04-17T00:12:36.7653723Z Looks like docker image is the same as before, not uploading
2020-04-17T00:12:44.4459218Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-17T00:12:44.4748047Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-17T00:12:44.4775011Z == clock drift check ==
2020-04-17T00:12:44.4782492Z   local time: Fri Apr 17 00:12:44 UTC 2020
2020-04-17T00:12:44.5756925Z   network time: Fri, 17 Apr 2020 00:12:44 GMT
2020-04-17T00:12:44.5785221Z Starting sccache server...
2020-04-17T00:12:44.6794805Z configure: processing command line
2020-04-17T00:12:44.6795603Z configure: 
2020-04-17T00:12:44.6797185Z configure: rust.dist-src        := False
---
2020-04-17T00:17:57.8360174Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-17T00:17:59.2497351Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-17T00:18:00.7680111Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-17T00:18:02.2738159Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-17T00:18:10.7275243Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-17T00:18:14.0037014Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-17T00:18:18.3176265Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-17T00:18:22.6517996Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-17T00:18:31.2818439Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-17T00:36:38.4280730Z    Compiling core v0.0.0 (/checkout/src/libcore)
2020-04-17T00:36:38.6542507Z error: could not compile `core`.
2020-04-17T00:36:38.6542738Z 
2020-04-17T00:36:38.6542861Z Caused by:
2020-04-17T00:36:38.6545636Z   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core --edition=2018 src/libcore/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C codegen-units=1 -C debuginfo=0 -C metadata=bd09ac763afd23e9 -C extra-filename=-bd09ac763afd23e9 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps -Zmacro-backtrace '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Wrust_2018_idioms -Wunused_lifetimes -Dwarnings -Cprefer-dynamic -Cllvm-args=-import-instr-limit=10 -Zbinary-dep-depinfo` (signal: 11, SIGSEGV: invalid memory reference)
2020-04-17T00:36:43.8096985Z error: build failed
2020-04-17T00:36:43.8119221Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-04-17T00:36:43.8120381Z expected success, got: exit code: 101
2020-04-17T00:36:43.8133296Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-17T00:36:43.8133296Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-17T00:36:43.8133622Z Build completed unsuccessfully in 0:22:21
2020-04-17T00:36:43.8200136Z == clock drift check ==
2020-04-17T00:36:43.8222074Z   local time: Fri Apr 17 00:36:43 UTC 2020
2020-04-17T00:36:43.9489428Z   network time: Fri, 17 Apr 2020 00:36:43 GMT
2020-04-17T00:36:45.3543537Z 
2020-04-17T00:36:45.3543537Z 
2020-04-17T00:36:45.3653159Z ##[error]Bash exited with code '1'.
2020-04-17T00:36:45.3671915Z ##[section]Finishing: Run build
2020-04-17T00:36:45.3730106Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71228/merge to s
2020-04-17T00:36:45.3736313Z Task         : Get sources
2020-04-17T00:36:45.3738485Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-17T00:36:45.3739433Z Version      : 1.0.0
2020-04-17T00:36:45.3741078Z Author       : Microsoft
2020-04-17T00:36:45.3741078Z Author       : Microsoft
2020-04-17T00:36:45.3741487Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-17T00:36:45.3741945Z ==============================================================================
2020-04-17T00:36:45.7725227Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-17T00:36:45.7779998Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71228/merge to s
2020-04-17T00:36:45.7880731Z Cleaning up task key
2020-04-17T00:36:45.7882171Z Start cleaning up orphan processes.
2020-04-17T00:36:45.8136856Z Terminate orphan process: pid (3850) (python)
2020-04-17T00:36:45.8341338Z ##[section]Finishing: Finalize Job
