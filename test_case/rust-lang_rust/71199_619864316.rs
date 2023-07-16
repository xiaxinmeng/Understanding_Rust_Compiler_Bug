plain
2020-04-27T08:59:18.5767199Z ========================== Starting Command Output ===========================
2020-04-27T08:59:18.5769587Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/054788df-a1a7-4807-a43c-aff14e9483de.sh
2020-04-27T08:59:18.5769830Z 
2020-04-27T08:59:18.5773968Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-27T08:59:18.5791493Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71199/merge to s
2020-04-27T08:59:18.5794624Z Task         : Get sources
2020-04-27T08:59:18.5794880Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-27T08:59:18.5795129Z Version      : 1.0.0
2020-04-27T08:59:18.5795298Z Author       : Microsoft
---
2020-04-27T08:59:19.5733312Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-27T08:59:19.5739846Z ##[command]git config gc.auto 0
2020-04-27T08:59:19.5744278Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-27T08:59:19.5748635Z ##[command]git config --get-all http.proxy
2020-04-27T08:59:19.5756597Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71199/merge:refs/remotes/pull/71199/merge
---
2020-04-27T09:01:37.4661447Z  ---> cb2676f08729
2020-04-27T09:01:37.4662791Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-27T09:01:37.4669847Z  ---> Using cache
2020-04-27T09:01:37.4670165Z  ---> df25ce111862
2020-04-27T09:01:37.4671024Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-27T09:01:37.4676403Z  ---> 599b9ac96b27
2020-04-27T09:01:37.4676616Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-27T09:01:37.4681049Z  ---> Using cache
2020-04-27T09:01:37.4681388Z  ---> 091087e35a36
---
2020-04-27T09:01:37.5027892Z Looks like docker image is the same as before, not uploading
2020-04-27T09:01:44.0272569Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-27T09:01:44.0567905Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-27T09:01:44.0598426Z == clock drift check ==
2020-04-27T09:01:44.0609342Z   local time: Mon Apr 27 09:01:44 UTC 2020
2020-04-27T09:01:44.3537453Z   network time: Mon, 27 Apr 2020 09:01:44 GMT
2020-04-27T09:01:44.3563954Z Starting sccache server...
2020-04-27T09:01:44.4390294Z configure: processing command line
2020-04-27T09:01:44.4390531Z configure: 
2020-04-27T09:01:44.4391369Z configure: rust.dist-src        := False
---
2020-04-27T09:03:59.9405334Z    Compiling proc_macro v0.0.0 (/checkout/src/libproc_macro)
2020-04-27T09:04:03.4735566Z    Compiling unicode-width v0.1.6
2020-04-27T09:04:03.5557704Z    Compiling getopts v0.2.21
2020-04-27T09:04:12.9294795Z    Compiling test v0.0.0 (/checkout/src/libtest)
2020-04-27T09:04:20.4301559Z {"reason":"build-finished","success":true}
2020-04-27T09:04:20.4419698Z Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2020-04-27T09:04:20.4534380Z Building stage0 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-27T09:04:20.9922720Z    Compiling cfg-if v0.1.10
2020-04-27T09:04:20.9923616Z    Compiling libc v0.2.69
---
2020-04-27T09:06:37.5695091Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-27T09:06:38.8442614Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-27T09:06:40.2529375Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-27T09:06:41.3004952Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-27T09:06:49.1362144Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-27T09:06:51.4814637Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-27T09:06:55.4615443Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-27T09:06:59.2375028Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-27T09:07:07.4363089Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-27T09:21:02.9159936Z    Compiling rustc_privacy v0.0.0 (/checkout/src/librustc_privacy)
2020-04-27T09:21:09.9166641Z    Compiling rustc_plugin_impl v0.0.0 (/checkout/src/librustc_plugin_impl)
2020-04-27T09:23:22.3555754Z    Compiling rustc-main v0.0.0 (/checkout/src/rustc)
2020-04-27T09:23:22.9239379Z     Finished release [optimized] target(s) in 19m 02s
2020-04-27T09:23:22.9239946Z {"reason":"build-finished","success":true}
2020-04-27T09:23:22.9725347Z Assembling stage1 compiler (x86_64-unknown-linux-gnu)
2020-04-27T09:23:22.9745316Z Building stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-27T09:23:23.2409929Z    Compiling cc v1.0.50
2020-04-27T09:23:23.2410718Z    Compiling core v0.0.0 (/checkout/src/libcore)
2020-04-27T09:23:23.2410718Z    Compiling core v0.0.0 (/checkout/src/libcore)
2020-04-27T09:23:26.5791657Z error: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-27T09:23:26.5792877Z   --> src/libcore/../stdarch/crates/core_arch/src/x86/eflags.rs:33:5
2020-04-27T09:23:26.5793556Z    |
2020-04-27T09:23:26.5794299Z 33 |     asm!("pushfq; popq $0" : "=r"(eflags) : : : "volatile");
2020-04-27T09:23:26.5795310Z    |     ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-27T09:23:26.5829746Z    = note: `-D deprecated` implied by `-D warnings`
2020-04-27T09:23:26.5829937Z 
2020-04-27T09:23:26.5829937Z 
2020-04-27T09:23:26.5830370Z error: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-27T09:23:26.5830908Z   --> src/libcore/../stdarch/crates/core_arch/src/x86/eflags.rs:64:5
2020-04-27T09:23:26.5831393Z    |
2020-04-27T09:23:26.5831889Z 64 |     asm!("pushq $0; popfq" : : "r"(eflags) : "cc", "flags" : "volatile");
2020-04-27T09:23:26.5832567Z    |     ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-27T09:23:26.5839895Z 
2020-04-27T09:23:26.5841685Z error: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-27T09:23:26.5842406Z   --> src/libcore/../stdarch/crates/core_arch/src/x86/cpuid.rs:68:9
2020-04-27T09:23:26.5843033Z    |
2020-04-27T09:23:26.5843557Z 68 |         asm!("cpuid"
2020-04-27T09:23:26.5844390Z    |         ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-27T09:23:26.5844789Z 
2020-04-27T09:23:26.5845855Z error: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-27T09:23:26.5846599Z   --> src/libcore/../stdarch/crates/core_arch/src/x86/xsave.rs:90:5
2020-04-27T09:23:26.5847100Z    |
2020-04-27T09:23:26.5847784Z 90 |     asm!("xgetbv" : "={eax}"(eax), "={edx}"(edx) : "{ecx}"(xcr_no));
2020-04-27T09:23:26.5848818Z    |     ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-27T09:23:26.5849206Z 
2020-04-27T09:23:26.5849833Z error: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-27T09:23:26.5850559Z   --> src/libcore/../stdarch/crates/core_arch/src/x86/bt.rs:10:5
2020-04-27T09:23:26.5851039Z    |
2020-04-27T09:23:26.5851640Z 10 |     asm!("btl $2, $1\n\tsetc ${0:b}"
2020-04-27T09:23:26.5852510Z    |     ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-27T09:23:26.5852897Z 
2020-04-27T09:23:26.5896771Z error: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-27T09:23:26.5897697Z   --> src/libcore/../stdarch/crates/core_arch/src/x86/bt.rs:23:5
2020-04-27T09:23:26.5898190Z    |
2020-04-27T09:23:26.5898758Z 23 |     asm!("btsl $2, $1\n\tsetc ${0:b}"
2020-04-27T09:23:26.5899581Z    |     ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-27T09:23:26.5899957Z 
2020-04-27T09:23:26.5900518Z error: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-27T09:23:26.5901344Z   --> src/libcore/../stdarch/crates/core_arch/src/x86/bt.rs:36:5
2020-04-27T09:23:26.5901803Z    |
2020-04-27T09:23:26.5902343Z 36 |     asm!("btrl $2, $1\n\tsetc ${0:b}"
2020-04-27T09:23:26.5903151Z    |     ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-27T09:23:26.5903528Z 
2020-04-27T09:23:26.5904148Z error: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-27T09:23:26.5904914Z   --> src/libcore/../stdarch/crates/core_arch/src/x86/bt.rs:49:5
2020-04-27T09:23:26.5905318Z    |
2020-04-27T09:23:26.5905787Z 49 |     asm!("btcl $2, $1\n\tsetc ${0:b}"
2020-04-27T09:23:26.5906495Z    |     ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-27T09:23:26.5906819Z 
2020-04-27T09:23:26.5907303Z error: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-27T09:23:26.5907892Z   --> src/libcore/../stdarch/crates/core_arch/src/x86_64/bt.rs:10:5
2020-04-27T09:23:26.5908297Z    |
2020-04-27T09:23:26.5908767Z 10 |     asm!("btq $2, $1\n\tsetc ${0:b}"
2020-04-27T09:23:26.5909469Z    |     ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-27T09:23:26.5909976Z 
2020-04-27T09:23:26.5910535Z error: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-27T09:23:26.5911426Z   --> src/libcore/../stdarch/crates/core_arch/src/x86_64/bt.rs:23:5
2020-04-27T09:23:26.5911929Z    |
2020-04-27T09:23:26.5912508Z 23 |     asm!("btsq $2, $1\n\tsetc ${0:b}"
2020-04-27T09:23:26.5913380Z    |     ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-27T09:23:26.5913785Z 
2020-04-27T09:23:26.5915294Z error: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-27T09:23:26.5916032Z   --> src/libcore/../stdarch/crates/core_arch/src/x86_64/bt.rs:36:5
2020-04-27T09:23:26.5916535Z    |
2020-04-27T09:23:26.5917123Z 36 |     asm!("btrq $2, $1\n\tsetc ${0:b}"
2020-04-27T09:23:26.5917990Z    |     ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-27T09:23:26.5918390Z 
2020-04-27T09:23:26.5918990Z error: use of deprecated item 'asm': the syntax of asm! will change soon, use llvm_asm! to avoid breakage
2020-04-27T09:23:26.5919724Z   --> src/libcore/../stdarch/crates/core_arch/src/x86_64/bt.rs:49:5
2020-04-27T09:23:26.5920225Z    |
2020-04-27T09:23:26.5920805Z 49 |     asm!("btcq $2, $1\n\tsetc ${0:b}"
2020-04-27T09:23:26.5921690Z    |     ^^^ help: replace the use of the deprecated item: `llvm_asm`
2020-04-27T09:23:28.0769794Z    Compiling libc v0.2.69
2020-04-27T09:23:28.6027050Z    Compiling autocfg v0.1.7
2020-04-27T09:23:29.7099597Z    Compiling std v0.0.0 (/checkout/src/libstd)
2020-04-27T09:23:30.0554431Z    Compiling compiler_builtins v0.1.27
---
2020-04-27T09:23:34.1709467Z error: could not compile `core`.
2020-04-27T09:23:34.1709973Z 
2020-04-27T09:23:34.1710951Z To learn more, run the command again with --verbose.
2020-04-27T09:23:34.1711501Z warning: build failed, waiting for other jobs to finish...
2020-04-27T09:23:38.3548297Z {"reason":"build-finished","success":false}
2020-04-27T09:23:38.3638774Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-04-27T09:23:38.3639636Z expected success, got: exit code: 101
2020-04-27T09:23:38.3651716Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-27T09:23:38.3652109Z Build completed unsuccessfully in 0:20:14
2020-04-27T09:23:38.3652109Z Build completed unsuccessfully in 0:20:14
2020-04-27T09:23:38.3704351Z == clock drift check ==
2020-04-27T09:23:38.3730270Z   local time: Mon Apr 27 09:23:38 UTC 2020
2020-04-27T09:23:38.6716985Z   network time: Mon, 27 Apr 2020 09:23:38 GMT
2020-04-27T09:23:40.3037001Z 
2020-04-27T09:23:40.3037001Z 
2020-04-27T09:23:40.3098242Z ##[error]Bash exited with code '1'.
2020-04-27T09:23:40.3111731Z ##[section]Finishing: Run build
2020-04-27T09:23:40.3159572Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71199/merge to s
2020-04-27T09:23:40.3164255Z Task         : Get sources
2020-04-27T09:23:40.3164634Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-27T09:23:40.3165063Z Version      : 1.0.0
2020-04-27T09:23:40.3165255Z Author       : Microsoft
2020-04-27T09:23:40.3165255Z Author       : Microsoft
2020-04-27T09:23:40.3165551Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-27T09:23:40.3165998Z ==============================================================================
2020-04-27T09:23:40.6222678Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-27T09:23:40.6265218Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71199/merge to s
2020-04-27T09:23:40.6359549Z Cleaning up task key
2020-04-27T09:23:40.6361256Z Start cleaning up orphan processes.
2020-04-27T09:23:40.6555526Z Terminate orphan process: pid (4045) (python)
2020-04-27T09:23:40.6706880Z ##[section]Finishing: Finalize Job
