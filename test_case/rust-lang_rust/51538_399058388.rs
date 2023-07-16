plain
[00:44:54] ....................................................................................................
[00:44:58] ....................................................................................................
[00:45:01] ....................................................................................................
[00:45:04] ....................................................................................................
[00:45:09] ..............................................................................F.....................
[00:45:19] ....................................................................................................
[00:45:25] ....................................................................................................
[00:45:30] ......i...............................................................................i.............
[00:45:35] ....................................................................................................
[00:45:35] ....................................................................................................
[00:45:41] ....................................................................................................
[00:45:47] ....................................................................................................
[00:45:53] .....................i..............................................................................
Iterator>::Item
[00:45:53] +    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::iter::Step` is not implemented for `impl std::fmt::Debug`
[00:45:53] +    |
[00:45:53] +    = note: required because of the requirements on the impl of `std::iter::Iterator` for `std::ops::Range<impl std::fmt::Debug>`
[00:45:53] + error: aborting due to 8 previous errors
[00:45:53] + 
[00:45:53] + Some errors occurred: E0223, E0277, E0308, E0667.
[00:45:53] 36 For more information about an error, try `rustc --explain E0223`.
[00:45:53] 36 For more information about an error, try `rustc --explain E0223`.
[00:45:53] 37 
[00:45:53] 
[00:45:53] 
[00:45:53] The actual stderr differed from the expected stderr.
[00:45:53] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl_trait_projections/impl_trait_projections.stderr
[00:45:53] To update references, rerun the tests and pass the `--bless` flag
[00:45:53] To only update this specific test, also pass `--test-args impl_trait_projections.rs`
[00:45:53] error: 1 errors occurred comparing output.
[00:45:53] status: exit code: 101
[00:45:53] status: exit code: 101
[00:45:53] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl_trait_projections.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl_trait_projections/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl_trait_projections/auxiliary" "-A" "unused"
[00:45:53] ------------------------------------------
[00:45:53] 
[00:45:53] ------------------------------------------
[00:45:53] stderr:
[00:45:53] stderr:
[00:45:53] ------------------------------------------
[00:45:53] {"message":"`impl Trait` is not allowed in path parameters","code":{"code":"E0667","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/impl_trait_projections.rs","byte_start":720,"byte_end":733,"line_start":21,"line_end":21,"column_start":51,"column_end":64,"is_primary":true,"text":[{"text":"fn projection_is_disallowed(x: impl Iterator) -> <impl Iterator>::Item {","highlight_start":51,"highlight_end":64}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0667]: `impl Trait` is not allowed in path parameters\n  --> /checkout/src/test/ui/impl_trait_projections.rs:21:51\n   |\nLL | fn projection_is_disallowed(x: impl Iterator) -> <impl Iterator>::Item {\n   |                                                   ^^^^^^^^^^^^^\n\n"}
[00:45:53] {"message":"`impl Trait` is not allowed in path parameters","code":{"code":"E0667","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/impl_trait_projections.rs","byte_start":935,"byte_end":948,"line_start":28,"line_end":28,"column_start":9,"column_end":22,"is_primary":true,"text":[{"text":"    -> <impl Iterator as Iterator>::Item","highlight_start":9,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0667]: `impl Trait` is not allowed in path parameters\n  --> /checkout/src/test/ui/impl_trait_projections.rs:28:9\n   |\nLL |     -> <impl Iterator as Iterator>::Item\n   |         ^^^^^^^^^^^^^\n\n"}
[00:45:53] {"message":"`impl Trait` is not allowed in path parameters","code":{"code":"E0667","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/impl_trait_projections.rs","byte_start":1138,"byte_end":1148,"line_start":35,"line_end":35,"column_start":27,"column_end":37,"is_primary":true,"text":[{"text":"    -> <::std::ops::Range<impl Debug> as Iterator>::Item","highlight_start":27,"highlight_end":37}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0667]: `impl Trait` is not allowed in path parameters\n  --> /checkout/src/test/ui/impl_trait_projections.rs:35:27\n   |\nLL |     -> <::std::ops::Range<impl Debug> as Iterator>::Item\n   |                           ^^^^^^^^^^\n\n"}
[00:45:53] {"message":"`impl Trait` is not allowed in path parameters","code":{"code":"E0667","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/impl_trait_projections.rs","byte_start":1355,"byte_end":1365,"line_start":42,"line_end":42,"column_start":29,"column_end":39,"is_primary":true,"text":[{"text":"    -> <dyn Iterator<Item = impl Debug> as Iterator>::Item","highlight_start":29,"highlight_end":39}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0667]: `impl Trait` is not allowed in path parameters\n  --> /checkout/src/test/ui/impl_trait_projections.rs:42:29\n   |\nLL |     -> <dyn Iterator<Item = impl DeE0277\nfn some_func<T>(foo: T) {\n    println!(\"{:?}\", foo); // error: the trait `core::fmt::Debug` is not\n                           //        implemented for the type `T`\n}\n\nfn main() {\n    // We now call the method with the i32 type,\n    // which *does* implement the Debug trait.\n    some_func(5i32);\n}\n