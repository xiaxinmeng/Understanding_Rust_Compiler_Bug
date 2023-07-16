\n\nThis syntax specifies that we want the X type from MyTrait, as made concrete in\nMyStruct. The reason that we cannot simply use `MyStruct::X` is that MyStruct\nmight implement two different traits with identically-named associated types.\nThis syntax allows disambiguation between the two.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/traits/trait-item-privacy.rs","byte_start":3591,"byte_end":3595,"line_start":129,"line_end":129,"column_start":12,"column_end":16,"is_primary":true,"text":[{"text":"    let _: S::C; //~ ERROR ambiguous associated type","highlight_start":12,"highlight_end":16}],"label":"ambiguous associated type","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"specify the type using the syntax `<S as Trait>::C`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0223]: ambiguous associated type\n  --> /checkout/src/test/ui/traits/trait-item-privacy.rs:129:12\n   |\nLL |     let _: S::C; //~ ERROR ambiguous associated type\n   |            ^^^^ ambiguous associated type\n   |\n   = note: specify the type using the syntax `<S as Trait>::C`\n\n"}
[00:46:20] {"message":"associated type `A` is private","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/traits/trait-item-privacy.rs","byte_start":3730,"byte_end":3734,"line_start":131,"line_end":131,"column_start":12,"column_end":16,"is_primary":true,"text":[{"text":"    let _: T::A; //~ ERROR associated type `A` is private","highlight_start":12,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: associated type `A` is private\n  --> /checkout/src/test/ui/t/ui/traits/trait-item-privacy.rs","byte_start":3869,"byte_end":3909,"line_start":136,"line_end":138,"column_start":12,"column_end":6,"is_primary":true,"text":[{"text":"    let _: assoc_ty::B<","highlight_start":12,"highlight_end":24},{"text":"        B = u8, // OK","highlight_start":1,"highlight_end":22},{"text":"    >;","highlight_start":1,"highlight_end":6}],"label":"doesn't have a size known at compile-time","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"the trait `std::marker::Sized` is not implemented for `dyn assoc_ty::B<B=u8>`","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"to learn more, visit <https://doc.rust-lang.org/book/second-edition/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"the return type of a function must have a statically known size","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0277]: the size for values of type `dyn assoc_ty::B<B=u8>` cannot be known at compilation time\n  --> /checkout/src/test/ui/traits/trait-item-privacy.rs:136:12\n   |\nLL |       let _: assoc_ty::B<\n   |  ____________^\nLL | |         B = u8, // OK\nLL | |     >;\n   | |_____^ doesn't have a size known at compile-time\n   |\n   = help: the trait `std::marker::Sized` is not implemented for `dyn assoc_ty::B<B=u8>`\n   = note: to learn more, visit <https://doc.rust-lang.org/book/second-edition/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>\n   = note: the return typerror, try `rustc --explain E0038`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0038`.\n"}
[00:46:20] ------------------------------------------
[00:46:20] 
[00:46:20] thread '[ui] ui/traits/trait-item-privacy.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3189:9
[00:46:20] 
[00:46:20] 
[00:46:20] ---- [ui] ui/traits/trait-object-macro-matcher.rs stdout ----
[00:46:20] diff of stderr:
[00:46:20] 
[00:46:20] 4 LL |     m!('static +); //~ ERROR at least one non-builtin trait is required for an object type
[00:46:20] 6 
[00:46:20] 6 
[00:46:20] + error[E0277]: the size for values of type `(dyn std::marker::Copy + std::marker::Send + 'static)` cannot be known at compilation time
[00:46:20] +   --> $DIR/trait-object-macro-matcher.rs:18:8
[00:46:20] +    |
[00:46:20] + LL |     m!(Copy + Send + 'static); //~ ERROR the trait `std::marker::Copy` cannot be made into an object
[00:46:20] +    |
[00:46:20] +    |
[00:46:20] +    = help: the trait `std::marker::Sized` is not implemented for `(dyn std::marker::Copy + std::marker::Send + 'static)`
[00:46:20] +    = note: the return type of a function must have a statically known size
[00:46:20] + 
[00:46:20] + 
[00:46:20] + error[E0277]: the size for values of type `(dyn std::marker::Send + 'static)` cannot be known at compilation time
r.rs`
[00:46:20] error: 1 errors occurred comparing output.
[00:46:20] status: exit code: 1
[00:46:20] status: exit code: 1
[00:46:20] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/trait-object-macro-matcher.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-object-macro-matcher/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-object-macro-matcher/auxiliary" "-A" "unused"
[00:46:20] ------------------------------------------
[00:46:20] 
[00:46:20] ------------------------------------------
[00:46:20] stderr:
[00:46:20] stderr:
[00:46:20] ------------------------------------------
[00:46:20] {"message":"at least one non-builtin trait is required for an object type","code":{"code":"E0224","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/traits/trait-object-macro-matcher.rs","byte_start":706,"byte_end":715,"line_start":20,"line_end":20,"column_start":8,"column_end":17,"is_primary":true,"text":[{"text":"    m!('static +); //~ ERROR at least one non-builtin trait is required for an object type","highlight_start":8,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0224]: at least one non-builtin trait is required for an object type\n  --> /checkout/src/test/ui/traits/trait-object-macro-matcher.rs:20:8\n   |\nLL |     m!('static +); //~ ERROR at least one non-builtin trait is required for an object type\n   |        ^^^^^^^^^\n\n"}
[00:46:20] {"message":"the size for values of type `(dyn std::marker::Copy + std::marker::Send + 'static)` cannot be known at compilation time","code":{"code":"E0277","explanation":"\nYou tried to use a type which doesn't implement some trait in a place which\nexpected that trait. Erroneous code example:\n\n