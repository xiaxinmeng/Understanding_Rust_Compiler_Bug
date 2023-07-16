\n\nIt is also possible to overload most operators for your own type by\nimplementing traits from `std::ops`.\n\nString concatenation appends the string on the right to the string on the\nleft and may require reallocation. This requires ownership of the string\non the left. If something should be added to a string literal, move the\nliteral to the heap by allocating it with `to_owned()` like in\n`\"Your text\".to_owned()`.\n\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-41394.rs","byte_start":19,"byte_end":25,"line_start":2,"line_end":2,"column_start":9,"column_end":15,"is_primary":true,"text":[{"text":"    A = \"\" + 1","highlight_start":9,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"an implementation of `std::ops::Add` might be missing for `&str`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0369]: binary operation `+` cannot be applied to type `&str`\n  --> /checkout/src/test/ui/issues/issue-41394.rs:2:9\n   |\nLL |     A = \"\" + 1\n   |         ^^^^^^\n   |\n   = note: an implementation of `std::ops::Add` might be missing for `&s | use std::f64::consts::LOG10_2;
[01:00:58] 12 
[01:00:58] - error[E0080]: evaluation of constant value failed
[01:00:58] - error[E0080]: evaluation of constant value failed
[01:00:58] + error: any use of this value will cause an error
[01:00:58] 15    |
[01:00:58] 15    |
[01:00:58] 16 LL |     let mut digits = [0u32; M];
[01:00:58] 
[01:00:58] 17    |                             ^ referenced constant has errors
[01:00:58] +    = note: #[deny(const_err)] on by default
[01:00:58] 18 
[01:00:58] 19 error: aborting due to 2 previous errors
[01:00:58] 20 
---
[01:00:58] 
[01:00:58] 
[01:00:58] The actual stderr differed from the expected stderr.
[01:00:58] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-50599/issue-50599.stderr
[01:00:58] To update references, rerun the tests and pass the `--bless` flag
[01:00:58] To only update this specific test, also pass `--test-args issues/issue-50599.rs`
[01:00:58] error: 1 errors occurred comparing output.
[01:00:58] status: exit code: 1
[01:00:58] status: exit code: 1
[01:00:58] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-50599.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-50599/a" "-Crpath" "-O" "-Zunstable.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-50599.rs","byte_start":85,"byte_end":92,"line_start":3,"line_end":3,"column_start":48,"column_end":55,"is_primary":true,"text":[{"text":"    const M: usize = (f64::from(N) * std::f64::LOG10_2) as usize; //~ ERROR cannot find value","highlight_start":48,"highlight_end":55}],"label":"not found in `std::f64`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"possible candidates are found in other modules, you can import them into scope","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-50599.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"fn main() {","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":"use std::f32::consts::LOG10_2;\n\n","suggestion_applicability":"Unspecified","expansion":null},{"file_name":"/checkout/src/test/ui/issues/issue-50599.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"fn main() {","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":"use std::f64::consts::LOG10_2;\n\n","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0425]: cannot find value `LOG10_2` in module `std::f64`\n  --> /checkout/src/test/ui/issues/issue-50599.rs:3:48\n   |\nLL |     const M: usize = (f64::from(N) * std::f64::LOG10_2) as usize; //~ ERROR cannot find value\n   |                                                ^^^^^^^ not found in `std::f64`\nhelp: possible candidates are found in other modules, you can import them into scope\n   |\nLL | use std::f32::consts::LOG10_2;\n   |\nLL | use std::f64::consts::LOG10_2;\n   |\n\n"}
[01:00:58] {"message":"any use of this value will cause an error","code":{"code":"const_err","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-50599.rs","byte_start":160,"byte_end":161,"line_start":4,"line_end":4,"column_start":29,"column_end":30,"is_primary":true,"text":[{"text":"    let mut digits = [0u32; M];","highlight_start":29,"highlight_end":30}],"label":"referenced constant has errors","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[deny(const_err)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: any use of this value will cause an error\n  --> /checkout/src/test/ui/issues/issue-50599.rs:4:29\n   |\nLL |     let mut digits = [0u32; M];\n   |                             ^ referenced constant has errors\n   |\n   = note: #[deny(const_err)] on by default\n\n"}
[01:00:58] {"message":"For more information about this error, try `rustc --explain E0425`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0425`.\n"}
[01:00:58] 
[01:00:58] ------------------------------------------
[01:00:58] 
[01:00:58] 
[01:00:58] thread '[ui] ui/issues/issue-50599.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:00:58] 
[01:00:58] ---- [ui] ui/type/type-dependent-def-issue-49241.rs stdout ----
[01:00:58] diff of stderr:
[01:00:58] 
[01:00:58] 6    |
[01:00:58] 7    = help: use the `|| { ... }` closure form instead
[01:00:58] - error[E0080]: evaluation of constant value failed
[01:00:58] - error[E0080]: evaluation of constant value failed
[01:00:58] + error: any use of this value will cause an error
[01:00:58] 11    |
[01:00:58] 11    |
[01:00:58] 12 LL |     let s: [u32; l] = v.into_iter().collect();
[01:00:58] 
[01:00:58] 13    |                  ^ referenced constant has errors
[01:00:58] +    = note: #[deny(const_err)] on by default
[01:00:58] 14 
[01:00:58] 15 error: aborting due to 2 previous errors
[01:00:58] 16 
---
[01:00:58] 19 
[01:00:58] 
[01:00:58] 
[01:00:58] The actual stderr differed from the expected stderr.
[01:00:58] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-dependent-def-issue-49241/type-dependent-def-issue-49241.stderr
[01:00:58] To update references, rerun the tests and pass the `--bless` flag
[01:00:58] To only update this specific test, also pass `--test-args type/type-dependent-def-issue-49241.rs`
[01:00:58] error: 1 errors occurred comparing output.
[01:00:58] status: exit code: 1
[01:00:58] status: exit code: 1
[01:00:58] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type/type-dependent-def-issue-49241.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-dependent-def-issue-49241/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-dependent-def-issue-49241/auxiliary" "-A" "unused"
[01:00:58] ------------------------------------------
[01:00:58] 
[01:00:58] ------------------------------------------
[01:00:58] stderr:
[01:00:58] stderr:
[01:00:58] ------------------------------------------
[01:00:58] {"message":"can't capture dynamic environment in a fn item","code":{"code":"E0434","explanation":"\nThis error indicates that a variable usage inside an inner function is invalid\nbecause the variable comes from a dynamic environment. Inner functions do not\nhave access to their containing environment.\n\nErroneous code example:\n\n