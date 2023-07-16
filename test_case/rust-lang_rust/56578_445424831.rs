plain
[01:08:49] 
[01:08:49] ---- [ui (nll)] ui/consts/const-eval/dont_promote_unstable_const_fn.rs stdout ----
[01:08:49] diff of stderr:
[01:08:49] 
[01:08:49] 4 LL | const fn bar() -> u32 { foo() } //~ ERROR `foo` is not yet stable as a const fn
[01:08:49] 6    |
[01:08:49] 6    |
[01:08:49] -    = help: in Nightly builds, add `#![feature(foo)]` to the crate attributes to enable
[01:08:49] +    = help: add `#![feature(foo)]` to the crate attributes to enable
[01:08:49] 8 
[01:08:49] 9 error[E0716]: temporary value dropped while borrowed
[01:08:49] 10   --> $DIR/dont_promote_unstable_const_fn.rs:28:28
[01:08:49] 
[01:08:49] The actual stderr differed from the expected stderr.
[01:08:49] The actual stderr differed from the expected stderr.
[01:08:49] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/dont_promote_unstable_const_fn.nll/dont_promote_unstable_const_fn.nll.stderr
[01:08:49] To update references, rerun the tests and pass the `--bless` flag
[01:08:49] To only update this specific test, also pass `--test-args consts/const-eval/dont_promote_unstable_const_fn.rs`
[01:08:49] error: 1 errors occurred comparing output.
[01:08:49] status: exit code: 1
[01:08:49] status: exit code: 1
[01:08:49] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/dont_promote_unstable_const_fn.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/dont_promote_unstable_const_fn.nll/a" "-Zborrowck=migrate" "-Ztwo-phase-borrows" "-Crpath" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/dont_promote_unstable_const_fn.nll/auxiliary" "-A" "unused"
[01:08:49] ------------------------------------------
[01:08:49] 
[01:08:49] ------------------------------------------
[01:08:49] stderr:
[01:08:49] stderr:
[01:08:49] ------------------------------------------
[01:08:49] {"message":"`foo` is not yet stable as a const fn","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/consts/const-eval/dont_promote_unstable_const_fn.rs","byte_start":867,"byte_end":872,"line_start":25,"line_end":25,"column_start":25,"column_end":30,"is_primary":true,"text":[{"text":"const fn bar() -> u32 { foo() } //~ ERROR `foo` is not yet stable as a const fn","highlight_start":25,"highlight_end":30}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"add `#![feature(foo)]` to the crate attributes to enable","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: `foo` is not yet stable as a const fn\n  --> /checkout/src/test/ui/consts/const-eval/dont_promote_unstable_const_fn.rs:25:25\n   |\nLL | const fn bar() -> u32 { foo() } //~ ERROR `foo` is not yet stable as a const fn\n   |                         ^^^^^\n   |\n   = help: add `#![feature(foo)]` to the crate attributes to enable\n\n"}
[01:08:49] {"message":"temporary value dropped while borrowed","code":{"code":"E0716","explanation":"\nThis error indicates that a temporary value is being dropped\nwhile a borrow is still in active use.\n\nErroneous code example:\n\n