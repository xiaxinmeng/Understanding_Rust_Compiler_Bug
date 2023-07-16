\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-20772.rs","byte_start":0,"byte_end":119,"line_start":1,"line_end":4,"column_start":1,"column_end":3,"is_primary":true,"text":[{"text":"trait T : Iterator<Item=Self::Item>","highlight_start":1,"highlight_end":36},{"text":"//~^ ERROR cycle detected","highlight_start":1,"highlight_end":26},{"text":"//~| ERROR associated type `Item` not found for `Self`","highlight_start":1,"highlight_end":55},{"text":"{}","highlight_start":1,"highlight_end":3}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"...which again requires computing the supertraits of `T`, completing the cycle","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"cycle used when collecting item types in top-level module","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-20772.rs","byte_start":0,"byte_end":119,"line_start":1,"line_end":4,"column_start":1,"column_end":3,"is_primary":true,"text":[{"text":"trait T : Iterator<Item=Self::Item>","highlight_start":1,"highlight_end":36},{"text":"//~^ ERROR cycle detected","highlight_start":1,"highlight_end":26},{"text":"//~| ERROR associated type `Item` not found for `Self`","highlight_start":1,"highlight_end":55},{"text":"{}","highlight_start":1,"highlight_end":3}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0391]: cycle detected when computing the supertraits of `T`\n  --> /checkout/src/test/ui/issues/issue-20772.rs:1:1\n   |\nLL | / trait T : Iterator<Item=Self::Item>\nLL | | //~^ ERROR cycle detected\nLL | | //~| ERROR associated type `Item` not found for `Self`\nLL | | {}\n   | |__^\n   |\n   = note: ...which again requires computing the supertraits of `T`, completing the cycle\nnote: cycle used when collecting item types in top-level module\n  --> /checkout/src/test/ui/issues/issue-20772.rs:1:1\n   |\nLL | / trait T : Iterator<Item=Self::Item>\nLL | | //~^ ERROR cycle detected\nLL | | //~| ERROR associated type `Item` not found for `Self`\nLL | | {}\n   | |__^\n\n"}
[01:10:42] {"message":"For more information about this error, try `rustc --explain E0391`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0391`.\n"}
[01:10:42] 
[01:10:42] ------------------------------------------
[01:10:42] 
[01:10:42] 
[01:10:42] thread '[ui] ui/issues/issue-20772.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3422:9
[01:10:42] 
[01:10:42] ---- [ui] ui/issues/issue-21177.rs stdout ----
[01:10:42] diff of stderr:
[01:10:42] 
[01:10:42] 11 LL | fn foo<T: Trait<A = T::B>>() { }
[01:10:42] 13 
[01:10:42] 13 
[01:10:42] - error[E0220]: associated type `B` not found for `T`
[01:10:42] -    |
[01:10:42] -    |
[01:10:42] - LL | fn foo<T: Trait<A = T::B>>() { }
[01:10:42] -    |                     ^^^^ associated type `B` not found
[01:10:42] + error: aborting due to previous error
[01:10:42] - error: aborting due to 2 previous errors
[01:10:42] - 
[01:10:42] - Some errors occurred: E0220, E0391.
[01:10:42] - For more information about an error, try `rustc --explain E0220`.
[01:10:42] - For more information about an error, try `rustc --explain E0220`.
[01:10:42] + For more information about this error, try `rustc --explain E0391`.
[01:10:42] 24 
[01:10:42] 
[01:10:42] 
[01:10:42] The actual stderr differed from the expected stderr.
[01:10:42] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-21177/issue-21177.stderr
[01:10:42] To update references, rerun the tests and pass the `--bless` flag
[01:10:42] To only update this specific test, also pass `--test-args issues/issue-21177.rs`
[01:10:42] error: 1 errors occurred comparing output.
[01:10:42] status: exit code: 1
[01:10:42] status: exit code: 1
[01:10:42] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-21177.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-21177/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-21177/auxiliary" "-A" "unused"
[01:10:42] ------------------------------------------
[01:10:42] 
[01:10:42] ------------------------------------------
[01:10:42] stderr:
[01:10:42] stderr:
[01:10:42] ------------------------------------------
[01:10:42] {"message":"cycle detected when computing the bounds for type parameter `T`","code":{"code":"E0391","explanation":"\nThis error indicates that some types or traits depend on each other\nand therefore cannot be constructed.\n\nThe following example contains a circular dependency between two traits:\n\n