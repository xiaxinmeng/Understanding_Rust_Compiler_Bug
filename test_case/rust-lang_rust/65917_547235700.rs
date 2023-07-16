plain
2019-10-29T02:56:34.8428680Z test [ui] ui/lint/unused_parens_json_suggestion.rs ... ok
2019-10-29T02:56:34.9600530Z test [ui] ui/lint/use_suggestion_json.rs ... ok
2019-10-29T02:56:35.0109080Z test [ui] ui/lint/unused_parens_remove_json_suggestion.rs ... ok
2019-10-29T02:56:35.0362110Z test [ui] ui/lint/use-redundant.rs ... ok
2019-10-29T02:56:35.0617600Z test [ui] ui/lint/warn-unused-inline-on-fn-prototypes.rs ... ok
2019-10-29T02:56:35.2948500Z test [ui] ui/list.rs ... ok
2019-10-29T02:56:35.3048480Z test [ui] ui/liveness-assign-imm-local-after-ret.rs ... ok
2019-10-29T02:56:35.3150150Z test [ui] ui/liveness/liveness-assign/liveness-assign-imm-local-in-op-eq.rs ... ok
2019-10-29T02:56:35.4303850Z test [ui] ui/liveness/liveness-closure-require-ret.rs ... ok
---
2019-10-29T03:02:14.6186950Z diff of stderr:
2019-10-29T03:02:14.6187000Z 
2019-10-29T03:02:14.6187650Z 10   --> $DIR/raw-ptr-const-param.rs:7:38
2019-10-29T03:02:14.6188090Z 11    |
2019-10-29T03:02:14.6188200Z 12 LL |     let _: Const<{15 as *const _}> = Const::<{10 as *const _}>;
2019-10-29T03:02:14.6189250Z -    |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^ expected `0x000000000000000f : *const u32`, found `0x000000000000000a : *const u32`
2019-10-29T03:02:14.6189510Z +    |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^ expected `0x0000000f : *const u32`, found `0x0000000a : *const u32`
2019-10-29T03:02:14.6189650Z 14    |
2019-10-29T03:02:14.6190350Z -    = note: expected type `Const<0x000000000000000f : *const u32>`
2019-10-29T03:02:14.6191060Z -               found type `Const<0x000000000000000a : *const u32>`
2019-10-29T03:02:14.6191190Z +    = note: expected type `Const<0x0000000f : *const u32>`
2019-10-29T03:02:14.6191310Z +               found type `Const<0x0000000a : *const u32>`
2019-10-29T03:02:14.6191490Z 18 error: aborting due to previous error
2019-10-29T03:02:14.6191580Z 19 
2019-10-29T03:02:14.6191640Z 
2019-10-29T03:02:14.6191680Z 
2019-10-29T03:02:14.6191680Z 
2019-10-29T03:02:14.6191780Z The actual stderr differed from the expected stderr.
2019-10-29T03:02:14.6192590Z Actual stderr saved to /Users/runner/runners/2.159.2/work/1/s/build/i686-apple-darwin/test/ui/const-generics/raw-ptr-const-param/raw-ptr-const-param.stderr
2019-10-29T03:02:14.6193340Z To update references, rerun the tests and pass the `--bless` flag
2019-10-29T03:02:14.6194060Z To only update this specific test, also pass `--test-args const-generics/raw-ptr-const-param.rs`
2019-10-29T03:02:14.6194240Z error: 1 errors occurred comparing output.
2019-10-29T03:02:14.6194340Z status: exit code: 1
2019-10-29T03:02:14.6194340Z status: exit code: 1
2019-10-29T03:02:14.6195880Z command: "/Users/runner/runners/2.159.2/work/1/s/build/i686-apple-darwin/stage2/bin/rustc" "/Users/runner/runners/2.159.2/work/1/s/src/test/ui/const-generics/raw-ptr-const-param.rs" "-Zthreads=1" "--target=i686-apple-darwin" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/Users/runner/runners/2.159.2/work/1/s/build/i686-apple-darwin/test/ui/const-generics/raw-ptr-const-param" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/Users/runner/runners/2.159.2/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "-L" "/Users/runner/runners/2.159.2/work/1/s/build/i686-apple-darwin/test/ui/const-generics/raw-ptr-const-param/auxiliary" "-A" "unused"
2019-10-29T03:02:14.6196900Z ------------------------------------------
2019-10-29T03:02:14.6196990Z 
2019-10-29T03:02:14.6197610Z ------------------------------------------
2019-10-29T03:02:14.6197740Z stderr:
---
2019-10-29T03:02:14.6199810Z 
2019-10-29T03:02:14.6199890Z error[E0308]: mismatched types
2019-10-29T03:02:14.6200630Z   --> /Users/runner/runners/2.159.2/work/1/s/src/test/ui/const-generics/raw-ptr-const-param.rs:7:38
2019-10-29T03:02:14.6200760Z    |
2019-10-29T03:02:14.6200860Z LL |     let _: Const<{15 as *const _}> = Const::<{10 as *const _}>; //~ mismatched types
2019-10-29T03:02:14.6201010Z    |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^ expected `0x0000000f : *const u32`, found `0x0000000a : *const u32`
2019-10-29T03:02:14.6201130Z    |
2019-10-29T03:02:14.6201230Z    = note: expected type `Const<0x0000000f : *const u32>`
2019-10-29T03:02:14.6201330Z               found type `Const<0x0000000a : *const u32>`
2019-10-29T03:02:14.6201860Z error: aborting due to previous error
2019-10-29T03:02:14.6201940Z 
2019-10-29T03:02:14.6202640Z For more information about this error, try `rustc --explain E0308`.
2019-10-29T03:02:14.6202730Z 
---
2019-10-29T03:02:14.6289920Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-29T03:02:14.6290320Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-29T03:02:14.6307660Z 
2019-10-29T03:02:14.6307850Z 
2019-10-29T03:02:14.6320540Z command did not execute successfully: "/Users/runner/runners/2.159.2/work/1/s/build/i686-apple-darwin/stage0-tools-bin/compiletest" "--compile-lib-path" "/Users/runner/runners/2.159.2/work/1/s/build/i686-apple-darwin/stage2/lib" "--run-lib-path" "/Users/runner/runners/2.159.2/work/1/s/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib" "--rustc-path" "/Users/runner/runners/2.159.2/work/1/s/build/i686-apple-darwin/stage2/bin/rustc" "--src-base" "/Users/runner/runners/2.159.2/work/1/s/src/test/ui" "--build-base" "/Users/runner/runners/2.159.2/work/1/s/build/i686-apple-darwin/test/ui" "--stage-id" "stage2-i686-apple-darwin" "--mode" "ui" "--target" "i686-apple-darwin" "--host" "i686-apple-darwin" "--llvm-filecheck" "/Users/runner/runners/2.159.2/work/1/s/build/i686-apple-darwin/llvm/build/bin/FileCheck" "--nodejs" "/usr/local/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/runner/runners/2.159.2/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/runner/runners/2.159.2/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "--docck-python" "/usr/local/bin/python2.7" "--lldb-python" "/usr/bin/python" "--lldb-version" "lldb-902.0.79.2\n  Swift-4.1\n" "--lldb-python-dir" "/Applications/Xcode_9.3.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python" "--llvm-version" "9.0.0-rust-1.40.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-29T03:02:14.6321580Z 
2019-10-29T03:02:14.6321640Z 
2019-10-29T03:02:14.6325970Z failed to run: /Users/runner/runners/2.159.2/work/1/s/build/bootstrap/debug/bootstrap test
2019-10-29T03:02:14.6326170Z Build completed unsuccessfully in 0:56:40
2019-10-29T03:02:14.6326170Z Build completed unsuccessfully in 0:56:40
2019-10-29T03:02:14.6448180Z == clock drift check ==
2019-10-29T03:02:14.6498790Z   local time: Tue Oct 29 03:02:14 UTC 2019
2019-10-29T03:02:14.7364700Z   network time: Tue, 29 Oct 2019 03:02:14 GMT
2019-10-29T03:02:14.7367750Z == end clock drift check ==
2019-10-29T03:02:14.7435320Z 
2019-10-29T03:02:14.7579630Z ##[error]Bash exited with code '1'.
2019-10-29T03:02:14.7631660Z ##[section]Starting: Upload CPU usage statistics
2019-10-29T03:02:14.7636370Z ==============================================================================
2019-10-29T03:02:14.7636490Z Task         : Bash
2019-10-29T03:02:14.7636580Z Description  : Run a Bash script on macOS, Linux, or Windows
