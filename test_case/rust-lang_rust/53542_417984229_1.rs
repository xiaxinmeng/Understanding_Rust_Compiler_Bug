\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/existential_types/no_inferrable_concrete_type.rs","byte_start":698,"byte_end":725,"line_start":16,"line_end":16,"column_start":1,"column_end":28,"is_primary":true,"text":[{"text":"existential type Foo: Copy; //~ cycle detected","highlight_start":1,"highlight_end":28}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"...which requires processing `bar`...","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/existential_types/no_inferrable_concrete_type.rs","byte_start":809,"byte_end":814,"line_start":19,"line_end":19,"column_start":23,"column_end":28,"is_primary":true,"text":[{"text":"fn bar(x: Foo) -> Foo { x }","highlight_start":23,"highlight_end":28}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"...which again requires processing `Foo`, completing the cycle","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0391]: cycle detected when processing `Foo`\n  --> /checkout/src/test/ui/existential_types/no_inferrable_concrete_type.rs:16:1\n   |\nLL | existential type Foo: Copy; //~ cycle detected\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\nnote: ...which requires processing `bar`...\n  --> /checkout/src/test/ui/existential_types/no_inferrable_concrete_type.rs:19:23\n   |\nLL | fn bar(x: Foo) -> Foo { x }\n   |                       ^^^^^\n   = note: ...which again requires processing `Foo`, completing the cycle\n\n"}
[00:47:07] {"message":"defining existential type use differs from previous","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/existential_types/no_inferrable_concrete_type.rs","byte_start":816,"byte_end":872,"line_start":21,"line_end":23,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"fn main() {","highlight_start":1,"highlight_end":12},{"text":"    let _: Foo = std::mem::transmute(0u8);","highlight_start":1,"highlight_end":43},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"previous use here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/existential_types/no_inferrable_concrete_type.rs","byte_start":787,"byte_end":814,"line_start":19,"line_end":19,"column_start":1,"column_end":28,"is_primary":true,"text":[{"text":"fn bar(x: Foo) -> Foo { x }","highlight_start":1,"highlight_end":28}],"label":null,"suggested_replacement":null,"suggestion_applicab`impl Trait` not allowed outside of function and inherent method return types or bindings
[00:47:07] 20   --> $DIR/where-allowed.rs:28:40
[00:47:07] 21    |
[00:47:07] 22 LL | fn in_fn_parameter_in_parameters(_: fn(impl Debug)) { panic!() }
[00:47:07] 23    |                                        ^^^^^^^^^^
[00:47:07] 24 
[00:47:07] 24 
[00:47:07] - error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[00:47:07] + error[E0562]: `impl Trait` not allowed outside of function and inherent method return types or bindings
[00:47:07] 26   --> $DIR/where-allowed.rs:32:42
[00:47:07] 27    |
[00:47:07] 28 LL | fn in_fn_return_in_parameters(_: fn() -> impl Debug) { panic!() }
[00:47:07] 29    |                                          ^^^^^^^^^^
[00:47:07] 30 
[00:47:07] 30 
[00:47:07] - error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[00:47:07] + error[E0562]: `impl Trait` not allowed outside of function and inherent method return types or bindings
[00:47:07] 32   --> $DIR/where-allowed.rs:36:38
[00:47:07] 33    |
[00:47:07] 34 LL | fn in_fn_parameter_in_return() -> fn(impl Debug) { panic!() }
[00:47:07] 35    |                                      ^^^^^^^^^^
[00:47:07] 36 
[00:47:07] 36 
[00:47:07] - error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[00:47:07] + error[E0562]: `impl Trait` not allowed outside of function and inherent method return types or bindings
[00:47:07] 38   --> $DIR/where-allowed.rs:40:40
[00:47:07] 39    |
[00:47:07] 40 LL | fn in_fn_return_in_return() -> fn() -> impl Dnd inherent method return types or bindings
[00:47:07] 80   --> $DIR/where-allowed.rs:69:57
[00:47:07] 81    |
[00:47:07] 82 LL | fn in_impl_Fn_parameter_in_return() -> &'static impl Fn(impl Debug) { panic!() }
[00:47:07] 83    |                                                         ^^^^^^^^^^
[00:47:07] 84 
[00:47:07] 84 
[00:47:07] - error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[00:47:07] + error[E0562]: `impl Trait` not allowed outside of function and inherent method return types or bindings
[00:47:07] 86   --> $DIR/where-allowed.rs:74:59
[00:47:07] 87    |
[00:47:07] 88 LL | fn in_impl_Fn_return_in_return() -> &'static impl Fn() -> impl Debug { panic!() }
[00:47:07] 89    |                                                           ^^^^^^^^^^
[00:47:07] 90 
[00:47:07] 90 
[00:47:07] - error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[00:47:07] + error[E0562]: `impl Trait` not allowed outside of function and inherent method return types or bindings
[00:47:07] 92   --> $DIR/where-allowed.rs:78:38
[00:47:07] 93    |
[00:47:07] 94 LL | fn in_Fn_parameter_in_generics<F: Fn(impl Debug)> (_: F) { panic!() }
[00:47:07] 95    |                                      ^^^^^^^^^^
[00:47:07] 96 
[00:47:07] 96 
[00:47:07] - error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[00:47:07] + error[E0562]: `impl Trait` not allowed outside of function and inherent method return types or bindings
[00:47:07] 98   --> $DIR/where-allowed.rs:82:40
[00:47:07] 99    |
[00:47:07] 100 LL | fn in_Fn_return_in_generics<F: Fn() -> impl Debug> (_: F) { panic!() }
[00:47:07] 101    |                                        ^^^^^^^^^^
[00:47:07] 102 
[00:47:07] 102 
[00:47:07] - error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[00:47:07] + error[E0562]: `impl Trait` not allowed outside of function and inherent method return types or bindings
[00:47:07] 104   --> $DIR/where-allowed.rs:95:32
[00:47:07] 105    |
[00:47:07] 106 LL | struct InBraceStructField { x: impl Debug }
[00:47:07] 107    |                                ^^^^^^^^^^
[00:47:07] 108 
[00:47:07] 108 
[00:47:07] - error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[00:47:07] + error[E0562]: `impl Trait` not allowed outside of function and inherent method return types or bindings
[00:47:07] 110   --> $DIR/where-allowed.rs:99:41
[00:47:07] 111    |
[00:47:07] 112 LL | struct InAdtInBraceStructField { x: Vec<impl Debug> }
[00:47:07] 113    |                                         ^^^^^^^^^^
[00:47:07] 114 
[00:47:07] 114 
[00:47:07] - error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[00:47:07] + error[E0562]: `impl Trait` not allowed outside of function and inherent method return types or bindings
[00:47:07] 116   --> $DIR/where-allowed.rs:103:27
[00:47:07] 117    |
[00:47:07] 118 LL | struct InTupleStructField(impl Debug);
[00:47:07] 119    |                           ^^^^^^^^^^
[00:47:07] 120 
[00:47:07] 120 
[00:47:07] - error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[00:47:07] + error[E0562]: `impl Trait` not allowed outside of function and inherent method return types or bindings
[00:47:07] 122   --> $DIR/where-allowed.rs:108:25
[00:47:07] 123    |
[00:47:07] 124 LL |     InBraceVariant { x: impl Debug },
[00:47:07] 125    |                         ^^^^^^^^^^
[00:47:07] 126 
[00:47:07] 126 
[00:47:07] - error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[00:47:07] + error[E0562]: `impl Trait` not allowed outside of function and inherent method return types or bindings
[00:47:07] 128   --> $DIR/where-allowed.rs:110:20
[00:47:07] 129    |
[00:47:07] 130 LL |     InTupleVariant(impl Debug),
[00:47:07] 131    |                    ^^^^^^^^^^
[00:47:07] 132 
[00:47:07] 132 
[00:47:07] - error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[00:47:07] + error[E0562]: `impl Trait` not allowed outside of function and inherent method return types or bindings
[00:47:07] 134   --> $DIR/where-allowed.rs:121:23
[00:47:07] 135    |
[00:47:07] 136 LL |     fn in_return() -> impl Debug;
[00:47:07] 137    |                       ^^^^^^^^^^
[00:47:07] 138 
[00:47:07] 138 
[00:47:07] - error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[00:47:07] + error[E0562]: `impl Trait` not allowed outside of function and inherent method return types or bindings
[00:47:07] 140   --> $DIR/where-allowed.rs:132:16
[00:47:07] 141    |
[00:47:07] 142 LL |     type Out = impl Debug;
[00:47:07] 143    |                ^^^^^^^^^^
[00:47:07] 144 
[00:47:07] 144 
[00:47:07] - error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[00:47:07] + error[E0562]: `impl Trait` not allowed outside of function and inherent method return types or bindings
[00:47:07] 146   --> $DIR/where-allowed.rs:138:34
[00:47:07] 147    |
[00:47:07] 148 LL |     fn in_trait_impl_return() -> impl Debug { () }
[00:47:07] 149    |                                  ^^^^^^^^^^
[00:47:07] 150 
[00:47:07] 150 
[00:47:07] - error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[00:47:07] + error[E0562]: `impl Trait` not allowed outside of function and inherent method return types or bindings
[00:47:07] 152   --> $DIR/where-allowed.rs:151:33
[00:47:07] 153    |
[00:47:07] 154 LL |     fn in_foreign_parameters(_: impl Debug);
[00:47:07] 155    |                                 ^^^^^^^^^^
[00:47:07] 156 
[00:47:07] 156 
[00:47:07] - error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[00:47:07] + error[E0562]: `impl Trait` not allowed outside of function and inherent method return types or bindings
[00:47:07] 158   --> $DIR/where-allowed.rs:154:31
[00:47:07] 159    |
[00:47:07] 160 LL |     fn in_foreign_return() -> impl Debug;
[00:47:07] 161    |                               ^^^^^^^^^^
[00:47:07] 162 
[00:47:07] 162 
[00:47:07] - error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[00:47:07] + error[E0562]: `impl Trait` not allowed outside of function and inherent method return types or bindings
[00:47:07] 164   --> $DIR/where-allowed.rs:167:23
[00:47:07] 165    |
[00:47:07] 166 LL | type InTypeAlias<R>    --> $DIR/where-allowed.rs:232:46
[00:47:07] 237    |
[00:47:07] 238 LL |     let _in_return_in_local_variable = || -> impl Fn() { || {} };
[00:47:07] 239    |                                              ^^^^^^^^^
[00:47:07] 240 
[00:47:07] - error: aborting due to 39 previous errors
[00:47:07] + error: aborting due to 38 previous errors
[00:47:07] + error: aborting due to 38 previous errors
[00:47:07] 242 
[00:47:07] 243 Some errors occurred: E0562, E0666.
[00:47:07] 244 For more information about an error, try `rustc --explain E0562`.
[00:47:07] 
[00:47:07] 
[00:47:07] The actual stderr differed from the expected stderr.
[00:47:07] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/where-allowed/where-allowed.stderr
[00:47:07] To update references, rerun the tests and pass the `--bless` flag
[00:47:07] To only update this specific test, also pass `--test-args impl-trait/where-allowed.rs`
[00:47:07] error: 1 errors occurred comparing output.
[00:47:07] status: exit code: 1
[00:47:07] status: exit code: 1
[00:47:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/where-allowed.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/where-allowed/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/where-allowed/auxiliary" "-A" "unused"
[00:47:07] ------------------------------------------
[00:47:07] 
[00:47:07] ------------------------------------------
[00:47:07] stderr:
[00:47:07] stderr:
[00:47:07] ------------------------------------------
[00:47:07] {"message":"nested `impl Trait` is not allowed","code":{"code":"E0666","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/impl-trait/where-allowed.rs","byte_start":2268,"byte_end":2287,"line_start":60,"line_end":60,"column_start":43,"column_end":62,"is_primary":false,"text":[{"text":"fn in_impl_Fn_parameter_in_parameters(_: &impl Fn(impl Debug)) { panic!() }","highlight_start":43,"highlight_end":62}],"label":"outer `impl Trait`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/impl-trait/where-allowed.rs","byte_start":2276,"byte_end":2286,"line_start":60,"line_end":60,"column_start":51,"column_end":61,"is_primary":true,"text":[{"text":"fn in_impl_Fn_parameter_in_parameters(_: &impl Fn(impl Debug)) { panic!() }","highlight_start":51,"highlight_end":61}],"label":"nested `impl Trait` here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0666]: nested `impl Trait` is not allowed\n  --> /checkout/src/test/ui/impl-trait/where-allowed.rs:60:51\n   |\nLL | fn in_impl_Fn_parameter_in_parameters(_: &impl Fn(impl Debug)) { panic!() }\n   |                                           --------^^^^^^^^^^-\n   |                                           |       |\n   |                                           |       nested `impl Trait` here\n   |                                           outer `impl Trait`\n\n"}
[00:47:07] {"message":"nested `iAbstract return types (written `impl Trait` for some trait `Trait`) are only\nallowed as function and inherent impl return types or binding types.\n\nErroneous code example:\n\n