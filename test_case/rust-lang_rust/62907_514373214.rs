plain
2019-07-23T20:03:48.6651520Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-23T20:03:48.6833494Z ##[command]git config gc.auto 0
2019-07-23T20:03:48.6894760Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-23T20:03:48.6940819Z ##[command]git config --get-all http.proxy
2019-07-23T20:03:48.7081119Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62907/merge:refs/remotes/pull/62907/merge
---
2019-07-23T20:04:23.9258587Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-23T20:04:23.9258629Z 
2019-07-23T20:04:23.9258833Z   git checkout -b <new-branch-name>
2019-07-23T20:04:23.9258860Z 
2019-07-23T20:04:23.9258901Z HEAD is now at 0eab39b77 Merge 05be55b1442842a63fe00a3229e1258a87689efa into 299ef86e1f8b3e53154f834115752c719b611fa1
2019-07-23T20:04:23.9392732Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-23T20:04:23.9395525Z ==============================================================================
2019-07-23T20:04:23.9395583Z Task         : Bash
2019-07-23T20:04:23.9395648Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-23T20:33:00.5705594Z    Compiling tempfile v3.0.5
2019-07-23T20:34:03.0118175Z     Finished release [optimized] target(s) in 1m 24s
2019-07-23T20:34:03.0195221Z Assembling stage1 compiler (x86_64-unknown-linux-gnu)
2019-07-23T20:34:03.0221311Z Building stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-07-23T20:34:03.2305427Z error: process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc -vV` (exit code: 1)
2019-07-23T20:34:03.2305805Z --- stdout
2019-07-23T20:34:03.2306024Z rustc 1.38.0-dev
2019-07-23T20:34:03.2306069Z binary: rustc
2019-07-23T20:34:03.2306268Z commit-hash: unknown
2019-07-23T20:34:03.2306447Z commit-date: unknown
2019-07-23T20:34:03.2306643Z host: x86_64-unknown-linux-gnu
2019-07-23T20:34:03.2306840Z release: 1.38.0-dev
2019-07-23T20:34:03.2307041Z --- stderr
2019-07-23T20:34:03.2307041Z --- stderr
2019-07-23T20:34:03.2307538Z error: couldn't load codegen backend "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so": "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends/librustc_codegen_llvm-llvm.so: undefined symbol: LLVMInitializeMSP430AsmParser"
2019-07-23T20:34:03.2307630Z 
2019-07-23T20:34:03.2326671Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
2019-07-23T20:34:03.2326945Z expected success, got: exit code: 101
2019-07-23T20:34:03.2338392Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-07-23T20:34:03.2338392Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-07-23T20:34:03.2338469Z Build completed unsuccessfully in 0:23:12
2019-07-23T20:34:03.8878156Z ##[error]Bash exited with code '1'.
2019-07-23T20:34:03.8914723Z ##[section]Starting: Checkout
2019-07-23T20:34:03.8916249Z ==============================================================================
2019-07-23T20:34:03.8916313Z Task         : Get sources
2019-07-23T20:34:03.8916355Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
