plain
2019-07-24T20:54:40.2845060Z 
2019-07-24T20:54:40.2845750Z ---- [ui] ui/feature-gate/rustc-private.rs stdout ----
2019-07-24T20:54:40.2845880Z diff of stderr:
2019-07-24T20:54:40.2845930Z 
2019-07-24T20:54:40.2846780Z - error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-24T20:54:40.2847670Z + error[E0658]: use of unstable library feature 'rustc_private': crate "libc" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-24T20:54:40.2848490Z 3    |
2019-07-24T20:54:40.2848570Z 4 LL | extern crate libc;
2019-07-24T20:54:40.2848620Z 
2019-07-24T20:54:40.2848670Z 
2019-07-24T20:54:40.2848670Z 
2019-07-24T20:54:40.2848770Z The actual stderr differed from the expected stderr.
2019-07-24T20:54:40.2849540Z Actual stderr saved to /Users/vsts/agent/2.154.3/work/1/s/build/i686-apple-darwin/test/ui/feature-gate/rustc-private/rustc-private.stderr
2019-07-24T20:54:40.2850220Z To update references, rerun the tests and pass the `--bless` flag
2019-07-24T20:54:40.2851220Z To only update this specific test, also pass `--test-args feature-gate/rustc-private.rs`
2019-07-24T20:54:40.2851390Z error: 1 errors occurred comparing output.
2019-07-24T20:54:40.2851480Z status: exit code: 1
2019-07-24T20:54:40.2851480Z status: exit code: 1
2019-07-24T20:54:40.2852930Z command: "/Users/vsts/agent/2.154.3/work/1/s/build/i686-apple-darwin/stage2/bin/rustc" "/Users/vsts/agent/2.154.3/work/1/s/src/test/ui/feature-gate/rustc-private.rs" "-Zthreads=1" "--target=i686-apple-darwin" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/Users/vsts/agent/2.154.3/work/1/s/build/i686-apple-darwin/test/ui/feature-gate/rustc-private" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/Users/vsts/agent/2.154.3/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "-L" "/Users/vsts/agent/2.154.3/work/1/s/build/i686-apple-darwin/test/ui/feature-gate/rustc-private/auxiliary" "-A" "unused"
2019-07-24T20:54:40.2853860Z ------------------------------------------
2019-07-24T20:54:40.2853950Z 
2019-07-24T20:54:40.2854570Z ------------------------------------------
2019-07-24T20:54:40.2854670Z stderr:
2019-07-24T20:54:40.2854670Z stderr:
2019-07-24T20:54:40.2855270Z ------------------------------------------
2019-07-24T20:54:40.2856100Z error[E0658]: use of unstable library feature 'rustc_private': crate "libc" is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
2019-07-24T20:54:40.2856980Z    |
2019-07-24T20:54:40.2856980Z    |
2019-07-24T20:54:40.2857630Z LL | extern crate libc; //~ ERROR  use of unstable library feature 'rustc_private'
2019-07-24T20:54:40.2857840Z    |
2019-07-24T20:54:40.2858510Z    = note: for more information, see https://github.com/rust-lang/rust/issues/27812
2019-07-24T20:54:40.2858510Z    = note: for more information, see https://github.com/rust-lang/rust/issues/27812
2019-07-24T20:54:40.2858630Z    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
2019-07-24T20:54:40.2858800Z error: aborting due to previous error
2019-07-24T20:54:40.2858860Z 
2019-07-24T20:54:40.2859500Z For more information about this error, try `rustc --explain E0658`.
2019-07-24T20:54:40.2859580Z 
---
2019-07-24T20:54:40.2903330Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:535:22
2019-07-24T20:54:40.2903560Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-07-24T20:54:40.2916420Z 
2019-07-24T20:54:40.2916620Z 
2019-07-24T20:54:40.2920290Z command did not execute successfully: "/Users/vsts/agent/2.154.3/work/1/s/build/i686-apple-darwin/stage0-tools-bin/compiletest" "--compile-lib-path" "/Users/vsts/agent/2.154.3/work/1/s/build/i686-apple-darwin/stage2/lib" "--run-lib-path" "/Users/vsts/agent/2.154.3/work/1/s/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib" "--rustc-path" "/Users/vsts/agent/2.154.3/work/1/s/build/i686-apple-darwin/stage2/bin/rustc" "--src-base" "/Users/vsts/agent/2.154.3/work/1/s/src/test/ui" "--build-base" "/Users/vsts/agent/2.154.3/work/1/s/build/i686-apple-darwin/test/ui" "--stage-id" "stage2-i686-apple-darwin" "--mode" "ui" "--target" "i686-apple-darwin" "--host" "i686-apple-darwin" "--llvm-filecheck" "/Users/vsts/agent/2.154.3/work/1/s/build/i686-apple-darwin/llvm/build/bin/FileCheck" "--nodejs" "/usr/local/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/vsts/agent/2.154.3/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/vsts/agent/2.154.3/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "--docck-python" "/usr/local/bin/python2.7" "--lldb-python" "/usr/bin/python" "--lldb-version" "lldb-902.0.79.2\n  Swift-4.1\n" "--lldb-python-dir" "/Applications/Xcode_9.3.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python" "--llvm-version" "9.0.0-rust-1.38.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-07-24T20:54:40.2921500Z 
2019-07-24T20:54:40.2921590Z 
2019-07-24T20:54:40.2932840Z failed to run: /Users/vsts/agent/2.154.3/work/1/s/build/bootstrap/debug/bootstrap test
2019-07-24T20:54:40.2933240Z Build completed unsuccessfully in 0:57:37
2019-07-24T20:54:40.2933240Z Build completed unsuccessfully in 0:57:37
2019-07-24T20:54:40.3179260Z ##[error]Bash exited with code '1'.
2019-07-24T20:54:40.3226140Z ##[section]Starting: Upload CPU usage statistics
2019-07-24T20:54:40.3256190Z ==============================================================================
2019-07-24T20:54:40.3256310Z Task         : Bash
2019-07-24T20:54:40.3256390Z Description  : Run a Bash script on macOS, Linux, or Windows
