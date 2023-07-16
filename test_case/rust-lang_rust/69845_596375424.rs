plain
2020-03-09T07:34:22.0636300Z test [ui] ui/hygiene/stdlib-prelude-from-opaque-early.rs ... ok
2020-03-09T07:34:22.1099820Z test [ui] ui/hygiene/specialization.rs ... ok
2020-03-09T07:34:22.1107080Z test [ui] ui/hygiene/trait_items.rs ... ok
2020-03-09T07:34:22.1108210Z test [ui] ui/hygiene/unpretty-debug.rs ... ok
2020-03-09T07:34:22.1744160Z test [ui] ui/if-attrs/bad-cfg.rs ... ok
2020-03-09T07:34:22.2832960Z test [ui] ui/if-attrs/builtin-if-attr.rs ... ok
2020-03-09T07:34:22.2931390Z test [ui] ui/hygiene/transparent-basic.rs ... ok
2020-03-09T07:34:22.3514900Z test [ui] ui/if-attrs/else-attrs.rs ... ok
2020-03-09T07:34:22.3616940Z test [ui] ui/if-attrs/cfg-false-if-attr.rs ... ok
2020-03-09T07:34:22.4320240Z test [ui] ui/if-attrs/let-chains-attr.rs ... ok
2020-03-09T07:34:22.5092890Z test [ui] ui/if-attrs/stmt-expr-gated.rs ... ok
2020-03-09T07:34:22.5674320Z test [ui] ui/hygiene/wrap_unhygienic_example.rs ... ok
2020-03-09T07:34:22.7366140Z test [ui] ui/if-attrs/gate-whole-expr.rs ... ok
2020-03-09T07:34:22.9064570Z test [ui] ui/if-bot.rs ... ok
2020-03-09T07:34:22.9267790Z test [ui] ui/if-else-type-mismatch.rs ... ok
2020-03-09T07:34:22.9983540Z test [ui] ui/if-check.rs ... ok
2020-03-09T07:34:23.0178210Z test [ui] ui/if/if-branch-types.rs ... ok
---
2020-03-09T07:39:47.7668740Z test [ui] ui/sanitize/memory.rs ... ignored
2020-03-09T07:39:49.1406000Z test [ui] ui/sanitize/address.rs ... ok
2020-03-09T07:39:49.2306460Z test [ui] ui/sanitize/leak.rs ... ok
2020-03-09T07:39:49.2700860Z test [ui] ui/sanitize/unsupported-target.rs ... ok
2020-03-09T07:39:49.3191680Z test [ui] ui/sanitize/badfree.rs ... ok
2020-03-09T07:39:49.5304390Z test [ui] ui/save-analysis/issue-59134-0.rs ... ok
2020-03-09T07:39:49.6208140Z test [ui] ui/save-analysis/issue-59134-1.rs ... ok
2020-03-09T07:39:49.7236290Z test [ui] ui/save-analysis/issue-63663.rs ... ok
2020-03-09T07:39:49.8065960Z test [ui] ui/save-analysis/issue-64659.rs ... ok
---
2020-03-09T07:42:04.2880550Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-09T07:42:04.2881140Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-09T07:42:04.2897120Z 
2020-03-09T07:42:04.2897400Z 
2020-03-09T07:42:04.2903790Z command did not execute successfully: "/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/stage0-tools-bin/compiletest" "--compile-lib-path" "/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/stage2/lib" "--run-lib-path" "/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "--rustc-path" "/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/stage2/bin/rustc" "--src-base" "/Users/runner/runners/2.165.0/work/1/s/src/test/ui" "--build-base" "/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/test/ui" "--stage-id" "stage2-x86_64-apple-darwin" "--mode" "ui" "--target" "x86_64-apple-darwin" "--host" "x86_64-apple-darwin" "--llvm-filecheck" "/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/llvm/build/bin/FileCheck" "--nodejs" "/usr/local/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/native/rust-test-helpers" "--docck-python" "/usr/local/bin/python2.7" "--lldb-python" "/usr/bin/python3" "--lldb-version" "lldb-1100.0.30.12\nApple Swift version 5.1.3 (swiftlang-1100.0.282.1 clang-1100.0.33.15)\n" "--lldb-python-dir" "/Applications/Xcode_11.3.1.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python3" "--llvm-version" "9.0.1-rust-1.43.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-09T07:42:04.2908160Z 
2020-03-09T07:42:04.2908280Z 
2020-03-09T07:42:04.2911440Z failed to run: /Users/runner/runners/2.165.0/work/1/s/build/bootstrap/debug/bootstrap test
2020-03-09T07:42:04.2911910Z Build completed unsuccessfully in 0:53:56
2020-03-09T07:42:04.2911910Z Build completed unsuccessfully in 0:53:56
2020-03-09T07:42:04.2961630Z == clock drift check ==
2020-03-09T07:42:04.3021040Z   local time: Mon Mar  9 07:42:04 UTC 2020
2020-03-09T07:42:04.3758220Z   network time: Mon, 09 Mar 2020 07:42:04 GMT
2020-03-09T07:42:04.3759850Z == end clock drift check ==
2020-03-09T07:42:04.3801130Z 
2020-03-09T07:42:04.3874490Z ##[error]Bash exited with code '1'.
2020-03-09T07:42:04.3953180Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2020-03-09T07:42:04.3959680Z ==============================================================================
2020-03-09T07:42:04.3960070Z Task         : Get sources
2020-03-09T07:42:04.3960500Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
