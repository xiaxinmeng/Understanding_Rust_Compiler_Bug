plain
[00:17:09]    Compiling rustc_lint v0.0.0 (file:///checkout/src/librustc_lint)
[00:17:09]    Compiling rustc_passes v0.0.0 (file:///checkout/src/librustc_passes)
[00:17:13]    Compiling rustc_plugin v0.0.0 (file:///checkout/src/librustc_plugin)
[00:17:23]    Compiling rustc_resolve v0.0.0 (file:///checkout/src/librustc_resolve)
[00:17:24] warning: diagnostic code E0432 already used
[00:17:24]     |
[00:17:24]     |
[00:17:24] 365 |             let mut err = struct_span_err!(resolver.session, span, E0432, "{}", msg);
[00:17:24]     |
[00:17:24] note: previous invocation
[00:17:24]    --> librustc_resolve/resolve_imports.rs:799:23
[00:17:24]     |
[00:17:24]     |
[00:17:24] 799 |           let mut err = struct_span_err!(self.resolver.session,
[00:17:24]     |  _______________________^
[00:17:24] 800 | |                                        multi_span.clone(), E0432, "{}", &msg);
[00:17:24]     = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:17:24] 
[00:18:37]    Compiling rustc-main v0.0.0 (file:///checkout/src/rustc)
[00:18:37]     Finished release [optimized] target(s) in 13m 16s
---
[00:31:53]    Compiling rustc_privacy v0.0.0 (file:///checkout/src/librustc_privacy)
[00:32:09]    Compiling rustc_save_analysis v0.0.0 (file:///checkout/src/librustc_save_analysis)
[00:32:58]    Compiling rustc_resolve v0.0.0 (file:///checkout/src/librustc_resolve)
[00:32:58]    Compiling rustc_plugin v0.0.0 (file:///checkout/src/librustc_plugin)
[00:32:59] warning: diagnostic code E0432 already used
[00:32:59]     |
[00:32:59]     |
[00:32:59] 365 |             let mut err = struct_span_err!(resolver.session, span, E0432, "{}", msg);
[00:32:59]     |
[00:32:59] note: previous invocation
[00:32:59]    --> librustc_resolve/resolve_imports.rs:799:23
[00:32:59]     |
[00:32:59]     |
[00:32:59] 799 |           let mut err = struct_span_err!(self.resolver.session,
[00:32:59]     |  _______________________^
[00:32:59] 800 | |                                        multi_span.clone(), E0432, "{}", &msg);
[00:32:59]     = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:32:59] 
[00:33:04]    Compiling rustc_codegen_utils v0.0.0 (file:///checkout/src/librustc_codegen_utils)
[00:33:04]    Compiling rustc_passes v0.0.0 (file:///checkout/src/librustc_passes)
---
[00:44:22] ....................................................................................................
[00:44:24] ....................................................................................................
[00:44:27] i..................................................................i................................
[00:44:30] ....................................................................................................
[00:44:33] ........................................F...........................................................
[00:44:38] ....................................................................................................
[00:44:41] ....................................................................................................
[00:44:44] ....................................................................................................
[00:44:47] ....................................................................................................
---
[00:45:46] 
[00:45:46] ---- [ui] ui/issue-53565.rs stdout ----
[00:45:46] diff of stderr:
[00:45:46] 
[00:45:46] 1 error[E0432]: unresolved imports `std::time::foo`, `std::time::bar`, `std::time::buzz`
[00:45:46] +   --> $DIR/issue-53565.rs:10:17
[00:45:46] 3    |
[00:45:46] 3    |
[00:45:46] 4 LL | use std::time::{foo, bar, buzz};
[00:45:46] 5    |                 ^^^  ^^^  ^^^^ no `buzz` in `time`
[00:45:46] 
[00:45:46] 8    |                 no `foo` in `time`
[00:45:46] 9 
[00:45:46] 10 error[E0432]: unresolved imports `std::time::abc`, `std::time::def`
[00:45:46] +   --> $DIR/issue-53565.rs:11:17
[00:45:46] 12    |
[00:45:46] 12    |
[00:45:46] 13 LL | use std::time::{abc, def};
[00:45:46] 14    |                 ^^^  ^^^ no `def` in `time`
[00:45:46] 
[00:45:46] The actual stderr differed from the expected stderr.
[00:45:46] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-53565/issue-53565.stderr
[00:45:46] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-53565/issue-53565.stderr
[00:45:46] To update references, rerun the tests and pass the `--bless` flag
[00:45:46] To only update this specific test, also pass `--test-args issue-53565.rs`
[00:45:46] error: 1 errors occurred comparing output.
[00:45:46] status: exit code: 1
[00:45:46] status: exit code: 1
[00:45:46] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issue-53565.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-53565/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-53565/auxiliary" "-A" "unused"
[00:45:46] ------------------------------------------
[00:45:46] 
[00:45:46] ------------------------------------------
[00:45:46] stderr:
[00:45:46] stderr:
[00:45:46] ------------------------------------------
[00:45:46] {"message":"unresolved imports `std::time::foo`, `std::time::bar`, `std::time::buzz`","code":{"code":"E0432","explanation":"\nAn import was unresolved.\n\nErroneous code example:\n\n