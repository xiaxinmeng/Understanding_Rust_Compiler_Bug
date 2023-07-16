plain
[00:48:23] ....................................................................................................
[00:48:27] ....................................................................................................
[00:48:32] ....................................................................................................
[00:48:38] ....................................................................................................
[00:48:44] .................................................................F..................................
[00:48:56] .............i......................................................................................
[00:49:01] ................................ii..................................................................
[00:49:01] ................................ii..................................................................
[00:49:08] ................F...................................................................................
[00:49:15] 
[00:49:15] failures:
[00:49:15] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:492:22
[00:49:15] 
[00:49:15] 
[00:49:15] ---- [ui] ui/lint/issue-47390-unused-variable-in-struct-pattern.rs stdout ----
[00:49:15]  diff of stderr:
[00:49:15] 
[00:49:15] 2   --> $DIR/issue-47390-unused-variable-in-struct-pattern.rs:31:9
[00:49:15] 3    |
[00:49:15] 4 LL |     let i_think_continually = 2;
[00:49:15] - R/issue-47390-unused-variable-in-struct-pattern.rs:42:26
[00:49:15] 
[00:49:15] The actual stderr differed from the expected stderr.
[00:49:15] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/issue-47390-unused-variable-in-struct-pattern.stderr
[00:49:15] To update references, run this command from build directory:
[00:49:15] To update references, run this command from build directory:
[00:49:15] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'lint/issue-47390-unused-variable-in-struct-pattern.rs'
[00:49:15] error: 1 errors occurred comparing output.
[00:49:15] status: exit code: 0
[00:49:15] status: exit code: 0
[00:49:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/issue-47390-unused-variable-in-struct-pattern.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/issue-47390-unused-variable-in-struct-pattern.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/issue-47390-unused-variable-in-struct-pattern.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[00:49:15] ------------------------------------------
[00:49:15] 
[00:49:15] ------------------------------------------
[00:49:15] stderr:
[00:49:15] stderr:
[00:49:15] ------------------------------------------
[00:49:15] {"message":"unused variable: `i_think_continuallc/test/ui/lint/issue-47390-unused-variable-in-struct-pattern.rs:31:9\n   |\nLL |     let i_think_continually = 2;\n   |         ^^^^^^^^^^^^^^^^^^^ help: consider using `_i_think_continually` instead: `_i_think_continually`\n   |\nnote: lint level defined here\n  --> /checkout/src/test/ui/lint/issue-47390-unused-variable-in-struct-pattern.rs:15:9\n   |\nLL | #![warn(unused)] // UI tests pass `-A unused` (#43896)\n   |         ^^^^^^\n   = note: #[warn(unused_variables)] implied by #[warn(unused)]\n\n"}
[00:49:15] {"message":"unused variable: `mut_unused_var`","code":{"code":"unused_variables","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/lint/issue-47390-unused-variable-in-struct-pattern.rs","byte_start":1001,"byte_end":1015,"line_start":38,"line_end":38,"column_start":13,"column_end":27,"is_primary":true,"text":[{"text":"    let mut mut_unused_var = 1;","highlight_start":13,"highlight_end":27}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[{"message":"consider using `_mut_unused_var` instead","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/lint/issue-47390-unused-variable-in-struct-pattern.rs","byte_start":1001,"byte_end":1015,"line_start":38,"line_end":38,"column_start":13,"column_end":27,"is_primary":true,"text":[{"text":"    let mut mut_unused_var = 1;","highlight_start":13,"highlight_end":27}],"label":null,"suggested_replacement":"_mut_unused_var","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unused variable: `mut_unused_var`\n  --> /checkout/src/test/ui/lint/issue-47390-unused-variable-in-struct-pattern.rs:_end":1050,"line_start":40,"line_end":40,"column_start":19,"column_end":29,"is_primary":true,"text":[{"text":"    let (mut var, unused_var) = (1, 2);","highlight_start":19,"highlight_end":29}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[{"message":"consider using `_unused_var` instead","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/lint/issue-47390-unused-variable-in-struct-pattern.rs","byte_start":1040,"byte_end":1050,"line_start":40,"line_end":40,"column_start":19,"column_end":29,"is_primary":true,"text":[{"text":"    let (mut var, unused_var) = (1, 2);","highlight_start":19,"highlight_end":29}],"label":null,"suggested_replacement":"_unused_var","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unused variable: `unused_var`\n  --> /checkout/src/test/ui/lint/issue-47390-unused-variable-in-struct-pattern.rs:40:19\n   |\nLL |     let (mut var, unused_var) = (1, 2);\n   |                   ^^^^^^^^^^ help: consider using `_unused_var` instead: `_unused_var`\n\n"}
[00:49:15] {"message":"unused variable: `corridors_of_light`","code":{"code":"unused_variables","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/lint/issue-47390-unused-variable-in-struct-pattern.rs","byte_start":1088,"byte_end":1106,"line_start":42,"line_end":42,"column_start":26,"column_end":44,"is_primary":true,"text":[{"text":"    if let SoulHistory { corridors_of_light,","highlight_start":26,"highlight_end":44}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[{"message":"try ignoring the field","code":null,"level":"help","spans":[                 ^^^^^^^^^^^^^^\n   |\n   = note: consider using `_hours_are_suns` instead\n\n"}
[00:49:15] {"message":"value assigned to `hours_are_suns` is never read","code":{"code":"unused_assignments","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/lint/issue-47390-unused-variable-in-struct-pattern.rs","byte_start":1247,"byte_end":1261,"line_start":45,"line_end":45,"column_start":9,"column_end":23,"is_primary":true,"text":[{"text":"        hours_are_suns = false;","highlight_start":9,"highlight_end":23}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[{"message":"lint level defined here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/lint/issue-47390-unused-variable-in-struct-pattern.rs","byte_start":542,"byte_end":548,"line_start":15,"line_end":15,"column_start":9,"column_end":15,"is_primary":true,"text":[{"text":"#![warn(unused)] // UI tests pass `-A unused` (#43896)","highlight_start":9,"highlight_end":15}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[],"rendered":null},{"message":"#[warn(unused_assignments)] implied by #[warn(unused)]","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: value assigned to `hours_are_suns` is never read\n  --> /checkout/src/test/ui/lint/issue-47390-unused-variable-in-struct-pattern.rs:45:9\n   |\nLL |         hours_are_suns = false;\n   |         ^^^^^^^^^^^^^^\n   |\nnote: lint level defined here\n  --> /checkout/src/test/ui/lint/issue-47390-unused-variable-in-struct-pattern.rs:15:9\n   |\nLL | #![warn(unused)] // UI tests pass `-A un  |\nLL |         (Large::Suit { case },) => {}\n   |                        ^^^^ help: try ignoring the field: `case: _`\n\n"}
[00:49:15] {"message":"unused variable: `case`","code":{"code":"unused_variables","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/lint/issue-47390-unused-variable-in-struct-pattern.rs","byte_start":1739,"byte_end":1743,"line_start":74,"line_end":74,"column_start":24,"column_end":28,"is_primary":true,"text":[{"text":"        [Large::Suit { case }] => {}","highlight_start":24,"highlight_end":28}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[{"message":"try ignoring the field","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/lint/issue-47390-unused-variable-in-struct-pattern.rs","byte_start":1739,"byte_end":1743,"line_start":74,"line_end":74,"column_start":24,"column_end":28,"is_primary":true,"text":[{"text":"        [Large::Suit { case }] => {}","highlight_start":24,"highlight_end":28}],"label":null,"suggested_replacement":"case: _","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unused variable: `case`\n  --> /checkout/src/test/ui/lint/issue-47390-unused-variable-in-struct-pattern.rs:74:24\n   |\nLL |         [Large::Suit { case }] => {}\n   |                        ^^^^ help: try ignoring the field: `case: _`\n\n"}
[00:49:15] {"message":"unused variable: `case`","code":{"code":"unused_variables","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/lint/issue-47390-unused-variable-in-struct-pattern.rs","byte_start":1848,"byte_end":1852,"line_start":79,"line_end"  |         ^^^^^^^^^^^ help: consider using `_theOtherTwo` instead
[00:49:15] +    |         ^^^^^^^^^^^ help: consider using `_theOtherTwo` instead: `_theOtherTwo`
[00:49:15] 7 note: lint level defined here
[00:49:15] 8   --> $DIR/issue-24690.rs:18:9
[00:49:15] 
[00:49:15] 
[00:49:15] 
[00:49:15] The actual stderr differed from the expected stderr.
[00:49:15] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-24690.stderr
[00:49:15] To update references, run this command from build directory:
[00:49:15] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'span/issue-24690.rs'
[00:49:15] error: 1 errors occurred comparing output.
[00:49:15] status: exit code: 101
[00:49:15] status: exit code: 101
[00:49:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/issue-24690.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-24690.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-24690.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[00:49:15] ------------------------------------------
[00:49:15] 
[00:49:15] ------------------------------------------
[00:49:15] stderr:
[00:49:15] stderr:
[00:49:15] ------------------------------------------
[00:49:15] {"message":"unused variable: `theOtherTwo`","code":{"code":"unused_variables","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/span/issue-24690.rs","byte_start":889,"byte_end":900,"line_start":23,"line_end":23,"column_start":9,"column_end":20,"is_primary":true,"text":[{"text":"    let theOtherTwo = 2; //~ WARN should have a snake case name","highlight_start":9,"highlight_end":20}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[{"message":"lint level defined here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/span/issue-24690.rs","byte_start":752,"byte_end":758,"line_start":18,"line_end":18,"column_start":9,"column_end":15,"is_primary":true,"text":[{"text":"#![warn(unused)]","highlight_start":9,"highlight_end":15}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[],"rendered":null},{"message":"#[warn(unused_variables)] implied by #[warn(unused)]","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider using `_theOtherTwo` instead","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/span/issue-24690.rs","byte_start":889,"byte_end":900,"line_start":23,"line_end":23,"column_start":9,"column_end":20,"is_primary":true,"text":[{"text":"    let theOtherTwo = 2; //~ WARN should have a snake case name","highlight_start":9,"highlight_end":20}],"label":null,"suggested_replacement":"_theOtherTwo","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unused variable: `theOtherTwo`\n  --> /checkout/src/test/ui/span/issue-24690.rs:23:9\n   |\nLL |     let theOt4018180 .
2744088 ./obj/build
1969036 ./obj/build/x86_64-unknown-linux-gnu
725904 ./src
584836 ./obj/build/bootstrap
---
149784 ./.git/modules
149780 ./.git/modules/src
149116 ./src/llvm-emscripten/test
124332 ./obj/build/bootstrap/debug/incremental/bootstrap-182x3aewwy26b
124328 ./obj/build/bootstrap/debug/incremental/bootstrap-182x3aewwy26b/s-f0ycmz0zyv-1ha8v8l-15rj8twogke02
