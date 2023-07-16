plain
2020-01-23T21:45:59.2798788Z    Compiling unicode-normalization v0.1.11
2020-01-23T21:45:59.4941383Z error: failed to run custom build command for `byteorder v1.3.2`
2020-01-23T21:45:59.4941579Z 
2020-01-23T21:45:59.4941695Z Caused by:
2020-01-23T21:45:59.4942193Z   process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/byteorder-4f81ecbabe171043/build-script-build` (signal: 11, SIGSEGV: invalid memory reference)
2020-01-23T21:46:05.7722794Z error: build failed
2020-01-23T21:46:05.7750220Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "jemalloc llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-01-23T21:46:05.7750722Z expected success, got: exit code: 101
2020-01-23T21:46:05.7765254Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
2020-01-23T21:46:05.7765254Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
2020-01-23T21:46:05.7765646Z Build completed unsuccessfully in 0:56:24
2020-01-23T21:46:05.7818737Z == clock drift check ==
2020-01-23T21:46:05.7834667Z   local time: Thu Jan 23 21:46:05 UTC 2020
2020-01-23T21:46:06.4993129Z   network time: Thu, 23 Jan 2020 21:46:06 GMT
2020-01-23T21:46:06.4994010Z == end clock drift check ==
2020-01-23T21:46:07.9003668Z 
2020-01-23T21:46:07.9092385Z ##[error]Bash exited with code '1'.
2020-01-23T21:46:07.9129154Z ##[section]Starting: Checkout rust-lang/rust@try to s
2020-01-23T21:46:07.9131215Z ==============================================================================
2020-01-23T21:46:07.9131324Z Task         : Get sources
2020-01-23T21:46:07.9131413Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
