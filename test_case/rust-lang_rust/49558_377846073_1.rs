\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/impl-trait/method-suggestion-no-duplication.rs","byte_start":510,"byte_end":521,"line_start":14,"line_end":14,"column_start":1,"column_end":12,"is_primary":false,"text":[{"text":"struct Foo;","highlight_start":1,"highlight_end":12}],"label":"method `is_empty` not found for this","suggested_replacement":null,"expansion":null},{"file_name":"/checkout/src/test/ui/impl-trait/method-suggestion-no-duplication.rs","byte_start":589,"byte_end":597,"line_start":19,"line_end":19,"column_start":15,"column_end":23,"is_primary":true,"text":[{"text":"    foo(|s| s.is_empty());","highlight_start":15,"highlight_end":23}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[{"message":"items from traits can only be used if the trait is implemented and in scope","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"the following traits define an item `is_empty`, perhaps you need to implement one of them:\ncandidate #1: `core::str::StrExt`\ncandidate #2: `core::slice::SliceExt`\ncandidate #3: `std::iter::ExactSizeIterator`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0599]: no method named `is_empty` found for type `Foo` in the current scope\n  --> /checkout/src/test/ui/impl-trait/method-suggestion-no-duplication.rs:19:15\n   |\nLL | struct Foo;\n   | ----------- method `is_empty` not found for this\n...\nLL |     foo(|s| s.is_empty());\n   |               ^^^^^^^^\n   |\n   = help: items from traits can only be used if the trait is implemented and in scope\n   = note: the following traits define an item `is_empty`, perhaps you need to implement one of them:\n           candidate #1: `core::str::StrExt`\n           candidate #2: `core::slice::SliceExt`\n           candidate #3: `std::iter::ExactSizeIterator`\n\n"}
[00:40:01] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:40:01] {"message":"For more information about this error  |
[00:40:01] 107    = help: items from traits can only be used if the trait is implemented and in scope
[00:40:01] 108    = note: the following traits define an item `method`, perhaps you need to implement one of them:
[00:40:01] -            candidate #1: `foo::Bar`
[00:40:01] -            candidate #2: `no_method_suggested_traits::foo::PubPub`
[00:40:01] -            candidate #3: `no_method_suggested_traits::qux::PrivPub`
[00:40:01] -            candidate #4: `no_method_suggested_traits::Reexported`
[00:40:01] +            candidate #1: `no_method_suggested_traits::Reexported`
[00:40:01] +            candidate #2: `no_method_suggested_traits::qux::PrivPub`
[00:40:01] +            candidate #3: `no_method_suggested_traits::foo::PubPub`
[00:40:01] +            candidate #4: `foo::Bar`
[00:40:01] 113
[00:40:01] 114 error[E0599]: no method named `method2` found for type `u64` in the current scope
---
[00:40:01] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'impl-trait/no-method-suggested-traits.rs'
[00:40:01]
[00:40:01] error: 1 errors occurred comparing output.
[00:40:01] status: exit code: 101
[00:40:01] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/no-method-suggested-traits.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/no-method-suggested-traits.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/no-method-suggested-traits.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
---
[00:40:01] {"message":"no method named `method` found for type `u32` in the current scope","code":{"code":"E0599","explanation":"\nThis error occurs when a method is used on a type which doesn't implement it:\n\nErroneous code example:\n\n