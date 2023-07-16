\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/cycle-projection-based-on-where-clause.rs","byte_start":548,"byte_end":555,"line_start":17,"line_end":17,"column_start":19,"column_end":26,"is_primary":true,"text":[{"text":"          T : Add<T::Item>","highlight_start":19,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"...which again requires computing the bounds for type parameter `T`, completing the cycle","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"cycle used when processing `A`","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/cycle-projection-based-on-where-clause.rs","byte_start":548,"byte_end":555,"line_start":17,"line_end":17,"column_start":19,"column_end":26,"is_primary":true,"text":[{"text":"          T : Add<T::Item>","highlight_start":19,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0391]: cycle detected when computing the bounds for type parameter `T`\n  --> /checkout/src/test/ui/cycle-projection-based-on-where-clause.rs:17:19\n   |\nLL |           T : Add<T::Item>\n   |                   ^^^^^^^\n   |\n   = note: ...which again requires computing the bounds for type parameter `T`, completing the cycle\nnote: cycle used when processing `A`\n  --> /checkout/src/test/ui/cycle-projection-based-on-where-clause.rs:17:19\n   |\nLL |           T : Add<T::Item>\n   |                   ^^^^^^^\n\n"}
[01:10:42] {"message":"For more information about this error, try `rustc --explain E0391`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0391`.\n"}
[01:10:42] 
[01:10:42] ------------------------------------------
[01:10:42] 
---
[01:10:42] 
[01:10:42] 17 LL | | {}
[01:10:42] 18    | |__^
[01:10:42] 19 
[01:10:42] - error[E0220]: associated type `Item` not found for `Self`
[01:10:42] -    |
[01:10:42] -    |
[01:10:42] - LL | trait T : Iterator<Item=Self::Item>
[01:10:42] -    |                         ^^^^^^^^^^ associated type `Item` not found
[01:10:42] + error: aborting due to previous error
[01:10:42] - error: aborting due to 2 previous errors
[01:10:42] - 
[01:10:42] - Some errors occurred: E0220, E0391.
[01:10:42] - For more information about an error, try `rustc --explain E0220`.
[01:10:42] - For more information about an error, try `rustc --explain E0220`.
[01:10:42] + For more information about this error, try `rustc --explain E0391`.
[01:10:42] 30 
[01:10:42] 
[01:10:42] 
[01:10:42] The actual stderr differed from the expected stderr.
[01:10:42] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-20772/issue-20772.stderr
[01:10:42] To update references, rerun the tests and pass the `--bless` flag
[01:10:42] To only update this specific test, also pass `--test-args issues/issue-20772.rs`
[01:10:42] error: 1 errors occurred comparing output.
[01:10:42] status: exit code: 1
[01:10:42] status: exit code: 1
[01:10:42] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-20772.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-20772/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-20772/auxiliary" "-A" "unused"
[01:10:42] ------------------------------------------
[01:10:42] 
[01:10:42] ------------------------------------------
[01:10:42] stderr:
[01:10:42] stderr:
[01:10:42] ------------------------------------------
[01:10:42] {"message":"cycle detected when computing the supertraits of `T`","code":{"code":"E0391","explanation":"\nThis error indicates that some types or traits depend on each other\nand therefore cannot be constructed.\n\nThe following example contains a circular dependency between two traits:\n\n