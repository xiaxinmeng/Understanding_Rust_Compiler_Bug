plain
[00:47:04] ....................................................................................................
[00:47:09] ....................................................................................................
[00:47:15] ...............................................................................................i....
[00:47:20] ..........................................................................i.........................
[00:47:25] ..........................................................................................F.F.......
[00:47:31] ....................................................................................................
[00:47:36] ..............................................................................F.....................
eckout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2005-default-binding-mode/enum/auxiliary" "-A" "unused"
[00:47:41] ------------------------------------------
[00:47:41] 
[00:47:41] ------------------------------------------
[00:47:41] stderr:
[00:47:41] stderr:
[00:47:41] ------------------------------------------
[00:47:41] {"message":"cannot assign to immutable borrowed content `*x`","code":{"code":"E0594","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rfc-2005-default-binding-mode/enum.rs","byte_start":568,"byte_end":575,"line_start":19,"line_end":19,"column_start":5,"column_end":12,"is_primary":true,"text":[{"text":"    *x += 1; //~ ERROR cannot assign to immutable","highlight_start":5,"highlight_end":12}],"label":"cannot borrow as mutable","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/rfc-2005-default-binding-mode/enum.rs","byte_start":549,"byte_end":550,"line_start":18,"line_end":18,"column_start":14,"column_end":15,"is_primary":false,"text":[{"text":"    let Wrap(x) = &Wrap(3);","highlight_start":14,"highlight_end":15}],"label":"consider changing this to `ref mut`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0594]: cannot assign to immutable borrowed content `*x`\n  --> /checkout/src/test/ui/rfc-2005-default-binding-mode/enum.rs:19:5\n   |\nLL |     let Wrap(x) = &Wrap(3);\n   |              - consider changing this to `ref mut`\nLL |     *x += 1; //~ ERROR cannot assign to immutable\n   |     ^^^^^^^ cannot borrow as mutable\n\n"}
[00:47:41] {"message":"cannot assign to immutable borrowed content `*x`","code":{"code":"E0594","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rfc-2005-default-binding-mode/enum.rs","byte_start":656,"byte_end":663,"line_start":23,"line_end":23,"column_start":9,"column_end":16,"is_primary":true,"text":[{"text":"        *x += 1; //~ ERROR cannot assign to immutable","highlight_start":9,"highlight_end":16}],"label":"cannot borrow as mutable","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/rfc-2005-default-binding-mode/enum.rs","byte_start":632,"byte_end":633,"line_start":22,"line_end":22,"column_start":17,"column_end":18,"is_primary":false,"text":[{"text":"    if let Some(x) = &Some(3) {","highlight_start":17,"highlight_end":18}],"label":"consider changing this to `ref mut`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0594]: cannot assign to immutable borrowed content `*x`\n  --> /checkout/src/test/ui/rfc-2005-default-binding-mode/enum.rs:23:9\n   |\nLL |     if let Some(x) = &Some(3) {\n   |                 - consider changing this to `ref mut`\nLL |         *x += 1; //~ ERROR cannot assign to immutable\n   |         ^^^^^^^ cannot borrow as mutable\n\n"}
[00:47:41] {"message":"cannot assign to immutable borrowed content `*x`","code":{"code":"E0594","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rfc-2005-default-binding-mode/enum.rs","byte_start":783,"byte_end":790,"line_start":29,"line_end":29,"column_start":9,"column_end":16,"is_primary":true,"text":[{"text":"        *x += 1; //~ ERROR cannot assign to immutable","highlight_start":9,"highlight_end":16}],"label":"cannot borrow as mutable","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/rfc-2005-default-binding-mode/enum.rs","byte_start":759,"byte_end":760,"line_start":28,"line_end":28,"column_start":20,"column_end":21,"is_primary":false,"text":[{"text":"    while let Some(x) = &Some(3) {","highlight_start":20,"highlight_end":21}],"label":"consider changing this to `ref mut`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0594]: cannot assign to immutable borrowed content `*x`\n  --> /checkout/src/test/ui/rfc-2005-default-binding-mode/enum.rs:29:9\n   |\nLL |     while let Some(x) = &Some(3) {\n   |                    - consider changing this to `ref mut`\nLL |         *x += 1; //~ ERROR cannot assign to immutable\n   |         ^^^^^^^ cannot borrow as mutable\n\n"}
[00:47:41] {"message":"aborting due to 3 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 3 previous errors\n\n"}
[00:47:41] {"message":"For more information about this error, try `rustc --explain E0594`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0594`.\n"}
[00:47:41] ------------------------------------------
[00:47:41] 
[00:47:41] thread '[ui] ui/rfc-2005-default-binding-mode/enum.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3096:9
[00:47:41] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:47:41] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:47:41] 
[00:47:41] ---- [ui] ui/rfc-2005-default-binding-mode/explicit-mut.rs stdout ----
[00:47:41] diff of stderr:
[00:47:41] 
[00:47:41] 2   --> $DIR/explicit-mut.rs:17:13
[00:47:41] 3    |
[00:47:41] 4 LL |         Some(n) => {
[00:47:41] -    |              - consider changing this to `n`
[00:47:41] +    |              - consider changing this to `ref mut`
[00:47:41] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:47:41] 6 LL |             *n += 1; //~ ERROR cannot assign to immutable
[00:47:41] 7    |             ^^^^^^^ cannot borrow as mutable
[00:47:41] 
[00:47:41] 10   --> $DIR/explicit-mut.rs:25:13
[00:47:41] 11    |
[00:47:41] 11    |
[00:47:41] 12 LL |         Some(n) => {
[00:47:41] -    |              - consider changing this to `n`
[00:47:41] +    |              - consider changing this to `ref mut`
[00:47:41] 14 LL |             *n += 1; //~ ERROR cannot assign to immutable
[00:47:41] 15    |             ^^^^^^^ cannot borrow as mutable
[00:47:41] 
[00:47:41] 18   --> $DIR/explicit-mut.rs:33:13
[00:47:41] 19    |
[00:47:41] 19    |
[00:47:41] 20 LL |         Some(n) => {
[00:47:41] -    |              - consider changing this to `n`
[00:47:41] +    |              - consider changing this to `ref mut`
[00:47:41] 22 LL |             *n += 1; //~ ERROR cannot assign to immutable
[00:47:41] 23    |             ^^^^^^^ cannot borrow as mutable
[00:47:41] 
[00:47:41] 
[00:47:41] 
[00:47:41] The actual stderr ext":"            *n += 1; //~ ERROR cannot assign to immutable","highlight_start":13,"highlight_end":20}],"label":"cannot borrow as mutable","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/rfc-2005-default-binding-mode/explicit-mut.rs","byte_start":639,"byte_end":640,"line_start":16,"line_end":16,"column_start":14,"column_end":15,"is_primary":false,"text":[{"text":"        Some(n) => {","highlight_start":14,"highlight_end":15}],"label":"consider changing this to `ref mut`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0594]: cannot assign to immutable borrowed content `*n`\n  --> /checkout/src/test/ui/rfc-2005-default-binding-mode/explicit-mut.rs:17:13\n   |\nLL |         Some(n) => {\n   |              - consider changing this to `ref mut`\nLL |             *n += 1; //~ ERROR cannot assign to immutable\n   |             ^^^^^^^ cannot borrow as mutable\n\n"}
[00:47:41] {"message":"cannot assign to immutable borrowed content `*n`","code":{"code":"E0594","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rfc-2005-default-binding-mode/explicit-mut.rs","byte_start":828,"byte_end":835,"line_start":25,"line_end":25,"column_start":13,"column_end":20,"is_primary":true,"text":[{"text":"            *n += 1; //~ ERROR cannot assign to immutable","highlight_start":13,"highlight_end":20}],"label":"cannot borrow as mutable","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/rfc-2005-default-binding-mode/explicit-" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-51244/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-51244/auxiliary" "-A" "unused"
[00:47:41] unexpected errors (from JSON output): [
[00:47:41]     Error {
[00:47:41]         line_num: 13,
[00:47:41]         kind: Some(
[00:47:41]         ),
[00:47:41]         ),
[00:47:41]         msg: "13:5: 13:16: cannot assign to immutable borrowed content `*my_ref` [E0594]"
[00:47:41] ]
[00:47:41] 
[00:47:41] thread '[ui] ui/suggestions/issue-51244.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1284:13
[00:47:41] 
---
[00:47:41] test result: FAILED. 1468 passed; 3 failed; 14 ignored; 0 measured; 0 filtered out
[00:47:41] 
[00:47:41] 
[00:47:41] 
[00:47:41] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-lin
