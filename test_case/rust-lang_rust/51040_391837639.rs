plain
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:46:40] 
[00:46:40] running 1388 tests
[00:46:45] ..................................................................................i.................
[00:46:51] .............F..................i...................................................................
[00:46:58] ....................................................................................................
[00:47:02] ....................................................................................................
[00:47:05] ....................................................................................................
[00:47:11] ....................................................................................................
---
[00:47:53] ---- [ui] ui/const-eval/dont_promote_unstable_const_fn.rs stdout ----
[00:47:53]  diff of stderr:
[00:47:53] 
[00:47:53] 27    |
[00:47:53] 28    = note: borrowed value must be valid for the static lifetime...
[00:47:53] - error: aborting due to 3 previous errors
[00:47:53] - error: aborting due to 3 previous errors
[00:47:53] + error[E0597]: borrowed value does not live long enough
[00:47:53] +   --> $DIR/dont_promote_unstable_const_fn.rs:33:26
[00:47:53] +    |
[00:47:53] + LL |     let x: &'static _ = &std::time::Duration::from_millis(42).subsec_millis();
[00:47:53] +    |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ temporary value does not live long enough
[00:47:53] + LL | }
[00:47:53] +    | - temporary value only lives until here
[00:47:53] +    |
[00:47:53] +    = note: borrowed value must be valid for the static lifetime...
[00:47:53] + error: aborting due to 4 previous errors
[00:47:53] 31 
[00:47:53] 32 For more information about this error, try `rustc --explain E0597`.
[00:47:53] 33 
---
[00:47:53] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'const-eval/dont_promote_unstable_const_fn.rs'
[00:47:53] 
[00:47:53] error: 1 errors occurred comparing output.
[00:47:53] status: exit code: 101
[00:47:53] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-eval/dont_promote_unstable_const_fn.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-eval/dont_promote_unstable_const_fn.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-eval/dont_promote_unstable_const_fn.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[00:47:53] ------------------------------------------
[00:47:53] 
[00:47:53] ------------------------------------------
[00:47:53] stderr:
[00:47:53] stderr:
[00:47:53] ------------------------------------------
[00:47:53] {"message":"`foo` is not yet stable as a const fn","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/const-eval/dont_promote_unstable_const_fn.rs","byte_start":867,"byte_end":872,"line_start":25,"line_end":25,"column_start":25,"column_end":30,"is_primary":true,"text":[{"text":"const fn bar() -> u32 { foo() } //~ ERROR `foo` is not yet stable as a const fn","highlight_start":25,"highlight_end":30}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[{"message":"in Nightly builds, add `#![feature(foo)]` to the crate attributes to enable","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: `foo` is not yet stable as a const fn\n  --> /checkout/src/test/ui/const-eval/dont_promote_unstable_const_fn.rs:25:25\n   |\nLL | const fn bar() -> u32 { foo() } //~ ERROR `foo` is not yet stable as a const fn\n   |                         ^^^^^\n   |\n   = help: in Nightly builds, add `#![feature(foo)]` to the crate attributes to enable\n\n"}
[00:47:53] {"message":"borrowed value does not live long enough","code":{"code":"E0597","explanation":"\nThis error occurs because a borrow was made inside a variable which has a\ngreater lifetime than the borrowed one.\n\nExample of erroneous code:\n\n