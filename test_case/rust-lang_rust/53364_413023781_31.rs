\n\nNote that type parameters for enum-variant constructors go after the variant,\nnot after the enum (`Option::None::<u32>`, not `Option::<u32>::None`).\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rfc1598-generic-associated-types/parameter_number_and_kind.rs","byte_start":1232,"byte_end":1233,"line_start":31,"line_end":31,"column_start":38,"column_end":39,"is_primary":true,"text":[{"text":"    type FErr2<T> = Self::E<'static, T, u32>; // Error","highlight_start":38,"highlight_end":39}],"label":"type parameter not allowed","suggested_replace = T>;
[00:52:18] + 
[00:52:18] + 
[00:52:18] + warning: generic associated type arguments as part of the `generic_associated_types` feature are currently incomplete and their use may result in unintended behaviour, such as compiler crashes
[00:52:18] +   --> $DIR/pointer_family.rs:28:5
[00:52:18] +    |
[00:52:18] + LL |     type Pointer<T> = Arc<T>;
[00:52:18] + 
[00:52:18] + 
[00:52:18] + warning: generic associated type arguments as part of the `generic_associated_types` feature are currently incomplete and their use may result in unintended behaviour, such as compiler crashes
[00:52:18] +   --> $DIR/pointer_family.rs:38:5
[00:52:18] +    |
[00:52:18] + LL |     type Pointer<T> = Rc<T>;
[00:52:18] + 
[00:52:18] + 
[00:52:18] 1 error[E0109]: type parameters are not allowed on this type
[00:52:18] 2   --> $DIR/pointer_family.rs:46:21
[00:52:18] 
[00:52:18] 
[00:52:18] The actual stderr differed from the expected stderr.
[00:52:18] The actual stderr differed from the expected stderr.
[00:52:18] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc1598-generic-associated-types/pointer_family/pointer_family.stderr
[00:52:18] To update references, rerun the tests and pass the `--bless` flag
[00:52:18] To only update this specific test, also pass `--test-args rfc1598-generic-associated-types/pointer_family.rs`
[00:52:18] error: 1 errors occurred comparing output.
[00:52:18] status: exit code: 1
[00:52:18] status: exit code: 1
[00:52:18] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc1598-generic-associated-types/pointer_family.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc1598-generic-associated-types/pointer_family/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc1598-generic-associated-types/pointer_family/auxiliary" "-A" "unused"
[00:52:18] ------------------------------------------
[00:52:18] 
[00:52:18] ------------------------------------------
[00:52:18] stderr:
[00:52:18] stderr:
[00:52:18] ------------------------------------------
[00:52:18] {"message":"generic associated type parameters as part of the `generic_associated_types` feature are currently incomplete and their use may result in unintended behaviour, such as compiler crashes","code":null,"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/rfc1598-generic-associated-types/pointer_family.rs","byte_start":681,"byte_end":716,"line_start":20,"line_end":20,"column_start":5,"column_end":40,"is_primary":true,"text":[{"text":"    type Pointer<T>: Deref<Target = T>;","highlight_start":5,"highlight_end":40}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: generic associated type parameters as part of the `generic_associated_types` feature are currently incomplete and their use may result in unintended behaviour, such as compiler crashes\n  --> /checkout/src/test/ui/rfc1598-generic-associated-types/poind on this type\n  --> /checkout/src/test/ui/rfc1598-generic-associated-types/pointer_family.rs:46:21\n   |\nLL |     bar: P::Pointer<String>,\n   |                     ^^^^^^ type parameter not allowed\n\n"}
[00:52:18] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[00:52:18] {"message":"type parameters are not allowed on this type","code":{"code":"E0109","explanation":"\nYou tried to give a type parameter to a type which doesn't need it. Erroneous\ncode example:\n\n