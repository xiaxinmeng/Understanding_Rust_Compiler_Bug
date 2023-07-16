plain
[00:43:26] ....................................................................................................
[00:43:31] ...........................................................................i........................
[00:43:36] ....................................................i...............................................
[00:43:40] ........................................................................ii..........................
[00:43:45] ....F.F.............................................................................................
[00:43:53] iiiii...................................................
[00:43:53] failures:
[00:43:53] 
[00:43:53] 
[00:43:53] ---- [ui] ui/rust-2018/inject-2015-use-root-module-path.rs stdout ----
[00:43:53] normalized stderr:
[00:43:53] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:43:53] error[E0432]: unresolved import `x::y`
[00:43:53]   --> $DIR/inject-2015-use-root-module-path.rs:28:5
[00:43:53]    |
[00:43:53] LL |     print_me!(x::y); //~ ERROR unresolved import `x::y`
[00:43:53]    |     ^^^^^^^^^^^^^^^^ no `y` in `x`
[00:43:53]    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:43:53] 
[00:43:53] error: aborting due to previous error
[00:43:53] 
[00:43:53] 
[00:43:53] For more information about this error, try `rustc --explain E0432`.
[00:43:53] 
[00:43:53] 
[00:43:53] 
[00:43:53] The actual stderr differed from the expected stderr.
[00:43:53] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/inject-2015-use-root-module-path/inject-2015-use-root-module-path.stderr
[00:43:53] To update references, rerun the tests and pass the `--bless` flag
[00:43:53] To only update this specific test, also pass `--test-args rust-2018/inject-2015-use-root-module-path.rs`
[00:43:53] error: 1 errors occurred comparing output.
[00:43:53] status: exit code: 101
[00:43:53] status: exit code: 101
[00:43:53] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rust-2018/inject-2015-use-root-module-path.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/inject-2015-use-root-module-path/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition" "2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/inject-2015-use-root-module-path/auxiliary" "-A" "unused"
[00:43:53] ------------------------------------------
[00:43:53] 
[00:43:53] ------------------------------------------
[00:43:53] stderr:
[00:43:53] stderr:
[00:43:53] ------------------------------------------
[00:43:53] {"message":"unresolved import `x::y`","code":{"code":"E0432","explanation":"\nAn import was unresolved.\n\nErroneous code example:\n\n