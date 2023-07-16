\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/impl-trait/auto-trait-leak.rs","byte_start":1427,"byte_end":1437,"line_start":48,"line_end":48,"column_start":16,"column_end":26,"is_primary":true,"text":[{"text":"fn cycle2() -> impl Clone {","highlight_start":16,"highlight_end":26}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[{"message":"...which requires processing `cycle2::{{impl-Trait}}`...","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/impl-trait/auto-trait-leak.rs","byte_start":1412,"byte_end":1437,"line_start":48,"line_end":48,"column_start":1,"column_end":26,"is_primary":true,"text":[{"text":"fn cycle2() -> impl Clone {","highlight_start":1,"highlight_end":26}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[],"rendered":null},{"message":"...which requires processing `cycle2`...","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/impl-trait/auto-trait-leak.rs","byte_start":1341,"byte_end":1351,"line_start":42,"line_end":42,"column_start":16,"column_end":26,"is_primary":true,"text":[{"text":"fn cycle1() -> impl Clone {","highlight_start":16,"highlight_end":26}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[],"rendered":null},{"message":"...which requires processing `cycle1::{{impl-Trait}}`...","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/impl-trait/auto-trait-leak.rs","byte_start":1341,"byte_end":1351,"line_start":42,"line_end":42,"column_start":16,"column_end":26,"is_primary":true,"text":[{"text":"fn cycle1() -> impl Clone {","highlight_start":16,"highlight_end":26}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[],"rendered":null},{"message":"...which again requires processing `cycle1`, completing the cycle","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"cycle used when type-check\n"}
[00:44:35] {"message":"Some errors occurred: E0277, E0391.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0277, E0391.\n"}
[00:44:35] {"message":"For more information about an error, try `rustc --explain E0277`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0277`.\n"}
---
[00:44:35] 1 error[E0391]: cycle detected when computing the supertraits of `t1`
[00:44:35] -    |
[00:44:35] - note: ...which requires computing the supertraits of `t2`...
[00:44:35] 4   --> $DIR/issue-12511.rs:11:1
[00:44:35] 5    |
[00:44:35] 6 LL | trait t1 : t2 {
[00:44:35]
[00:44:35] 7    | ^^^^^^^^^^^^^
[00:44:35] - note: ...which again requires computing the supertraits of `t1`, completing the cycle
[00:44:35] +    |
[00:44:35] + note: ...which requires computing the supertraits of `t2`...
[00:44:35] 9   --> $DIR/issue-12511.rs:14:1
[00:44:35] 10    |
[00:44:35] 11 LL | trait t2 : t1 {
[00:44:35]
[00:44:35] 12    | ^^^^^^^^^^^^^
[00:44:35] +    = note: ...which again requires computing the supertraits of `t1`, completing the cycle
---
[00:44:35] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'issue-12511.rs'
[00:44:35]
[00:44:35] error: 1 errors occurred comparing output.
[00:44:35] status: exit code: 101
[00:44:35] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issue-12511.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-12511.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-12511.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
---
[00:44:35] {"message":"cycle detected when computing the supertraits of `t1`","code":{"code":"E0391","explanation":"\nThis error indicates that some types or traits depend on each other\nand therefore cannot be constructed.\n\nThe following example contains a circular dependency between two traits:\n\n