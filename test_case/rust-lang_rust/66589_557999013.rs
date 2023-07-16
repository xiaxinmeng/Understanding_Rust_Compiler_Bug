plain
2019-11-25T05:32:57.6747230Z test [ui] ui/tail-call-arg-leak.rs ... ok
2019-11-25T05:32:57.7149810Z test [ui] ui/tail-direct.rs ... ok
2019-11-25T05:32:57.7250420Z test [ui] ui/tail-cps.rs ... ok
2019-11-25T05:32:57.7351660Z test [ui] ui/target-feature/invalid-attribute.rs ... ok
2019-11-25T05:32:57.8460810Z test [ui] ui/terminal-width/non-1-width-unicode-multiline-label.rs ... ok
2019-11-25T05:32:57.9652450Z test [ui] ui/terminal-width/non-whitespace-trimming-unicode.rs ... FAILED
2019-11-25T05:32:58.0660500Z test [ui] ui/terminal-width/non-whitespace-trimming.rs ... ok
2019-11-25T05:32:58.1975990Z test [ui] ui/terminal-width/whitespace-trimming-2.rs ... ok
2019-11-25T05:32:58.4034620Z test [ui] ui/terminal-width/whitespace-trimming.rs ... ok
---
2019-11-25T05:34:41.5651780Z ---- [ui] ui/terminal-width/non-whitespace-trimming-unicode.rs stdout ----
2019-11-25T05:34:41.5652020Z diff of stderr:
2019-11-25T05:34:41.5655940Z 
2019-11-25T05:34:41.5656150Z 3    |
2019-11-25T05:34:41.5657780Z 4 LL | ...♭♮♯♰♱♲♳♴♵♶♷♸♹♺♻♼♽♾♿⚀⚁⚂⚃⚄⚅⚆⚈⚉4"; let _: () = 42;  let _: &str = "🦀☀☁☂☃☄★☆☇☈☉☊☋☌☍☎☏☐☑☒☓  ☖☗☘☙☚☛☜☝☞☟☠☡☢☣☤☥☦☧☨☩☪☫☬☭☮☯☰☱☲☳☴☵☶☷☸☹☺☻☼☽☾☿♀♁♂♃♄...
2019-11-25T05:34:41.5658760Z 5    |                                            --   ^^ expected `()`, found integer
2019-11-25T05:34:41.5659640Z +    |                                            |
2019-11-25T05:34:41.5659780Z 7    |                                            expected due to this
2019-11-25T05:34:41.5659870Z 8 
2019-11-25T05:34:41.5659960Z 9 error: aborting due to previous error
2019-11-25T05:34:41.5659960Z 9 error: aborting due to previous error
2019-11-25T05:34:41.5660020Z 
2019-11-25T05:34:41.5660060Z 
2019-11-25T05:34:41.5660150Z The actual stderr differed from the expected stderr.
2019-11-25T05:34:41.5661020Z Actual stderr saved to /Users/runner/runners/2.160.1/work/1/s/build/i686-apple-darwin/test/ui/terminal-width/non-whitespace-trimming-unicode/non-whitespace-trimming-unicode.stderr
2019-11-25T05:34:41.5661770Z To update references, rerun the tests and pass the `--bless` flag
2019-11-25T05:34:41.5662540Z To only update this specific test, also pass `--test-args terminal-width/non-whitespace-trimming-unicode.rs`
2019-11-25T05:34:41.5663060Z error: 1 errors occurred comparing output.
2019-11-25T05:34:41.5663150Z status: exit code: 1
2019-11-25T05:34:41.5663150Z status: exit code: 1
2019-11-25T05:34:41.5664800Z command: "/Users/runner/runners/2.160.1/work/1/s/build/i686-apple-darwin/stage2/bin/rustc" "/Users/runner/runners/2.160.1/work/1/s/src/test/ui/terminal-width/non-whitespace-trimming-unicode.rs" "-Zthreads=1" "--target=i686-apple-darwin" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/Users/runner/runners/2.160.1/work/1/s/build/i686-apple-darwin/test/ui/terminal-width/non-whitespace-trimming-unicode" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/Users/runner/runners/2.160.1/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "-L" "/Users/runner/runners/2.160.1/work/1/s/build/i686-apple-darwin/test/ui/terminal-width/non-whitespace-trimming-unicode/auxiliary" "-A" "unused"
2019-11-25T05:34:41.5665880Z ------------------------------------------
2019-11-25T05:34:41.5665980Z 
2019-11-25T05:34:41.5666620Z ------------------------------------------
2019-11-25T05:34:41.5666750Z stderr:
2019-11-25T05:34:41.5666750Z stderr:
2019-11-25T05:34:41.5667390Z ------------------------------------------
2019-11-25T05:34:41.5667520Z error[E0308]: mismatched types
2019-11-25T05:34:41.5668270Z   --> /Users/runner/runners/2.160.1/work/1/s/src/test/ui/terminal-width/non-whitespace-trimming-unicode.rs:4:415
2019-11-25T05:34:41.5668400Z    |
2019-11-25T05:34:41.5669290Z LL | ...♭♮♯♰♱♲♳♴♵♶♷♸♹♺♻♼♽♾♿⚀⚁⚂⚃⚄⚅⚆⚈⚉4"; let _: () = 42;  let _: &str = "🦀☀☁☂☃☄★☆☇☈☉☊☋☌☍☎☏☐☑☒☓  ☖☗☘☙☚☛☜☝☞☟☠☡☢☣☤☥☦☧☨☩☪☫☬☭☮☯☰☱☲☳☴☵☶☷☸☹☺☻☼☽☾☿♀♁♂♃♄...
2019-11-25T05:34:41.5670070Z    |                                            --   ^^ expected `()`, found integer
2019-11-25T05:34:41.5670320Z    |                                            expected due to this
2019-11-25T05:34:41.5670390Z 
2019-11-25T05:34:41.5670470Z error: aborting due to previous error
2019-11-25T05:34:41.5670530Z 
---
2019-11-25T05:34:41.5768120Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-25T05:34:41.5768350Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-25T05:34:41.5787150Z 
2019-11-25T05:34:41.5787480Z 
2019-11-25T05:34:41.5791210Z command did not execute successfully: "/Users/runner/runners/2.160.1/work/1/s/build/i686-apple-darwin/stage0-tools-bin/compiletest" "--compile-lib-path" "/Users/runner/runners/2.160.1/work/1/s/build/i686-apple-darwin/stage2/lib" "--run-lib-path" "/Users/runner/runners/2.160.1/work/1/s/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib" "--rustc-path" "/Users/runner/runners/2.160.1/work/1/s/build/i686-apple-darwin/stage2/bin/rustc" "--src-base" "/Users/runner/runners/2.160.1/work/1/s/src/test/ui" "--build-base" "/Users/runner/runners/2.160.1/work/1/s/build/i686-apple-darwin/test/ui" "--stage-id" "stage2-i686-apple-darwin" "--mode" "ui" "--target" "i686-apple-darwin" "--host" "i686-apple-darwin" "--llvm-filecheck" "/Users/runner/runners/2.160.1/work/1/s/build/i686-apple-darwin/llvm/build/bin/FileCheck" "--nodejs" "/usr/local/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/runner/runners/2.160.1/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/runner/runners/2.160.1/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "--docck-python" "/usr/local/bin/python2.7" "--lldb-python" "/usr/bin/python" "--lldb-version" "lldb-902.0.79.2\n  Swift-4.1\n" "--lldb-python-dir" "/Applications/Xcode_9.3.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python" "--llvm-version" "9.0.0-rust-1.41.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-25T05:34:41.5792280Z 
2019-11-25T05:34:41.5792330Z 
2019-11-25T05:34:41.5802810Z failed to run: /Users/runner/runners/2.160.1/work/1/s/build/bootstrap/debug/bootstrap test
2019-11-25T05:34:41.5802990Z Build completed unsuccessfully in 0:57:19
2019-11-25T05:34:41.5802990Z Build completed unsuccessfully in 0:57:19
2019-11-25T05:34:41.5873060Z == clock drift check ==
2019-11-25T05:34:41.5937020Z   local time: Mon Nov 25 05:34:41 UTC 2019
2019-11-25T05:34:41.6388120Z   network time: Mon, 25 Nov 2019 05:34:41 GMT
2019-11-25T05:34:41.6391090Z == end clock drift check ==
2019-11-25T05:34:41.6443570Z 
2019-11-25T05:34:41.6604160Z ##[error]Bash exited with code '1'.
2019-11-25T05:34:41.6654670Z ##[section]Starting: Checkout
2019-11-25T05:34:41.6657270Z ==============================================================================
2019-11-25T05:34:41.6657400Z Task         : Get sources
2019-11-25T05:34:41.6657480Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
