\n\nRust only looks at the signature of the called function, as such it must\nalready specify all requirements that will be used for every type parameter.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/type-check-defaults.rs","byte_start":1396,"byte_end":1471,"line_start":34,"line_end":34,"column_start":1,"column_end":76,"is_primary":true,"text":[{"text":"trait ProjectionPred<T:Iterator = IntoIter<i32>> where T::Item : Add<u8> {}","highlight_start":1,"highlight_end":76}],"label":"no implementation for `i32 + u8`","suggested_replacement":null,"expansion":null}],"children":[{"message":"the trait `std::ops::Add<u8>` is not implemented for `i32`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0277]: cannot add `u8` to `i32`\n  --> /checkout/src/test/ui/type-check-defaults.rs:34:1\n   |\nLL | trait ProjectionPred<T:Iterator = IntoIter<i32>> where T::Item : Add<u8> {}\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no implementation for `i32 + u8`\n   |\n   = help: the trait `std::ops::Add<u8>` try `rustc --explain E0282`.
[00:48:32] 
[00:48:32] 
[00:48:32] The actual stderr differed from the expected stderr.
[00:48:32] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-check/issue-40294.stderr
[00:48:32] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-check/issue-40294.stderr
[00:48:32] To update references, run this command from build directory:
[00:48:32] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'type-check/issue-40294.rs'
[00:48:32] error: 1 errors occurred comparing output.
[00:48:32] status: exit code: 101
[00:48:32] status: exit code: 101
[00:48:32] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-check/issue-40294.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-check/issue-40294.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-check/issue-40294.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[00:48:32] ------------------------------------------
[00:48:32] 
[00:48:32] ------------------------------------------
[00:48:32] stderr:
[00:48:32] stderr:
[00:48:32] ------------------------------------------
[00:48:32] {"message":"type annotations needed","code":{"code":"E0282","explanation":"\nThis error indicates that type inference did not result in one unique possible\ntype, and extra information is required. In most cases this can be provided\nby adding a type annotation. Sometimes you need to specify a generic type\nparameter manually.\n\nA common example is the `collect` method on `Iterator`. It has a generic type\nparameter with a `FromIterator` bound, which for a `char` iterator is\nimplemented by `Vec` and `String` among others. Consider the following snippet\nthat reverses the characters of a string:\n\n