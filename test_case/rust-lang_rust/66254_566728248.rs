
2019-12-17T20:01:59.7352322Z Checking core v0.0.0 (/checkout/src/libcore)
2019-12-17T20:02:01.7389052Z error: cannot find attribute `rustc_const_stable` in this scope
2019-12-17T20:02:01.7389777Z --> src/libcore/alloc.rs:121:7
2019-12-17T20:02:01.7390006Z |
2019-12-17T20:02:01.7390949Z 121 | #[rustc_const_stable(feature = "alloc_layout_const_new", since = "1.42.0")]
2019-12-17T20:02:01.7391447Z | ^^^^^^^^^^^^^^^^^^ help: a built-in attribute with a similar name exists: `rustc_const_unstable`
2019-12-17T20:02:01.7391507Z
2019-12-17T20:02:08.4887652Z Compiling build_helper v0.1.0 (/checkout/src/build_helper)
2019-12-17T20:02:10.2096154Z Compiling libc v0.2.64
2019-12-17T20:02:11.2222092Z Compiling autocfg v0.1.6
2019-12-17T20:02:11.2454741Z error: aborting due to previous error
2019-12-17T20:02:11.2454992Z
2019-12-17T20:02:11.3434631Z error: could not compile `core`.
2019-12-17T20:02:11.3435004Z warning: build failed, waiting for other jobs to finish...
2019-12-17T20:02:12.3055077Z error: build failed
2019-12-17T20:02:12.3086094Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "--message-format" "json-render-diagnostics"
2019-12-17T20:02:12.3086287Z expected success, got: exit code: 101
2019-12-17T20:02:12.3090828Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-12-17T20:02:12.3090957Z Build completed unsuccessfully in 0:01:53
2019-12-17T20:02:12.3150063Z == clock drift check ==
2019-12-17T20:02:12.3163052Z local time: Tue Dec 17 20:02:12 UTC 2019
2019-12-17T20:02:12.5950495Z network time: Tue, 17 Dec 2019 20:02:12 GMT
2019-12-17T20:02:12.5954233Z == end clock drift check ==
2019-12-17T20:02:27.5891711Z
2019-12-17T20:02:27.6003893Z ##[error]Bash exited with code '1'.
